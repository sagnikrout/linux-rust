//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/rtw88xxa.h
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
// Copyright(c) 2024  Realtek Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8821au_efuse {
    pub /: *mut *mut u8 res4[48]; / 0xd0,
    pub /: *mut *mut u8 vid[2]; / 0x100,
    pub pid: [u8; 2],
    pub res8: [u8; 3],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x107,
    pub res9: [u8; 243],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8812au_efuse {
    pub /: *mut *mut u8 vid[2]; / 0xd0,
    pub /: *mut *mut u8 pid[2]; / 0xd2,
    pub res0: [u8; 3],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd7,
    pub res1: [u8; 291],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw88xxa_efuse {
    pub rtl_id: __le16,
    pub /: *mut *mut u8 res0[6]; / 0x02,
    pub /: *mut *mut u8 usb_mode; / 0x08,
    pub /: *mut *mut u8 res1[7]; / 0x09,
// power index for four RF paths
    pub txpwr_idx_table: [rtw_txpwr_idx; 4],
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub xtal_k: u8,
    pub thermal_meter: u8,
    pub iqk_lck: u8,
    pub /: *mut *mut u8 pa_type; / 0xbc,
    pub /: *mut *mut u8 lna_type_2g; / 0xbd,
    pub res2: u8,
    pub /: *mut *mut u8 lna_type_5g; / 0xbf,
    pub res3: u8,
    pub /: *mut *mut u8 rf_board_option; / 0xc1,
    pub rf_feature_option: u8,
    pub rf_bt_setting: u8,
    pub eeprom_version: u8,
    pub /: *mut *mut u8 eeprom_customer_id; / 0xc5,
    pub tx_bb_swing_setting_2g: u8,
    pub tx_bb_swing_setting_5g: u8,
    pub tx_pwr_calibrate_rate: u8,
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub rfe_option: u8,
    pub country_code: [u8; 2],
    pub res4: [u8; 3],
    pub rtw8821au: rtw8821au_efuse,
    pub rtw8812au: rtw8812au_efuse,
}

pub const WLAN_BCN_DMA_TIME: c_uint = 0x02;
pub const WLAN_TBTT_PROHIBIT: c_uint = 0x04;
pub const WLAN_TBTT_HOLD_TIME: c_uint = 0x064;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_jaguar_phy_status_rpt {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub __packed: },

// CCK:

// OFDM:

// Stream 1 and 2 RX EVM:

// 8812a, stream 1 and 2 CSI:

// 8814a:

    pub on): *mut *mut void rtw88xxa_efuse_grant(struct rtw_dev rtwdev, bool,
    pub log_map): *mut *mut int rtw88xxa_read_efuse(struct rtw_dev rtwdev, u8,
    pub enter_lps_flow): *const *const rtw_pwr_seq_cmd,
    pub rtwdev): *mut int rtw88xxa_power_on(struct rtw_dev,
    pub mask): rtw_rf_path rf_path, u32 addr, u32,
    pub primary_chan_idx): u8,
    pub vga_idx)): *mut *mut s8 (cck_rx_pwr)(u8 lna_idx, u8,
    pub rtwdev): *mut void rtw88xxa_set_tx_power_index(struct rtw_dev,
    pub rtwdev): *mut void rtw88xxa_false_alarm_statistics(struct rtw_dev,
    pub macbb_num): u32,
    pub afe_num): *const *const u32 backup_afe_reg, u32,
    pub macbb_num): u32,
    pub rtwdev): *mut void rtw88xxa_iqk_configure_mac(struct rtw_dev,
    pub break_outer): bool break_inner, bool,
    pub rtwdev)): *mut *mut void (do_iqk)(struct rtw_dev,
    pub new_lvl): *mut *mut void rtw88xxa_phy_cck_pd_set(struct rtw_dev rtwdev, u8,
