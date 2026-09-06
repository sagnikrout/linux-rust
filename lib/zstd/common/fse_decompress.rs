//! Automatically rewritten from C to Rust
//! Source: lib/zstd/common/fse_decompress.c
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
// FSE : Finite State Entropy decoder
// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// You can contact the author at :
// - FSE source repository : https://github.com/Cyan4973/FiniteStateEntropy
// - Public forum : https://groups.google.com/forum/#!forum/lz4c
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// Includes
//

// Macro flag: #define FSE_STATIC_LINKING_ONLY

//
// Error Management
//

//
// Templates
//
    designed to be included
    for type-specific functions (template emulation in C)
    Objective is to write these functions only once, for improved maintenance
//
// safety checks

// Function names

#[no_mangle]
unsafe extern "C" fn FSE_buildDTable_internal(dt: *mut *mut FSE_DTable, normalizedCounter: *const *const c_short, maxSymbolValue: unsigned, tableLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize) -> usize {
    static size_t FSE_buildDTable_internal(FSE_DTable* dt, const short* normalizedCounter, unsigned maxSymbolValue, unsigned tableLog, void* workSpace, size_t wkspSize)
    {
    void* const tdPtr = dt+1;   /* because *dt is unsigned, 32-bits aligned on 32-bits */
    let mut tableDecode: *mut FSE_DECODE_TYPE const = (FSE_DECODE_TYPE*) (tdPtr);
    let mut symbolNext: *mut U16 = (U16*)workSpace;
    let mut spread: *mut BYTE = (BYTE*)(symbolNext + maxSymbolValue + 1);
    let mut maxSV1: U32 const = maxSymbolValue + 1;
    let mut tableSize: U32 const = 1 << tableLog;
    let mut highThreshold: U32 = tableSize-1;
// Sanity Checks
    if (FSE_BUILD_DTABLE_WKSP_SIZE(tableLog, maxSymbolValue) > wkspSize) return ERROR(maxSymbolValue_tooLarge);
    if (maxSymbolValue > FSE_MAX_SYMBOL_VALUE) return ERROR(maxSymbolValue_tooLarge);
    if (tableLog > FSE_MAX_TABLELOG) return ERROR(tableLog_tooLarge);
// Init, lay down lowprob symbols
    {   FSE_DTableHeader DTableH;
    DTableH.tableLog = (U16)tableLog;
    DTableH.fastMode = 1;
    {   S16 const largeLimit= (S16)(1 << (tableLog-1));
    U32 s;
    for (s=0; s<maxSV1; s++) {
    if (normalizedCounter[s]==-1) {
    tableDecode[highThreshold--].symbol = (FSE_FUNCTION_TYPE)s;
    symbolNext[s] = 1;
    } else {
    if (normalizedCounter[s] >= largeLimit) DTableH.fastMode=0;
    symbolNext[s] = (U16)normalizedCounter[s];
    }   }   }
    ZSTD_memcpy(dt, &DTableH, sizeof(DTableH));
    }
// Spread symbols
    if (highThreshold == tableSize - 1) {
    let mut tableMask: size_t const = tableSize-1;
    let mut step: size_t const = FSE_TABLESTEP(tableSize);
// First lay down the symbols in order.
// We use a uint64_t to lay down 8 bytes at a time. This reduces branch
// misses since small blocks generally have small table logs, so nearly
// all symbols have counts <= 8. We ensure we have 8 bytes at the end of
// our buffer to handle the over-write.
//
    {   U64 const add = 0x0101010101010101ull;
    let mut pos: usize = 0;
    let mut sv: U64 = 0;
    U32 s;
    for (s=0; s<maxSV1; ++s, sv += add) {
    int i;
    let mut n: int const = normalizedCounter[s];
    MEM_write64(spread + pos, sv);
    for (i = 8; i < n; i += 8) {
    MEM_write64(spread + pos + i, sv);
    }
    pos += (size_t)n;
    }   }
// Now we spread those positions across the table.
// The benefit of doing it in two stages is that we avoid the
// variable size inner loop, which caused lots of branch misses.
// Now we can run through all the positions without any branch misses.
// We unroll the loop twice, since that is what empirically worked best.
//
    {
    let mut position: usize = 0;
    size_t s;
    let mut unroll: size_t const = 2;
    assert(tableSize % unroll == 0); /* FSE_MIN_TABLELOG is 5 */
    for (s = 0; s < (size_t)tableSize; s += unroll) {
    size_t u;
    for (u = 0; u < unroll; ++u) {
    let mut uPosition: size_t const = (position + (u * step)) & tableMask;
    tableDecode[uPosition].symbol = spread[s + u];
    }
    position = (position + (unroll * step)) & tableMask;
    }
    assert(position == 0);
    }
    } else {
    let mut tableMask: U32 const = tableSize-1;
    let mut step: U32 const = FSE_TABLESTEP(tableSize);
    U32 s, position = 0;
    for (s=0; s<maxSV1; s++) {
    int i;
    for (i=0; i<normalizedCounter[s]; i++) {
    tableDecode[position].symbol = (FSE_FUNCTION_TYPE)s;
    position = (position + step) & tableMask;
    while (position > highThreshold) position = (position + step) & tableMask;   /* lowprob area */
    }   }
    if (position!=0) return ERROR(GENERIC);   /* position must reach all cells once, otherwise normalizedCounter is incorrect */
    }
// Build Decoding table
    {   U32 u;
    for (u=0; u<tableSize; u++) {
    let mut symbol: FSE_FUNCTION_TYPE const = (FSE_FUNCTION_TYPE)(tableDecode[u].symbol);
    let mut nextState: U32 const = symbolNext[symbol]++;
    tableDecode[u].nbBits = (BYTE) (tableLog - ZSTD_highbit32(nextState) );
    tableDecode[u].newState = (U16) ( (nextState << tableDecode[u].nbBits) - tableSize);
    }   }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTable_wksp(dt: *mut *mut FSE_DTable, normalizedCounter: *const *const c_short, maxSymbolValue: unsigned, tableLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize) -> usize {
    size_t FSE_buildDTable_wksp(FSE_DTable* dt, const short* normalizedCounter, unsigned maxSymbolValue, unsigned tableLog, void* workSpace, size_t wkspSize)
    {
    return FSE_buildDTable_internal(dt, normalizedCounter, maxSymbolValue, tableLog, workSpace, wkspSize);
    }

// -
// Decompression (Byte symbols)
//
    FORCE_INLINE_TEMPLATE size_t FSE_decompress_usingDTable_generic(
    void* dst, size_t maxDstSize,
    const void* cSrc, size_t cSrcSize,
    const FSE_DTable* dt, const unsigned fast)
    {
    let mut ostart: *mut BYTE const = (BYTE*) dst;
    let mut op: *mut BYTE = ostart;
    let mut omax: *mut BYTE const = op + maxDstSize;
    let mut olimit: *mut BYTE const = omax-3;
    BIT_DStream_t bitD;
    FSE_DState_t state1;
    FSE_DState_t state2;
// Init
    CHECK_F(BIT_initDStream(&bitD, cSrc, cSrcSize));
    FSE_initDState(&state1, &bitD, dt);
    FSE_initDState(&state2, &bitD, dt);
    RETURN_ERROR_IF(BIT_reloadDStream(&bitD)==BIT_DStream_overflow, corruption_detected, "");

// 4 symbols per loop
    for ( ; (BIT_reloadDStream(&bitD)==BIT_DStream_unfinished) & (op<olimit) ; op+=4) {
    op[0] = FSE_GETSYMBOL(&state1);
    if (FSE_MAX_TABLELOG*2+7 > sizeof(bitD.bitContainer)*8)    /* This test must be static */
    BIT_reloadDStream(&bitD);
    op[1] = FSE_GETSYMBOL(&state2);
    if (FSE_MAX_TABLELOG*4+7 > sizeof(bitD.bitContainer)*8)    /* This test must be static */
    { if (BIT_reloadDStream(&bitD) > BIT_DStream_unfinished) { op+=2; break; } }
    op[2] = FSE_GETSYMBOL(&state1);
    if (FSE_MAX_TABLELOG*2+7 > sizeof(bitD.bitContainer)*8)    /* This test must be static */
    BIT_reloadDStream(&bitD);
    op[3] = FSE_GETSYMBOL(&state2);
    }
// tail
// note : BIT_reloadDStream(&bitD) >= FSE_DStream_partiallyFilled; Ends at exactly BIT_DStream_completed
    while (1) {
    if (op>(omax-2)) return ERROR(dstSize_tooSmall);
// op++ = FSE_GETSYMBOL(&state1);
    if (BIT_reloadDStream(&bitD)==BIT_DStream_overflow) {
// op++ = FSE_GETSYMBOL(&state2);
    break;
    }
    if (op>(omax-2)) return ERROR(dstSize_tooSmall);
// op++ = FSE_GETSYMBOL(&state2);
    if (BIT_reloadDStream(&bitD)==BIT_DStream_overflow) {
// op++ = FSE_GETSYMBOL(&state1);
    break;
    }   }
    assert(op >= ostart);
    return (size_t)(op-ostart);
    }
    typedef struct {
    short ncount[FSE_MAX_SYMBOL_VALUE + 1];
    } FSE_DecompressWksp;
    FORCE_INLINE_TEMPLATE size_t FSE_decompress_wksp_body(
    void* dst, size_t dstCapacity,
    const void* cSrc, size_t cSrcSize,
    unsigned maxLog, void* workSpace, size_t wkspSize,
    int bmi2)
    {
    let mut istart: *const BYTE const = (const BYTE*)cSrc;
    let mut ip: *const BYTE = istart;
    unsigned tableLog;
    let mut maxSymbolValue: unsigned = FSE_MAX_SYMBOL_VALUE;
    let mut wksp: *mut FSE_DecompressWksp const = (FSE_DecompressWksp*)workSpace;
    let mut dtablePos: size_t const = sizeof(FSE_DecompressWksp) / sizeof(FSE_DTable);
    let mut dtable: *mut FSE_DTable const = (FSE_DTable*)workSpace + dtablePos;
    FSE_STATIC_ASSERT((FSE_MAX_SYMBOL_VALUE + 1) % 2 == 0);
    if (wkspSize < sizeof(*wksp)) return ERROR(GENERIC);
// correct offset to dtable depends on this property
    FSE_STATIC_ASSERT(sizeof(FSE_DecompressWksp) % sizeof(FSE_DTable) == 0);
// normal FSE decoding mode
    {   size_t const NCountLength =
    FSE_readNCount_bmi2(wksp.ncount, &maxSymbolValue, &tableLog, istart, cSrcSize, bmi2);
    if (FSE_isError(NCountLength)) return NCountLength;
    if (tableLog > maxLog) return ERROR(tableLog_tooLarge);
    assert(NCountLength <= cSrcSize);
    ip += NCountLength;
    cSrcSize -= NCountLength;
    }
    if (FSE_DECOMPRESS_WKSP_SIZE(tableLog, maxSymbolValue) > wkspSize) return ERROR(tableLog_tooLarge);
    assert(sizeof(*wksp) + FSE_DTABLE_SIZE(tableLog) <= wkspSize);
    workSpace = (BYTE*)workSpace + sizeof(*wksp) + FSE_DTABLE_SIZE(tableLog);
    wkspSize -= sizeof(*wksp) + FSE_DTABLE_SIZE(tableLog);
    CHECK_F( FSE_buildDTable_internal(dtable, wksp.ncount, maxSymbolValue, tableLog, workSpace, wkspSize) );
    {
    let mut ptr: *const c_void = dtable;
    let mut DTableH: *const FSE_DTableHeader = (const FSE_DTableHeader*)ptr;
    let mut fastMode: U32 = DTableH.fastMode;
// select fast mode (static)
    if (fastMode) return FSE_decompress_usingDTable_generic(dst, dstCapacity, ip, cSrcSize, dtable, 1);
    return FSE_decompress_usingDTable_generic(dst, dstCapacity, ip, cSrcSize, dtable, 0);
    }
    }
// Avoids the FORCE_INLINE of the _body() function.
#[no_mangle]
unsafe extern "C" fn FSE_decompress_wksp_body_default(dst: *mut *mut c_void, dstCapacity: usize, cSrc: *const *const c_void, cSrcSize: usize, maxLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize) -> usize {
    static size_t FSE_decompress_wksp_body_default(void* dst, size_t dstCapacity, const void* cSrc, size_t cSrcSize, unsigned maxLog, void* workSpace, size_t wkspSize)
    {
    return FSE_decompress_wksp_body(dst, dstCapacity, cSrc, cSrcSize, maxLog, workSpace, wkspSize, 0);
    }

#[no_mangle]
pub unsafe extern "C" fn FSE_decompress_wksp_body_bmi2(dst: *mut *mut c_void, dstCapacity: usize, cSrc: *const *const c_void, cSrcSize: usize, maxLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize) -> BMI2_TARGET_ATTRIBUTE static size_t {
    BMI2_TARGET_ATTRIBUTE static size_t FSE_decompress_wksp_body_bmi2(void* dst, size_t dstCapacity, const void* cSrc, size_t cSrcSize, unsigned maxLog, void* workSpace, size_t wkspSize)
    {
    return FSE_decompress_wksp_body(dst, dstCapacity, cSrc, cSrcSize, maxLog, workSpace, wkspSize, 1);
    }

#[no_mangle]
pub unsafe extern "C" fn FSE_decompress_wksp_bmi2(dst: *mut *mut c_void, dstCapacity: usize, cSrc: *const *const c_void, cSrcSize: usize, maxLog: unsigned, workSpace: *mut *mut c_void, wkspSize: usize, bmi2: c_int) -> usize {
    size_t FSE_decompress_wksp_bmi2(void* dst, size_t dstCapacity, const void* cSrc, size_t cSrcSize, unsigned maxLog, void* workSpace, size_t wkspSize, int bmi2)
    {

    if (bmi2) {
    return FSE_decompress_wksp_body_bmi2(dst, dstCapacity, cSrc, cSrcSize, maxLog, workSpace, wkspSize);
    }

    (void)bmi2;
    return FSE_decompress_wksp_body_default(dst, dstCapacity, cSrc, cSrcSize, maxLog, workSpace, wkspSize);
    }
