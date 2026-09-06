//! Automatically rewritten from C to Rust
//! Source: lib/zstd/common/entropy_common.c
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
// Common functions of New Generation Entropy library
// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// You can contact the author at :
// - FSE+HUF source repository : https://github.com/Cyan4973/FiniteStateEntropy
// - Public forum : https://groups.google.com/forum/#!forum/lz4c
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// Dependencies
//

// ===   Version   ===
    unsigned FSE_versionNumber(void) { return FSE_VERSION_NUMBER; }
// ===   Error Management   ===
    unsigned FSE_isError(size_t code) { return ERR_isError(code); }
    const char* FSE_getErrorName(size_t code) { return ERR_getErrorName(code); }
    unsigned HUF_isError(size_t code) { return ERR_isError(code); }
    const char* HUF_getErrorName(size_t code) { return ERR_getErrorName(code); }
// -
// FSE NCount encoding-decoding
//
    FORCE_INLINE_TEMPLATE
    size_t FSE_readNCount_body(short* normalizedCounter, unsigned* maxSVPtr, unsigned* tableLogPtr,
    const void* headerBuffer, size_t hbSize)
    {
    let mut istart: *const BYTE const = (const BYTE*) headerBuffer;
    let mut iend: *const BYTE const = istart + hbSize;
    let mut ip: *const BYTE = istart;
    int nbBits;
    int remaining;
    int threshold;
    U32 bitStream;
    int bitCount;
    let mut charnum: unsigned = 0;
    let mut maxSV1: unsigned const = *maxSVPtr + 1;
    let mut previous0: c_int = 0;
    if (hbSize < 8) {
// This function only works when hbSize >= 8
    char buffer[8] = {0};
    ZSTD_memcpy(buffer, headerBuffer, hbSize);
    {   size_t const countSize = FSE_readNCount(normalizedCounter, maxSVPtr, tableLogPtr,
    buffer, sizeof(buffer));
    if (FSE_isError(countSize)) return countSize;
    if (countSize > hbSize) return ERROR(corruption_detected);
    return countSize;
    }   }
    assert(hbSize >= 8);
// init
    ZSTD_memset(normalizedCounter, 0, (*maxSVPtr+1) * sizeof(normalizedCounter[0]));   /* all symbols not present in NCount have a frequency of 0 */
    bitStream = MEM_readLE32(ip);
    nbBits = (bitStream & 0xF) + FSE_MIN_TABLELOG;   /* extract tableLog */
    if (nbBits > FSE_TABLELOG_ABSOLUTE_MAX) return ERROR(tableLog_tooLarge);
    bitStream >>= 4;
    bitCount = 4;
// tableLogPtr = nbBits;
    remaining = (1<<nbBits)+1;
    threshold = 1<<nbBits;
    nbBits++;
    for (;;) {
    if (previous0) {
// Count the number of repeats. Each time the
// 2-bit repeat code is 0b11 there is another
// repeat.
// Avoid UB by setting the high bit to 1.
//
    let mut repeats: c_int = ZSTD_countTrailingZeros32(~bitStream | 0x80000000) >> 1;
    while (repeats >= 12) {
    charnum += 3 * 12;
    if (LIKELY(ip <= iend-7)) {
    ip += 3;
    } else {
    bitCount -= (int)(8 * (iend - 7 - ip));
    bitCount &= 31;
    ip = iend - 4;
    }
    bitStream = MEM_readLE32(ip) >> bitCount;
    repeats = ZSTD_countTrailingZeros32(~bitStream | 0x80000000) >> 1;
    }
    charnum += 3 * repeats;
    bitStream >>= 2 * repeats;
    bitCount += 2 * repeats;
// Add the final repeat which isn't 0b11.
    assert((bitStream & 3) < 3);
    charnum += bitStream & 3;
    bitCount += 2;
// This is an error, but break and return an error
// at the end, because returning out of a loop makes
// it harder for the compiler to optimize.
//
    if (charnum >= maxSV1) break;
// We don't need to set the normalized count to 0
// because we already memset the whole buffer to 0.
//
    if (LIKELY(ip <= iend-7) || (ip + (bitCount>>3) <= iend-4)) {
    assert((bitCount >> 3) <= 3); /* For first condition to work */
    ip += bitCount>>3;
    bitCount &= 7;
    } else {
    bitCount -= (int)(8 * (iend - 4 - ip));
    bitCount &= 31;
    ip = iend - 4;
    }
    bitStream = MEM_readLE32(ip) >> bitCount;
    }
    {
    let mut max: int const = (2*threshold-1) - remaining;
    int count;
    if ((bitStream & (threshold-1)) < (U32)max) {
    count = bitStream & (threshold-1);
    bitCount += nbBits-1;
    } else {
    count = bitStream & (2*threshold-1);
    if (count >= threshold) count -= max;
    bitCount += nbBits;
    }
    count--;   /* extra accuracy */
// When it matters (small blocks), this is a
// predictable branch, because we don't use -1.
//
    if (count >= 0) {
    remaining -= count;
    } else {
    assert(count == -1);
    remaining += count;
    }
    normalizedCounter[charnum++] = (short)count;
    previous0 = !count;
    assert(threshold > 1);
    if (remaining < threshold) {
// This branch can be folded into the
// threshold update condition because we
// know that threshold > 1.
//
    if (remaining <= 1) break;
    nbBits = ZSTD_highbit32(remaining) + 1;
    threshold = 1 << (nbBits - 1);
    }
    if (charnum >= maxSV1) break;
    if (LIKELY(ip <= iend-7) || (ip + (bitCount>>3) <= iend-4)) {
    ip += bitCount>>3;
    bitCount &= 7;
    } else {
    bitCount -= (int)(8 * (iend - 4 - ip));
    bitCount &= 31;
    ip = iend - 4;
    }
    bitStream = MEM_readLE32(ip) >> bitCount;
    }   }
    if (remaining != 1) return ERROR(corruption_detected);
// Only possible when there are too many zeros.
    if (charnum > maxSV1) return ERROR(maxSymbolValue_tooSmall);
    if (bitCount > 32) return ERROR(corruption_detected);
// maxSVPtr = charnum-1;
    ip += (bitCount+7)>>3;
    return ip-istart;
    }
// Avoids the FORCE_INLINE of the _body() function.
    static size_t FSE_readNCount_body_default(
    short* normalizedCounter, unsigned* maxSVPtr, unsigned* tableLogPtr,
    const void* headerBuffer, size_t hbSize)
    {
    return FSE_readNCount_body(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer, hbSize);
    }

    BMI2_TARGET_ATTRIBUTE static size_t FSE_readNCount_body_bmi2(
    short* normalizedCounter, unsigned* maxSVPtr, unsigned* tableLogPtr,
    const void* headerBuffer, size_t hbSize)
    {
    return FSE_readNCount_body(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer, hbSize);
    }

    size_t FSE_readNCount_bmi2(
    short* normalizedCounter, unsigned* maxSVPtr, unsigned* tableLogPtr,
    const void* headerBuffer, size_t hbSize, int bmi2)
    {

    if (bmi2) {
    return FSE_readNCount_body_bmi2(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer, hbSize);
    }

    (void)bmi2;
    return FSE_readNCount_body_default(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer, hbSize);
    }
    size_t FSE_readNCount(
    short* normalizedCounter, unsigned* maxSVPtr, unsigned* tableLogPtr,
    const void* headerBuffer, size_t hbSize)
    {
    return FSE_readNCount_bmi2(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer, hbSize, /* bmi2 */ 0);
    }
// ! HUF_readStats() :
    Read compact Huffman tree, saved by HUF_writeCTable().
    `huffWeight` is destination buffer.
    `rankStats` is assumed to be a table of at least HUF_TABLELOG_MAX U32.
    @return : size read from `src` , or an error Code .
    Note : Needed by HUF_readCTable() and HUF_readDTableX?() .
//
    size_t HUF_readStats(BYTE* huffWeight, size_t hwSize, U32* rankStats,
    U32* nbSymbolsPtr, U32* tableLogPtr,
    const void* src, size_t srcSize)
    {
    U32 wksp[HUF_READ_STATS_WORKSPACE_SIZE_U32];
    return HUF_readStats_wksp(huffWeight, hwSize, rankStats, nbSymbolsPtr, tableLogPtr, src, srcSize, wksp, sizeof(wksp), /* flags */ 0);
    }
    FORCE_INLINE_TEMPLATE size_t
    HUF_readStats_body(BYTE* huffWeight, size_t hwSize, U32* rankStats,
    U32* nbSymbolsPtr, U32* tableLogPtr,
    const void* src, size_t srcSize,
    void* workSpace, size_t wkspSize,
    int bmi2)
    {
    U32 weightTotal;
    let mut ip: *const BYTE = (const BYTE*) src;
    size_t iSize;
    size_t oSize;
    if (!srcSize) return ERROR(srcSize_wrong);
    iSize = ip[0];
// ZSTD_memset(huffWeight, 0, hwSize);   *//* is not necessary, even though some analyzer complain ...
    if (iSize >= 128) {  /* special header */
    oSize = iSize - 127;
    iSize = ((oSize+1)/2);
    if (iSize+1 > srcSize) return ERROR(srcSize_wrong);
    if (oSize >= hwSize) return ERROR(corruption_detected);
    ip += 1;
    {   U32 n;
    for (n=0; n<oSize; n+=2) {
    huffWeight[n]   = ip[n/2] >> 4;
    huffWeight[n+1] = ip[n/2] & 15;
    }   }   }
    else  {   /* header compressed with FSE (normal case) */
    if (iSize+1 > srcSize) return ERROR(srcSize_wrong);
// max (hwSize-1) values decoded, as last one is implied
    oSize = FSE_decompress_wksp_bmi2(huffWeight, hwSize-1, ip+1, iSize, 6, workSpace, wkspSize, bmi2);
    if (FSE_isError(oSize)) return oSize;
    }
// collect weight stats
    ZSTD_memset(rankStats, 0, (HUF_TABLELOG_MAX + 1) * sizeof(U32));
    weightTotal = 0;
    {   U32 n; for (n=0; n<oSize; n++) {
    if (huffWeight[n] > HUF_TABLELOG_MAX) return ERROR(corruption_detected);
    rankStats[huffWeight[n]]++;
    weightTotal += (1 << huffWeight[n]) >> 1;
    }   }
    if (weightTotal == 0) return ERROR(corruption_detected);
// get last non-null symbol weight (implied, total must be 2^n)
    {   U32 const tableLog = ZSTD_highbit32(weightTotal) + 1;
    if (tableLog > HUF_TABLELOG_MAX) return ERROR(corruption_detected);
// tableLogPtr = tableLog;
// determine last weight
    {   U32 const total = 1 << tableLog;
    let mut rest: U32 const = total - weightTotal;
    let mut verif: U32 const = 1 << ZSTD_highbit32(rest);
    let mut lastWeight: U32 const = ZSTD_highbit32(rest) + 1;
    if (verif != rest) return ERROR(corruption_detected);    /* last value must be a clean power of 2 */
    huffWeight[oSize] = (BYTE)lastWeight;
    rankStats[lastWeight]++;
    }   }
// check tree construction validity
    if ((rankStats[1] < 2) || (rankStats[1] & 1)) return ERROR(corruption_detected);   /* by construction : at least 2 elts of rank 1, must be even */
// results
// nbSymbolsPtr = (U32)(oSize+1);
    return iSize+1;
    }
// Avoids the FORCE_INLINE of the _body() function.
    static size_t HUF_readStats_body_default(BYTE* huffWeight, size_t hwSize, U32* rankStats,
    U32* nbSymbolsPtr, U32* tableLogPtr,
    const void* src, size_t srcSize,
    void* workSpace, size_t wkspSize)
    {
    return HUF_readStats_body(huffWeight, hwSize, rankStats, nbSymbolsPtr, tableLogPtr, src, srcSize, workSpace, wkspSize, 0);
    }

    static BMI2_TARGET_ATTRIBUTE size_t HUF_readStats_body_bmi2(BYTE* huffWeight, size_t hwSize, U32* rankStats,
    U32* nbSymbolsPtr, U32* tableLogPtr,
    const void* src, size_t srcSize,
    void* workSpace, size_t wkspSize)
    {
    return HUF_readStats_body(huffWeight, hwSize, rankStats, nbSymbolsPtr, tableLogPtr, src, srcSize, workSpace, wkspSize, 1);
    }

    size_t HUF_readStats_wksp(BYTE* huffWeight, size_t hwSize, U32* rankStats,
    U32* nbSymbolsPtr, U32* tableLogPtr,
    const void* src, size_t srcSize,
    void* workSpace, size_t wkspSize,
    int flags)
    {

    if (flags & HUF_flags_bmi2) {
    return HUF_readStats_body_bmi2(huffWeight, hwSize, rankStats, nbSymbolsPtr, tableLogPtr, src, srcSize, workSpace, wkspSize);
    }

    (void)flags;
    return HUF_readStats_body_default(huffWeight, hwSize, rankStats, nbSymbolsPtr, tableLogPtr, src, srcSize, workSpace, wkspSize);
    }
