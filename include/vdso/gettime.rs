//! Automatically rewritten from C Header to Rust Module
//! Source: include/vdso/gettime.h
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

extern "C" {
    pub fn __vdso_clock_getres(clock: clockid_t, res: *mut old_timespec32) -> c_int;
}
extern "C" {
    pub fn __vdso_clock_gettime(clock: clockid_t, ts: *mut old_timespec32) -> c_int;
}

extern "C" {
    pub fn __vdso_clock_getres(clock: clockid_t, res: *mut __kernel_timespec) -> c_int;
}
extern "C" {
    pub fn __vdso_clock_gettime(clock: clockid_t, ts: *mut __kernel_timespec) -> c_int;
}

extern "C" {
    pub fn __vdso_time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t;
}
extern "C" {
    pub fn __vdso_gettimeofday(tv: *mut __kernel_old_timeval, tz: *mut timezone) -> c_int;
}
extern "C" {
    pub fn __vdso_clock_gettime64(clock: clockid_t, ts: *mut __kernel_timespec) -> c_int;
}
extern "C" {
    pub fn __vdso_clock_getres_time64(clock: clockid_t, ts: *mut __kernel_timespec) -> c_int;
}
