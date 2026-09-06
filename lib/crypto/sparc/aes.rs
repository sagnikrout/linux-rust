//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/sparc/aes.h
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
// AES accelerated using the sparc64 aes opcodes
//
// Copyright (C) 2008, Intel Corp.
// Copyright (c) 2010, Intel Corporation.
// Copyright 2026 Google LLC
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_aes_opcodes) -> static __ro_after_init;
}
extern "C" {
    pub fn aes_sparc64_encrypt_128(key: *const u64, input: *const u32, output: *mut u32);
}
extern "C" {
    pub fn aes_sparc64_encrypt_192(key: *const u64, input: *const u32, output: *mut u32);
}
extern "C" {
    pub fn aes_sparc64_encrypt_256(key: *const u64, input: *const u32, output: *mut u32);
}
extern "C" {
    pub fn aes_sparc64_decrypt_128(key: *const u64, input: *const u32, output: *mut u32);
}
extern "C" {
    pub fn aes_sparc64_decrypt_192(key: *const u64, input: *const u32, output: *mut u32);
}
extern "C" {
    pub fn aes_sparc64_decrypt_256(key: *const u64, input: *const u32, output: *mut u32);
}
//
// Note that nothing needs to be written to inv_k (if it's
// non-NULL) here, since the SPARC64 assembly code uses
// k->sparc_rndkeys for both encryption and decryption.
//

extern "C" {
    pub fn __volatile__(%%asr26: "rd, (cfr): %0" : "=r") -> __asm__;
}
