//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/s390/sha3.h
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
// SHA-3 optimized using the CP Assist for Cryptographic Functions (CPACF)
//
// Copyright 2025 Google LLC
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_sha3) -> static __ro_after_init;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_sha3_init_optim) -> static __ro_after_init;
}
//
// Note that KIMD assumes little-endian order of the state
// words.  sha3_state already uses that order, though, so
// there's no need for a byteswap.
//
// This case handles both SHA3-256 and SHAKE256, since
// they have the same block size.
//
// Passing zeroes into any of CPACF_KIMD_SHA3_* gives the plain
// Keccak-f permutation, which is what we want here.  Use
// SHA3-512 since it has the smallest block size.
//

//
// Since all the SHA-3 functions are in Message-Security-Assist
// Extension 6, just treat them as all or nothing.  This way we need
// only one static_key.
//

