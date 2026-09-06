//! Automatically rewritten from C Header to Rust Module
//! Source: net/wireless/nl80211.h
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
// Portions of this file
// Copyright (C) 2018, 2020-2026 Intel Corporation
//

extern "C" {
    pub fn nl80211_init() -> c_int;
}
extern "C" {
    pub fn nl80211_exit();
}
extern "C" {
    pub fn nl80211_send_sched_scan(req: *mut cfg80211_sched_scan_request, cmd: u32);
}
// For STA/GC, indicate port authorized with AP/GO bssid.
// For GO/AP, use peer GC/STA mac_addr.
//
extern "C" {
    pub fn nl80211_send_ap_stopped(wdev: *mut wireless_dev, link_id: c_uint);
}
extern "C" {
    pub fn cfg80211_free_coalesce(coalesce: *mut cfg80211_coalesce);
}
// peer measurement
extern "C" {
    pub fn nl80211_pmsr_start(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
