//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/utils.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Cryptographic utilities
//
// Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
//

extern "C" {
    pub fn __crypto_xor(dst: *mut u8, src1: *const u8, src2: *const u8, size: c_uint);
}
extern "C" {
    pub fn __crypto_memneq(a: *const c_void, b: *const c_void, size: usize) -> noinline unsigned long;
}
//
// crypto_memneq - Compare two areas of memory without leaking
// timing information.
//
// @a: One area of memory
// @b: Another area of memory
// @size: The size of the area.
//
// Returns 0 when data is equal, 1 otherwise.
//
