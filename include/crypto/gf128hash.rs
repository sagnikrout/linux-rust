//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/gf128hash.h
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
// GF(2^128) polynomial hashing: GHASH and POLYVAL
//
// Copyright 2025 Google LLC
//

pub const POLYVAL_BLOCK_SIZE: c_int = 16;
pub const POLYVAL_DIGEST_SIZE: c_int = 16;
//
// struct polyval_elem - An element of the POLYVAL finite field
// @bytes: View of the element as a byte array (unioned with @lo and @hi)
// @lo: The low 64 terms of the element's polynomial
// @hi: The high 64 terms of the element's polynomial
//
// This represents an element of the finite field GF(2^128), using the POLYVAL
// convention: little-endian byte order and natural bit order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct polyval_elem {
    pub bytes: [u8; POLYVAL_BLOCK_SIZE],
    pub lo: __le64,
    pub hi: __le64,
}

//
// struct ghash_key - Prepared key for GHASH
//
// Use ghash_preparekey() to initialize this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghash_key {

// @htable: GHASH key format used by the POWER8 assembly code
    pub htable: [u64; 4][2],
// @h_raw: The hash key H, in GHASH format
    pub h_raw: [u8; GHASH_BLOCK_SIZE],
// @h: The hash key H, in POLYVAL format
    pub h: polyval_elem,
}

//
// struct polyval_key - Prepared key for POLYVAL
//
// This may contain just the raw key H, or it may contain precomputed key
// powers, depending on the platform's POLYVAL implementation.  Use
// polyval_preparekey() to initialize this.
//
// By H^i we mean H^(i-1) * H * x^-128, with base case H^1 = H.  I.e. the
// exponentiation repeats the POLYVAL dot operation, with its "extra" x^-128.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct polyval_key {

// @h_powers: Powers of the hash key H^8 through H^1
    pub h_powers: [polyval_elem; 8],
// @h: The hash key H
    pub h: polyval_elem,

}

//
// struct ghash_ctx - Context for computing a GHASH value
// @key: Pointer to the prepared GHASH key.  The user of the API is
// responsible for ensuring that the key lives as long as the context.
// @acc: The accumulator.  It is stored in POLYVAL format rather than GHASH
// format, since most implementations want it in POLYVAL format.
// @partial: Number of data bytes processed so far modulo GHASH_BLOCK_SIZE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghash_ctx {
    pub key: *const ghash_key,
    pub acc: polyval_elem,
    pub partial: usize,
}

//
// struct polyval_ctx - Context for computing a POLYVAL value
// @key: Pointer to the prepared POLYVAL key.  The user of the API is
// responsible for ensuring that the key lives as long as the context.
// @acc: The accumulator
// @partial: Number of data bytes processed so far modulo POLYVAL_BLOCK_SIZE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct polyval_ctx {
    pub key: *const polyval_key,
    pub acc: polyval_elem,
    pub partial: usize,
}

//
// ghash_preparekey() - Prepare a GHASH key
// @key: (output) The key structure to initialize
// @raw_key: The raw hash key
//
// Initialize a GHASH key structure from a raw key.
//
// Context: Any context.
//
// polyval_preparekey() - Prepare a POLYVAL key
// @key: (output) The key structure to initialize
// @raw_key: The raw hash key
//
// Initialize a POLYVAL key structure from a raw key.  This may be a simple
// copy, or it may involve precomputing powers of the key, depending on the
// platform's POLYVAL implementation.
//
// Context: Any context.
//
// ghash_init() - Initialize a GHASH context for a new message
// @ctx: The context to initialize
// @key: The key to use.  Note that a pointer to the key is saved in the
// context, so the key must live at least as long as the context.
//
// ctx = (struct ghash_ctx){ .key = key };
//
// polyval_init() - Initialize a POLYVAL context for a new message
// @ctx: The context to initialize
// @key: The key to use.  Note that a pointer to the key is saved in the
// context, so the key must live at least as long as the context.
//
// ctx = (struct polyval_ctx){ .key = key };
//
// polyval_import_blkaligned() - Import a POLYVAL accumulator value
// @ctx: The context to initialize
// @key: The key to import.  Note that a pointer to the key is saved in the
// context, so the key must live at least as long as the context.
// @acc: The accumulator value to import.
//
// This imports an accumulator that was saved by polyval_export_blkaligned().
// The same key must be used.
//
// ctx = (struct polyval_ctx){ .key = key, .acc = *acc };
//
// polyval_export_blkaligned() - Export a POLYVAL accumulator value
// @ctx: The context to export the accumulator value from
// @acc: (output) The exported accumulator value
//
// This exports the accumulator from a POLYVAL context.  The number of data
// bytes processed so far must be a multiple of POLYVAL_BLOCK_SIZE.
//
// acc = ctx->acc;
//
// ghash_update() - Update a GHASH context with message data
// @ctx: The context to update; must have been initialized
// @data: The message data
// @len: The data length in bytes.  Doesn't need to be block-aligned.
//
// This can be called any number of times.
//
// Context: Any context.
//
extern "C" {
    pub fn ghash_update(ctx: *mut ghash_ctx, data: *const u8, len: usize);
}
//
// polyval_update() - Update a POLYVAL context with message data
// @ctx: The context to update; must have been initialized
// @data: The message data
// @len: The data length in bytes.  Doesn't need to be block-aligned.
//
// This can be called any number of times.
//
// Context: Any context.
//
extern "C" {
    pub fn polyval_update(ctx: *mut polyval_ctx, data: *const u8, len: usize);
}
//
// ghash_final() - Finish computing a GHASH value
// @ctx: The context to finalize
// @out: The output value
//
// If the total data length isn't a multiple of GHASH_BLOCK_SIZE, then the
// final block is automatically zero-padded.
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn ghash_final(ctx: *mut ghash_ctx, out[GHASH_BLOCK_SIZE]: u8);
}
//
// polyval_final() - Finish computing a POLYVAL value
// @ctx: The context to finalize
// @out: The output value
//
// If the total data length isn't a multiple of POLYVAL_BLOCK_SIZE, then the
// final block is automatically zero-padded.
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn polyval_final(ctx: *mut polyval_ctx, out[POLYVAL_BLOCK_SIZE]: u8);
}
//
// ghash() - Compute a GHASH value
// @key: The prepared key
// @data: The message data
// @len: The data length in bytes.  Doesn't need to be block-aligned.
// @out: The output value
//
// Context: Any context.
//
// polyval() - Compute a POLYVAL value
// @key: The prepared key
// @data: The message data
// @len: The data length in bytes.  Doesn't need to be block-aligned.
// @out: The output value
//
// Context: Any context.
//
