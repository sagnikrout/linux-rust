//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/sta.h
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
// Implementation of mac80211 API.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_sta_priv {
    pub link_id: c_int,
    pub vif_id: c_int,
}

// mac80211 interface
extern "C" {
    pub fn wfx_start(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn wfx_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn wfx_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int;
}
extern "C" {
    pub fn wfx_set_rts_threshold(hw: *mut ieee80211_hw, radio_idx: c_int, value: u32) -> c_int;
}
extern "C" {
    pub fn wfx_set_default_unicast_key(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif, idx: c_int);
}
extern "C" {
    pub fn wfx_add_interface(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wfx_remove_interface(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn wfx_join_ibss(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wfx_leave_ibss(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn wfx_sta_add(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif, sta: *mut ieee80211_sta) -> c_int;
}
extern "C" {
    pub fn wfx_sta_remove(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif, sta: *mut ieee80211_sta) -> c_int;
}
extern "C" {
    pub fn wfx_set_tim(hw: *mut ieee80211_hw, sta: *mut ieee80211_sta, set: bool) -> c_int;
}
extern "C" {
    pub fn wfx_add_chanctx(hw: *mut ieee80211_hw, conf: *mut ieee80211_chanctx_conf) -> c_int;
}
extern "C" {
    pub fn wfx_remove_chanctx(hw: *mut ieee80211_hw, conf: *mut ieee80211_chanctx_conf);
}
extern "C" {
    pub fn wfx_change_chanctx(hw: *mut ieee80211_hw, conf: *mut ieee80211_chanctx_conf, changed: u32);
}
extern "C" {
    pub fn wfx_suspend(hw: *mut ieee80211_hw, wowlan: *mut cfg80211_wowlan) -> c_int;
}
extern "C" {
    pub fn wfx_resume(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn wfx_set_wakeup(hw: *mut ieee80211_hw, enabled: bool);
}
// Hardware API Callbacks
extern "C" {
    pub fn wfx_cooling_timeout_work(work: *mut work_struct);
}
extern "C" {
    pub fn wfx_suspend_hot_dev(wdev: *mut wfx_dev, cmd: sta_notify_cmd);
}
extern "C" {
    pub fn wfx_suspend_resume_mc(wvif: *mut wfx_vif, cmd: sta_notify_cmd);
}
extern "C" {
    pub fn wfx_event_report_rssi(wvif: *mut wfx_vif, raw_rcpi_rssi: u8);
}
extern "C" {
    pub fn wfx_update_pm(wvif: *mut wfx_vif) -> c_int;
}
// Other Helpers
extern "C" {
    pub fn wfx_reset(wvif: *mut wfx_vif);
}
