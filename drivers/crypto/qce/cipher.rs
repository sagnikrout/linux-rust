//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/qce/cipher.h
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

pub const QCE_MAX_KEY_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_cipher_ctx {
    pub enc_key: [u8; QCE_MAX_KEY_SIZE],
    pub enc_keylen: c_uint,
    pub fallback: *mut crypto_skcipher,
}

//
// struct qce_cipher_reqctx - holds private cipher objects per request
// @flags: operation flags
// @iv: pointer to the IV
// @ivsize: IV size
// @src_nents: source entries
// @dst_nents: destination entries
// @result_sg: scatterlist used for result buffer
// @dst_tbl: destination sg table
// @dst_sg: destination sg pointer table beginning
// @src_tbl: source sg table
// @src_sg: source sg pointer table beginning;
// @cryptlen: crypto length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_cipher_reqctx {
    pub flags: c_ulong,
    pub iv: *mut u8,
    pub ivsize: c_uint,
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub result_sg: scatterlist,
    pub dst_tbl: sg_table,
    pub dst_sg: *mut scatterlist,
    pub src_sg: *mut scatterlist,
    pub cryptlen: c_uint,
    pub end: skcipher_request fallback_req; // keep at the,
}

extern "C" {
    pub fn container_of(_arg: alg, qce_alg_template: struct, _arg: alg.skcipher) -> return;
}
