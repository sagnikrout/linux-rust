//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/btcoexist/halbtc8192e2ant.h
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
//
// The following is for 8192E 2Ant BT Co-exist definition
//

pub const BTC_RSSI_COEX_THRESH_TOL_8192E_2ANT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_info_src_8192e_2ant {
    BT_INFO_SRC_8192E_2ANT_WIFI_FW			= 0x0,
    BT_INFO_SRC_8192E_2ANT_BT_RSP			= 0x1,
    BT_INFO_SRC_8192E_2ANT_BT_ACTIVE_SEND		= 0x2,
    BT_INFO_SRC_8192E_2ANT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_8192e_2ant_bt_status {
    BT_8192E_2ANT_BT_STATUS_NON_CONNECTED_IDLE	= 0x0,
    BT_8192E_2ANT_BT_STATUS_CONNECTED_IDLE		= 0x1,
    BT_8192E_2ANT_BT_STATUS_INQ_PAGE		= 0x2,
    BT_8192E_2ANT_BT_STATUS_ACL_BUSY		= 0x3,
    BT_8192E_2ANT_BT_STATUS_SCO_BUSY		= 0x4,
    BT_8192E_2ANT_BT_STATUS_ACL_SCO_BUSY		= 0x5,
    BT_8192E_2ANT_BT_STATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_8192e_2ant_coex_algo {
    BT_8192E_2ANT_COEX_ALGO_UNDEFINED		= 0x0,
    BT_8192E_2ANT_COEX_ALGO_SCO			= 0x1,
    BT_8192E_2ANT_COEX_ALGO_SCO_PAN			= 0x2,
    BT_8192E_2ANT_COEX_ALGO_HID			= 0x3,
    BT_8192E_2ANT_COEX_ALGO_A2DP			= 0x4,
    BT_8192E_2ANT_COEX_ALGO_A2DP_PANHS		= 0x5,
    BT_8192E_2ANT_COEX_ALGO_PANEDR			= 0x6,
    BT_8192E_2ANT_COEX_ALGO_PANHS			= 0x7,
    BT_8192E_2ANT_COEX_ALGO_PANEDR_A2DP		= 0x8,
    BT_8192E_2ANT_COEX_ALGO_PANEDR_HID		= 0x9,
    BT_8192E_2ANT_COEX_ALGO_HID_A2DP_PANEDR		= 0xa,
    BT_8192E_2ANT_COEX_ALGO_HID_A2DP		= 0xb,
    BT_8192E_2ANT_COEX_ALGO_MAX			= 0xc
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_dm_8192e_2ant {
// fw mechanism
    pub pre_dec_bt_pwr: u8,
    pub cur_dec_bt_pwr: u8,
    pub pre_fw_dac_swing_lvl: u8,
    pub cur_fw_dac_swing_lvl: u8,
    pub cur_ignore_wlan_act: bool,
    pub pre_ignore_wlan_act: bool,
    pub pre_ps_tdma: u8,
    pub cur_ps_tdma: u8,
    pub ps_tdma_para: [u8; 5],
    pub tdma_adj_type: u8,
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
    pub /: *mut *mut u32 backup_arfr_cnt1; / Auto Rate Fallback Retry cnt,
    pub /: *mut *mut u32 backup_arfr_cnt2; / Auto Rate Fallback Retry cnt,
    pub backup_retry_limit: u16,
    pub backup_ampdu_maxtime: u8,
// algorithm related
    pub pre_algorithm: u8,
    pub cur_algorithm: u8,
    pub bt_status: u8,
    pub wifi_chnl_info: [u8; 3],
    pub pre_ss_type: u8,
    pub cur_ss_type: u8,
    pub pre_ra_mask: u32,
    pub cur_ra_mask: u32,
    pub cur_ra_mask_type: u8,
    pub pre_arfr_type: u8,
    pub cur_arfr_type: u8,
    pub pre_retry_limit_type: u8,
    pub cur_retry_limit_type: u8,
    pub pre_ampdu_time_type: u8,
    pub cur_ampdu_time_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_sta_8192e_2ant {
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
    pub pre_bt_rssi_state: u8,
    pub pre_wifi_rssi_state: [u8; 4],
    pub c2h_bt_info_req_sent: bool,
    pub bt_info_c2h: [u8; BT_INFO_SRC_8192E_2ANT_MAX][10],
    pub bt_info_c2h_cnt: [u32; BT_INFO_SRC_8192E_2ANT_MAX],
    pub c2h_bt_inquiry_page: bool,
    pub bt_retry_cnt: u8,
    pub bt_info_ext: u8,
}

//
// The following is interface which will notify coex module.
//
extern "C" {
    pub fn ex_btc8192e2ant_init_hwconfig(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn ex_btc8192e2ant_init_coex_dm(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn ex_btc8192e2ant_ips_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8192e2ant_lps_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8192e2ant_scan_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8192e2ant_connect_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8192e2ant_halt_notify(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn ex_btc8192e2ant_periodical(btcoexist: *mut btc_coexist);
}
