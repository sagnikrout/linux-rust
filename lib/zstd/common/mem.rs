//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/mem.h
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

// Macro flag: #define MEM_H_MODULE
// -
// Dependencies
//

// -
// Compiler specifics
//

// -
// Basic Types
//
pub type BYTE = u8;
pub type U8 = u8;
pub type S8 = i8;
pub type U16 = u16;
pub type S16 = i16;
pub type U32 = u32;
pub type S32 = i32;
pub type U64 = u64;
pub type S64 = i64;
// -
// Memory I/O API
//
// === Static platform detection ===
extern "C" {
    pub fn MEM_32bits() -> MEM_STATIC unsigned;
}
extern "C" {
    pub fn MEM_64bits() -> MEM_STATIC unsigned;
}
extern "C" {
    pub fn MEM_isLittleEndian() -> MEM_STATIC unsigned;
}
// === Native unaligned read/write ===
extern "C" {
    pub fn MEM_read16(memPtr: *const *const c_void) -> MEM_STATIC U16;
}
extern "C" {
    pub fn MEM_read32(memPtr: *const *const c_void) -> MEM_STATIC U32;
}
extern "C" {
    pub fn MEM_read64(memPtr: *const *const c_void) -> MEM_STATIC U64;
}
extern "C" {
    pub fn MEM_readST(memPtr: *const *const c_void) -> MEM_STATIC size_t;
}
extern "C" {
    pub fn MEM_write16(memPtr: *mut *mut c_void, value: U16) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_write32(memPtr: *mut *mut c_void, value: U32) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_write64(memPtr: *mut *mut c_void, value: U64) -> MEM_STATIC void;
}
// === Little endian unaligned read/write ===
extern "C" {
    pub fn MEM_readLE16(memPtr: *const *const c_void) -> MEM_STATIC U16;
}
extern "C" {
    pub fn MEM_readLE24(memPtr: *const *const c_void) -> MEM_STATIC U32;
}
extern "C" {
    pub fn MEM_readLE32(memPtr: *const *const c_void) -> MEM_STATIC U32;
}
extern "C" {
    pub fn MEM_readLE64(memPtr: *const *const c_void) -> MEM_STATIC U64;
}
extern "C" {
    pub fn MEM_readLEST(memPtr: *const *const c_void) -> MEM_STATIC size_t;
}
extern "C" {
    pub fn MEM_writeLE16(memPtr: *mut *mut c_void, val: U16) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_writeLE24(memPtr: *mut *mut c_void, val: U32) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_writeLE32(memPtr: *mut *mut c_void, val32: U32) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_writeLE64(memPtr: *mut *mut c_void, val64: U64) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_writeLEST(memPtr: *mut *mut c_void, val: usize) -> MEM_STATIC void;
}
// === Big endian unaligned read/write ===
extern "C" {
    pub fn MEM_readBE32(memPtr: *const *const c_void) -> MEM_STATIC U32;
}
extern "C" {
    pub fn MEM_readBE64(memPtr: *const *const c_void) -> MEM_STATIC U64;
}
extern "C" {
    pub fn MEM_readBEST(memPtr: *const *const c_void) -> MEM_STATIC size_t;
}
extern "C" {
    pub fn MEM_writeBE32(memPtr: *mut *mut c_void, val32: U32) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_writeBE64(memPtr: *mut *mut c_void, val64: U64) -> MEM_STATIC void;
}
extern "C" {
    pub fn MEM_writeBEST(memPtr: *mut *mut c_void, val: usize) -> MEM_STATIC void;
}
// === Byteswap ===
extern "C" {
    pub fn MEM_swap32(in: U32) -> MEM_STATIC U32;
}
extern "C" {
    pub fn MEM_swap64(in: U64) -> MEM_STATIC U64;
}
extern "C" {
    pub fn MEM_swapST(in: usize) -> MEM_STATIC size_t;
}
// -
// Memory I/O Implementation
//

pub const MEM_LITTLE_ENDIAN: c_int = 1;

pub const MEM_LITTLE_ENDIAN: c_int = 0;

extern "C" {
    pub fn get_unaligned()memPtr: *const (U16) -> return;
}
extern "C" {
    pub fn get_unaligned()memPtr: *const (U32) -> return;
}
extern "C" {
    pub fn get_unaligned()memPtr: *const (U64) -> return;
}
extern "C" {
    pub fn get_unaligned()memPtr: *const (size_t) -> return;
}
// === Little endian r/w ===
extern "C" {
    pub fn get_unaligned_le16(_arg: memPtr) -> return;
}
extern "C" {
    pub fn MEM_readLE16(16: *const *const memPtr) + (((BYTE )memPtr)[2] <<) -> return;
}
extern "C" {
    pub fn get_unaligned_le32(_arg: memPtr) -> return;
}
extern "C" {
    pub fn get_unaligned_le64(_arg: memPtr) -> return;
}
// === Big endian r/w ===
extern "C" {
    pub fn get_unaligned_be32(_arg: memPtr) -> return;
}
extern "C" {
    pub fn get_unaligned_be64(_arg: memPtr) -> return;
}
extern "C" {
    pub fn swab32(_arg: in) -> return;
}
extern "C" {
    pub fn swab64(_arg: in) -> return;
}
