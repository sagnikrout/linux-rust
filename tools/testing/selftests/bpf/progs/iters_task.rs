//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_task.c
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
// Copyright (C) 2023 Chuyi Zhou <zhouchuyi@bytedance.com>

    char _license[] SEC("license") = "GPL";
    pid_t target_pid;
    int procs_cnt, threads_cnt, proc_threads_cnt, invalid_cnt;
    void bpf_rcu_read_lock(void) __ksym;
    void bpf_rcu_read_unlock(void) __ksym;
    SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn iter_task_for_each_sleep(ctx: *mut c_void) -> c_int {
    int iter_task_for_each_sleep(void *ctx)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct task_struct *pos;
    if (cur_task.pid != target_pid)
    return 0;
    procs_cnt = threads_cnt = proc_threads_cnt = 0;
    bpf_rcu_read_lock();
    bpf_for_each(task, pos, core::ptr::null_mut(), ~0U) {
// Below instructions shouldn't be executed for invalid flags
    invalid_cnt++;
    }
    bpf_for_each(task, pos, core::ptr::null_mut(), BPF_TASK_ITER_PROC_THREADS) {
// Below instructions shouldn't be executed for invalid task__nullable
    invalid_cnt++;
    }
    bpf_for_each(task, pos, core::ptr::null_mut(), BPF_TASK_ITER_ALL_PROCS)
    if (pos.pid == target_pid)
    procs_cnt++;
    bpf_for_each(task, pos, cur_task, BPF_TASK_ITER_PROC_THREADS)
    proc_threads_cnt++;
    bpf_for_each(task, pos, core::ptr::null_mut(), BPF_TASK_ITER_ALL_THREADS)
    if (pos.tgid == target_pid)
    threads_cnt++;
    bpf_rcu_read_unlock();
    return 0;
    }
