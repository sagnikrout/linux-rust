//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/inline_crypto/ch_ktls/chcr_ktls.h
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

pub const CHCR_TCB_STATE_CLOSED: c_int = 0;
pub const CHCR_KTLS_KEY_CTX_LEN: c_int = 16;

pub const FALLBACK: c_int = 35;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ch_ktls_open_state {
    CH_KTLS_OPEN_SUCCESS = 0,
    CH_KTLS_OPEN_PENDING = 1,
    CH_KTLS_OPEN_FAILURE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ktls_info {
    pub sk: *mut sock,
    pub /: *mut *mut spinlock_t lock; / lock for pending_close,
    pub key_ctx: ktls_key_ctx,
    pub adap: *mut adapter,
    pub l2te: *mut l2t_entry,
    pub netdev: *mut net_device,
    pub completion: completion,
    pub iv: u64,
    pub record_no: u64,
    pub tid: c_int,
    pub atid: c_int,
    pub rx_qid: c_int,
    pub iv_size: u32,
    pub prev_seq: u32,
    pub prev_ack: u32,
    pub salt_size: u32,
    pub key_ctx_len: u32,
    pub scmd0_seqno_numivs: u32,
    pub scmd0_ivgen_hdrlen: u32,
    pub tcp_start_seq_number: u32,
    pub scmd0_short_seqno_numivs: u32,
    pub scmd0_short_ivgen_hdrlen: u32,
    pub prev_win: u16,
    pub tx_chan: u8,
    pub smt_idx: u8,
    pub port_id: u8,
    pub ip_family: u8,
    pub first_qset: u8,
    pub open_state: ch_ktls_open_state,
    pub pending_close: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ktls_ctx_tx {
    pub chcr_info: *mut chcr_ktls_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chcr_ktls_uld_ctx {
    pub entry: list_head,
    pub lldi: cxgb4_lld_info,
    pub tid_list: xarray,
    pub detach: bool,
}

// u_ctx is saved in adap, fetch it
extern "C" {
    pub fn int(adap: *mut *mut chcr_handler_func)(struct adapter, input: *mut c_uchar) -> typedef;
}
