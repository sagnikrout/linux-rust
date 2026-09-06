//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192se/phy.h
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
pub const MAX_TXPWR_IDX_NMODE_92S: c_int = 63;
pub const MAX_DOZE_WAITING_TIMES_9x: c_int = 64;
// Channel switch:The size of
// command tables for switch channel
pub const MAX_PRECMD_CNT: c_int = 16;
pub const MAX_RFDEPENDCMD_CNT: c_int = 16;
pub const MAX_POSTCMD_CNT: c_int = 16;
pub const RF90_PATH_MAX: c_int = 4;
pub const RF6052_MAX_PATH: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum version_8192s {
    VERSION_8192S_ACUT,
    VERSION_8192S_BCUT,
    VERSION_8192S_CCUT
}

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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum baseband_config_type {
// Radio Path A
    BASEBAND_CONFIG_PHY_REG = 0,
// Radio Path B
    BASEBAND_CONFIG_AGC_TAB = 1,
}

extern "C" {
    pub fn rtl92s_phy_query_bb_reg(hw: *mut ieee80211_hw, regaddr: u32, bitmask: u32) -> u32;
}
extern "C" {
    pub fn rtl92s_phy_scan_operation_backup(hw: *mut ieee80211_hw, operation: u8);
}
extern "C" {
    pub fn rtl92s_phy_sw_chnl(hw: *mut ieee80211_hw) -> u8;
}
extern "C" {
    pub fn rtl92s_phy_mac_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl92s_phy_switch_ephy_parameter(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92s_phy_bb_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl92s_phy_rf_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl92s_phy_get_hw_reg_originalvalue(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92s_phy_set_txpower(hw: *mut ieee80211_hw, channel: u8);
}
extern "C" {
    pub fn rtl92s_phy_set_fw_cmd(hw: *mut ieee80211_hw, fwcmd_io: fwcmd_iotype) -> bool;
}
extern "C" {
    pub fn rtl92s_phy_chk_fwcmd_iodone(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92s_phy_set_beacon_hwreg(hw: *mut ieee80211_hw, beaconinterval: u16);
}
extern "C" {
    pub fn rtl92s_phy_config_rf(hw: *mut ieee80211_hw, rfpath: radio_path) -> u8;
}
