//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192de/phy.h
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
pub const MAX_PRECMD_CNT: c_int = 16;
pub const MAX_RFDEPENDCMD_CNT: c_int = 16;
pub const MAX_POSTCMD_CNT: c_int = 16;
pub const MAX_DOZE_WAITING_TIMES_9x: c_int = 64;
pub const HIGHPOWER_RADIOA_ARRAYLEN: c_int = 22;
pub const APK_BB_REG_NUM: c_int = 5;
pub const APK_AFE_REG_NUM: c_int = 16;
pub const APK_CURVE_REG_NUM: c_int = 4;
pub const PATH_NUM: c_int = 2;
pub const LOOP_LIMIT: c_int = 5;
pub const MAX_STALL_TIME: c_int = 50;
pub const ANTENNA_DIVERSITY_VALUE: c_uint = 0x80;
pub const MAX_TXPWR_IDX_NMODE_92S: c_int = 63;
pub const RESET_CNT_LIMIT: c_int = 3;
pub const IQK_ADDA_REG_NUM: c_int = 16;
pub const IQK_BB_REG_NUM_test: c_int = 6;
pub const IQK_MAC_REG_NUM: c_int = 4;

pub const CT_OFFSET_CCK_TX_PWR_IDX: c_uint = 0x5A;
pub const CT_OFFSET_HT401S_TX_PWR_IDX: c_uint = 0x60;
pub const CT_OFFSET_HT402S_TX_PWR_IDX_DIFF: c_uint = 0x66;
pub const CT_OFFSET_HT20_TX_PWR_IDX_DIFF: c_uint = 0x69;
pub const CT_OFFSET_OFDM_TX_PWR_IDX_DIFF: c_uint = 0x6C;
pub const CT_OFFSET_HT40_MAX_PWR_OFFSET: c_uint = 0x6F;
pub const CT_OFFSET_HT20_MAX_PWR_OFFSET: c_uint = 0x72;
pub const CT_OFFSET_CHANNEL_PLAH: c_uint = 0x75;
pub const CT_OFFSET_THERMAL_METER: c_uint = 0x78;
pub const CT_OFFSET_RF_OPTION: c_uint = 0x79;
pub const CT_OFFSET_VERSION: c_uint = 0x7E;
pub const CT_OFFSET_CUSTOMER_ID: c_uint = 0x7F;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum swchnlcmd_id {
    CMDID_END,
    CMDID_SET_TXPOWEROWER_LEVEL,
    CMDID_BBREGWRITE10,
    CMDID_WRITEPORT_ULONG,
    CMDID_WRITEPORT_USHORT,
    CMDID_WRITEPORT_UCHAR,
    CMDID_RF_WRITEREG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swchnlcmd {
    pub cmdid: swchnlcmd_id,
    pub para1: u32,
    pub para2: u32,
    pub msdelay: u32,
}

extern "C" {
    pub fn rtl92d_phy_mac_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl92d_phy_bb_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl92d_phy_rf_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl92d_phy_sw_chnl(hw: *mut ieee80211_hw) -> u8;
}
extern "C" {
    pub fn rtl92d_phy_set_poweron(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_phy_check_poweroff(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl92d_phy_lc_calibrate(hw: *mut ieee80211_hw, is2t: bool);
}
extern "C" {
    pub fn rtl92d_update_bbrf_configuration(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_phy_iq_calibrate(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_phy_reload_iqk_setting(hw: *mut ieee80211_hw, channel: u8);
}
