//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/btcoexist/rtl_btc.h
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
// Copyright(c) 2009-2010  Realtek Corporation.

extern "C" {
    pub fn rtl_btc_init_variables(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_init_variables_wifi_only(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_deinit_variables(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_init_hal_vars(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_power_on_setting(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_init_hw_config(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_init_hw_config_wifi_only(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_ips_notify(rtlpriv: *mut rtl_priv, type: u8);
}
extern "C" {
    pub fn rtl_btc_lps_notify(rtlpriv: *mut rtl_priv, type: u8);
}
extern "C" {
    pub fn rtl_btc_scan_notify(rtlpriv: *mut rtl_priv, scantype: u8);
}
extern "C" {
    pub fn rtl_btc_scan_notify_wifi_only(rtlpriv: *mut rtl_priv, scantype: u8);
}
extern "C" {
    pub fn rtl_btc_connect_notify(rtlpriv: *mut rtl_priv, action: u8);
}
extern "C" {
    pub fn rtl_btc_periodical(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_halt_notify(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_btc_btinfo_notify(rtlpriv: *mut rtl_priv, tmpbuf: *mut u8, length: u8);
}
extern "C" {
    pub fn rtl_btc_btmpinfo_notify(rtlpriv: *mut rtl_priv, tmp_buf: *mut u8, length: u8);
}
extern "C" {
    pub fn rtl_btc_is_limited_dig(rtlpriv: *mut rtl_priv) -> bool;
}
extern "C" {
    pub fn rtl_btc_is_disable_edca_turbo(rtlpriv: *mut rtl_priv) -> bool;
}
extern "C" {
    pub fn rtl_btc_is_bt_disabled(rtlpriv: *mut rtl_priv) -> bool;
}
extern "C" {
    pub fn rtl_btc_special_packet_notify(rtlpriv: *mut rtl_priv, pkt_type: u8);
}
extern "C" {
    pub fn rtl_btc_display_bt_coex_info(rtlpriv: *mut rtl_priv, m: *mut seq_file);
}
extern "C" {
    pub fn rtl_btc_record_pwr_mode(rtlpriv: *mut rtl_priv, buf: *mut u8, len: u8);
}
extern "C" {
    pub fn rtl_btc_get_lps_val(rtlpriv: *mut rtl_priv) -> u8;
}
extern "C" {
    pub fn rtl_btc_get_rpwm_val(rtlpriv: *mut rtl_priv) -> u8;
}
extern "C" {
    pub fn rtl_btc_is_bt_ctrl_lps(rtlpriv: *mut rtl_priv) -> bool;
}
extern "C" {
    pub fn rtl_btc_is_bt_lps_on(rtlpriv: *mut rtl_priv) -> bool;
}
extern "C" {
    pub fn rtl_get_hwpg_bt_exist(rtlpriv: *mut rtl_priv) -> u8;
}
extern "C" {
    pub fn rtl_get_hwpg_bt_type(rtlpriv: *mut rtl_priv) -> u8;
}
extern "C" {
    pub fn rtl_get_hwpg_ant_num(rtlpriv: *mut rtl_priv) -> u8;
}
extern "C" {
    pub fn rtl_get_hwpg_single_ant_path(rtlpriv: *mut rtl_priv) -> u8;
}
extern "C" {
    pub fn rtl_get_hwpg_package_type(rtlpriv: *mut rtl_priv) -> u8;
}
extern "C" {
    pub fn mgnt_link_status_query(hw: *mut ieee80211_hw) -> rt_media_status;
}
