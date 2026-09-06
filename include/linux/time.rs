//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/time.h
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
    pub fn clear_itimer();
}

extern "C" {
    pub fn do_utimes(dfd: c_int, filename: *const char __user, times: *mut timespec64, flags: c_int) -> c_long;
}
//
// Similar to the struct tm in userspace <time.h>, but it needs to be here so
// that the kernel source is self contained.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
//
// the number of seconds after the minute, normally in the range
// 0 to 59, but can be up to 60 to allow for leap seconds
//
    pub tm_sec: c_int,
// the number of minutes after the hour, in the range 0 to 59
    pub tm_min: c_int,
// the number of hours past midnight, in the range 0 to 23
    pub tm_hour: c_int,
// the day of the month, in the range 1 to 31
    pub tm_mday: c_int,
// the number of months since January, in the range 0 to 11
    pub tm_mon: c_int,
// the number of years since 1900
    pub tm_year: c_long,
// the number of days since Sunday, in the range 0 to 6
    pub tm_wday: c_int,
// the number of days since January 1, in the range 0 to 365
    pub tm_yday: c_int,
}

extern "C" {
    pub fn time64_to_tm(totalsecs: time64_t, offset: c_int, result: *mut tm);
}

//
// time_after32 - compare two 32-bit relative times
// @a:	the time which may be after @b
// @b:	the time which may be before @a
//
// time_after32(a, b) returns true if the time @a is after time @b.
// time_before32(b, a) returns true if the time @b is before time @a.
//
// Similar to time_after(), compare two 32-bit timestamps for relative
// times.  This is useful for comparing 32-bit seconds values that can't
// be converted to 64-bit values (e.g. due to disk format or wire protocol
// issues) when it is known that the times are less than 68 years apart.
//

//
// time_between32 - check if a 32-bit timestamp is within a given time range
// @t:	the time which may be within [l,h]
// @l:	the lower bound of the range
// @h:	the higher bound of the range
//
// time_before32(t, l, h) returns true if @l <= @t <= @h. All operands are
// treated as 32-bit integers.
//
// Equivalent to !(time_before32(@t, @l) || time_after32(@t, @h)).
//

