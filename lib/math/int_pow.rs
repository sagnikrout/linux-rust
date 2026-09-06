//! Automatically rewritten from C to Rust
//! Source: lib/math/int_pow.c
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
// An integer based power function
//
// Derived from drivers/video/backlight/pwm_bl.c
//

//
// int_pow - computes the exponentiation of the given base and exponent
// @base: base which will be raised to the given power
// @exp: power to be raised to
//
// Computes: pow(base, exp), i.e. @base raised to the @exp power
//
#[no_mangle]
pub unsafe extern "C" fn int_pow(base: u64, exp: c_uint) -> u64 {
    u64 int_pow(u64 base, unsigned int exp)
    {
    let mut result: u64 = 1;
    while (exp) {
    if (exp & 1)
    result *= base;
    exp >>= 1;
    base *= base;
    }
    return result;
    }
    EXPORT_SYMBOL_GPL(int_pow);
