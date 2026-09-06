//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_cgroup1_hierarchy.c
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
// Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com>

    __u32 target_ancestor_level;
    __u64 target_ancestor_cgid;
    int target_pid, target_hid;
    struct cgroup *bpf_task_get_cgroup1(struct task_struct *task, int hierarchy_id) __ksym;
    struct cgroup *bpf_cgroup_ancestor(struct cgroup *cgrp, int level) __ksym;
    void bpf_cgroup_release(struct cgroup *cgrp) __ksym;
#[no_mangle]
unsafe extern "C" fn bpf_link_create_verify(cmd: c_int) -> c_int {
    static int bpf_link_create_verify(int cmd)
    {
    struct cgroup *cgrp, *ancestor;
    struct task_struct *task;
    let mut ret: c_int = 0;
    if (cmd != BPF_LINK_CREATE)
    return 0;
    task = bpf_get_current_task_btf();
// Then it can run in parallel with others
    if (task.pid != target_pid)
    return 0;
    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if (!cgrp)
    return 0;
// Refuse it if its cgid or its ancestor's cgid is the target cgid
    if (cgrp.kn.id == target_ancestor_cgid)
    ret = -1;
    ancestor = bpf_cgroup_ancestor(cgrp, target_ancestor_level);
    if (!ancestor)
    goto out;
    if (ancestor.kn.id == target_ancestor_cgid)
    ret = -1;
    bpf_cgroup_release(ancestor);
    out:
    bpf_cgroup_release(cgrp);
    return ret;
    }
    SEC("lsm/bpf")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_run, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(lsm_run, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    return bpf_link_create_verify(cmd);
    }
    SEC("lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_s_run, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(lsm_s_run, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    return bpf_link_create_verify(cmd);
    }
    SEC("fentry")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_run) -> c_int {
    int BPF_PROG(fentry_run)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
