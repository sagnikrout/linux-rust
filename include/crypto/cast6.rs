//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/cast6.h
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

pub const CAST6_BLOCK_SIZE: c_int = 16;
pub const CAST6_MIN_KEY_SIZE: c_int = 16;
pub const CAST6_MAX_KEY_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cast6_ctx {
    pub Km: [u32; 12][4],
    pub Kr: [u8; 12][4],
}

extern "C" {
    pub fn __cast6_setkey(ctx: *mut cast6_ctx, key: *const u8, keylen: c_uint) -> c_int;
}
extern "C" {
    pub fn cast6_setkey(tfm: *mut crypto_tfm, key: *const u8, keylen: c_uint) -> c_int;
}
extern "C" {
    pub fn __cast6_encrypt(ctx: *const c_void, dst: *mut u8, src: *const u8);
}
extern "C" {
    pub fn __cast6_decrypt(ctx: *const c_void, dst: *mut u8, src: *const u8);
}
