//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/ctype.h
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
// NOTE! This ctype does not handle EOF like the standard C
// library is required to.
//
pub const _U: c_uint = 0x01	/* upper */;
pub const _L: c_uint = 0x02	/* lower */;
pub const _D: c_uint = 0x04	/* digit */;
pub const _C: c_uint = 0x08	/* cntrl */;
pub const _P: c_uint = 0x10	/* punct */;
pub const _S: c_uint = 0x20	/* white space (space/lf/tab) */;
pub const _X: c_uint = 0x40	/* hex digit */;
pub const _SP: c_uint = 0x80	/* hard space (0x20) */;

// Note: isspace() must return false for %NUL-terminator

//
// Fast implementation of tolower() for internal usage. Do not use in your
// code.
//
// Fast check for octal digit
