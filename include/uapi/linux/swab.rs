//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/swab.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// casts are necessary for constants, because we never know how for sure
// how U/UL/ULL map to __u16, __u32, __u64. At least not in a portable way.
//

//
// Implement the following as inlines, but define the interface using
// macros to allow constant folding when possible:
// ___swab16, ___swab32, ___swab64, ___swahw32, ___swahb32
//

extern "C" {
    pub fn __arch_swab16(_arg: val) -> return;
}

extern "C" {
    pub fn ___constant_swab16(_arg: val) -> return;
}

extern "C" {
    pub fn __arch_swab32(_arg: val) -> return;
}

extern "C" {
    pub fn ___constant_swab32(_arg: val) -> return;
}

extern "C" {
    pub fn __arch_swab64(_arg: val) -> return;
}

extern "C" {
    pub fn ___constant_swab64(_arg: val) -> return;
}

extern "C" {
    pub fn __arch_swahw32(_arg: val) -> return;
}

extern "C" {
    pub fn ___constant_swahw32(_arg: val) -> return;
}

extern "C" {
    pub fn __arch_swahb32(_arg: val) -> return;
}

extern "C" {
    pub fn ___constant_swahb32(_arg: val) -> return;
}

//
// __swab16 - return a byteswapped 16-bit value
// @x: value to byteswap
//

//
// __swab32 - return a byteswapped 32-bit value
// @x: value to byteswap
//

//
// __swab64 - return a byteswapped 64-bit value
// @x: value to byteswap
//

extern "C" {
    pub fn __swab64(_arg: y) -> return;
}

extern "C" {
    pub fn __swab32(_arg: y) -> return;
}

//
// __swahw32 - return a word-swapped 32-bit value
// @x: value to wordswap
//
// __swahw32(0x12340000) is 0x00001234
//

//
// __swahb32 - return a high and low byte-swapped 32-bit value
// @x: value to byteswap
//
// __swahb32(0x12345678) is 0x34127856
//

//
// __swab16p - return a byteswapped 16-bit value from a pointer
// @p: pointer to a naturally-aligned 16-bit value
//

extern "C" {
    pub fn __arch_swab16p(_arg: p) -> return;
}

extern "C" {
    pub fn __swab16(_arg: *mut p) -> return;
}

//
// __swab32p - return a byteswapped 32-bit value from a pointer
// @p: pointer to a naturally-aligned 32-bit value
//

extern "C" {
    pub fn __arch_swab32p(_arg: p) -> return;
}

extern "C" {
    pub fn __swab32(_arg: *mut p) -> return;
}

//
// __swab64p - return a byteswapped 64-bit value from a pointer
// @p: pointer to a naturally-aligned 64-bit value
//

extern "C" {
    pub fn __arch_swab64p(_arg: p) -> return;
}

extern "C" {
    pub fn __swab64(_arg: *mut p) -> return;
}

//
// __swahw32p - return a wordswapped 32-bit value from a pointer
// @p: pointer to a naturally-aligned 32-bit value
//
// See __swahw32() for details of wordswapping.
//

extern "C" {
    pub fn __arch_swahw32p(_arg: p) -> return;
}

extern "C" {
    pub fn __swahw32(_arg: *mut p) -> return;
}

//
// __swahb32p - return a high and low byteswapped 32-bit value from a pointer
// @p: pointer to a naturally-aligned 32-bit value
//
// See __swahb32() for details of high/low byteswapping.
//

extern "C" {
    pub fn __arch_swahb32p(_arg: p) -> return;
}

extern "C" {
    pub fn __swahb32(_arg: *mut p) -> return;
}

//
// __swab16s - byteswap a 16-bit value in-place
// @p: pointer to a naturally-aligned 16-bit value
//

// p = __swab16p(p);

//
// __swab32s - byteswap a 32-bit value in-place
// @p: pointer to a naturally-aligned 32-bit value
//

// p = __swab32p(p);

//
// __swab64s - byteswap a 64-bit value in-place
// @p: pointer to a naturally-aligned 64-bit value
//

// p = __swab64p(p);

//
// __swahw32s - wordswap a 32-bit value in-place
// @p: pointer to a naturally-aligned 32-bit value
//
// See __swahw32() for details of wordswapping
//

// p = __swahw32p(p);

//
// __swahb32s - high and low byteswap a 32-bit value in-place
// @p: pointer to a naturally-aligned 32-bit value
//
// See __swahb32() for details of high and low byte swapping
//

// p = __swahb32p(p);

