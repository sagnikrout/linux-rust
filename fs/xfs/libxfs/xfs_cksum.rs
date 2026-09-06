//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_cksum.h
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
pub const _XFS_CKSUM_H: c_int = 1;

//
// Calculate the intermediate checksum for a buffer that has the CRC field
// inside it.  The offset of the 32bit crc fields is passed as the
// cksum_offset parameter. We do not modify the buffer during verification,
// hence we have to split the CRC calculation across the cksum_offset.
//
// Calculate CRC up to the checksum.
// Skip checksum field
// Calculate the rest of the CRC.
//
// Fast CRC method where the buffer is modified. Callers must have exclusive
// access to the buffer while the calculation takes place.
//
// zero the CRC field
// (__le32 *)(buffer + cksum_offset) = 0;
// single pass CRC calculation for the entire buffer
extern "C" {
    pub fn crc32c(_arg: XFS_CRC_SEED, _arg: buffer, _arg: length) -> return;
}
//
// Convert the intermediate checksum to the final ondisk format.
//
// The CRC32c calculation uses LE format even on BE machines, but returns the
// result in host endian format. Hence we need to byte swap it back to LE format
// so that it is consistent on disk.
//
// Helper to generate the checksum for a buffer.
//
// This modifies the buffer temporarily - callers must have exclusive
// access to the buffer while the calculation takes place.
//
// (__le32 *)(buffer + cksum_offset) = xfs_end_cksum(crc);
//
// Helper to verify the checksum for a buffer.
//
