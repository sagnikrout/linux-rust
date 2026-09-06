//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgrp_ls_tp_btf.c
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
    struct {
    __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, long);
    } map_b SEC(".maps");
pub const MAGIC_VALUE: c_uint = 0xabcd1234;
    let mut target_pid: pid_t = 0;
    let mut mismatch_cnt: c_int = 0;
    let mut enter_cnt: c_int = 0;
    let mut exit_cnt: c_int = 0;
    let mut target_hid: c_int = 0;
    let mut is_cgroup1: bool = 0;
    struct cgroup *bpf_task_get_cgroup1(struct task_struct *task, int hierarchy_id) __ksym;
    void bpf_cgroup_release(struct cgroup *cgrp) __ksym;
#[no_mangle]
unsafe extern "C" fn __on_enter(regs: *mut pt_regs, id: c_long, cgrp: *mut cgroup) {
    static void __on_enter(struct pt_regs *regs, long id, struct cgroup *cgrp)
    {
    long *ptr;
    int err;
// populate value 0
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!ptr)
    return;
// delete value 0
    err = bpf_cgrp_storage_delete(&map_a, cgrp);
    if (err)
    return;
// value is not available
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0, 0);
    if (ptr)
    return;
// re-populate the value
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!ptr)
    return;
    __sync_fetch_and_add(&enter_cnt, 1);
// ptr = MAGIC_VALUE + enter_cnt;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_enter, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(on_enter, struct pt_regs *regs, long id)
    {
    struct task_struct *task;
    struct cgroup *cgrp;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    if (is_cgroup1) {
    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if (!cgrp)
    return 0;
    __on_enter(regs, id, cgrp);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    __on_enter(regs, id, task.cgroups.dfl_cgrp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __on_exit(regs: *mut pt_regs, id: c_long, cgrp: *mut cgroup) {
    static void __on_exit(struct pt_regs *regs, long id, struct cgroup *cgrp)
    {
    long *ptr;
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!ptr)
    return;
    __sync_fetch_and_add(&exit_cnt, 1);
    if (*ptr != MAGIC_VALUE + exit_cnt)
    __sync_fetch_and_add(&mismatch_cnt, 1);
    }
    SEC("tp_btf/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_exit, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(on_exit, struct pt_regs *regs, long id)
    {
    struct task_struct *task;
    struct cgroup *cgrp;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    if (is_cgroup1) {
    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if (!cgrp)
    return 0;
    __on_exit(regs, id, cgrp);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    __on_exit(regs, id, task.cgroups.dfl_cgrp);
    return 0;
    }
