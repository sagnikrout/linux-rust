//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bitsperlong.h
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

pub const BITS_PER_LONG: c_int = 64;

pub const BITS_PER_LONG: c_int = 32;

//
// FIXME: The check currently breaks x86-64 build, so it's
// temporarily disabled. Please fix x86-64 and reenable
//

pub const BITS_PER_LONG_LONG: c_int = 64;

//
// small_const_nbits(n) is true precisely when it is known at compile-time
// that BITMAP_SIZE(n) is 1, i.e. 1 <= n <= BITS_PER_LONG. This allows
// various bit/bitmap APIs to provide a fast inline implementation. Bitmaps
// of size 0 are very rare, and a compile-time-known-size 0 is most likely
// a sign of error. They will be handled correctly by the bit/bitmap APIs,
// but using the out-of-line functions, so that the inline implementations
// can unconditionally dereference the pointer(s).
//

