//! Automatically rewritten from C to Rust
//! Source: lib/math/gcd.c
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


// SPDX-License-Identifier: GPL-2.0-only

//
// This implements the binary GCD algorithm. (Often attributed to Stein,
// but as Knuth has noted, appears in a first-century Chinese math text.)
//
// This is faster than the division-based algorithm even on x86, which
// has decent hardware division.
//
    DEFINE_STATIC_KEY_TRUE(efficient_ffs_key);

// If __ffs is available, the even/odd algorithm benchmarks slower.
#[no_mangle]
unsafe extern "C" fn binary_gcd(a: c_ulong, b: c_ulong) -> c_ulong {
    static unsigned long binary_gcd(unsigned long a, unsigned long b)
    {
    let mut r: c_ulong = a | b;
    b >>= __ffs(b);
    if (b == 1)
    return r & -r;
    for (;;) {
    a >>= __ffs(a);
    if (a == 1)
    return r & -r;
    if (a == b)
    return a << __ffs(r);
    if (a < b)
    swap(a, b);
    a -= b;
    }
    }

// If normalization is done by loops, the even/odd algorithm is a win.
//
// gcd - calculate and return the greatest common divisor of 2 unsigned longs
// @a: first value
// @b: second value
//
#[no_mangle]
pub unsafe extern "C" fn gcd(a: c_ulong, b: c_ulong) -> c_ulong {
    unsigned long gcd(unsigned long a, unsigned long b)
    {
    let mut r: c_ulong = a | b;
    if (!a || !b)
    return r;

    if (static_branch_likely(&efficient_ffs_key))
    return binary_gcd(a, b);

// Isolate lsbit of r
    r &= -r;
    while (!(b & r))
    b >>= 1;
    if (b == r)
    return r;
    for (;;) {
    while (!(a & r))
    a >>= 1;
    if (a == r)
    return r;
    if (a == b)
    return a;
    if (a < b)
    swap(a, b);
    a -= b;
    a >>= 1;
    if (a & r)
    a += b;
    a >>= 1;
    }
    }
    EXPORT_SYMBOL_GPL(gcd);
