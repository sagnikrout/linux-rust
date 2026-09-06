//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/qce/aead.h
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
// Copyright (c) 2021, Linaro Limited. All rights reserved.
//

pub const QCE_MAX_KEY_SIZE: c_int = 64;
pub const QCE_CCM4309_SALT_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_aead_ctx {
    pub enc_key: [u8; QCE_MAX_KEY_SIZE],
    pub auth_key: [u8; QCE_MAX_KEY_SIZE],
    pub ccm4309_salt: [u8; QCE_CCM4309_SALT_SIZE],
    pub enc_keylen: c_uint,
    pub auth_keylen: c_uint,
    pub authsize: c_uint,
    pub need_fallback: bool,
    pub fallback: *mut crypto_aead,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_aead_reqctx {
    pub flags: c_ulong,
    pub iv: *mut u8,
    pub ivsize: c_uint,
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub result_sg: scatterlist,
    pub adata_sg: scatterlist,
    pub dst_tbl: sg_table,
    pub src_tbl: sg_table,
    pub dst_sg: *mut scatterlist,
    pub src_sg: *mut scatterlist,
    pub cryptlen: c_uint,
    pub assoclen: c_uint,
    pub adata: *mut c_uchar,
    pub ccm_nonce: [u8; QCE_MAX_NONCE],
    pub ccmresult_buf: [u8; QCE_BAM_BURST_SIZE],
    pub ccm_rfc4309_iv: [u8; QCE_MAX_IV_SIZE],
    pub fallback_req: aead_request,
}

extern "C" {
    pub fn container_of(_arg: alg, qce_alg_template: struct, _arg: alg.aead) -> return;
}
