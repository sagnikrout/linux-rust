//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/allwinner/sun8i-ce/sun8i-ce.h
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
// sun8i-ce.h - hardware cryptographic offloader for
// Allwinner H3/A64/H5/H2+/H6 SoC
//
// Copyright (C) 2016-2019 Corentin LABBE <clabbe.montjoie@gmail.com>
//

// CE Registers
pub const CE_TDQ: c_uint = 0x00;
pub const CE_CTR: c_uint = 0x04;
pub const CE_ICR: c_uint = 0x08;
pub const CE_ISR: c_uint = 0x0C;
pub const CE_TLR: c_uint = 0x10;
pub const CE_TSR: c_uint = 0x14;
pub const CE_ESR: c_uint = 0x18;
pub const CE_CSSGR: c_uint = 0x1C;
pub const CE_CDSGR: c_uint = 0x20;
pub const CE_CSAR: c_uint = 0x24;
pub const CE_CDAR: c_uint = 0x28;
pub const CE_TPR: c_uint = 0x2C;
// Used in struct ce_task
// ce_task common
pub const CE_ENCRYPTION: c_int = 0;

// ce_task symmetric
pub const CE_AES_128BITS: c_int = 0;
pub const CE_AES_192BITS: c_int = 1;
pub const CE_AES_256BITS: c_int = 2;
pub const CE_OP_ECB: c_int = 0;

pub const CE_ALG_AES: c_int = 0;
pub const CE_ALG_DES: c_int = 1;
pub const CE_ALG_3DES: c_int = 2;
pub const CE_ALG_MD5: c_int = 16;
pub const CE_ALG_SHA1: c_int = 17;
pub const CE_ALG_SHA224: c_int = 18;
pub const CE_ALG_SHA256: c_int = 19;
pub const CE_ALG_SHA384: c_int = 20;
pub const CE_ALG_SHA512: c_int = 21;
pub const CE_ALG_TRNG: c_int = 48;
pub const CE_ALG_TRNG_V2: c_uint = 0x1c;
// Used in ce_variant
pub const CE_ID_NOTSUPP: c_uint = 0xFF;
pub const CE_ID_CIPHER_AES: c_int = 0;
pub const CE_ID_CIPHER_DES: c_int = 1;
pub const CE_ID_CIPHER_DES3: c_int = 2;
pub const CE_ID_CIPHER_MAX: c_int = 3;
pub const CE_ID_HASH_MD5: c_int = 0;
pub const CE_ID_HASH_SHA1: c_int = 1;
pub const CE_ID_HASH_SHA224: c_int = 2;
pub const CE_ID_HASH_SHA256: c_int = 3;
pub const CE_ID_HASH_SHA384: c_int = 4;
pub const CE_ID_HASH_SHA512: c_int = 5;
pub const CE_ID_HASH_MAX: c_int = 6;
pub const CE_ID_OP_ECB: c_int = 0;
pub const CE_ID_OP_CBC: c_int = 1;
pub const CE_ID_OP_MAX: c_int = 2;
// Used in CE registers

pub const ESR_H3: c_int = 0;
pub const ESR_A64: c_int = 1;
pub const ESR_R40: c_int = 2;
pub const ESR_H5: c_int = 3;
pub const ESR_H6: c_int = 4;
pub const ESR_D1: c_int = 5;
pub const CE_DIE_ID_SHIFT: c_int = 16;
pub const CE_DIE_ID_MASK: c_uint = 0x07;
pub const MAX_SG: c_int = 8;
pub const CE_MAX_CLOCKS: c_int = 4;
pub const CE_DMA_TIMEOUT_MS: c_int = 3000;
pub const MAXFLOW: c_int = 4;

//
// struct ce_clock - Describe clocks used by sun8i-ce
// @name:	Name of clock needed by this variant
// @freq:	Frequency to set for each clock
// @max_freq:	Maximum frequency for each clock (generally given by datasheet)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_clock {
    pub name: *const c_char,
    pub freq: c_ulong,
    pub max_freq: c_ulong,
}

//
// struct ce_variant - Describe CE capability for each variant hardware
// @alg_cipher:	list of supported ciphers. for each CE_ID_ this will give the
// coresponding CE_ALG_XXX value
// @alg_hash:	list of supported hashes. for each CE_ID_ this will give the
// corresponding CE_ALG_XXX value
// @op_mode:	list of supported block modes
// @cipher_t_dlen_in_bytes:	Does the request size for cipher is in
// bytes or words
// @hash_t_dlen_in_bytes:	Does the request size for hash is in
// bits or words
// @trng_t_dlen_in_bytes:	Does the request size for TRNG is in
// bytes or words
// @ce_clks:	list of clocks needed by this variant
// @esr:	The type of error register
// @trng:	The CE_ALG_XXX value for the TRNG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_variant {
    pub alg_cipher: [c_char; CE_ID_CIPHER_MAX],
    pub alg_hash: [c_char; CE_ID_HASH_MAX],
    pub op_mode: [u32; CE_ID_OP_MAX],
    pub cipher_t_dlen_in_bytes: bool,
    pub hash_t_dlen_in_bits: bool,
    pub trng_t_dlen_in_bytes: bool,
    pub needs_word_addresses: bool,
    pub ce_clks: [ce_clock; CE_MAX_CLOCKS],
    pub esr: c_int,
    pub trng: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sginfo {
    pub addr: __le32,
    pub len: __le32,
    pub __packed: },
//
// struct ce_task - CE Task descriptor
// The structure of this descriptor could be found in the datasheet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_task {
    pub t_id: __le32,
    pub t_common_ctl: __le32,
    pub t_sym_ctl: __le32,
    pub t_asym_ctl: __le32,
    pub t_key: __le32,
    pub t_iv: __le32,
    pub t_ctr: __le32,
    pub t_dlen: __le32,
    pub t_src: [sginfo; MAX_SG],
    pub t_dst: [sginfo; MAX_SG],
    pub next: __le32,
    pub reserved: [__le32; 3],
    pub __aligned(8): } __packed,
//
// struct sun8i_ce_flow - Information used by each flow
// @engine:	ptr to the crypto_engine for this flow
// @complete:	completion for the current task on this flow
// @status:	set to 1 by interrupt if task is done
// @t_phy:	Physical address of task
// @tl:		pointer to the current ce_task for this flow
// @stat_req:	number of request done by this flow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ce_flow {
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: c_int,
    pub t_phy: dma_addr_t,
    pub tl: *mut ce_task,

    pub stat_req: c_ulong,

}

//
// struct sun8i_ce_dev - main container for all this driver information
// @base:	base address of CE
// @ceclks:	clocks used by CE
// @reset:	pointer to reset controller
// @dev:	the platform device
// @mlock:	Control access to device registers
// @rnglock:	Control access to the RNG (dedicated channel 3)
// @chanlist:	array of all flow
// @flow:	flow to use in next request
// @variant:	pointer to variant specific data
// @dbgfs_dir:	Debugfs dentry for statistic directory
// @dbgfs_stats: Debugfs dentry for statistic counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ce_dev {
    pub base: *mut void __iomem,
    pub ceclks: [*mut clk; CE_MAX_CLOCKS],
    pub reset: *mut reset_control,
    pub dev: *mut device,
    pub mlock: mutex,
    pub rnglock: mutex,
    pub chanlist: *mut sun8i_ce_flow,
    pub flow: core::sync::atomic::AtomicI32,
    pub variant: *const ce_variant,

    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,

    pub trng: hwrng,

    pub hwrng_stat_req: c_ulong,
    pub hwrng_stat_bytes: c_ulong,

}

extern "C" {
    pub fn cpu_to_le32(_arg: desc_addr_val(dev, _arg: addr)) -> return;
}
//
// struct sun8i_cipher_req_ctx - context for a skcipher request
// @op_dir:		direction (encrypt vs decrypt) for this request
// @flow:		the flow to use for this request
// @nr_sgs:		The number of source SG (as given by dma_map_sg())
// @nr_sgd:		The number of destination SG (as given by dma_map_sg())
// @addr_iv:		The IV addr returned by dma_map_single, need to unmap later
// @addr_key:		The key addr returned by dma_map_single, need to unmap later
// @bounce_iv:		Current IV buffer
// @backup_iv:		Next IV buffer
// @fallback_req:	request struct for invoking the fallback skcipher TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_cipher_req_ctx {
    pub op_dir: u32,
    pub flow: c_int,
    pub nr_sgs: c_int,
    pub nr_sgd: c_int,
    pub addr_iv: dma_addr_t,
    pub addr_key: dma_addr_t,
    pub __aligned(sizeof(u32)): u8 bounce_iv[AES_BLOCK_SIZE],
    pub backup_iv: [u8; AES_BLOCK_SIZE],
    pub end: skcipher_request fallback_req; // keep at the,
}

//
// struct sun8i_cipher_tfm_ctx - context for a skcipher TFM
// @key:		pointer to key data
// @keylen:		len of the key
// @ce:			pointer to the private data of driver handling this TFM
// @fallback_tfm:	pointer to the fallback TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_cipher_tfm_ctx {
    pub key: *mut u32,
    pub keylen: u32,
    pub ce: *mut sun8i_ce_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}

//
// struct sun8i_ce_hash_tfm_ctx - context for an ahash TFM
// @ce:			pointer to the private data of driver handling this TFM
// @fallback_tfm:	pointer to the fallback TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ce_hash_tfm_ctx {
    pub ce: *mut sun8i_ce_dev,
    pub fallback_tfm: *mut crypto_ahash,
}

//
// struct sun8i_ce_hash_reqctx - context for an ahash request
// @fallback_req:	pre-allocated fallback request
// @flow:	the flow to use for this request
// @nr_sgs: number of entries in the source scatterlist
// @result_len: result length in bytes
// @pad_len: padding length in bytes
// @addr_res: DMA address of the result buffer, returned by dma_map_single()
// @addr_pad: DMA address of the padding buffer, returned by dma_map_single()
// @result: per-request result buffer
// @pad: per-request padding buffer (up to 2 blocks)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ce_hash_reqctx {
    pub flow: c_int,
    pub nr_sgs: c_int,
    pub result_len: usize,
    pub pad_len: usize,
    pub addr_res: dma_addr_t,
    pub addr_pad: dma_addr_t,
    pub __aligned(CRYPTO_DMA_ALIGN): u8 result[CE_MAX_HASH_DIGEST_SIZE],
    pub CE_MAX_HASH_BLOCK_SIZE]: *mut *mut u8 pad[2,
    pub end: ahash_request fallback_req; // keep at the,
}

//
// struct sun8i_ce_alg_template - crypto_alg template
// @type:		the CRYPTO_ALG_TYPE for this template
// @ce_algo_id:		the CE_ID for this template
// @ce_blockmode:	the type of block operation CE_ID
// @ce:			pointer to the sun8i_ce_dev structure associated with
// this template
// @alg:		one of sub struct must be used
// @stat_req:		number of request done on this template
// @stat_fb:		number of request which has fallbacked
// @stat_bytes:		total data size done by this template
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_ce_alg_template {
    pub type: u32,
    pub ce_algo_id: u32,
    pub ce_blockmode: u32,
    pub ce: *mut sun8i_ce_dev,
    pub skcipher: skcipher_engine_alg,
    pub hash: ahash_engine_alg,
    pub alg: },
    pub stat_req: c_ulong,
    pub stat_fb: c_ulong,
    pub stat_bytes: c_ulong,
    pub stat_fb_maxsg: c_ulong,
    pub stat_fb_leniv: c_ulong,
    pub stat_fb_len0: c_ulong,
    pub stat_fb_mod16: c_ulong,
    pub stat_fb_srcali: c_ulong,
    pub stat_fb_srclen: c_ulong,
    pub stat_fb_dstali: c_ulong,
    pub stat_fb_dstlen: c_ulong,
    pub fbname: [c_char; CRYPTO_MAX_ALG_NAME],
}

extern "C" {
    pub fn sun8i_ce_cipher_init(tfm: *mut crypto_tfm) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_cipher_exit(tfm: *mut crypto_tfm);
}
extern "C" {
    pub fn sun8i_ce_cipher_do_one(engine: *mut crypto_engine, areq: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_skdecrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_skencrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_get_engine_number(ce: *mut sun8i_ce_dev) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_run_task(ce: *mut sun8i_ce_dev, flow: c_int, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_init_tfm(tfm: *mut crypto_ahash) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_exit_tfm(tfm: *mut crypto_ahash);
}
extern "C" {
    pub fn sun8i_ce_hash_init(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_export(areq: *mut ahash_request, out: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_import(areq: *mut ahash_request, in: *const c_void) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_final(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_update(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_finup(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_digest(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hash_run(engine: *mut crypto_engine, breq: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hwrng_register(ce: *mut sun8i_ce_dev) -> c_int;
}
extern "C" {
    pub fn sun8i_ce_hwrng_unregister(ce: *mut sun8i_ce_dev);
}
