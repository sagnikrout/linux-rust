//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/crypto/aria-avx.h
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

pub const ARIA_AESNI_PARALLEL_BLOCKS: c_int = 16;

pub const ARIA_AESNI_AVX2_PARALLEL_BLOCKS: c_int = 32;

pub const ARIA_GFNI_AVX512_PARALLEL_BLOCKS: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aria_avx_ops {
    pub src): *const *const *const *const void (aria_encrypt_16way)(void ctx, u8 dst, u8,
    pub src): *const *const *const *const void (aria_decrypt_16way)(void ctx, u8 dst, u8,
    pub iv): *mut *mut u8 keystream, u8,
    pub src): *const *const *const *const void (aria_encrypt_32way)(void ctx, u8 dst, u8,
    pub src): *const *const *const *const void (aria_decrypt_32way)(void ctx, u8 dst, u8,
    pub iv): *mut *mut u8 keystream, u8,
    pub src): *const *const *const *const void (aria_encrypt_64way)(void ctx, u8 dst, u8,
    pub src): *const *const *const *const void (aria_decrypt_64way)(void ctx, u8 dst, u8,
    pub iv): *mut *mut u8 keystream, u8,
}
