//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/inline_crypto/ch_ipsec/chcr_ipsec.h
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
// Copyright (c) 2018 Chelsio Communications, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_uld_ctx {
    pub entry: list_head,
    pub lldi: cxgb4_lld_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ipsec_req {
    pub ulptx: ulp_txpkt,
    pub sc_imm: ulptx_idata,
    pub sec_cpl: cpl_tx_sec_pdu,
    pub key_ctx: _key_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ipsec_wr {
    pub wreq: fw_ulptx_wr,
    pub req: chcr_ipsec_req,
}

pub const ESN_IV_INSERT_OFFSET: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ipsec_aadiv {
    pub spi: __be32,
    pub seq_no: [u8; 8],
    pub iv: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_sa_entry {
    pub hmac_ctrl: c_int,
    pub esn: u16,
    pub resv: u16,
    pub enckey_len: c_uint,
    pub kctx_len: c_uint,
    pub authsize: c_uint,
    pub key_ctx_hdr: __be32,
    pub salt: [c_char; MAX_SALT],
    pub AES_MAX_KEY_SIZE]: *mut *mut char key[2,
}
