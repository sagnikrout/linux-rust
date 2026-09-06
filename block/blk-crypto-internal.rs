//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-crypto-internal.h
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
// Copyright 2019 Google LLC
//

// Represents a crypto mode supported by blk-crypto
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_mode {
    pub /: *const *const *const char name; / name of this mode, shown in sysfs,
    pub /: *const *const *const char cipher_str; / crypto API name (for fallback case),
    pub /: *mut *mut unsigned int keysize; / key size in bytes,
    pub /: *mut *mut unsigned int security_strength; / security strength in bytes,
    pub /: *mut *mut unsigned int ivsize; / iv size in bytes,
}

extern "C" {
    pub fn blk_crypto_sysfs_register(disk: *mut gendisk) -> c_int;
}
extern "C" {
    pub fn blk_crypto_sysfs_unregister(disk: *mut gendisk);
}
extern "C" {
    pub fn bio_crypt_rq_ctx_compatible(rq: *mut request, bio: *mut bio) -> bool;
}
extern "C" {
    pub fn blk_crypto_put_keyslot(slot: *mut blk_crypto_keyslot);
}

extern "C" {
    pub fn __bio_crypt_advance(bio: *mut bio, bytes: c_uint);
}
extern "C" {
    pub fn __bio_crypt_free_ctx(bio: *mut bio);
}

extern "C" {
    pub fn __blk_crypto_rq_get_keyslot(rq: *mut request) -> blk_status_t;
}
extern "C" {
    pub fn __blk_crypto_rq_get_keyslot(_arg: rq) -> return;
}
extern "C" {
    pub fn __blk_crypto_rq_put_keyslot(rq: *mut request);
}
extern "C" {
    pub fn __blk_crypto_free_request(rq: *mut request);
}
//
// blk_crypto_rq_bio_prep - Prepare a request's crypt_ctx when its first bio
// is inserted
// @rq: The request to prepare
// @bio: The first bio being inserted into the request
// @gfp_mask: Memory allocation flags
//
// Return: 0 on success, -ENOMEM if out of memory.  -ENOMEM is only possible if
// @gfp_mask doesn't include %__GFP_DIRECT_RECLAIM.
//
extern "C" {
    pub fn __blk_crypto_rq_bio_prep(_arg: rq, _arg: bio, _arg: gfp_mask) -> return;
}
extern "C" {
    pub fn blk_crypto_fallback_bio_prep(bio: *mut bio) -> bool;
}

extern "C" {
    pub fn blk_crypto_fallback_start_using_mode(mode_num: blk_crypto_mode_num) -> c_int;
}
extern "C" {
    pub fn blk_crypto_fallback_evict_key(key: *const blk_crypto_key) -> c_int;
}

