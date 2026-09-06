//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/allwinner/sun8i-ss/sun8i-ss.h
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
// sun8i-ss.h - hardware cryptographic offloader for
// Allwinner A80/A83T SoC
//
// Copyright (C) 2016-2019 Corentin LABBE <clabbe.montjoie@gmail.com>
//

pub const SS_START: c_int = 1;
pub const SS_ENCRYPTION: c_int = 0;

pub const SS_ALG_AES: c_int = 0;

pub const SS_CTL_REG: c_uint = 0x00;
pub const SS_INT_CTL_REG: c_uint = 0x04;
pub const SS_INT_STA_REG: c_uint = 0x08;
pub const SS_KEY_ADR_REG: c_uint = 0x10;
pub const SS_IV_ADR_REG: c_uint = 0x18;
pub const SS_SRC_ADR_REG: c_uint = 0x20;
pub const SS_DST_ADR_REG: c_uint = 0x28;
pub const SS_LEN_ADR_REG: c_uint = 0x30;
pub const SS_ID_NOTSUPP: c_uint = 0xFF;
pub const SS_ID_CIPHER_AES: c_int = 0;
pub const SS_ID_CIPHER_DES: c_int = 1;
pub const SS_ID_CIPHER_DES3: c_int = 2;
pub const SS_ID_CIPHER_MAX: c_int = 3;
pub const SS_ID_OP_ECB: c_int = 0;
pub const SS_ID_OP_CBC: c_int = 1;
pub const SS_ID_OP_MAX: c_int = 2;
pub const SS_AES_128BITS: c_int = 0;
pub const SS_AES_192BITS: c_int = 1;
pub const SS_AES_256BITS: c_int = 2;
pub const SS_OP_ECB: c_int = 0;

pub const SS_ID_HASH_MD5: c_int = 0;
pub const SS_ID_HASH_SHA1: c_int = 1;
pub const SS_ID_HASH_SHA224: c_int = 2;
pub const SS_ID_HASH_SHA256: c_int = 3;
pub const SS_ID_HASH_MAX: c_int = 4;

pub const MAX_SG: c_int = 8;
pub const MAXFLOW: c_int = 2;
pub const SS_MAX_CLOCKS: c_int = 2;
pub const SS_DIE_ID_SHIFT: c_int = 20;
pub const SS_DIE_ID_MASK: c_uint = 0x07;
pub const MAX_PAD_SIZE: c_int = 4096;
//
// struct ss_clock - Describe clocks used by sun8i-ss
// @name:       Name of clock needed by this variant
// @freq:       Frequency to set for each clock
// @max_freq:   Maximum frequency for each clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ss_clock {
    pub name: *const c_char,
    pub freq: c_ulong,
    pub max_freq: c_ulong,
}

//
// struct ss_variant - Describe SS capability for each variant hardware
// @alg_cipher:	list of supported ciphers. for each SS_ID_ this will give the
// coresponding SS_ALG_XXX value
// @alg_hash:	list of supported hashes. for each SS_ID_ this will give the
// corresponding SS_ALG_XXX value
// @op_mode:	list of supported block modes
// @ss_clks:	list of clock needed by this variant
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ss_variant {
    pub alg_cipher: [c_char; SS_ID_CIPHER_MAX],
    pub alg_hash: [c_char; SS_ID_HASH_MAX],
    pub op_mode: [u32; SS_ID_OP_MAX],
    pub ss_clks: [ss_clock; SS_MAX_CLOCKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sginfo {
    pub addr: u32,
    pub len: u32,
}

//
// struct sun8i_ss_flow - Information used by each flow
// @engine:	ptr to the crypto_engine for this flow
// @complete:	completion for the current task on this flow
// @status:	set to 1 by interrupt if task is done
// @stat_req:	number of request done by this flow
// @iv:		list of IV to use for each step
// @biv:	buffer which contain the backuped IV
// @pad:	padding buffer for hash operations
// @result:	buffer for storing the result of hash operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ss_flow {
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: c_int,
    pub iv: [*mut u8; MAX_SG],
    pub biv: *mut u8,
    pub pad: *mut c_void,
    pub result: *mut c_void,

    pub stat_req: c_ulong,

}

//
// struct sun8i_ss_dev - main container for all this driver information
// @base:	base address of SS
// @ssclks:	clocks used by SS
// @reset:	pointer to reset controller
// @dev:	the platform device
// @mlock:	Control access to device registers
// @flows:	array of all flow
// @flow:	flow to use in next request
// @variant:	pointer to variant specific data
// @dbgfs_dir:	Debugfs dentry for statistic directory
// @dbgfs_stats: Debugfs dentry for statistic counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ss_dev {
    pub base: *mut void __iomem,
    pub ssclks: [*mut clk; SS_MAX_CLOCKS],
    pub reset: *mut reset_control,
    pub dev: *mut device,
    pub mlock: mutex,
    pub flows: *mut sun8i_ss_flow,
    pub flow: core::sync::atomic::AtomicI32,
    pub variant: *const ss_variant,

    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,

}

//
// struct sun8i_cipher_req_ctx - context for a skcipher request
// @t_src:		list of mapped SGs with their size
// @t_dst:		list of mapped SGs with their size
// @p_key:		DMA address of the key
// @p_iv:		DMA address of the IVs
// @niv:		Number of IVs DMA mapped
// @method:		current algorithm for this request
// @op_mode:		op_mode for this request
// @op_dir:		direction (encrypt vs decrypt) for this request
// @flow:		the flow to use for this request
// @ivlen:		size of IVs
// @keylen:		keylen for this request
// @fallback_req:	request struct for invoking the fallback skcipher TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_cipher_req_ctx {
    pub t_src: [sginfo; MAX_SG],
    pub t_dst: [sginfo; MAX_SG],
    pub p_key: u32,
    pub p_iv: [u32; MAX_SG],
    pub niv: c_int,
    pub method: u32,
    pub op_mode: u32,
    pub op_dir: u32,
    pub flow: c_int,
    pub ivlen: c_uint,
    pub keylen: c_uint,
    pub end: skcipher_request fallback_req; // keep at the,
}

//
// struct sun8i_cipher_tfm_ctx - context for a skcipher TFM
// @key:		pointer to key data
// @keylen:		len of the key
// @ss:			pointer to the private data of driver handling this TFM
// @fallback_tfm:	pointer to the fallback TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_cipher_tfm_ctx {
    pub key: *mut u32,
    pub keylen: u32,
    pub ss: *mut sun8i_ss_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}

//
// struct sun8i_ss_hash_tfm_ctx - context for an ahash TFM
// @fallback_tfm:	pointer to the fallback TFM
// @ss:			pointer to the private data of driver handling this TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ss_hash_tfm_ctx {
    pub fallback_tfm: *mut crypto_ahash,
    pub ss: *mut sun8i_ss_dev,
    pub ipad: *mut u8,
    pub opad: *mut u8,
    pub key: [u8; SHA256_BLOCK_SIZE],
    pub keylen: c_int,
}

//
// struct sun8i_ss_hash_reqctx - context for an ahash request
// @t_src:	list of DMA address and size for source SGs
// @t_dst:	list of DMA address and size for destination SGs
// @fallback_req:	pre-allocated fallback request
// @method:	the register value for the algorithm used by this request
// @flow:	the flow to use for this request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ss_hash_reqctx {
    pub t_src: [sginfo; MAX_SG],
    pub t_dst: [sginfo; MAX_SG],
    pub method: u32,
    pub flow: c_int,
// Must be last as it ends in a flexible-array member.
    pub fallback_req: ahash_request,
}

//
// struct sun8i_ss_alg_template - crypto_alg template
// @type:		the CRYPTO_ALG_TYPE for this template
// @ss_algo_id:		the SS_ID for this template
// @ss_blockmode:	the type of block operation SS_ID
// @ss:			pointer to the sun8i_ss_dev structure associated with
// this template
// @alg:		one of sub struct must be used
// @stat_req:		number of request done on this template
// @stat_fb:		number of request which has fallbacked
// @stat_bytes:		total data size done by this template
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ss_alg_template {
    pub type: u32,
    pub ss_algo_id: u32,
    pub ss_blockmode: u32,
    pub ss: *mut sun8i_ss_dev,
    pub skcipher: skcipher_engine_alg,
    pub hash: ahash_engine_alg,
    pub alg: },
    pub stat_req: c_ulong,
    pub stat_fb: c_ulong,
    pub stat_bytes: c_ulong,
    pub stat_fb_len: c_ulong,
    pub stat_fb_sglen: c_ulong,
    pub stat_fb_align: c_ulong,
    pub stat_fb_sgnum: c_ulong,
    pub fbname: [c_char; CRYPTO_MAX_ALG_NAME],
}

extern "C" {
    pub fn sun8i_ss_cipher_init(tfm: *mut crypto_tfm) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_cipher_exit(tfm: *mut crypto_tfm);
}
extern "C" {
    pub fn sun8i_ss_handle_cipher_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_skdecrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_skencrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_get_engine_number(ss: *mut sun8i_ss_dev) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_run_task(ss: *mut sun8i_ss_dev, rctx: *mut sun8i_cipher_req_ctx, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_init_tfm(tfm: *mut crypto_ahash) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_exit_tfm(tfm: *mut crypto_ahash);
}
extern "C" {
    pub fn sun8i_ss_hash_init(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_export(areq: *mut ahash_request, out: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_import(areq: *mut ahash_request, in: *const c_void) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_final(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_update(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_finup(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_digest(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ss_hash_run(engine: *mut crypto_engine, breq: *mut c_void) -> c_int;
}
