//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/compress/zstd_ldm.h
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
// Long distance matching
//

//
// ZSTD_ldm_generateSequences():
//
// Generates the sequences using the long distance match finder.
// Generates long range matching sequences in `sequences`, which parse a prefix
// of the source. `sequences` must be large enough to store every sequence,
// which can be checked with `ZSTD_ldm_getMaxNbSeq()`.
// @returns 0 or an error code.
//
// NOTE: The user must have called ZSTD_window_update() for all of the input
// they have, even if they pass it to ZSTD_ldm_generateSequences() in chunks.
// NOTE: This function returns an error if it runs out of space to store
// sequences.
//
// ZSTD_ldm_blockCompress():
//
// Compresses a block using the predefined sequences, along with a secondary
// block compressor. The literals section of every sequence is passed to the
// secondary block compressor, and those sequences are interspersed with the
// predefined sequences. Returns the length of the last literals.
// Updates `rawSeqStore.pos` to indicate how many sequences have been consumed.
// `rawSeqStore.seq` may also be updated to split the last sequence between two
// blocks.
// @return The length of the last literals.
//
// NOTE: The source must be at most the maximum block size, but the predefined
// sequences can be any size, and may be longer than the block. In the case that
// they are longer than the block, the last sequences may need to be split into
// two. We handle that case correctly, and update `rawSeqStore` appropriately.
// NOTE: This function does not return any errors.
//
// ZSTD_ldm_skipSequences():
//
// Skip past `srcSize` bytes worth of sequences in `rawSeqStore`.
// Avoids emitting matches less than `minMatch` bytes.
// Must be called for data that is not passed to ZSTD_ldm_blockCompress().
//
// ZSTD_ldm_skipRawSeqStoreBytes():
// Moves forward in rawSeqStore by nbBytes, updating fields 'pos' and 'posInSequence'.
// Not to be used in conjunction with ZSTD_ldm_skipSequences().
// Must be called for data with is not passed to ZSTD_ldm_blockCompress().
//
extern "C" {
    pub fn ZSTD_ldm_skipRawSeqStoreBytes(rawSeqStore: *mut *mut RawSeqStore_t, nbBytes: usize);
}
// ZSTD_ldm_getTableSize() :
// Estimate the space needed for long distance matching tables or 0 if LDM is
// disabled.
//
extern "C" {
    pub fn ZSTD_ldm_getTableSize(params: ldmParams_t) -> usize;
}
// ZSTD_ldm_getSeqSpace() :
// Return an upper bound on the number of sequences that can be produced by
// the long distance matcher, or 0 if LDM is disabled.
//
extern "C" {
    pub fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, maxChunkSize: usize) -> usize;
}
// ZSTD_ldm_adjustParameters() :
// If the params->hashRateLog is not set, set it to its default value based on
// windowLog and params->hashLog.
//
// Ensures that params->bucketSizeLog is <= params->hashLog (setting it to
// params->hashLog if it is not).
//
// Ensures that the minMatchLength >= targetLength during optimal parsing.
//
