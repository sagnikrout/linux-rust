//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192se/hw.h
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
pub const MSR_LINK_MANAGED: c_int = 2;
pub const MSR_LINK_NONE: c_int = 0;
pub const MSR_LINK_SHIFT: c_int = 0;
pub const MSR_LINK_ADHOC: c_int = 1;
pub const MSR_LINK_MASTER: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WIRELESS_NETWORK_TYPE {
    WIRELESS_11B = 1,
    WIRELESS_11G = 2,
    WIRELESS_11A = 4,
    WIRELESS_11N = 8
}

extern "C" {
    pub fn rtl92se_read_eeprom_info(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_hw_init(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl92se_card_disable(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_enable_interrupt(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_disable_interrupt(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_set_check_bssid(hw: *mut ieee80211_hw, check_bssid: bool);
}
extern "C" {
    pub fn rtl92se_set_mac_addr(io: *mut rtl_io, addr: *const u8);
}
extern "C" {
    pub fn rtl92se_set_qos(hw: *mut ieee80211_hw, aci: c_int);
}
extern "C" {
    pub fn rtl92se_set_beacon_related_registers(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_set_beacon_interval(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_update_channel_access_setting(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8192se_gpiobit3_cfg_inputmode(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_enable_hw_security_config(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_suspend(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92se_resume(hw: *mut ieee80211_hw);
}
