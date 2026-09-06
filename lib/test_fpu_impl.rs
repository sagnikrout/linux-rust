//! Automatically rewritten from C to Rust
//! Source: lib/test_fpu_impl.c
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


// SPDX-License-Identifier: GPL-2.0+

#[no_mangle]
pub unsafe extern "C" fn test_fpu() -> c_int {
    int test_fpu(void)
    {
//
// This sequence of operations tests that rounding mode is
// to nearest and that denormal numbers are supported.
// Volatile variables are used to avoid compiler optimizing
// the calculations away.
//
    volatile double a, b, c, d, e, f, g;
    a = 4.0;
    b = 1e-15;
    c = 1e-310;
// Sets precision flag
    d = a + b;
// Result depends on rounding mode
    e = a + b / 2;
// Denormal and very large values
    f = b / c;
// Depends on denormal support
    g = a + c * f;
    if (d > a && e > a && g > a)
    return 0;
    else
    return -EINVAL;
    }
