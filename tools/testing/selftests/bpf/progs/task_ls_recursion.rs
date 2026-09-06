//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_ls_recursion.c
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
// Copyright (c) 2021 Facebook

pub const EBUSY: c_int = 16;

    char _license[] SEC("license") = "GPL";
    let mut nr_del_errs: c_int = 0;
    let mut test_pid: c_int = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, long);
    } map_a SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, long);
    } map_b SEC(".maps");
    SEC("fentry/bpf_local_storage_update")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_update) -> c_int {
    int BPF_PROG(on_update)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    long *ptr;
    if (!test_pid || task.pid != test_pid)
    return 0;
// This will succeed as there is no real deadlock
    ptr = bpf_task_storage_get(&map_a, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr) {
    int err;
// ptr += 1;
    err = bpf_task_storage_delete(&map_a, task);
    if (err == -EBUSY)
    nr_del_errs++;
    }
// This will succeed as there is no real deadlock
    ptr = bpf_task_storage_get(&map_b, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
// ptr += 1;
    return 0;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_enter, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(on_enter, struct pt_regs *regs, long id)
    {
    struct task_struct *task;
    long *ptr;
    task = bpf_get_current_task_btf();
    if (!test_pid || task.pid != test_pid)
    return 0;
    ptr = bpf_task_storage_get(&map_a, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr && !*ptr)
// ptr = 200;
    ptr = bpf_task_storage_get(&map_b, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr && !*ptr)
// ptr = 100;
    return 0;
    }
