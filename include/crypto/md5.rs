//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/md5.h
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

pub const MD5_DIGEST_SIZE: c_int = 16;
pub const MD5_HMAC_BLOCK_SIZE: c_int = 64;
pub const MD5_BLOCK_SIZE: c_int = 64;
pub const MD5_BLOCK_WORDS: c_int = 16;
pub const MD5_HASH_WORDS: c_int = 4;
pub const MD5_STATE_SIZE: c_int = 24;
pub const MD5_H0: c_uint = 0x67452301UL;
pub const MD5_H1: c_uint = 0xefcdab89UL;
pub const MD5_H2: c_uint = 0x98badcfeUL;
pub const MD5_H3: c_uint = 0x10325476UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct md5_state {
    pub hash: [u32; MD5_HASH_WORDS],
    pub byte_count: u64,
    pub block: [u32; MD5_BLOCK_WORDS],
}

// State for the MD5 compression function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md5_block_state {
    pub h: [u32; MD5_HASH_WORDS],
}

//
// struct md5_ctx - Context for hashing a message with MD5
// @state: the compression function state
// @bytecount: number of bytes processed so far
// @buf: partial block buffer; bytecount % MD5_BLOCK_SIZE bytes are valid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md5_ctx {
    pub state: md5_block_state,
    pub bytecount: u64,
    pub __aligned(__alignof__(__le64)): u8 buf[MD5_BLOCK_SIZE],
}

//
// md5_init() - Initialize an MD5 context for a new message
// @ctx: the context to initialize
//
// If you don't need incremental computation, consider md5() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn md5_init(ctx: *mut md5_ctx);
}
//
// md5_update() - Update an MD5 context with message data
// @ctx: the context to update; must have been initialized
// @data: the message data
// @len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
extern "C" {
    pub fn md5_update(ctx: *mut md5_ctx, data: *const u8, len: usize);
}
//
// md5_final() - Finish computing an MD5 message digest
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting MD5 message digest
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn md5_final(ctx: *mut md5_ctx, MD5_DIGEST_SIZE]: u8 out[at_least);
}
//
// md5() - Compute MD5 message digest in one shot
// @data: the message data
// @len: the data length in bytes
// @out: (output) the resulting MD5 message digest
//
// Context: Any context.
//
extern "C" {
    pub fn md5(data: *const u8, len: usize, MD5_DIGEST_SIZE]: u8 out[at_least);
}
//
// struct hmac_md5_key - Prepared key for HMAC-MD5
// @istate: private
// @ostate: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_md5_key {
    pub istate: md5_block_state,
    pub ostate: md5_block_state,
}

//
// struct hmac_md5_ctx - Context for computing HMAC-MD5 of a message
// @hash_ctx: private
// @ostate: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_md5_ctx {
    pub hash_ctx: md5_ctx,
    pub ostate: md5_block_state,
}

//
// hmac_md5_preparekey() - Prepare a key for HMAC-MD5
// @key: (output) the key structure to initialize
// @raw_key: the raw HMAC-MD5 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// Note: the caller is responsible for zeroizing both the struct hmac_md5_key
// and the raw key once they are no longer needed.
//
// Context: Any context.
//
// hmac_md5_init() - Initialize an HMAC-MD5 context for a new message
// @ctx: (output) the HMAC context to initialize
// @key: the prepared HMAC key
//
// If you don't need incremental computation, consider hmac_md5() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn hmac_md5_init(ctx: *mut hmac_md5_ctx, key: *const hmac_md5_key);
}
//
// hmac_md5_init_usingrawkey() - Initialize an HMAC-MD5 context for a new
// message, using a raw key
// @ctx: (output) the HMAC context to initialize
// @raw_key: the raw HMAC-MD5 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// If you don't need incremental computation, consider hmac_md5_usingrawkey()
// instead.
//
// Context: Any context.
//
// hmac_md5_update() - Update an HMAC-MD5 context with message data
// @ctx: the HMAC context to update; must have been initialized
// @data: the message data
// @data_len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// hmac_md5_final() - Finish computing an HMAC-MD5 value
// @ctx: the HMAC context to finalize; must have been initialized
// @out: (output) the resulting HMAC-MD5 value
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn hmac_md5_final(ctx: *mut hmac_md5_ctx, MD5_DIGEST_SIZE]: u8 out[at_least);
}
//
// hmac_md5() - Compute HMAC-MD5 in one shot, using a prepared key
// @key: the prepared HMAC key
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-MD5 value
//
// If you're using the key only once, consider using hmac_md5_usingrawkey().
//
// Context: Any context.
//
// hmac_md5_usingrawkey() - Compute HMAC-MD5 in one shot, using a raw key
// @raw_key: the raw HMAC-MD5 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-MD5 value
//
// If you're using the key multiple times, prefer to use hmac_md5_preparekey()
// followed by multiple calls to hmac_md5() instead.
//
// Context: Any context.
//
