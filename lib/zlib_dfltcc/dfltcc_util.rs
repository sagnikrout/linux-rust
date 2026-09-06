//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zlib_dfltcc/dfltcc_util.h
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


// SPDX-License-Identifier: Zlib

//
// C wrapper for the DEFLATE CONVERSION CALL instruction.
//
pub const DFLTCC_QAF: c_int = 0;
pub const DFLTCC_GDHT: c_int = 1;
pub const DFLTCC_CMPR: c_int = 2;
pub const DFLTCC_XPND: c_int = 4;

pub const HB_BITS: c_int = 15;

//
// Unpoison the parameter block and the output buffer.
// This is a no-op in non-KMSAN builds.
//
// op1 = t2;
// len1 = t3;
// op2 = t4;
// len2 = t5;
