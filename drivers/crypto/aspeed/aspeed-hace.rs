//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/aspeed/aspeed-hace.h
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


// SPDX-License-Identifier: GPL-2.0+

//
// HACE register definitions
//
pub const ASPEED_HACE_SRC: c_uint = 0x00	/* Crypto Data Source Base Address Register */;
pub const ASPEED_HACE_DEST: c_uint = 0x04	/* Crypto Data Destination Base Address Register */;
pub const ASPEED_HACE_CONTEXT: c_uint = 0x08	/* Crypto Context Buffer Base Address Register */;
pub const ASPEED_HACE_DATA_LEN: c_uint = 0x0C	/* Crypto Data Length Register */;
pub const ASPEED_HACE_CMD: c_uint = 0x10	/* Crypto Engine Command Register */;
// G5
pub const ASPEED_HACE_TAG: c_uint = 0x18	/* HACE Tag Register */;
// G6
pub const ASPEED_HACE_GCM_ADD_LEN: c_uint = 0x14	/* Crypto AES-GCM Additional Data Length Register */;
pub const ASPEED_HACE_GCM_TAG_BASE_ADDR: c_uint = 0x18	/* Crypto AES-GCM Tag Write Buff Base Address Reg */;
pub const ASPEED_HACE_STS: c_uint = 0x1C	/* HACE Status Register */;
pub const ASPEED_HACE_HASH_SRC: c_uint = 0x20	/* Hash Data Source Base Address Register */;
pub const ASPEED_HACE_HASH_DIGEST_BUFF: c_uint = 0x24	/* Hash Digest Write Buffer Base Address Register */;
pub const ASPEED_HACE_HASH_KEY_BUFF: c_uint = 0x28	/* Hash HMAC Key Buffer Base Address Register */;
pub const ASPEED_HACE_HASH_DATA_LEN: c_uint = 0x2C	/* Hash Data Length Register */;
pub const ASPEED_HACE_HASH_CMD: c_uint = 0x30	/* Hash Engine Command Register */;
// crypto cmd
pub const HACE_CMD_SINGLE_DES: c_int = 0;

pub const HACE_CMD_AES_SELECT: c_int = 0;

// G5

// G6

// interrupt status reg

// hash cmd reg

pub const SHA_OP_UPDATE: c_int = 1;
pub const SHA_OP_FINAL: c_int = 2;

pub const ASPEED_CRYPTO_SRC_DMA_BUF_LEN: c_uint = 0xa000;
pub const ASPEED_CRYPTO_DST_DMA_BUF_LEN: c_uint = 0xa000;
pub const ASPEED_CRYPTO_GCM_TAG_OFFSET: c_uint = 0x9ff0;
pub const ASPEED_HASH_SRC_DMA_BUF_LEN: c_uint = 0xa000;
pub const ASPEED_HASH_QUEUE_LENGTH: c_int = 50;

extern "C" {
    pub fn int(: *mut *mut aspeed_hace_fn_t)(struct aspeed_hace_dev) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_sg_list {
    pub len: __le32,
    pub phy_addr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_engine_hash {
    pub done_task: tasklet_struct,
    pub flags: c_ulong,
    pub req: *mut ahash_request,
// input buffer
    pub ahash_src_addr: *mut c_void,
    pub ahash_src_dma_addr: dma_addr_t,
    pub src_dma: dma_addr_t,
    pub digest_dma: dma_addr_t,
    pub src_length: usize,
// callback func
    pub resume: aspeed_hace_fn_t,
    pub dma_prepare: aspeed_hace_fn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_sham_ctx {
    pub hace_dev: *mut aspeed_hace_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_sham_reqctx {
// DMA buffer written by hardware
    pub __aligned(64): u8 digest[SHA512_DIGEST_SIZE],
// Software state sorted by size.
    pub digcnt: [u64; 2],
    pub use*/: *mut *mut unsigned long flags; / final update flag should no,
    pub /: *mut *mut u32 cmd; / trigger cmd,
// walk state
    pub src_sg: *mut scatterlist,
    pub src_nents: c_int,
    pub /: *mut *mut unsigned int offset; / offset in current sg,
    pub /: *mut *mut unsigned int total; / per update length,
    pub digsize: usize,
    pub block_size: usize,
    pub ivsize: usize,
    pub buffer_dma_addr: dma_addr_t,
    pub digest_dma_addr: dma_addr_t,
// This is DMA too but read-only for hardware.
    pub 16]: u8 buffer[SHA512_BLOCK_SIZE +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_engine_crypto {
    pub done_task: tasklet_struct,
    pub flags: c_ulong,
    pub req: *mut skcipher_request,
// context buffer
    pub cipher_ctx: *mut c_void,
    pub cipher_ctx_dma: dma_addr_t,
// input buffer, could be single/scatter-gather lists
    pub cipher_addr: *mut c_void,
    pub cipher_dma_addr: dma_addr_t,
// output buffer, only used in scatter-gather lists
    pub dst_sg_addr: *mut c_void,
    pub dst_sg_dma_addr: dma_addr_t,
// callback func
    pub resume: aspeed_hace_fn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_cipher_ctx {
    pub hace_dev: *mut aspeed_hace_dev,
    pub key_len: c_int,
    pub key: [u8; AES_MAX_KEYLENGTH],
// callback func
    pub start: aspeed_hace_fn_t,
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_cipher_reqctx {
    pub enc_cmd: c_int,
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub /: *mut *mut skcipher_request fallback_req; / keep at the end,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_hace_dev {
    pub regs: *mut void __iomem,
    pub dev: *mut device,
    pub irq: c_int,
    pub clk: *mut clk,
    pub version: c_ulong,
    pub crypt_engine_hash: *mut crypto_engine,
    pub crypt_engine_crypto: *mut crypto_engine,
    pub hash_engine: aspeed_engine_hash,
    pub crypto_engine: aspeed_engine_crypto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_hace_alg {
    pub hace_dev: *mut aspeed_hace_dev,
    pub alg_base: *const c_char,
    pub skcipher: skcipher_engine_alg,
    pub ahash: ahash_engine_alg,
    pub alg: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aspeed_version {
    AST2500_VERSION = 5,
    AST2600_VERSION
}

extern "C" {
    pub fn aspeed_register_hace_hash_algs(hace_dev: *mut aspeed_hace_dev);
}
extern "C" {
    pub fn aspeed_unregister_hace_hash_algs(hace_dev: *mut aspeed_hace_dev);
}
extern "C" {
    pub fn aspeed_register_hace_crypto_algs(hace_dev: *mut aspeed_hace_dev);
}
extern "C" {
    pub fn aspeed_unregister_hace_crypto_algs(hace_dev: *mut aspeed_hace_dev);
}
