//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_local_storage_exit_creds.c
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
    __type(value, __u64);
    } task_storage SEC(".maps");
    let mut run_count: c_int = 0;
    let mut valid_ptr_count: c_int = 0;
    let mut null_ptr_count: c_int = 0;
    SEC("fentry/exit_creds")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_exit_creds, task: *mut task_struct) -> c_int {
    int BPF_PROG(trace_exit_creds, struct task_struct *task)
    {
    __u64 *ptr;
    ptr = bpf_task_storage_get(&task_storage, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (ptr)
    __sync_fetch_and_add(&valid_ptr_count, 1);
    else
    __sync_fetch_and_add(&null_ptr_count, 1);
    __sync_fetch_and_add(&run_count, 1);
    return 0;
    }
