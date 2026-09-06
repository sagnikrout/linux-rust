//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/sm3.h
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
// SM3 hash algorithm
//
// Copyright (C) 2017 ARM Limited or its affiliates.
// Copyright (C) 2017 Gilad Ben-Yossef <gilad@benyossef.com>
// Copyright (C) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
//

pub const SM3_DIGEST_SIZE: c_int = 32;
pub const SM3_BLOCK_SIZE: c_int = 64;
pub const SM3_IVA: c_uint = 0x7380166f;
pub const SM3_IVB: c_uint = 0x4914b2b9;
pub const SM3_IVC: c_uint = 0x172442d7;
pub const SM3_IVD: c_uint = 0xda8a0600;
pub const SM3_IVE: c_uint = 0xa96f30bc;
pub const SM3_IVF: c_uint = 0x163138aa;
pub const SM3_IVG: c_uint = 0xe38dee4d;
pub const SM3_IVH: c_uint = 0xb0fb0e4e;
// State for the SM3 compression function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm3_block_state {
    pub 4]: u32 h[SM3_DIGEST_SIZE /,
}

//
// struct sm3_ctx - Context for hashing a message with SM3
// @state: the compression function state
// @bytecount: number of bytes processed so far
// @buf: partial block buffer; bytecount % SM3_BLOCK_SIZE bytes are valid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm3_ctx {
    pub state: sm3_block_state,
    pub bytecount: u64,
    pub __aligned(__alignof__(__be64)): u8 buf[SM3_BLOCK_SIZE],
}

//
// sm3_init() - Initialize an SM3 context for a new message
// @ctx: the context to initialize
//
// If you don't need incremental computation, consider sm3() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn sm3_init(ctx: *mut sm3_ctx);
}
//
// sm3_update() - Update an SM3 context with message data
// @ctx: the context to update; must have been initialized
// @data: the message data
// @len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
extern "C" {
    pub fn sm3_update(ctx: *mut sm3_ctx, data: *const u8, len: usize);
}
//
// sm3_final() - Finish computing an SM3 message digest
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting SM3 message digest
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn sm3_final(ctx: *mut sm3_ctx, SM3_DIGEST_SIZE]: u8 out[at_least);
}
//
// sm3() - Compute SM3 message digest in one shot
// @data: the message data
// @len: the data length in bytes
// @out: (output) the resulting SM3 message digest
//
// Context: Any context.
//
extern "C" {
    pub fn sm3(data: *const u8, len: usize, SM3_DIGEST_SIZE]: u8 out[at_least);
}
