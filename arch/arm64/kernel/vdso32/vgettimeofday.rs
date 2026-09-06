//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/vdso32/vgettimeofday.c
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
//
// ARM64 compat userspace implementations of gettimeofday() and similar.
//
// Copyright (C) 2018 ARM Limited
//
// Macro flag: #define BUILD_VDSO32_64

    int __vdso_clock_gettime(clockid_t clock,
    struct old_timespec32 *ts)
    {
    return __cvdso_clock_gettime32(clock, ts);
    }
    int __vdso_clock_getres(clockid_t clock_id,
    struct old_timespec32 *res)
    {
    return __cvdso_clock_getres_time32(clock_id, res);
    }
    int __vdso_gettimeofday(struct __kernel_old_timeval *tv,
    struct timezone *tz)
    {
    return __cvdso_gettimeofday(tv, tz);
    }

    int __vdso_clock_gettime64(clockid_t clock,
    struct __kernel_timespec *ts)
    {
    return __cvdso_clock_gettime(clock, ts);
    }
#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_getres_time64(clock_id: clockid_t, res: *mut __kernel_timespec) -> c_int {
    int __vdso_clock_getres_time64(clockid_t clock_id, struct __kernel_timespec *res)
    {
    return __cvdso_clock_getres(clock_id, res);
    }
// Avoid unresolved references emitted by GCC
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {
    void __aeabi_unwind_cpp_pr0(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr1() {
    void __aeabi_unwind_cpp_pr1(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr2() {
    void __aeabi_unwind_cpp_pr2(void)
    {
    }
