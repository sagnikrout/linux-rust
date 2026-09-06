//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/cpt/cptvf_algs.h
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
// Copyright (C) 2016 Cavium, Inc.
//

pub const MAX_DEVICES: c_int = 16;
pub const MAJOR_OP_FC: c_uint = 0x33;
pub const MAX_ENC_KEY_SIZE: c_int = 32;
pub const MAX_HASH_KEY_SIZE: c_int = 64;

pub const CONTROL_WORD_LEN: c_int = 8;
pub const KEY2_OFFSET: c_int = 48;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum req_type {
    AE_CORE_REQ,
    SE_CORE_REQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cipher_type {
    DES3_CBC = 0x1,
    DES3_ECB = 0x2,
    AES_CBC = 0x3,
    AES_ECB = 0x4,
    AES_CFB = 0x5,
    AES_CTR = 0x6,
    AES_GCM = 0x7,
    AES_XTS = 0x8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aes_type {
    AES_128_BIT = 0x1,
    AES_192_BIT = 0x2,
    AES_256_BIT = 0x3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union encr_ctrl {
    pub flags: u64,

    pub enc_cipher:4: u64,
    pub reserved1:1: u64,
    pub aes_key:2: u64,
    pub iv_source:1: u64,
    pub hash_type:4: u64,
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
    pub hash_type:4: u64,
    pub iv_source:1: u64,
    pub aes_key:2: u64,
    pub reserved1:1: u64,
    pub enc_cipher:4: u64,

    pub e: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_cipher {
    pub name: *const c_char,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_context {
    pub enc_ctrl: encr_ctrl,
    pub encr_key: [u8; 32],
    pub encr_iv: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fchmac_context {
    pub ipad: [u8; 64],
    pub /: *mut *mut u8 opad[64]; / or OPAD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_context {
    pub enc: enc_context,
    pub hmac: fchmac_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_enc_ctx {
    pub key_len: u32,
    pub enc_key: [u8; MAX_KEY_SIZE],
    pub cipher_type:4: u8,
    pub key_type:2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_des3_ctx {
    pub key_len: u32,
    pub des3_key: [u8; MAX_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_req_ctx {
    pub cpt_req: cpt_request_info,
    pub control_word: u64,
    pub fctx: fc_context,
}

extern "C" {
    pub fn cptvf_do_request(cptvf: *mut c_void, req: *mut cpt_request_info) -> c_int;
}
