//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf_crypto.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_crypto_type {
    pub algo): *const *const *const void (alloc_tfm)(char,
    pub tfm): *mut *mut void (free_tfm)(void,
    pub algo): *const *const int (has_algo)(char,
    pub keylen): *const *const *const *const int (setkey)(void tfm, u8 key, unsigned int,
    pub authsize): *mut *mut *mut int (setauthsize)(void tfm, unsigned int,
    pub iv): *const *const *const *const *const int (encrypt)(void tfm, u8 src, u8 dst, unsigned int len, u8,
    pub iv): *const *const *const *const *const int (decrypt)(void tfm, u8 src, u8 dst, unsigned int len, u8,
    pub tfm): *mut *mut unsigned int (ivsize)(void,
    pub tfm): *mut *mut unsigned int (statesize)(void,
    pub tfm): *mut *mut u32 (get_flags)(void,
    pub owner: *mut module,
    pub name: [c_char; 14],
}

extern "C" {
    pub fn bpf_crypto_register_type(type: *const bpf_crypto_type) -> c_int;
}
extern "C" {
    pub fn bpf_crypto_unregister_type(type: *const bpf_crypto_type) -> c_int;
}
