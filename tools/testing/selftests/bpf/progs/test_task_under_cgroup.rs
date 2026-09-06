//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_task_under_cgroup.c
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
// Copyright (c) 2023 Bytedance

    struct cgroup *bpf_cgroup_from_id(u64 cgid) __ksym;
    long bpf_task_under_cgroup(struct task_struct *task, struct cgroup *ancestor) __ksym;
    void bpf_cgroup_release(struct cgroup *p) __ksym;
    struct task_struct *bpf_task_acquire(struct task_struct *p) __ksym;
    void bpf_task_release(struct task_struct *p) __ksym;
    const volatile int local_pid;
    const volatile __u64 cgid;
    int remote_pid;
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tp_btf_run, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(tp_btf_run, struct task_struct *task, u64 clone_flags)
    {
    struct cgroup *cgrp = core::ptr::null_mut();
    struct task_struct *acquired;
    if (local_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    acquired = bpf_task_acquire(task);
    if (!acquired)
    return 0;
    if (local_pid == acquired.tgid)
    goto out;
    cgrp = bpf_cgroup_from_id(cgid);
    if (!cgrp)
    goto out;
    if (bpf_task_under_cgroup(acquired, cgrp))
    remote_pid = acquired.tgid;
    out:
    if (cgrp)
    bpf_cgroup_release(cgrp);
    bpf_task_release(acquired);
    return 0;
    }
    SEC("lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_run, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(lsm_run, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    struct cgroup *cgrp = core::ptr::null_mut();
    struct task_struct *task;
    let mut ret: c_int = 0;
    task = bpf_get_current_task_btf();
    if (local_pid != task.pid)
    return 0;
    if (cmd != BPF_LINK_CREATE)
    return 0;
// 1 is the root cgroup
    cgrp = bpf_cgroup_from_id(1);
    if (!cgrp)
    goto out;
    if (!bpf_task_under_cgroup(task, cgrp))
    ret = -1;
    bpf_cgroup_release(cgrp);
    out:
    return ret;
    }
    char _license[] SEC("license") = "GPL";
