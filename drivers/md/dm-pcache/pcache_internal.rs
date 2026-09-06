//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-pcache/pcache_internal.h
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

// Maximum number of metadata indices
pub const PCACHE_META_INDEX_MAX: c_int = 2;
pub const PCACHE_CRC_SEED: c_uint = 0x3B15A;
//
// struct pcache_meta_header - PCACHE metadata header structure
// @crc: CRC checksum for validating metadata integrity.
// @seq: Sequence number to track metadata updates.
// @version: Metadata version.
// @res: Reserved space for future use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_meta_header {
    pub crc: __u32,
    pub seq: __u8,
    pub version: __u8,
    pub res: __u16,
}

//
// pcache_meta_crc - Calculate CRC for the given metadata header.
// @header: Pointer to the metadata header.
// @meta_size: Size of the metadata structure.
//
// Returns the CRC checksum calculated by excluding the CRC field itself.
//
extern "C" {
    pub fn crc32c(_arg: PCACHE_CRC_SEED, 4: *mut *mut (void )header +, 4: meta_size -) -> return;
}
//
// pcache_meta_seq_after - Check if a sequence number is more recent, accounting for overflow.
// @seq1: First sequence number.
// @seq2: Second sequence number.
//
// Determines if @seq1 is more recent than @seq2 by calculating the signed
// difference between them. This approach allows handling sequence number
// overflow correctly because the difference wraps naturally, and any value
// greater than zero indicates that @seq1 is "after" @seq2. This method
// assumes 8-bit unsigned sequence numbers, where the difference wraps
// around if seq1 overflows past seq2.
//
// Returns:
// - true if @seq1 is more recent than @seq2, indicating it comes "after"
// - false otherwise.
//
// pcache_meta_find_latest - Find the latest valid metadata.
// @header: Pointer to the metadata header.
// @meta_size: Size of each metadata block.
//
// Finds the latest valid metadata by checking sequence numbers. If a
// valid entry with the highest sequence number is found, its pointer
// is returned. Returns NULL if no valid metadata is found.
//
extern "C" {
    pub fn ERR_PTR(_arg: -EIO) -> return;
}
// Skip if CRC check fails, which means corrupted
// Update latest if a more recent sequence is found
extern "C" {
    pub fn ERR_PTR(_arg: -EIO) -> return;
}
