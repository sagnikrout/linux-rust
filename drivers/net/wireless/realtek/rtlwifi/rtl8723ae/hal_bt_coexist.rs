//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723ae/hal_bt_coexist.h
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
// Copyright(c) 2009-2012  Realtek Corporation.

// The reg define is for 8723
pub const REG_HIGH_PRIORITY_TXRX: c_uint = 0x770;
pub const REG_LOW_PRIORITY_TXRX: c_uint = 0x774;
pub const BT_FW_COEX_THRESH_TOL: c_int = 6;
pub const BT_FW_COEX_THRESH_20: c_int = 20;
pub const BT_FW_COEX_THRESH_23: c_int = 23;
pub const BT_FW_COEX_THRESH_25: c_int = 25;
pub const BT_FW_COEX_THRESH_30: c_int = 30;
pub const BT_FW_COEX_THRESH_35: c_int = 35;
pub const BT_FW_COEX_THRESH_40: c_int = 40;
pub const BT_FW_COEX_THRESH_45: c_int = 45;
pub const BT_FW_COEX_THRESH_47: c_int = 47;
pub const BT_FW_COEX_THRESH_50: c_int = 50;
pub const BT_FW_COEX_THRESH_55: c_int = 55;

pub const BT_RSSI_STATE_HIGH: c_int = 0;
pub const BT_RSSI_STATE_MEDIUM: c_int = 1;
pub const BT_RSSI_STATE_LOW: c_int = 2;
pub const BT_RSSI_STATE_STAY_HIGH: c_int = 3;
pub const BT_RSSI_STATE_STAY_MEDIUM: c_int = 4;
pub const BT_RSSI_STATE_STAY_LOW: c_int = 5;
pub const BT_AGCTABLE_OFF: c_int = 0;
pub const BT_AGCTABLE_ON: c_int = 1;
pub const BT_BB_BACKOFF_OFF: c_int = 0;
pub const BT_BB_BACKOFF_ON: c_int = 1;
pub const BT_FW_NAV_OFF: c_int = 0;
pub const BT_FW_NAV_ON: c_int = 1;
pub const BT_COEX_MECH_NONE: c_int = 0;
pub const BT_COEX_MECH_SCO: c_int = 1;
pub const BT_COEX_MECH_HID: c_int = 2;
pub const BT_COEX_MECH_A2DP: c_int = 3;
pub const BT_COEX_MECH_PAN: c_int = 4;
pub const BT_COEX_MECH_HID_A2DP: c_int = 5;
pub const BT_COEX_MECH_HID_PAN: c_int = 6;
pub const BT_COEX_MECH_PAN_A2DP: c_int = 7;
pub const BT_COEX_MECH_HID_SCO_ESCO: c_int = 8;
pub const BT_COEX_MECH_FTP_A2DP: c_int = 9;
pub const BT_COEX_MECH_COMMON: c_int = 10;
pub const BT_COEX_MECH_MAX: c_int = 11;
pub const BT_DBG_PROFILE_NONE: c_int = 0;
pub const BT_DBG_PROFILE_SCO: c_int = 1;
pub const BT_DBG_PROFILE_HID: c_int = 2;
pub const BT_DBG_PROFILE_A2DP: c_int = 3;
pub const BT_DBG_PROFILE_PAN: c_int = 4;
pub const BT_DBG_PROFILE_HID_A2DP: c_int = 5;
pub const BT_DBG_PROFILE_HID_PAN: c_int = 6;
pub const BT_DBG_PROFILE_PAN_A2DP: c_int = 7;
pub const BT_DBG_PROFILE_MAX: c_int = 9;

extern "C" {
    pub fn rtl8723e_btdm_coex_all_off(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_fw_coex_all_off(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_sw_coex_all_off(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_hw_coex_all_off(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_get_rx_ss(hw: *mut ieee80211_hw) -> c_long;
}
extern "C" {
    pub fn rtl8723e_dm_bt_agc_table(hw: *mut ieee80211_hw, type: u8);
}
extern "C" {
    pub fn rtl8723e_dm_bt_bb_back_off_level(hw: *mut ieee80211_hw, type: u8);
}
extern "C" {
    pub fn _rtl8723_dm_bt_check_wifi_state(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_is_coexist_state_changed(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl8723e_dm_bt_is_wifi_up_link(hw: *mut ieee80211_hw) -> bool;
}
