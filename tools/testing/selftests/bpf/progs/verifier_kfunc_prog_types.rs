//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_kfunc_prog_types.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
//
// Task kfuncs
//
#[no_mangle]
unsafe extern "C" fn task_kfunc_load_test() {
    static void task_kfunc_load_test(void)
    {
    struct task_struct *current, *ref_1, *ref_2;
    current = bpf_get_current_task_btf();
    ref_1 = bpf_task_from_pid(current.pid);
    if (!ref_1)
    return;
    ref_2 = bpf_task_acquire(ref_1);
    if (ref_2)
    bpf_task_release(ref_2);
    bpf_task_release(ref_1);
    }
    SEC("raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_raw_tp) -> c_int {
    int BPF_PROG(task_kfunc_raw_tp)
    {
    task_kfunc_load_test();
    return 0;
    }
    SEC("syscall")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_syscall) -> c_int {
    int BPF_PROG(task_kfunc_syscall)
    {
    task_kfunc_load_test();
    return 0;
    }
    SEC("tracepoint")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_tracepoint) -> c_int {
    int BPF_PROG(task_kfunc_tracepoint)
    {
    task_kfunc_load_test();
    return 0;
    }
    SEC("perf_event")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_perf_event) -> c_int {
    int BPF_PROG(task_kfunc_perf_event)
    {
    task_kfunc_load_test();
    return 0;
    }
//
// cgroup kfuncs
//
#[no_mangle]
unsafe extern "C" fn cgrp_kfunc_load_test() {
    static void cgrp_kfunc_load_test(void)
    {
    struct cgroup *cgrp, *ref;
    cgrp = bpf_cgroup_from_id(0);
    if (!cgrp)
    return;
    ref = bpf_cgroup_acquire(cgrp);
    if (!ref) {
    bpf_cgroup_release(cgrp);
    return;
    }
    bpf_cgroup_release(ref);
    bpf_cgroup_release(cgrp);
    }
    SEC("raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_raw_tp) -> c_int {
    int BPF_PROG(cgrp_kfunc_raw_tp)
    {
    cgrp_kfunc_load_test();
    return 0;
    }
    SEC("syscall")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_syscall) -> c_int {
    int BPF_PROG(cgrp_kfunc_syscall)
    {
    cgrp_kfunc_load_test();
    return 0;
    }
    SEC("tracepoint")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_tracepoint) -> c_int {
    int BPF_PROG(cgrp_kfunc_tracepoint)
    {
    cgrp_kfunc_load_test();
    return 0;
    }
    SEC("perf_event")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgrp_kfunc_perf_event) -> c_int {
    int BPF_PROG(cgrp_kfunc_perf_event)
    {
    cgrp_kfunc_load_test();
    return 0;
    }
//
// cpumask kfuncs
//
#[no_mangle]
unsafe extern "C" fn cpumask_kfunc_load_test() {
    static void cpumask_kfunc_load_test(void)
    {
    struct bpf_cpumask *alloc, *ref;
    alloc = bpf_cpumask_create();
    if (!alloc)
    return;
    ref = bpf_cpumask_acquire(alloc);
    bpf_cpumask_set_cpu(0, alloc);
    bpf_cpumask_test_cpu(0, (const struct cpumask *)ref);
    bpf_cpumask_release(ref);
    bpf_cpumask_release(alloc);
    }
    SEC("raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cpumask_kfunc_raw_tp) -> c_int {
    int BPF_PROG(cpumask_kfunc_raw_tp)
    {
    cpumask_kfunc_load_test();
    return 0;
    }
    SEC("syscall")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cpumask_kfunc_syscall) -> c_int {
    int BPF_PROG(cpumask_kfunc_syscall)
    {
    cpumask_kfunc_load_test();
    return 0;
    }
    SEC("tracepoint")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cpumask_kfunc_tracepoint) -> c_int {
    int BPF_PROG(cpumask_kfunc_tracepoint)
    {
    cpumask_kfunc_load_test();
    return 0;
    }
    SEC("perf_event")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cpumask_kfunc_perf_event) -> c_int {
    int BPF_PROG(cpumask_kfunc_perf_event)
    {
    cpumask_kfunc_load_test();
    return 0;
    }
