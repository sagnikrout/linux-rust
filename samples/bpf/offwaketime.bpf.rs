//! Automatically rewritten from C to Rust
//! Source: samples/bpf/offwaketime.bpf.c
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

pub const PERF_MAX_STACK_DEPTH: c_int = 127;

pub const MINBLOCK_US: c_int = 1;
pub const MAX_ENTRIES: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_t {
    pub waker: [c_char; TASK_COMM_LEN],
    pub target: [c_char; TASK_COMM_LEN],
    pub wret: u32,
    pub tret: u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, struct key_t);
    __type(value, u64);
    __uint(max_entries, MAX_ENTRIES);
    } counts SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, u32);
    __type(value, u64);
    __uint(max_entries, MAX_ENTRIES);
    } start SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wokeby_t {
    pub name: [c_char; TASK_COMM_LEN],
    pub ret: u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, u32);
    __type(value, struct wokeby_t);
    __uint(max_entries, MAX_ENTRIES);
    } wokeby SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(key_size, sizeof(u32));
    __uint(value_size, PERF_MAX_STACK_DEPTH * sizeof(u64));
    __uint(max_entries, MAX_ENTRIES);
    } stackmap SEC(".maps");

    SEC("kprobe/try_to_wake_up")
#[no_mangle]
pub unsafe extern "C" fn waker(ctx: *mut pt_regs) -> c_int {
    int waker(struct pt_regs *ctx)
    {
    struct task_struct *p = (void *)PT_REGS_PARM1_CORE(ctx);
    let mut pid: u32 = BPF_CORE_READ(p, pid);
    struct wokeby_t woke;
    bpf_get_current_comm(&woke.name, sizeof(woke.name));
    woke.ret = bpf_get_stackid(ctx, &stackmap, STACKID_FLAGS);
    bpf_map_update_elem(&wokeby, &pid, &woke, BPF_ANY);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn update_counts(ctx: *mut c_void, pid: u32, delta: u64) -> c_int {
    static inline int update_counts(void *ctx, u32 pid, u64 delta)
    {
    struct wokeby_t *woke;
    let mut zero: u64 = 0, *val;
    struct key_t key;
    __builtin_memset(&key.waker, 0, sizeof(key.waker));
    bpf_get_current_comm(&key.target, sizeof(key.target));
    key.tret = bpf_get_stackid(ctx, &stackmap, STACKID_FLAGS);
    key.wret = 0;
    woke = bpf_map_lookup_elem(&wokeby, &pid);
    if (woke) {
    key.wret = woke.ret;
    __builtin_memcpy(&key.waker, woke.name, sizeof(key.waker));
    bpf_map_delete_elem(&wokeby, &pid);
    }
    val = bpf_map_lookup_elem(&counts, &key);
    if (!val) {
    bpf_map_update_elem(&counts, &key, &zero, BPF_NOEXIST);
    val = bpf_map_lookup_elem(&counts, &key);
    if (!val)
    return 0;
    }
    (*val) += delta;
    return 0;
    }

// taken from /sys/kernel/tracing/events/sched/sched_switch/format
    SEC("tracepoint/sched/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn oncpu(ctx: *mut trace_event_raw_sched_switch) -> c_int {
    int oncpu(struct trace_event_raw_sched_switch *ctx)
    {
// record previous thread sleep time
    let mut pid: u32 = ctx.prev_pid;

    SEC("kprobe.multi/finish_task_switch*")
#[no_mangle]
pub unsafe extern "C" fn oncpu(ctx: *mut pt_regs) -> c_int {
    int oncpu(struct pt_regs *ctx)
    {
    struct task_struct *p = (void *)PT_REGS_PARM1_CORE(ctx);
// record previous thread sleep time
    let mut pid: u32 = BPF_CORE_READ(p, pid);

    u64 delta, ts, *tsp;
    ts = bpf_ktime_get_ns();
    bpf_map_update_elem(&start, &pid, &ts, BPF_ANY);
// calculate current thread's delta time
    pid = bpf_get_current_pid_tgid();
    tsp = bpf_map_lookup_elem(&start, &pid);
    if (!tsp)
// missed start or filtered
    return 0;
    delta = bpf_ktime_get_ns() - *tsp;
    bpf_map_delete_elem(&start, &pid);
    delta = delta / 1000;
    if (delta < MINBLOCK_US)
    return 0;
    return update_counts(ctx, pid, delta);
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
