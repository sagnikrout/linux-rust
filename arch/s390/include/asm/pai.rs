//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pai.h
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
// Processor Activity Instrumentation support for cryptography counters
//
// Copyright IBM Corp. 2022
// Author(s): Thomas Richter <tmricht@linux.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qpaci_info_block {
    pub header: u64,
    pub 8: u64 :,
    pub /: *mut *mut u64 num_cc : 8; / # of supported crypto counters,
    pub 9: u64 :,
    pub /: *mut *mut u64 num_nnpa : 7; / # of supported NNPA counters,
    pub 32: u64 :,
}

// Size of info (in double words minus one)
pub const PAI_CRYPTO_BASE: c_uint = 0x1000	/* First event number */;

pub const PAI_CRYPTO_KERNEL_OFFSET: c_int = 2048;
pub const PAI_NNPA_BASE: c_uint = 0x1800	/* First event number */;

