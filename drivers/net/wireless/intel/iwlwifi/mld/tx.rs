//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/tx.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024 Intel Corporation
//

// Macro flag: #define __iwl_mld_tx_h__

pub const IWL_MLD_INVALID_QUEUE: c_uint = 0xFFFF;
pub const IWL_MLD_INVALID_DROP_TX: c_uint = 0xFFFE;
//
// struct iwl_mld_txq - TX Queue data
//
// @fw_id: the fw id of this txq. Only valid when &status.allocated is true.
// @status: bitmap of the txq status
// @status.allocated: Indicates that the queue was allocated.
// @status.stop_full: Indicates that the queue is full and should stop TXing.
// @list: list pointer, for &mld::txqs_to_add
// @tx_request: makes sure that if there are multiple threads that want to tx
// from this txq, only one of them will do all the TXing.
// This is needed to avoid spinning the trans txq lock, which is expensive
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_txq {
// Add here fields that need clean up on restart
    pub fw_id: u16,
    pub allocated:1: u8,
    pub stop_full:1: u8,
    pub status: },
    pub list: list_head,
    pub tx_request: core::sync::atomic::AtomicI32,
// And here fields that survive a fw restart
}

extern "C" {
    pub fn iwl_mld_add_txqs_wk(wiphy: *mut wiphy, wk: *mut wiphy_work);
}
extern "C" {
    pub fn iwl_mld_remove_txq(mld: *mut iwl_mld, txq: *mut ieee80211_txq);
}
extern "C" {
    pub fn iwl_mld_add_txq_list(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_tx_from_txq(mld: *mut iwl_mld, txq: *mut ieee80211_txq);
}
extern "C" {
    pub fn iwl_mld_flush_link_sta_txqs(mld: *mut iwl_mld, fw_sta_id: u32) -> c_int;
}
extern "C" {
    pub fn iwl_mld_ensure_queue(mld: *mut iwl_mld, txq: *mut ieee80211_txq) -> c_int;
}
extern "C" {
    pub fn iwl_mld_toggle_tx_ant(mld: *mut iwl_mld, ant: *mut u8);
}
