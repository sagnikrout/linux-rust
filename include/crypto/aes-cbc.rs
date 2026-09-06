//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/aes-cbc.h
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
// AES-CBC and AES-CBC-CTS unauthenticated encryption and decryption
//
// Copyright 2026 Google LLC
//

//
// aes_cbc_encrypt() - Encrypt data using AES-CBC
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to encrypt.  Must be a multiple of AES_BLOCK_SIZE.
// @iv: The initialization vector.  It is updated with the next value, i.e. the
// last ciphertext block (or left unchanged if @len == 0).
// @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
//
// This supports incremental encryption.  The length of each chunk must be a
// multiple of AES_BLOCK_SIZE, and the updated @iv must be passed in each time.
//
// Context: Any context.
//
// aes_cbc_decrypt() - Decrypt data using AES-CBC
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to decrypt.  Must be a multiple of AES_BLOCK_SIZE.
// @iv: The initialization vector.  It is updated with the next value, i.e. the
// last ciphertext block (or left unchanged if @len == 0).
// @key: The key, already prepared using aes_preparekey()
//
// This supports incremental decryption.  The length of each chunk must be a
// multiple of AES_BLOCK_SIZE, and the updated @iv must be passed in each time.
//
// Context: Any context.
//
// aes_cbc_cts_encrypt() - Encrypt data using AES-CBC-CTS (CS3 variant)
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to encrypt, at least AES_BLOCK_SIZE
// @iv: The initialization vector, clobbered by this function
// @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
//
// Context: Any context.
//
// aes_cbc_cts_decrypt() - Decrypt data using AES-CBC-CTS (CS3 variant)
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to decrypt, at least AES_BLOCK_SIZE
// @iv: The initialization vector, clobbered by this function
// @key: The key, already prepared using aes_preparekey()
//
// Context: Any context.
//
