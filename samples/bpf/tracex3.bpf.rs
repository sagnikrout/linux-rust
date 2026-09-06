//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tracex3.bpf.c
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
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_key {
    pub dev: dev_t,
    pub _pad: u32,
    pub sector: sector_t,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, long);
    __type(value, u64);
    __uint(max_entries, 4096);
    } my_map SEC(".maps");
// from /sys/kernel/tracing/events/block/block_io_start/format
    SEC("tracepoint/block/block_io_start")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut trace_event_raw_block_rq) -> c_int {
    int bpf_prog1(struct trace_event_raw_block_rq *ctx)
    {
    let mut val: u64 = bpf_ktime_get_ns();
    struct start_key key = {
    .dev = ctx.dev,
    .sector = ctx.sector
    };
    bpf_map_update_elem(&my_map, &key, &val, BPF_ANY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn log2l(n: c_ulonglong) -> c_uint {
    static unsigned int log2l(unsigned long long n)
    {

    let mut i: c_int = -(n == 0);
    S(32); S(16); S(8); S(4); S(2); S(1);
    return i;

    }
pub const SLOTS: c_int = 100;
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u64));
    __uint(max_entries, SLOTS);
    } lat_map SEC(".maps");
// from /sys/kernel/tracing/events/block/block_io_done/format
    SEC("tracepoint/block/block_io_done")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut trace_event_raw_block_rq) -> c_int {
    int bpf_prog2(struct trace_event_raw_block_rq *ctx)
    {
    struct start_key key = {
    .dev = ctx.dev,
    .sector = ctx.sector
    };
    u64 *value, l, base;
    u32 index;
    value = bpf_map_lookup_elem(&my_map, &key);
    if (!value)
    return 0;
    let mut cur_time: u64 = bpf_ktime_get_ns();
    let mut delta: u64 = cur_time - *value;
    bpf_map_delete_elem(&my_map, &key);
// the lines below are computing index = log10(delta)*10
// using integer arithmetic
// index = 29 ~ 1 usec
// index = 59 ~ 1 msec
// index = 89 ~ 1 sec
// index = 99 ~ 10sec or more
// log10(x)*10 = log2(x)*10/log2(10) = log2(x)*3
//
    l = log2l(delta);
    base = 1ll << l;
    index = (l * 64 + (delta - base) * 64 / base) * 3 / 64;
    if (index >= SLOTS)
    index = SLOTS - 1;
    value = bpf_map_lookup_elem(&lat_map, &index);
    if (value)
// value += 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
