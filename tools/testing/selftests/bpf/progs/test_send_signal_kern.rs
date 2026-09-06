//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_send_signal_kern.c
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
// Copyright (c) 2019 Facebook

    struct task_struct *bpf_task_from_pid(int pid) __ksym;
    void bpf_task_release(struct task_struct *p) __ksym;
    int bpf_send_signal_task(struct task_struct *task, int sig, enum pid_type type, u64 value) __ksym;
    let mut sig: __u32 = 0, pid = 0, status = 0, signal_thread = 0, target_pid = 0;
#[no_mangle]
unsafe extern "C" fn bpf_send_signal_test(ctx: *mut c_void) -> __always_inline int {
    static __always_inline int bpf_send_signal_test(void *ctx)
    {
    struct task_struct *target_task = core::ptr::null_mut();
    int ret;
    u64 value;
    if (status != 0 || pid == 0)
    return 0;
    if ((bpf_get_current_pid_tgid() >> 32) == pid) {
    if (target_pid) {
    target_task = bpf_task_from_pid(target_pid);
    if (!target_task)
    return 0;
    value = 8;
    }
    if (signal_thread) {
    if (target_pid)
    ret = bpf_send_signal_task(target_task, sig, PIDTYPE_PID, value);
    else
    ret = bpf_send_signal_thread(sig);
    } else {
    if (target_pid)
    ret = bpf_send_signal_task(target_task, sig, PIDTYPE_TGID, value);
    else
    ret = bpf_send_signal(sig);
    }
    if (ret == 0)
    status = 1;
    }
    if (target_task)
    bpf_task_release(target_task);
    return 0;
    }
    SEC("tracepoint/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn send_signal_tp(ctx: *mut c_void) -> c_int {
    int send_signal_tp(void *ctx)
    {
    return bpf_send_signal_test(ctx);
    }
    SEC("tracepoint/sched/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn send_signal_tp_sched(ctx: *mut c_void) -> c_int {
    int send_signal_tp_sched(void *ctx)
    {
    return bpf_send_signal_test(ctx);
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn send_signal_perf(ctx: *mut c_void) -> c_int {
    int send_signal_perf(void *ctx)
    {
    return bpf_send_signal_test(ctx);
    }
    char __license[] SEC("license") = "GPL";
