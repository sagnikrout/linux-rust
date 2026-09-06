//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/tests/test-utils.h
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
// Test utility functions shared by the crypto library tests.
//
// For now this is simply a header that's included into the KUnit test suites
// that need it.  If this gets too large it could be made its own translation
// unit and libcrypto_test_utils module, but that seems overkill for now.
//

//
// Allocate a KUnit-managed buffer that has length @size bytes (> 0) immediately
// followed by an unmapped page, and assert that the allocation succeeds.
//
extern "C" {
    pub fn memcpy(_arg: dst, _arg: src, _arg: size) -> return;
}
//
// This is a simple linear congruential generator.  It is used only for testing,
// which does not require cryptographically secure random numbers.  A hard-coded
// algorithm is used instead of <linux/prandom.h> so that it matches the
// algorithm used by the test vector generation script.  This allows the input
// data in random test vectors to be concisely stored as just the seed.
//
// Generate a random length, preferring small lengths.
extern "C" {
    pub fn min(128: rand32() %, _arg: max_offset) -> return;
}
