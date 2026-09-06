//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/local_storage_rcu_tasks_trace_bench.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } task_storage SEC(".maps");
    long hits;
    long gp_hits;
    long gp_times;
    long current_gp_start;
    long unexpected;
    bool postgp_seen;
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn get_local(ctx: *mut c_void) -> c_int {
    int get_local(void *ctx)
    {
    struct task_struct *task;
    int idx;
    int *s;
    idx = 0;
    task = bpf_get_current_task_btf();
    s = bpf_task_storage_get(&task_storage, task, &idx,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!s)
    return 0;
// s = 3;
    bpf_task_storage_delete(&task_storage, task);
    __sync_add_and_fetch(&hits, 1);
    return 0;
    }
    SEC("fentry/rcu_tasks_trace_pregp_step")
#[no_mangle]
pub unsafe extern "C" fn pregp_step(ctx: *mut pt_regs) -> c_int {
    int pregp_step(struct pt_regs *ctx)
    {
    current_gp_start = bpf_ktime_get_ns();
    return 0;
    }
    SEC("fentry/rcu_tasks_trace_postgp")
#[no_mangle]
pub unsafe extern "C" fn postgp(ctx: *mut pt_regs) -> c_int {
    int postgp(struct pt_regs *ctx)
    {
    if (!current_gp_start && postgp_seen) {
// Will only happen if prog tracing rcu_tasks_trace_pregp_step doesn't
// execute before this prog
//
    __sync_add_and_fetch(&unexpected, 1);
    return 0;
    }
    __sync_add_and_fetch(&gp_times, bpf_ktime_get_ns() - current_gp_start);
    __sync_add_and_fetch(&gp_hits, 1);
    current_gp_start = 0;
    postgp_seen = true;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
