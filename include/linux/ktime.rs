//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ktime.h
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


//
// include/linux/ktime.h
//
// ktime_t - nanosecond-resolution time format.
//
// Copyright(C) 2005, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005, Red Hat, Inc., Ingo Molnar
//
// data type definitions, declarations, prototypes and macros.
//
// Started by: Thomas Gleixner and Ingo Molnar
//
// Credits:
//
// Roman Zippel provided the ideas and primary code snippets of
// the ktime_t union and further simplifications of the original
// code.
//
// For licencing details see kernel-base/COPYING
//

//
// ktime_set - Set a ktime_t variable from a seconds/nanoseconds value
// @secs:	seconds to set
// @nsecs:	nanoseconds to set
//
// Return: The ktime_t representation of the value.
//
// Subtract two ktime_t variables. rem = lhs -rhs:

// Add two ktime_t variables. res = lhs + rhs:

//
// Same as ktime_add(), but avoids undefined behaviour on overflow; however,
// this means that you must check the result for overflow yourself.
//

//
// Add a ktime_t variable and a scalar nanosecond value.
// res = kt + nsval:
//

//
// Subtract a scalar nanosecod from a ktime_t variable
// res = kt - nsval:
//

// convert a timespec64 to ktime_t format:
extern "C" {
    pub fn ktime_set(_arg: ts.tv_sec, _arg: ts.tv_nsec) -> return;
}
// Map the ktime_t to timespec conversion to ns_to_timespec function

// Convert ktime_t to nanoseconds
//
// ktime_compare - Compares two ktime_t variables for less, greater or equal
// @cmp1:	comparable1
// @cmp2:	comparable2
//
// Return: ...
// cmp1  < cmp2: return <0
// cmp1 == cmp2: return 0
// cmp1  > cmp2: return >0
//
// ktime_after - Compare if a ktime_t value is bigger than another one.
// @cmp1:	comparable1
// @cmp2:	comparable2
//
// Return: true if cmp1 happened after cmp2.
//
// ktime_before - Compare if a ktime_t value is smaller than another one.
// @cmp1:	comparable1
// @cmp2:	comparable2
//
// Return: true if cmp1 happened before cmp2.
//

extern "C" {
    pub fn __ktime_divns(kt: ktime_t, div: i64) -> i64;
}
//
// Negative divisors could cause an inf loop,
// so bug out here.
//
extern "C" {
    pub fn __ktime_divns(_arg: kt, _arg: div) -> return;
}

//
// 32-bit implementation cannot handle negative divisors,
// so catch them on 64bit as well.
//

extern "C" {
    pub fn ktime_divns(_arg: kt, _arg: NSEC_PER_USEC) -> return;
}
extern "C" {
    pub fn ktime_divns(_arg: kt, _arg: NSEC_PER_MSEC) -> return;
}
extern "C" {
    pub fn ktime_to_us(_arg: ktime_sub(later, _arg: earlier)) -> return;
}
extern "C" {
    pub fn ktime_to_ms(_arg: ktime_sub(later, _arg: earlier)) -> return;
}
extern "C" {
    pub fn ktime_add_ns(_arg: kt, NSEC_PER_USEC: *mut *mut usec) -> return;
}
extern "C" {
    pub fn ktime_add_ns(_arg: kt, NSEC_PER_MSEC: *mut *mut msec) -> return;
}
extern "C" {
    pub fn ktime_sub_ns(_arg: kt, NSEC_PER_USEC: *mut *mut usec) -> return;
}
extern "C" {
    pub fn ktime_sub_ns(_arg: kt, NSEC_PER_MSEC: *mut *mut msec) -> return;
}
extern "C" {
    pub fn ktime_add_safe(lhs: ktime_t, rhs: ktime_t) -> ktime_t;
}
//
// ktime_to_timespec64_cond - convert a ktime_t variable to timespec64
// format only if the variable contains data
// @kt:		the ktime_t variable to convert
// @ts:		the timespec variable to store the result in
//
// Return: %true if there was a successful conversion, %false if kt was 0.
//
// ts = ktime_to_timespec64(kt);

