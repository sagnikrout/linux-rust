//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/blake2b.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blake2b_lengths {
    BLAKE2B_BLOCK_SIZE = 128,
    BLAKE2B_HASH_SIZE = 64,
    BLAKE2B_KEY_SIZE = 64,

    BLAKE2B_160_HASH_SIZE = 20,
    BLAKE2B_256_HASH_SIZE = 32,
    BLAKE2B_384_HASH_SIZE = 48,
    BLAKE2B_512_HASH_SIZE = 64,
}

//
// struct blake2b_ctx - Context for hashing a message with BLAKE2b
// @h: compression function state
// @t: block counter
// @f: finalization indicator
// @buf: partial block buffer; 'buflen' bytes are valid
// @buflen: number of bytes buffered in @buf
// @outlen: length of output hash value in bytes, at most BLAKE2B_HASH_SIZE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blake2b_ctx {
// 'h', 't', and 'f' are used in assembly code, so keep them as-is.
    pub h: [u64; 8],
    pub t: [u64; 2],
    pub f: [u64; 2],
    pub buf: [u8; BLAKE2B_BLOCK_SIZE],
    pub buflen: c_uint,
    pub outlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blake2b_iv {
    BLAKE2B_IV0 = 0x6A09E667F3BCC908ULL,
    BLAKE2B_IV1 = 0xBB67AE8584CAA73BULL,
    BLAKE2B_IV2 = 0x3C6EF372FE94F82BULL,
    BLAKE2B_IV3 = 0xA54FF53A5F1D36F1ULL,
    BLAKE2B_IV4 = 0x510E527FADE682D1ULL,
    BLAKE2B_IV5 = 0x9B05688C2B3E6C1FULL,
    BLAKE2B_IV6 = 0x1F83D9ABFB41BD6BULL,
    BLAKE2B_IV7 = 0x5BE0CD19137E2179ULL,
}

//
// blake2b_init() - Initialize a BLAKE2b context for a new message (unkeyed)
// @ctx: the context to initialize
// @outlen: length of output hash value in bytes, at most BLAKE2B_HASH_SIZE
//
// Context: Any context.
//
// blake2b_init_key() - Initialize a BLAKE2b context for a new message (keyed)
// @ctx: the context to initialize
// @outlen: length of output hash value in bytes, at most BLAKE2B_HASH_SIZE
// @key: the key
// @keylen: the key length in bytes, at most BLAKE2B_KEY_SIZE
//
// Context: Any context.
//
// blake2b_update() - Update a BLAKE2b context with message data
// @ctx: the context to update; must have been initialized
// @in: the message data
// @inlen: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
extern "C" {
    pub fn blake2b_update(ctx: *mut blake2b_ctx, in: *const u8, inlen: usize);
}
//
// blake2b_final() - Finish computing a BLAKE2b hash
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting BLAKE2b hash.  Its length will be equal to the
// @outlen that was passed to blake2b_init() or blake2b_init_key().
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn blake2b_final(ctx: *mut blake2b_ctx, out: *mut u8);
}
//
// blake2b() - Compute BLAKE2b hash in one shot
// @key: the key, or NULL for an unkeyed hash
// @keylen: the key length in bytes (at most BLAKE2B_KEY_SIZE), or 0 for an
// unkeyed hash
// @in: the message data
// @inlen: the data length in bytes
// @out: (output) the resulting BLAKE2b hash, with length @outlen
// @outlen: length of output hash value in bytes, at most BLAKE2B_HASH_SIZE
//
// Context: Any context.
//
