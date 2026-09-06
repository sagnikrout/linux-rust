//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crc32.h
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
// crc32_le() - Compute least-significant-bit-first IEEE CRC-32
// @crc: Initial CRC value.  ~0 (recommended) or 0 for a new CRC computation, or
// the previous CRC value if computing incrementally.
// @p: Pointer to the data buffer
// @len: Length of data in bytes
//
// This implements the CRC variant that is often known as the IEEE CRC-32, or
// simply CRC-32, and is widely used in Ethernet and other applications:
//
// - Polynomial: x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 +
// x^7 + x^5 + x^4 + x^2 + x^1 + x^0
// - Bit order: Least-significant-bit-first
// - Polynomial in integer form: 0xedb88320
//
// This does *not* invert the CRC at the beginning or end.  The caller is
// expected to do that if it needs to.  Inverting at both ends is recommended.
//
// For new applications, prefer to use CRC-32C instead.  See crc32c().
//
// Context: Any context
// Return: The new CRC value
//
extern "C" {
    pub fn crc32_le(crc: u32, p: *const c_void, len: usize) -> u32;
}
// This is just an alias for crc32_le().
extern "C" {
    pub fn crc32_le(_arg: crc, _arg: p, _arg: len) -> return;
}
//
// crc32_be() - Compute most-significant-bit-first IEEE CRC-32
// @crc: Initial CRC value.  ~0 (recommended) or 0 for a new CRC computation, or
// the previous CRC value if computing incrementally.
// @p: Pointer to the data buffer
// @len: Length of data in bytes
//
// crc32_be() is the same as crc32_le() except that crc32_be() computes the
// *most-significant-bit-first* variant of the CRC.  I.e., within each byte, the
// most significant bit is processed first (treated as highest order polynomial
// coefficient).  The same bit order is also used for the CRC value itself:
//
// - Polynomial: x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 +
// x^7 + x^5 + x^4 + x^2 + x^1 + x^0
// - Bit order: Most-significant-bit-first
// - Polynomial in integer form: 0x04c11db7
//
// Context: Any context
// Return: The new CRC value
//
extern "C" {
    pub fn crc32_be(crc: u32, p: *const c_void, len: usize) -> u32;
}
//
// crc32c() - Compute CRC-32C
// @crc: Initial CRC value.  ~0 (recommended) or 0 for a new CRC computation, or
// the previous CRC value if computing incrementally.
// @p: Pointer to the data buffer
// @len: Length of data in bytes
//
// This implements CRC-32C, i.e. the Castagnoli CRC.  This is the recommended
// CRC variant to use in new applications that want a 32-bit CRC.
//
// - Polynomial: x^32 + x^28 + x^27 + x^26 + x^25 + x^23 + x^22 + x^20 + x^19 +
// x^18 + x^14 + x^13 + x^11 + x^10 + x^9 + x^8 + x^6 + x^0
// - Bit order: Least-significant-bit-first
// - Polynomial in integer form: 0x82f63b78
//
// This does *not* invert the CRC at the beginning or end.  The caller is
// expected to do that if it needs to.  Inverting at both ends is recommended.
//
// Context: Any context
// Return: The new CRC value
//
extern "C" {
    pub fn crc32c(crc: u32, p: *const c_void, len: usize) -> u32;
}
//
// crc32_optimizations() returns flags that indicate which CRC32 library
// functions are using architecture-specific optimizations.  Unlike
// IS_ENABLED(CONFIG_CRC32_ARCH) it takes into account the different CRC32
// variants and also whether any needed CPU features are available at runtime.
//

extern "C" {
    pub fn crc32_optimizations() -> u32;
}

//
// Helpers for hash table generation of ethernet nics:
//
// Ethernet sends the least significant bit of a byte first, thus crc32_le
// is used. The output of crc32_le is bit reversed [most significant bit
// is in bit nr 0], thus it must be reversed before use. Except for
// nics that bit swap the result internally...
//

