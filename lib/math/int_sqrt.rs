//! Automatically rewritten from C to Rust
//! Source: lib/math/int_sqrt.c
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
// Copyright (C) 2013 Davidlohr Bueso <davidlohr.bueso@hp.com>
//
// Based on the shift-and-subtract algorithm for computing integer
// square root from Guy L. Steele.
//

//
// int_sqrt - computes the integer square root
// @x: integer of which to calculate the sqrt
//
// Computes: floor(sqrt(x))
//
#[no_mangle]
pub unsafe extern "C" fn int_sqrt(x: c_ulong) -> c_ulong {
    unsigned long int_sqrt(unsigned long x)
    {
    unsigned long b, m, y = 0;
    if (x <= 1)
    return x;
    m = 1UL << (__fls(x) & ~1UL);
    while (m != 0) {
    b = y + m;
    y >>= 1;
    if (x >= b) {
    x -= b;
    y += m;
    }
    m >>= 2;
    }
    return y;
    }
    EXPORT_SYMBOL(int_sqrt);

//
// int_sqrt64 - strongly typed int_sqrt function when minimum 64 bit input
// is expected.
// @x: 64bit integer of which to calculate the sqrt
//
#[no_mangle]
pub unsafe extern "C" fn int_sqrt64(x: u64) -> u32 {
    u32 int_sqrt64(u64 x)
    {
    u64 b, m, y = 0;
    if (x <= ULONG_MAX)
    return int_sqrt((unsigned long) x);
    m = 1ULL << ((fls64(x) - 1) & ~1ULL);
    while (m != 0) {
    b = y + m;
    y >>= 1;
    if (x >= b) {
    x -= b;
    y += m;
    }
    m >>= 2;
    }
    return y;
    }
    EXPORT_SYMBOL(int_sqrt64);
