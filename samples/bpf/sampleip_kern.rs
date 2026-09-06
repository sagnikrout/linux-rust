//! Automatically rewritten from C to Rust
//! Source: samples/bpf/sampleip_kern.c
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


// Copyright 2016 Netflix, Inc.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const MAX_IPS: c_int = 8192;
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, u64);
    __type(value, u32);
    __uint(max_entries, MAX_IPS);
    } ip_map SEC(".maps");
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn do_sample(ctx: *mut bpf_perf_event_data) -> c_int {
    int do_sample(struct bpf_perf_event_data *ctx)
    {
    u64 ip;
    u32 *value, init_val = 1;
    ip = PT_REGS_IP(&ctx.regs);
    value = bpf_map_lookup_elem(&ip_map, &ip);
    if (value)
// value += 1;
    else
// E2BIG not tested for this example only
    bpf_map_update_elem(&ip_map, &ip, &init_val, BPF_NOEXIST);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
