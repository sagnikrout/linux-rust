//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crc32poly.h
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
// The polynomial used by crc32_le(), in integer form.  See crc32_le().
pub const CRC32_POLY_LE: c_uint = 0xedb88320;
// The polynomial used by crc32_be(), in integer form.  See crc32_be().
pub const CRC32_POLY_BE: c_uint = 0x04c11db7;
// The polynomial used by crc32c(), in integer form.  See crc32c().
pub const CRC32C_POLY_LE: c_uint = 0x82f63b78;
