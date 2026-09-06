//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/pt_log2.h
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
// Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
//
// Helper macros for working with log2 values
//

// Compute a

// Compute a - 1 (aka all low bits set)

// Compute a / b

//
// Compute:
// a / c == b / c
// aka the high bits are equal
//

// Compute a % b

//
// Compute:
// a % b == b - 1
// aka the low bits are all 1s
//

//
// Return a value such that:
// a / b == ret / b
// ret % b == val
// aka set the low bits to val. val must be < b
//

// Return a value such that:
// a / b == ret / b
// ret % b == b - 1
// aka set the low bits to all 1s
//

// Compute a * b

//
// Return the highest value such that:
// fls_t(u32, 0) == 0
// fls_t(u3, 1) == 1
// a >= log2_to_int(ret - 1)
// aka find last set bit
//
extern "C" {
    pub fn fls(_arg: a) -> return;
}

//
// Return the highest value such that:
// ffs_t(u32, 0) == UNDEFINED
// ffs_t(u32, 1) == 0
// log_mod(a, ret) == 0
// aka find first set bit
//
extern "C" {
    pub fn __ffs(_arg: a) -> return;
}

//
// Return the highest value such that:
// ffz_t(u32, U32_MAX) == UNDEFINED
// ffz_t(u32, 0) == 0
// ffz_t(u32, 1) == 1
// log_mod(a, ret) == log_to_max_int(ret)
// aka find first zero bit
//
extern "C" {
    pub fn ffz(_arg: a) -> return;
}
extern "C" {
    pub fn ffz(_arg: a) -> return;
}
extern "C" {
    pub fn ffz32(_arg: a) -> return;
}

