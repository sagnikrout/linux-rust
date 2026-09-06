//! Automatically rewritten from C to Rust
//! Source: samples/bpf/trace_event_kern.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// Copyright (c) 2016 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_t {
    pub comm: [c_char; TASK_COMM_LEN],
    pub kernstack: u32,
    pub userstack: u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, struct key_t);
    __type(value, u64);
    __uint(max_entries, 10000);
    } counts SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(key_size, sizeof(u32));
    __uint(value_size, PERF_MAX_STACK_DEPTH * sizeof(u64));
    __uint(max_entries, 10000);
    } stackmap SEC(".maps");

    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut bpf_perf_event_data) -> c_int {
    int bpf_prog1(struct bpf_perf_event_data *ctx)
    {
    char time_fmt1[] = "Time Enabled: %llu, Time Running: %llu";
    char time_fmt2[] = "Get Time Failed, ErrCode: %d";
    char addr_fmt[] = "Address recorded on event: %llx";
    char fmt[] = "CPU-%d period %lld ip %llx";
    let mut cpu: u32 = bpf_get_smp_processor_id();
    struct bpf_perf_event_value value_buf;
    struct key_t key;
    u64 *val, one = 1;
    int ret;
    if (ctx.sample_period < 10000)
// ignore warmup
    return 0;
    bpf_get_current_comm(&key.comm, sizeof(key.comm));
    key.kernstack = bpf_get_stackid(ctx, &stackmap, KERN_STACKID_FLAGS);
    key.userstack = bpf_get_stackid(ctx, &stackmap, USER_STACKID_FLAGS);
    if ((int)key.kernstack < 0 && (int)key.userstack < 0) {
    bpf_trace_printk(fmt, sizeof(fmt), cpu, ctx.sample_period,
    PT_REGS_IP(&ctx.regs));
    return 0;
    }
    ret = bpf_perf_prog_read_value(ctx, (void *)&value_buf, sizeof(struct bpf_perf_event_value));
    if (!ret)
    bpf_trace_printk(time_fmt1, sizeof(time_fmt1), value_buf.enabled, value_buf.running);
    else
    bpf_trace_printk(time_fmt2, sizeof(time_fmt2), ret);
    if (ctx.addr != 0)
    bpf_trace_printk(addr_fmt, sizeof(addr_fmt), ctx.addr);
    val = bpf_map_lookup_elem(&counts, &key);
    if (val)
    (*val)++;
    else
    bpf_map_update_elem(&counts, &key, &one, BPF_NOEXIST);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
