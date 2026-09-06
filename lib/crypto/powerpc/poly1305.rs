//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/powerpc/poly1305.h
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
// Poly1305 authenticator algorithm, RFC7539.
//
// Copyright 2023- IBM Corp. All rights reserved.
//

extern "C" {
    pub fn poly1305_p10le_4blocks(state: *mut poly1305_block_state, m: *const u8, mlen: u32) -> asmlinkage void;
}
extern "C" {
    pub fn poly1305_64s(state: *mut poly1305_block_state, m: *const u8, mlen: u32, highbit: c_int) -> asmlinkage void;
}
extern "C" {
    pub fn poly1305_emit_64(state: *const poly1305_state, nonce[4]: u32, digest[POLY1305_DIGEST_SIZE]: u8) -> asmlinkage void;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_p10) -> static __ro_after_init;
}
extern "C" {
    pub fn poly1305_block_init_generic(_arg: dctx, _arg: raw_key) -> return;
}
extern "C" {
    pub fn poly1305_blocks_generic(_arg: state, _arg: src, _arg: len, _arg: padbit) -> return;
}
extern "C" {
    pub fn poly1305_emit_generic(_arg: state, _arg: digest, _arg: nonce) -> return;
}
