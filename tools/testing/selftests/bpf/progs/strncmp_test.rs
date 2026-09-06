//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/strncmp_test.c
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
// Copyright (C) 2021. Huawei Technologies Co., Ltd

pub const STRNCMP_STR_SZ: c_int = 8;
    const char target[STRNCMP_STR_SZ] = "EEEEEEE";
    char str[STRNCMP_STR_SZ];
    let mut cmp_ret: c_int = 0;
    let mut target_pid: c_int = 0;
    const char no_str_target[STRNCMP_STR_SZ] = "12345678";
    char writable_target[STRNCMP_STR_SZ];
    let mut no_const_str_size: c_uint = STRNCMP_STR_SZ;
    char _license[] SEC("license") = "GPL";
    SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn do_strncmp(ctx: *mut c_void) -> c_int {
    int do_strncmp(void *ctx)
    {
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid)
    return 0;
    cmp_ret = bpf_strncmp(str, STRNCMP_STR_SZ, target);
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn strncmp_bad_not_const_str_size(ctx: *mut c_void) -> c_int {
    int strncmp_bad_not_const_str_size(void *ctx)
    {
// The value of string size is not const, so will fail
    cmp_ret = bpf_strncmp(str, no_const_str_size, target);
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn strncmp_bad_writable_target(ctx: *mut c_void) -> c_int {
    int strncmp_bad_writable_target(void *ctx)
    {
// Compared target is not read-only, so will fail
    cmp_ret = bpf_strncmp(str, STRNCMP_STR_SZ, writable_target);
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn strncmp_bad_not_null_term_target(ctx: *mut c_void) -> c_int {
    int strncmp_bad_not_null_term_target(void *ctx)
    {
// Compared target is not null-terminated, so will fail
    cmp_ret = bpf_strncmp(str, STRNCMP_STR_SZ, no_str_target);
    return 0;
    }
