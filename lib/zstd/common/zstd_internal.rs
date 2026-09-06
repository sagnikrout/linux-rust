//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/zstd_internal.h
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

// Macro flag: #define ZSTD_CCOMMON_H_MODULE
// this module contains definitions which must be identical
// across compression, decompression and dictBuilder.
// It also contains a few functions useful to at least 2 of them
// and which benefit from being inlined
// -
// Dependencies
//

// Macro flag: #define ZSTD_STATIC_LINKING_ONLY

// Macro flag: #define FSE_STATIC_LINKING_ONLY

pub const ZSTD_TRACE: c_int = 0;
// ---- static assert (debug) ---

// -
// shared macros
//

// -
// Common constants
//

pub const BIT7: c_int = 128;
pub const BIT6: c_int = 64;
pub const BIT5: c_int = 32;
pub const BIT4: c_int = 16;
pub const BIT1: c_int = 2;
pub const BIT0: c_int = 1;
pub const ZSTD_WINDOWLOG_ABSOLUTEMIN: c_int = 10;

pub const ZSTD_FRAMECHECKSUMSIZE: c_int = 4;

pub const MIN_LITERALS_FOR_4_STREAMS: c_int = 6;
pub const LONGNBSEQ: c_uint = 0x7F00;
pub const MINMATCH: c_int = 3;
pub const Litbits: c_int = 8;
pub const LitHufLog: c_int = 11;

pub const MaxML: c_int = 52;
pub const MaxLL: c_int = 35;
pub const DefaultMaxOff: c_int = 28;
pub const MaxOff: c_int = 31;

pub const MLFSELog: c_int = 9;
pub const LLFSELog: c_int = 9;
pub const OffFSELog: c_int = 8;

pub const MaxMLBits: c_int = 16;
pub const MaxLLBits: c_int = 16;

// Each table cannot take more than #symbols * FSELog bits

// -
// Shared functions to include for inlining
//

// Need to use memmove here since the literal buffer can now be located within

// ZSTD_memmove is not inlined properly by gcc

pub const WILDCOPY_OVERLENGTH: c_int = 32;
pub const WILDCOPY_VECLEN: c_int = 16;
// ZSTD_overlap_dst_before_src,
// ! ZSTD_wildcopy() :
// Custom version of ZSTD_memcpy(), can over read/write up to WILDCOPY_OVERLENGTH bytes (if length==0)
// @param ovtype controls the overlap detection
// - ZSTD_no_overlap: The source and destination are guaranteed to be at least WILDCOPY_VECLEN bytes apart.
// - ZSTD_overlap_src_before_dst: The src and dst may overlap, but they MUST be at least 8 bytes apart.
// The src buffer must be before the dst buffer.
//
// Handle short offset copies.
// Separate out the first COPY16() call because the copy length is
// almost certain to be short, so the branches have different
// probabilities. Since it is almost certain to be short, only do
// one COPY16() in the first call. Then, do two calls per loop since
// at that point it is more likely to have a high trip count.
//
// define "workspace is too large" as this number of times larger than needed
pub const ZSTD_WORKSPACETOOLARGE_FACTOR: c_int = 3;
// when workspace is continuously too large
// during at least this number of times,
// context's memory usage is considered wasteful,
// because it's sized to handle a worst case scenario which rarely happens.
// In which case, resize it down to free some memory
pub const ZSTD_WORKSPACETOOLARGE_MAXDURATION: c_int = 128;
// Controls whether the input/output buffer is buffered or stable.
// -
// Private declarations
//
// Contains the compressed frame size and an upper-bound for the decompressed frame size.
// Note: before using `compressedSize`, check for errors using ZSTD_isError().
// similarly, before using `decompressedBound`, check for errors using:
// `decompressedBound != ZSTD_CONTENTSIZE_ERROR`
//
// ZSTD_invalidateRepCodes() :
// ensures next compression will not use repcodes from previous block.
// Note : only works with regular variant;
// do not use with extDict variant !
// ! ZSTD_getcBlockSize() :
// Provides the size of compressed block from block header `src`
// Used by: decompress, fullbench
// ! ZSTD_decodeSeqHeaders() :
// decode sequence header from src
// Used by: zstd_decompress_block, fullbench
//
// @returns true iff the CPU supports dynamic BMI2 dispatch.
//
extern "C" {
    pub fn ZSTD_cpuid_bmi1(ZSTD_cpuid_bmi2(cpuid: cpuid) &&) -> return;
}
