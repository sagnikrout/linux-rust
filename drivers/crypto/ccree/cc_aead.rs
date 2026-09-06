//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_aead.h
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
// \file cc_aead.h
// ARM CryptoCell AEAD Crypto API
//

// mac_cmp - HW writes 8 B but all bytes hold the same value
pub const ICV_CMP_SIZE: c_int = 8;

// defines for AES GCM configuration buffer
pub const GCM_BLOCK_LEN_SIZE: c_int = 8;
pub const GCM_BLOCK_RFC4_IV_OFFSET: c_int = 4;

pub const GCM_BLOCK_RFC4_NONCE_OFFSET: c_int = 0;
pub const GCM_BLOCK_RFC4_NONCE_SIZE: c_int = 4;
// Offsets into AES CCM configuration buffer
pub const CCM_B0_OFFSET: c_int = 0;
pub const CCM_A0_OFFSET: c_int = 16;
pub const CCM_CTR_COUNT_0_OFFSET: c_int = 32;
// CCM B0 and CTR_COUNT constants.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aead_ccm_header_size {
    ccm_header_size_null = -1,
    ccm_header_size_zero = 0,
    ccm_header_size_2 = 2,
    ccm_header_size_6 = 6,
    ccm_header_size_max = S32_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_req_ctx {
// Allocate cache line although only 4 bytes are needed to
// assure next field falls @ cache line
// Used for both: digest HW compare and CCM/GCM MAC value
//
    pub ____cacheline_aligned: u8 mac_buf[MAX_MAC_SIZE],
    pub ____cacheline_aligned: u8 ctr_iv[AES_BLOCK_SIZE],
// used in gcm
    pub ____cacheline_aligned: u8 gcm_iv_inc1[AES_BLOCK_SIZE],
    pub ____cacheline_aligned: u8 gcm_iv_inc2[AES_BLOCK_SIZE],
    pub ____cacheline_aligned: u8 hkey[AES_BLOCK_SIZE],
    pub ____cacheline_aligned: u8 len_a[GCM_BLOCK_LEN_SIZE],
    pub len_c: [u8; GCM_BLOCK_LEN_SIZE],
    pub gcm_len_block: },
    pub ____cacheline_aligned: u8 ccm_config[CCM_CONFIG_BUF_SIZE],
// HW actual size input
    pub ____cacheline_aligned: unsigned int hw_iv_size,
// used to prevent cache coherence problem
    pub backup_mac: [u8; MAX_MAC_SIZE],
    pub /: *mut *mut *mut u8 backup_iv; / store orig iv,
    pub /: *mut *mut u32 assoclen; / size of AAD buffer to authenticate,
    pub /: *mut *mut dma_addr_t mac_buf_dma_addr; / internal ICV DMA buffer,
// buffer for internal ccm configurations
    pub ccm_iv0_dma_addr: dma_addr_t,
    pub /: *mut *mut dma_addr_t icv_dma_addr; / Phys. address of ICV,
// used in gcm
// buffer for internal gcm configurations
    pub gcm_iv_inc1_dma_addr: dma_addr_t,
// buffer for internal gcm configurations
    pub gcm_iv_inc2_dma_addr: dma_addr_t,
    pub /: *mut *mut dma_addr_t hkey_dma_addr; / Phys. address of hkey,
    pub /: *mut *mut dma_addr_t gcm_block_len_dma_addr; / Phys. address of gcm block len,
    pub /: *mut *mut *mut u8 icv_virt_addr; / Virt. address of ICV,
    pub gen_ctx: async_gen_req_ctx,
    pub assoc: cc_mlli,
    pub src: cc_mlli,
    pub dst: cc_mlli,
    pub src_sgl: *mut scatterlist,
    pub dst_sgl: *mut scatterlist,
    pub src_offset: c_uint,
    pub dst_offset: c_uint,
    pub assoc_buff_type: cc_req_dma_buf_type,
    pub data_buff_type: cc_req_dma_buf_type,
    pub mlli_params: mlli_params,
    pub cryptlen: c_uint,
    pub ccm_adata_sg: scatterlist,
    pub ccm_hdr_size: aead_ccm_header_size,
    pub req_authsize: c_uint,
    pub cipher_mode: drv_cipher_mode,
    pub is_icv_fragmented: bool,
    pub is_single_pass: bool,
    pub gcm_rfc4543: bool plaintext_authenticate_only; //for,
}

extern "C" {
    pub fn cc_aead_alloc(drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn cc_aead_free(drvdata: *mut cc_drvdata) -> c_int;
}
