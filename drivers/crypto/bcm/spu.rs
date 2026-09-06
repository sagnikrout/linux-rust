//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/bcm/spu.h
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
// Copyright 2016 Broadcom
//
// This file contains the definition of SPU messages. There are currently two
// SPU message formats: SPU-M and SPU2. The hardware uses different values to
// identify the same things in SPU-M vs SPU2. So this file defines values that
// are hardware independent. Software can use these values for any version of
// SPU hardware. These values are used in APIs in spu.c. Functions internal to
// spu.c and spu2.c convert these to hardware-specific values.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu_cipher_alg {
    CIPHER_ALG_NONE = 0x0,
    CIPHER_ALG_RC4 = 0x1,
    CIPHER_ALG_DES = 0x2,
    CIPHER_ALG_3DES = 0x3,
    CIPHER_ALG_AES = 0x4,
    CIPHER_ALG_LAST = 0x5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu_cipher_mode {
    CIPHER_MODE_NONE = 0x0,
    CIPHER_MODE_ECB = 0x0,
    CIPHER_MODE_CBC = 0x1,
    CIPHER_MODE_OFB = 0x2,
    CIPHER_MODE_CFB = 0x3,
    CIPHER_MODE_CTR = 0x4,
    CIPHER_MODE_CCM = 0x5,
    CIPHER_MODE_GCM = 0x6,
    CIPHER_MODE_XTS = 0x7,
    CIPHER_MODE_LAST = 0x8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu_cipher_type {
    CIPHER_TYPE_NONE = 0x0,
    CIPHER_TYPE_DES = 0x0,
    CIPHER_TYPE_3DES = 0x0,
    CIPHER_TYPE_INIT = 0x0,	/* used for ARC4 */
    CIPHER_TYPE_AES128 = 0x0,
    CIPHER_TYPE_AES192 = 0x1,
    CIPHER_TYPE_UPDT = 0x1,	/* used for ARC4 */
    CIPHER_TYPE_AES256 = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hash_alg {
    HASH_ALG_NONE = 0x0,
    HASH_ALG_MD5 = 0x1,
    HASH_ALG_SHA1 = 0x2,
    HASH_ALG_SHA224 = 0x3,
    HASH_ALG_SHA256 = 0x4,
    HASH_ALG_AES = 0x5,
    HASH_ALG_SHA384 = 0x6,
    HASH_ALG_SHA512 = 0x7,
// Keep SHA3 algorithms at the end always
    HASH_ALG_SHA3_224 = 0x8,
    HASH_ALG_SHA3_256 = 0x9,
    HASH_ALG_SHA3_384 = 0xa,
    HASH_ALG_SHA3_512 = 0xb,
    HASH_ALG_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hash_mode {
    HASH_MODE_NONE = 0x0,
    HASH_MODE_HASH = 0x0,
    HASH_MODE_XCBC = 0x0,
    HASH_MODE_CMAC = 0x1,
    HASH_MODE_CTXT = 0x1,
    HASH_MODE_HMAC = 0x2,
    HASH_MODE_RABIN = 0x4,
    HASH_MODE_FHMAC = 0x6,
    HASH_MODE_CCM = 0x5,
    HASH_MODE_GCM = 0x6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hash_type {
    HASH_TYPE_NONE = 0x0,
    HASH_TYPE_FULL = 0x0,
    HASH_TYPE_INIT = 0x1,
    HASH_TYPE_UPDT = 0x2,
    HASH_TYPE_FIN = 0x3,
    HASH_TYPE_AES128 = 0x0,
    HASH_TYPE_AES192 = 0x1,
    HASH_TYPE_AES256 = 0x2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aead_type {
    AES_CCM,
    AES_GCM,
    AUTHENC,
    AEAD_TYPE_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_request_opts {
    pub is_inbound: bool,
    pub auth_first: bool,
    pub is_aead: bool,
    pub is_esp: bool,
    pub bd_suppress: bool,
    pub is_rfc4543: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_cipher_parms {
    pub alg: spu_cipher_alg,
    pub mode: spu_cipher_mode,
    pub type: spu_cipher_type,
    pub key_buf: *mut u8,
    pub key_len: u16,
// iv_buf and iv_len include salt, if applicable
    pub iv_buf: *mut u8,
    pub iv_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_hash_parms {
    pub alg: hash_alg,
    pub mode: hash_mode,
    pub type: hash_type,
    pub digestsize: u8,
    pub key_buf: *mut u8,
    pub key_len: u16,
    pub prebuf_len: u16,
// length of hash pad. signed, needs to handle roll-overs
    pub pad_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_aead_parms {
    pub assoc_size: u32,
    pub /: *mut *mut u16 iv_len; / length of IV field between assoc data and data,
    pub /: *mut *mut u8 aad_pad_len; / For AES GCM/CCM, length of padding after AAD,
    pub /: *mut *mut u8 data_pad_len;/ For AES GCM/CCM, length of padding after data,
    pub /: *mut *mut bool return_iv; / True if SPU should return an IV,
    pub /: *mut *mut u32 ret_iv_len; / Length in bytes of returned IV,
    pub /: *mut *mut u32 ret_iv_off; / Offset into full IV if partial IV returned,
}

// SPU sizes
pub const SPU_RX_STATUS_LEN: c_int = 4;
// Max length of padding for 4-byte alignment of STATUS field
pub const SPU_STAT_PAD_MAX: c_int = 4;
// Max length of pad fragment. 4 is for 4-byte alignment of STATUS field

// GCM and CCM require 16-byte alignment
pub const SPU_GCM_CCM_ALIGN: c_int = 16;
// Length up SUPDT field in SPU response message for RC4
pub const SPU_SUPDT_LEN: c_int = 260;
// SPU status error codes. These used as common error codes across all
// SPU variants.
//
pub const SPU_INVALID_ICV: c_int = 1;
// Indicates no limit to the length of the payload in a SPU message
pub const SPU_MAX_PAYLOAD_INF: c_uint = 0xFFFFFFFF;
// Size of XTS tweak ("i" parameter), in bytes
pub const SPU_XTS_TWEAK_SIZE: c_int = 16;
// CCM B_0 field definitions, common for SPU-M and SPU2
pub const CCM_B0_ADATA: c_uint = 0x40;
pub const CCM_B0_ADATA_SHIFT: c_int = 6;
pub const CCM_B0_M_PRIME: c_uint = 0x38;
pub const CCM_B0_M_PRIME_SHIFT: c_int = 3;
pub const CCM_B0_L_PRIME: c_uint = 0x07;
pub const CCM_B0_L_PRIME_SHIFT: c_int = 0;
pub const CCM_ESP_L_VALUE: c_int = 4;
//
// spu_req_incl_icv() - Return true if SPU request message should include the
// ICV as a separate buffer.
// @cipher_mode:  the cipher mode being requested
// @is_encrypt:   true if encrypting. false if decrypting.
//
// Return:  true if ICV to be included as separate buffer
//
// SPU Functions Prototypes
extern "C" {
    pub fn spum_dump_msg_hdr(buf: *mut u8, buf_len: c_uint);
}
extern "C" {
    pub fn spum_payload_length(spu_hdr: *mut u8) -> u32;
}
extern "C" {
    pub fn spum_response_hdr_len(auth_key_len: u16, enc_key_len: u16, is_hash: bool) -> u16;
}
extern "C" {
    pub fn spum_aead_ivlen(cipher_mode: spu_cipher_mode, iv_len: u16) -> u8;
}
extern "C" {
    pub fn spu_req_incl_icv(cipher_mode: spu_cipher_mode, is_encrypt: bool) -> bool;
}
extern "C" {
    pub fn spum_hash_type(src_sent: u32) -> hash_type;
}
extern "C" {
    pub fn spum_cipher_req_init(spu_hdr: *mut u8, cipher_parms: *mut spu_cipher_parms) -> u16;
}
extern "C" {
    pub fn spum_xts_tweak_in_payload() -> u8;
}
extern "C" {
    pub fn spum_tx_status_len() -> u8;
}
extern "C" {
    pub fn spum_rx_status_len() -> u8;
}
extern "C" {
    pub fn spum_status_process(statp: *mut u8) -> c_int;
}
extern "C" {
    pub fn spum_wordalign_padlen(data_size: u32) -> u32;
}
