//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/powerpc/gf128hash.h
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
// GHASH routines supporting VMX instructions on the Power 8
//
// Copyright (C) 2015, 2019 International Business Machines Inc.
// Copyright (C) 2014 - 2018 Linaro Ltd.
// Copyright 2026 Google LLC
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_vec_crypto) -> static __ro_after_init;
}
extern "C" {
    pub fn gcm_init_p8(htable[4][2]: u64, h[16]: u8);
}
extern "C" {
    pub fn gcm_gmult_p8(Xi[16]: u8, htable[4][2]: u64);
}
extern "C" {
    pub fn gcm_ghash_p8(Xi[16]: u8, htable[4][2]: u64, in: *const u8, len: usize);
}

// This reproduces gcm_init_p8() on both LE and BE systems.

