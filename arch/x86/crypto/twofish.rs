//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/crypto/twofish.h
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

// regular block cipher functions from twofish_x86_64 module
extern "C" {
    pub fn twofish_enc_blk(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
extern "C" {
    pub fn twofish_dec_blk(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
// 3-way parallel cipher functions
extern "C" {
    pub fn twofish_dec_blk_3way(ctx: *const c_void, dst: *mut u8, src: *const u8) -> asmlinkage void;
}
// helpers from twofish_x86_64-3way module
extern "C" {
    pub fn twofish_dec_blk_cbc_3way(ctx: *const c_void, dst: *mut u8, src: *const u8);
}
