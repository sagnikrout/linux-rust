//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/amcc/crypto4xx_core.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMCC SoC PPC4xx Crypto Driver
//
// Copyright (c) 2008 Applied Micro Circuits Corporation.
// All rights reserved. James Hsiao <jhsiao@amcc.com>
//
// This is the header file for AMCC Crypto offload Linux device driver for
// use with Linux CryptoAPI.
//

pub const PPC460SX_SDR0_SRST: c_uint = 0x201;
pub const PPC405EX_SDR0_SRST: c_uint = 0x200;
pub const PPC460EX_SDR0_SRST: c_uint = 0x201;
pub const PPC460EX_CE_RESET: c_uint = 0x08000000;
pub const PPC460SX_CE_RESET: c_uint = 0x20000000;
pub const PPC405EX_CE_RESET: c_uint = 0x00000008;
pub const CRYPTO4XX_CRYPTO_PRIORITY: c_int = 300;
pub const PPC4XX_NUM_PD: c_int = 256;

pub const PPC4XX_NUM_GD: c_int = 1024;

pub const PPC4XX_NUM_SD: c_int = 256;

pub const PPC4XX_SD_BUFFER_SIZE: c_int = 2048;

pub const PD_ENTRY_FREE: c_int = 0;
pub const ERING_WAS_FULL: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub union shadow_sa_buf {
    pub sa: dynamic_sa_ctl,
// alloc 256 bytes which is enough for any kind of dynamic sa
    pub buf: [u8; 256],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pd_uinfo {
    pub dev: *mut crypto4xx_device,
    pub state: u32,
    pub discriptor: *mut *mut u32 first_gd; / first gather,
    pub discriptor: *mut *mut u32 num_gd; / number of gather,
    pub discriptor: *mut *mut u32 first_sd; / first scatter,
    pub discriptors: *mut *mut u32 num_sd; / number of scatter,
    pub /: *mut *mut *mut dynamic_sa_ctl sa_va; / shadow sa,
    pub /: *mut *mut *mut sa_state_record sr_va; / state record for shadow sa,
    pub sr_pa: u32,
    pub dest_va: *mut scatterlist,
    pub request: *mut *mut *mut crypto_async_request async_req; / base crypto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto4xx_device {
    pub core_dev: *mut crypto4xx_core_device,
    pub ce_base: *mut void __iomem,
    pub trng_base: *mut void __iomem,
    pub /: *mut *mut *mut ce_pd pdr; / base address of packet descriptor ring,
    pub /: *mut *mut dma_addr_t pdr_pa; / physical address of pdr_base_register,
    pub /: *mut *mut *mut ce_gd gdr; / gather descriptor ring,
    pub /: *mut *mut dma_addr_t gdr_pa; / physical address of gdr_base_register,
    pub /: *mut *mut *mut ce_sd sdr; / scatter descriptor ring,
    pub /: *mut *mut dma_addr_t sdr_pa; / physical address of sdr_base_register,
    pub scatter_buffer_va: *mut c_void,
    pub scatter_buffer_pa: dma_addr_t,
    pub shadow_sa_pool: *mut shadow_sa_buf,
    pub shadow_sa_pool_pa: dma_addr_t,
    pub shadow_sr_pool: *mut sa_state_record,
    pub shadow_sr_pool_pa: dma_addr_t,
    pub pdr_tail: u32,
    pub pdr_head: u32,
    pub gdr_tail: u32,
    pub gdr_head: u32,
    pub sdr_tail: u32,
    pub sdr_head: u32,
    pub pdr_uinfo: *mut pd_uinfo,
    pub supported: *mut *mut list_head alg_list; / List of algorithm,
    pub aead_ratelimit: ratelimit_state,
    pub is_revb: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto4xx_core_device {
    pub device: *mut device,
    pub ofdev: *mut platform_device,
    pub dev: *mut crypto4xx_device,
    pub trng: *mut hwrng,
    pub int_status: u32,
    pub irq: c_int,
    pub tasklet: tasklet_struct,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto4xx_ctx {
    pub dev: *mut crypto4xx_device,
    pub sa_in: *mut dynamic_sa_ctl,
    pub sa_out: *mut dynamic_sa_ctl,
    pub iv_nonce: __le32,
    pub sa_len: u32,
    pub cipher: *mut crypto_sync_skcipher,
    pub aead: *mut crypto_aead,
    pub sw_cipher: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto4xx_aead_reqctx {
    pub dst: [scatterlist; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto4xx_alg_common {
    pub type: u32,
    pub cipher: skcipher_alg,
    pub aead: aead_alg,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto4xx_alg {
    pub entry: list_head,
    pub alg: crypto4xx_alg_common,
    pub dev: *mut crypto4xx_device,
}

// Macro flag: #define BUILD_PD_ACCESS

extern "C" {
    pub fn crypto4xx_alloc_sa(ctx: *mut crypto4xx_ctx, size: u32) -> c_int;
}
extern "C" {
    pub fn crypto4xx_free_sa(ctx: *mut crypto4xx_ctx);
}
extern "C" {
    pub fn crypto4xx_encrypt_ctr(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_decrypt_ctr(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_encrypt_iv_stream(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_decrypt_iv_stream(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_encrypt_iv_block(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_decrypt_iv_block(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_encrypt_noiv_block(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_decrypt_noiv_block(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_rfc3686_encrypt(req: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_rfc3686_decrypt(req: *mut skcipher_request) -> c_int;
}
//
// Note: Only use this function to copy items that is word aligned.
//
// dst++ = __swab32p((u32 *) buf);
// dst = (tmp[2] << 16) |
// dst = (tmp[1] << 8) |
// dst = tmp[0];
extern "C" {
    pub fn crypto4xx_encrypt_aes_ccm(req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_decrypt_aes_ccm(req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_encrypt_aes_gcm(req: *mut aead_request) -> c_int;
}
extern "C" {
    pub fn crypto4xx_decrypt_aes_gcm(req: *mut aead_request) -> c_int;
}
