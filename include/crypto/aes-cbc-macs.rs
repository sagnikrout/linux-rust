//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/aes-cbc-macs.h
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
// Support for AES-CMAC, AES-XCBC-MAC, and AES-CBC-MAC
//
// Copyright 2026 Google LLC
//

//
// struct aes_cmac_key - Prepared key for AES-CMAC or AES-XCBC-MAC
// @aes: The AES key for cipher block chaining
// @k_final: Finalization subkeys for the final block.
// k_final[0] (CMAC K1, XCBC-MAC K2) is used if it's a full block.
// k_final[1] (CMAC K2, XCBC-MAC K3) is used if it's a partial block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_cmac_key {
    pub aes: aes_enckey,
    pub b: [u8; AES_BLOCK_SIZE],
    pub w: [__be64; 2],
    pub k_final: [}; 2],
}

//
// aes_cmac_zeroize_key() - Zeroize an aes_cmac_key structure
// @key: The location of the key structure that should be zeroized
//
// Explicitly fills the aes_cmac_key with zeroes. This should be done once
// the key is not required anymore to avoid that its contents are leaked
// on the stack or heap (if not using kfree_sensitive()).
//
// struct aes_cmac_ctx - Context for computing an AES-CMAC or AES-XCBC-MAC value
// @key: Pointer to the key struct.  A pointer is used rather than a copy of the
// struct, since the key struct size may be large.  It is assumed that the
// key lives at least as long as the context.
// @partial_len: Number of bytes that have been XOR'ed into @h since the last
// AES encryption.  This is 0 if no data has been processed yet,
// or between 1 and AES_BLOCK_SIZE inclusive otherwise.
// @h: The current chaining value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_cmac_ctx {
    pub key: *const aes_cmac_key,
    pub partial_len: usize,
    pub h: [u8; AES_BLOCK_SIZE],
}

//
// aes_cmac_zeroize_ctx() - Zeroize an aes_cmac_ctx structure
// @ctx: The location of the context that should be zeroized
//
// Explicitly fills the aes_cmac_ctx with zeroes. This should be done once
// the context is not required anymore to avoid that its contents are
// leaked on the stack or heap. Only required if not using aes_cmac_final().
//
// aes_cmac_preparekey() - Prepare a key for AES-CMAC
// @key: (output) The key struct to initialize
// @in_key: The raw AES key
// @key_len: Length of the raw key in bytes.  The supported values are
// AES_KEYSIZE_128, AES_KEYSIZE_192, and AES_KEYSIZE_256.
//
// On success, the caller should ensure that the prepared key is zeroized
// at the end of its lifetime, e.g. by calling aes_cmac_zeroize_key() or
// kfree_sensitive().
//
// Context: Any context.
// Return: 0 on success or -EINVAL if the given key length is invalid.  No other
// errors are possible, so callers that always pass a valid key length
// don't need to check for errors.
//
// aes_xcbcmac_preparekey() - Prepare a key for AES-XCBC-MAC
// @key: (output) The key struct to initialize
// @in_key: The raw key.  As per the AES-XCBC-MAC specification (RFC 3566), this
// is 128 bits, matching the internal use of AES-128.
//
// AES-XCBC-MAC and AES-CMAC are the same except for the key preparation.  After
// that step, AES-XCBC-MAC is supported via the aes_cmac_* functions.
//
// New users should use AES-CMAC instead of AES-XCBC-MAC.
//
// Context: Any context.
//
// aes_cmac_init() - Start computing an AES-CMAC or AES-XCBC-MAC value
// @ctx: (output) The context to initialize
// @key: The key to use.  Note that a pointer to the key is saved in the
// context, so the key must live at least as long as the context.
//
// This supports both AES-CMAC and AES-XCBC-MAC.  Which one is done depends on
// whether aes_cmac_preparekey() or aes_xcbcmac_preparekey() was called.
//
// The caller should ensure that the context is zeroized at the end of its
// lifetime, e.g. by calling aes_cmac_final() or aes_cmac_zeroize_ctx().
//
// ctx = (struct aes_cmac_ctx){ .key = key };
//
// aes_cmac_update() - Update an AES-CMAC or AES-XCBC-MAC context with more data
// @ctx: The context to update; must have been initialized
// @data: The message data
// @data_len: The data length in bytes.  Doesn't need to be block-aligned.
//
// This can be called any number of times.
//
// Context: Any context.
//
extern "C" {
    pub fn aes_cmac_update(ctx: *mut aes_cmac_ctx, data: *const u8, data_len: usize);
}
//
// aes_cmac_final() - Finish computing an AES-CMAC or AES-XCBC-MAC value
// @ctx: The context to finalize; must have been initialized
// @out: (output) The resulting MAC
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn aes_cmac_final(ctx: *mut aes_cmac_ctx, AES_BLOCK_SIZE]: u8 out[at_least);
}
//
// aes_cmac() - Compute AES-CMAC or AES-XCBC-MAC in one shot
// @key: The key to use
// @data: The message data
// @data_len: The data length in bytes
// @out: (output) The resulting AES-CMAC or AES-XCBC-MAC value
//
// This supports both AES-CMAC and AES-XCBC-MAC.  Which one is done depends on
// whether aes_cmac_preparekey() or aes_xcbcmac_preparekey() was called.
//
// Context: Any context.
//
// AES-CBC-MAC support.  This is provided only for use by the implementation of
// AES-CCM.  It should have no other users.  Warning: unlike AES-CMAC and
// AES-XCBC-MAC, AES-CBC-MAC isn't a secure MAC for variable-length messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_cbcmac_ctx {
    pub key: *const aes_enckey,
    pub partial_len: usize,
    pub h: [u8; AES_BLOCK_SIZE],
}

// ctx = (struct aes_cbcmac_ctx){ .key = key };
