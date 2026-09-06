//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lzo.h
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
// LZO Public Kernel Interface
// A mini subset of the LZO real-time data compression library
//
// Copyright (C) 1996-2012 Markus F.X.J. Oberhumer <markus@oberhumer.com>
//
// The full LZO package can be found at:
// http://www.oberhumer.com/opensource/lzo
//
// Changed for Linux kernel use by:
// Nitin Gupta <nitingupta910@gmail.com>
// Richard Purdie <rpurdie@openedhand.com>
//

// This requires 'wrkmem' of size LZO1X_1_MEM_COMPRESS
// Same as above but does not write more than dst_len to dst.
// This requires 'wrkmem' of size LZO1X_1_MEM_COMPRESS
// Same as above but does not write more than dst_len to dst.
// safe decompression with overrun testing
//
// Return values (< 0 = Error)
//
pub const LZO_E_OK: c_int = 0;

