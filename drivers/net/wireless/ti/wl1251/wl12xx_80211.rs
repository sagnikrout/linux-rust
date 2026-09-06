//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/wl12xx_80211.h
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

// RATES
pub const IEEE80211_CCK_RATE_1MB: c_uint = 0x02;
pub const IEEE80211_CCK_RATE_2MB: c_uint = 0x04;
pub const IEEE80211_CCK_RATE_5MB: c_uint = 0x0B;
pub const IEEE80211_CCK_RATE_11MB: c_uint = 0x16;
pub const IEEE80211_OFDM_RATE_6MB: c_uint = 0x0C;
pub const IEEE80211_OFDM_RATE_9MB: c_uint = 0x12;
pub const IEEE80211_OFDM_RATE_12MB: c_uint = 0x18;
pub const IEEE80211_OFDM_RATE_18MB: c_uint = 0x24;
pub const IEEE80211_OFDM_RATE_24MB: c_uint = 0x30;
pub const IEEE80211_OFDM_RATE_36MB: c_uint = 0x48;
pub const IEEE80211_OFDM_RATE_48MB: c_uint = 0x60;
pub const IEEE80211_OFDM_RATE_54MB: c_uint = 0x6C;
pub const IEEE80211_BASIC_RATE_MASK: c_uint = 0x80;

pub const IEEE80211_CCK_RATES_MASK: c_uint = 0x0000000F;

pub const IEEE80211_OFDM_RATES_MASK: c_uint = 0x00000FF0;

// This really should be 8, but not for our firmware
pub const MAX_SUPPORTED_RATES: c_int = 32;
pub const MAX_COUNTRY_TRIPLETS: c_int = 32;
// Headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_header {
    pub frame_ctl: __le16,
    pub duration_id: __le16,
    pub da: [u8; ETH_ALEN],
    pub sa: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub seq_ctl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_ie_header {
    pub id: u8,
    pub len: u8,
    pub __packed: },
// IEs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_ie_ssid {
    pub header: wl12xx_ie_header,
    pub ssid: [c_char; IEEE80211_MAX_SSID_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_ie_rates {
    pub header: wl12xx_ie_header,
    pub rates: [u8; MAX_SUPPORTED_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_ie_ds_params {
    pub header: wl12xx_ie_header,
    pub channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct country_triplet {
    pub channel: u8,
    pub num_channels: u8,
    pub max_tx_power: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_ie_country {
    pub header: wl12xx_ie_header,
    pub country_string: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub triplets: [country_triplet; MAX_COUNTRY_TRIPLETS],
    pub __packed: },
// Templates
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_beacon_template {
    pub header: ieee80211_header,
    pub time_stamp: [__le32; 2],
    pub beacon_interval: __le16,
    pub capability: __le16,
    pub ssid: wl12xx_ie_ssid,
    pub rates: wl12xx_ie_rates,
    pub ext_rates: wl12xx_ie_rates,
    pub ds_params: wl12xx_ie_ds_params,
    pub country: wl12xx_ie_country,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_null_data_template {
    pub header: ieee80211_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_ps_poll_template {
    pub fc: __le16,
    pub aid: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub ta: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_qos_null_data_template {
    pub header: ieee80211_header,
    pub qos_ctl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_probe_req_template {
    pub header: ieee80211_header,
    pub ssid: wl12xx_ie_ssid,
    pub rates: wl12xx_ie_rates,
    pub ext_rates: wl12xx_ie_rates,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_probe_resp_template {
    pub header: ieee80211_header,
    pub time_stamp: [__le32; 2],
    pub beacon_interval: __le16,
    pub capability: __le16,
    pub ssid: wl12xx_ie_ssid,
    pub rates: wl12xx_ie_rates,
    pub ext_rates: wl12xx_ie_rates,
    pub ds_params: wl12xx_ie_ds_params,
    pub country: wl12xx_ie_country,
    pub __packed: },
