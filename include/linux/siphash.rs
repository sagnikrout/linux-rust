//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/siphash.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// Copyright (C) 2016-2022 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//
// SipHash: a fast short-input PRF
// https://131002.net/siphash
//
// This implementation is specifically for SipHash2-4 for a secure PRF
// and HalfSipHash1-3/SipHash1-3 for an insecure PRF only suitable for
// hashtables.
//

extern "C" {
    pub fn __siphash_aligned(data: *const c_void, len: usize, key: *const siphash_key_t) -> u64;
}
extern "C" {
    pub fn __siphash_unaligned(data: *const c_void, len: usize, key: *const siphash_key_t) -> u64;
}
extern "C" {
    pub fn siphash_1u64(a: u64, key: *const siphash_key_t) -> u64;
}
extern "C" {
    pub fn siphash_2u64(a: u64, b: u64, key: *const siphash_key_t) -> u64;
}
extern "C" {
    pub fn siphash_1u32(a: u32, key: *const siphash_key_t) -> u64;
}
extern "C" {
    pub fn siphash_1u64(a: (u64)b << 32 |, _arg: key) -> return;
}
extern "C" {
    pub fn siphash_2u64(a: (u64)b << 32 |, c: (u64)d << 32 |, _arg: key) -> return;
}
extern "C" {
    pub fn siphash_1u32()data): *const le32_to_cpup((__le32, _arg: key) -> return;
}
extern "C" {
    pub fn siphash_1u64(_arg: le64_to_cpu(data[0]), _arg: key) -> return;
}
extern "C" {
    pub fn __siphash_aligned(_arg: data, _arg: len, _arg: key) -> return;
}
//
// siphash - compute 64-bit siphash PRF value
// @data: buffer to hash
// @size: size of @data
// @key: the siphash key
//
extern "C" {
    pub fn __siphash_unaligned(_arg: data, _arg: len, _arg: key) -> return;
}
extern "C" {
    pub fn ___siphash_aligned(_arg: data, _arg: len, _arg: key) -> return;
}

extern "C" {
    pub fn hsiphash_1u32(a: u32, key: *const hsiphash_key_t) -> u32;
}
extern "C" {
    pub fn hsiphash_2u32(a: u32, b: u32, key: *const hsiphash_key_t) -> u32;
}
extern "C" {
    pub fn hsiphash_1u32(_arg: le32_to_cpu(data[0]), _arg: key) -> return;
}
extern "C" {
    pub fn __hsiphash_aligned(_arg: data, _arg: len, _arg: key) -> return;
}
//
// hsiphash - compute 32-bit hsiphash PRF value
// @data: buffer to hash
// @size: size of @data
// @key: the hsiphash key
//
extern "C" {
    pub fn __hsiphash_unaligned(_arg: data, _arg: len, _arg: key) -> return;
}
extern "C" {
    pub fn ___hsiphash_aligned(_arg: data, _arg: len, _arg: key) -> return;
}
//
// These macros expose the raw SipHash and HalfSipHash permutations.
// Do not use them directly! If you think you have a use for them,
// be sure to CC the maintainer of this file explaining why.
//

pub const SIPHASH_CONST_0: c_uint = 0x736f6d6570736575ULL;
pub const SIPHASH_CONST_1: c_uint = 0x646f72616e646f6dULL;
pub const SIPHASH_CONST_2: c_uint = 0x6c7967656e657261ULL;
pub const SIPHASH_CONST_3: c_uint = 0x7465646279746573ULL;

pub const HSIPHASH_CONST_2: c_uint = 0x6c796765U;
pub const HSIPHASH_CONST_3: c_uint = 0x74656462U;
