//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/std.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// Standard definitions and types for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// Declare a few quite common macros and types that usually are in stdlib.h,
// stdint.h, ctype.h, unistd.h and a few other common locations. Please place
// integer type definitions and generic macros here, but avoid OS-specific and
// syscall-specific stuff, as this file is expected to be included very early.
//

// those are commonly provided by sys/types.h
pub type dev_t = u64;
pub type ino_t = u64;
pub type mode_t = c_uint;
pub type pid_t = i32;
pub type uid_t = c_uint;
pub type gid_t = c_uint;
pub type nlink_t = c_ulong;
pub type off_t = i64;
pub type blksize_t = signed long;
pub type blkcnt_t = signed long;
pub type time_t = __kernel_time64_t;
