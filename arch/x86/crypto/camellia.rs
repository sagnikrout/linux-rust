//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/crypto/camellia.h
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

pub const CAMELLIA_MIN_KEY_SIZE: c_int = 16;
pub const CAMELLIA_MAX_KEY_SIZE: c_int = 32;
pub const CAMELLIA_BLOCK_SIZE: c_int = 16;
pub const CAMELLIA_TABLE_BYTE_LEN: c_int = 272;
pub const CAMELLIA_PARALLEL_BLOCKS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camellia_ctx {
    pub sizeof(u64)]: u64 key_table[CAMELLIA_TABLE_BYTE_LEN /,
    pub key_length: u32,
}

// regular block cipher functions
extern "C" {
    pub fn camellia_dec_blk(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
// 2-way parallel cipher functions
extern "C" {
    pub fn camellia_dec_blk_2way(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
// 16-way parallel cipher functions (avx/aes-ni)
extern "C" {
    pub fn camellia_ecb_enc_16way(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
extern "C" {
    pub fn camellia_ecb_dec_16way(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
extern "C" {
    pub fn camellia_cbc_dec_16way(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
// glue helpers
extern "C" {
    pub fn camellia_decrypt_cbc_2way(ctx: *const c_void, dst: *mut u8, src: *const u8);
}
