//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/log2.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Integer base 2 logarithm calculation
//
// Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// non-constant log of base 2 calculators
// - the arch may override these in asm/bitops.h if they can be implemented
// more efficiently than using fls() and fls64()
// - the arch is not required to handle n==0 if implementing the fallback
//

//
// is_power_of_2() - check if a value is a power of two
// @n: the value to check
//
// Determine whether some value is a power of two, where zero is
// *not* considered a power of two.
// Return: true if @n is a power of 2, otherwise false.
//
// __roundup_pow_of_two() - round up to nearest power of two
// @n: value to round up
//
// __rounddown_pow_of_two() - round down to nearest power of two
// @n: value to round down
//
// const_ilog2 - log base 2 of 32-bit or a 64-bit constant unsigned value
// @n: parameter
//
// Use this where sparse expects a true constant expression, e.g. for array
// indices.
//

//
// ilog2 - log base 2 of 32-bit or a 64-bit unsigned value
// @n: parameter
//
// constant-capable log of base 2 calculation
// - this can be used to initialise global variables from constant data, hence
// the massive ternary operator construction
//
// selects the appropriately-sized optimised version depending on sizeof(n)
//

//
// roundup_pow_of_two - round the given value up to nearest power of two
// @n: parameter
//
// round the given value up to the nearest power of two
// - the result is undefined when n == 0
// - this can be used to initialise global variables from constant data
//

//
// rounddown_pow_of_two - round the given value down to nearest power of two
// @n: parameter
//
// round the given value down to the nearest power of two
// - the result is undefined when n == 0
// - this can be used to initialise global variables from constant data
//

//
// order_base_2 - calculate the (rounded up) base 2 order of the argument
// @n: parameter
//
// The first few values calculated by this routine:
// ob2(0) = 0
// ob2(1) = 0
// ob2(2) = 1
// ob2(3) = 2
// ob2(4) = 2
// ob2(5) = 3
// ... and so on.
//

extern "C" {
    pub fn order_base_2(_arg: n) -> return;
}
//
// bits_per - calculate the number of bits required for the argument
// @n: parameter
//
// This is constant-capable and can be used for compile time
// initializations, e.g bitfields.
//
// The first few values calculated by this routine:
// bf(0) = 1
// bf(1) = 1
// bf(2) = 2
// bf(3) = 2
// bf(4) = 3
// ... and so on.
//

//
// max_pow_of_two_factor - return highest power-of-2 factor
// @n: parameter
//
// find highest power-of-2 which is evenly divisible into n.
// 0 is returned for n == 0 or 1.
//
