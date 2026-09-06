//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/huf.h
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
// huff0 huffman codec,
// part of Finite State Entropy library
// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// You can contact the author at :
// - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//

// Macro flag: #define HUF_H_298734234
// *** Dependencies ***

// Macro flag: #define FSE_STATIC_LINKING_ONLY

// ***   Tool functions ***

// Error Management

// *** Constants ***

pub const HUF_SYMBOLVALUE_MAX: c_int = 255;

//
// Static allocation
//
// HUF buffer bounds
pub const HUF_CTABLEBOUND: c_int = 129;

// static allocation of HUF's Compression Table
// this is a private definition, just exposed for allocation and strict aliasing purpose. never EVER access its members directly

// static allocation of HUF's DTable
pub type HUF_DTable = U32;

//
// Advanced decompression functions
//
// Huffman flags bitset.
// For all flags, 0 is the default value.
//
// If compiled with DYNAMIC_BMI2: Set flag only if the CPU supports BMI2 at runtime.
// Otherwise: Ignored.
//
// If set: Test possible table depths to find the one that produces the smallest header + encoded size.
// If unset: Use heuristic to find the table depth.
//
// If set: If the previous table can encode the input, always reuse the previous table.
// If unset: If the previous table can encode the input, reuse the previous table if it results in a smaller output.
//
// If set: Sample the input and check if the sample is uncompressible, if it is then don't attempt to compress.
// If unset: Always histogram the entire input.
//
// If set: Don't use assembly implementations
// If unset: Allow using assembly implementations
//
// If set: Don't use the fast decoding loop, always use the fallback decoding loop.
// If unset: Use the fast decoding loop when possible.
//
// HUF detailed API
//

// ! HUF_compress() does the following:
// 1. count symbol occurrence from source[] into table count[] using FSE_count() (exposed within "fse.h")
// 2. (optional) refine tableLog using HUF_optimalTableLog()
// 3. build Huffman table from count using HUF_buildCTable()
// 4. save Huffman table to memory buffer using HUF_writeCTable()
// 5. encode the data stream using HUF_compress4X_usingCTable()
//
// The following API allows targeting specific sub-functions for advanced tasks.
// For example, it's possible to compress several blocks using the same 'CTable',
// or to save and regenerate 'CTable' using external methods.
//
extern "C" {
    pub fn HUF_minTableLog(symbolCardinality: unsigned) -> unsigned;
}
extern "C" {
    pub fn HUF_cardinality(count: *const *const unsigned, maxSymbolValue: unsigned) -> unsigned;
}
extern "C" {
    pub fn HUF_writeCTable_wksp(dst: *mut *mut c_void, maxDstSize: usize, CTable: *const *const HUF_CElt, maxSymbolValue: unsigned, huffLog: unsigned, workspace: *mut *mut c_void, workspaceSize: usize) -> usize;
}
extern "C" {
    pub fn HUF_compress4X_usingCTable(dst: *mut *mut c_void, dstSize: usize, src: *const *const c_void, srcSize: usize, CTable: *const *const HUF_CElt, flags: c_int) -> usize;
}
extern "C" {
    pub fn HUF_estimateCompressedSize(CTable: *const *const HUF_CElt, count: *const *const unsigned, maxSymbolValue: unsigned) -> usize;
}
extern "C" {
    pub fn HUF_validateCTable(CTable: *const *const HUF_CElt, count: *const *const unsigned, maxSymbolValue: unsigned) -> c_int;
}
// HUF_compress4X_repeat() :
// Same as HUF_compress4X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
// If it uses hufTable it does not modify hufTable or repeat.
// If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
// If preferRepeat then the old table will always be used if valid.
// If suspectUncompressible then some sampling checks will be run to potentially skip huffman coding
// HUF_buildCTable_wksp() :
// Same as HUF_buildCTable(), but using externally allocated scratch buffer.
// `workSpace` must be aligned on 4-bytes boundaries, and its size must be >= HUF_CTABLE_WORKSPACE_SIZE.
//

// ! HUF_readStats() :
// Read compact Huffman tree, saved by HUF_writeCTable().
// `huffWeight` is destination buffer.
// @return : size read from `src` , or an error Code .
// Note : Needed by HUF_readCTable() and HUF_readDTableXn() .
// ! HUF_readStats_wksp() :
// Same as HUF_readStats() but takes an external workspace which must be
// 4-byte aligned and its size must be >= HUF_READ_STATS_WORKSPACE_SIZE.
// If the CPU has BMI2 support, pass bmi2=1, otherwise pass bmi2=0.
//

// HUF_readCTable() :
// Loading a CTable saved with HUF_writeCTable()
extern "C" {
    pub fn HUF_readCTable(CTable: *mut *mut HUF_CElt, maxSymbolValuePtr: *mut *mut unsigned, src: *const *const c_void, srcSize: usize, hasZeroWeights: *mut unsigned) -> usize;
}
// HUF_getNbBitsFromCTable() :
// Read nbBits from CTable symbolTable, for symbol `symbolValue` presumed <= HUF_SYMBOLVALUE_MAX
// Note 1 : If symbolValue > HUF_readCTableHeader(symbolTable).maxSymbolValue, returns 0
// Note 2 : is not inlined, as HUF_CElt definition is private
//
extern "C" {
    pub fn HUF_getNbBitsFromCTable(symbolTable: *const *const HUF_CElt, symbolValue: U32) -> U32;
}
// HUF_readCTableHeader() :
// @returns The header from the CTable specifying the tableLog and the maxSymbolValue.
//
extern "C" {
    pub fn HUF_readCTableHeader(ctable: *mut *mut HUF_CElt const) -> HUF_CTableHeader;
}
//
// HUF_decompress() does the following:
// 1. select the decompression algorithm (X1, X2) based on pre-computed heuristics
// 2. build Huffman table from save, using HUF_readDTableX?()
// 3. decode 1 or 4 segments in parallel using HUF_decompress?X?_usingDTable()
//
// HUF_selectDecoder() :
// Tells which decoder is likely to decode faster,
// based on a set of pre-computed metrics.
// @return : 0==HUF_decompress4X1, 1==HUF_decompress4X2 .
// Assumption : 0 < dstSize <= 128 KB
extern "C" {
    pub fn HUF_selectDecoder(dstSize: usize, cSrcSize: usize) -> U32;
}
//
// The minimum workspace size for the `workSpace` used in
// HUF_readDTableX1_wksp() and HUF_readDTableX2_wksp().
//
// The space used depends on HUF_TABLELOG_MAX, ranging from ~1500 bytes when
// HUF_TABLE_LOG_MAX=12 to ~1850 bytes when HUF_TABLE_LOG_MAX=15.
// Buffer overflow errors may potentially occur if code modifications result in
// a required workspace size greater than that specified in the following
// macro.
//

// ======================
// single stream variants
// ======================
extern "C" {
    pub fn HUF_compress1X_usingCTable(dst: *mut *mut c_void, dstSize: usize, src: *const *const c_void, srcSize: usize, CTable: *const *const HUF_CElt, flags: c_int) -> usize;
}
// HUF_compress1X_repeat() :
// Same as HUF_compress1X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
// If it uses hufTable it does not modify hufTable or repeat.
// If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
// If preferRepeat then the old table will always be used if valid.
// If suspectUncompressible then some sampling checks will be run to potentially skip huffman coding
extern "C" {
    pub fn HUF_decompress1X_DCtx_wksp(dctx: *mut *mut HUF_DTable, dst: *mut *mut c_void, dstSize: usize, cSrc: *const *const c_void, cSrcSize: usize, workSpace: *mut *mut c_void, wkspSize: usize, flags: c_int) -> usize;
}

// BMI2 variants.
// If the CPU has BMI2 support, pass bmi2=1, otherwise pass bmi2=0.
//
extern "C" {
    pub fn HUF_decompress1X_usingDTable(dst: *mut *mut c_void, maxDstSize: usize, cSrc: *const *const c_void, cSrcSize: usize, DTable: *const *const HUF_DTable, flags: c_int) -> usize;
}

extern "C" {
    pub fn HUF_decompress1X1_DCtx_wksp(dctx: *mut *mut HUF_DTable, dst: *mut *mut c_void, dstSize: usize, cSrc: *const *const c_void, cSrcSize: usize, workSpace: *mut *mut c_void, wkspSize: usize, flags: c_int) -> usize;
}

extern "C" {
    pub fn HUF_decompress4X_usingDTable(dst: *mut *mut c_void, maxDstSize: usize, cSrc: *const *const c_void, cSrcSize: usize, DTable: *const *const HUF_DTable, flags: c_int) -> usize;
}
extern "C" {
    pub fn HUF_decompress4X_hufOnly_wksp(dctx: *mut *mut HUF_DTable, dst: *mut *mut c_void, dstSize: usize, cSrc: *const *const c_void, cSrcSize: usize, workSpace: *mut *mut c_void, wkspSize: usize, flags: c_int) -> usize;
}

extern "C" {
    pub fn HUF_readDTableX1_wksp(DTable: *mut *mut HUF_DTable, src: *const *const c_void, srcSize: usize, workSpace: *mut *mut c_void, wkspSize: usize, flags: c_int) -> usize;
}

extern "C" {
    pub fn HUF_readDTableX2_wksp(DTable: *mut *mut HUF_DTable, src: *const *const c_void, srcSize: usize, workSpace: *mut *mut c_void, wkspSize: usize, flags: c_int) -> usize;
}

