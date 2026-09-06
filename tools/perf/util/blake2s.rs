//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/blake2s.h
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
//
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

pub const BLAKE2S_BLOCK_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blake2s_ctx {
    pub h: [u32; 8],
    pub t: [u32; 2],
    pub f: [u32; 2],
    pub buf: [u8; BLAKE2S_BLOCK_SIZE],
    pub buflen: c_uint,
    pub outlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blake2s_iv {
    BLAKE2S_IV0 = 0x6A09E667UL,
    BLAKE2S_IV1 = 0xBB67AE85UL,
    BLAKE2S_IV2 = 0x3C6EF372UL,
    BLAKE2S_IV3 = 0xA54FF53AUL,
    BLAKE2S_IV4 = 0x510E527FUL,
    BLAKE2S_IV5 = 0x9B05688CUL,
    BLAKE2S_IV6 = 0x1F83D9ABUL,
    BLAKE2S_IV7 = 0x5BE0CD19UL,
}

extern "C" {
    pub fn blake2s_update(ctx: *mut blake2s_ctx, in: *const u8, inlen: usize);
}
extern "C" {
    pub fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut u8);
}
