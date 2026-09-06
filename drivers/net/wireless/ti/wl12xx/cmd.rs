//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl12xx/cmd.h
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
// Copyright (C) 1998-2009, 2011 Texas Instruments. All rights reserved.
// Copyright (C) 2009 Nokia Corporation
//

pub const TEST_CMD_INI_FILE_RADIO_PARAM: c_uint = 0x19;
pub const TEST_CMD_INI_FILE_GENERAL_PARAM: c_uint = 0x1E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_general_parms_cmd {
    pub header: wl1271_cmd_header,
    pub test: wl1271_cmd_test_header,
    pub general_params: wl1271_ini_general_params,
    pub sr_debug_table: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub sr_sen_n_p: u8,
    pub sr_sen_n_p_gain: u8,
    pub sr_sen_nrn: u8,
    pub sr_sen_prn: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_general_parms_cmd {
    pub header: wl1271_cmd_header,
    pub test: wl1271_cmd_test_header,
    pub general_params: wl128x_ini_general_params,
    pub sr_debug_table: [u8; WL1271_INI_MAX_SMART_REFLEX_PARAM],
    pub sr_sen_n_p: u8,
    pub sr_sen_n_p_gain: u8,
    pub sr_sen_nrn: u8,
    pub sr_sen_prn: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_radio_parms_cmd {
    pub header: wl1271_cmd_header,
    pub test: wl1271_cmd_test_header,
// Static radio parameters
    pub static_params_2: wl1271_ini_band_params_2,
    pub static_params_5: wl1271_ini_band_params_5,
// Dynamic radio parameters
    pub dyn_params_2: wl1271_ini_fem_params_2,
    pub padding2: u8,
    pub dyn_params_5: wl1271_ini_fem_params_5,
    pub padding3: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_radio_parms_cmd {
    pub header: wl1271_cmd_header,
    pub test: wl1271_cmd_test_header,
// Static radio parameters
    pub static_params_2: wl128x_ini_band_params_2,
    pub static_params_5: wl128x_ini_band_params_5,
    pub fem_vendor_and_options: u8,
// Dynamic radio parameters
    pub dyn_params_2: wl128x_ini_fem_params_2,
    pub padding2: u8,
    pub dyn_params_5: wl128x_ini_fem_params_5,
    pub __packed: },
pub const TEST_CMD_INI_FILE_RF_EXTENDED_PARAM: c_uint = 0x26;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ext_radio_parms_cmd {
    pub header: wl1271_cmd_header,
    pub test: wl1271_cmd_test_header,
    pub tx_per_channel_power_compensation_2: [u8; CONF_TX_PWR_COMPENSATION_LEN_2],
    pub tx_per_channel_power_compensation_5: [u8; CONF_TX_PWR_COMPENSATION_LEN_5],
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_channel_switch {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
// The new serving channel
    pub channel: u8,
// Relative time of the serving channel switch in TBTT units
    pub switch_time: u8,
// Stop the role TX, should expect it after radar detection
    pub stop_tx: u8,
// The target channel tx status 1-stopped 0-open
    pub post_switch_tx_disable: u8,
    pub padding: [u8; 3],
    pub __packed: },
    pub wl): *mut int wl1271_cmd_general_parms(struct wl1271,
    pub wl): *mut int wl128x_cmd_general_parms(struct wl1271,
    pub wl): *mut int wl1271_cmd_radio_parms(struct wl1271,
    pub wl): *mut int wl128x_cmd_radio_parms(struct wl1271,
    pub wl): *mut int wl1271_cmd_ext_radio_parms(struct wl1271,
    pub ch_switch): *mut ieee80211_channel_switch,
