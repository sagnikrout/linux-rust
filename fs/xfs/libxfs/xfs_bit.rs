//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_bit.h
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
// Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// XFS bit manipulation routines.
//
// masks with n high/low bits set, 64-bit values
//
// Get high bit set out of 32-bit argument, -1 if none set
// Get high bit set out of 64-bit argument, -1 if none set
// Get low bit set out of 32-bit argument, -1 if none set
// Get low bit set out of 64-bit argument, -1 if none set
// Return whether bitmap is empty (1 == empty)
extern "C" {
    pub fn xfs_bitmap_empty(map: *mut c_uint, size: c_uint) -> c_int;
}
// Count continuous one bits in map starting with start_bit
extern "C" {
    pub fn xfs_contig_bits(map: *mut c_uint, size: c_uint, start_bit: c_uint) -> c_int;
}
// Find next set bit in map
extern "C" {
    pub fn xfs_next_bit(map: *mut c_uint, size: c_uint, start_bit: c_uint) -> c_int;
}
