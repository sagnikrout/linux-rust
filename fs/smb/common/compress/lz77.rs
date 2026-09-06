//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/compress/lz77.h
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
// Copyright (C) 2024-2026, SUSE LLC
//
// Authors: Enzo Matsumiya <ematsumiya@suse.de>
//
// Implementation of the LZ77 "plain" compression algorithm, as per MS-XCA spec.
//

//
// smb_lz77_compressed_alloc_size() - Compute compressed buffer size.
// @size:	uncompressed (src) size
//
// Compute allocation size for the compressed buffer based on uncompressed size.
// Accounts for metadata and overprovision for the worst case scenario.
//
// LZ77 metadata is a 4-byte flag that is written:
// - on dst begin (pos 0)
// - every 32 literals or matches
// - on end-of-stream (possibly, if last write was another flag)
//
// Worst case scenario is an all-literal compression, which means:
// metadata bytes = 4 + ((@size / 32) * 4) + 4, or, simplified, (@size >> 3) + 8
//
// The worst case scenario rarely happens, but such overprovisioning also
// allows smb_lz77_compress() main loop to run without ever bound checking dst,
// which is a huge perf improvement, while also being safe when compression goes
// bad.
//
// Return: required (*) allocation size for compressed buffer.
//
// (*) checked once in the beginning of smb_lz77_compress()
//
extern "C" {
    pub fn smb_lz77_compress(src: *const c_void, slen: u32, dst: *mut c_void, dlen: *mut u32) -> c_int;
}
