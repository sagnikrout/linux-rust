//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/pick.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2026 Intel Corporation
//
// Given the first two numbers __a and __b of arbitrarily many evenly spaced
// numbers, pick the 0-based __index'th value.
//
// Always prefer this over _PICK() if the numbers are evenly spaced.
//

//
// Like _PICK_EVEN(), but supports 2 ranges of evenly spaced address offsets.
// @__c_index corresponds to the index in which the second range starts to be
// used. Using math interval notation, the first range is used for indexes [ 0,
// @__c_index), while the second range is used for [ @__c_index, ... ). Example:
//
// #define _FOO_A			0xf000
// #define _FOO_B			0xf004
// #define _FOO_C			0xf008
// #define _SUPER_FOO_A			0xa000
// #define _SUPER_FOO_B			0xa100
// #define FOO(x)			_MMIO(_PICK_EVEN_2RANGES(x, 3,		\
// _FOO_A, _FOO_B,			\
// _SUPER_FOO_A, _SUPER_FOO_B))
//
// This expands to:
// 0: 0xf000,
// 1: 0xf004,
// 2: 0xf008,
// 3: 0xa000,
// 4: 0xa100,
// 5: 0xa200,
// ...
//

//
// Given the arbitrary numbers in varargs, pick the 0-based __index'th number.
//
// Always prefer _PICK_EVEN() over this if the numbers are evenly spaced.
//

