//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192ce/hw.h
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
extern "C" {
    pub fn rtl92ce_get_hw_reg(hw: *mut ieee80211_hw, variable: u8, val: *mut u8);
}
extern "C" {
    pub fn rtl92ce_read_eeprom_info(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_hw_init(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl92ce_card_disable(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_enable_interrupt(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_disable_interrupt(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_set_network_type(hw: *mut ieee80211_hw, type: nl80211_iftype) -> c_int;
}
extern "C" {
    pub fn rtl92ce_set_check_bssid(hw: *mut ieee80211_hw, check_bssid: bool);
}
extern "C" {
    pub fn rtl92ce_set_qos(hw: *mut ieee80211_hw, aci: c_int);
}
extern "C" {
    pub fn rtl92ce_set_beacon_related_registers(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_set_beacon_interval(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_set_hw_reg(hw: *mut ieee80211_hw, variable: u8, val: *mut u8);
}
extern "C" {
    pub fn rtl92ce_update_channel_access_setting(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_gpio_radio_on_off_checking(hw: *mut ieee80211_hw, valid: *mut u8) -> bool;
}
extern "C" {
    pub fn rtl92ce_enable_hw_security_config(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8192ce_bt_reg_init(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8192ce_bt_hw_init(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_suspend(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ce_resume(hw: *mut ieee80211_hw);
}
