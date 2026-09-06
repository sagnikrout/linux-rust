//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/ccp-crypto.h
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
// AMD Cryptographic Coprocessor (CCP) crypto API support
//
// Copyright (C) 2013,2017 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
//

// We want the module name in front of our messages

pub const CCP_CRA_PRIORITY: c_int = 300;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_crypto_skcipher_alg {
    pub entry: list_head,
    pub mode: u32,
    pub alg: skcipher_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_crypto_aead {
    pub entry: list_head,
    pub mode: u32,
    pub alg: aead_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_crypto_ahash_alg {
    pub entry: list_head,
    pub init: *const __be32,
    pub type: u32,
    pub mode: u32,
// Child algorithm used for HMAC, CMAC, etc
    pub child_alg: [c_char; CRYPTO_MAX_ALG_NAME],
    pub alg: ahash_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_crypto_akcipher_alg {
    pub entry: list_head,
    pub alg: akcipher_alg,
}

extern "C" {
    pub fn container_of(_arg: alg, ccp_crypto_skcipher_alg: struct, _arg: alg) -> return;
}
extern "C" {
    pub fn container_of(_arg: ahash_alg, ccp_crypto_ahash_alg: struct, _arg: alg) -> return;
}
// AES related defines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_aes_ctx {
// Fallback cipher for XTS with unsupported unit sizes
    pub tfm_skcipher: *mut crypto_skcipher,
    pub engine: ccp_engine,
    pub type: ccp_aes_type,
    pub mode: ccp_aes_mode,
    pub key_sg: scatterlist,
    pub key_len: c_uint,
    pub 2]: *mut *mut u8 key[AES_MAX_KEY_SIZE,
    pub nonce: [u8; CTR_RFC3686_NONCE_SIZE],
// CMAC key structures
    pub k1_sg: scatterlist,
    pub k2_sg: scatterlist,
    pub kn_len: c_uint,
    pub k1: [u8; AES_BLOCK_SIZE],
    pub k2: [u8; AES_BLOCK_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_aes_req_ctx {
    pub iv_sg: scatterlist,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub tag_sg: scatterlist,
    pub tag: [u8; AES_BLOCK_SIZE],
// Fields used for RFC3686 requests
    pub rfc3686_info: *mut u8,
    pub rfc3686_iv: [u8; AES_BLOCK_SIZE],
    pub cmd: ccp_cmd,
    pub end: skcipher_request fallback_req; // keep at the,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_aes_cmac_req_ctx {
    pub null_msg: c_uint,
    pub final: c_uint,
    pub src: *mut scatterlist,
    pub nbytes: c_uint,
    pub hash_cnt: u64,
    pub hash_rem: c_uint,
    pub data_sg: sg_table,
    pub iv_sg: scatterlist,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub buf_sg: scatterlist,
    pub buf_count: c_uint,
    pub buf: [u8; AES_BLOCK_SIZE],
    pub pad_sg: scatterlist,
    pub pad_count: c_uint,
    pub pad: [u8; AES_BLOCK_SIZE],
    pub cmd: ccp_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_aes_cmac_exp_ctx {
    pub null_msg: c_uint,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub buf_count: c_uint,
    pub buf: [u8; AES_BLOCK_SIZE],
}

// 3DES related defines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_des3_ctx {
    pub engine: ccp_engine,
    pub type: ccp_des3_type,
    pub mode: ccp_des3_mode,
    pub key_sg: scatterlist,
    pub key_len: c_uint,
    pub key: [u8; AES_MAX_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_des3_req_ctx {
    pub iv_sg: scatterlist,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub cmd: ccp_cmd,
}

// SHA-related defines
// These values must be large enough to accommodate any variant
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_sha_ctx {
    pub opad_sg: scatterlist,
    pub opad_count: c_uint,
    pub key_len: c_uint,
    pub key: [u8; MAX_SHA_BLOCK_SIZE],
    pub ipad: [u8; MAX_SHA_BLOCK_SIZE],
    pub opad: [u8; MAX_SHA_BLOCK_SIZE],
    pub hmac_tfm: *mut crypto_shash,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_sha_req_ctx {
    pub type: ccp_sha_type,
    pub msg_bits: u64,
    pub first: c_uint,
    pub final: c_uint,
    pub src: *mut scatterlist,
    pub nbytes: c_uint,
    pub hash_cnt: u64,
    pub hash_rem: c_uint,
    pub data_sg: sg_table,
    pub ctx_sg: scatterlist,
    pub ctx: [u8; MAX_SHA_CONTEXT_SIZE],
    pub buf_sg: scatterlist,
    pub buf_count: c_uint,
    pub buf: [u8; MAX_SHA_BLOCK_SIZE],
// CCP driver command
    pub cmd: ccp_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_sha_exp_ctx {
    pub type: ccp_sha_type,
    pub msg_bits: u64,
    pub first: c_uint,
    pub ctx: [u8; MAX_SHA_CONTEXT_SIZE],
    pub buf_count: c_uint,
    pub buf: [u8; MAX_SHA_BLOCK_SIZE],
}

// RSA related defines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_rsa_ctx {
    pub /: *mut *mut unsigned int key_len; / in bits,
    pub e_sg: scatterlist,
    pub e_buf: *mut u8,
    pub e_len: c_uint,
    pub n_sg: scatterlist,
    pub n_buf: *mut u8,
    pub n_len: c_uint,
    pub d_sg: scatterlist,
    pub d_buf: *mut u8,
    pub d_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_rsa_req_ctx {
    pub cmd: ccp_cmd,
}

// Common Context Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_ctx {
    pub ret): *mut *mut *mut int (complete)(struct crypto_async_request req, int,
    pub aes: ccp_aes_ctx,
    pub rsa: ccp_rsa_ctx,
    pub sha: ccp_sha_ctx,
    pub des3: ccp_des3_ctx,
    pub u: },
}

extern "C" {
    pub fn ccp_register_aes_algs(head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ccp_register_aes_cmac_algs(head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ccp_register_aes_xts_algs(head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ccp_register_aes_aeads(head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ccp_register_sha_algs(head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ccp_register_des3_algs(head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ccp_register_rsa_algs(head: *mut list_head) -> c_int;
}
