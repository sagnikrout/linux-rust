//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/bcm/spu2.h
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
// This file contains SPU message definitions specific to SPU2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu2_cipher_type {
    SPU2_CIPHER_TYPE_NONE = 0x0,
    SPU2_CIPHER_TYPE_AES128 = 0x1,
    SPU2_CIPHER_TYPE_AES192 = 0x2,
    SPU2_CIPHER_TYPE_AES256 = 0x3,
    SPU2_CIPHER_TYPE_DES = 0x4,
    SPU2_CIPHER_TYPE_3DES = 0x5,
    SPU2_CIPHER_TYPE_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu2_cipher_mode {
    SPU2_CIPHER_MODE_ECB = 0x0,
    SPU2_CIPHER_MODE_CBC = 0x1,
    SPU2_CIPHER_MODE_CTR = 0x2,
    SPU2_CIPHER_MODE_CFB = 0x3,
    SPU2_CIPHER_MODE_OFB = 0x4,
    SPU2_CIPHER_MODE_XTS = 0x5,
    SPU2_CIPHER_MODE_CCM = 0x6,
    SPU2_CIPHER_MODE_GCM = 0x7,
    SPU2_CIPHER_MODE_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu2_hash_type {
    SPU2_HASH_TYPE_NONE = 0x0,
    SPU2_HASH_TYPE_AES128 = 0x1,
    SPU2_HASH_TYPE_AES192 = 0x2,
    SPU2_HASH_TYPE_AES256 = 0x3,
    SPU2_HASH_TYPE_MD5 = 0x6,
    SPU2_HASH_TYPE_SHA1 = 0x7,
    SPU2_HASH_TYPE_SHA224 = 0x8,
    SPU2_HASH_TYPE_SHA256 = 0x9,
    SPU2_HASH_TYPE_SHA384 = 0xa,
    SPU2_HASH_TYPE_SHA512 = 0xb,
    SPU2_HASH_TYPE_SHA512_224 = 0xc,
    SPU2_HASH_TYPE_SHA512_256 = 0xd,
    SPU2_HASH_TYPE_SHA3_224 = 0xe,
    SPU2_HASH_TYPE_SHA3_256 = 0xf,
    SPU2_HASH_TYPE_SHA3_384 = 0x10,
    SPU2_HASH_TYPE_SHA3_512 = 0x11,
    SPU2_HASH_TYPE_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu2_hash_mode {
    SPU2_HASH_MODE_CMAC = 0x0,
    SPU2_HASH_MODE_CBC_MAC = 0x1,
    SPU2_HASH_MODE_XCBC_MAC = 0x2,
    SPU2_HASH_MODE_HMAC = 0x3,
    SPU2_HASH_MODE_RABIN = 0x4,
    SPU2_HASH_MODE_CCM = 0x5,
    SPU2_HASH_MODE_GCM = 0x6,
    SPU2_HASH_MODE_RESERVED = 0x7,
    SPU2_HASH_MODE_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu2_ret_md_opts {
    SPU2_RET_NO_MD = 0,	/* return no metadata */
    SPU2_RET_FMD_OMD = 1,	/* return both FMD and OMD */
    SPU2_RET_FMD_ONLY = 2,	/* return only FMD */
    SPU2_RET_FMD_OMD_IV = 3,	/* return FMD and OMD with just IVs */
}

// Fixed Metadata format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SPU2_FMD {
    pub ctrl0: __le64,
    pub ctrl1: __le64,
    pub ctrl2: __le64,
    pub ctrl3: __le64,
}

// Fixed part of request message header length in bytes. Just FMD.

// FMD ctrl0 field masks
pub const SPU2_CIPH_ENCRYPT_EN: c_uint = 0x1 /* 0: decrypt, 1: encrypt */;
pub const SPU2_CIPH_TYPE: c_uint = 0xF0 /* one of spu2_cipher_type */;
pub const SPU2_CIPH_TYPE_SHIFT: c_int = 4;
pub const SPU2_CIPH_MODE: c_uint = 0xF00 /* one of spu2_cipher_mode */;
pub const SPU2_CIPH_MODE_SHIFT: c_int = 8;
pub const SPU2_CFB_MASK: c_uint = 0x7000 /* cipher feedback mask */;
pub const SPU2_CFB_MASK_SHIFT: c_int = 12;
pub const SPU2_PROTO_SEL: c_uint = 0xF00000 /* MACsec, IPsec, TLS... */;
pub const SPU2_PROTO_SEL_SHIFT: c_int = 20;
pub const SPU2_HASH_FIRST: c_uint = 0x1000000 /* 1: hash input is input pkt;
// data
//
pub const SPU2_CHK_TAG: c_uint = 0x2000000 /* 1: check digest provided */;
pub const SPU2_HASH_TYPE: c_uint = 0x1F0000000 /* one of spu2_hash_type */;
pub const SPU2_HASH_TYPE_SHIFT: c_int = 28;
pub const SPU2_HASH_MODE: c_uint = 0xF000000000 /* one of spu2_hash_mode */;
pub const SPU2_HASH_MODE_SHIFT: c_int = 36;
pub const SPU2_CIPH_PAD_EN: c_uint = 0x100000000000 /* 1: Add pad to end of payload for;
// enc
//
pub const SPU2_CIPH_PAD: c_uint = 0xFF000000000000 /* cipher pad value */;
pub const SPU2_CIPH_PAD_SHIFT: c_int = 48;
// FMD ctrl1 field masks
pub const SPU2_TAG_LOC: c_uint = 0x1 /* 1: end of payload, 0: undef */;
pub const SPU2_HAS_FR_DATA: c_uint = 0x2 /* 1: msg has frame data */;
pub const SPU2_HAS_AAD1: c_uint = 0x4 /* 1: msg has AAD1 field */;
pub const SPU2_HAS_NAAD: c_uint = 0x8 /* 1: msg has NAAD field */;
pub const SPU2_HAS_AAD2: c_uint = 0x10 /* 1: msg has AAD2 field */;
pub const SPU2_HAS_ESN: c_uint = 0x20 /* 1: msg has ESN field */;
pub const SPU2_HASH_KEY_LEN: c_uint = 0xFF00 /* len of hash key in bytes.;
// HMAC only.
//
pub const SPU2_HASH_KEY_LEN_SHIFT: c_int = 8;
pub const SPU2_CIPH_KEY_LEN: c_uint = 0xFF00000 /* len of cipher key in bytes */;
pub const SPU2_CIPH_KEY_LEN_SHIFT: c_int = 20;
pub const SPU2_GENIV: c_uint = 0x10000000 /* 1: hw generates IV */;
pub const SPU2_HASH_IV: c_uint = 0x20000000 /* 1: IV incl in hash */;
pub const SPU2_RET_IV: c_uint = 0x40000000 /* 1: return IV in output msg;
// b4 payload
//
pub const SPU2_RET_IV_LEN: c_uint = 0xF00000000 /* length in bytes of IV returned.;
// 0 = 16 bytes
//
pub const SPU2_RET_IV_LEN_SHIFT: c_int = 32;
pub const SPU2_IV_OFFSET: c_uint = 0xF000000000 /* gen IV offset */;
pub const SPU2_IV_OFFSET_SHIFT: c_int = 36;
pub const SPU2_IV_LEN: c_uint = 0x1F0000000000 /* length of input IV in bytes */;
pub const SPU2_IV_LEN_SHIFT: c_int = 40;
pub const SPU2_HASH_TAG_LEN: c_uint = 0x7F000000000000 /* hash tag length in bytes */;
pub const SPU2_HASH_TAG_LEN_SHIFT: c_int = 48;
pub const SPU2_RETURN_MD: c_uint = 0x300000000000000 /* return metadata */;
pub const SPU2_RETURN_MD_SHIFT: c_int = 56;
pub const SPU2_RETURN_FD: c_uint = 0x400000000000000;
pub const SPU2_RETURN_AAD1: c_uint = 0x800000000000000;
pub const SPU2_RETURN_NAAD: c_uint = 0x1000000000000000;
pub const SPU2_RETURN_AAD2: c_uint = 0x2000000000000000;
pub const SPU2_RETURN_PAY: c_uint = 0x4000000000000000 /* return payload */;
// FMD ctrl2 field masks
pub const SPU2_AAD1_OFFSET: c_uint = 0xFFF /* byte offset of AAD1 field */;
pub const SPU2_AAD1_LEN: c_uint = 0xFF000 /* length of AAD1 in bytes */;
pub const SPU2_AAD1_LEN_SHIFT: c_int = 12;
pub const SPU2_AAD2_OFFSET: c_uint = 0xFFF00000 /* byte offset of AAD2 field */;
pub const SPU2_AAD2_OFFSET_SHIFT: c_int = 20;
pub const SPU2_PL_OFFSET: c_uint = 0xFFFFFFFF00000000 /* payload offset from AAD2 */;
pub const SPU2_PL_OFFSET_SHIFT: c_int = 32;
// FMD ctrl3 field masks
pub const SPU2_PL_LEN: c_uint = 0xFFFFFFFF /* payload length in bytes */;
pub const SPU2_TLS_LEN: c_uint = 0xFFFF00000000 /* TLS encrypt: cipher len;
// TLS decrypt: compressed len
//
pub const SPU2_TLS_LEN_SHIFT: c_int = 32;
//
// Max value that can be represented in the Payload Length field of the
// ctrl3 word of FMD.
//

// Error values returned in STATUS field of response messages
pub const SPU2_INVALID_ICV: c_int = 1;
extern "C" {
    pub fn spu2_dump_msg_hdr(buf: *mut u8, buf_len: c_uint);
}
extern "C" {
    pub fn spu2_payload_length(spu_hdr: *mut u8) -> u32;
}
extern "C" {
    pub fn spu2_response_hdr_len(auth_key_len: u16, enc_key_len: u16, is_hash: bool) -> u16;
}
extern "C" {
    pub fn spu2_hash_type(src_sent: u32) -> hash_type;
}
extern "C" {
    pub fn spu2_cipher_req_init(spu_hdr: *mut u8, cipher_parms: *mut spu_cipher_parms) -> u16;
}
extern "C" {
    pub fn spu2_xts_tweak_in_payload() -> u8;
}
extern "C" {
    pub fn spu2_tx_status_len() -> u8;
}
extern "C" {
    pub fn spu2_rx_status_len() -> u8;
}
extern "C" {
    pub fn spu2_status_process(statp: *mut u8) -> c_int;
}
extern "C" {
    pub fn spu2_wordalign_padlen(data_size: u32) -> u32;
}
