//! Automatically rewritten from C to Rust
//! Source: samples/bpf/lathist_kern.c
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


// Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
// Copyright (c) 2015 BMW Car IT GmbH
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const MAX_ENTRIES: c_int = 20;
pub const MAX_CPU: c_int = 4;
// We need to stick to static allocated memory (an array instead of
// hash table) because managing dynamic memory from the
// trace_preempt_[on|off] tracepoints hooks is not supported.
//
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, u64);
    __uint(max_entries, MAX_CPU);
    } my_map SEC(".maps");
    SEC("kprobe/trace_preempt_off")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> c_int {
    int bpf_prog1(struct pt_regs *ctx)
    {
    let mut cpu: c_int = bpf_get_smp_processor_id();
    u64 *ts = bpf_map_lookup_elem(&my_map, &cpu);
    if (ts)
// ts = bpf_ktime_get_ns();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn log2(v: c_uint) -> c_uint {
    static unsigned int log2(unsigned int v)
    {
    unsigned int r;
    unsigned int shift;
    r = (v > 0xFFFF) << 4; v >>= r;
    shift = (v > 0xFF) << 3; v >>= shift; r |= shift;
    shift = (v > 0xF) << 2; v >>= shift; r |= shift;
    shift = (v > 0x3) << 1; v >>= shift; r |= shift;
    r |= (v >> 1);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn log2l(v: c_ulong) -> c_uint {
    static unsigned int log2l(unsigned long v)
    {
    let mut hi: c_uint = v >> 32;
    if (hi)
    return log2(hi) + 32;
    else
    return log2(v);
    }
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, long);
    __uint(max_entries, MAX_CPU * MAX_ENTRIES);
    } my_lat SEC(".maps");
    SEC("kprobe/trace_preempt_on")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut pt_regs) -> c_int {
    int bpf_prog2(struct pt_regs *ctx)
    {
    u64 *ts, cur_ts, delta;
    int key, cpu;
    long *val;
    cpu = bpf_get_smp_processor_id();
    ts = bpf_map_lookup_elem(&my_map, &cpu);
    if (!ts)
    return 0;
    cur_ts = bpf_ktime_get_ns();
    delta = log2l(cur_ts - *ts);
    if (delta > MAX_ENTRIES - 1)
    delta = MAX_ENTRIES - 1;
    key = cpu * MAX_ENTRIES + delta;
    val = bpf_map_lookup_elem(&my_lat, &key);
    if (val)
    __sync_fetch_and_add((long *)val, 1);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
