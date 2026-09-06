//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crc64.h
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
// crc64_be - Calculate bitwise big-endian ECMA-182 CRC64
// @crc: seed value for computation. 0 or (u64)~0 for a new CRC calculation,
// or the previous crc64 value if computing incrementally.
// @p: pointer to buffer over which CRC64 is run
// @len: length of buffer @p
//
extern "C" {
    pub fn crc64_be(crc: u64, p: *const c_void, len: usize) -> u64;
}
//
// crc64_nvme - Calculate CRC64-NVME
// @crc: seed value for computation. 0 for a new CRC calculation, or the
// previous crc64 value if computing incrementally.
// @p: pointer to buffer over which CRC64 is run
// @len: length of buffer @p
//
// This computes the CRC64 defined in the NVME NVM Command Set Specification,
// *including the bitwise inversion at the beginning and end*.
//
extern "C" {
    pub fn crc64_nvme(crc: u64, p: *const c_void, len: usize) -> u64;
}
