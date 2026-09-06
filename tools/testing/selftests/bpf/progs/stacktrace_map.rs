//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stacktrace_map.c
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
    typedef __u64 stack_trace_t[PERF_MAX_STACK_DEPTH];
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(max_entries, 16384);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stackmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 16384);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stack_amap SEC(".maps");
// taken from /sys/kernel/tracing/events/sched/sched_switch/format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_switch_args {
    pub pad: c_ulonglong,
    pub prev_comm: [c_char; TASK_COMM_LEN],
    pub prev_pid: c_int,
    pub prev_prio: c_int,
    pub prev_state: c_longlong,
    pub next_comm: [c_char; TASK_COMM_LEN],
    pub next_pid: c_int,
    pub next_prio: c_int,
}

    __u32 stack_id;
    SEC("tracepoint/sched/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn oncpu(ctx: *mut sched_switch_args) -> c_int {
    int oncpu(struct sched_switch_args *ctx)
    {
    let mut max_len: __u32 = PERF_MAX_STACK_DEPTH * sizeof(__u64);
    let mut key: __u32 = 0, val = 0, *value_p;
    void *stack_p;
    value_p = bpf_map_lookup_elem(&control_map, &key);
    if (value_p && *value_p)
    return 0; /* skip if non-zero *value_p */
// The size of stackmap and stackid_hmap should be the same
    key = bpf_get_stackid(ctx, &stackmap, 0);
    if ((int)key >= 0) {
    stack_id = key;
    bpf_map_update_elem(&stackid_hmap, &key, &val, 0);
    stack_p = bpf_map_lookup_elem(&stack_amap, &key);
    if (stack_p)
    bpf_get_stack(ctx, stack_p, max_len, 0);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
