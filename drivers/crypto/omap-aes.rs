//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/omap-aes.h
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
// Cryptographic API.
//
// Support for OMAP AES HW ACCELERATOR defines
//
// Copyright (c) 2015 Texas Instruments Incorporated
//

pub const DST_MAXBURST: c_int = 4;

//
// OMAP TRM gives bitfields as start:end, where start is the higher bit
// number. For example 7:0
//

pub const AES_REG_CTRL_CTR_WIDTH_32: c_int = 0;

pub const AES_REG_C_LEN_0: c_uint = 0x54;
pub const AES_REG_C_LEN_1: c_uint = 0x58;
pub const AES_REG_A_LEN: c_uint = 0x5C;

pub const DEFAULT_AUTOSUSPEND_DELAY: c_int = 1000;
pub const FLAGS_MODE_MASK: c_uint = 0x001f;

pub const FLAGS_IN_DATA_ST_SHIFT: c_int = 8;
pub const FLAGS_OUT_DATA_ST_SHIFT: c_int = 10;
pub const FLAGS_ASSOC_DATA_ST_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_gcm_result {
    pub completion: completion,
    pub err: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_ctx {
    pub keylen: c_int,
    pub sizeof(u32)]: u32 key[AES_KEYSIZE_256 /,
    pub nonce: [u8; 4],
    pub fallback: *mut crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_gcm_ctx {
    pub octx: omap_aes_ctx,
    pub akey: aes_enckey,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_reqctx {
    pub dd: *mut omap_aes_dev,
    pub mode: c_ulong,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub sizeof(u32)]: u32 auth_tag[AES_BLOCK_SIZE /,
    pub end: skcipher_request fallback_req; // keep at the,
}

pub const OMAP_AES_QUEUE_LENGTH: c_int = 1;
pub const OMAP_AES_CACHE_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_algs_info {
    pub algs_list: *mut skcipher_engine_alg,
    pub size: c_uint,
    pub registered: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_aead_algs {
    pub algs_list: *mut aead_engine_alg,
    pub size: c_uint,
    pub registered: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_pdata {
    pub algs_info: *mut omap_aes_algs_info,
    pub algs_info_size: c_uint,
    pub aead_algs_info: *mut omap_aes_aead_algs,
    pub length): *mut *mut *mut void (trigger)(struct omap_aes_dev dd, int,
    pub key_ofs: u32,
    pub iv_ofs: u32,
    pub ctrl_ofs: u32,
    pub data_ofs: u32,
    pub rev_ofs: u32,
    pub mask_ofs: u32,
    pub irq_enable_ofs: u32,
    pub irq_status_ofs: u32,
    pub dma_enable_in: u32,
    pub dma_enable_out: u32,
    pub dma_start: u32,
    pub major_mask: u32,
    pub major_shift: u32,
    pub minor_mask: u32,
    pub minor_shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_aes_dev {
    pub list: list_head,
    pub phys_base: c_ulong,
    pub io_base: *mut void __iomem,
    pub ctx: *mut omap_aes_ctx,
    pub dev: *mut device,
    pub flags: c_ulong,
    pub err: c_int,
    pub done_task: work_struct,
    pub aead_queue: aead_queue,
    pub lock: spinlock_t,
    pub req: *mut skcipher_request,
    pub aead_req: *mut aead_request,
    pub engine: *mut crypto_engine,
//
// total is used by PIO mode for book keeping so introduce
// variable total_save as need it to calc page_order
//
    pub total: usize,
    pub total_save: usize,
    pub assoc_len: usize,
    pub authsize: usize,
    pub in_sg: *mut scatterlist,
    pub out_sg: *mut scatterlist,
// Buffers for copying for unaligned cases
    pub in_sgl: [scatterlist; 2],
    pub out_sgl: scatterlist,
    pub orig_out: *mut scatterlist,
    pub in_sg_offset: c_uint,
    pub out_sg_offset: c_uint,
    pub dma_lch_in: *mut dma_chan,
    pub dma_lch_out: *mut dma_chan,
    pub in_sg_len: c_int,
    pub out_sg_len: c_int,
    pub pio_only: c_int,
    pub pdata: *const omap_aes_pdata,
}

extern "C" {
    pub fn omap_aes_read(dd: *mut omap_aes_dev, offset: u32) -> u32;
}
extern "C" {
    pub fn omap_aes_write(dd: *mut omap_aes_dev, offset: u32, value: u32);
}
extern "C" {
    pub fn omap_aes_gcm_encrypt(req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn omap_aes_gcm_decrypt(req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn omap_aes_gcm_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int;
}
extern "C" {
    pub fn omap_aes_4106gcm_encrypt(req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn omap_aes_4106gcm_decrypt(req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn omap_aes_gcm_cra_init(tfm: *mut crypto_aead) -> c_int;
}
extern "C" {
    pub fn omap_aes_write_ctrl(dd: *mut omap_aes_dev) -> c_int;
}
extern "C" {
    pub fn omap_aes_crypt_dma_start(dd: *mut omap_aes_dev) -> c_int;
}
extern "C" {
    pub fn omap_aes_crypt_dma_stop(dd: *mut omap_aes_dev) -> c_int;
}
extern "C" {
    pub fn omap_aes_gcm_dma_out_callback(data: *mut c_void);
}
extern "C" {
    pub fn omap_aes_clear_copy_flags(dd: *mut omap_aes_dev);
}
extern "C" {
    pub fn omap_aes_gcm_crypt_req(engine: *mut crypto_engine, areq: *mut c_void) -> c_int;
}
