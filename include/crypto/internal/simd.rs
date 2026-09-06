//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/simd.h
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
//
// Shared crypto simd helpers
//

//
// crypto_simd_usable() - is it allowed at this time to use SIMD instructions or
// access the SIMD register file?
//
// This delegates to may_use_simd(), except that this also returns false if SIMD
// in crypto code has been temporarily disabled on this CPU by the crypto
// self-tests, in order to test the no-SIMD fallback code.  This override is
// currently limited to configurations where the "full" self-tests are enabled,
// because it might be a bit too invasive to be part of the "fast" self-tests.
//

