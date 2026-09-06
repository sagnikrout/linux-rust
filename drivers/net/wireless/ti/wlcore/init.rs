//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/init.h
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
// Copyright (C) 2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

extern "C" {
    pub fn wl1271_hw_init_power_auth(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_init_templates_config(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_init_pta(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_init_energy_detection(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_chip_specific_init(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_hw_init(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_init_vif_specific(wl: *mut wl1271, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_init_ap_rates(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_ap_init_templates(wl: *mut wl1271, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_sta_hw_init(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
