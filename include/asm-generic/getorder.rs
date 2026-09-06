//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/getorder.h
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
// get_order - Determine the allocation order of a memory size
// @size: The size for which to get the order
//
// Determine the allocation order of a particular sized block of memory.  This
// is on a logarithmic scale, where:
//
// 0 -> 2^0 * PAGE_SIZE and below
// 1 -> 2^1 * PAGE_SIZE to 2^0 * PAGE_SIZE + 1
// 2 -> 2^2 * PAGE_SIZE to 2^1 * PAGE_SIZE + 1
// 3 -> 2^3 * PAGE_SIZE to 2^2 * PAGE_SIZE + 1
// 4 -> 2^4 * PAGE_SIZE to 2^3 * PAGE_SIZE + 1
// ...
//
// The order returned is used to find the smallest allocation granule required
// to hold an object of the specified size.
//
// The result is undefined if the size is 0.
//

extern "C" {
    pub fn fls(_arg: size) -> return;
}

extern "C" {
    pub fn fls64(_arg: size) -> return;
}

