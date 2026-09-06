//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bitops/fls64.h
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
// fls64 - find last set bit in a 64-bit word
// @x: the word to search
//
// This is defined in a similar way as the libc and compiler builtin
// ffsll, but returns the position of the most significant set bit.
//
// fls64(value) returns 0 if value is 0 or the position of the last
// set bit if value is nonzero. The last (most significant) bit is
// at position 64.
//

extern "C" {
    pub fn fls(_arg: x) -> return;
}

