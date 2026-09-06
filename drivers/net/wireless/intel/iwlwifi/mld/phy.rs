//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/phy.h
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

// Macro flag: #define __iwl_mld_phy_h__

//
// struct iwl_mld_phy - PHY configuration parameters
//
// @fw_id: fw id of the phy.
// @chandef: the last chandef that mac80211 configured the driver
// with. Used to detect a no-op when the chanctx changes.
// @channel_load_by_us: channel load on this channel caused by
// the NIC itself, as indicated by firmware
// @avg_channel_load_not_by_us: averaged channel load on this channel caused by
// others. This value is invalid when in EMLSR (due to FW limitations)
// @mld: pointer to the MLD context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_phy {
// Add here fields that need clean up on hw restart
    pub fw_id: u8,
    pub chandef: cfg80211_chan_def,
// And here fields that survive a hw restart
    pub channel_load_by_us: u32,
    pub avg_channel_load_not_by_us: u32,
    pub mld: *mut iwl_mld,
}

// Cleanup function for struct iwl_mld_phy, will be called in restart
extern "C" {
    pub fn iwl_mld_allocate_fw_phy_id(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_get_fw_ctrl_pos(chandef: *const cfg80211_chan_def) -> u8;
}
extern "C" {
    pub fn iwl_mld_send_phy_cfg_cmd(mld: *mut iwl_mld) -> c_int;
}
