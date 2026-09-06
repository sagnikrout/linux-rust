//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/aes-ctr.h
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
// AES-CTR and AES-XCTR stream ciphers
//
// Copyright 2026 Google LLC
//

//
// aes_ctr() - AES-CTR en/decryption
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to en/decrypt
// @ctr: The counter.  It will be incremented by ceil(@len / AES_BLOCK_SIZE).
// @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
//
// This implements AES in counter mode with a 128-bit big endian counter.
//
// This exists only for use by the implementation of modes built on top of CTR
// (e.g., GCM and CCM) and some legacy protocols that use CTR mode directly.
// Callers are expected to know how to use CTR mode appropriately, including
// choosing (key, counter) pairs appropriately to avoid keystream reuse.
//
// This supports incremental en/decryption.  The length of each non-final chunk
// must be a multiple of AES_BLOCK_SIZE, and the updated @ctr must be passed in
// each time.
//
// Context: Any context.
//
// aes_xctr() - AES-XCTR en/decryption
// @dst: The destination buffer.  Can be in-place or out-of-place.  For other
// overlaps the behavior is unspecified.
// @src: The source data
// @len: Number of bytes to en/decrypt
// @ctr: The block counter (in host endianness).  For the first call, set it to
// 1.  It will be incremented by ceil(@len / AES_BLOCK_SIZE).
// @iv: The initialization vector
// @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
//
// This implements AES in XOR Counter mode, as specified in the paper
// "Length-preserving encryption with HCTR2"
// (https://eprint.iacr.org/2021/1441.pdf).
//
// This exists only for use by the implementation of modes built on top of XCTR.
// Callers are expected to know how to use XCTR mode appropriately, including
// choosing (key, IV) pairs appropriately to avoid keystream reuse.
//
// This supports incremental en/decryption.  The length of each non-final chunk
// must be a multiple of AES_BLOCK_SIZE, and the updated @ctr must be passed in
// each time.
//
// Context: Any context.
//
