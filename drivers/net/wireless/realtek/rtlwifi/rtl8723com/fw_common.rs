//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723com/fw_common.h
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
// Copyright(c) 2009-2014  Realtek Corporation.
pub const REG_SYS_FUNC_EN: c_uint = 0x0002;
pub const REG_MCUFWDL: c_uint = 0x0080;
pub const FW_8192C_PAGE_SIZE: c_int = 4096;
pub const FW_8723A_POLLING_TIMEOUT_COUNT: c_int = 1000;
pub const FW_8723B_POLLING_TIMEOUT_COUNT: c_int = 6000;
pub const FW_8192C_POLLING_DELAY: c_int = 5;

pub const REG_RSV_CTRL: c_uint = 0x001C;
pub const REG_HMETFR: c_uint = 0x01CC;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum version_8723e {
    VERSION_TEST_UMC_CHIP_8723 = 0x0081,
    VERSION_NORMAL_UMC_CHIP_8723_1T1R_A_CUT = 0x0089,
    VERSION_NORMAL_UMC_CHIP_8723_1T1R_B_CUT = 0x1089,
    VERSION_TEST_CHIP_1T1R_8723B = 0x0106,
    VERSION_NORMAL_SMIC_CHIP_1T1R_8723B = 0x010E,
    VERSION_UNKNOWN = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8723be_cmd {
    H2C_8723BE_RSVDPAGE = 0,
    H2C_8723BE_JOINBSSRPT = 1,
    H2C_8723BE_SCAN = 2,
    H2C_8723BE_KEEP_ALIVE_CTRL = 3,
    H2C_8723BE_DISCONNECT_DECISION = 4,
    H2C_8723BE_INIT_OFFLOAD = 6,
    H2C_8723BE_AP_OFFLOAD = 8,
    H2C_8723BE_BCN_RSVDPAGE = 9,
    H2C_8723BE_PROBERSP_RSVDPAGE = 10,

    H2C_8723BE_SETPWRMODE = 0x20,
    H2C_8723BE_PS_TUNING_PARA = 0x21,
    H2C_8723BE_PS_TUNING_PARA2 = 0x22,
    H2C_8723BE_PS_LPS_PARA = 0x23,
    H2C_8723BE_P2P_PS_OFFLOAD = 0x24,

    H2C_8723BE_WO_WLAN = 0x80,
    H2C_8723BE_REMOTE_WAKE_CTRL = 0x81,
    H2C_8723BE_AOAC_GLOBAL_INFO = 0x82,
    H2C_8723BE_AOAC_RSVDPAGE = 0x83,
    H2C_8723BE_RSSI_REPORT = 0x42,
    H2C_8723BE_RA_MASK = 0x40,
    H2C_8723BE_SELECTIVE_SUSPEND_ROF_CMD,
    H2C_8723BE_P2P_PS_MODE,
    H2C_8723BE_PSD_RESULT,
// Not defined CTW CMD for P2P yet
    H2C_8723BE_P2P_PS_CTW_CMD,
    MAX_8723BE_H2CCMD
}

extern "C" {
    pub fn rtl8723ae_firmware_selfreset(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723be_firmware_selfreset(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723_enable_fw_download(hw: *mut ieee80211_hw, enable: bool);
}
extern "C" {
    pub fn rtl8723_fw_free_to_go(hw: *mut ieee80211_hw, is_8723be: bool, count: c_int) -> c_int;
}
extern "C" {
    pub fn rtl8723_download_fw(hw: *mut ieee80211_hw, is_8723be: bool, count: c_int) -> c_int;
}
