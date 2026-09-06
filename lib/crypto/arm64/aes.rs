//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/arm64/aes.h
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
// AES block cipher, optimized for ARM64
//
// Copyright (C) 2013 - 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
// Copyright 2026 Google LLC
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_neon) -> static __ro_after_init;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_aes) -> static __ro_after_init;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_block {
    pub b: [u8; AES_BLOCK_SIZE],
}

extern "C" {
    pub fn __aes_ce_sub(l: u32) -> asmlinkage u32;
}
//
// Expand an AES key using the crypto extensions if supported and usable or
// generic code otherwise.  The expanded key format is compatible between the
// two cases.  The outputs are @rndkeys (required) and @inv_rndkeys (optional).
//
// The AES key schedule round constants
//
// Generate the decryption keys for the Equivalent Inverse
// Cipher.  This involves reversing the order of the round
// keys, and applying the Inverse Mix Columns transformation on
// all but the first and the last one.
//
// This is here temporarily until the remaining AES mode implementations are
// migrated from arch/arm64/crypto/ to lib/crypto/arm64/.
//

