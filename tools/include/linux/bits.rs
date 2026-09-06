//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/bits.h
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

pub const BITS_PER_BYTE: c_int = 8;

//
// Create a contiguous bitmask starting at bit position @l and ending at
// position @h. For example
// GENMASK_ULL(39, 21) gives us the 64bit vector 0x000000ffffe00000.
//

//
// Missing asm support
//
// GENMASK_U*() and BIT_U*() depend on BITS_PER_TYPE() which relies on sizeof(),
// something not available in asm. Nevertheless, fixed width integers is a C
// concept. Assembly code can rely on the long and long long versions instead.
//

//
// Generate a mask for the specified type @t. Additional checks are made to
// guarantee the value returned fits in that type, relying on
// -Wshift-count-overflow compiler check to detect incompatible arguments.
// For example, all these create build errors or warnings:
//
// - GENMASK(15, 20): wrong argument order
// - GENMASK(72, 15): doesn't fit unsigned long
// - GENMASK_U32(33, 15): doesn't fit in a u32
//

//
// Fixed-type variants of BIT(), with additional checks like GENMASK_TYPE(). The
// following examples generate compiler warnings due to -Wshift-count-overflow:
//
// - BIT_U8(8)
// - BIT_U32(-1)
// - BIT_U32(40)
//

//
// BUILD_BUG_ON_ZERO is not available in h files included from asm files,
// disable the input check if that is the case.
//

