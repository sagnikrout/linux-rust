//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/arm/chacha.h
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
// ChaCha and HChaCha functions (ARM optimized)
//
// Copyright (C) 2016-2019 Linaro, Ltd. <ard.biesheuvel@linaro.org>
// Copyright (C) 2015 Martin Willi
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: use_neon) -> static __ro_after_init;
}
extern "C" {
    pub fn static_branch_likely(crypto_simd_usable(: &use_neon) &&) -> return;
}

//
// The Cortex-A7 and Cortex-A5 do not perform well with
// the NEON implementation but do incredibly with the
// scalar one and use less power.
//
