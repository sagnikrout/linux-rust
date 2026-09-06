//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/fse.h
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
// FSE : Finite State Entropy codec
// Public Prototypes declaration
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
// -
// Dependencies
//

// -
// FSE_PUBLIC_API : control library symbols visibility
//

// ------   Version   ------
pub const FSE_VERSION_MAJOR: c_int = 0;
pub const FSE_VERSION_MINOR: c_int = 9;
pub const FSE_VERSION_RELEASE: c_int = 0;

// -
// Tool functions
//
// Error Management
// -
// FSE detailed API
//
// !
//
// *** COMPRESSION ***
// ! FSE_optimalTableLog():
extern "C" {
    pub fn FSE_optimalTableLog(maxTableLog: unsigned, srcSize: usize, maxSymbolValue: unsigned) -> FSE_PUBLIC_API unsigned;
}
// ! FSE_normalizeCount():
// ! FSE_NCountWriteBound():
extern "C" {
    pub fn FSE_NCountWriteBound(maxSymbolValue: unsigned, tableLog: unsigned) -> FSE_PUBLIC_API size_t;
}
// ! FSE_writeNCount():
// ! Constructor and Destructor of FSE_CTable.
// ! FSE_buildCTable():
extern "C" {
    pub fn FSE_buildCTable(ct: *mut *mut FSE_CTable, normalizedCounter: *const *const c_short, maxSymbolValue: unsigned, tableLog: unsigned) -> FSE_PUBLIC_API size_t;
}
// ! FSE_compress_usingCTable():
extern "C" {
    pub fn FSE_compress_usingCTable(dst: *mut *mut c_void, dstCapacity: usize, src: *const *const c_void, srcSize: usize, ct: *const *const FSE_CTable) -> FSE_PUBLIC_API size_t;
}
// !
//
// *** DECOMPRESSION ***
// ! FSE_readNCount():
// ! FSE_readNCount_bmi2():
// Same as FSE_readNCount() but pass bmi2=1 when your CPU supports BMI2 and 0 otherwise.
//
// !
//

// Macro flag: #define FSE_H_FSE_STATIC_LINKING_ONLY

//
// Static allocation
//
// FSE buffer bounds
pub const FSE_NCOUNTBOUND: c_int = 512;

// It is possible to statically allocate FSE CTable/DTable as a table of FSE_CTable/FSE_DTable using below macros

// or use the size to malloc() space directly. Pay attention to alignment restrictions though

//
// FSE advanced API
//
extern "C" {
    pub fn FSE_optimalTableLog_internal(maxTableLog: unsigned, srcSize: usize, maxSymbolValue: unsigned, minus: unsigned) -> unsigned;
}
// < same as FSE_optimalTableLog(), which used `minus==2`
extern "C" {
    pub fn FSE_buildCTable_rle(ct: *mut *mut FSE_CTable, symbolValue: c_uchar) -> usize;
}
// < build a fake FSE_CTable, designed to compress always the same symbolValue
// FSE_buildCTable_wksp() :
// Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
// `wkspSize` must be >= `FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue, tableLog)` of `unsigned`.
// See FSE_buildCTable_wksp() for breakdown of workspace usage.
//

extern "C" {
    pub fn FSE_buildCTable_wksp(ct: *mut *mut FSE_CTable, normalizedCounter: *const *const c_short, maxSymbolValue: unsigned, tableLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize) -> usize;
}

extern "C" {
    pub fn FSE_buildDTable_wksp(dt: *mut *mut FSE_DTable, normalizedCounter: *const *const c_short, maxSymbolValue: unsigned, tableLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize) -> FSE_PUBLIC_API size_t;
}
// < Same as FSE_buildDTable(), using an externally allocated `workspace` produced with `FSE_BUILD_DTABLE_WKSP_SIZE_U32(maxSymbolValue)`

extern "C" {
    pub fn FSE_decompress_wksp_bmi2(dst: *mut *mut c_void, dstCapacity: usize, cSrc: *const *const c_void, cSrcSize: usize, maxLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize, bmi2: c_int) -> usize;
}
// < same as FSE_decompress(), using an externally allocated `workSpace` produced with `FSE_DECOMPRESS_WKSP_SIZE_U32(maxLog, maxSymbolValue)`.
// Set bmi2 to 1 if your CPU supports BMI2 or 0 if it doesn't
//
// FSE symbol compression API
//
// !
//
extern "C" {
    pub fn FSE_initCState(CStatePtr: *mut *mut FSE_CState_t, ct: *const *const FSE_CTable) -> static void;
}
extern "C" {
    pub fn FSE_encodeSymbol(bitC: *mut *mut BIT_CStream_t, CStatePtr: *mut *mut FSE_CState_t, symbol: unsigned) -> static void;
}
extern "C" {
    pub fn FSE_flushCState(bitC: *mut *mut BIT_CStream_t, CStatePtr: *const *const FSE_CState_t) -> static void;
}
// <
extern "C" {
    pub fn BIT_initCStream(code: ) can produce an error, tested: so its result should be, FSE_isError(: using) -> Note that;
}
//
// FSE symbol decompression API
//
extern "C" {
    pub fn FSE_initDState(DStatePtr: *mut *mut FSE_DState_t, bitD: *mut *mut BIT_DStream_t, dt: *const *const FSE_DTable) -> static void;
}
extern "C" {
    pub fn FSE_decodeSymbol(DStatePtr: *mut *mut FSE_DState_t, bitD: *mut *mut BIT_DStream_t) -> static unsigned char;
}
extern "C" {
    pub fn FSE_endOfDState(DStatePtr: *const *const FSE_DState_t) -> static unsigned;
}
// <
//
// FSE unsafe API
//
extern "C" {
    pub fn FSE_decodeSymbolFast(DStatePtr: *mut *mut FSE_DState_t, bitD: *mut *mut BIT_DStream_t) -> static unsigned char;
}
// faster, but works only if nbBits is always >= 1 (otherwise, result will be corrupted)
//
// Implementation of inlined functions
//
// ! FSE_initCState2() :
// Same as FSE_initCState(), but the first symbol to include (which will be the last to be read)
// uses the smallest state value possible, saving the cost of this symbol
// FSE_getMaxNbBits() :
// Approximate maximum cost of a symbol, in bits.
// Fractional get rounded up (i.e. a symbol with a normalized frequency of 3 gives the same result as a frequency of 2)
// note 1 : assume symbolValue is valid (<= maxSymbolValue)
// note 2 : if freq[symbolValue]==0, @return a fake cost of tableLog+1 bits
// FSE_bitCost() :
// Approximate symbol cost, as fractional value, using fixed-point format (accuracyLog fractional bits)
// note 1 : assume symbolValue is valid (<= maxSymbolValue)
// note 2 : if freq[symbolValue]==0, @return a fake cost of tableLog+1 bits
// ======    Decompression    ======
// ! FSE_decodeSymbolFast() :

//
// Tuning parameters
//
// !MEMORY_USAGE :
// Memory usage formula : N->2^N Bytes (examples : 10 -> 1KB; 12 -> 4KB ; 16 -> 64KB; 20 -> 1MB; etc.)
// Increasing memory usage improves compression ratio
// Reduced memory usage can improve speed, due to cache effect
// Recommended max value is 14, for 16KB, which nicely fits into Intel x86 L1 cache

// !FSE_MAX_SYMBOL_VALUE :
// Maximum symbol value authorized.
// Required for proper stack allocation

//
// template functions type & suffix
//

// Macro flag: #define FSE_FUNCTION_EXTENSION

//
// Constants
//

pub const FSE_MIN_TABLELOG: c_int = 5;
pub const FSE_TABLELOG_ABSOLUTE_MAX: c_int = 15;

