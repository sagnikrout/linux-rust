//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/btcoexist/halbtc8821a1ant.h
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
// The following is for 8821A 1ANT BT Co-exist definition
// ===========================================
//

pub const BTC_RSSI_COEX_THRESH_TOL_8821A_1ANT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BT_INFO_SRC_8821A_1ANT {
    BT_INFO_SRC_8821A_1ANT_WIFI_FW			= 0x0,
    BT_INFO_SRC_8821A_1ANT_BT_RSP			= 0x1,
    BT_INFO_SRC_8821A_1ANT_BT_ACTIVE_SEND		= 0x2,
    BT_INFO_SRC_8821A_1ANT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BT_8821A_1ANT_BT_STATUS {
    BT_8821A_1ANT_BT_STATUS_NON_CONNECTED_IDLE		= 0x0,
    BT_8821A_1ANT_BT_STATUS_CONNECTED_IDLE			= 0x1,
    BT_8821A_1ANT_BT_STATUS_INQ_PAGE			= 0x2,
    BT_8821A_1ANT_BT_STATUS_ACL_BUSY			= 0x3,
    BT_8821A_1ANT_BT_STATUS_SCO_BUSY			= 0x4,
    BT_8821A_1ANT_BT_STATUS_ACL_SCO_BUSY			= 0x5,
    BT_8821A_1ANT_BT_STATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BT_8821A_1ANT_WIFI_STATUS {
    BT_8821A_1ANT_WIFI_STATUS_NON_CONNECTED_IDLE		= 0x0,
    BT_8821A_1ANT_WIFI_STATUS_NON_CONNECTED_ASSO_AUTH_SCAN	= 0x1,
    BT_8821A_1ANT_WIFI_STATUS_CONNECTED_SCAN		= 0x2,
    BT_8821A_1ANT_WIFI_STATUS_CONNECTED_SPECIAL_PKT		= 0x3,
    BT_8821A_1ANT_WIFI_STATUS_CONNECTED_IDLE		= 0x4,
    BT_8821A_1ANT_WIFI_STATUS_CONNECTED_BUSY		= 0x5,
    BT_8821A_1ANT_WIFI_STATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BT_8821A_1ANT_COEX_ALGO {
    BT_8821A_1ANT_COEX_ALGO_UNDEFINED		= 0x0,
    BT_8821A_1ANT_COEX_ALGO_SCO			= 0x1,
    BT_8821A_1ANT_COEX_ALGO_HID			= 0x2,
    BT_8821A_1ANT_COEX_ALGO_A2DP			= 0x3,
    BT_8821A_1ANT_COEX_ALGO_A2DP_PANHS		= 0x4,
    BT_8821A_1ANT_COEX_ALGO_PANEDR			= 0x5,
    BT_8821A_1ANT_COEX_ALGO_PANHS			= 0x6,
    BT_8821A_1ANT_COEX_ALGO_PANEDR_A2DP		= 0x7,
    BT_8821A_1ANT_COEX_ALGO_PANEDR_HID		= 0x8,
    BT_8821A_1ANT_COEX_ALGO_HID_A2DP_PANEDR		= 0x9,
    BT_8821A_1ANT_COEX_ALGO_HID_A2DP		= 0xa,
    BT_8821A_1ANT_COEX_ALGO_MAX			= 0xb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_dm_8821a_1ant {
// fw mechanism
    pub cur_ignore_wlan_act: bool,
    pub pre_ignore_wlan_act: bool,
    pub pre_ps_tdma: u8,
    pub cur_ps_tdma: u8,
    pub ps_tdma_para: [u8; 5],
    pub tdma_adj_type: u8,
    pub auto_tdma_adjust: bool,
    pub pre_ps_tdma_on: bool,
    pub cur_ps_tdma_on: bool,
    pub pre_bt_auto_report: bool,
    pub cur_bt_auto_report: bool,
    pub pre_lps: u8,
    pub cur_lps: u8,
    pub pre_rpwm: u8,
    pub cur_rpwm: u8,
// sw mechanism
    pub pre_low_penalty_ra: bool,
    pub cur_low_penalty_ra: bool,
    pub pre_val_0x6c0: u32,
    pub cur_val_0x6c0: u32,
    pub pre_val_0x6c4: u32,
    pub cur_val_0x6c4: u32,
    pub pre_val_0x6c8: u32,
    pub cur_val_0x6c8: u32,
    pub pre_val_0x6cc: u8,
    pub cur_val_0x6cc: u8,
// Auto Rate Fallback Retry cnt
    pub backup_arfr_cnt1: u32,
// Auto Rate Fallback Retry cnt
    pub backup_arfr_cnt2: u32,
    pub backup_retry_limit: u16,
    pub backup_ampdu_max_time: u8,
// algorithm related
    pub pre_algorithm: u8,
    pub cur_algorithm: u8,
    pub bt_status: u8,
    pub wifi_chnl_info: [u8; 3],
    pub pre_ra_mask: u32,
    pub cur_ra_mask: u32,
    pub pre_arfr_type: u8,
    pub cur_arfr_type: u8,
    pub pre_retry_limit_type: u8,
    pub cur_retry_limit_type: u8,
    pub pre_ampdu_time_type: u8,
    pub cur_ampdu_time_type: u8,
    pub arp_cnt: u32,
    pub error_condition: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_sta_8821a_1ant {
    pub bt_disabled: bool,
    pub bt_link_exist: bool,
    pub sco_exist: bool,
    pub a2dp_exist: bool,
    pub hid_exist: bool,
    pub pan_exist: bool,
    pub under_lps: bool,
    pub under_ips: bool,
    pub special_pkt_period_cnt: u32,
    pub high_priority_tx: u32,
    pub high_priority_rx: u32,
    pub low_priority_tx: u32,
    pub low_priority_rx: u32,
    pub bt_rssi: u8,
    pub bt_tx_rx_mask: bool,
    pub pre_bt_rssi_state: u8,
    pub pre_wifi_rssi_state: [u8; 4],
    pub c2h_bt_info_req_sent: bool,
    pub bt_info_c2h: [u8; BT_INFO_SRC_8821A_1ANT_MAX][10],
    pub bt_info_c2h_cnt: [u32; BT_INFO_SRC_8821A_1ANT_MAX],
    pub c2h_bt_inquiry_page: bool,
    pub wifi_is_high_pri_task: bool,
    pub bt_retry_cnt: u8,
    pub bt_info_ext: u8,
}

// ===========================================
// The following is interface which will notify coex module.
// ===========================================
//
extern "C" {
    pub fn ex_btc8821a1ant_init_coex_dm(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn ex_btc8821a1ant_ips_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8821a1ant_lps_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8821a1ant_scan_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8821a1ant_connect_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn ex_btc8821a1ant_halt_notify(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn ex_btc8821a1ant_pnp_notify(btcoexist: *mut btc_coexist, pnpstate: u8);
}
extern "C" {
    pub fn ex_btc8821a1ant_periodical(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn ex_btc8821a1ant_pnp_notify(btcoexist: *mut btc_coexist, pnp_state: u8);
}
