//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/rtw8851b.h
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
// Copyright(c) 2022-2023  Realtek Corporation
//

pub const RF_PATH_NUM_8851B: c_int = 1;
pub const BB_PATH_NUM_8851B: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8851bu_efuse {
    pub rsvd: [u8; 0x88],
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8851be_efuse {
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8851b_tssi_offset {
    pub cck_tssi: [u8; TSSI_CCK_CH_GROUP_NUM],
    pub bw40_tssi: [u8; TSSI_MCS_2G_CH_GROUP_NUM],
    pub rsvd: [u8; 7],
    pub bw40_1s_tssi_5g: [u8; TSSI_MCS_5G_CH_GROUP_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8851b_efuse {
    pub rsvd: [u8; 0x210],
    pub path_a_tssi: rtw8851b_tssi_offset,
    pub rsvd1: [u8; 136],
    pub channel_plan: u8,
    pub xtal_k: u8,
    pub rsvd2: u8,
    pub iqk_lck: u8,
    pub rsvd3: [u8; 8],
    pub eeprom_version: u8,
    pub customer_id: u8,
    pub tx_bb_swing_2g: u8,
    pub tx_bb_swing_5g: u8,
    pub tx_cali_pwr_trk_mode: u8,
    pub trx_path_selection: u8,
    pub rfe_type: u8,
    pub country_code: [u8; 2],
    pub rsvd4: [u8; 3],
    pub path_a_therm: u8,
    pub rsvd5: [u8; 3],
    pub rx_gain_2g_ofdm: u8,
    pub rsvd6: u8,
    pub rx_gain_2g_cck: u8,
    pub rsvd7: u8,
    pub rx_gain_5g_low: u8,
    pub rsvd8: u8,
    pub rx_gain_5g_mid: u8,
    pub rsvd9: u8,
    pub rx_gain_5g_high: u8,
    pub rsvd10: [u8; 35],
    pub path_a_cck_pwr_idx: [u8; 6],
    pub path_a_bw40_1tx_pwr_idx: [u8; 5],
    pub path_a_ofdm_1tx_pwr_idx_diff:4: u8,
    pub path_a_bw20_1tx_pwr_idx_diff:4: u8,
    pub path_a_bw20_2tx_pwr_idx_diff:4: u8,
    pub path_a_bw40_2tx_pwr_idx_diff:4: u8,
    pub path_a_cck_2tx_pwr_idx_diff:4: u8,
    pub path_a_ofdm_2tx_pwr_idx_diff:4: u8,
    pub rsvd11: [u8; 0xf2],
    pub u: rtw8851bu_efuse,
    pub e: rtw8851be_efuse,
}
