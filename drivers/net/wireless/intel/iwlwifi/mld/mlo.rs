//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/mlo.h
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
// Copyright (C) 2024-2025 Intel Corporation
//

// Macro flag: #define __iwl_mld_mlo_h__

extern "C" {
    pub fn iwl_mld_emlsr_prevent_done_wk(wiphy: *mut wiphy, wk: *mut wiphy_work);
}
// Set on phy context activation, so should be a good proxy
// We only track/permit EMLSR state once authorized
// No EMLSR on dual radio devices
// For now, do not accept more links on other interface types
// In AP mode, there is no primary link
extern "C" {
    pub fn __ffs(_arg: vif->active_links) -> return;
}
extern "C" {
    pub fn __ffs(_arg: vif->active_links) -> return;
}
//
// For non-MLO/single link, this will return the deflink/single active link,
// respectively
//
extern "C" {
    pub fn __ffs(_arg: vif->active_links) -> return;
}
extern "C" {
    pub fn __ffs(~BIT(link_id): vif->active_links &) -> return;
}
// EMLSR block/unblock and exit
extern "C" {
    pub fn iwl_mld_emlsr_check_tpt(wiphy: *mut wiphy, wk: *mut wiphy_work);
}
extern "C" {
    pub fn iwl_mld_emlsr_unblock_tpt_wk(wiphy: *mut wiphy, wk: *mut wiphy_work);
}
extern "C" {
    pub fn iwl_mld_select_links(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_emlsr_check_bt(mld: *mut iwl_mld);
}
//
// iwl_mld_retry_emlsr - Retry entering EMLSR
// @mld: MLD context
// @vif: VIF to retry EMLSR on
//
// Retry entering EMLSR on the given VIF.
// Use this if one of the parameters that can prevent EMLSR has changed.
//
extern "C" {
    pub fn iwl_mld_retry_emlsr(mld: *mut iwl_mld, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mld_emlsr_check_nan_block(mld: *mut iwl_mld, vif: *mut ieee80211_vif) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_link_sel_data {
    pub link_id: u8,
    pub chandef: *const cfg80211_chan_def,
    pub signal: i32,
    pub grade: u16,
}

extern "C" {
    pub fn iwl_mld_emlsr_block_tmp_non_bss(mld: *mut iwl_mld);
}

extern "C" {
    pub fn iwl_mld_start_ignoring_tpt_updates(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_stop_ignoring_tpt_updates(mld: *mut iwl_mld);
}
