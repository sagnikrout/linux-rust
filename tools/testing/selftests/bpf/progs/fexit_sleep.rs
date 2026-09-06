//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fexit_sleep.c
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

    char LICENSE[] SEC("license") = "GPL";
    let mut pid: c_int = 0;
    let mut fentry_cnt: c_int = 0;
    let mut fexit_cnt: c_int = 0;
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn nanosleep_fentry(ctx: *mut c_void) -> c_int {
    int nanosleep_fentry(void *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    fentry_cnt++;
    return 0;
    }
    SEC("fexit/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn nanosleep_fexit(ctx: *mut c_void) -> c_int {
    int nanosleep_fexit(void *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    fexit_cnt++;
    return 0;
    }
