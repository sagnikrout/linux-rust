//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/debug.h
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
// Copyright(c) 2019-2020  Realtek Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_debug_mask {
    RTW89_DBG_TXRX = BIT(0),
    RTW89_DBG_RFK = BIT(1),
    RTW89_DBG_RFK_TRACK = BIT(2),
    RTW89_DBG_CFO = BIT(3),
    RTW89_DBG_TSSI = BIT(4),
    RTW89_DBG_TXPWR = BIT(5),
    RTW89_DBG_HCI = BIT(6),
    RTW89_DBG_RA = BIT(7),
    RTW89_DBG_REGD = BIT(8),
    RTW89_DBG_PHY_TRACK = BIT(9),
    RTW89_DBG_DIG = BIT(10),
    RTW89_DBG_SER = BIT(11),
    RTW89_DBG_FW = BIT(12),
    RTW89_DBG_BTC = BIT(13),
    RTW89_DBG_BF = BIT(14),
    RTW89_DBG_HW_SCAN = BIT(15),
    RTW89_DBG_SAR = BIT(16),
    RTW89_DBG_STATE = BIT(17),
    RTW89_DBG_WOW = BIT(18),
    RTW89_DBG_UL_TB = BIT(19),
    RTW89_DBG_CHAN = BIT(20),
    RTW89_DBG_ACPI = BIT(21),
    RTW89_DBG_EDCCA = BIT(22),
    RTW89_DBG_PS = BIT(23),
    RTW89_DBG_LED = BIT(24),

    RTW89_DBG_UNEXP = BIT(31),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_debug_mac_reg_sel {
    RTW89_DBG_SEL_MAC_00,
    RTW89_DBG_SEL_MAC_30,
    RTW89_DBG_SEL_MAC_40,
    RTW89_DBG_SEL_MAC_80,
    RTW89_DBG_SEL_MAC_C0,
    RTW89_DBG_SEL_MAC_E0,
    RTW89_DBG_SEL_BB,
    RTW89_DBG_SEL_IQK,
    RTW89_DBG_SEL_RFC,
}

extern "C" {
    pub fn rtw89_debugfs_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_debugfs_deinit(rtwdev: *mut rtw89_dev);
}

