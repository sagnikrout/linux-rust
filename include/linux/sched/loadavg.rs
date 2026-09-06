//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/loadavg.h
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
// These are the constant used to fake the fixed-point load-average
// counting. Some notes:
// - 11 bit fractions expand to 22 bits by the multiplies: this gives
// a load-average precision of 10 bits integer + 11 bits fractional
// - if you want to count load-averages more often, you need more
// precision, or rounding will get you. With 2-second counting freq,
// the EXP_n values would be 1981, 2034 and 2043 if still using only
// 11 bit fractions.
//
extern "C" {
    pub fn get_avenrun(loads: *mut c_ulong, offset: c_ulong, shift: c_int);
}

//
// a1 = a0 * e + a * (1 - e)
//

extern "C" {
    pub fn calc_global_load();
}
