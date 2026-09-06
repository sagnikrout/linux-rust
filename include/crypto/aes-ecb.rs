//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/aes-ecb.h
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
// AES-ECB unauthenticated encryption and decryption
//
// Copyright 2026 Google LLC
//

//
// aes_ecb_encrypt() - Encrypt data using AES-ECB
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to encrypt.  Must be a multiple of AES_BLOCK_SIZE.
// @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
//
// ECB mode is insecure by itself.  This function exists only for compatibility
// with legacy protocols and for internal use by other modes.
//
// This supports incremental encryption, but the length of each chunk must be a
// multiple of AES_BLOCK_SIZE.
//
// Context: Any context.
//
extern "C" {
    pub fn aes_ecb_encrypt(dst: *mut u8, src: *const u8, len: usize, key: aes_encrypt_arg);
}
//
// aes_ecb_decrypt() - Decrypt data using AES-ECB
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to decrypt.  Must be a multiple of AES_BLOCK_SIZE.
// @key: The key, already prepared using aes_preparekey()
//
// ECB mode is insecure by itself.  This function exists only for compatibility
// with legacy protocols and for internal use by other modes.
//
// This supports incremental decryption, but the length of each chunk must be a
// multiple of AES_BLOCK_SIZE.
//
// Context: Any context.
//
