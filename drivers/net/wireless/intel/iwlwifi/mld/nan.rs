//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/nan.h
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
// Copyright (C) 2025-2026 Intel Corporation
//

// Macro flag: #define __iwl_mld_nan_h__

//
// struct iwl_mld_nan_link - struct representing a NAN link
// @chanctx: the channel context
// @active: indicates the NAN link is currently active
// @fw_id: FW link ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_nan_link {
    pub chanctx: *mut ieee80211_chanctx_conf,
    pub active: bool,
    pub fw_id: u8,
}

// Cleanup function for struct iwl_mld_nan_link, will be called in restart
extern "C" {
    pub fn iwl_mld_nan_supported(mld: *mut iwl_mld) -> bool;
}
extern "C" {
    pub fn iwl_mld_nan_get_mgmt_queue(mld: *mut iwl_mld, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mld_nan_use_nan_stations(mld: *mut iwl_mld) -> bool;
}
