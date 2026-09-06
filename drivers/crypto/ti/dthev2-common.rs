//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ti/dthev2-common.h
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
// K3 DTHE V2 crypto accelerator driver
//
// Copyright (C) Texas Instruments 2025 - https://www.ti.com
// Author: T Pratham <t-pratham@ti.com>
//

pub const DTHE_REG_SIZE: c_int = 4;
pub const DTHE_DMA_TIMEOUT_MS: c_int = 2000;
//
// Size of largest possible key (of all algorithms) to be stored in dthe_tfm_ctx
// This is currently the keysize of XTS-AES-256 which is 512 bits (64 bytes)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dthe_aes_mode {
    DTHE_AES_ECB = 0,
    DTHE_AES_CBC,
    DTHE_AES_CTR,
    DTHE_AES_XTS,
    DTHE_AES_GCM,
    DTHE_AES_CCM,
}

// Driver specific struct definitions
//
// struct dthe_data - DTHE_V2 driver instance data
// @dev: Device pointer
// @regs: Base address of the register space
// @list: list node for dev
// @engine: Crypto engine instance
// @dma_aes_rx: AES Rx DMA Channel
// @dma_aes_tx: AES Tx DMA Channel
// @dma_sha_tx: SHA Tx DMA Channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dthe_data {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub list: list_head,
    pub engine: *mut crypto_engine,
    pub dma_aes_rx: *mut dma_chan,
    pub dma_aes_tx: *mut dma_chan,
    pub dma_sha_tx: *mut dma_chan,
}

//
// struct dthe_list - device data list head
// @dev_list: linked list head
// @lock: Spinlock protecting accesses to the list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dthe_list {
    pub dev_list: list_head,
    pub lock: spinlock_t,
}

//
// struct dthe_tfm_ctx - Transform ctx struct containing ctx for all sub-components of DTHE V2
// @dev_data: Device data struct pointer
// @keylen: AES key length
// @authsize: Authentication size for modes with authentication
// @key: AES key
// @aes_mode: AES mode
// @aead_fb: Fallback crypto aead handle
// @skcipher_fb: Fallback crypto skcipher handle for AES-XTS mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dthe_tfm_ctx {
    pub dev_data: *mut dthe_data,
    pub keylen: c_uint,
    pub authsize: c_uint,
    pub sizeof(u32)]: u32 key[DTHE_MAX_KEYSIZE /,
    pub aes_mode: dthe_aes_mode,
    pub aead_fb: *mut crypto_sync_aead,
    pub skcipher_fb: *mut crypto_sync_skcipher,
}

//
// struct dthe_aes_req_ctx - AES engine req ctx struct
// @enc: flag indicating encryption or decryption operation
// @padding: padding buffer for handling unaligned data
// @aes_compl: Completion variable for use in manual completion in case of DMA callback failure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dthe_aes_req_ctx {
    pub enc: c_int,
    pub AES_BLOCK_SIZE]: *mut *mut u8 padding[2,
    pub aes_compl: completion,
}

// Struct definitions end
//
// dthe_copy_sg - Copy sg entries from src to dst
// @dst: Destination sg to be filled
// @src: Source sg to be copied from
// @buflen: Number of bytes to be copied
//
// Description:
// Copy buflen bytes of data from src to dst.
//
extern "C" {
    pub fn dthe_register_aes_algs() -> c_int;
}
extern "C" {
    pub fn dthe_unregister_aes_algs();
}
