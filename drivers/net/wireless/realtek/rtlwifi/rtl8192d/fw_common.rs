//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192d/fw_common.h
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
pub const FW_8192D_START_ADDRESS: c_uint = 0x1000;
pub const FW_8192D_PAGE_SIZE: c_int = 4096;
pub const FW_8192D_POLLING_TIMEOUT_COUNT: c_int = 1000;

// Firmware Header(8-byte alinment required)
// --- LONG WORD 0 ----

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl92d_rate_mask_h2c {
    pub rate_mask_and_raid: __le32,
    pub macid_and_short_gi: u8,
    pub __packed: },
    pub rtlpriv): *mut bool rtl92d_is_fw_downloaded(struct rtl_priv,
    pub enable): *mut *mut void rtl92d_enable_fw_download(struct ieee80211_hw hw, bool,
    pub size): *mut *mut version_8192d version, u8 buffer, u32,
    pub hw): *mut int rtl92d_fw_free_to_go(struct ieee80211_hw,
    pub hw): *mut void rtl92d_firmware_selfreset(struct ieee80211_hw,
    pub hw): *mut int rtl92d_fw_init(struct ieee80211_hw,
    pub p_cmdbuffer): *mut u32 cmd_len, u8,
    pub mstatus): *mut *mut void rtl92d_set_fw_joinbss_report_cmd(struct ieee80211_hw hw, u8,
