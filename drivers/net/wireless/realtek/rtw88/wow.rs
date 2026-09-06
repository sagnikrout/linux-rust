//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/wow.h
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
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const PNO_CHECK_BYTE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_wow_pattern_type {
    RTW_PATTERN_BROADCAST = 0,
    RTW_PATTERN_MULTICAST,
    RTW_PATTERN_UNICAST,
    RTW_PATTERN_VALID,
    RTW_PATTERN_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_wake_reason {
    RTW_WOW_RSN_RX_PTK_REKEY = 0x1,
    RTW_WOW_RSN_RX_GTK_REKEY = 0x2,
    RTW_WOW_RSN_RX_DEAUTH = 0x8,
    RTW_WOW_RSN_DISCONNECT = 0x10,
    RTW_WOW_RSN_RX_MAGIC_PKT = 0x21,
    RTW_WOW_RSN_RX_PATTERN_MATCH = 0x23,
    RTW_WOW_RSN_RX_NLO = 0x55,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fw_media_status_iter_data {
    pub rtwdev: *mut rtw_dev,
    pub connect: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fw_key_type_iter_data {
    pub rtwdev: *mut rtw_dev,
    pub group_key_type: u8,
    pub pairwise_key_type: u8,
}

extern "C" {
    pub fn rtw_wow_suspend(rtwdev: *mut rtw_dev, wowlan: *mut cfg80211_wowlan) -> c_int;
}
extern "C" {
    pub fn rtw_wow_resume(rtwdev: *mut rtw_dev) -> c_int;
}
