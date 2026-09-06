//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/vdso/common/vclock_gettime.c
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
// Fast user context implementation of clock_gettime, gettimeofday, and time.
//
// Copyright 2006 Andi Kleen, SUSE Labs.
// Copyright 2019 ARM Limited
//
// 32 Bit compat layer by Stefani Seibold <stefani@seibold.net>
// sponsored by Rohde & Schwarz GmbH & Co. KG Munich/Germany
//

#[no_mangle]
pub unsafe extern "C" fn __vdso_gettimeofday(tv: *mut __kernel_old_timeval, tz: *mut timezone) -> c_int {
    int __vdso_gettimeofday(struct __kernel_old_timeval *tv, struct timezone *tz)
    {
    return __cvdso_gettimeofday(tv, tz);
    }
#[no_mangle]
pub unsafe extern "C" fn gettimeofday(: *mut __kernel_old_timeval, : *mut timezone) -> c_int {
    int gettimeofday(struct __kernel_old_timeval *, struct timezone *)
    __attribute__((weak, alias("__vdso_gettimeofday")));
#[no_mangle]
pub unsafe extern "C" fn __vdso_time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    __kernel_old_time_t __vdso_time(__kernel_old_time_t *t)
    {
    return __cvdso_time(t);
    }
    __kernel_old_time_t time(__kernel_old_time_t *t)	__attribute__((weak, alias("__vdso_time")));

// both 64-bit and x32 use these
#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime(clock: clockid_t, ts: *mut __kernel_timespec) -> c_int {
    int __vdso_clock_gettime(clockid_t clock, struct __kernel_timespec *ts)
    {
    return __cvdso_clock_gettime(clock, ts);
    }
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(_arg: clockid_t, : *mut __kernel_timespec) -> c_int {
    int clock_gettime(clockid_t, struct __kernel_timespec *)
    __attribute__((weak, alias("__vdso_clock_gettime")));
    int __vdso_clock_getres(clockid_t clock,
    struct __kernel_timespec *res)
    {
    return __cvdso_clock_getres(clock, res);
    }
#[no_mangle]
pub unsafe extern "C" fn clock_getres(_arg: clockid_t, : *mut __kernel_timespec) -> c_int {
    int clock_getres(clockid_t, struct __kernel_timespec *)
    __attribute__((weak, alias("__vdso_clock_getres")));

// i386 only

#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime(clock: clockid_t, ts: *mut old_timespec32) -> c_int {
    int __vdso_clock_gettime(clockid_t clock, struct old_timespec32 *ts)
    {
    return __cvdso_clock_gettime32(clock, ts);
    }
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(_arg: clockid_t, : *mut old_timespec32) -> c_int {
    int clock_gettime(clockid_t, struct old_timespec32 *)
    __attribute__((weak, alias("__vdso_clock_gettime")));
#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_getres(clock: clockid_t, res: *mut old_timespec32) -> c_int {
    int __vdso_clock_getres(clockid_t clock, struct old_timespec32 *res)
    {
    return __cvdso_clock_getres_time32(clock, res);
    }
#[no_mangle]
pub unsafe extern "C" fn clock_getres(_arg: clockid_t, : *mut old_timespec32) -> c_int {
    int clock_getres(clockid_t, struct old_timespec32 *)
    __attribute__((weak, alias("__vdso_clock_getres")));

#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime64(clock: clockid_t, ts: *mut __kernel_timespec) -> c_int {
    int __vdso_clock_gettime64(clockid_t clock, struct __kernel_timespec *ts)
    {
    return __cvdso_clock_gettime(clock, ts);
    }
#[no_mangle]
pub unsafe extern "C" fn clock_gettime64(_arg: clockid_t, : *mut __kernel_timespec) -> c_int {
    int clock_gettime64(clockid_t, struct __kernel_timespec *)
    __attribute__((weak, alias("__vdso_clock_gettime64")));
#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_getres_time64(clock: clockid_t, ts: *mut __kernel_timespec) -> c_int {
    int __vdso_clock_getres_time64(clockid_t clock, struct __kernel_timespec *ts)
    {
    return __cvdso_clock_getres(clock, ts);
    }
#[no_mangle]
pub unsafe extern "C" fn clock_getres_time64(_arg: clockid_t, : *mut __kernel_timespec) -> c_int {
    int clock_getres_time64(clockid_t, struct __kernel_timespec *)
    __attribute__((weak, alias("__vdso_clock_getres_time64")));
