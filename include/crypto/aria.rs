//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/aria.h
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
// Cryptographic API.
//
// ARIA Cipher Algorithm.
//
// Documentation of ARIA can be found in RFC 5794.
// Copyright (c) 2022 Taehee Yoo <ap420073@gmail.com>
//
// Information for ARIA
// http://210.104.33.10/ARIA/index-e.html (English)
// http://seed.kisa.or.kr/ (Korean)
//
// Public domain version is distributed above.
//

pub const ARIA_MIN_KEY_SIZE: c_int = 16;
pub const ARIA_MAX_KEY_SIZE: c_int = 32;
pub const ARIA_BLOCK_SIZE: c_int = 16;
pub const ARIA_MAX_RD_KEYS: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aria_ctx {
    pub enc_key: [u32; ARIA_MAX_RD_KEYS][ARIA_RD_KEY_WORDS],
    pub dec_key: [u32; ARIA_MAX_RD_KEYS][ARIA_RD_KEY_WORDS],
    pub rounds: c_int,
    pub key_length: c_int,
}

extern "C" {
    pub fn rotr32(_arg: t0, rotr32(t0: 8) ^ rotr32(t0 ^, _arg: 8), _arg: 16) -> return;
}
// S-Box Layer 1 + M
// t0 = s1[get_u8(*t0, 0)] ^
// t1 = s1[get_u8(*t1, 0)] ^
// t2 = s1[get_u8(*t2, 0)] ^
// t3 = s1[get_u8(*t3, 0)] ^
// S-Box Layer 2 + M
// t0 = x1[get_u8(*t0, 0)] ^
// t1 = x1[get_u8(*t1, 0)] ^
// t2 = x1[get_u8(*t2, 0)] ^
// t3 = x1[get_u8(*t3, 0)] ^
// Word-level diffusion
// t1 ^= *t2;
// t2 ^= *t3;
// t0 ^= *t1;
// t3 ^= *t1;
// t2 ^= *t0;
// t1 ^= *t2;
// Byte-level diffusion
// t1 = ((*t1 << 8) & 0xff00ff00) ^ ((*t1 >> 8) & 0x00ff00ff);
// t2 = rotr32(*t2, 16);
// t3 = bswap32(*t3);
// Key XOR Layer
// t0 ^= rk[0];
// t1 ^= rk[1];
// t2 ^= rk[2];
// t3 ^= rk[3];
// Odd round Substitution & Diffusion
// Even round Substitution & Diffusion
// Q, R Macro expanded ARIA GSRK
extern "C" {
    pub fn aria_encrypt(ctx: *mut c_void, out: *mut u8, in: *const u8);
}
extern "C" {
    pub fn aria_decrypt(ctx: *mut c_void, out: *mut u8, in: *const u8);
}
