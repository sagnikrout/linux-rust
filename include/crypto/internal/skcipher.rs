//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/skcipher.h
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
// Symmetric key ciphers.
//
// Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
//

//
// Set this if your algorithm is sync but needs a reqsize larger
// than MAX_SYNC_SKCIPHER_REQSIZE.
//
// Reuse bit that is specific to hash algorithms.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_instance {
    pub inst): *mut *mut void (free)(struct skcipher_instance,
    pub base)]: char head[offsetof(struct skcipher_alg,,
    pub base: crypto_instance,
    pub s: },
    pub alg: skcipher_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lskcipher_instance {
    pub inst): *mut *mut void (free)(struct lskcipher_instance,
    pub co.base)]: char head[offsetof(struct lskcipher_alg,,
    pub base: crypto_instance,
    pub s: },
    pub alg: lskcipher_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_skcipher_spawn {
    pub base: crypto_spawn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_lskcipher_spawn {
    pub base: crypto_spawn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_walk {
// Virtual address of the source.
    pub addr: *const *const c_void,
    pub virt: },
    pub src: },
// Private field for the API, do not use.
    pub in: scatter_walk,
}

// Virtual address of the destination.
// Private field for the API, do not use.
extern "C" {
    pub fn crypto_instance_ctx(_arg: skcipher_crypto_instance(inst)) -> return;
}
extern "C" {
    pub fn crypto_instance_ctx(_arg: lskcipher_crypto_instance(inst)) -> return;
}
extern "C" {
    pub fn container_of(_arg: spawn->base.alg, lskcipher_alg: struct, _arg: co.base) -> return;
}
extern "C" {
    pub fn container_of(_arg: spawn->base.alg, skcipher_alg_common: struct, _arg: base) -> return;
}
extern "C" {
    pub fn crypto_lskcipher_spawn_alg(_arg: spawn) -> return;
}
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
extern "C" {
    pub fn crypto_register_skcipher(alg: *mut skcipher_alg) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_skcipher(alg: *mut skcipher_alg);
}
extern "C" {
    pub fn crypto_register_skciphers(algs: *mut skcipher_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: c_int);
}
extern "C" {
    pub fn crypto_register_lskcipher(alg: *mut lskcipher_alg) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_lskcipher(alg: *mut lskcipher_alg);
}
extern "C" {
    pub fn crypto_register_lskciphers(algs: *mut lskcipher_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_lskciphers(algs: *mut lskcipher_alg, count: c_int);
}
extern "C" {
    pub fn skcipher_walk_done(walk: *mut skcipher_walk, res: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx_dma(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn PTR_ALIGN(_arg: skcipher_request_ctx(req), _arg: align) -> return;
}
// Helpers for simple block cipher modes of operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_ctx_simple {
    pub /: *mut *mut *mut crypto_cipher cipher; / underlying block cipher,
}

extern "C" {
    pub fn crypto_spawn_cipher_alg(_arg: spawn) -> return;
}
extern "C" {
    pub fn crypto_lskcipher_spawn_alg(_arg: spawn) -> return;
}
