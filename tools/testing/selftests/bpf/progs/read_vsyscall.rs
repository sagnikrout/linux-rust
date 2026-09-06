//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/read_vsyscall.c
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
// Copyright (C) 2024. Huawei Technologies Co., Ltd

    let mut target_pid: c_int = 0;
    void *user_ptr = 0;
    int read_ret[10];
    char _license[] SEC("license") = "GPL";
//
// These are the kfuncs, the others are helpers
//
    int bpf_copy_from_user_str(void *dst, u32, const void *, u64) __weak __ksym;
    int bpf_copy_from_user_task_str(void *dst, u32, const void *,
    struct task_struct *, u64) __weak __ksym;
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn do_probe_read(ctx: *mut c_void) -> c_int {
    int do_probe_read(void *ctx)
    {
    char buf[8];
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid)
    return 0;
    read_ret[0] = bpf_probe_read_kernel(buf, sizeof(buf), user_ptr);
    read_ret[1] = bpf_probe_read_kernel_str(buf, sizeof(buf), user_ptr);
    read_ret[2] = bpf_probe_read(buf, sizeof(buf), user_ptr);
    read_ret[3] = bpf_probe_read_str(buf, sizeof(buf), user_ptr);
    read_ret[4] = bpf_probe_read_user(buf, sizeof(buf), user_ptr);
    read_ret[5] = bpf_probe_read_user_str(buf, sizeof(buf), user_ptr);
    return 0;
    }
    SEC("fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn do_copy_from_user(ctx: *mut c_void) -> c_int {
    int do_copy_from_user(void *ctx)
    {
    char buf[8];
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid)
    return 0;
    read_ret[6] = bpf_copy_from_user(buf, sizeof(buf), user_ptr);
    read_ret[7] = bpf_copy_from_user_task(buf, sizeof(buf), user_ptr,
    bpf_get_current_task_btf(), 0);
    read_ret[8] = bpf_copy_from_user_str((char *)buf, sizeof(buf), user_ptr, 0);
    read_ret[9] = bpf_copy_from_user_task_str((char *)buf,
    sizeof(buf),
    user_ptr,
    bpf_get_current_task_btf(),
    0);
    return 0;
    }
