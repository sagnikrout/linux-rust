//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/coex.h
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
// Copyright (C) 2023-2025 Intel Corporation
// Copyright (C) 2013-2014, 2018-2019 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
// Copyright (C) 2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_coex_h__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_coex_lut_type {
    BT_COEX_TIGHT_LUT = 0,
    BT_COEX_LOOSE_LUT,
    BT_COEX_TX_DIS_LUT,

    BT_COEX_MAX_LUT,
    BT_COEX_INVALID_LUT = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_coex_mode {
    BT_COEX_DISABLE			= 0x0,
    BT_COEX_NW			= 0x1,
    BT_COEX_BT			= 0x2,
    BT_COEX_WIFI			= 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_coex_enabled_modules {
    BT_COEX_MPLUT_ENABLED		= BIT(0),
    BT_COEX_MPLUT_BOOST_ENABLED	= BIT(1),
    BT_COEX_SYNC2SCO_ENABLED	= BIT(2),
    BT_COEX_CORUN_ENABLED		= BIT(3),
    BT_COEX_HIGH_BAND_RET		= BIT(4),
}

//
// struct iwl_bt_coex_cmd - bt coex configuration command
// @mode: &enum iwl_bt_coex_mode
// @enabled_modules: &enum iwl_bt_coex_enabled_modules
//
// The structure is used for the BT_COEX command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_cmd {
    pub mode: __le32,
    pub enabled_modules: __le32,
    pub /: *mut *mut } __packed; / BT_COEX_CMD_API_S_VER_6,
//
// struct iwl_bt_coex_reduced_txp_update_cmd - reduced TX power command
// @reduced_txp: bit BT_REDUCED_TX_POWER_BIT to enable / disable, rest of the
// bits are the sta_id (value)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_reduced_txp_update_cmd {
    pub reduced_txp: __le32,
    pub /: *mut *mut } __packed; / BT_COEX_UPDATE_REDUCED_TX_POWER_API_S_VER_1,
//
// struct iwl_bt_coex_ci_cmd - bt coex channel inhibition command
// @bt_primary_ci: primary channel inhibition bitmap
// @primary_ch_phy_id: primary channel PHY ID
// @bt_secondary_ci: secondary channel inhibition bitmap
// @secondary_ch_phy_id: secondary channel PHY ID
//
// Used for BT_COEX_CI command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_ci_cmd {
    pub bt_primary_ci: __le64,
    pub primary_ch_phy_id: __le32,
    pub bt_secondary_ci: __le64,
    pub secondary_ch_phy_id: __le32,
    pub /: *mut *mut } __packed; / BT_CI_MSG_API_S_VER_2,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_activity_grading {
    BT_OFF			= 0,
    BT_ON_NO_CONNECTION	= 1,
    BT_LOW_TRAFFIC		= 2,
    BT_HIGH_TRAFFIC		= 3,
    BT_VERY_HIGH_TRAFFIC	= 4,

    BT_MAX_AG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_ci_compliance {
    BT_CI_COMPLIANCE_NONE		= 0,
    BT_CI_COMPLIANCE_PRIMARY	= 1,
    BT_CI_COMPLIANCE_SECONDARY	= 2,
    BT_CI_COMPLIANCE_BOTH		= 3,
}

//
// struct iwl_bt_coex_prof_old_notif - notification about BT coex
// @mbox_msg: message from BT to WiFi
// @msg_idx: the index of the message
// @bt_ci_compliance: enum %iwl_bt_ci_compliance
// @primary_ch_lut: LUT used for primary channel &enum iwl_bt_coex_lut_type
// @secondary_ch_lut: LUT used for secondary channel &enum iwl_bt_coex_lut_type
// @bt_activity_grading: the activity of BT &enum iwl_bt_activity_grading
// @ttc_status: is TTC enabled - one bit per PHY
// @rrc_status: is RRC enabled - one bit per PHY
// The following fields are only for version 5, and are reserved in version 4:
// @wifi_loss_low_rssi: The predicted lost WiFi rate (% of air time that BT is
// utilizing) when the RSSI is low (<= -65 dBm)
// @wifi_loss_mid_high_rssi: The predicted lost WiFi rate (% of air time that
// BT is utilizing) when the RSSI is mid/high (>= -65 dBm)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_prof_old_notif {
    pub mbox_msg: [__le32; 4],
    pub msg_idx: __le32,
    pub bt_ci_compliance: __le32,
    pub primary_ch_lut: __le32,
    pub secondary_ch_lut: __le32,
    pub bt_activity_grading: __le32,
    pub ttc_status: u8,
    pub rrc_status: u8,
    pub wifi_loss_low_rssi: u8,
    pub wifi_loss_mid_high_rssi: u8,
    pub BT_COEX_PROFILE_NTFY_API_S_VER_4: *mut *mut } __packed; /,
// BT_COEX_PROFILE_NTFY_API_S_VER_5
//
// enum iwl_bt_coex_subcmd_ids - coex configuration command IDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_coex_subcmd_ids {
//
// @PROFILE_NOTIF: &struct iwl_bt_coex_profile_notif
//
    PROFILE_NOTIF = 0xFF,
}

pub const COEX_NUM_BAND: c_int = 3;
pub const COEX_NUM_CHAINS: c_int = 2;
//
// struct iwl_bt_coex_profile_notif - notification about BT coex
// @wifi_loss_low_rssi: The predicted lost WiFi rate (% of air time that BT is
// utilizing) when the RSSI is low (<= -65 dBm)
// @wifi_loss_mid_high_rssi: The predicted lost WiFi rate (% of air time that
// BT is utilizing) when the RSSI is mid/high (>= -65 dBm)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_profile_notif {
    pub wifi_loss_low_rssi: [u8; COEX_NUM_BAND][COEX_NUM_CHAINS],
    pub wifi_loss_mid_high_rssi: [u8; COEX_NUM_BAND][COEX_NUM_CHAINS],
    pub /: *mut *mut } __packed; / BT_COEX_BT_PROFILE_NTF_API_S_VER_1,
