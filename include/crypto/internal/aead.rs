//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/aead.h
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
// AEAD: Authenticated Encryption with Associated Data
//
// Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_instance {
    pub inst): *mut *mut void (free)(struct aead_instance,
    pub base)]: char head[offsetof(struct aead_alg,,
    pub base: crypto_instance,
    pub s: },
    pub alg: aead_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aead_spawn {
    pub base: crypto_spawn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_queue {
    pub base: crypto_queue,
}

extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx_dma(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn container_of(_arg: &inst->alg.base, crypto_instance: struct, _arg: alg) -> return;
}
extern "C" {
    pub fn container_of(_arg: &inst->alg, aead_instance: struct, _arg: alg.base) -> return;
}
extern "C" {
    pub fn aead_instance(_arg: crypto_tfm_alg_instance(&aead->base)) -> return;
}
extern "C" {
    pub fn crypto_instance_ctx(_arg: aead_crypto_instance(inst)) -> return;
}
extern "C" {
    pub fn PTR_ALIGN(_arg: aead_request_ctx(req), _arg: align) -> return;
}
extern "C" {
    pub fn container_of(_arg: req, aead_request: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: spawn->base.alg, aead_alg: struct, _arg: base) -> return;
}
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
//
// crypto_aead_chunksize() - obtain chunk size
// @tfm: cipher handle
//
// The block size is set to one for ciphers such as CCM.  However,
// you still need to provide incremental updates in multiples of
// the underlying block size as the IV does not have sub-block
// granularity.  This is known in this API as the chunk size.
//
// Return: chunk size in bytes
//
extern "C" {
    pub fn crypto_aead_alg_chunksize(_arg: crypto_aead_alg(tfm)) -> return;
}
extern "C" {
    pub fn crypto_register_aead(alg: *mut aead_alg) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_aead(alg: *mut aead_alg);
}
extern "C" {
    pub fn crypto_register_aeads(algs: *mut aead_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_aeads(algs: *mut aead_alg, count: c_int);
}
