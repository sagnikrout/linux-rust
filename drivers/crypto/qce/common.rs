//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/qce/common.h
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
//
// Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
//

// xts du size
pub const QCE_SECTOR_SIZE: c_int = 512;
// key size in bytes
pub const QCE_SHA_HMAC_KEY_SIZE: c_int = 64;

// IV length in bytes

// max of AES_BLOCK_SIZE

// maximum nonce bytes
pub const QCE_MAX_NONCE: c_int = 16;

// burst size alignment requirement
pub const QCE_MAX_ALIGN_SIZE: c_int = 64;
// cipher algorithms

// hash and hmac algorithms

// cipher modes

// cipher encryption/decryption operations

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_alg_template {
    pub entry: list_head,
    pub crypto_alg_type: u32,
    pub alg_flags: c_ulong,
    pub std_iv: *const u32,
    pub skcipher: skcipher_alg,
    pub ahash: ahash_alg,
    pub aead: aead_alg,
    pub alg: },
    pub qce: *mut qce_device,
    pub hash_zero: *const u8,
    pub digest_size: u32,
}

extern "C" {
    pub fn qce_cpu_to_be32p_array(dst: *mut __be32, src: *const u8, len: c_uint);
}
extern "C" {
    pub fn qce_check_status(qce: *mut qce_device, status: *mut u32) -> c_int;
}
extern "C" {
    pub fn qce_get_version(qce: *mut qce_device, major: *mut u32, minor: *mut u32, step: *mut u32);
}
extern "C" {
    pub fn qce_start(async_req: *mut crypto_async_request, type: u32) -> c_int;
}
