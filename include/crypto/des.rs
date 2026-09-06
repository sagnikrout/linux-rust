//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/des.h
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
// DES & Triple DES EDE Cipher Algorithms.
//

pub const DES_KEY_SIZE: c_int = 8;
pub const DES_EXPKEY_WORDS: c_int = 32;
pub const DES_BLOCK_SIZE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct des_ctx {
    pub expkey: [u32; DES_EXPKEY_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct des3_ede_ctx {
    pub expkey: [u32; DES3_EDE_EXPKEY_WORDS],
}

extern "C" {
    pub fn des_encrypt(ctx: *const des_ctx, dst: *mut u8, src: *const u8);
}
extern "C" {
    pub fn des_decrypt(ctx: *const des_ctx, dst: *mut u8, src: *const u8);
}
extern "C" {
    pub fn des3_ede_encrypt(dctx: *const des3_ede_ctx, dst: *mut u8, src: *const u8);
}
extern "C" {
    pub fn des3_ede_decrypt(dctx: *const des3_ede_ctx, dst: *mut u8, src: *const u8);
}
//
// des_expand_key - Expand a DES input key into a key schedule
// @ctx: the key schedule
// @key: buffer containing the input key
// @keylen: size of the buffer contents
//
// Returns: 0 on success, -EINVAL if the input key is rejected and -ENOKEY if
// the key is accepted but has been found to be weak.
//
extern "C" {
    pub fn des_expand_key(ctx: *mut des_ctx, key: *const u8, keylen: c_uint) -> c_int;
}
//
// des3_ede_expand_key - Expand a triple DES input key into a key schedule
// @ctx: the key schedule
// @key: buffer containing the input key
// @keylen: size of the buffer contents
//
// Returns: 0 on success, -EINVAL if the input key is rejected and -ENOKEY if
// the key is accepted but has been found to be weak. Note that weak keys will
// be rejected (and -EINVAL will be returned) when running in FIPS mode.
//
