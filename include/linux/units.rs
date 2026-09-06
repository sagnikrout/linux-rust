//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/units.h
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

// Metric prefixes in accordance with Système international (d'unités)

//
// Percentage and related scaling units
//
// These macros define scaling factors used to convert between ratio and
// percentage-based representations with different decimal resolutions.
// They are used for precise fractional calculations in engineering, finance,
// and measurement applications.
//
// Examples:
// 1%     = 0.01    = 1 / PERCENT
// 0.1%   = 0.001   = 1 / PERMILLE
// 0.01%  = 0.0001  = 1 / PERMYRIAD (1 basis point)
// 0.001% = 0.00001 = 1 / PERCENTMILLE
//
pub const PERCENT: c_int = 100;
pub const PERMILLE: c_int = 1000;
pub const PERMYRIAD: c_int = 10000;
pub const PERCENTMILLE: c_int = 100000;

// Hz based multipliers

// kHz based multipliers

pub const MILLIDEGREE_PER_DEGREE: c_int = 1000;
pub const MILLIDEGREE_PER_DECIDEGREE: c_int = 100;
extern "C" {
    pub fn milli_kelvin_to_millicelsius(MILLIDEGREE_PER_DEGREE: *mut *mut t) -> return;
}
extern "C" {
    pub fn DIV_ROUND_CLOSEST(_arg: t, _arg: MILLIDEGREE_PER_DEGREE) -> return;
}
extern "C" {
    pub fn DIV_ROUND_CLOSEST(_arg: t, _arg: MILLIDEGREE_PER_DEGREE) -> return;
}
extern "C" {
    pub fn DIV_ROUND_CLOSEST(_arg: t, _arg: MILLIDEGREE_PER_DECIDEGREE) -> return;
}
//
// deci_kelvin_to_millicelsius_with_offset - convert Kelvin to Celsius
// @t: temperature value in decidegrees Kelvin
// @offset: difference between Kelvin and Celsius in millidegrees
//
// Return: temperature value in millidegrees Celsius
//
extern "C" {
    pub fn milli_kelvin_to_millicelsius(MILLIDEGREE_PER_DECIDEGREE: *mut *mut t) -> return;
}
extern "C" {
    pub fn DIV_ROUND_CLOSEST(_arg: t, _arg: MILLIDEGREE_PER_DECIDEGREE) -> return;
}
