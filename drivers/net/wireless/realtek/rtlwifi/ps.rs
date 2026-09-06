//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/ps.h
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
pub const MAX_SW_LPS_SLEEP_INTV: c_int = 5;
extern "C" {
    pub fn rtl_ps_enable_nic(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl_ps_disable_nic(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl_ips_nic_off(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_ips_nic_on(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_ips_nic_off_wq_callback(work: *mut work_struct);
}
extern "C" {
    pub fn rtl_lps_enter(hw: *mut ieee80211_hw, may_block: bool);
}
extern "C" {
    pub fn rtl_lps_leave(hw: *mut ieee80211_hw, may_block: bool);
}
extern "C" {
    pub fn rtl_lps_set_psmode(hw: *mut ieee80211_hw, rt_psmode: u8);
}
extern "C" {
    pub fn rtl_swlps_beacon(hw: *mut ieee80211_hw, data: *mut c_void, len: c_uint);
}
extern "C" {
    pub fn rtl_swlps_wq_callback(work: *mut work_struct);
}
extern "C" {
    pub fn rtl_swlps_rfon_wq_callback(work: *mut work_struct);
}
extern "C" {
    pub fn rtl_swlps_rf_awake(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_swlps_rf_sleep(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_p2p_ps_cmd(hw: *mut ieee80211_hw, p2p_ps_state: u8);
}
extern "C" {
    pub fn rtl_p2p_info(hw: *mut ieee80211_hw, data: *mut c_void, len: c_uint);
}
extern "C" {
    pub fn rtl_lps_change_work_callback(work: *mut work_struct);
}
