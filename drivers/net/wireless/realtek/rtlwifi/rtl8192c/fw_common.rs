//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192c/fw_common.h
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
pub const FW_8192C_SIZE: c_uint = 0x3000;
pub const FW_8192C_START_ADDRESS: c_uint = 0x1000;
pub const FW_8192C_END_ADDRESS: c_uint = 0x1FFF;
pub const FW_8192C_PAGE_SIZE: c_int = 4096;
pub const FW_8192C_POLLING_DELAY: c_int = 5;
pub const FW_8192C_POLLING_TIMEOUT_COUNT: c_int = 100;

pub const H2C_92C_KEEP_ALIVE_CTRL: c_int = 48;

// (u8 *)(__ph2ccmd) = __val

// (u8 *)(__ph2ccmd + 1) = __val

// (u8 *)(__ph2ccmd + 2) = __val

// (u8 *)(__ph2ccmd) = __val

// (u8 *)(__ph2ccmd) = __val

// (u8 *)(__ph2ccmd + 1) = __val

// (u8 *)(__ph2ccmd + 2) = __val
extern "C" {
    pub fn rtl92c_download_fw(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl92c_firmware_selfreset(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_set_fw_pwrmode_cmd(hw: *mut ieee80211_hw, mode: u8);
}
extern "C" {
    pub fn rtl92c_set_fw_joinbss_report_cmd(hw: *mut ieee80211_hw, mstatus: u8);
}
extern "C" {
    pub fn usb_writeN_async(rtlpriv: *mut rtl_priv, addr: u32, data: *mut c_void, len: u16);
}
extern "C" {
    pub fn rtl92c_set_p2p_ps_offload_cmd(hw: *mut ieee80211_hw, p2p_ps_state: u8);
}
