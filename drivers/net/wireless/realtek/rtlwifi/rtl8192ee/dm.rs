//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192ee/dm.h
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
// Copyright(c) 2009-2014  Realtek Corporation.
pub const OFDMCCA_TH: c_int = 500;
pub const BW_IND_BIAS: c_int = 500;
pub const MF_USC: c_int = 2;
pub const MF_LSC: c_int = 1;
pub const MF_USC_LSC: c_int = 0;
pub const MONITOR_TIME: c_int = 30;
pub const MAIN_ANT: c_int = 0;
pub const AUX_ANT: c_int = 1;
pub const MAIN_ANT_CG_TRX: c_int = 1;
pub const AUX_ANT_CG_TRX: c_int = 0;
pub const MAIN_ANT_CGCS_RX: c_int = 0;
pub const AUX_ANT_CGCS_RX: c_int = 1;
// RF REG LIST
pub const DM_REG_RF_MODE_11N: c_uint = 0x00;
pub const DM_REG_RF_0B_11N: c_uint = 0x0B;
pub const DM_REG_CHNBW_11N: c_uint = 0x18;
pub const DM_REG_T_METER_11N: c_uint = 0x24;
pub const DM_REG_RF_25_11N: c_uint = 0x25;
pub const DM_REG_RF_26_11N: c_uint = 0x26;
pub const DM_REG_RF_27_11N: c_uint = 0x27;
pub const DM_REG_RF_2B_11N: c_uint = 0x2B;
pub const DM_REG_RF_2C_11N: c_uint = 0x2C;
pub const DM_REG_RXRF_A3_11N: c_uint = 0x3C;
pub const DM_REG_T_METER_92D_11N: c_uint = 0x42;
pub const DM_REG_T_METER_92E_11N: c_uint = 0x42;
// BB REG LIST
// PAGE 8
pub const DM_REG_BB_CTRL_11N: c_uint = 0x800;
pub const DM_REG_RF_PIN_11N: c_uint = 0x804;
pub const DM_REG_PSD_CTRL_11N: c_uint = 0x808;
pub const DM_REG_TX_ANT_CTRL_11N: c_uint = 0x80C;
pub const DM_REG_BB_PWR_SAV5_11N: c_uint = 0x818;
pub const DM_REG_CCK_RPT_FORMAT_11N: c_uint = 0x824;
pub const DM_REG_RX_DEFUALT_A_11N: c_uint = 0x858;
pub const DM_REG_RX_DEFUALT_B_11N: c_uint = 0x85A;
pub const DM_REG_BB_PWR_SAV3_11N: c_uint = 0x85C;
pub const DM_REG_ANTSEL_CTRL_11N: c_uint = 0x860;
pub const DM_REG_RX_ANT_CTRL_11N: c_uint = 0x864;
pub const DM_REG_PIN_CTRL_11N: c_uint = 0x870;
pub const DM_REG_BB_PWR_SAV1_11N: c_uint = 0x874;
pub const DM_REG_ANTSEL_PATH_11N: c_uint = 0x878;
pub const DM_REG_BB_3WIRE_11N: c_uint = 0x88C;
pub const DM_REG_SC_CNT_11N: c_uint = 0x8C4;
pub const DM_REG_PSD_DATA_11N: c_uint = 0x8B4;
// PAGE 9
pub const DM_REG_ANT_MAPPING1_11N: c_uint = 0x914;
pub const DM_REG_ANT_MAPPING2_11N: c_uint = 0x918;
// PAGE A
pub const DM_REG_CCK_ANTDIV_PARA1_11N: c_uint = 0xA00;
pub const DM_REG_CCK_CCA_11N: c_uint = 0xA0A;
pub const DM_REG_CCK_ANTDIV_PARA2_11N: c_uint = 0xA0C;
pub const DM_REG_CCK_ANTDIV_PARA3_11N: c_uint = 0xA10;
pub const DM_REG_CCK_ANTDIV_PARA4_11N: c_uint = 0xA14;
pub const DM_REG_CCK_FILTER_PARA1_11N: c_uint = 0xA22;
pub const DM_REG_CCK_FILTER_PARA2_11N: c_uint = 0xA23;
pub const DM_REG_CCK_FILTER_PARA3_11N: c_uint = 0xA24;
pub const DM_REG_CCK_FILTER_PARA4_11N: c_uint = 0xA25;
pub const DM_REG_CCK_FILTER_PARA5_11N: c_uint = 0xA26;
pub const DM_REG_CCK_FILTER_PARA6_11N: c_uint = 0xA27;
pub const DM_REG_CCK_FILTER_PARA7_11N: c_uint = 0xA28;
pub const DM_REG_CCK_FILTER_PARA8_11N: c_uint = 0xA29;
pub const DM_REG_CCK_FA_RST_11N: c_uint = 0xA2C;
pub const DM_REG_CCK_FA_MSB_11N: c_uint = 0xA58;
pub const DM_REG_CCK_FA_LSB_11N: c_uint = 0xA5C;
pub const DM_REG_CCK_CCA_CNT_11N: c_uint = 0xA60;
pub const DM_REG_BB_PWR_SAV4_11N: c_uint = 0xA74;
// PAGE B
pub const DM_REG_LNA_SWITCH_11N: c_uint = 0xB2C;
pub const DM_REG_PATH_SWITCH_11N: c_uint = 0xB30;
pub const DM_REG_RSSI_CTRL_11N: c_uint = 0xB38;
pub const DM_REG_CONFIG_ANTA_11N: c_uint = 0xB68;
pub const DM_REG_RSSI_BT_11N: c_uint = 0xB9C;
// PAGE C
pub const DM_REG_OFDM_FA_HOLDC_11N: c_uint = 0xC00;
pub const DM_REG_RX_PATH_11N: c_uint = 0xC04;
pub const DM_REG_TRMUX_11N: c_uint = 0xC08;
pub const DM_REG_OFDM_FA_RSTC_11N: c_uint = 0xC0C;
pub const DM_REG_RXIQI_MATRIX_11N: c_uint = 0xC14;
pub const DM_REG_TXIQK_MATRIX_LSB1_11N: c_uint = 0xC4C;
pub const DM_REG_IGI_A_11N: c_uint = 0xC50;
pub const DM_REG_ANTDIV_PARA2_11N: c_uint = 0xC54;
pub const DM_REG_IGI_B_11N: c_uint = 0xC58;
pub const DM_REG_ANTDIV_PARA3_11N: c_uint = 0xC5C;

pub const DM_REG_BB_PWR_SAV2_11N: c_uint = 0xC70;
pub const DM_REG_RX_OFF_11N: c_uint = 0xC7C;
pub const DM_REG_TXIQK_MATRIXA_11N: c_uint = 0xC80;
pub const DM_REG_TXIQK_MATRIXB_11N: c_uint = 0xC88;
pub const DM_REG_TXIQK_MATRIXA_LSB2_11N: c_uint = 0xC94;
pub const DM_REG_TXIQK_MATRIXB_LSB2_11N: c_uint = 0xC9C;
pub const DM_REG_RXIQK_MATRIX_LSB_11N: c_uint = 0xCA0;
pub const DM_REG_ANTDIV_PARA1_11N: c_uint = 0xCA4;
pub const DM_REG_OFDM_FA_TYPE1_11N: c_uint = 0xCF0;
// PAGE D
pub const DM_REG_OFDM_FA_RSTD_11N: c_uint = 0xD00;
pub const DM_REG_OFDM_FA_TYPE2_11N: c_uint = 0xDA0;
pub const DM_REG_OFDM_FA_TYPE3_11N: c_uint = 0xDA4;
pub const DM_REG_OFDM_FA_TYPE4_11N: c_uint = 0xDA8;
// PAGE E
pub const DM_REG_TXAGC_A_6_18_11N: c_uint = 0xE00;
pub const DM_REG_TXAGC_A_24_54_11N: c_uint = 0xE04;
pub const DM_REG_TXAGC_A_1_MCS32_11N: c_uint = 0xE08;
pub const DM_REG_TXAGC_A_MCS0_3_11N: c_uint = 0xE10;
pub const DM_REG_TXAGC_A_MCS4_7_11N: c_uint = 0xE14;
pub const DM_REG_TXAGC_A_MCS8_11_11N: c_uint = 0xE18;
pub const DM_REG_TXAGC_A_MCS12_15_11N: c_uint = 0xE1C;
pub const DM_REG_FPGA0_IQK_11N: c_uint = 0xE28;
pub const DM_REG_TXIQK_TONE_A_11N: c_uint = 0xE30;
pub const DM_REG_RXIQK_TONE_A_11N: c_uint = 0xE34;
pub const DM_REG_TXIQK_PI_A_11N: c_uint = 0xE38;
pub const DM_REG_RXIQK_PI_A_11N: c_uint = 0xE3C;
pub const DM_REG_TXIQK_11N: c_uint = 0xE40;
pub const DM_REG_RXIQK_11N: c_uint = 0xE44;
pub const DM_REG_IQK_AGC_PTS_11N: c_uint = 0xE48;
pub const DM_REG_IQK_AGC_RSP_11N: c_uint = 0xE4C;
pub const DM_REG_BLUETOOTH_11N: c_uint = 0xE6C;
pub const DM_REG_RX_WAIT_CCA_11N: c_uint = 0xE70;
pub const DM_REG_TX_CCK_RFON_11N: c_uint = 0xE74;
pub const DM_REG_TX_CCK_BBON_11N: c_uint = 0xE78;
pub const DM_REG_OFDM_RFON_11N: c_uint = 0xE7C;
pub const DM_REG_OFDM_BBON_11N: c_uint = 0xE80;
pub const DM_REG_TX2RX_11N: c_uint = 0xE84;
pub const DM_REG_TX2TX_11N: c_uint = 0xE88;
pub const DM_REG_RX_CCK_11N: c_uint = 0xE8C;
pub const DM_REG_RX_OFDM_11N: c_uint = 0xED0;
pub const DM_REG_RX_WAIT_RIFS_11N: c_uint = 0xED4;
pub const DM_REG_RX2RX_11N: c_uint = 0xED8;
pub const DM_REG_STANDBY_11N: c_uint = 0xEDC;
pub const DM_REG_SLEEP_11N: c_uint = 0xEE0;
pub const DM_REG_PMPD_ANAEN_11N: c_uint = 0xEEC;
// MAC REG LIST
pub const DM_REG_BB_RST_11N: c_uint = 0x02;
pub const DM_REG_ANTSEL_PIN_11N: c_uint = 0x4C;
pub const DM_REG_EARLY_MODE_11N: c_uint = 0x4D0;
pub const DM_REG_RSSI_MONITOR_11N: c_uint = 0x4FE;
pub const DM_REG_EDCA_VO_11N: c_uint = 0x500;
pub const DM_REG_EDCA_VI_11N: c_uint = 0x504;
pub const DM_REG_EDCA_BE_11N: c_uint = 0x508;
pub const DM_REG_EDCA_BK_11N: c_uint = 0x50C;
pub const DM_REG_TXPAUSE_11N: c_uint = 0x522;
pub const DM_REG_RESP_TX_11N: c_uint = 0x6D8;
pub const DM_REG_ANT_TRAIN_PARA1_11N: c_uint = 0x7b0;
pub const DM_REG_ANT_TRAIN_PARA2_11N: c_uint = 0x7b4;
// DIG Related
pub const DM_BIT_IGI_11N: c_uint = 0x0000007F;

pub const OFDM_TABLE_LENGTH: c_int = 43;
pub const CCK_TABLE_LENGTH: c_int = 33;
pub const OFDM_TABLE_SIZE: c_int = 43;
pub const CCK_TABLE_SIZE: c_int = 33;
pub const BW_AUTO_SWITCH_HIGH_LOW: c_int = 25;
pub const BW_AUTO_SWITCH_LOW_HIGH: c_int = 30;
pub const DM_DIG_FA_UPPER: c_uint = 0x3e;
pub const DM_DIG_FA_LOWER: c_uint = 0x1e;
pub const DM_DIG_FA_TH0: c_uint = 0x200;
pub const DM_DIG_FA_TH1: c_uint = 0x300;
pub const DM_DIG_FA_TH2: c_uint = 0x400;
pub const RXPATHSELECTION_SS_TH_LOW: c_int = 30;
pub const RXPATHSELECTION_DIFF_TH: c_int = 18;
pub const DM_RATR_STA_INIT: c_int = 0;
pub const DM_RATR_STA_HIGH: c_int = 1;
pub const DM_RATR_STA_MIDDLE: c_int = 2;
pub const DM_RATR_STA_LOW: c_int = 3;
pub const CTS2SELF_THVAL: c_int = 30;
pub const REGC38_TH: c_int = 20;
pub const WAIOTTHVAL: c_int = 25;
pub const TXHIGHPWRLEVEL_NORMAL: c_int = 0;
pub const TXHIGHPWRLEVEL_LEVEL1: c_int = 1;
pub const TXHIGHPWRLEVEL_LEVEL2: c_int = 2;
pub const TXHIGHPWRLEVEL_BT1: c_int = 3;
pub const TXHIGHPWRLEVEL_BT2: c_int = 4;
pub const DM_TYPE_BYFW: c_int = 0;
pub const DM_TYPE_BYDRIVER: c_int = 1;
pub const TX_POWER_NEAR_FIELD_THRESH_LVL2: c_int = 74;
pub const TX_POWER_NEAR_FIELD_THRESH_LVL1: c_int = 67;
pub const TXPWRTRACK_MAX_IDX: c_int = 6;
// Dynamic ATC switch
pub const ATC_STATUS_OFF: c_uint = 0x0	/* enable */;
pub const ATC_STATUS_ON: c_uint = 0x1	/* disable */;

// RSSI Dump Message
pub const RA_RSSIDUMP: c_uint = 0xcb0;
pub const RB_RSSIDUMP: c_uint = 0xcb1;
pub const RS1_RXEVMDUMP: c_uint = 0xcb2;
pub const RS2_RXEVMDUMP: c_uint = 0xcb3;
pub const RA_RXSNRDUMP: c_uint = 0xcb4;
pub const RB_RXSNRDUMP: c_uint = 0xcb5;
pub const RA_CFOSHORTDUMP: c_uint = 0xcb6;
pub const RB_CFOSHORTDUMP: c_uint = 0xcb8;
pub const RA_CFOLONGDUMP: c_uint = 0xcba;
pub const RB_CFOLONGDUMP: c_uint = 0xcbc;
extern "C" {
    pub fn rtl92ee_dm_init(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ee_dm_watchdog(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ee_dm_write_dig(hw: *mut ieee80211_hw, current_igi: u8);
}
extern "C" {
    pub fn rtl92ee_dm_init_edca_turbo(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92ee_dm_init_rate_adaptive_mask(hw: *mut ieee80211_hw);
}
