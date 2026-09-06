//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/inline_crypto/ch_ktls/chcr_common.h
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
// Copyright (C) 2020 Chelsio Communications.  All rights reserved.

pub const CHCR_MAX_SALT: c_int = 4;
pub const CHCR_KEYCTX_MAC_KEY_SIZE_128: c_int = 0;
pub const CHCR_KEYCTX_CIPHER_KEY_SIZE_128: c_int = 0;
pub const CHCR_SCMD_CIPHER_MODE_AES_GCM: c_int = 2;
pub const CHCR_SCMD_CIPHER_MODE_AES_CTR: c_int = 3;
pub const CHCR_CPL_TX_SEC_PDU_LEN_64BIT: c_int = 2;
pub const CHCR_SCMD_SEQ_NO_CTRL_64BIT: c_int = 3;
pub const CHCR_SCMD_PROTO_VERSION_TLS: c_int = 0;
pub const CHCR_SCMD_PROTO_VERSION_GENERIC: c_int = 4;
pub const CHCR_SCMD_AUTH_MODE_GHASH: c_int = 4;
pub const AES_BLOCK_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktls_key_ctx {
    pub ctx_hdr: __be32,
    pub salt: [u8; CHCR_MAX_SALT],
    pub iv_to_auth: __be64,
}

// Crypto key context
pub const KEY_CONTEXT_CTX_LEN_S: c_int = 24;

pub const KEY_CONTEXT_SALT_PRESENT_S: c_int = 10;

pub const KEY_CONTEXT_VALID_S: c_int = 0;

pub const KEY_CONTEXT_CK_SIZE_S: c_int = 6;

pub const KEY_CONTEXT_MK_SIZE_S: c_int = 2;

pub const KEY_CONTEXT_OPAD_PRESENT_S: c_int = 11;

// 0-pad to multiple of 16
// p = 0;
extern "C" {
    pub fn DIV_ROUND_UP(_arg: n, _arg: 8) -> return;
}
