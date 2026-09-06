//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/time64.h
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

pub type time64_t = __s64;
pub type timeu64_t = __u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec64 {
    pub /: *mut *mut time64_t tv_sec; / seconds,
    pub /: *mut *mut long tv_nsec; / nanoseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct itimerspec64 {
    pub it_interval: timespec64,
    pub it_value: timespec64,
}

// Parameters used to convert the timespec values:

// Located here for timespec[64]_valid_strict

//
// Limits for settimeofday():
//
// To prevent setting the time close to the wraparound point time setting
// is limited so a reasonable uptime can be accomodated. Uptime of 30 years
// should be really sufficient, which means the cutoff is 2232. At that
// point the cutoff is just a small part of the larger problem.
//

//
// lhs < rhs:  return <0
// lhs == rhs: return 0
// lhs > rhs:  return >0
//
extern "C" {
    pub fn set_normalized_timespec64(ts: *mut timespec64, sec: time64_t, nsec: i64);
}
//
// sub = lhs - rhs, in normalized form
//
// Returns true if the timespec64 is norm, false if denorm:
//
// Dates before 1970 are bogus
// Can't have more nanoseconds then a second
// Disallow values that could overflow ktime_t
// Disallow values which cause overflow issues vs. CLOCK_REALTIME
//
// timespec64_to_ns - Convert timespec64 to nanoseconds
// @ts:		pointer to the timespec64 variable to be converted
//
// Returns the scalar nanosecond representation of the timespec64
// parameter.
//
// Prevent multiplication overflow / underflow
//
// ns_to_timespec64 - Convert nanoseconds to timespec64
// @nsec:	the nanoseconds value to be converted
//
// Returns the timespec64 representation of the nsec parameter.
//
extern "C" {
    pub fn ns_to_timespec64(nsec: i64) -> timespec64;
}
//
// timespec64_add_ns - Adds nanoseconds to a timespec64
// @a:		pointer to timespec64 to be incremented
// @ns:		unsigned nanoseconds value to be added
//
// This must always be inlined because its used from the x86-64 vdso,
// which cannot call other kernel functions.
//
// timespec64_add_safe assumes both values are positive and checks for
// overflow. It will return TIME64_MAX in case of overflow.
//
