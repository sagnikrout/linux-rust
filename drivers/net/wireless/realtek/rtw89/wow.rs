//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/wow.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2019-2022  Realtek Corporation
//

pub const RTW89_WOW_VALID_CHECK: c_uint = 0xDD;

pub const RTW89_MIC_KEY_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_wake_reason {
    RTW89_WOW_RSN_RX_PTK_REKEY = 0x1,
    RTW89_WOW_RSN_RX_GTK_REKEY = 0x2,
    RTW89_WOW_RSN_RX_DISASSOC = 0x4,
    RTW89_WOW_RSN_RX_DEAUTH = 0x8,
    RTW89_WOW_RSN_DISCONNECT = 0x10,
    RTW89_WOW_RSN_RX_MAGIC_PKT = 0x21,
    RTW89_WOW_RSN_RX_PATTERN_MATCH = 0x23,
    RTW89_WOW_RSN_RX_NLO = 0x55,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_alg {
    RTW89_WOW_FW_ALG_WEP40 = 0x1,
    RTW89_WOW_FW_ALG_WEP104 = 0x2,
    RTW89_WOW_FW_ALG_TKIP = 0x3,
    RTW89_WOW_FW_ALG_CCMP = 0x6,
    RTW89_WOW_FW_ALG_CCMP_256 = 0x7,
    RTW89_WOW_FW_ALG_GCMP = 0x8,
    RTW89_WOW_FW_ALG_GCMP_256 = 0x9,
    RTW89_WOW_FW_ALG_AES_CMAC = 0xa,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_cipher_suite {
    pub oui: [u8; 3],
    pub type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rsn_ie {
    pub tag_number: u8,
    pub tag_length: u8,
    pub rsn_version: __le16,
    pub group_cipher_suite: rtw89_cipher_suite,
    pub pairwise_cipher_suite_cnt: __le16,
    pub pairwise_cipher_suite: rtw89_cipher_suite,
    pub akm_cipher_suite_cnt: __le16,
    pub akm_cipher_suite: rtw89_cipher_suite,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_cipher_info {
    pub cipher: u32,
    pub fw_alg: u8,
    pub len: ieee80211_key_len,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_set_key_info_iter_data {
    pub gtk_cipher: u32,
    pub igtk_cipher: u32,
    pub rx_ready: bool,
    pub error: bool,
    pub tkip_gtk_swapped: bool,
}

extern "C" {
    pub fn __rtw89_wow_parse_akm(rtwdev: *mut rtw89_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn rtw89_wow_suspend(rtwdev: *mut rtw89_dev, wowlan: *mut cfg80211_wowlan) -> c_int;
}
extern "C" {
    pub fn rtw89_wow_resume(rtwdev: *mut rtw89_dev) -> c_int;
}

