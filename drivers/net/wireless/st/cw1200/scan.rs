//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/scan.h
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
// Scan interface for ST-Ericsson CW1200 mac80211 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//

// Macro flag: #define SCAN_H_INCLUDED

// external */ struct sk_buff;
// external */ struct cfg80211_scan_request;
// external */ struct ieee80211_channel;
// external */ struct ieee80211_hw;
// external */ struct work_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_scan {
    pub lock: semaphore,
    pub work: work_struct,
    pub timeout: delayed_work,
    pub req: *mut cfg80211_scan_request,
    pub begin: *mut ieee80211_channel,
    pub curr: *mut ieee80211_channel,
    pub end: *mut ieee80211_channel,
    pub ssids: [wsm_ssid; WSM_SCAN_MAX_NUM_OF_SSIDS],
    pub output_power: c_int,
    pub n_ssids: c_int,
    pub status: c_int,
    pub in_progress: core::sync::atomic::AtomicI32,
// Direct probe requests workaround
    pub probe_work: delayed_work,
    pub direct_probe: c_int,
}

extern "C" {
    pub fn cw1200_scan_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_scan_timeout(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_clear_recent_scan_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_scan_failed_cb(priv: *mut cw1200_common);
}
// ********************************************************************
// Raw probe requests TX workaround
extern "C" {
    pub fn cw1200_probe_work(work: *mut work_struct);
}
