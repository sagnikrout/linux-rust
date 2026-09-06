//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgrp_ls_recursion.c
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
    let mut target_hid: c_int = 0;
    let mut is_cgroup1: bool = 0;
    struct cgroup *bpf_task_get_cgroup1(struct task_struct *task, int hierarchy_id) __ksym;
    void bpf_cgroup_release(struct cgroup *cgrp) __ksym;
#[no_mangle]
unsafe extern "C" fn __on_update(cgrp: *mut cgroup) {
    static void __on_update(struct cgroup *cgrp)
    {
    long *ptr;
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
// ptr += 1;
    ptr = bpf_cgrp_storage_get(&map_b, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
// ptr += 1;
    }
    SEC("fentry/bpf_local_storage_update")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_update) -> c_int {
    int BPF_PROG(on_update)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    struct cgroup *cgrp;
    if (is_cgroup1) {
    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if (!cgrp)
    return 0;
    __on_update(cgrp);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    __on_update(task.cgroups.dfl_cgrp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __on_enter(regs: *mut pt_regs, id: c_long, cgrp: *mut cgroup) {
    static void __on_enter(struct pt_regs *regs, long id, struct cgroup *cgrp)
    {
    long *ptr;
    ptr = bpf_cgrp_storage_get(&map_a, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
// ptr = 200;
    ptr = bpf_cgrp_storage_get(&map_b, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
// ptr = 100;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_enter, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(on_enter, struct pt_regs *regs, long id)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    struct cgroup *cgrp;
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
