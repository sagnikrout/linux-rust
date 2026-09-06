//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_hash.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).
// \file cc_hash.h
// ARM CryptoCell Hash Crypto API
//

pub const HMAC_IPAD_CONST: c_uint = 0x36363636;
pub const HMAC_OPAD_CONST: c_uint = 0x5C5C5C5C;
pub const HASH_LEN_SIZE_712: c_int = 16;
pub const HASH_LEN_SIZE_630: c_int = 8;

pub const XCBC_MAC_K1_OFFSET: c_int = 0;
pub const XCBC_MAC_K2_OFFSET: c_int = 16;
pub const XCBC_MAC_K3_OFFSET: c_int = 32;
pub const CC_EXPORT_MAGIC: c_uint = 0xC2EE1070U;
// this struct was taken from drivers/crypto/nx/nx-aes-xcbc.c and it is used
// for xcbc/cmac statesize
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aeshash_state {
    pub state: [u8; AES_BLOCK_SIZE],
    pub count: c_uint,
    pub buffer: [u8; AES_BLOCK_SIZE],
}

// ahash state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahash_req_ctx {
    pub ____cacheline_aligned: u8 buffers[2][CC_MAX_HASH_BLCK_SIZE],
    pub ____cacheline_aligned: u8 digest_result_buff[CC_MAX_HASH_DIGEST_SIZE],
    pub ____cacheline_aligned: u8 digest_buff[CC_MAX_HASH_DIGEST_SIZE],
    pub ____cacheline_aligned: u8 opad_digest_buff[CC_MAX_HASH_DIGEST_SIZE],
    pub ____cacheline_aligned: u8 digest_bytes_len[HASH_MAX_LEN_SIZE],
    pub ____cacheline_aligned: async_gen_req_ctx gen_ctx,
    pub data_dma_buf_type: cc_req_dma_buf_type,
    pub opad_digest_dma_addr: dma_addr_t,
    pub digest_buff_dma_addr: dma_addr_t,
    pub digest_bytes_len_dma_addr: dma_addr_t,
    pub digest_result_dma_addr: dma_addr_t,
    pub buf_cnt: [u32; 2],
    pub buff_index: u32,
    pub /: *mut *mut u32 xcbc_count; / count xcbc update operatations,
    pub buff_sg: [scatterlist; 2],
    pub curr_sg: *mut scatterlist,
    pub in_nents: u32,
    pub mlli_nents: u32,
    pub mlli_params: mlli_params,
}

extern "C" {
    pub fn cc_hash_alloc(drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn cc_init_hash_sram(drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn cc_hash_free(drvdata: *mut cc_drvdata) -> c_int;
}
//
// cc_digest_len_addr() - Gets the initial digest length
//
// @drvdata: Associated device driver context
// @mode: The Hash mode. Supported modes: MD5/SHA1/SHA224/SHA256/SHA384/SHA512
//
// Return:
// Returns the address of the initial digest length in SRAM
//
extern "C" {
    pub fn cc_digest_len_addr(drvdata: *mut c_void, mode: u32) -> u32;
}
//
// cc_larval_digest_addr() - Gets the address of the initial digest in SRAM
// according to the given hash mode
//
// @drvdata: Associated device driver context
// @mode: The Hash mode. Supported modes: MD5/SHA1/SHA224/SHA256/SHA384/SHA512
//
// Return:
// The address of the initial digest in SRAM
//
extern "C" {
    pub fn cc_larval_digest_addr(drvdata: *mut c_void, mode: u32) -> u32;
}
