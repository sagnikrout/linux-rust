//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_stacktrace_build_id.c
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
// Copyright (c) 2018 Facebook

pub const PERF_MAX_STACK_DEPTH: c_int = 127;

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } control_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 16384);
    __type(key, __u32);
    __type(value, __u32);
    } stackid_hmap SEC(".maps");
    typedef struct bpf_stack_build_id stack_trace_t[PERF_MAX_STACK_DEPTH];
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(max_entries, 128);
    __uint(map_flags, BPF_F_STACK_BUILD_ID);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stackmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 128);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stack_amap SEC(".maps");
    SEC("kprobe/urandom_read_iter")
#[no_mangle]
pub unsafe extern "C" fn oncpu(args: *mut pt_regs) -> c_int {
    int oncpu(struct pt_regs *args)
    {
    __u32 max_len = sizeof(struct bpf_stack_build_id)
// PERF_MAX_STACK_DEPTH;
    let mut key: __u32 = 0, val = 0, *value_p;
    void *stack_p;
    value_p = bpf_map_lookup_elem(&control_map, &key);
    if (value_p && *value_p)
    return 0; /* skip if non-zero *value_p */
// The size of stackmap and stackid_hmap should be the same
    key = bpf_get_stackid(args, &stackmap, BPF_F_USER_STACK);
    if ((int)key >= 0) {
    bpf_map_update_elem(&stackid_hmap, &key, &val, 0);
    stack_p = bpf_map_lookup_elem(&stack_amap, &key);
    if (stack_p)
    bpf_get_stack(args, stack_p, max_len,
    BPF_F_USER_STACK | BPF_F_USER_BUILD_ID);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
