//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uptr_update_failure.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct value_lock_type);
    } datamap SEC(".maps");
// load test only. not used
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn not_used(ctx: *mut c_void) -> c_int {
    int not_used(void *ctx)
    {
    struct value_lock_type *ptr;
    struct task_struct *task;
    struct user_data *udata;
    task = bpf_get_current_task_btf();
    ptr = bpf_task_storage_get(&datamap, task, 0, 0);
    if (!ptr)
    return 0;
    bpf_spin_lock(&ptr.lock);
    udata = ptr.udata;
    if (!udata) {
    bpf_spin_unlock(&ptr.lock);
    return 0;
    }
    udata.result = MAGIC_VALUE + udata.a + udata.b;
    bpf_spin_unlock(&ptr.lock);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
