//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/microchip/wilc1000/fw.h
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
//
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries.
// All rights reserved.
//

pub const WILC_MAX_NUM_STA: c_int = 9;
pub const WILC_MAX_RATES_SUPPORTED: c_int = 12;
pub const WILC_MAX_NUM_PMKIDS: c_int = 16;
pub const WILC_MAX_NUM_SCANNED_CH: c_int = 14;
pub const WILC_NVMEM_MAX_NUM_BANK: c_int = 6;
pub const WILC_NVMEM_BANK_BASE: c_uint = 0x30000000;
pub const WILC_NVMEM_LOW_BANK_OFFSET: c_uint = 0x102c;
pub const WILC_NVMEM_HIGH_BANK_OFFSET: c_uint = 0x1380;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_assoc_resp {
    pub capab_info: __le16,
    pub status_code: __le16,
    pub aid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_pmkid {
    pub bssid: [u8; ETH_ALEN],
    pub pmkid: [u8; WLAN_PMKID_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_pmkid_attr {
    pub numpmkid: u8,
    pub pmkidlist: [wilc_pmkid; WILC_MAX_NUM_PMKIDS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_reg_frame {
    pub reg: u8,
    pub reg_id: u8,
    pub frame_type: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_drv_handler {
    pub handler: __le32,
    pub mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_sta_wpa_ptk {
    pub mac_addr: [u8; ETH_ALEN],
    pub key_len: u8,
    pub key: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_ap_wpa_ptk {
    pub mac_addr: [u8; ETH_ALEN],
    pub index: u8,
    pub key_len: u8,
    pub key: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_wpa_igtk {
    pub index: u8,
    pub pn_len: u8,
    pub pn: [u8; 6],
    pub key_len: u8,
    pub key: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_gtk_key {
    pub mac_addr: [u8; ETH_ALEN],
    pub rsc: [u8; 8],
    pub index: u8,
    pub key_len: u8,
    pub key: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_op_mode {
    pub mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_noa_opp_enable {
    pub ct_window: u8,
    pub cnt: u8,
    pub duration: __le32,
    pub interval: __le32,
    pub start_time: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_noa_opp_disable {
    pub cnt: u8,
    pub duration: __le32,
    pub interval: __le32,
    pub start_time: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_join_bss_param {
    pub ssid: [c_char; IEEE80211_MAX_SSID_LEN],
    pub ssid_terminator: u8,
    pub bss_type: u8,
    pub ch: u8,
    pub cap_info: __le16,
    pub sa: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub beacon_period: __le16,
    pub dtim_period: u8,
    pub 1]: u8 supp_rates[WILC_MAX_RATES_SUPPORTED +,
    pub wmm_cap: u8,
    pub uapsd_cap: u8,
    pub ht_capable: u8,
    pub rsn_found: u8,
    pub rsn_grp_policy: u8,
    pub mode_802_11i: u8,
    pub p_suites: [u8; 3],
    pub akm_suites: [u8; 3],
    pub rsn_cap: [u8; 2],
    pub noa_enabled: u8,
    pub tsf_lo: __le32,
    pub idx: u8,
    pub opp_enabled: u8,
    pub opp_dis: wilc_noa_opp_disable,
    pub opp_en: wilc_noa_opp_enable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_external_auth_param {
    pub action: u8,
    pub bssid: [u8; ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
    pub key_mgmt_suites: __le32,
    pub status: __le16,
    pub __packed: },
    pub 16): *mut *mut WILC_NVMEM_HIGH_BANK_OFFSET + ((i) - 2),
