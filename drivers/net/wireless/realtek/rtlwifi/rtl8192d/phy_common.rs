//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192d/phy_common.h
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
pub const TARGET_CHNL_NUM_5G: c_int = 221;
pub const TARGET_CHNL_NUM_2G: c_int = 14;
pub const CV_CURVE_CNT: c_int = 64;

pub const RX_INDEX_MAPPING_NUM: c_int = 15;
pub const IQK_BB_REG_NUM: c_int = 10;
pub const IQK_DELAY_TIME: c_int = 1;
pub const MAX_TOLERANCE: c_int = 5;
pub const MAX_TOLERANCE_92D: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum baseband_config_type {
    BASEBAND_CONFIG_PHY_REG = 0,
    BASEBAND_CONFIG_AGC_TAB = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_content {
    radioa_txt = 0,
    radiob_txt = 1,
    radioc_txt = 2,
    radiod_txt = 3
}

// flag);
extern "C" {
    pub fn rtl92d_phy_init_bb_rf_register_definition(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_phy_get_hw_reg_originalvalue(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_phy_set_txpower_level(hw: *mut ieee80211_hw, channel: u8);
}
extern "C" {
    pub fn rtl92d_get_rightchnlplace_for_iqk(chnl: u8) -> u8;
}
extern "C" {
    pub fn rtl92d_phy_reset_iqk_result(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_phy_set_io_cmd(hw: *mut ieee80211_hw, iotype: io_type) -> bool;
}
extern "C" {
    pub fn rtl92d_phy_config_macphymode(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_phy_config_macphymode_info(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92d_get_chnlgroup_fromarray(chnl: u8) -> u8;
}
extern "C" {
    pub fn rtl92d_phy_get_chnlgroup_bypg(chnlindex: u8) -> u8;
}
extern "C" {
    pub fn rtl92d_phy_config_maccoexist_rfpage(hw: *mut ieee80211_hw);
}
// Without these declarations sparse warns about context imbalance.
// Without these helpers and the declarations sparse warns about
// context imbalance.
//
extern "C" {
    pub fn rtl92d_pci_lock(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl92d_pci_unlock(rtlpriv: *mut rtl_priv);
}
