//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/sta.h
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
// Mac80211 STA interface for ST-Ericsson CW1200 mac80211 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//

// Macro flag: #define STA_H_INCLUDED
// ********************************************************************
// mac80211 API
extern "C" {
    pub fn cw1200_start(dev: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn cw1200_stop(dev: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn cw1200_config(dev: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int;
}
extern "C" {
    pub fn cw1200_set_pm(priv: *mut cw1200_common, arg: *const wsm_set_pm) -> c_int;
}
// ********************************************************************
// WSM callbacks
// ********************************************************************
// WSM events
extern "C" {
    pub fn cw1200_free_event_queue(priv: *mut cw1200_common);
}
extern "C" {
    pub fn cw1200_event_handler(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_bss_loss_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_bss_params_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_keep_alive_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_tx_failure_work(work: *mut work_struct);
}
// ********************************************************************
// Internal API
extern "C" {
    pub fn cw1200_setup_mac(priv: *mut cw1200_common) -> c_int;
}
extern "C" {
    pub fn cw1200_join_timeout(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_unjoin_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_join_complete_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_wep_key_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_update_listening(priv: *mut cw1200_common, enabled: bool);
}
extern "C" {
    pub fn cw1200_update_filtering(priv: *mut cw1200_common);
}
extern "C" {
    pub fn cw1200_update_filtering_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_set_beacon_wakeup_period_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_enable_listening(priv: *mut cw1200_common) -> c_int;
}
extern "C" {
    pub fn cw1200_disable_listening(priv: *mut cw1200_common) -> c_int;
}
extern "C" {
    pub fn cw1200_ba_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_ba_timer(arg: c_ulong);
}
// AP stuffs
extern "C" {
    pub fn cw1200_set_tim_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_set_cts_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_multicast_start_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_multicast_stop_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_mcast_timeout(t: *mut timer_list);
}
