//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crc/s390/crc32.h
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
// CRC-32 implemented with the z/Architecture Vector Extension Facility.
//
// Copyright IBM Corp. 2015
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

pub const VX_MIN_LEN: c_int = 64;

//
// DEFINE_CRC32_VX() - Define a CRC-32 function using the vector extension
//
// Creates a function to perform a particular CRC-32 computation. Depending
// on the message buffer, the hardware-accelerated or software implementation
// is used.   Note that the message buffer is aligned to improve fetch
// operations of VECTOR LOAD MULTIPLE instructions.
//
