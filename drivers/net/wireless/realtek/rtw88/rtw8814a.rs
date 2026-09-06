//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/rtw8814a.h
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
// Copyright(c) 2025  Realtek Corporation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8814au_efuse {
    pub /: *mut *mut u8 vid[2]; / 0xd0,
    pub /: *mut *mut u8 pid[2]; / 0xd2,
    pub /: *mut *mut u8 res[4]; / 0xd4,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8814ae_efuse {
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd0,
    pub /: *mut *mut u8 vid[2]; / 0xd6,
    pub /: *mut *mut u8 did[2]; / 0xd8,
    pub /: *mut *mut u8 svid[2]; / 0xda,
    pub /: *mut *mut u8 smid[2]; / 0xdc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8814a_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x0c],
    pub /: *mut *mut u8 usb_mode; / 0x0e,
    pub res1: u8,
// power index for four RF paths
    pub txpwr_idx_table: [rtw_txpwr_idx; 4],
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub /: *mut *mut u8 xtal_k; / 0xb9,
    pub /: *mut *mut u8 thermal_meter; / 0xba,
    pub /: *mut *mut u8 iqk_lck; / 0xbb,
    pub /: *mut *mut u8 pa_type; / 0xbc,
    pub /: *mut *mut u8 lna_type_2g[2]; / 0xbd,
    pub /: *mut *mut u8 lna_type_5g[2]; / 0xbf,
    pub /: *mut *mut u8 rf_board_option; / 0xc1,
    pub res2: u8,
    pub /: *mut *mut u8 rf_bt_setting; / 0xc3,
    pub /: *mut *mut u8 eeprom_version; / 0xc4,
    pub /: *mut *mut u8 eeprom_customer_id; / 0xc5,
    pub /: *mut *mut u8 tx_bb_swing_setting_2g; / 0xc6,
    pub /: *mut *mut u8 tx_bb_swing_setting_5g; / 0xc7,
    pub res3: u8,
    pub /: *mut *mut u8 trx_antenna_option; / 0xc9,
    pub /: *mut *mut u8 rfe_option; / 0xca,
    pub /: *mut *mut u8 country_code[2]; / 0xcb,
    pub res4: [u8; 3],
    pub u: rtw8814au_efuse,
    pub e: rtw8814ae_efuse,
}
