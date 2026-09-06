//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/vdso/vgettimeofday.c
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
// ARM64 userspace implementations of gettimeofday() and similar.
//
// Copyright (C) 2018 ARM Limited
//
    int __kernel_clock_gettime(clockid_t clock, struct __kernel_timespec *ts);
    int __kernel_gettimeofday(struct __kernel_old_timeval *tv, struct timezone *tz);
    int __kernel_clock_getres(clockid_t clock_id, struct __kernel_timespec *res);
    int __kernel_clock_gettime(clockid_t clock,
    struct __kernel_timespec *ts)
    {
    return __cvdso_clock_gettime(clock, ts);
    }
    int __kernel_gettimeofday(struct __kernel_old_timeval *tv,
    struct timezone *tz)
    {
    return __cvdso_gettimeofday(tv, tz);
    }
    int __kernel_clock_getres(clockid_t clock_id,
    struct __kernel_timespec *res)
    {
    return __cvdso_clock_getres(clock_id, res);
    }
