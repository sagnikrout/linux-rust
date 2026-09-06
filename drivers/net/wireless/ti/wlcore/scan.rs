//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/scan.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// This file is part of wl1271
//
// Copyright (C) 2009-2010 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

extern "C" {
    pub fn wl1271_scan_stm(wl: *mut wl1271, wlvif: *mut wl12xx_vif);
}
extern "C" {
    pub fn wl1271_scan_complete_work(work: *mut work_struct);
}
extern "C" {
    pub fn wl1271_scan_sched_scan_start(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wlcore_scan_sched_scan_results(wl: *mut wl1271);
}
pub const WL1271_SCAN_MAX_CHANNELS: c_int = 24;
pub const WL1271_SCAN_DEFAULT_TAG: c_int = 1;
pub const WL1271_SCAN_CURRENT_TX_PWR: c_int = 0;
pub const WL1271_SCAN_OPT_ACTIVE: c_int = 0;
pub const WL1271_SCAN_OPT_PASSIVE: c_int = 1;
pub const WL1271_SCAN_OPT_SPLIT_SCAN: c_int = 2;
pub const WL1271_SCAN_OPT_PRIORITY_HIGH: c_int = 4;
// scan even if we fail to enter psm
pub const WL1271_SCAN_OPT_FORCE: c_int = 8;
pub const WL1271_SCAN_BAND_2_4_GHZ: c_int = 0;
pub const WL1271_SCAN_BAND_5_GHZ: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_trigger_scan_to {
    pub header: wl1271_cmd_header,
    pub timeout: __le32,
    pub __packed: },
pub const MAX_CHANNELS_2GHZ: c_int = 14;
pub const MAX_CHANNELS_4GHZ: c_int = 4;
//
// This max value here is used only for the struct definition of
// wlcore_scan_channels. This struct is used by both 12xx
// and 18xx (which have different max 5ghz channels value).
// In order to make sure this is large enough, just use the
// max possible 5ghz channels.
//
pub const MAX_CHANNELS_5GHZ: c_int = 42;
pub const SCAN_MAX_CYCLE_INTERVALS: c_int = 16;
// The FW intervals can take up to 16 entries.
// The 1st entry isn't used (scan is immediate). The last
// entry should be used for the long_interval
//

pub const SCAN_MAX_BANDS: c_int = 3;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conn_scan_ch_params {
    pub min_duration: __le16,
    pub max_duration: __le16,
    pub passive_duration: __le16,
    pub channel: u8,
    pub tx_power_att: u8,
// bit 0: DFS channel; bit 1: DFS enabled
    pub flags: u8,
    pub padding: [u8; 3],
    pub __packed: },
pub const SCHED_SCAN_MAX_SSIDS: c_int = 16;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ssid {
    pub type: u8,
    pub len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
// u8 padding[2];
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_sched_scan_ssid_list {
    pub header: wl1271_cmd_header,
    pub n_ssids: u8,
    pub ssids: [wl1271_ssid; SCHED_SCAN_MAX_SSIDS],
    pub role_id: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_scan_channels {
    pub /: *mut *mut u8 passive[SCAN_MAX_BANDS]; / number of passive scan channels,
    pub /: *mut *mut u8 active[SCAN_MAX_BANDS]; / number of active scan channels,
    pub /: *mut *mut u8 dfs; / number of dfs channels in 5ghz,
    pub /: *mut *mut u8 passive_active; / number of passive before active channels 2.4ghz,
    pub channels_2: [conn_scan_ch_params; MAX_CHANNELS_2GHZ],
    pub channels_5: [conn_scan_ch_params; MAX_CHANNELS_5GHZ],
    pub channels_4: [conn_scan_ch_params; MAX_CHANNELS_4GHZ],
}
