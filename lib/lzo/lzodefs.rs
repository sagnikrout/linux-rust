//! Automatically rewritten from C Header to Rust Module
//! Source: lib/lzo/lzodefs.h
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
// lzodefs.h -- architecture, OS and compiler specific defines
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
// Version
// 0: original lzo version
// 1: lzo with support for RLE
//
pub const LZO_VERSION: c_int = 1;

pub const LZO_USE_CTZ64: c_int = 1;
pub const LZO_USE_CTZ32: c_int = 1;
// Macro flag: #define LZO_FAST_64BIT_MEMORY_ACCESS

pub const LZO_USE_CTZ32: c_int = 1;

pub const LZO_USE_CTZ32: c_int = 1;

pub const M1_MAX_OFFSET: c_uint = 0x0400;
pub const M2_MAX_OFFSET: c_uint = 0x0800;
pub const M3_MAX_OFFSET: c_uint = 0x4000;
pub const M4_MAX_OFFSET_V0: c_uint = 0xbfff;
pub const M4_MAX_OFFSET_V1: c_uint = 0xbffe;
pub const M1_MIN_LEN: c_int = 2;
pub const M1_MAX_LEN: c_int = 2;
pub const M2_MIN_LEN: c_int = 3;
pub const M2_MAX_LEN: c_int = 8;
pub const M3_MIN_LEN: c_int = 3;
pub const M3_MAX_LEN: c_int = 33;
pub const M4_MIN_LEN: c_int = 3;
pub const M4_MAX_LEN: c_int = 9;
pub const M1_MARKER: c_int = 0;
pub const M2_MARKER: c_int = 64;
pub const M3_MARKER: c_int = 32;
pub const M4_MARKER: c_int = 16;
pub const MIN_ZERO_RUN_LENGTH: c_int = 4;

pub const D_BITS: c_int = 13;

