//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx/otx_cptvf_algs.h
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
// Marvell OcteonTX CPT driver
//
// Copyright (C) 2019 Marvell International Ltd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

pub const OTX_CPT_MAX_ENC_KEY_SIZE: c_int = 32;
pub const OTX_CPT_MAX_HASH_KEY_SIZE: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_request_type {
    OTX_CPT_ENC_DEC_REQ            = 0x1,
    OTX_CPT_AEAD_ENC_DEC_REQ       = 0x2,
    OTX_CPT_AEAD_ENC_DEC_NULL_REQ  = 0x3,
    OTX_CPT_PASSTHROUGH_REQ	       = 0x4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_major_opcodes {
    OTX_CPT_MAJOR_OP_MISC = 0x01,
    OTX_CPT_MAJOR_OP_FC   = 0x33,
    OTX_CPT_MAJOR_OP_HMAC = 0x35,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_req_type {
    OTX_CPT_AE_CORE_REQ,
    OTX_CPT_SE_CORE_REQ
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_cipher_type {
    OTX_CPT_CIPHER_NULL = 0x0,
    OTX_CPT_DES3_CBC = 0x1,
    OTX_CPT_DES3_ECB = 0x2,
    OTX_CPT_AES_CBC  = 0x3,
    OTX_CPT_AES_ECB  = 0x4,
    OTX_CPT_AES_CFB  = 0x5,
    OTX_CPT_AES_CTR  = 0x6,
    OTX_CPT_AES_GCM  = 0x7,
    OTX_CPT_AES_XTS  = 0x8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_mac_type {
    OTX_CPT_MAC_NULL = 0x0,
    OTX_CPT_MD5      = 0x1,
    OTX_CPT_SHA1     = 0x2,
    OTX_CPT_SHA224   = 0x3,
    OTX_CPT_SHA256   = 0x4,
    OTX_CPT_SHA384   = 0x5,
    OTX_CPT_SHA512   = 0x6,
    OTX_CPT_GMAC     = 0x7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_aes_key_len {
    OTX_CPT_AES_128_BIT = 0x1,
    OTX_CPT_AES_192_BIT = 0x2,
    OTX_CPT_AES_256_BIT = 0x3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_encr_ctrl {
    pub flags: __be64,
    pub cflags: u64,

    pub enc_cipher:4: u64,
    pub reserved1:1: u64,
    pub aes_key:2: u64,
    pub iv_source:1: u64,
    pub mac_type:4: u64,
    pub reserved2:3: u64,
    pub auth_input_type:1: u64,
    pub mac_len:8: u64,
    pub reserved3:8: u64,
    pub encr_offset:16: u64,
    pub iv_offset:8: u64,
    pub auth_offset:8: u64,

    pub auth_offset:8: u64,
    pub iv_offset:8: u64,
    pub encr_offset:16: u64,
    pub reserved3:8: u64,
    pub mac_len:8: u64,
    pub auth_input_type:1: u64,
    pub reserved2:3: u64,
    pub mac_type:4: u64,
    pub iv_source:1: u64,
    pub aes_key:2: u64,
    pub reserved1:1: u64,
    pub enc_cipher:4: u64,

    pub e: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_cipher {
    pub name: *const c_char,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_enc_context {
    pub enc_ctrl: otx_cpt_encr_ctrl,
    pub encr_key: [u8; 32],
    pub encr_iv: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_fchmac_ctx {
    pub ipad: [u8; 64],
    pub opad: [u8; 64],
    pub e: },
    pub /: *mut *mut u8 hmac_calc[64]; / HMAC calculated,
    pub /: *mut *mut u8 hmac_recv[64]; / HMAC received,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_fc_ctx {
    pub enc: otx_cpt_enc_context,
    pub hmac: otx_cpt_fchmac_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_enc_ctx {
    pub key_len: u32,
    pub enc_key: [u8; OTX_CPT_MAX_KEY_SIZE],
    pub cipher_type: u8,
    pub key_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_des3_ctx {
    pub key_len: u32,
    pub des3_key: [u8; OTX_CPT_MAX_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_offset_ctrl_word {
    pub flags: __be64,
    pub cflags: u64,

    pub reserved:32: u64,
    pub enc_data_offset:16: u64,
    pub iv_offset:8: u64,
    pub auth_offset:8: u64,

    pub auth_offset:8: u64,
    pub iv_offset:8: u64,
    pub enc_data_offset:16: u64,
    pub reserved:32: u64,

    pub e: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_req_ctx {
    pub cpt_req: otx_cpt_req_info,
    pub ctrl_word: otx_cpt_offset_ctrl_word,
    pub fctx: otx_cpt_fc_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_sdesc {
    pub shash: shash_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_aead_ctx {
    pub key: [u8; OTX_CPT_MAX_KEY_SIZE],
    pub hashalg: *mut crypto_shash,
    pub sdesc: *mut otx_cpt_sdesc,
    pub ipad: *mut u8,
    pub opad: *mut u8,
    pub enc_key_len: u32,
    pub auth_key_len: u32,
    pub cipher_type: u8,
    pub mac_type: u8,
    pub key_type: u8,
    pub is_trunc_hmac: u8,
}
