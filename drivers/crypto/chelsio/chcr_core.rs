//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/chelsio/chcr_core.h
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

pub const MAX_PENDING_REQ_TO_HW: c_int = 20;
pub const CHCR_TEST_RESPONSE_TIMEOUT: c_int = 1000;

pub const PAD_ERROR_BIT: c_int = 1;

pub const MAC_ERROR_BIT: c_int = 0;

pub const MAX_SALT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _key_ctx {
    pub ctx_hdr: __be32,
    pub salt: [u8; MAX_SALT],
    pub iv_to_auth: __be64,
    pub key: [c_uchar; ],
}

pub const WQ_RETRY: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_driver_data {
    pub act_dev: list_head,
    pub inact_dev: list_head,
    pub dev_count: core::sync::atomic::AtomicI32,
    pub drv_mutex: mutex,
    pub last_dev: *mut uld_ctx,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chcr_state {
    CHCR_INIT = 0,
    CHCR_ATTACH,
    CHCR_DETACH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_wr {
    pub wreq: fw_crypto_lookaside_wr,
    pub ulptx: ulp_txpkt,
    pub sc_imm: ulptx_idata,
    pub sec_cpl: cpl_tx_sec_pdu,
    pub key_ctx: _key_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_dev {
    pub lock_chcr_dev: spinlock_t,
    pub state: chcr_state,
    pub inflight: core::sync::atomic::AtomicI32,
    pub wqretry: c_int,
    pub detach_work: delayed_work,
    pub detach_comp: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uld_ctx {
    pub entry: list_head,
    pub lldi: cxgb4_lld_info,
    pub dev: chcr_dev,
}

//
// sgl_len - calculates the size of an SGL of the given capacity
// @n: the number of SGL entries
// Calculates the number of flits needed for a scatter/gather list that
// can hold the given number of entries.
//
extern "C" {
    pub fn pci_get_drvdata(_arg: u_ctx->lldi.pdev) -> return;
}
extern "C" {
    pub fn chcr_send_wr(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn start_crypto() -> c_int;
}
extern "C" {
    pub fn stop_crypto() -> c_int;
}
