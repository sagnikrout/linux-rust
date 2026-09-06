//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/tegra/tegra-se.h
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
// SPDX-FileCopyrightText: Copyright (c) 2023 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//
// Header file for NVIDIA Security Engine driver.
//

pub const SE_OWNERSHIP: c_uint = 0x14;

pub const TEGRA_GPSE_ID: c_int = 3;
pub const SE_STREAM_ID: c_uint = 0x90;
pub const SE_SHA_CFG: c_uint = 0x4004;
pub const SE_SHA_IN_ADDR: c_uint = 0x400c;
pub const SE_SHA_KEY_ADDR: c_uint = 0x4094;
pub const SE_SHA_KEY_DATA: c_uint = 0x4098;
pub const SE_SHA_KEYMANIFEST: c_uint = 0x409c;
pub const SE_SHA_CRYPTO_CFG: c_uint = 0x40a4;
pub const SE_SHA_KEY_DST: c_uint = 0x40a8;
pub const SE_SHA_SRC_KSLT: c_uint = 0x4180;
pub const SE_SHA_TGT_KSLT: c_uint = 0x4184;
pub const SE_SHA_MSG_LENGTH: c_uint = 0x401c;
pub const SE_SHA_OPERATION: c_uint = 0x407c;
pub const SE_SHA_HASH_RESULT: c_uint = 0x40b0;

// AES Configuration
pub const SE_AES0_CFG: c_uint = 0x1004;
pub const SE_AES0_CRYPTO_CONFIG: c_uint = 0x1008;
pub const SE_AES0_KEY_DST: c_uint = 0x1030;
pub const SE_AES0_OPERATION: c_uint = 0x1038;
pub const SE_AES0_LINEAR_CTR: c_uint = 0x101c;
pub const SE_AES0_LAST_BLOCK: c_uint = 0x102c;
pub const SE_AES0_KEY_ADDR: c_uint = 0x10bc;
pub const SE_AES0_KEY_DATA: c_uint = 0x10c0;
pub const SE_AES0_CMAC_RESULT: c_uint = 0x10c4;
pub const SE_AES0_SRC_KSLT: c_uint = 0x1100;
pub const SE_AES0_TGT_KSLT: c_uint = 0x1104;
pub const SE_AES0_KEYMANIFEST: c_uint = 0x1114;
pub const SE_AES0_AAD_LEN: c_uint = 0x112c;
pub const SE_AES0_CRYPTO_MSG_LEN: c_uint = 0x1134;
pub const SE_AES1_CFG: c_uint = 0x2004;
pub const SE_AES1_CRYPTO_CONFIG: c_uint = 0x2008;
pub const SE_AES1_KEY_DST: c_uint = 0x2030;
pub const SE_AES1_OPERATION: c_uint = 0x2038;
pub const SE_AES1_LINEAR_CTR: c_uint = 0x201c;
pub const SE_AES1_LAST_BLOCK: c_uint = 0x202c;
pub const SE_AES1_KEY_ADDR: c_uint = 0x20bc;
pub const SE_AES1_KEY_DATA: c_uint = 0x20c0;
pub const SE_AES1_CMAC_RESULT: c_uint = 0x20c4;
pub const SE_AES1_SRC_KSLT: c_uint = 0x2100;
pub const SE_AES1_TGT_KSLT: c_uint = 0x2104;
pub const SE_AES1_KEYMANIFEST: c_uint = 0x2114;
pub const SE_AES1_AAD_LEN: c_uint = 0x212c;
pub const SE_AES1_CRYPTO_MSG_LEN: c_uint = 0x2134;

// AES Crypto Configuration

pub const HASH_RESULT_REG_COUNT: c_int = 50;
pub const CMAC_RESULT_REG_COUNT: c_int = 4;
pub const SE_CRYPTO_CTR_REG_COUNT: c_int = 4;
pub const SE_MAX_KEYSLOT: c_int = 15;

pub const TEGRA_AES_RESERVED_KSLT: c_int = 14;
pub const TEGRA_XTS_RESERVED_KSLT: c_int = 15;

// Security Engine operation modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum se_aes_alg {
    SE_ALG_CBC,		/* Cipher Block Chaining (CBC) mode */
    SE_ALG_ECB,		/* Electronic Codebook (ECB) mode */
    SE_ALG_CTR,		/* Counter (CTR) mode */
    SE_ALG_XTS,		/* XTS mode */
    SE_ALG_GMAC,		/* GMAC mode */
    SE_ALG_GCM,		/* GCM mode */
    SE_ALG_GCM_FINAL,	/* GCM FINAL mode */
    SE_ALG_CMAC,	/* Cipher-based MAC (CMAC) mode */
    SE_ALG_CBC_MAC,	/* CBC MAC mode */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum se_hash_alg {
    SE_ALG_RNG_DRBG,	/* Deterministic Random Bit Generator */
    SE_ALG_SHA1,		/* Secure Hash Algorithm-1 (SHA1) mode */
    SE_ALG_SHA224,		/* Secure Hash Algorithm-224  (SHA224) mode */
    SE_ALG_SHA256,		/* Secure Hash Algorithm-256  (SHA256) mode */
    SE_ALG_SHA384,		/* Secure Hash Algorithm-384  (SHA384) mode */
    SE_ALG_SHA512,		/* Secure Hash Algorithm-512  (SHA512) mode */
    SE_ALG_SHA3_224,	/* Secure Hash Algorithm3-224 (SHA3-224) mode */
    SE_ALG_SHA3_256,	/* Secure Hash Algorithm3-256 (SHA3-256) mode */
    SE_ALG_SHA3_384,	/* Secure Hash Algorithm3-384 (SHA3-384) mode */
    SE_ALG_SHA3_512,	/* Secure Hash Algorithm3-512 (SHA3-512) mode */
    SE_ALG_SHAKE128,	/* Secure Hash Algorithm3 (SHAKE128) mode */
    SE_ALG_SHAKE256,	/* Secure Hash Algorithm3 (SHAKE256) mode */
    SE_ALG_HMAC_SHA224,	/* Hash based MAC (HMAC) - 224 */
    SE_ALG_HMAC_SHA256,	/* Hash based MAC (HMAC) - 256 */
    SE_ALG_HMAC_SHA384,	/* Hash based MAC (HMAC) - 384 */
    SE_ALG_HMAC_SHA512,	/* Hash based MAC (HMAC) - 512 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_se_alg {
    pub se_dev: *mut tegra_se,
    pub alg_base: *const c_char,
    pub skcipher: skcipher_engine_alg,
    pub aead: aead_engine_alg,
    pub ahash: ahash_engine_alg,
    pub alg: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_se_regs {
    pub op: u32,
    pub config: u32,
    pub last_blk: u32,
    pub linear_ctr: u32,
    pub out_addr: u32,
    pub aad_len: u32,
    pub cryp_msg_len: u32,
    pub manifest: u32,
    pub key_addr: u32,
    pub key_data: u32,
    pub key_dst: u32,
    pub result: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_se_hw {
    pub regs: *const tegra_se_regs,
    pub se): *mut *mut int (init_alg)(struct tegra_se,
    pub se): *mut *mut void (deinit_alg)(struct tegra_se,
    pub support_sm_alg: bool,
    pub host1x_class: u32,
    pub kac_ver: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_se {
    pub keylen): *mut *mut int (manifest)(u32 user, u32 alg, u32,
    pub hw: *const tegra_se_hw,
    pub client: host1x_client,
    pub channel: *mut host1x_channel,
    pub cmdbuf: *mut tegra_se_cmdbuf,
    pub keybuf: *mut tegra_se_cmdbuf,
    pub engine: *mut crypto_engine,
    pub syncpt: *mut host1x_syncpt,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub opcode_addr: c_uint,
    pub stream_id: c_uint,
    pub syncpt_id: c_uint,
    pub base: *mut void __iomem,
    pub owner: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_se_cmdbuf {
    pub iova: dma_addr_t,
    pub addr: *mut u32,
    pub dev: *mut device,
    pub ref: kref,
    pub bo: host1x_bo,
    pub size: isize,
    pub words: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_se_datbuf {
    pub buf: *mut u8,
    pub addr: dma_addr_t,
    pub size: isize,
}

// Functions
extern "C" {
    pub fn tegra_init_aes(se: *mut tegra_se) -> c_int;
}
extern "C" {
    pub fn tegra_init_hash(se: *mut tegra_se) -> c_int;
}
extern "C" {
    pub fn tegra_deinit_aes(se: *mut tegra_se);
}
extern "C" {
    pub fn tegra_deinit_hash(se: *mut tegra_se);
}
extern "C" {
    pub fn tegra_key_invalidate(se: *mut tegra_se, keyid: u32, alg: u32);
}
extern "C" {
    pub fn tegra_key_invalidate_reserved(se: *mut tegra_se, keyid: u32, alg: u32);
}
extern "C" {
    pub fn tegra_se_host1x_submit(se: *mut tegra_se, cmdbuf: *mut tegra_se_cmdbuf, size: u32) -> c_int;
}
// keyid = TEGRA_AES_RESERVED_KSLT;
extern "C" {
    pub fn tegra_key_submit_reserved(_arg: se, _arg: key, _arg: keylen, _arg: alg, _arg: keyid) -> return;
}
// keyid = TEGRA_XTS_RESERVED_KSLT;
extern "C" {
    pub fn tegra_key_submit_reserved(_arg: se, _arg: key, _arg: keylen, _arg: alg, _arg: keyid) -> return;
}
// HOST1x OPCODES
// 22-bit offset supported

