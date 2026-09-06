//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/rng.h
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
// RNG: Random Number Generator  algorithms under the crypto API
//
// Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
// Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
//

extern "C" {
    pub fn crypto_register_rng(alg: *mut rng_alg) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_rng(alg: *mut rng_alg);
}
extern "C" {
    pub fn crypto_register_rngs(algs: *mut rng_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_rngs(algs: *mut rng_alg, count: c_int);
}

extern "C" {
    pub fn crypto_del_default_rng() -> c_int;
}

extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
