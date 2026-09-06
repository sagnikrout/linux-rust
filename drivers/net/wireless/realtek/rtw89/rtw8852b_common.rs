//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/rtw8852b_common.h
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

pub const RF_PATH_NUM_8852BX: c_int = 2;
pub const BB_PATH_NUM_8852BX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw8852bx_pmac_mode {
    NONE_TEST,
    PKTS_TX,
    PKTS_RX,
    CONT_TX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852bx_u_efuse {
    pub rsvd: [u8; 0x88],
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852bx_e_efuse {
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852bx_tssi_offset {
    pub cck_tssi: [u8; TSSI_CCK_CH_GROUP_NUM],
    pub bw40_tssi: [u8; TSSI_MCS_2G_CH_GROUP_NUM],
    pub rsvd: [u8; 7],
    pub bw40_1s_tssi_5g: [u8; TSSI_MCS_5G_CH_GROUP_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852bx_efuse {
    pub rsvd: [u8; 0x210],
    pub path_a_tssi: rtw8852bx_tssi_offset,
    pub rsvd1: [u8; 10],
    pub path_b_tssi: rtw8852bx_tssi_offset,
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
    pub path_a_cck_pwr_idx: [u8; 6],
    pub path_a_bw40_1tx_pwr_idx: [u8; 5],
    pub path_a_ofdm_1tx_pwr_idx_diff:4: u8,
    pub path_a_bw20_1tx_pwr_idx_diff:4: u8,
    pub path_a_bw20_2tx_pwr_idx_diff:4: u8,
    pub path_a_bw40_2tx_pwr_idx_diff:4: u8,
    pub path_a_cck_2tx_pwr_idx_diff:4: u8,
    pub path_a_ofdm_2tx_pwr_idx_diff:4: u8,
    pub rsvd14: [u8; 0xf2],
    pub u: rtw8852bx_u_efuse,
    pub e: rtw8852bx_e_efuse,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852bx_bb_pmac_info {
    pub en_pmac_tx:1: u8,
    pub is_cck:1: u8,
    pub mode:3: u8,
    pub rsvd:3: u8,
    pub tx_cnt: u16,
    pub period: u16,
    pub tx_time: u16,
    pub duty_cycle: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852bx_bb_tssi_bak {
    pub tx_path: u8,
    pub rx_path: u8,
    pub p0_rfmode: u32,
    pub p0_rfmode_ftm: u32,
    pub p1_rfmode: u32,
    pub p1_rfmode_ftm: u32,
    pub /: *mut *mut s16 tx_pwr; / S9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8852bx_info {
    pub rtwdev): *mut *mut int (mac_enable_bb_rf)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (mac_disable_bb_rf)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (bb_sethw)(struct rtw89_dev,
    pub phy_idx): *mut *mut *mut void (bb_reset_all)(struct rtw89_dev rtwdev, enum rtw89_phy_idx,
    pub rtwdev): *mut *mut void (bb_cfg_txrx_path)(struct rtw89_dev,
    pub tx_path): *mut *mut *mut void (bb_cfg_tx_path)(struct rtw89_dev rtwdev, u8,
    pub chan): *const rtw89_chan,
    pub rtwdev): *mut *mut void (bb_set_plcp_tx)(struct rtw89_dev,
    pub idx): rtw89_phy_idx,
    pub chan): *const rtw89_phy_idx idx, struct rtw89_chan,
    pub bak): *mut rtw8852bx_bb_tssi_bak,
    pub bak): *const rtw8852bx_bb_tssi_bak,
    pub mode): rtw89_phy_idx idx, u8,
    pub mac_idx): *const *const rtw89_chan chan, u8,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub status): *mut ieee80211_rx_status,
    pub phy_ppdu): *mut rtw89_rx_phy_ppdu,
    pub block): rtw89_efuse_block,
    pub phycap_map): *mut *mut *mut int (read_phycap)(struct rtw89_dev rtwdev, u8,
    pub rtwdev): *mut *mut void (power_trim)(struct rtw89_dev,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): *mut *mut *mut int (init_txpwr_unit)(struct rtw89_dev rtwdev, enum rtw89_phy_idx,
    pub mac_idx): s8 pw_ofst, enum rtw89_mac_idx,
    pub rf_path): *mut *mut *mut u8 (get_thermal)(struct rtw89_dev rtwdev, enum rtw89_rf_path,
    pub path): *mut *mut *mut void (adc_cfg)(struct rtw89_dev rtwdev, u8 bw, u8,
    pub rtwdev): *mut *mut void (btc_init_cfg)(struct rtw89_dev,
    pub state): *mut *mut *mut void (btc_set_wl_pri)(struct rtw89_dev rtwdev, u8 map, bool,
    pub val): *mut *mut *mut s8 (btc_get_bt_rssi)(struct rtw89_dev rtwdev, s8,
    pub rtwdev): *mut *mut void (btc_update_bt_cnt)(struct rtw89_dev,
    pub state): *mut *mut *mut void (btc_wl_s1_standby)(struct rtw89_dev rtwdev, bool,
    pub level): *mut *mut *mut void (btc_set_wl_rx_gain)(struct rtw89_dev rtwdev, u32,
}
