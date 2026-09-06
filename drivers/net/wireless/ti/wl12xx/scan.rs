//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl12xx/scan.h
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
// This file is part of wl12xx
//
// Copyright (C) 2012 Texas Instruments. All rights reserved.
//

pub const WL12XX_MAX_CHANNELS_5GHZ: c_int = 23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_scan_params {
// Scan option flags (WL1271_SCAN_OPT_*)
    pub scan_options: __le16,
    pub role_id: u8,
// Number of scan channels in the list (maximum 30)
    pub n_ch: u8,
// This field indicates the number of probe requests to send
    pub n_probe_reqs: u8,
    pub tid_trigger: u8,
    pub ssid_len: u8,
    pub use_ssid_list: u8,
// Rate bit field for sending the probes
    pub tx_rate: __le32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
// Band to scan
    pub band: u8,
    pub scan_tag: u8,
    pub padding2: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_scan_channel_params {
// Duration in TU to wait for frames on a channel for active scan
    pub min_duration: __le32,
    pub max_duration: __le32,
    pub bssid_lsb: __le32,
    pub bssid_msb: __le16,
    pub early_termination: u8,
    pub tx_power_att: u8,
    pub channel: u8,
// FW internal use only!
    pub dfs_candidate: u8,
    pub activity_detected: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_scan {
    pub header: wl1271_cmd_header,
    pub params: basic_scan_params,
    pub channels: [basic_scan_channel_params; WL1271_SCAN_MAX_CHANNELS],
// src mac address
    pub addr: [u8; ETH_ALEN],
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_sched_scan_config {
    pub header: wl1271_cmd_header,
    pub intervals: [__le32; SCAN_MAX_CYCLE_INTERVALS],
    pub /: *mut *mut s8 rssi_threshold; / for filtering (in dBm),
    pub /: *mut *mut s8 snr_threshold; / for filtering (in dB),
    pub /: *mut *mut u8 cycles; / maximum number of scan cycles,
    pub /: *mut *mut u8 report_after; / report when this number of results are received,
    pub /: *mut *mut u8 terminate; / stop scanning after reporting,
    pub tag: u8,
    pub /: *mut *mut u8 bss_type; / for filtering,
    pub filter_type: u8,
    pub /: *mut *mut u8 ssid_len; / For SCAN_SSID_FILTER_SPECIFIC,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub /: *mut *mut u8 n_probe_reqs; / Number of probes requests per channel,
    pub passive: [u8; SCAN_MAX_BANDS],
    pub active: [u8; SCAN_MAX_BANDS],
    pub dfs: u8,
    pub energy): *mut *mut u8 n_pactive_ch; / number of pactive (passive until fw detects,
    pub role_id: u8,
    pub padding: [u8; 1],
    pub channels_2: [conn_scan_ch_params; MAX_CHANNELS_2GHZ],
    pub channels_5: [conn_scan_ch_params; WL12XX_MAX_CHANNELS_5GHZ],
    pub channels_4: [conn_scan_ch_params; MAX_CHANNELS_4GHZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_sched_scan_start {
    pub header: wl1271_cmd_header,
    pub tag: u8,
    pub role_id: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_sched_scan_stop {
    pub header: wl1271_cmd_header,
    pub tag: u8,
    pub role_id: u8,
    pub padding: [u8; 2],
    pub __packed: },
    pub req): *mut cfg80211_scan_request,
    pub wlvif): *mut *mut int wl12xx_scan_stop(struct wl1271 wl, struct wl12xx_vif,
    pub wlvif): *mut *mut void wl12xx_scan_completed(struct wl1271 wl, struct wl12xx_vif,
    pub ies): *mut ieee80211_scan_ies,
    pub wlvif): *mut *mut void wl12xx_scan_sched_scan_stop(struct wl1271 wl, struct wl12xx_vif,
