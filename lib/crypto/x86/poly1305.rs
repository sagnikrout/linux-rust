//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/x86/poly1305.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_arch_internal {
    pub h: [u32; 5],
    pub is_base2_26: u32,
}

//
// The AVX code uses base 2^26, while the scalar code uses base 2^64. If we hit
// the unfortunate situation of using AVX and then having to go back to scalar
// -- because the user is silly and has called the update function from two
// separate contexts -- then we need to convert back to the original base before
// proceeding. It is possible to reason that the initial reduction below is
// sufficient given the implementation invariants. However, for an avoidance of
// doubt and because this is not performance critical, we do the full reduction
// anyway. Z3 proof of below function: https://xn--4db.cc/ltPtHCKN/py
//
// Unsigned Less Than: branchlessly produces 1 if a < b, else 0.

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: poly1305_use_avx) -> static __ro_after_init;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: poly1305_use_avx2) -> static __ro_after_init;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: poly1305_use_avx512) -> static __ro_after_init;
}
// SIMD disables preemption, so relax after processing each page.
//
// The AVX implementations have significant setup overhead (e.g. key
// power computation, kernel FPU enabling) which makes them slower for
// short messages.  Fall back to the scalar implementation for messages
// shorter than 288 bytes, unless the AVX-specific key setup has already
// been performed (indicated by ctx->is_base2_26).
//

// Skylake downclocks unacceptably much when using zmm, but later generations are fast.
