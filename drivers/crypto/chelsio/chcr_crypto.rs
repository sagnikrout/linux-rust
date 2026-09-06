//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/chelsio/chcr_crypto.h
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


//
// This file is part of the Chelsio T6 Crypto driver for Linux.
//
// Copyright (c) 2003-2016 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
pub const GHASH_BLOCK_SIZE: c_int = 16;
pub const GHASH_DIGEST_SIZE: c_int = 16;
pub const CCM_B0_SIZE: c_int = 16;
pub const CCM_AAD_FIELD_SIZE: c_int = 2;
// 511 - 16(For IV)
pub const T6_MAX_AAD_SIZE: c_int = 495;
// Define following if h/w is not dropping the AAD and IV data before
// giving the processed data
//
pub const CHCR_CRA_PRIORITY: c_int = 500;
pub const CHCR_AEAD_PRIORITY: c_int = 6000;

pub const CHCR_GIVENCRYPT_OP: c_int = 2;
// CPL/SCMD parameters
pub const CHCR_ENCRYPT_OP: c_int = 0;
pub const CHCR_DECRYPT_OP: c_int = 1;
pub const CHCR_SCMD_SEQ_NO_CTRL_32BIT: c_int = 1;
pub const CHCR_SCMD_SEQ_NO_CTRL_48BIT: c_int = 2;
pub const CHCR_SCMD_SEQ_NO_CTRL_64BIT: c_int = 3;
pub const CHCR_SCMD_PROTO_VERSION_GENERIC: c_int = 4;
pub const CHCR_SCMD_AUTH_CTRL_AUTH_CIPHER: c_int = 0;
pub const CHCR_SCMD_AUTH_CTRL_CIPHER_AUTH: c_int = 1;
pub const CHCR_SCMD_CIPHER_MODE_NOP: c_int = 0;
pub const CHCR_SCMD_CIPHER_MODE_AES_CBC: c_int = 1;
pub const CHCR_SCMD_CIPHER_MODE_AES_GCM: c_int = 2;
pub const CHCR_SCMD_CIPHER_MODE_AES_CTR: c_int = 3;
pub const CHCR_SCMD_CIPHER_MODE_GENERIC_AES: c_int = 4;
pub const CHCR_SCMD_CIPHER_MODE_AES_XTS: c_int = 6;
pub const CHCR_SCMD_CIPHER_MODE_AES_CCM: c_int = 7;
pub const CHCR_SCMD_AUTH_MODE_NOP: c_int = 0;
pub const CHCR_SCMD_AUTH_MODE_SHA1: c_int = 1;
pub const CHCR_SCMD_AUTH_MODE_SHA224: c_int = 2;
pub const CHCR_SCMD_AUTH_MODE_SHA256: c_int = 3;
pub const CHCR_SCMD_AUTH_MODE_GHASH: c_int = 4;
pub const CHCR_SCMD_AUTH_MODE_SHA512_224: c_int = 5;
pub const CHCR_SCMD_AUTH_MODE_SHA512_256: c_int = 6;
pub const CHCR_SCMD_AUTH_MODE_SHA512_384: c_int = 7;
pub const CHCR_SCMD_AUTH_MODE_SHA512_512: c_int = 8;
pub const CHCR_SCMD_AUTH_MODE_CBCMAC: c_int = 9;
pub const CHCR_SCMD_AUTH_MODE_CMAC: c_int = 10;
pub const CHCR_SCMD_HMAC_CTRL_NOP: c_int = 0;
pub const CHCR_SCMD_HMAC_CTRL_NO_TRUNC: c_int = 1;
pub const CHCR_SCMD_HMAC_CTRL_TRUNC_RFC4366: c_int = 2;
pub const CHCR_SCMD_HMAC_CTRL_IPSEC_96BIT: c_int = 3;
pub const CHCR_SCMD_HMAC_CTRL_PL1: c_int = 4;
pub const CHCR_SCMD_HMAC_CTRL_PL2: c_int = 5;
pub const CHCR_SCMD_HMAC_CTRL_PL3: c_int = 6;
pub const CHCR_SCMD_HMAC_CTRL_DIV2: c_int = 7;
pub const VERIFY_HW: c_int = 0;
pub const VERIFY_SW: c_int = 1;
pub const CHCR_SCMD_IVGEN_CTRL_HW: c_int = 0;
pub const CHCR_SCMD_IVGEN_CTRL_SW: c_int = 1;
// This are not really mac key size. They are intermediate values
// of sha engine and its size
//
pub const CHCR_KEYCTX_MAC_KEY_SIZE_128: c_int = 0;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_160: c_int = 1;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_192: c_int = 2;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_256: c_int = 3;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_512: c_int = 4;
pub const CHCR_KEYCTX_CIPHER_KEY_SIZE_128: c_int = 0;
pub const CHCR_KEYCTX_CIPHER_KEY_SIZE_192: c_int = 1;
pub const CHCR_KEYCTX_CIPHER_KEY_SIZE_256: c_int = 2;
pub const CHCR_KEYCTX_NO_KEY: c_int = 15;

pub const KEY_CONTEXT_HDR_SALT_AND_PAD: c_int = 16;

pub const IV_NOP: c_int = 0;
pub const IV_IMMEDIATE: c_int = 1;
pub const IV_DSGL: c_int = 2;
pub const AEAD_H_SIZE: c_int = 16;
pub const CRYPTO_ALG_SUB_TYPE_MASK: c_uint = 0x0f000000;
pub const CRYPTO_ALG_SUB_TYPE_HASH_HMAC: c_uint = 0x01000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_RFC4106: c_uint = 0x02000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_GCM: c_uint = 0x03000000;
pub const CRYPTO_ALG_SUB_TYPE_CBC_SHA: c_uint = 0x04000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_CCM: c_uint = 0x05000000;
pub const CRYPTO_ALG_SUB_TYPE_AEAD_RFC4309: c_uint = 0x06000000;
pub const CRYPTO_ALG_SUB_TYPE_CBC_NULL: c_uint = 0x07000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR: c_uint = 0x08000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR_RFC3686: c_uint = 0x09000000;
pub const CRYPTO_ALG_SUB_TYPE_XTS: c_uint = 0x0a000000;
pub const CRYPTO_ALG_SUB_TYPE_CBC: c_uint = 0x0b000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR_SHA: c_uint = 0x0c000000;
pub const CRYPTO_ALG_SUB_TYPE_CTR_NULL: c_uint = 0x0d000000;

pub const MAX_SCRATCH_PAD_SIZE: c_int = 32;
pub const CHCR_HASH_MAX_BLOCK_SIZE_64: c_int = 64;
pub const CHCR_HASH_MAX_BLOCK_SIZE_128: c_int = 128;

pub const CHCR_DST_SG_SIZE: c_int = 2048;
extern "C" {
    pub fn crypto_aead_ctx(_arg: tfm) -> return;
}
extern "C" {
    pub fn crypto_skcipher_ctx(_arg: tfm) -> return;
}
extern "C" {
    pub fn crypto_tfm_ctx(_arg: crypto_ahash_tfm(tfm)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ablk_ctx {
    pub sw_cipher: *mut crypto_skcipher,
    pub key_ctx_hdr: __be32,
    pub enckey_len: c_uint,
    pub ciph_mode: c_uchar,
    pub key: [u8; CHCR_AES_MAX_KEY_LEN],
    pub nonce: [u8; 4],
    pub rrkey: [u8; AES_MAX_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_aead_reqctx {
    pub skb: *mut sk_buff,
    pub iv_dma: dma_addr_t,
    pub b0_dma: dma_addr_t,
    pub b0_len: c_uint,
    pub op: c_uint,
    pub imm: u16,
    pub verify: u16,
    pub txqidx: u16,
    pub rxqidx: u16,
    pub MAX_SCRATCH_PAD_SIZE]: u8 iv[CHCR_MAX_CRYPTO_IV_LEN +,
    pub scratch_pad: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulptx_walk {
    pub sgl: *mut ulptx_sgl,
    pub nents: c_uint,
    pub pair_idx: c_uint,
    pub last_sg_len: c_uint,
    pub last_sg: *mut scatterlist,
    pub pair: *mut ulptx_sge_pair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsgl_walk {
    pub nents: c_uint,
    pub last_sg_len: c_uint,
    pub last_sg: *mut scatterlist,
    pub dsgl: *mut cpl_rx_phys_dsgl,
    pub to: *mut phys_sge_pairs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_gcm_ctx {
    pub ghash_h: [u8; AEAD_H_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_authenc_ctx {
    pub dec_rrkey: [u8; AES_MAX_KEY_SIZE],
    pub CHCR_HASH_MAX_DIGEST_SIZE]: *mut *mut u8 h_iopad[2,
    pub auth_mode: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __aead_ctx {
    pub gcm): DECLARE_FLEX_ARRAY(struct chcr_gcm_ctx,,
    pub authenc): DECLARE_FLEX_ARRAY(struct chcr_authenc_ctx,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_aead_ctx {
    pub key_ctx_hdr: __be32,
    pub enckey_len: c_uint,
    pub sw_cipher: *mut crypto_aead,
    pub salt: [u8; MAX_SALT],
    pub key: [u8; CHCR_AES_MAX_KEY_LEN],
    pub nonce: [u8; 4],
    pub hmac_ctrl: u16,
    pub mayverify: u16,
    pub ctx: [__aead_ctx; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_ctx {
    pub ipad: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128],
    pub opad: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __crypto_ctx {
    pub hmacctx: hmac_ctx,
    pub ablkctx: ablk_ctx,
    pub aeadctx: chcr_aead_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_context {
    pub dev: *mut chcr_dev,
    pub rxq_perchan: c_uchar,
    pub txq_perchan: c_uchar,
    pub ntxq: c_uint,
    pub nrxq: c_uint,
    pub cbc_aes_aio_done: completion,
    pub crypto_ctx: [__crypto_ctx; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_hctx_per_wr {
    pub srcsg: *mut scatterlist,
    pub skb: *mut sk_buff,
    pub dma_addr: dma_addr_t,
    pub dma_len: u32,
    pub src_ofst: c_uint,
    pub processed: c_uint,
    pub result: u32,
    pub is_sg_map: u8,
    pub imm: u8,
// Final callback called. Driver cannot rely on nbytes to decide
// final call
//
    pub isfinal: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ahash_req_ctx {
    pub hctx_wr: chcr_hctx_per_wr,
    pub reqbfr: *mut u8,
    pub skbfr: *mut u8,
// SKB which is being sent to the hardware for processing
    pub /: *mut *mut u64 data_len; / Data len till time,
    pub txqidx: u16,
    pub rxqidx: u16,
    pub reqlen: u8,
    pub partial_hash: [u8; CHCR_HASH_MAX_DIGEST_SIZE],
    pub bfr1: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128],
    pub bfr2: [u8; CHCR_HASH_MAX_BLOCK_SIZE_128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_skcipher_req_ctx {
    pub skb: *mut sk_buff,
    pub dstsg: *mut scatterlist,
    pub processed: c_uint,
    pub last_req_len: c_uint,
    pub partial_req: c_uint,
    pub srcsg: *mut scatterlist,
    pub src_ofst: c_uint,
    pub dst_ofst: c_uint,
    pub op: c_uint,
    pub imm: u16,
    pub iv: [u8; CHCR_MAX_CRYPTO_IV_LEN],
    pub init_iv: [u8; CHCR_MAX_CRYPTO_IV_LEN],
    pub txqidx: u16,
    pub rxqidx: u16,
    pub end: skcipher_request fallback_req; // keep at the,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_alg_template {
    pub type: u32,
    pub is_registered: u32,
    pub skcipher: skcipher_alg,
    pub hash: ahash_alg,
    pub aead: aead_alg,
    pub alg: },
}

extern "C" {
    pub fn chcr_verify_tag(req: *mut aead_request, input: *mut u8, err: *mut c_int);
}
extern "C" {
    pub fn chcr_add_aead_src_ent(req: *mut aead_request, ulptx: *mut ulptx_sgl);
}
extern "C" {
    pub fn chcr_cipher_dma_map(dev: *mut device, req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn chcr_cipher_dma_unmap(dev: *mut device, req: *mut skcipher_request);
}
extern "C" {
    pub fn chcr_hash_dma_map(dev: *mut device, req: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn chcr_hash_dma_unmap(dev: *mut device, req: *mut ahash_request);
}
extern "C" {
    pub fn chcr_aead_common_exit(req: *mut aead_request);
}
