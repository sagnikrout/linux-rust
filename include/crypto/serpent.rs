//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/serpent.h
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
// Common values for serpent algorithms
//

pub const SERPENT_MIN_KEY_SIZE: c_int = 0;
pub const SERPENT_MAX_KEY_SIZE: c_int = 32;
pub const SERPENT_EXPKEY_WORDS: c_int = 132;
pub const SERPENT_BLOCK_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serpent_ctx {
    pub expkey: [u32; SERPENT_EXPKEY_WORDS],
}

extern "C" {
    pub fn serpent_setkey(tfm: *mut crypto_tfm, key: *const u8, keylen: c_uint) -> c_int;
}
extern "C" {
    pub fn __serpent_encrypt(ctx: *const c_void, dst: *mut u8, src: *const u8);
}
extern "C" {
    pub fn __serpent_decrypt(ctx: *const c_void, dst: *mut u8, src: *const u8);
}
