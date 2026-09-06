//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/prime_numbers.h
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
    pub fn is_prime_number(x: c_ulong) -> bool;
}
extern "C" {
    pub fn next_prime_number(x: c_ulong) -> c_ulong;
}
//
// for_each_prime_number - iterate over each prime upto a value
// @prime: the current prime number in this iteration
// @max: the upper limit
//
// Starting from the first prime number 2 iterate over each prime number up to
// the @max value. On each iteration, @prime is set to the current prime number.
// @max should be less than ULONG_MAX to ensure termination. To begin with
// @prime set to 1 on the first iteration use for_each_prime_number_from()
// instead.
//

//
// for_each_prime_number_from - iterate over each prime upto a value
// @prime: the current prime number in this iteration
// @from: the initial value
// @max: the upper limit
//
// Starting from @from iterate over each successive prime number up to the
// @max value. On each iteration, @prime is set to the current prime number.
// @max should be less than ULONG_MAX, and @from less than @max, to ensure
// termination.
//

