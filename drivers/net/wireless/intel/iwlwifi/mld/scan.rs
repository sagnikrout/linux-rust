//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/scan.h
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
// Copyright (C) 2024-2025 Intel Corporation
//

// Macro flag: #define __iwl_mld_scan_h__
extern "C" {
    pub fn iwl_mld_alloc_scan_cmd(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_int_mlo_scan(mld: *mut iwl_mld, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mld_scan_stop(mld: *mut iwl_mld, type: c_int, notify: bool) -> c_int;
}
pub const WFA_TPC_IE_LEN: c_int = 9;
pub const MAC_HDR_LEN: c_int = 24;
pub const DS_IE_LEN: c_int = 3;
pub const SSID_IE_LEN: c_int = 2;
// driver create the 802.11 header, WFA TPC IE, DS parameter and SSID IE

extern "C" {
    pub fn iwl_mld_report_scan_aborted(mld: *mut iwl_mld);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_scan_status {
    IWL_MLD_SCAN_NONE		= 0,
    IWL_MLD_SCAN_REGULAR		= BIT(0),
    IWL_MLD_SCAN_SCHED		= BIT(1),
    IWL_MLD_SCAN_NETDETECT		= BIT(2),
    IWL_MLD_SCAN_INT_MLO		= BIT(3),
}

// enum iwl_mld_pass_all_sched_results_states - Defines the states for
// handling/passing scheduled scan results to mac80211
// @SCHED_SCAN_PASS_ALL_STATE_DISABLED: Don't pass all scan results, only when
// a match found.
// @SCHED_SCAN_PASS_ALL_STATE_ENABLED: Pass all scan results is enabled
// (no filtering).
// @SCHED_SCAN_PASS_ALL_STATE_FOUND: A scan result is found, pass it on the
// next scan iteration complete notification.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_pass_all_sched_results_states {
    SCHED_SCAN_PASS_ALL_STATE_DISABLED,
    SCHED_SCAN_PASS_ALL_STATE_ENABLED,
    SCHED_SCAN_PASS_ALL_STATE_FOUND,
}

//
// enum iwl_mld_traffic_load - Levels of traffic load
//
// @IWL_MLD_TRAFFIC_LOW: low traffic load
// @IWL_MLD_TRAFFIC_MEDIUM: medium traffic load
// @IWL_MLD_TRAFFIC_HIGH: high traffic load
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_traffic_load {
    IWL_MLD_TRAFFIC_LOW,
    IWL_MLD_TRAFFIC_MEDIUM,
    IWL_MLD_TRAFFIC_HIGH,
}

//
// struct iwl_mld_scan - Scan data
// @status: scan status, a combination of %enum iwl_mld_scan_status,
// reflects the %scan.uid_status array.
// @uid_status: array to track the scan status per uid.
// @start_tsf: start time of last scan in TSF of the link that requested
// the scan.
// @last_ebs_failed: true if the last EBS (Energy Based Scan) failed.
// @pass_all_sched_res: see %enum iwl_mld_pass_all_sched_results_states.
// @fw_link_id: the current (regular) scan fw link id, used by scan
// complete notif.
// @traffic_load: traffic load related data
// @traffic_load.last_stats_ts_usec: The timestamp of the last statistics
// notification, used to calculate the elapsed time between two
// notifications and determine the traffic load
// @traffic_load.status: The current traffic load status, see
// &enum iwl_mld_traffic_load
// @cmd_size: size of %cmd.
// @cmd_ver: version of the scan command format.
// @cmd: pointer to scan cmd buffer (allocated once in op mode start).
// @last_6ghz_passive_jiffies: stores the last 6GHz passive scan time
// in jiffies.
// @last_start_time_jiffies: stores the last start time in jiffies
// (interface up/reset/resume).
// @last_mlo_scan_start_time: start time of the last MLO scan in nanoseconds
// since boot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_scan {
// Add here fields that need clean up on restart
    pub status: c_uint,
    pub uid_status: [u32; IWL_MAX_UMAC_SCANS],
    pub start_tsf: u64,
    pub last_ebs_failed: bool,
    pub pass_all_sched_res: iwl_mld_pass_all_sched_results_states,
    pub fw_link_id: u8,
    pub last_stats_ts_usec: u32,
    pub status: iwl_mld_traffic_load,
    pub traffic_load: },
// And here fields that survive a fw restart
    pub cmd_size: usize,
    pub cmd: *mut c_void,
    pub cmd_ver: u8,
    pub last_6ghz_passive_jiffies: c_ulong,
    pub last_start_time_jiffies: c_ulong,
    pub last_mlo_scan_start_time: u64,
}

//
// struct iwl_mld_survey_channel - per-channel survey information
//
// Driver version of &struct survey_info with just the data we want to report.
//
// @time: time in ms the radio was on the channel
// @time_busy: time in ms the channel was sensed busy
// @noise: channel noise in dBm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_survey_channel {
    pub time: u32,
    pub time_busy: u32,
    pub noise: i8,
}

//
// struct iwl_mld_survey - survey information
//
// Survey information for all available channels.
//
// @bands: per-band array for per-channel survey data, points into @channels
// @n_channels: Number of @channels entries that are allocated
// @channels: per-channel information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_survey {
    pub bands: [*mut iwl_mld_survey_channel; NUM_NL80211_BANDS],
    pub n_channels: c_int,
    pub __counted_by(n_channels): iwl_mld_survey_channel channels[],
}
