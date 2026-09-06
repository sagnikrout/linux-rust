//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/sha1.h
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
// Common values for SHA-1 algorithms
//

pub const SHA1_DIGEST_SIZE: c_int = 20;
pub const SHA1_BLOCK_SIZE: c_int = 64;

pub const SHA1_H0: c_uint = 0x67452301UL;
pub const SHA1_H1: c_uint = 0xefcdab89UL;
pub const SHA1_H2: c_uint = 0x98badcfeUL;
pub const SHA1_H3: c_uint = 0x10325476UL;
pub const SHA1_H4: c_uint = 0xc3d2e1f0UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha1_state {
    pub 4]: u32 state[SHA1_DIGEST_SIZE /,
    pub count: u64,
    pub buffer: [u8; SHA1_BLOCK_SIZE],
}

// State for the SHA-1 compression function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha1_block_state {
    pub 4]: u32 h[SHA1_DIGEST_SIZE /,
}

//
// struct sha1_ctx - Context for hashing a message with SHA-1
// @state: the compression function state
// @bytecount: number of bytes processed so far
// @buf: partial block buffer; bytecount % SHA1_BLOCK_SIZE bytes are valid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha1_ctx {
    pub state: sha1_block_state,
    pub bytecount: u64,
    pub buf: [u8; SHA1_BLOCK_SIZE],
}

//
// sha1_init() - Initialize a SHA-1 context for a new message
// @ctx: the context to initialize
//
// If you don't need incremental computation, consider sha1() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn sha1_init(ctx: *mut sha1_ctx);
}
//
// sha1_update() - Update a SHA-1 context with message data
// @ctx: the context to update; must have been initialized
// @data: the message data
// @len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
extern "C" {
    pub fn sha1_update(ctx: *mut sha1_ctx, data: *const u8, len: usize);
}
//
// sha1_final() - Finish computing a SHA-1 message digest
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting SHA-1 message digest
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn sha1_final(ctx: *mut sha1_ctx, SHA1_DIGEST_SIZE]: u8 out[at_least);
}
//
// sha1() - Compute SHA-1 message digest in one shot
// @data: the message data
// @len: the data length in bytes
// @out: (output) the resulting SHA-1 message digest
//
// Context: Any context.
//
extern "C" {
    pub fn sha1(data: *const u8, len: usize, SHA1_DIGEST_SIZE]: u8 out[at_least);
}
//
// struct hmac_sha1_key - Prepared key for HMAC-SHA1
// @istate: private
// @ostate: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha1_key {
    pub istate: sha1_block_state,
    pub ostate: sha1_block_state,
}

//
// struct hmac_sha1_ctx - Context for computing HMAC-SHA1 of a message
// @sha_ctx: private
// @ostate: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha1_ctx {
    pub sha_ctx: sha1_ctx,
    pub ostate: sha1_block_state,
}

//
// hmac_sha1_preparekey() - Prepare a key for HMAC-SHA1
// @key: (output) the key structure to initialize
// @raw_key: the raw HMAC-SHA1 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// Note: the caller is responsible for zeroizing both the struct hmac_sha1_key
// and the raw key once they are no longer needed.
//
// Context: Any context.
//
// hmac_sha1_init() - Initialize an HMAC-SHA1 context for a new message
// @ctx: (output) the HMAC context to initialize
// @key: the prepared HMAC key
//
// If you don't need incremental computation, consider hmac_sha1() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn hmac_sha1_init(ctx: *mut hmac_sha1_ctx, key: *const hmac_sha1_key);
}
//
// hmac_sha1_init_usingrawkey() - Initialize an HMAC-SHA1 context for a new
// message, using a raw key
// @ctx: (output) the HMAC context to initialize
// @raw_key: the raw HMAC-SHA1 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// If you don't need incremental computation, consider hmac_sha1_usingrawkey()
// instead.
//
// Context: Any context.
//
// hmac_sha1_update() - Update an HMAC-SHA1 context with message data
// @ctx: the HMAC context to update; must have been initialized
// @data: the message data
// @data_len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// hmac_sha1_final() - Finish computing an HMAC-SHA1 value
// @ctx: the HMAC context to finalize; must have been initialized
// @out: (output) the resulting HMAC-SHA1 value
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
// hmac_sha1() - Compute HMAC-SHA1 in one shot, using a prepared key
// @key: the prepared HMAC key
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA1 value
//
// If you're using the key only once, consider using hmac_sha1_usingrawkey().
//
// Context: Any context.
//
// hmac_sha1_usingrawkey() - Compute HMAC-SHA1 in one shot, using a raw key
// @raw_key: the raw HMAC-SHA1 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA1 value
//
// If you're using the key multiple times, prefer to use hmac_sha1_preparekey()
// followed by multiple calls to hmac_sha1() instead.
//
// Context: Any context.
//
