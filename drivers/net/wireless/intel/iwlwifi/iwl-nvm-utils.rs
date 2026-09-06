//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-nvm-utils.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2018, 2020-2023, 2026 Intel Corporation
// Copyright (C) 2015 Intel Mobile Communications GmbH
//

// Macro flag: #define __iwl_eeprom_parse_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_data {
    pub n_hw_addrs: c_int,
    pub hw_addr: [u8; ETH_ALEN],
    pub calib_version: u8,
    pub calib_voltage: __le16,
    pub raw_temperature: __le16,
    pub kelvin_temperature: __le16,
    pub kelvin_voltage: __le16,
    pub xtal_calib: [__le16; 2],
    pub sku_cap_band_24ghz_enable: bool,
    pub sku_cap_band_52ghz_enable: bool,
    pub sku_cap_11n_enable: bool,
    pub sku_cap_11ac_enable: bool,
    pub sku_cap_11ax_enable: bool,
    pub sku_cap_amt_enable: bool,
    pub sku_cap_ipan_enable: bool,
    pub sku_cap_mimo_disabled: bool,
    pub sku_cap_11be_enable: bool,
    pub sku_cap_11bn_enable: bool,
    pub radio_cfg_type: u16,
    pub radio_cfg_step: u8,
    pub radio_cfg_dash: u8,
    pub radio_cfg_pnum: u8,
    pub valid_rx_ant: u8 valid_tx_ant,,
    pub nvm_version: u32,
    pub max_tx_pwr_half_dbm: i8,
    pub lar_enabled: bool,
    pub vht160_supported: bool,
    pub bands: [ieee80211_supported_band; NUM_NL80211_BANDS],
//
// iftype data for low (2.4 GHz) high (5 GHz) and uhb (6 GHz) bands
//
    pub low: [ieee80211_sband_iftype_data; 2],
    pub high: [ieee80211_sband_iftype_data; 2],
    pub uhb: [ieee80211_sband_iftype_data; 2],
    pub iftd: },
    pub ht: ieee80211_sta_ht_cap,
    pub vht: ieee80211_sta_vht_cap,
    pub he: ieee80211_sta_he_cap,
    pub nan_phy_capa: },
    pub channels: [ieee80211_channel; ],
}
