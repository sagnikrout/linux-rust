//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/btcoexist/halbtc8821a2ant.h
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
// Copyright(c) 2012  Realtek Corporation.
// ===========================================
// The following is for 8821A 2Ant BT Co-exist definition
// ===========================================
//

pub const BTC_RSSI_COEX_THRESH_TOL_8821A_2ANT: c_int = 2;
// WiFi RSSI Threshold for 2-Ant TDMA/1-Ant PS-TDMA translation
pub const BT_8821A_2ANT_WIFI_RSSI_COEXSWITCH_THRES: c_int = 42;
// BT RSSI Threshold for 2-Ant TDMA/1-Ant PS-TDMA translation
pub const BT_8821A_2ANT_BT_RSSI_COEXSWITCH_THRES: c_int = 46;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BT_INFO_SRC_8821A_2ANT {
    BT_INFO_SRC_8821A_2ANT_WIFI_FW		= 0x0,
    BT_INFO_SRC_8821A_2ANT_BT_RSP		= 0x1,
    BT_INFO_SRC_8821A_2ANT_BT_ACTIVE_SEND	= 0x2,
    BT_INFO_SRC_8821A_2ANT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BT_8821A_2ANT_BT_STATUS {
    BT_8821A_2ANT_BT_STATUS_IDLE		= 0x0,
    BT_8821A_2ANT_BT_STATUS_CON_IDLE	= 0x1,
    BT_8821A_2ANT_BT_STATUS_NON_IDLE	= 0x2,
    BT_8821A_2ANT_BT_STATUS_ACL_BUSY	= 0x3,
    BT_8821A_2ANT_BT_STATUS_SCO_BUSY	= 0x4,
    BT_8821A_2ANT_BT_STATUS_ACL_SCO_BUSY	= 0x5,
    BT_8821A_2ANT_BT_STATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BT_8821A_2ANT_COEX_ALGO {
    BT_8821A_2ANT_COEX_ALGO_UNDEFINED		= 0x0,
    BT_8821A_2ANT_COEX_ALGO_SCO			= 0x1,
    BT_8821A_2ANT_COEX_ALGO_HID			= 0x2,
    BT_8821A_2ANT_COEX_ALGO_A2DP			= 0x3,
    BT_8821A_2ANT_COEX_ALGO_A2DP_PANHS		= 0x4,
    BT_8821A_2ANT_COEX_ALGO_PANEDR			= 0x5,
    BT_8821A_2ANT_COEX_ALGO_PANHS			= 0x6,
    BT_8821A_2ANT_COEX_ALGO_PANEDR_A2DP		= 0x7,
    BT_8821A_2ANT_COEX_ALGO_PANEDR_HID		= 0x8,
    BT_8821A_2ANT_COEX_ALGO_HID_A2DP_PANEDR		= 0x9,
    BT_8821A_2ANT_COEX_ALGO_HID_A2DP		= 0xa,
    BT_8821A_2ANT_COEX_ALGO_MAX			= 0xb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_dm_8821a_2ant {
// fw mechanism
    pub pre_dec_bt_pwr_lvl: bool,
    pub cur_dec_bt_pwr_lvl: bool,
    pub pre_fw_dac_swing_lvl: u8,
    pub cur_fw_dac_swing_lvl: u8,
    pub cur_ignore_wlan_act: bool,
    pub pre_ignore_wlan_act: bool,
    pub pre_ps_tdma: u8,
    pub cur_ps_tdma: u8,
    pub ps_tdma_para: [u8; 5],
    pub ps_tdma_du_adj_type: u8,
    pub reset_tdma_adjust: bool,
    pub auto_tdma_adjust: bool,
    pub pre_ps_tdma_on: bool,
    pub cur_ps_tdma_on: bool,
    pub pre_bt_auto_report: bool,
    pub cur_bt_auto_report: bool,
// sw mechanism
    pub pre_rf_rx_lpf_shrink: bool,
    pub cur_rf_rx_lpf_shrink: bool,
    pub bt_rf0x1e_backup: u32,
    pub pre_low_penalty_ra: bool,
    pub cur_low_penalty_ra: bool,
    pub pre_dac_swing_on: bool,
    pub pre_dac_swing_lvl: u32,
    pub cur_dac_swing_on: bool,
    pub cur_dac_swing_lvl: u32,
    pub pre_adc_back_off: bool,
    pub cur_adc_back_off: bool,
    pub pre_agc_table_en: bool,
    pub cur_agc_table_en: bool,
    pub pre_val0x6c0: u32,
    pub cur_val0x6c0: u32,
    pub pre_val0x6c4: u32,
    pub cur_val0x6c4: u32,
    pub pre_val0x6c8: u32,
    pub cur_val0x6c8: u32,
    pub pre_val0x6cc: u8,
    pub cur_val0x6cc: u8,
    pub limited_dig: bool,
// algorithm related
    pub pre_algorithm: u8,
    pub cur_algorithm: u8,
    pub bt_status: u8,
    pub wifi_chnl_info: [u8; 3],
    pub pre_lps: u8,
    pub cur_lps: u8,
    pub pre_rpwm: u8,
    pub cur_rpwm: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_sta_8821a_2ant {
    pub bt_link_exist: bool,
    pub sco_exist: bool,
    pub a2dp_exist: bool,
    pub hid_exist: bool,
    pub pan_exist: bool,
    pub under_lps: bool,
    pub under_ips: bool,
    pub high_priority_tx: u32,
    pub high_priority_rx: u32,
    pub low_priority_tx: u32,
    pub low_priority_rx: u32,
    pub bt_rssi: u8,
    pub bt_tx_rx_mask: bool,
    pub pre_bt_rssi_state: u8,
    pub pre_wifi_rssi_state: [u8; 4],
    pub c2h_bt_info_req_sent: bool,
    pub bt_info_c2h: [u8; BT_INFO_SRC_8821A_2ANT_MAX][10],
    pub bt_info_c2h_cnt: [u32; BT_INFO_SRC_8821A_2ANT_MAX],
    pub c2h_bt_inquiry_page: bool,
    pub bt_retry_cnt: u8,
    pub bt_info_ext: u8,
    pub crc_ok_cck: u32,
    pub crc_ok_11g: u32,
    pub crc_ok_11n: u32,
    pub crc_ok_11n_agg: u32,
    pub crc_err_cck: u32,
    pub crc_err_11g: u32,
    pub crc_err_11n: u32,
    pub crc_err_11n_agg: u32,
    pub coex_table_type: u8,
    pub force_lps_on: bool,
    pub dis_ver_info_cnt: u8,
}

// ===========================================
// The following is interface which will notify coex module.
// ===========================================
//
extern "C" {
    pub fn ex_btc8821a2ant_pnp_notify(btcoexist: *mut btc_coexist, pnp_state: u8);
}
extern "C" {
    pub fn ex_btc8821a2ant_pre_load_firmware(btcoexist: *mut btc_coexist);
}
