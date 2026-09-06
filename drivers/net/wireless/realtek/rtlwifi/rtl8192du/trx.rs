//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192du/trx.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2024  Realtek Corporation.

pub const TX_TOTAL_PAGE_NUMBER_92DU: c_uint = 0xF8;
pub const TEST_PAGE_NUM_PUBQ_92DU: c_uint = 0x89;
pub const TX_TOTAL_PAGE_NUMBER_92D_DUAL_MAC: c_uint = 0x7A;
pub const NORMAL_PAGE_NUM_PUBQ_92D_DUAL_MAC: c_uint = 0x5A;
pub const NORMAL_PAGE_NUM_HPQ_92D_DUAL_MAC: c_uint = 0x10;
pub const NORMAL_PAGE_NUM_LPQ_92D_DUAL_MAC: c_uint = 0x10;
pub const NORMAL_PAGE_NUM_NORMALQ_92D_DUAL_MAC: c_int = 0;
pub const WMM_NORMAL_TX_TOTAL_PAGE_NUMBER: c_uint = 0xF5;
pub const WMM_NORMAL_PAGE_NUM_PUBQ_92D: c_uint = 0x65;
pub const WMM_NORMAL_PAGE_NUM_HPQ_92D: c_uint = 0x30;
pub const WMM_NORMAL_PAGE_NUM_LPQ_92D: c_uint = 0x30;
pub const WMM_NORMAL_PAGE_NUM_NPQ_92D: c_uint = 0x30;
pub const WMM_NORMAL_PAGE_NUM_PUBQ_92D_DUAL_MAC: c_uint = 0x32;
pub const WMM_NORMAL_PAGE_NUM_HPQ_92D_DUAL_MAC: c_uint = 0x18;
pub const WMM_NORMAL_PAGE_NUM_LPQ_92D_DUAL_MAC: c_uint = 0x18;
pub const WMM_NORMAL_PAGE_NUM_NPQ_92D_DUAL_MAC: c_uint = 0x18;
extern "C" {
    pub fn rtl92du_endpoint_mapping(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl92du_mq_to_hwq(fc: __le16, mac80211_queue_index: u16) -> u16;
}
extern "C" {
    pub fn rtl92du_tx_cleanup(hw: *mut ieee80211_hw, skb: *mut sk_buff);
}
