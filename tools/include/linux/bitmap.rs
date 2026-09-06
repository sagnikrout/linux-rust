//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/bitmap.h
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


// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn __bitmap_weight(bitmap: *const c_ulong, bits: c_int) -> c_uint;
}
extern "C" {
    pub fn __bitmap_set(map: *mut c_ulong, start: c_uint, len: c_int);
}
extern "C" {
    pub fn __bitmap_clear(map: *mut c_ulong, start: c_uint, len: c_int);
}

// dst = 0UL;
// dst = *src;
extern "C" {
    pub fn hweight_long(BITMAP_LAST_WORD_MASK(nbits): *mut *mut src &) -> return;
}
extern "C" {
    pub fn __bitmap_weight(_arg: src, _arg: nbits) -> return;
}
// dst = *src1 | *src2;
extern "C" {
    pub fn __bitmap_andnot(_arg: dst, _arg: src1, _arg: src2, _arg: nbits) -> return;
}
extern "C" {
    pub fn malloc(_arg: bitmap_size(nbits)) -> return;
}
//
// bitmap_zalloc - Allocate bitmap
// @nbits: Number of bits
//
extern "C" {
    pub fn calloc(_arg: 1, _arg: bitmap_size(nbits)) -> return;
}
//
// bitmap_free - Free bitmap
// @bitmap: pointer to bitmap
//
// bitmap_scnprintf - print bitmap list into buffer
// @bitmap: bitmap
// @nbits: size of bitmap
// @buf: buffer to store output
// @size: size of @buf
//
// bitmap_and - Do logical and on bitmaps
// @dst: resulting bitmap
// @src1: operand 1
// @src2: operand 2
// @nbits: size of bitmap
//
extern "C" {
    pub fn __bitmap_and(_arg: dst, _arg: src1, _arg: src2, _arg: nbits) -> return;
}

pub const BITMAP_MEM_ALIGNMENT: c_int = 8;

extern "C" {
    pub fn __bitmap_equal(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
extern "C" {
    pub fn __bitmap_intersects(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
extern "C" {
    pub fn __bitmap_subset(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
// map |= GENMASK(start + nbits - 1, start);
// map &= ~GENMASK(start + nbits - 1, start);
// dst = *src1 ^ *src2;
