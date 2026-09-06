//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/count_zeros.h
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
// Count leading and trailing zeros functions
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// count_leading_zeros - Count the number of zeros from the MSB back
// @x: The value
//
// Count the number of leading zeros from the MSB going towards the LSB in @x.
//
// If the MSB of @x is set, the result is 0.
// If only the LSB of @x is set, then the result is BITS_PER_LONG-1.
// If @x is 0 then the result is BITS_PER_LONG.
//
// count_trailing_zeros - Count the number of zeros from the LSB forwards
// @x: The value
//
// Count the number of trailing zeros from the LSB going towards the MSB in @x.
//
// If the LSB of @x is set, the result is 0.
// If only the MSB of @x is set, then the result is BITS_PER_LONG-1.
// If @x is 0 then the result is BITS_PER_LONG.
//
