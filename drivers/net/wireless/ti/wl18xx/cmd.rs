//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/cmd.h
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
// Copyright (C) 2011 Texas Instruments. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_cmd_channel_switch {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
// The new serving channel
    pub channel: u8,
// Relative time of the serving channel switch in TBTT units
    pub switch_time: u8,
// Stop the role TX, should expect it after radar detection
    pub stop_tx: u8,
    pub local_supported_rates: __le32,
    pub channel_type: u8,
    pub band: u8,
    pub padding: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_cmd_smart_config_start {
    pub header: wl1271_cmd_header,
    pub group_id_bitmask: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_cmd_smart_config_set_group_key {
    pub header: wl1271_cmd_header,
    pub group_id: __le32,
    pub key: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_cmd_dfs_radar_debug {
    pub header: wl1271_cmd_header,
    pub channel: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_cmd_dfs_master_restart {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub padding: [u8; 3],
    pub __packed: },
// cac_start and cac_stop share the same params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_cmd_cac_start {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub channel: u8,
    pub band: u8,
    pub bandwidth: u8,
    pub __packed: },
    pub ch_switch): *mut ieee80211_channel_switch,
    pub group_bitmap): *mut *mut int wl18xx_cmd_smart_config_start(struct wl1271 wl, u32,
    pub wl): *mut int wl18xx_cmd_smart_config_stop(struct wl1271,
    pub key): *mut u8 key_len, u8,
    pub start): *mut *mut *mut int wl18xx_cmd_set_cac(struct wl1271 wl, struct wl12xx_vif wlvif, bool,
    pub channel): *mut *mut int wl18xx_cmd_radar_detection_debug(struct wl1271 wl, u8,
    pub wlvif): *mut *mut int wl18xx_cmd_dfs_master_restart(struct wl1271 wl, struct wl12xx_vif,
