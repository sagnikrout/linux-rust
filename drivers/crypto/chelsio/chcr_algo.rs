//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/chelsio/chcr_algo.h
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


//
// This file is part of the Chelsio T6 Crypto driver for Linux.
//
// Copyright (c) 2003-2016 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Crypto key context
pub const KEY_CONTEXT_CTX_LEN_S: c_int = 24;
pub const KEY_CONTEXT_CTX_LEN_M: c_uint = 0xff;

pub const KEY_CONTEXT_DUAL_CK_S: c_int = 12;
pub const KEY_CONTEXT_DUAL_CK_M: c_uint = 0x1;

pub const KEY_CONTEXT_SALT_PRESENT_S: c_int = 10;
pub const KEY_CONTEXT_SALT_PRESENT_M: c_uint = 0x1;

pub const KEY_CONTEXT_VALID_S: c_int = 0;
pub const KEY_CONTEXT_VALID_M: c_uint = 0x1;

pub const KEY_CONTEXT_CK_SIZE_S: c_int = 6;
pub const KEY_CONTEXT_CK_SIZE_M: c_uint = 0xf;

pub const KEY_CONTEXT_MK_SIZE_S: c_int = 2;
pub const KEY_CONTEXT_MK_SIZE_M: c_uint = 0xf;

pub const KEY_CONTEXT_OPAD_PRESENT_S: c_int = 11;
pub const KEY_CONTEXT_OPAD_PRESENT_M: c_uint = 0x1;

pub const CHCR_HASH_MAX_DIGEST_SIZE: c_int = 64;
pub const CHCR_MAX_SHA_DIGEST_SIZE: c_int = 64;
pub const IPSEC_TRUNCATED_ICV_SIZE: c_int = 12;
pub const TLS_TRUNCATED_HMAC_SIZE: c_int = 10;
pub const CBCMAC_DIGEST_SIZE: c_int = 16;
pub const MAX_HASH_NAME: c_int = 20;
pub const SHA1_INIT_STATE_5X4B: c_int = 5;
pub const SHA256_INIT_STATE_8X4B: c_int = 8;
pub const SHA512_INIT_STATE_8X8B: c_int = 8;

pub const DUMMY_BYTES: c_int = 16;
pub const IPAD_DATA: c_uint = 0x36363636;
pub const OPAD_DATA: c_uint = 0x5c5c5c5c;
// Macro flag: #define TRANSHDR_SIZE(kctx_len)\

// Macro flag: #define HASH_TRANSHDR_SIZE(kctx_len)\

pub const MAX_NK: c_int = 8;
pub const MAX_DSGL_ENT: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct algo_param {
    pub auth_mode: c_uint,
    pub mk_size: c_uint,
    pub result_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_wr_param {
    pub alg_prm: algo_param,
    pub opad_needed: c_uint,
    pub more: c_uint,
    pub last: c_uint,
    pub kctx_len: c_uint,
    pub sg_len: c_uint,
    pub bfr_len: c_uint,
    pub hash_size: c_uint,
    pub scmd1: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipher_wr_param {
    pub req: *mut skcipher_request,
    pub iv: *mut c_char,
    pub bytes: c_int,
    pub qid: c_ushort,
}

//
// CCM defines values of 4, 6, 8, 10, 12, 14, and 16 octets,
// where they indicate the size of the integrity check value (ICV)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phys_sge_pairs {
    pub len: [__be16; 8],
    pub addr: [__be64; 8],
}

// Number of len fields(8) * size of one addr field
pub const PHYSDSGL_MAX_LEN_SIZE: c_int = 16;
// len field size + addr field size
// The AES s-transform matrix (s-box).
// (u32 *)(&bytes[0]) = w;
