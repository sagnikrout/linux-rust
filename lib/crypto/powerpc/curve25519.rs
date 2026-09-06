//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/powerpc/curve25519.h
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
// Copyright 2024- IBM Corp.
//
// X25519 scalar multiplication with 51 bits limbs for PPC64le.
// Based on RFC7748 and AArch64 optimized implementation for X25519
// - Algorithm 1 Scalar multiplication of a variable point
//

extern "C" {
    pub fn x25519_fe51_mul(h: fe51, f: fe51, g: fe51) -> asmlinkage void;
}
extern "C" {
    pub fn x25519_fe51_sqr(h: fe51, f: fe51) -> asmlinkage void;
}
extern "C" {
    pub fn x25519_fe51_mul121666(h: fe51, f: fe51) -> asmlinkage void;
}
extern "C" {
    pub fn x25519_fe51_sqr_times(h: fe51, f: fe51, n: c_int) -> asmlinkage void;
}
extern "C" {
    pub fn x25519_fe51_frombytes(h: fe51, s: *const u8) -> asmlinkage void;
}
extern "C" {
    pub fn x25519_fe51_tobytes(s: *mut u8, h: fe51) -> asmlinkage void;
}
extern "C" {
    pub fn x25519_cswap(p: fe51, q: fe51, bit: c_uint) -> asmlinkage void;
}

//
// Prime = 2 ** 255 - 19, 255 bits
// (0x7fffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffed)
//
// Prime in 5 51-bit limbs
//
// Make sure 64-bit aligned.
//
