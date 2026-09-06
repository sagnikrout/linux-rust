//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/checksum.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2016 ARM Ltd.
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

//
// turns a 32-bit partial checksum (e.g. from csum_partial) into a
// 1's complement 16-bit checksum.
//
// swap the two 16-bit halves of sum
// if there is a carry from adding the two 16-bit halves,
// it will carry from the lower half into the upper half,
// giving us the correct sum in the upper half.
//

//
// This is a version of ip_compute_csum() optimized for IP headers,
// which always checksum on 4 octet boundaries.  ihl is the number
// of 32-bit words and is always >= 5.
//
extern "C" {
    pub fn csum_fold(32): ( __wsum)(sum >>) -> return;
}

extern "C" {
    pub fn do_csum(buff: *const c_uchar, len: c_int) -> c_uint;
}

