//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_ls_uptr.c
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

    struct task_struct *bpf_task_from_pid(s32 pid) __ksym;
    void bpf_task_release(struct task_struct *p) __ksym;
    void bpf_cgroup_release(struct cgroup *cgrp) __ksym;
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct value_type);
    } datamap SEC(".maps");
    let mut target_pid: pid_t = 0;
    let mut parent_pid: pid_t = 0;
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn on_enter(ctx: *mut __u64) -> c_int {
    int on_enter(__u64 *ctx)
    {
    struct task_struct *task, *data_task;
    struct value_type *ptr;
    struct user_data *udata;
    struct cgroup *cgrp;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    data_task = bpf_task_from_pid(parent_pid);
    if (!data_task)
    return 0;
    ptr = bpf_task_storage_get(&datamap, data_task, 0, 0);
    bpf_task_release(data_task);
    if (!ptr)
    return 0;
    cgrp = bpf_kptr_xchg(&ptr.cgrp, core::ptr::null_mut());
    if (cgrp) {
    let mut lvl: c_int = cgrp.level;
    bpf_cgroup_release(cgrp);
    return lvl;
    }
    udata = ptr.udata;
    if (!udata || udata.result)
    return 0;
    udata.result = MAGIC_VALUE + udata.a + udata.b;
    udata = ptr.nested.udata;
    if (udata && !udata.nested_result)
    udata.nested_result = udata.result;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
