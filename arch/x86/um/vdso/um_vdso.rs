//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/vdso/um_vdso.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
//
// This vDSO turns all calls into a syscall so that UML can trap them.
//
// Disable profiling for userspace code
// Macro flag: #define DISABLE_BRANCH_PROFILING

#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime(clock: clockid_t, ts: *mut __kernel_timespec) -> c_int {
    int __vdso_clock_gettime(clockid_t clock, struct __kernel_timespec *ts)
    {
    long ret;
    asm("syscall"
    : "=a" (ret)
    : "0" (__NR_clock_gettime), "D" (clock), "S" (ts)
    : "rcx", "r11", "memory");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(_arg: clockid_t, : *mut __kernel_timespec) -> c_int {
    int clock_gettime(clockid_t, struct __kernel_timespec *)
    __attribute__((weak, alias("__vdso_clock_gettime")));
#[no_mangle]
pub unsafe extern "C" fn __vdso_gettimeofday(tv: *mut __kernel_old_timeval, tz: *mut timezone) -> c_int {
    int __vdso_gettimeofday(struct __kernel_old_timeval *tv, struct timezone *tz)
    {
    long ret;
    asm("syscall"
    : "=a" (ret)
    : "0" (__NR_gettimeofday), "D" (tv), "S" (tz)
    : "rcx", "r11", "memory");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn gettimeofday(: *mut __kernel_old_timeval, : *mut timezone) -> c_int {
    int gettimeofday(struct __kernel_old_timeval *, struct timezone *)
    __attribute__((weak, alias("__vdso_gettimeofday")));
#[no_mangle]
pub unsafe extern "C" fn __vdso_time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    __kernel_old_time_t __vdso_time(__kernel_old_time_t *t)
    {
    long secs;
    asm volatile("syscall"
    : "=a" (secs)
    : "0" (__NR_time), "D" (t) : "cc", "r11", "cx", "memory");
    return secs;
    }
    __kernel_old_time_t time(__kernel_old_time_t *t) __attribute__((weak, alias("__vdso_time")));
