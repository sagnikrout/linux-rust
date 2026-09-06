//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crc/x86/crc32.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// x86-optimized CRC32 functions
//
// Copyright (C) 2008 Intel Corporation
// Copyright 2012 Xyratex Technology Limited
// Copyright 2024 Google LLC
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_crc32) -> static __ro_after_init;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_pclmulqdq) -> static __ro_after_init;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_vpclmul_avx512) -> static __ro_after_init;
}
extern "C" {
    pub fn crc32_le_base(_arg: crc, _arg: p, _arg: len) -> return;
}

//
// Use carryless multiply version of crc32c when buffer size is >= 512 to
// account for FPU state save/restore overhead.
//
pub const CRC32C_PCLMUL_BREAKEVEN: c_int = 512;
extern "C" {
    pub fn crc32c_x86_3way(crc: u32, buffer: *const u8, len: usize) -> asmlinkage u32;
}
extern "C" {
    pub fn crc32c_base(_arg: crc, _arg: p, _arg: len) -> return;
}
//
// Long length, the vector registers are usable, and the CPU is
// 64-bit and supports both CRC32 and PCLMULQDQ instructions.
// It is worthwhile to divide the data into multiple streams,
// CRC them independently, and combine them using PCLMULQDQ.
// crc32c_x86_3way() does this using 3 streams, which is the
// most that x86_64 CPUs have traditionally been capable of.
//
// However, due to improved VPCLMULQDQ performance on newer
// CPUs, use crc32_lsb_vpclmul_avx512() instead of
// crc32c_x86_3way() when the CPU supports VPCLMULQDQ and has a
// "good" implementation of AVX-512.
//
// Future work: the optimal strategy on Zen 3--5 is actually to
// use both crc32q and VPCLMULQDQ in parallel.  Unfortunately,
// different numbers of streams and vector lengths are optimal
// on each CPU microarchitecture, making it challenging to take
// advantage of this.  (Zen 5 even supports 7 parallel crc32q, a
// major upgrade.)  For now, just choose between
// crc32c_x86_3way() and crc32_lsb_vpclmul_avx512().  The latter
// is needed anyway for crc32_le(), so we just reuse it here.
//
// Short length, XMM registers unusable, or the CPU is 32-bit; but the
// CPU supports CRC32 instructions.  Just issue a single stream of CRC32
// instructions inline.  While this doesn't use the CPU's CRC32
// throughput very well, it avoids the need to combine streams.  Stream
// combination would be inefficient here.
//

