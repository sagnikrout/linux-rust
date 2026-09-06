//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/exhandler_kern.c
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
// Copyright (c) 2021, Oracle and/or its affiliates.

    char _license[] SEC("license") = "GPL";
    unsigned int exception_triggered;
    int test_pid;
// TRACE_EVENT(task_newtask,
// TP_PROTO(struct task_struct *p, u64 clone_flags)
//
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_task_newtask, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(trace_task_newtask, struct task_struct *task, u64 clone_flags)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
    struct callback_head *work;
    void *func;
    if (test_pid != pid)
    return 0;
// To verify we hit an exception we dereference task->task_works->func.
// If task work has been added,
// - task->task_works is non-NULL; and
// - task->task_works->func is non-NULL also (the callback function
// must be specified for the task work.
//
// However, for a newly-created task, task->task_works is NULLed,
// so we know the exception handler triggered if task_works is
// NULL and func is NULL.
//
    work = task.task_works;
    func = work.func;
// Currently verifier will fail for `btf_ptr |= btf_ptr` * instruction.
// To workaround the issue, use barrier_var() and rewrite as below to
// prevent compiler from generating verifier-unfriendly code.
//
    barrier_var(work);
    if (work)
    return 0;
    barrier_var(func);
    if (func)
    return 0;
    exception_triggered++;
    return 0;
    }
