//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/arc4.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Common values for ARC4 Cipher Algorithm
//

pub const ARC4_MIN_KEY_SIZE: c_int = 1;
pub const ARC4_MAX_KEY_SIZE: c_int = 256;
pub const ARC4_BLOCK_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc4_ctx {
    pub S: [u32; 256],
    pub y: u32 x,,
}

extern "C" {
    pub fn arc4_setkey(ctx: *mut arc4_ctx, in_key: *const u8, key_len: c_uint) -> c_int;
}
extern "C" {
    pub fn arc4_crypt(ctx: *mut arc4_ctx, out: *mut u8, in: *const u8, len: c_uint);
}
