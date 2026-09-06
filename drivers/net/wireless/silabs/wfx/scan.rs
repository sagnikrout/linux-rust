//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/scan.h
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
// Scan related functions.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
//

extern "C" {
    pub fn wfx_hw_scan_work(work: *mut work_struct);
}
extern "C" {
    pub fn wfx_cancel_hw_scan(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn wfx_scan_complete(wvif: *mut wfx_vif, nb_chan_done: c_int);
}
extern "C" {
    pub fn wfx_remain_on_channel_work(work: *mut work_struct);
}
extern "C" {
    pub fn wfx_cancel_remain_on_channel(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> c_int;
}
