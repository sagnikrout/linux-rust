//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723be/phy.h
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
// Copyright(c) 2009-2014  Realtek Corporation.
// MAX_TX_COUNT must always set to 4, otherwise read efuse table sequence
// will be wrong.
//
pub const MAX_TX_COUNT: c_int = 4;
pub const TX_1S: c_int = 0;
pub const TX_2S: c_int = 1;
pub const TX_3S: c_int = 2;
pub const TX_4S: c_int = 3;
pub const MAX_POWER_INDEX: c_uint = 0x3F;
pub const MAX_PRECMD_CNT: c_int = 16;
pub const MAX_RFDEPENDCMD_CNT: c_int = 16;
pub const MAX_POSTCMD_CNT: c_int = 16;
pub const MAX_DOZE_WAITING_TIMES_9x: c_int = 64;

pub const HIGHPOWER_RADIOA_ARRAYLEN: c_int = 22;
pub const TARGET_CHNL_NUM_2G_5G: c_int = 59;
pub const IQK_ADDA_REG_NUM: c_int = 16;
pub const IQK_BB_REG_NUM: c_int = 9;
pub const MAX_TOLERANCE: c_int = 5;
pub const IQK_DELAY_TIME: c_int = 10;
pub const index_mapping_NUM: c_int = 15;
pub const APK_BB_REG_NUM: c_int = 5;
pub const APK_AFE_REG_NUM: c_int = 16;
pub const APK_CURVE_REG_NUM: c_int = 4;
pub const PATH_NUM: c_int = 1;
pub const LOOP_LIMIT: c_int = 5;
pub const MAX_STALL_TIME: c_int = 50;
pub const ANTENNADIVERSITYVALUE: c_uint = 0x80;
pub const MAX_TXPWR_IDX_NMODE_92S: c_int = 63;
pub const RESET_CNT_LIMIT: c_int = 3;
pub const IQK_ADDA_REG_NUM: c_int = 16;
pub const IQK_MAC_REG_NUM: c_int = 4;
pub const RF6052_MAX_PATH: c_int = 2;

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
pub const RTL92C_MAX_PATH_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum baseband_config_type {
    BASEBAND_CONFIG_PHY_REG = 0,
    BASEBAND_CONFIG_AGC_TAB = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ant_div_type {
    NO_ANTDIV		= 0xFF,
    CG_TRX_HW_ANTDIV	= 0x01,
    CGCS_RX_HW_ANTDIV	= 0x02,
    FIXED_HW_ANTDIV         = 0x03,
    CG_TRX_SMART_ANTDIV	= 0x04,
    CGCS_RX_SW_ANTDIV	= 0x05,

}

extern "C" {
    pub fn rtl8723be_phy_mac_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl8723be_phy_bb_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl8723be_phy_rf_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl8723be_phy_get_hw_reg_originalvalue(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723be_phy_set_bw_mode_callback(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723be_phy_sw_chnl_callback(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723be_phy_sw_chnl(hw: *mut ieee80211_hw) -> u8;
}
extern "C" {
    pub fn rtl8723be_phy_lc_calibrate(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723be_phy_set_rfpath_switch(hw: *mut ieee80211_hw, bmain: bool);
}
extern "C" {
    pub fn rtl8723be_phy_set_io_cmd(hw: *mut ieee80211_hw, iotype: io_type) -> bool;
}
