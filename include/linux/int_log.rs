//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/int_log.h
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


// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Provides fixed-point logarithm operations.
//
// Copyright (C) 2006 Christoph Pfister (christophpfister@gmail.com)
//

//
// intlog2 - computes log2 of a value; the result is shifted left by 24 bits
//
// @value: The value (must be != 0)
//
// to use rational values you can use the following method:
//
// intlog2(value) = intlog2(value * 2^x) - x * 2^24
//
// Some usecase examples:
//
// intlog2(8) will give 3 << 24 = 3 * 2^24
//
// intlog2(9) will give 3 << 24 + ... = 3.16... * 2^24
//
// intlog2(1.5) = intlog2(3) - 2^24 = 0.584... * 2^24
//
// return: log2(value) * 2^24
//
extern "C" {
    pub fn intlog2(value: u32) -> c_uint;
}
//
// intlog10 - computes log10 of a value; the result is shifted left by 24 bits
//
// @value: The value (must be != 0)
//
// to use rational values you can use the following method:
//
// intlog10(value) = intlog10(value * 10^x) - x * 2^24
//
// An usecase example:
//
// intlog10(1000) will give 3 << 24 = 3 * 2^24
//
// due to the implementation intlog10(1000) might be not exactly 3 * 2^24
//
// look at intlog2 for similar examples
//
// return: log10(value) * 2^24
//
extern "C" {
    pub fn intlog10(value: u32) -> c_uint;
}
