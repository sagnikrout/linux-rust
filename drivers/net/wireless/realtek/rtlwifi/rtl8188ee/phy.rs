//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8188ee/phy.h
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
// Copyright(c) 2009-2013  Realtek Corporation.
// MAX_TX_COUNT must always set to 4, otherwise read efuse
// table secquence will be wrong.
//
pub const MAX_TX_COUNT: c_int = 4;
pub const MAX_PRECMD_CNT: c_int = 16;
pub const MAX_RFDEPENDCMD_CNT: c_int = 16;
pub const MAX_POSTCMD_CNT: c_int = 16;
pub const MAX_DOZE_WAITING_TIMES_9x: c_int = 64;

pub const HIGHPOWER_RADIOA_ARRAYLEN: c_int = 22;
pub const IQK_ADDA_REG_NUM: c_int = 16;
pub const IQK_BB_REG_NUM: c_int = 9;
pub const MAX_TOLERANCE: c_int = 5;
pub const IQK_DELAY_TIME: c_int = 10;
pub const INDEX_MAPPING_NUM: c_int = 15;
pub const APK_BB_REG_NUM: c_int = 5;
pub const APK_AFE_REG_NUM: c_int = 16;
pub const APK_CURVE_REG_NUM: c_int = 4;
pub const PATH_NUM: c_int = 2;
pub const LOOP_LIMIT: c_int = 5;
pub const MAX_STALL_TIME: c_int = 50;
pub const ANTENNADIVERSITYVALUE: c_uint = 0x80;
pub const MAX_TXPWR_IDX_NMODE_92S: c_int = 63;
pub const RESET_CNT_LIMIT: c_int = 3;
pub const IQK_ADDA_REG_NUM: c_int = 16;
pub const IQK_MAC_REG_NUM: c_int = 4;
pub const RF6052_MAX_PATH: c_int = 2;

pub const CT_OFFSET_CCK_TX_PWR_IDX: c_uint = 0x5A;
pub const CT_OFFSET_HT401S_TX_PWR_IDX: c_uint = 0x60;
pub const CT_OFFSET_HT402S_TX_PWR_IDX_DIFF: c_uint = 0x66;
pub const CT_OFFSET_HT20_TX_PWR_IDX_DIFF: c_uint = 0x69;
pub const CT_OFFSET_OFDM_TX_PWR_IDX_DIFF: c_uint = 0x6C;
pub const CT_OFFSET_HT40_MAX_PWR_OFFSET: c_uint = 0x6F;
pub const CT_OFFSET_HT20_MAX_PWR_OFFSET: c_uint = 0x72;
pub const CT_OFFSET_CHANNEL_PLAH: c_uint = 0x75;
pub const CT_OFFSET_THERMAL_METER: c_uint = 0x78;
pub const CT_OFFSET_RF_OPTION: c_uint = 0x79;
pub const CT_OFFSET_VERSION: c_uint = 0x7E;
pub const CT_OFFSET_CUSTOMER_ID: c_uint = 0x7F;
pub const RTL92C_MAX_PATH_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum swchnlcmd_id {
    CMDID_END,
    CMDID_SET_TXPOWEROWER_LEVEL,
    CMDID_BBREGWRITE10,
    CMDID_WRITEPORT_ULONG,
    CMDID_WRITEPORT_USHORT,
    CMDID_WRITEPORT_UCHAR,
    CMDID_RF_WRITEREG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swchnlcmd {
    pub cmdid: swchnlcmd_id,
    pub para1: u32,
    pub para2: u32,
    pub msdelay: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw90_block_e {
    HW90_BLOCK_MAC = 0,
    HW90_BLOCK_PHY0 = 1,
    HW90_BLOCK_PHY1 = 2,
    HW90_BLOCK_RF = 3,
    HW90_BLOCK_MAXIMUM = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum baseband_config_type {
    BASEBAND_CONFIG_PHY_REG = 0,
    BASEBAND_CONFIG_AGC_TAB = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ra_offset_area {
    RA_OFFSET_LEGACY_OFDM1,
    RA_OFFSET_LEGACY_OFDM2,
    RA_OFFSET_HT_OFDM1,
    RA_OFFSET_HT_OFDM2,
    RA_OFFSET_HT_OFDM3,
    RA_OFFSET_HT_OFDM4,
    RA_OFFSET_HT_CCK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum antenna_path {
    ANTENNA_NONE,
    ANTENNA_D,
    ANTENNA_C,
    ANTENNA_CD,
    ANTENNA_B,
    ANTENNA_BD,
    ANTENNA_BC,
    ANTENNA_BCD,
    ANTENNA_A,
    ANTENNA_AD,
    ANTENNA_AC,
    ANTENNA_ACD,
    ANTENNA_AB,
    ANTENNA_ABD,
    ANTENNA_ABC,
    ANTENNA_ABCD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r_antenna_select_ofdm {
    pub r_tx_antenna:4: u32,
    pub r_ant_l:4: u32,
    pub r_ant_non_ht:4: u32,
    pub r_ant_ht1:4: u32,
    pub r_ant_ht2:4: u32,
    pub r_ant_ht_s1:4: u32,
    pub r_ant_non_ht_s1:4: u32,
    pub ofdm_txsc:2: u32,
    pub reserved:2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r_antenna_select_cck {
    pub r_cckrx_enable_2:2: u8,
    pub r_cckrx_enable:2: u8,
    pub r_ccktx_enable:4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efuse_contents {
    pub mac_addr: [u8; ETH_ALEN],
    pub cck_tx_power_idx: [u8; 6],
    pub ht40_1s_tx_power_idx: [u8; 6],
    pub ht40_2s_tx_power_idx_diff: [u8; 3],
    pub ht20_tx_power_idx_diff: [u8; 3],
    pub ofdm_tx_power_idx_diff: [u8; 3],
    pub ht40_max_power_offset: [u8; 3],
    pub ht20_max_power_offset: [u8; 3],
    pub channel_plan: u8,
    pub thermal_meter: u8,
    pub rf_option: [u8; 5],
    pub version: u8,
    pub oem_id: u8,
    pub regulatory: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_power_struct {
    pub cck: [u8; RTL92C_MAX_PATH_NUM][CHANNEL_MAX_NUMBER],
    pub ht40_1s: [u8; RTL92C_MAX_PATH_NUM][CHANNEL_MAX_NUMBER],
    pub ht40_2s: [u8; RTL92C_MAX_PATH_NUM][CHANNEL_MAX_NUMBER],
    pub ht20_diff: [u8; RTL92C_MAX_PATH_NUM][CHANNEL_MAX_NUMBER],
    pub legacy_ht_diff: [u8; RTL92C_MAX_PATH_NUM][CHANNEL_MAX_NUMBER],
    pub legacy_ht_txpowerdiff: u8,
    pub groupht20: [u8; RTL92C_MAX_PATH_NUM][CHANNEL_MAX_NUMBER],
    pub groupht40: [u8; RTL92C_MAX_PATH_NUM][CHANNEL_MAX_NUMBER],
    pub pwrgroup_cnt: u8,
    pub mcs_original_offset: [u32; 4][16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _ANT_DIV_TYPE {
    NO_ANTDIV				= 0xFF,
    CG_TRX_HW_ANTDIV		= 0x01,
    CGCS_RX_HW_ANTDIV		= 0x02,
    FIXED_HW_ANTDIV         = 0x03,
    CG_TRX_SMART_ANTDIV		= 0x04,
    CGCS_RX_SW_ANTDIV		= 0x05,
}

extern "C" {
    pub fn rtl88e_phy_mac_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl88e_phy_bb_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl88e_phy_rf_config(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl88e_phy_get_hw_reg_originalvalue(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl88e_phy_set_txpower_level(hw: *mut ieee80211_hw, channel: u8);
}
extern "C" {
    pub fn rtl88e_phy_set_bw_mode_callback(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl88e_phy_sw_chnl_callback(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl88e_phy_sw_chnl(hw: *mut ieee80211_hw) -> u8;
}
extern "C" {
    pub fn rtl88e_phy_iq_calibrate(hw: *mut ieee80211_hw, b_recovery: bool);
}
extern "C" {
    pub fn rtl88e_phy_lc_calibrate(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl88e_phy_set_rfpath_switch(hw: *mut ieee80211_hw, bmain: bool);
}
extern "C" {
    pub fn rtl88e_phy_set_io_cmd(hw: *mut ieee80211_hw, iotype: io_type) -> bool;
}
