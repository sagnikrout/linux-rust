//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/hash.h
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
// Hash algorithms.
//
// Copyright (c) 2008 Herbert Xu <herbert@gondor.apana.org.au>
//

// Set this bit to handle partial blocks in the API.
pub const CRYPTO_AHASH_ALG_BLOCK_ONLY: c_uint = 0x01000000;
// Set this bit if final requires at least one byte.
pub const CRYPTO_AHASH_ALG_FINAL_NONZERO: c_uint = 0x02000000;
// Set this bit if finup can deal with multiple blocks.
pub const CRYPTO_AHASH_ALG_FINUP_MAX: c_uint = 0x04000000;
// This bit is set by the Crypto API if export_core is not supported.
pub const CRYPTO_AHASH_ALG_NO_EXPORT_CORE: c_uint = 0x08000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_hash_walk {
    pub data: *const c_char,
    pub offset: c_uint,
    pub flags: c_uint,
    pub pg: *mut page,
    pub entrylen: c_uint,
    pub total: c_uint,
    pub sg: *mut scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahash_instance {
    pub inst): *mut *mut void (free)(struct ahash_instance,
    pub halg.base)]: char head[offsetof(struct ahash_alg,,
    pub base: crypto_instance,
    pub s: },
    pub alg: ahash_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shash_instance {
    pub inst): *mut *mut void (free)(struct shash_instance,
    pub base)]: char head[offsetof(struct shash_alg,,
    pub base: crypto_instance,
    pub s: },
    pub alg: shash_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_ahash_spawn {
    pub base: crypto_spawn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_shash_spawn {
    pub base: crypto_spawn,
}

extern "C" {
    pub fn crypto_hash_walk_done(walk: *mut crypto_hash_walk, err: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_register_ahash(alg: *mut ahash_alg) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_ahash(alg: *mut ahash_alg);
}
extern "C" {
    pub fn crypto_register_ahashes(algs: *mut ahash_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_ahashes(algs: *mut ahash_alg, count: c_int);
}
extern "C" {
    pub fn ahash_free_singlespawn_instance(inst: *mut ahash_instance);
}
extern "C" {
    pub fn crypto_hash_alg_has_setkey(halg: *mut hash_alg_common) -> bool;
}
extern "C" {
    pub fn __crypto_hash_alg_common(_arg: spawn->base.alg) -> return;
}
extern "C" {
    pub fn crypto_register_shash(alg: *mut shash_alg) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_shash(alg: *mut shash_alg);
}
extern "C" {
    pub fn crypto_register_shashes(algs: *mut shash_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_shashes(algs: *mut shash_alg, count: c_int);
}
extern "C" {
    pub fn shash_free_singlespawn_instance(inst: *mut shash_instance);
}
extern "C" {
    pub fn __crypto_shash_alg(_arg: spawn->base.alg) -> return;
}
extern "C" {
    pub fn shash_ahash_update(req: *mut ahash_request, desc: *mut shash_desc) -> c_int;
}
extern "C" {
    pub fn shash_ahash_finup(req: *mut ahash_request, desc: *mut shash_desc) -> c_int;
}
extern "C" {
    pub fn shash_ahash_digest(req: *mut ahash_request, desc: *mut shash_desc) -> c_int;
}
extern "C" {
    pub fn crypto_tfm_ctx(_arg: crypto_ahash_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx_dma(_arg: crypto_ahash_tfm(tfm)) -> return;
}
extern "C" {
    pub fn container_of(_arg: inst, ahash_instance: struct, _arg: s.base) -> return;
}
extern "C" {
    pub fn ahash_instance(_arg: crypto_tfm_alg_instance(&ahash->base)) -> return;
}
extern "C" {
    pub fn crypto_instance_ctx(_arg: ahash_crypto_instance(inst)) -> return;
}
extern "C" {
    pub fn PTR_ALIGN(_arg: ahash_request_ctx(req), _arg: align) -> return;
}
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
extern "C" {
    pub fn crypto_enqueue_request(_arg: queue, _arg: &request->base) -> return;
}
extern "C" {
    pub fn ahash_request_cast(_arg: crypto_dequeue_request(queue)) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn container_of(_arg: inst, shash_instance: struct, _arg: s.base) -> return;
}
extern "C" {
    pub fn shash_instance(_arg: crypto_tfm_alg_instance(&shash->base)) -> return;
}
extern "C" {
    pub fn crypto_instance_ctx(_arg: shash_crypto_instance(inst)) -> return;
}
extern "C" {
    pub fn crypto_spawn_tfm2(_arg: &spawn->base) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm, crypto_shash: struct, _arg: base) -> return;
}
extern "C" {
    pub fn crypto_tfm_req_virt(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn __crypto_ahash_cast(_arg: crypto_ahash_tfm(tfm)->fb) -> return;
}
// Return the state size without partial block for block-only algorithms.
// This can only be used if the request was never cloned.

//
// crypto_ahash_export_core() - extract core state for message digest
// @req: reference to the ahash_request handle whose state is exported
// @out: output buffer of sufficient size that can hold the hash state
//
// Export the hash state without the partial block buffer.
//
// Context: Softirq or process context.
// Return: 0 if the export creation was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_ahash_export_core(req: *mut ahash_request, out: *mut c_void) -> c_int;
}
//
// crypto_ahash_import_core() - import core state
// @req: reference to ahash_request handle the state is imported into
// @in: buffer holding the state
//
// Import the hash state without the partial block buffer.
//
// Context: Softirq or process context.
// Return: 0 if the import was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_ahash_import_core(req: *mut ahash_request, in: *const c_void) -> c_int;
}
//
// crypto_shash_export_core() - extract core state for message digest
// @desc: reference to the operational state handle whose state is exported
// @out: output buffer of sufficient size that can hold the hash state
//
// Export the hash state without the partial block buffer.
//
// Context: Softirq or process context.
// Return: 0 if the export creation was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_shash_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int;
}
//
// crypto_shash_import_core() - import core state
// @desc: reference to the operational state handle the state imported into
// @in: buffer holding the state
//
// Import the hash state without the partial block buffer.
//
// Context: Softirq or process context.
// Return: 0 if the import was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_shash_import_core(desc: *mut shash_desc, in: *const c_void) -> c_int;
}
