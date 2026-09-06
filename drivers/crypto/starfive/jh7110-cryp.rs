//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/starfive/jh7110-cryp.h
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

pub const STARFIVE_ALG_CR_OFFSET: c_uint = 0x0;
pub const STARFIVE_ALG_FIFO_OFFSET: c_uint = 0x4;
pub const STARFIVE_IE_MASK_OFFSET: c_uint = 0x8;
pub const STARFIVE_IE_FLAG_OFFSET: c_uint = 0xc;
pub const STARFIVE_DMA_IN_LEN_OFFSET: c_uint = 0x10;
pub const STARFIVE_DMA_OUT_LEN_OFFSET: c_uint = 0x14;
pub const STARFIVE_IE_MASK_AES_DONE: c_uint = 0x1;
pub const STARFIVE_IE_MASK_HASH_DONE: c_uint = 0x4;
pub const STARFIVE_IE_MASK_PKA_DONE: c_uint = 0x8;
pub const STARFIVE_IE_FLAG_AES_DONE: c_uint = 0x1;
pub const STARFIVE_IE_FLAG_HASH_DONE: c_uint = 0x4;
pub const STARFIVE_IE_FLAG_PKA_DONE: c_uint = 0x8;

pub const STARFIVE_RSA_MAX_KEYSZ: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub union starfive_aes_csr {
    pub v: u32,
    pub :1: u32 cmode,
pub const STARFIVE_AES_KEYMODE_128: c_uint = 0x0;
pub const STARFIVE_AES_KEYMODE_192: c_uint = 0x1;
pub const STARFIVE_AES_KEYMODE_256: c_uint = 0x2;
    pub :2: u32 keymode,

    pub :1: u32 busy,
    pub :1: u32 done,

    pub :1: u32 krdy,
    pub :1: u32 aesrst,
    pub :1: u32 ie,

    pub :1: u32 ccm_start,
pub const STARFIVE_AES_MODE_ECB: c_uint = 0x0;
pub const STARFIVE_AES_MODE_CBC: c_uint = 0x1;
pub const STARFIVE_AES_MODE_CTR: c_uint = 0x4;
pub const STARFIVE_AES_MODE_CCM: c_uint = 0x5;
pub const STARFIVE_AES_MODE_GCM: c_uint = 0x6;
    pub :3: u32 mode,

    pub :1: u32 gcm_start,

    pub :1: u32 gcm_done,
    pub :1: u32 delay_aes,
    pub :1: u32 vaes_start,
    pub :8: u32 rsvd_0,
pub const STARFIVE_AES_MODE_XFB_1: c_uint = 0x0;
pub const STARFIVE_AES_MODE_XFB_128: c_uint = 0x5;
    pub :3: u32 stmode,
    pub :5: u32 rsvd_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union starfive_hash_csr {
    pub v: u32,
    pub :1: u32 start,
    pub :1: u32 reset,
    pub :1: u32 ie,
    pub :1: u32 firstb,
pub const STARFIVE_HASH_SM3: c_uint = 0x0;
pub const STARFIVE_HASH_SHA224: c_uint = 0x3;
pub const STARFIVE_HASH_SHA256: c_uint = 0x4;
pub const STARFIVE_HASH_SHA384: c_uint = 0x5;
pub const STARFIVE_HASH_SHA512: c_uint = 0x6;
pub const STARFIVE_HASH_MODE_MASK: c_uint = 0x7;
    pub :3: u32 mode,
    pub :1: u32 rsvd_1,
    pub :1: u32 final,
    pub :2: u32 rsvd_2,
pub const STARFIVE_HASH_HMAC_FLAGS: c_uint = 0x800;
    pub :1: u32 hmac,
    pub :1: u32 rsvd_3,

    pub :1: u32 key_done,
    pub :1: u32 key_flag,

    pub :1: u32 hmac_done,

    pub :1: u32 busy,
    pub :1: u32 hashdone,
    pub :14: u32 rsvd_4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union starfive_pka_cacr {
    pub v: u32,
    pub :1: u32 start,
    pub :1: u32 reset,
    pub :1: u32 ie,
    pub :1: u32 rsvd_0,
    pub :1: u32 fifo_mode,
    pub :1: u32 not_r2,
    pub :1: u32 ecc_sub,
    pub :1: u32 pre_expf,
    pub :4: u32 cmd,
    pub :1: u32 rsvd_1,
    pub :1: u32 ctrl_dummy,
    pub :1: u32 ctrl_false,
    pub :1: u32 cln_done,
    pub :6: u32 opsize,
    pub :2: u32 rsvd_2,
    pub :6: u32 exposize,
    pub :1: u32 rsvd_3,
    pub :1: u32 bigendian,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union starfive_pka_casr {
    pub v: u32,

    pub :1: u32 done,
    pub :31: u32 rsvd_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct starfive_rsa_key {
    pub n: *mut u8,
    pub e: *mut u8,
    pub d: *mut u8,
    pub e_bitlen: c_int,
    pub d_bitlen: c_int,
    pub bitlen: c_int,
    pub key_sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union starfive_alg_cr {
    pub v: u32,
    pub :1: u32 start,
    pub :1: u32 aes_dma_en,
    pub :1: u32 rsvd_0,
    pub :1: u32 hash_dma_en,
    pub :1: u32 alg_done,
    pub :3: u32 rsvd_1,
    pub :1: u32 clear,
    pub :23: u32 rsvd_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct starfive_cryp_ctx {
    pub cryp: *mut starfive_cryp_dev,
    pub rctx: *mut starfive_cryp_request_ctx,
    pub hash_mode: c_uint,
    pub key: [u8; MAX_KEY_SIZE],
    pub keylen: c_int,
    pub is_hmac: bool,
    pub rsa_key: starfive_rsa_key,
    pub akcipher_fbk: *mut crypto_akcipher,
    pub ahash_fbk: *mut crypto_ahash,
    pub aead_fbk: *mut crypto_aead,
    pub skcipher_fbk: *mut crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct starfive_cryp_dev {
    pub list: list_head,
    pub dev: *mut device,
    pub hclk: *mut clk,
    pub ahb: *mut clk,
    pub rst: *mut reset_control,
    pub base: *mut void __iomem,
    pub phys_base: phys_addr_t,
    pub dma_maxburst: u32,
    pub tx: *mut dma_chan,
    pub rx: *mut dma_chan,
    pub cfg_in: dma_slave_config,
    pub cfg_out: dma_slave_config,
    pub engine: *mut crypto_engine,
    pub dma_done: completion,
    pub assoclen: usize,
    pub total_in: usize,
    pub total_out: usize,
    pub tag_in: [u32; 4],
    pub tag_out: [u32; 4],
    pub authsize: c_uint,
    pub flags: c_ulong,
    pub err: c_int,
    pub side_chan: bool,
    pub alg_cr: starfive_alg_cr,
    pub hreq: *mut ahash_request,
    pub areq: *mut aead_request,
    pub sreq: *mut skcipher_request,
    pub req: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct starfive_cryp_request_ctx {
    pub hash: starfive_hash_csr,
    pub pka: starfive_pka_cacr,
    pub aes: starfive_aes_csr,
    pub csr: },
    pub in_sg: *mut scatterlist,
    pub out_sg: *mut scatterlist,
    pub total: usize,
    pub blksize: c_uint,
    pub digsize: c_uint,
    pub in_sg_len: c_ulong,
    pub adata: *mut c_uchar,
    pub __aligned(sizeof(u32)): u8 rsa_data[STARFIVE_RSA_MAX_KEYSZ],
// Must be last as it ends in a flexible-array member.
    pub ahash_fbk_req: ahash_request,
}

extern "C" {
    pub fn starfive_hash_register_algs() -> c_int;
}
extern "C" {
    pub fn starfive_hash_unregister_algs();
}
extern "C" {
    pub fn starfive_rsa_register_algs() -> c_int;
}
extern "C" {
    pub fn starfive_rsa_unregister_algs();
}
extern "C" {
    pub fn starfive_aes_register_algs() -> c_int;
}
extern "C" {
    pub fn starfive_aes_unregister_algs();
}
