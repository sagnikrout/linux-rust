//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgrp_ls_sleepable.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, long);
    } map_a SEC(".maps");
    __s32 target_pid;
    __u64 cgroup_id;
    long update_err;
    int target_hid;
    bool is_cgroup1;
    struct cgroup *bpf_task_get_cgroup1(struct task_struct *task, int hierarchy_id) __ksym;
    void bpf_cgroup_release(struct cgroup *cgrp) __ksym;
    void bpf_rcu_read_lock(void) __ksym;
    void bpf_rcu_read_unlock(void) __ksym;
    SEC("?iter.s/cgroup")
#[no_mangle]
pub unsafe extern "C" fn cgroup_iter(ctx: *mut bpf_iter__cgroup) -> c_int {
    int cgroup_iter(struct bpf_iter__cgroup *ctx)
    {
    struct cgroup *cgrp = ctx.cgroup;
    long *ptr;
    if (cgrp == core::ptr::null_mut())
    return 0;
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
    cgroup_id = cgrp.kn.id;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __no_rcu_lock(cgrp: *mut cgroup) {
    static void __no_rcu_lock(struct cgroup *cgrp)
    {
    long *ptr;
// Note that trace rcu is held in sleepable prog, so we can use
// bpf_cgrp_storage_get() in sleepable prog.
//
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
    cgroup_id = cgrp.kn.id;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn cgrp1_no_rcu_lock(ctx: *mut c_void) -> c_int {
    int cgrp1_no_rcu_lock(void *ctx)
    {
    struct task_struct *task;
    struct cgroup *cgrp;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
// bpf_task_get_cgroup1 can work in sleepable prog
    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if (!cgrp)
    return 0;
    __no_rcu_lock(cgrp);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn no_rcu_lock(ctx: *mut c_void) -> c_int {
    int no_rcu_lock(void *ctx)
    {
    struct task_struct *task;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
// task->cgroups is untrusted in sleepable prog outside of RCU CS
    __no_rcu_lock(task.cgroups.dfl_cgrp);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn yes_rcu_lock(ctx: *mut c_void) -> c_int {
    int yes_rcu_lock(void *ctx)
    {
    struct task_struct *task;
    struct cgroup *cgrp;
    long *ptr;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    if (is_cgroup1) {
    bpf_rcu_read_lock();
    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if (!cgrp) {
    bpf_rcu_read_unlock();
    return 0;
    }
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
    cgroup_id = cgrp.kn.id;
    bpf_cgroup_release(cgrp);
    bpf_rcu_read_unlock();
    return 0;
    }
    bpf_rcu_read_lock();
    cgrp = task.cgroups.dfl_cgrp;
// cgrp is trusted under RCU CS
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
    cgroup_id = cgrp.kn.id;
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("fexit/bpf_local_storage_update")
    int BPF_PROG(fexit_update, void *owner, struct bpf_local_storage_map *smap,
    void *value, u64 map_flags, bool swap_uptrs,
    struct bpf_local_storage_data *ret)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    if (IS_ERR_VALUE(ret))
    update_err = PTR_ERR(ret);
    return 0;
    }
