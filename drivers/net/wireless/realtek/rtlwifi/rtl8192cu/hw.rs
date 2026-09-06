//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192cu/hw.h
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
pub const H2C_RA_MASK: c_int = 6;
pub const LLT_LAST_ENTRY_OF_TX_PKT_BUFFER: c_int = 255;

// Note: We will divide number of page equally for each queue
// other than public queue!
pub const TX_TOTAL_PAGE_NUMBER: c_uint = 0xF8;

pub const CHIP_B_PAGE_NUM_PUBQ: c_uint = 0xE7;
// For Test Chip Setting
// (HPQ + LPQ + PUBQ) shall be TX_TOTAL_PAGE_NUMBER
pub const CHIP_A_PAGE_NUM_PUBQ: c_uint = 0x7E;
// For Chip A Setting
pub const WMM_CHIP_A_TX_TOTAL_PAGE_NUMBER: c_uint = 0xF5;

pub const WMM_CHIP_A_PAGE_NUM_PUBQ: c_uint = 0xA3;
pub const WMM_CHIP_A_PAGE_NUM_HPQ: c_uint = 0x29;
pub const WMM_CHIP_A_PAGE_NUM_LPQ: c_uint = 0x29;
// Note: For Chip B Setting ,modify later
pub const WMM_CHIP_B_TX_TOTAL_PAGE_NUMBER: c_uint = 0xF5;

pub const WMM_CHIP_B_PAGE_NUM_PUBQ: c_uint = 0xB0;
pub const WMM_CHIP_B_PAGE_NUM_HPQ: c_uint = 0x29;
pub const WMM_CHIP_B_PAGE_NUM_LPQ: c_uint = 0x1C;
pub const WMM_CHIP_B_PAGE_NUM_NPQ: c_uint = 0x1C;
pub const BOARD_TYPE_NORMAL_MASK: c_uint = 0xE0;
pub const BOARD_TYPE_TEST_MASK: c_uint = 0x0F;
// should be renamed and moved to another file
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BOARD_TYPE_8192CUSB {
    BOARD_USB_DONGLE		= 0,	/* USB dongle */
    BOARD_USB_HIGH_PA		= 1,	/* USB dongle - high power PA */
    BOARD_MINICARD			= 2,	/* Minicard */
    BOARD_USB_SOLO			= 3,	/* USB solo-Slim module */
    BOARD_USB_COMBO			= 4,	/* USB Combo-Slim module */
}

pub const RTL92C_DRIVER_INFO_SIZE: c_int = 4;
extern "C" {
    pub fn rtl92cu_read_eeprom_info(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92cu_enable_hw_security_config(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92cu_hw_init(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl92cu_card_disable(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92cu_set_network_type(hw: *mut ieee80211_hw, type: nl80211_iftype) -> c_int;
}
extern "C" {
    pub fn rtl92cu_set_beacon_related_registers(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92cu_set_beacon_interval(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92cu_get_hw_reg(hw: *mut ieee80211_hw, variable: u8, val: *mut u8);
}
extern "C" {
    pub fn rtl92cu_set_hw_reg(hw: *mut ieee80211_hw, variable: u8, val: *mut u8);
}
extern "C" {
    pub fn rtl92cu_update_channel_access_setting(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92cu_gpio_radio_on_off_checking(hw: *mut ieee80211_hw, valid: *mut *mut u8) -> bool;
}
extern "C" {
    pub fn rtl92cu_set_check_bssid(hw: *mut ieee80211_hw, check_bssid: bool);
}
extern "C" {
    pub fn rtl92c_download_fw(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl92c_set_fw_pwrmode_cmd(hw: *mut ieee80211_hw, mode: u8);
}
extern "C" {
    pub fn rtl92c_set_fw_joinbss_report_cmd(hw: *mut ieee80211_hw, mstatus: u8);
}
extern "C" {
    pub fn rtl92cu_phy_mac_config(hw: *mut ieee80211_hw) -> bool;
}
