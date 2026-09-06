//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/amcc/crypto4xx_sa.h
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
// This file defines the security context
// associate format.
//
pub const AES_IV_SIZE: c_int = 16;
//
// Contents of Dynamic Security Association (SA) with all possible fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dynamic_sa_contents {
    pub arc4_state_ptr:1: u32,
    pub arc4_ij_ptr:1: u32,
    pub state_ptr:1: u32,
    pub iv3:1: u32,
    pub iv2:1: u32,
    pub iv1:1: u32,
    pub iv0:1: u32,
    pub seq_num_mask3:1: u32,
    pub seq_num_mask2:1: u32,
    pub seq_num_mask1:1: u32,
    pub seq_num_mask0:1: u32,
    pub seq_num1:1: u32,
    pub seq_num0:1: u32,
    pub spi:1: u32,
    pub outer_size:5: u32,
    pub inner_size:5: u32,
    pub key_size:4: u32,
    pub cmd_size:4: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
pub const DIR_OUTBOUND: c_int = 0;
pub const DIR_INBOUND: c_int = 1;
pub const SA_OP_GROUP_BASIC: c_int = 0;
pub const SA_OPCODE_ENCRYPT: c_int = 0;
pub const SA_OPCODE_DECRYPT: c_int = 0;
pub const SA_OPCODE_ENCRYPT_HASH: c_int = 1;
pub const SA_OPCODE_HASH_DECRYPT: c_int = 1;
pub const SA_OPCODE_HASH: c_int = 3;
pub const SA_CIPHER_ALG_DES: c_int = 0;
pub const SA_CIPHER_ALG_3DES: c_int = 1;
pub const SA_CIPHER_ALG_ARC4: c_int = 2;
pub const SA_CIPHER_ALG_AES: c_int = 3;
pub const SA_CIPHER_ALG_KASUMI: c_int = 4;
pub const SA_CIPHER_ALG_NULL: c_int = 15;
pub const SA_HASH_ALG_MD5: c_int = 0;
pub const SA_HASH_ALG_SHA1: c_int = 1;
pub const SA_HASH_ALG_GHASH: c_int = 12;
pub const SA_HASH_ALG_CBC_MAC: c_int = 14;
pub const SA_HASH_ALG_NULL: c_int = 15;
pub const SA_HASH_ALG_SHA1_DIGEST_SIZE: c_int = 20;
pub const SA_LOAD_HASH_FROM_SA: c_int = 0;
pub const SA_LOAD_HASH_FROM_STATE: c_int = 2;
pub const SA_NOT_LOAD_HASH: c_int = 3;
pub const SA_LOAD_IV_FROM_SA: c_int = 0;
pub const SA_LOAD_IV_FROM_INPUT: c_int = 1;
pub const SA_LOAD_IV_FROM_STATE: c_int = 2;
pub const SA_LOAD_IV_GEN_IV: c_int = 3;
pub const SA_PAD_TYPE_CONSTANT: c_int = 2;
pub const SA_PAD_TYPE_ZERO: c_int = 3;
pub const SA_PAD_TYPE_TLS: c_int = 5;
pub const SA_PAD_TYPE_DTLS: c_int = 5;
pub const SA_NOT_SAVE_HASH: c_int = 0;
pub const SA_SAVE_HASH: c_int = 1;
pub const SA_NOT_SAVE_IV: c_int = 0;
pub const SA_SAVE_IV: c_int = 1;
pub const SA_HEADER_PROC: c_int = 1;
pub const SA_NO_HEADER_PROC: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sa_command_0 {
    pub scatter:1: u32,
    pub gather:1: u32,
    pub save_hash_state:1: u32,
    pub save_iv:1: u32,
    pub load_hash_state:2: u32,
    pub load_iv:2: u32,
    pub digest_len:4: u32,
    pub hdr_proc:1: u32,
    pub extend_pad:1: u32,
    pub stream_cipher_pad:1: u32,
    pub rsv:1: u32,
    pub hash_alg:4: u32,
    pub cipher_alg:4: u32,
    pub pad_type:2: u32,
    pub op_group:2: u32,
    pub dir:1: u32,
    pub opcode:3: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
pub const CRYPTO_MODE_ECB: c_int = 0;
pub const CRYPTO_MODE_CBC: c_int = 1;
pub const CRYPTO_MODE_OFB: c_int = 2;
pub const CRYPTO_MODE_CFB: c_int = 3;
pub const CRYPTO_MODE_CTR: c_int = 4;
pub const CRYPTO_FEEDBACK_MODE_NO_FB: c_int = 0;
pub const CRYPTO_FEEDBACK_MODE_64BIT_OFB: c_int = 0;
pub const CRYPTO_FEEDBACK_MODE_8BIT_CFB: c_int = 1;
pub const CRYPTO_FEEDBACK_MODE_1BIT_CFB: c_int = 2;
pub const CRYPTO_FEEDBACK_MODE_128BIT_CFB: c_int = 3;
pub const SA_AES_KEY_LEN_128: c_int = 2;
pub const SA_AES_KEY_LEN_192: c_int = 3;
pub const SA_AES_KEY_LEN_256: c_int = 4;
pub const SA_REV2: c_int = 1;
//
// The follow defines bits sa_command_1
// In Basic hash mode  this bit define simple hash or hmac.
// In IPsec mode, this bit define muting control.
//
pub const SA_HASH_MODE_HASH: c_int = 0;
pub const SA_HASH_MODE_HMAC: c_int = 1;
pub const SA_MC_ENABLE: c_int = 0;
pub const SA_MC_DISABLE: c_int = 1;
pub const SA_NOT_COPY_HDR: c_int = 0;
pub const SA_COPY_HDR: c_int = 1;
pub const SA_NOT_COPY_PAD: c_int = 0;
pub const SA_COPY_PAD: c_int = 1;
pub const SA_NOT_COPY_PAYLOAD: c_int = 0;
pub const SA_COPY_PAYLOAD: c_int = 1;
pub const SA_EXTENDED_SN_OFF: c_int = 0;
pub const SA_EXTENDED_SN_ON: c_int = 1;
pub const SA_SEQ_MASK_OFF: c_int = 0;
pub const SA_SEQ_MASK_ON: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sa_command_1 {
    pub crypto_mode31:1: u32,
    pub save_arc4_state:1: u32,
    pub arc4_stateful:1: u32,
    pub key_len:5: u32,
    pub hash_crypto_offset:8: u32,
    pub sa_rev:2: u32,
    pub byte_offset:1: u32,
    pub hmac_muting:1: u32,
    pub feedback_mode:2: u32,
    pub crypto_mode9_8:2: u32,
    pub extended_seq_num:1: u32,
    pub seq_num_mask:1: u32,
    pub mutable_bit_proc:1: u32,
    pub ip_version:1: u32,
    pub copy_pad:1: u32,
    pub copy_payload:1: u32,
    pub copy_hdr:1: u32,
    pub rsv1:1: u32,
    pub bf: },
    pub w: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_sa_ctl {
    pub sa_contents: dynamic_sa_contents,
    pub sa_command_0: sa_command_0,
    pub sa_command_1: sa_command_1,
    pub __attribute__((packed)): },
//
// State Record for Security Association (SA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_state_record {
    pub save_iv: [__le32; 4],
    pub save_hash_byte_cnt: [__le32; 2],
    pub /: *mut *mut u32 save_digest[16]; / for MD5/SHA,
    pub /: *mut *mut __le32 save_digest_le32[16]; / GHASH / CBC,
}

//
// Security Association (SA) for AES128
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_sa_aes128 {
    pub ctrl: dynamic_sa_ctl,
    pub key: [__le32; 4],
    pub /: *mut *mut __le32 iv[4]; / for CBC, OFC, and CFB mode,
    pub state_ptr: u32,
    pub reserved: u32,
    pub __attribute__((packed)): },

pub const SA_AES128_CONTENTS: c_uint = 0x3e000042;
//
// Security Association (SA) for AES192
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_sa_aes192 {
    pub ctrl: dynamic_sa_ctl,
    pub key: [__le32; 6],
    pub /: *mut *mut __le32 iv[4]; / for CBC, OFC, and CFB mode,
    pub state_ptr: u32,
    pub reserved: u32,
    pub __attribute__((packed)): },

pub const SA_AES192_CONTENTS: c_uint = 0x3e000062;
//
// Security Association (SA) for AES256
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_sa_aes256 {
    pub ctrl: dynamic_sa_ctl,
    pub key: [__le32; 8],
    pub /: *mut *mut __le32 iv[4]; / for CBC, OFC, and CFB mode,
    pub state_ptr: u32,
    pub reserved: u32,
    pub __attribute__((packed)): },

pub const SA_AES256_CONTENTS: c_uint = 0x3e000082;
pub const SA_AES_CONTENTS: c_uint = 0x3e000002;
//
// Security Association (SA) for AES128 CCM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_sa_aes128_ccm {
    pub ctrl: dynamic_sa_ctl,
    pub key: [__le32; 4],
    pub iv: [__le32; 4],
    pub state_ptr: u32,
    pub reserved: u32,
    pub __packed: },

pub const SA_AES128_CCM_CONTENTS: c_uint = 0x3e000042;
pub const SA_AES_CCM_CONTENTS: c_uint = 0x3e000002;
//
// Security Association (SA) for AES128_GCM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_sa_aes128_gcm {
    pub ctrl: dynamic_sa_ctl,
    pub key: [__le32; 4],
    pub inner_digest: [__le32; 4],
    pub iv: [__le32; 4],
    pub state_ptr: u32,
    pub reserved: u32,
    pub __packed: },

pub const SA_AES128_GCM_CONTENTS: c_uint = 0x3e000442;
pub const SA_AES_GCM_CONTENTS: c_uint = 0x3e000402;
//
// Security Association (SA) for HASH160: HMAC-SHA1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_sa_hash160 {
    pub ctrl: dynamic_sa_ctl,
    pub inner_digest: [__le32; 5],
    pub outer_digest: [__le32; 5],
    pub state_ptr: u32,
    pub reserved: u32,
    pub __attribute__((packed)): },

pub const SA_HASH160_CONTENTS: c_uint = 0x2000a502;
    pub offset: u32,
    pub cts->sa_contents.bf.iv3: +,
    pub 4: *mut *mut return sizeof(struct dynamic_sa_ctl) + offset,
    pub dynamic_sa_ctl)): *mut *mut return (__le32 ) ((unsigned long)cts + sizeof(struct,
    pub 4): *mut *mut cts->sa_contents.bf.key_size,
