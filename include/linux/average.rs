//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/average.h
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
// Exponentially weighted moving average (EWMA)
//
// This implements a fixed-precision EWMA algorithm, with both the
// precision and fall-off coefficient determined at compile-time
// and built into the generated helper funtions.
//
// The first argument to the macro is the name that will be used
// for the struct and helper functions.
//
// The second argument, the precision, expresses how many bits are
// used for the fractional part of the fixed-precision values.
//
// The third argument, the weight reciprocal, determines how the
// new values will be weighed vs. the old state, new values will
// get weight 1/weight_rcp and old values 1-1/weight_rcp. Note
// that this parameter must be a power of two for efficiency.
//

// \
// Even if you want to feed it just 0/1 you should have	\
// some bits for the non-fractional part...		\
// \
