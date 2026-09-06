//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/bitstream.h
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
// bitstream
// Part of FSE library
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

// Macro flag: #define BITSTREAM_H_MODULE
//
// This API consists of small unitary functions, which must be inlined for best performance.
// Since link-time-optimization is not available for all compilers,
// these functions are defined into a .h to be included.
//
// -
// Dependencies
//

// =========================================
// Target specific
pub const STREAM_ACCUMULATOR_MIN_32: c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: c_int = 57;

// -
// bitStream encoding API (write forward)
//
pub type BitContainerType = usize;
// bitStream can mix input from multiple sources.
// A critical property of these streams is that they encode and decode in **reverse** direction.
// So the first bit sequence you add will be the last to be read, like a LIFO stack.
//
extern "C" {
    pub fn BIT_initCStream(bitC: *mut *mut BIT_CStream_t, dstBuffer: *mut *mut c_void, dstCapacity: usize) -> MEM_STATIC size_t;
}
extern "C" {
    pub fn BIT_addBits(bitC: *mut *mut BIT_CStream_t, value: BitContainerType, nbBits: unsigned) -> MEM_STATIC void;
}
extern "C" {
    pub fn BIT_flushBits(bitC: *mut *mut BIT_CStream_t) -> MEM_STATIC void;
}
extern "C" {
    pub fn BIT_closeCStream(bitC: *mut *mut BIT_CStream_t) -> MEM_STATIC size_t;
}
// Start with initCStream, providing the size of buffer to write into.
// bitStream will never write outside of this buffer.
// `dstCapacity` must be >= sizeof(bitD->bitContainer), otherwise @return will be an error code.
//
// bits are first added to a local register.
// Local register is BitContainerType, 64-bits on 64-bits systems, or 32-bits on 32-bits systems.
// Writing data into memory is an explicit operation, performed by the flushBits function.
// Hence keep track how many bits are potentially stored into local register to avoid register overflow.
// After a flushBits, a maximum of 7 bits might still be stored into local register.
//
// Avoid storing elements of more than 24 bits if you want compatibility with 32-bits bitstream readers.
//
// Last operation is to close the bitStream.
// The function returns the final size of CStream in bytes.
// If data couldn't fit into `dstBuffer`, it will return a 0 ( == not storable)
//
// -
// bitStream decoding API (read backward)
//
extern "C" {
    pub fn BIT_initDStream(bitD: *mut *mut BIT_DStream_t, srcBuffer: *const *const c_void, srcSize: usize) -> MEM_STATIC size_t;
}
extern "C" {
    pub fn BIT_readBits(bitD: *mut *mut BIT_DStream_t, nbBits: unsigned) -> MEM_STATIC BitContainerType;
}
extern "C" {
    pub fn BIT_reloadDStream(bitD: *mut *mut BIT_DStream_t) -> MEM_STATIC BIT_DStream_status;
}
extern "C" {
    pub fn BIT_endOfDStream(bitD: *const *const BIT_DStream_t) -> MEM_STATIC unsigned;
}
// Start by invoking BIT_initDStream().
// A chunk of the bitStream is then stored into a local register.
// Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (BitContainerType).
// You can then retrieve bitFields stored into the local register, **in reverse order**.
// Local register is explicitly reloaded from memory by the BIT_reloadDStream() method.
// A reload guarantee a minimum of ((8*sizeof(bitD->bitContainer))-7) bits when its result is BIT_DStream_unfinished.
// Otherwise, it can be less than that, so proceed accordingly.
// Checking if DStream has reached its end can be performed with BIT_endOfDStream().
//
// -
// unsafe API
//
extern "C" {
    pub fn BIT_addBitsFast(bitC: *mut *mut BIT_CStream_t, value: BitContainerType, nbBits: unsigned) -> MEM_STATIC void;
}
// faster, but works only if value is "clean", meaning all high bits above nbBits are 0
extern "C" {
    pub fn BIT_flushBitsFast(bitC: *mut *mut BIT_CStream_t) -> MEM_STATIC void;
}
// unsafe version; does not check buffer overflow
extern "C" {
    pub fn BIT_readBitsFast(bitD: *mut *mut BIT_DStream_t, nbBits: unsigned) -> MEM_STATIC size_t;
}
// faster, but works only if nbBits >= 1
// =====    Local Constants   =====

// -
// bitStream encoding
//
// ! BIT_initCStream() :
// `dstCapacity` must be > sizeof(size_t)
// @return : 0 if success,
// otherwise an error code (can be tested using ERR_isError())
// ! BIT_addBits() :
// can add up to 31 bits into `bitC`.
// Note : does not check for register overflow !
// ! BIT_addBitsFast() :
// works only if `value` is _clean_,
// meaning all high bits above nbBits are 0
// ! BIT_flushBitsFast() :
// assumption : bitContainer has not overflowed
// unsafe version; does not check buffer overflow
// ! BIT_flushBits() :
// assumption : bitContainer has not overflowed
// safe version; check for buffer overflow, and prevents it.
// note : does not signal buffer overflow.
// overflow will be revealed later on using BIT_closeCStream()
// ! BIT_closeCStream() :
// @return : size of CStream, in bytes,
// or 0 if it could not fit into dstBuffer
// -
// bitStream decoding
//
// ! BIT_initDStream() :
// Initialize a BIT_DStream_t.
// `bitD` : a pointer to an already allocated BIT_DStream_t structure.
// `srcSize` must be the *exact* size of the bitStream, in bytes.
// @return : size of stream (== srcSize), or an errorCode if a problem is detected
//
// if start > regMask, bitstream is corrupted, and result is undefined
// x86 transform & ((1 << nbBits) - 1) to bzhi instruction, it is better
// than accessing memory. When bmi2 instruction is not present, we consider
// such cpus old (pre-Haswell, 2013) and their performance is not of that
// importance.
//

// ! BIT_lookBits() :
// Provides next n bits from local register.
// local register is not modified.
// On 32-bits, maxNbBits==24.
// On 64-bits, maxNbBits==56.
// @return : value extracted
// arbitrate between double-shift and shift+mask

// if bitD->bitsConsumed + nbBits > sizeof(bitD->bitContainer)*8,
// bitstream is likely corrupted, and result is undefined
extern "C" {
    pub fn BIT_getMiddleBits(_arg: bitD->bitContainer, nbBits: *mut *mut (sizeof(bitD->bitContainer)8) - bitD->bitsConsumed -, _arg: nbBits) -> return;
}

// this code path is slower on my os-x laptop

// ! BIT_lookBitsFast() :
// unsafe version; only works if nbBits >= 1
// ! BIT_readBits() :
// Read (consume) next n bits from local register and update.
// Pay attention to not read more than nbBits contained into local register.
// @return : extracted value.
// ! BIT_readBitsFast() :
// unsafe version; only works if nbBits >= 1
// ! BIT_reloadDStream_internal() :
// Simple variant of BIT_reloadDStream(), with two conditions:
// 1. bitstream is valid : bitsConsumed <= sizeof(bitD->bitContainer)*8
// 2. look window is valid after shifted down : bitD->ptr >= bitD->start
//
// ! BIT_reloadDStreamFast() :
// Similar to BIT_reloadDStream(), but with two differences:
// 1. bitsConsumed <= sizeof(bitD->bitContainer)*8 must hold!
// 2. Returns BIT_DStream_overflow when bitD->ptr < bitD->limitPtr, at this
// point you must use BIT_reloadDStream() to reload.
//
extern "C" {
    pub fn BIT_reloadDStream_internal(_arg: bitD) -> return;
}
// ! BIT_reloadDStream() :
// Refill `bitD` from buffer previously set in BIT_initDStream() .
// This function is safe, it guarantees it will not never beyond src buffer.
// @return : status of `BIT_DStream_t` internal register.
// when status == BIT_DStream_unfinished, internal register is filled with at least 25 or 57 bits
// note : once in overflow mode, a bitstream remains in this mode until it's reset
// overflow detected, erroneous scenario or end of stream: no update
extern "C" {
    pub fn BIT_reloadDStream_internal(_arg: bitD) -> return;
}
// reached end of bitStream => no update
// start < ptr < limitPtr => cautious update
// ! BIT_endOfDStream() :
// @return : 1 if DStream has _exactly_ reached its end (all bits consumed).
//
