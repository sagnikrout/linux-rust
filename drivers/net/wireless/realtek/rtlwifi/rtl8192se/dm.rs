//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192se/dm.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_dig_alg {
    DIG_ALGO_BY_FALSE_ALARM = 0,
    DIG_ALGO_BY_RSSI	= 1,
    DIG_ALGO_BEFORE_CONNECT_BY_RSSI_AND_ALARM = 2,
    DIG_ALGO_BY_TOW_PORT = 3,
    DIG_ALGO_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_dig_two_port_alg {
    DIG_TWO_PORT_ALGO_RSSI = 0,
    DIG_TWO_PORT_ALGO_FALSE_ALARM = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_dig_dbg {
    DM_DBG_OFF = 0,
    DM_DBG_ON = 1,
    DM_DBG_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_dig_sta {
    DM_STA_DIG_OFF = 0,
    DM_STA_DIG_ON,
    DM_STA_DIG_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_ratr_sta {
    DM_RATR_STA_HIGH = 0,
    DM_RATR_STA_MIDDLEHIGH = 1,
    DM_RATR_STA_MIDDLE = 2,
    DM_RATR_STA_MIDDLELOW = 3,
    DM_RATR_STA_LOW = 4,
    DM_RATR_STA_ULTRALOW = 5,
    DM_RATR_STA_MAX
}

pub const DM_TYPE_BYFW: c_int = 0;
pub const DM_TYPE_BYDRIVER: c_int = 1;
pub const TX_HIGH_PWR_LEVEL_NORMAL: c_int = 0;
pub const TX_HIGH_PWR_LEVEL_LEVEL1: c_int = 1;
pub const TX_HIGH_PWR_LEVEL_LEVEL2: c_int = 2;

pub const TX_HIGHPWR_LEVEL_NORMAL: c_int = 0;
pub const TX_HIGHPWR_LEVEL_NORMAL1: c_int = 1;
pub const TX_HIGHPWR_LEVEL_NORMAL2: c_int = 2;
pub const TX_POWER_NEAR_FIELD_THRESH_LVL2: c_int = 74;
pub const TX_POWER_NEAR_FIELD_THRESH_LVL1: c_int = 67;
pub const DM_DIG_HIGH_PWR_THRESH_HIGH: c_int = 75;
pub const DM_DIG_HIGH_PWR_THRESH_LOW: c_int = 70;
pub const DM_DIG_MIN_NETCORE: c_uint = 0x12;
extern "C" {
    pub fn rtl92s_dm_watchdog(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92s_dm_init(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92s_dm_init_edca_turbo(hw: *mut ieee80211_hw);
}
