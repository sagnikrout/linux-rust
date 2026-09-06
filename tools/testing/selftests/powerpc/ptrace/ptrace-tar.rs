//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/ptrace/ptrace-tar.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
//
pub const TAR_1: c_int = 10;
pub const TAR_2: c_int = 20;
pub const TAR_3: c_int = 30;
pub const TAR_4: c_int = 40;
pub const TAR_5: c_int = 50;
pub const DSCR_1: c_int = 100;
pub const DSCR_2: c_int = 200;
pub const DSCR_3: c_int = 300;
pub const DSCR_4: c_int = 400;
pub const DSCR_5: c_int = 500;
pub const PPR_1: c_uint = 0x4000000000000         /* or 31,31,31*/;
pub const PPR_2: c_uint = 0x8000000000000         /* or 1,1,1 */;
pub const PPR_3: c_uint = 0xc000000000000         /* or 6,6,6 */;
pub const PPR_4: c_uint = 0x10000000000000        /* or 2,2,2 */;
