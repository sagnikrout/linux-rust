//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/decompress/zstd_decompress_block.h
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
// -
// Dependencies
//

// ===   Prototypes   ===
// note: prototypes already published within `zstd.h` :
// ZSTD_decompressBlock()
//
// note: prototypes already published within `zstd_internal.h` :
// ZSTD_getcBlockSize()
// ZSTD_decodeSeqHeaders()
//
// Streaming state is used to inform allocation of the literal buffer
// ZSTD_decompressBlock_internal() :
// decompress block, starting at `src`,
// into destination buffer `dst`.
// @return : decompressed block size,
// or an error code (which can be tested using ZSTD_isError())
//
// ZSTD_buildFSETable() :
// generate FSE decoding table for one symbol (ll, ml or off)
// this function must be called with valid parameters only
// (dt is large enough, normalizedCounter distribution total is a power of 2, max is within range, etc.)
// in which case it cannot fail.
// The workspace must be 4-byte aligned and at least ZSTD_BUILD_FSE_TABLE_WKSP_SIZE bytes, which is
// defined in zstd_decompress_internal.h.
// Internal use only.
//
// Internal definition of ZSTD_decompressBlock() to avoid deprecation warnings.
