//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/rtw8852c.h
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

pub const RF_PATH_NUM_8852C: c_int = 2;
pub const BB_PATH_NUM_8852C: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852c_u_efuse {
    pub rsvd: [u8; 0x88],
    pub mac_addr: [u8; ETH_ALEN],
    pub rsvd1: [u8; 8],
    pub sn: [u8; RTW89_EFUSE_SN_LEN],
    pub rsvd2: [u8; 29],
    pub uuid: [u8; RTW89_EFUSE_UUID_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852c_e_efuse {
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852c_tssi_offset {
    pub cck_tssi: [u8; TSSI_CCK_CH_GROUP_NUM],
    pub bw40_tssi: [u8; TSSI_MCS_2G_CH_GROUP_NUM],
    pub rsvd: [u8; 7],
    pub bw40_1s_tssi_5g: [u8; TSSI_MCS_5G_CH_GROUP_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852c_efuse {
    pub rsvd: [u8; 0x210],
    pub path_a_tssi: rtw8852c_tssi_offset,
    pub rsvd1: [u8; 10],
    pub path_b_tssi: rtw8852c_tssi_offset,
    pub rsvd2: [u8; 94],
    pub channel_plan: u8,
    pub xtal_k: u8,
    pub rsvd3: u8,
    pub iqk_lck: u8,
    pub rsvd4: [u8; 5],
    pub reg_setting:2: u8,
    pub tx_diversity:1: u8,
    pub rx_diversity:2: u8,
    pub ac_mode:1: u8,
    pub module_type:2: u8,
    pub rsvd5: u8,
    pub shared_ant:1: u8,
    pub coex_type:3: u8,
    pub ant_iso:1: u8,
    pub radio_on_off:1: u8,
    pub rsvd6:2: u8,
    pub eeprom_version: u8,
    pub customer_id: u8,
    pub tx_bb_swing_2g: u8,
    pub tx_bb_swing_5g: u8,
    pub tx_cali_pwr_trk_mode: u8,
    pub trx_path_selection: u8,
    pub rfe_type: u8,
    pub country_code: [u8; 2],
    pub rsvd7: [u8; 3],
    pub path_a_therm: u8,
    pub path_b_therm: u8,
    pub rsvd8: [u8; 2],
    pub rx_gain_2g_ofdm: u8,
    pub rsvd9: u8,
    pub rx_gain_2g_cck: u8,
    pub rsvd10: u8,
    pub rx_gain_5g_low: u8,
    pub rsvd11: u8,
    pub rx_gain_5g_mid: u8,
    pub rsvd12: u8,
    pub rx_gain_5g_high: u8,
    pub rsvd13: [u8; 35],
    pub bw40_1s_tssi_6g_a: [u8; TSSI_MCS_6G_CH_GROUP_NUM],
    pub rsvd14: [u8; 10],
    pub bw40_1s_tssi_6g_b: [u8; TSSI_MCS_6G_CH_GROUP_NUM],
    pub rsvd15: [u8; 94],
    pub rx_gain_6g_l0: u8,
    pub rsvd16: u8,
    pub rx_gain_6g_l1: u8,
    pub rsvd17: u8,
    pub rx_gain_6g_m0: u8,
    pub rsvd18: u8,
    pub rx_gain_6g_m1: u8,
    pub rsvd19: u8,
    pub rx_gain_6g_h0: u8,
    pub rsvd20: u8,
    pub rx_gain_6g_h1: u8,
    pub rsvd21: u8,
    pub rx_gain_6g_uh0: u8,
    pub rsvd22: u8,
    pub rx_gain_6g_uh1: u8,
    pub rsvd23: u8,
    pub channel_plan_6g: u8,
    pub rsvd24: [u8; 71],
    pub u: rtw8852c_u_efuse,
    pub e: rtw8852c_e_efuse,
}
