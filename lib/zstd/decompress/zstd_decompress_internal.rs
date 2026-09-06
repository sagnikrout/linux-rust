//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/decompress/zstd_decompress_internal.h
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// zstd_decompress_internal:
// objects and definitions shared within lib/decompress modules
// -
// Dependencies
//

// -
// Constants
//
// -
// Decompression types
//

pub const ZSTD_HUFFDTABLE_CAPACITY_LOG: c_int = 12;
// Hashset for storing references to multiple ZSTD_DDict within ZSTD_DCtx

pub const ZSTD_LBMIN: c_int = 64;

// extra buffer, compensates when dst is not large enough to store litBuffer

// dictionary
// streaming
// workspace

// Tracing

// -
// Shared internal functions
//
// ! ZSTD_loadDEntropy() :
// dict : must point at beginning of a valid zstd dictionary.
// @return : size of dictionary header (size of magic number + dict ID + entropy tables)
// ! ZSTD_checkContinuity() :
// check if next `dst` follows previous position, where decompression ended.
// If yes, do nothing (continue on current segment).
// If not, classify previous segment as "external dictionary", and start a new segment.
// This function cannot fail.
extern "C" {
    pub fn ZSTD_checkContinuity(dctx: *mut *mut ZSTD_DCtx, dst: *const *const c_void, dstSize: usize);
}
