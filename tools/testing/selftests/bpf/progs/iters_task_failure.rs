//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_task_failure.c
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
    struct cgroup *bpf_cgroup_from_id(u64 cgid) __ksym;
    void bpf_cgroup_release(struct cgroup *p) __ksym;
    void bpf_rcu_read_lock(void) __ksym;
    void bpf_rcu_read_unlock(void) __ksym;
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(protection": "kernel func bpf_iter_task_new requires RCU critical section) -> __failure {
    __failure __msg("kernel func bpf_iter_task_new requires RCU critical section protection")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: iter_tasks_without_lock) -> c_int {
    int BPF_PROG(iter_tasks_without_lock)
    {
    struct task_struct *pos;
    bpf_for_each(task, pos, core::ptr::null_mut(), BPF_TASK_ITER_ALL_PROCS) {
    }
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(protection": "kernel func bpf_iter_css_new requires RCU critical section) -> __failure {
    __failure __msg("kernel func bpf_iter_css_new requires RCU critical section protection")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: iter_css_without_lock) -> c_int {
    int BPF_PROG(iter_css_without_lock)
    {
    let mut cg_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp = bpf_cgroup_from_id(cg_id);
    struct cgroup_subsys_state *root_css, *pos;
    if (!cgrp)
    return 0;
    root_css = &cgrp.self;
    bpf_for_each(css, pos, root_css, BPF_CGROUP_ITER_DESCENDANTS_POST) {
    }
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_iter_task_next": "expected an RCU CS when using) -> __failure {
    __failure __msg("expected an RCU CS when using bpf_iter_task_next")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: iter_tasks_lock_and_unlock) -> c_int {
    int BPF_PROG(iter_tasks_lock_and_unlock)
    {
    struct task_struct *pos;
    bpf_rcu_read_lock();
    bpf_for_each(task, pos, core::ptr::null_mut(), BPF_TASK_ITER_ALL_PROCS) {
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    }
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_iter_task_next": "expected an RCU CS when using) -> __failure {
    __failure __msg("expected an RCU CS when using bpf_iter_task_next")
    __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: iter_tasks_rcu_state_pruning) -> c_int {
    int BPF_PROG(iter_tasks_rcu_state_pruning)
    {
    struct bpf_iter_task it;
    asm volatile (
    "call %[bpf_rcu_read_lock];"
    "r1 = %[it];"
    "r2 = 0;"
    "r3 = 0;" /* BPF_TASK_ITER_ALL_PROCS */
    "call %[bpf_iter_task_new];"
    "call %[bpf_get_prandom_u32];"
    "if w0 == 0 goto unprotected_%=;"
// Keep the outer RCU lock active on the straight-line path.
    "call %[bpf_rcu_read_lock];"
    "call %[bpf_rcu_read_unlock];"
    "goto merge_%=;"
    "unprotected_%=:"
// Create an unprotected gap on the taken path.
    "call %[bpf_rcu_read_unlock];"
    "call %[bpf_rcu_read_lock];"
    "merge_%=: r1 = %[it];"
    "call %[bpf_iter_task_next];"
    "r1 = %[it];"
    "call %[bpf_iter_task_destroy];"
    "call %[bpf_rcu_read_unlock];"
    :
    : __imm_ptr(it),
    __imm(bpf_get_prandom_u32),
    __imm(bpf_iter_task_new),
    __imm(bpf_iter_task_next),
    __imm(bpf_iter_task_destroy),
    __imm(bpf_rcu_read_lock),
    __imm(bpf_rcu_read_unlock)
    : __clobber_common
    );
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_iter_css_next": "expected an RCU CS when using) -> __failure {
    __failure __msg("expected an RCU CS when using bpf_iter_css_next")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: iter_css_lock_and_unlock) -> c_int {
    int BPF_PROG(iter_css_lock_and_unlock)
    {
    let mut cg_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp = bpf_cgroup_from_id(cg_id);
    struct cgroup_subsys_state *root_css, *pos;
    if (!cgrp)
    return 0;
    root_css = &cgrp.self;
    bpf_rcu_read_lock();
    bpf_for_each(css, pos, root_css, BPF_CGROUP_ITER_DESCENDANTS_POST) {
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    }
    bpf_rcu_read_unlock();
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("?fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_lsm: "css_task_iter is only allowed in, progs": bpf_iter and sleepable) -> __failure {
    __failure __msg("css_task_iter is only allowed in bpf_lsm, bpf_iter and sleepable progs")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: iter_css_task_for_each) -> c_int {
    int BPF_PROG(iter_css_task_for_each)
    {
    let mut cg_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp = bpf_cgroup_from_id(cg_id);
    struct cgroup_subsys_state *css;
    struct task_struct *task;
    if (cgrp == core::ptr::null_mut())
    return 0;
    css = &cgrp.self;
    bpf_for_each(css_task, task, css, CSS_TASK_ITER_PROCS) {
    }
    bpf_cgroup_release(cgrp);
    return 0;
    }
