//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/engine.h
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
// Crypto engine API
//
// Copyright (c) 2016 Baolin Wang <baolin.wang@linaro.org>
//

//
// struct crypto_engine_op - crypto hardware engine operations
// @do_one_request: do encryption for current request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_engine_op {
    pub areq): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_engine_alg {
    pub base: aead_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahash_engine_alg {
    pub base: ahash_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct akcipher_engine_alg {
    pub base: akcipher_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kpp_engine_alg {
    pub base: kpp_alg,
    pub op: crypto_engine_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_engine_alg {
    pub base: skcipher_alg,
    pub op: crypto_engine_op,
}

extern "C" {
    pub fn crypto_engine_start(engine: *mut crypto_engine) -> c_int;
}
extern "C" {
    pub fn crypto_engine_stop(engine: *mut crypto_engine) -> c_int;
}
extern "C" {
    pub fn crypto_engine_exit(engine: *mut crypto_engine);
}
extern "C" {
    pub fn crypto_engine_register_aead(alg: *mut aead_engine_alg) -> c_int;
}
extern "C" {
    pub fn crypto_engine_unregister_aead(alg: *mut aead_engine_alg);
}
extern "C" {
    pub fn crypto_engine_register_aeads(algs: *mut aead_engine_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_engine_unregister_aeads(algs: *mut aead_engine_alg, count: c_int);
}
extern "C" {
    pub fn crypto_engine_register_ahash(alg: *mut ahash_engine_alg) -> c_int;
}
extern "C" {
    pub fn crypto_engine_unregister_ahash(alg: *mut ahash_engine_alg);
}
extern "C" {
    pub fn crypto_engine_register_ahashes(algs: *mut ahash_engine_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_engine_register_akcipher(alg: *mut akcipher_engine_alg) -> c_int;
}
extern "C" {
    pub fn crypto_engine_unregister_akcipher(alg: *mut akcipher_engine_alg);
}
extern "C" {
    pub fn crypto_engine_register_kpp(alg: *mut kpp_engine_alg) -> c_int;
}
extern "C" {
    pub fn crypto_engine_unregister_kpp(alg: *mut kpp_engine_alg);
}
extern "C" {
    pub fn crypto_engine_register_skcipher(alg: *mut skcipher_engine_alg) -> c_int;
}
extern "C" {
    pub fn crypto_engine_unregister_skcipher(alg: *mut skcipher_engine_alg);
}
