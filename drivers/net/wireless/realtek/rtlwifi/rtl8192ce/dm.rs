//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192ce/dm.h
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

pub const OFDM_TABLE_LENGTH: c_int = 37;
pub const CCK_TABLE_LENGTH: c_int = 33;
pub const OFDM_TABLE_SIZE: c_int = 37;
pub const CCK_TABLE_SIZE: c_int = 33;
pub const BW_AUTO_SWITCH_HIGH_LOW: c_int = 25;
pub const BW_AUTO_SWITCH_LOW_HIGH: c_int = 30;
pub const DM_DIG_FA_UPPER: c_uint = 0x32;
pub const DM_DIG_FA_LOWER: c_uint = 0x20;
pub const DM_DIG_FA_TH0: c_uint = 0x20;
pub const DM_DIG_FA_TH1: c_uint = 0x100;
pub const DM_DIG_FA_TH2: c_uint = 0x200;
pub const RXPATHSELECTION_SS_TH_LOW: c_int = 30;
pub const RXPATHSELECTION_DIFF_TH: c_int = 18;
pub const DM_RATR_STA_INIT: c_int = 0;
pub const DM_RATR_STA_HIGH: c_int = 1;
pub const DM_RATR_STA_MIDDLE: c_int = 2;
pub const DM_RATR_STA_LOW: c_int = 3;
pub const CTS2SELF_THVAL: c_int = 30;
pub const REGC38_TH: c_int = 20;
pub const WAIOTTHVAL: c_int = 25;
pub const TXHIGHPWRLEVEL_NORMAL: c_int = 0;
pub const TXHIGHPWRLEVEL_LEVEL1: c_int = 1;
pub const TXHIGHPWRLEVEL_LEVEL2: c_int = 2;
pub const TXHIGHPWRLEVEL_BT1: c_int = 3;
pub const TXHIGHPWRLEVEL_BT2: c_int = 4;
pub const DM_TYPE_BYFW: c_int = 0;
pub const DM_TYPE_BYDRIVER: c_int = 1;
pub const TX_POWER_NEAR_FIELD_THRESH_LVL2: c_int = 74;
pub const TX_POWER_NEAR_FIELD_THRESH_LVL1: c_int = 67;
extern "C" {
    pub fn rtl92c_dm_init(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_dm_watchdog(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_dm_write_dig(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_dm_init_edca_turbo(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_dm_check_txpower_tracking(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_dm_init_rate_adaptive_mask(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_dm_rf_saving(hw: *mut ieee80211_hw, bforce_in_normal: u8);
}
extern "C" {
    pub fn rtl92c_dm_bt_coexist(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_dm_dynamic_txpower(hw: *mut ieee80211_hw);
}
