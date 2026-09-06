//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/scan.h
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
// This file is part of wl18xx
//
// Copyright (C) 2012 Texas Instruments. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracking_ch_params {
    pub channel: conn_scan_ch_params,
    pub bssid_lsb: __le32,
    pub bssid_msb: __le16,
    pub padding: [u8; 2],
    pub __packed: },
// probe request rate
}

pub const WL18XX_MAX_CHANNELS_5GHZ: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_cmd_scan_params {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub scan_type: u8,
    pub /: *mut *mut s8 rssi_threshold; / for filtering (in dBm),
    pub /: *mut *mut s8 snr_threshold; / for filtering (in dB),
    pub /: *mut *mut u8 bss_type; / for filtering,
    pub /: *mut *mut u8 ssid_from_list; / use ssid from configured ssid list,
    pub /: *mut *mut u8 filter; / forward only results with matching ssids,
//
// add broadcast ssid in addition to the configured ssids.
// the driver should add dummy entry for it (?).
//
    pub add_broadcast: u8,
    pub urgency: u8,
    pub /: *mut *mut u8 protect; / ???,
    pub /: *mut *mut u8 n_probe_reqs; / Number of probes requests per channel,
    pub /: *mut *mut u8 terminate_after; / early terminate scan operation,
    pub /: *mut *mut u8 passive[SCAN_MAX_BANDS]; / number of passive scan channels,
    pub /: *mut *mut u8 active[SCAN_MAX_BANDS]; / number of active scan channels,
    pub /: *mut *mut u8 dfs; / number of dfs channels in 5ghz,
    pub /: *mut *mut u8 passive_active; / number of passive before active channels 2.4ghz,
    pub short_cycles_msec: __le16,
    pub long_cycles_msec: __le16,
    pub short_cycles_count: u8,
    pub /: *mut *mut u8 total_cycles; / 0 - infinite,
    pub padding: [u8; 2],
    pub channels_2: [conn_scan_ch_params; MAX_CHANNELS_2GHZ],
    pub channels_5: [conn_scan_ch_params; WL18XX_MAX_CHANNELS_5GHZ],
    pub channels_4: [conn_scan_ch_params; MAX_CHANNELS_4GHZ],
}

// send SCAN_REPORT_EVENT in periodic scans after each cycle
// if number of results >= report_threshold. Must be 0 for
// non periodic scans
//
// Should periodic scan stop after a report event was created.
// Must be 0 for non periodic scans.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_cmd_scan_stop {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub scan_type: u8,
    pub padding: [u8; 2],
    pub __packed: },
    pub req): *mut cfg80211_scan_request,
    pub wlvif): *mut *mut int wl18xx_scan_stop(struct wl1271 wl, struct wl12xx_vif,
    pub wlvif): *mut *mut void wl18xx_scan_completed(struct wl1271 wl, struct wl12xx_vif,
    pub ies): *mut ieee80211_scan_ies,
    pub wlvif): *mut *mut void wl18xx_scan_sched_scan_stop(struct wl1271 wl, struct wl12xx_vif,
