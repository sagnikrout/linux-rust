//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/perf_event_stackmap.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

pub const PERF_MAX_STACK_DEPTH: c_int = 127;

    typedef __u64 stack_trace_t[PERF_MAX_STACK_DEPTH];
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(max_entries, 16384);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stackmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stackdata_map SEC(".maps");
    let mut stackid_kernel: c_long = 1;
    let mut stackid_user: c_long = 1;
    let mut stack_kernel: c_long = 1;
    let mut stack_user: c_long = 1;
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn oncpu(ctx: *mut c_void) -> c_int {
    int oncpu(void *ctx)
    {
    stack_trace_t *trace;
    let mut key: __u32 = 0;
    long val;
    val = bpf_get_stackid(ctx, &stackmap, 0);
    if (val >= 0)
    stackid_kernel = 2;
    val = bpf_get_stackid(ctx, &stackmap, BPF_F_USER_STACK);
    if (val >= 0)
    stackid_user = 2;
    trace = bpf_map_lookup_elem(&stackdata_map, &key);
    if (!trace)
    return 0;
    val = bpf_get_stack(ctx, trace, sizeof(stack_trace_t), 0);
    if (val > 0)
    stack_kernel = 2;
    val = bpf_get_stack(ctx, trace, sizeof(stack_trace_t), BPF_F_USER_STACK);
    if (val > 0)
    stack_user = 2;
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
