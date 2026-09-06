//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/core.h
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
// Copyright(c) 2009-2012  Realtek Corporation.

pub const DM_DIG_THRESH_HIGH: c_int = 40;
pub const DM_DIG_THRESH_LOW: c_int = 35;
pub const DM_FALSEALARM_THRESH_LOW: c_int = 400;
pub const DM_FALSEALARM_THRESH_HIGH: c_int = 1000;
pub const DM_DIG_MAX: c_uint = 0x3e;
pub const DM_DIG_MIN: c_uint = 0x1e;
pub const DM_DIG_MAX_AP: c_uint = 0x32;
pub const DM_DIG_BACKOFF_MAX: c_int = 12;

pub const DM_DIG_BACKOFF_DEFAULT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cck_packet_detection_threshold {
    CCK_PD_STAGE_LOWRSSI = 0,
    CCK_PD_STAGE_HIGHRSSI = 1,
    CCK_FA_STAGE_LOW = 2,
    CCK_FA_STAGE_HIGH = 3,
    CCK_PD_STAGE_MAX = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_dig_ext_port_alg_e {
    DIG_EXT_PORT_STAGE_0 = 0,
    DIG_EXT_PORT_STAGE_1 = 1,
    DIG_EXT_PORT_STAGE_2 = 2,
    DIG_EXT_PORT_STAGE_3 = 3,
    DIG_EXT_PORT_STAGE_MAX = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_dig_connect_e {
    DIG_STA_DISCONNECT,
    DIG_STA_CONNECT,
    DIG_STA_BEFORE_CONNECT,
    DIG_MULTISTA_DISCONNECT,
    DIG_MULTISTA_CONNECT,
    DIG_AP_DISCONNECT,
    DIG_AP_CONNECT,
    DIG_AP_ADD_STATION,
    DIG_CONNECT_MAX
}

extern "C" {
    pub fn rtl_init_sw_leds(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_fw_cb(firmware: *const firmware, context: *mut c_void);
}
extern "C" {
    pub fn rtl_wowlan_fw_cb(firmware: *const firmware, context: *mut c_void);
}
extern "C" {
    pub fn rtl_addr_delay(addr: u32);
}
extern "C" {
    pub fn rtl_cmd_send_packet(hw: *mut ieee80211_hw, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn rtl_btc_status_false() -> bool;
}
extern "C" {
    pub fn rtl_dm_diginit(hw: *mut ieee80211_hw, cur_igval: u32);
}
extern "C" {
    pub fn rtl_update_beacon_work_callback(work: *mut work_struct);
}
