//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wordpart.h
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
// upper_32_bits - return bits 32-63 of a number
// @n: the number we're accessing
//
// A basic shift-right of a 64- or 32-bit quantity.  Use this to suppress
// the "right shift count >= width of type" warning when that quantity is
// 32-bits.
//

//
// lower_32_bits - return bits 0-31 of a number
// @n: the number we're accessing
//

//
// upper_16_bits - return bits 16-31 of a number
// @n: the number we're accessing
//

//
// lower_16_bits - return bits 0-15 of a number
// @n: the number we're accessing
//

//
// REPEAT_BYTE - repeat the value @x multiple times as an unsigned long value
// @x: value to repeat
//
// NOTE: @x is not checked for > 0xff; larger values produce odd results.
//

//
// REPEAT_BYTE_U32 - repeat the value @x multiple times as a u32 value
// @x: value to repeat
//
// NOTE: @x is not checked for > 0xff; larger values produce odd results.
//

// Set bits in the first 'n' bytes when loaded from memory

