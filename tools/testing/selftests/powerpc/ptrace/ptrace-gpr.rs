//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/ptrace/ptrace-gpr.h
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
pub const GPR_1: c_int = 1;
pub const GPR_2: c_int = 2;
pub const GPR_3: c_int = 3;
pub const GPR_4: c_int = 4;

pub const FPR_1_REP: c_uint = 0x3f50624dd2f1a9fcull;
pub const FPR_2_REP: c_uint = 0x3f60624dd2f1a9fcull;
pub const FPR_3_REP: c_uint = 0x3f689374bc6a7efaull;
pub const FPR_4_REP: c_uint = 0x3f70624dd2f1a9fcull;
// Buffer must have 18 elements
// Buffer must have 32 elements
