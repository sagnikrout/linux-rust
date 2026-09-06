//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/log2.h
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
// Determine whether some value is a power of two, where zero is
// *not* considered a power of two.
//
// round up to nearest power of two
//
// round down to nearest power of two
//
// ilog2 - log of base 2 of 32-bit or a 64-bit unsigned value
// @n - parameter
//
// constant-capable log of base 2 calculation
// - this can be used to initialise global variables from constant data, hence
// the massive ternary operator construction
//
// selects the appropriately-sized optimised version depending on sizeof(n)
//

//
// roundup_pow_of_two - round the given value up to nearest power of two
// @n - parameter
//
// round the given value up to the nearest power of two
// - the result is undefined when n == 0
// - this can be used to initialise global variables from constant data
//

//
// rounddown_pow_of_two - round the given value down to nearest power of two
// @n - parameter
//
// round the given value down to the nearest power of two
// - the result is undefined when n == 0
// - this can be used to initialise global variables from constant data
//

