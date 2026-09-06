//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_local_storage.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, long);
    } enter_id SEC(".maps");

pub const MAGIC_VALUE: c_uint = 0xabcd1234;
    let mut target_pid: pid_t = 0;
    let mut mismatch_cnt: c_int = 0;
    let mut enter_cnt: c_int = 0;
    let mut exit_cnt: c_int = 0;
    let mut update_err: c_long = 0;
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_enter, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(on_enter, struct pt_regs *regs, long id)
    {
    struct task_struct *task;
    long *ptr;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    ptr = bpf_task_storage_get(&enter_id, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!ptr)
    return 0;
    __sync_fetch_and_add(&enter_cnt, 1);
// ptr = MAGIC_VALUE + enter_cnt;
    return 0;
    }
    SEC("tp_btf/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_exit, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(on_exit, struct pt_regs *regs, long id)
    {
    struct task_struct *task;
    long *ptr;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    ptr = bpf_task_storage_get(&enter_id, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!ptr)
    return 0;
    __sync_fetch_and_add(&exit_cnt, 1);
    if (*ptr != MAGIC_VALUE + exit_cnt)
    __sync_fetch_and_add(&mismatch_cnt, 1);
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
