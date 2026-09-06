//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/coex.h
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
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const COEX_CCK_2: c_uint = 0x1;
pub const COEX_RESP_ACK_BY_WL_FW: c_uint = 0x1;

pub const COEX_BT_GAMEHID_CNT: c_int = 800;
pub const COEX_RF_OFF: c_uint = 0x0;
pub const COEX_RF_ON: c_uint = 0x1;
pub const COEX_H2C69_WL_LEAKAP: c_uint = 0xc;
pub const PARA1_H2C69_DIS_5MS: c_uint = 0x1;
pub const PARA1_H2C69_EN_5MS: c_uint = 0x0;
pub const COEX_H2C69_TDMA_SLOT: c_uint = 0xb;
pub const PARA1_H2C69_TDMA_4SLOT: c_uint = 0xc1;
pub const PARA1_H2C69_TDMA_2SLOT: c_uint = 0x1;

pub const COEX_H2C69_TOGGLE_TABLE_A: c_uint = 0xd;
pub const COEX_H2C69_TOGGLE_TABLE_B: c_uint = 0x7;

pub const TDMA_TIMER_TYPE_2SLOT: c_int = 0;
pub const TDMA_TIMER_TYPE_4SLOT: c_int = 3;
pub const COEX_RSSI_STEP: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_mp_info_op {
    BT_MP_INFO_OP_PATCH_VER	= 0x00,
    BT_MP_INFO_OP_READ_REG	= 0x11,
    BT_MP_INFO_OP_SUPP_FEAT	= 0x2a,
    BT_MP_INFO_OP_SUPP_VER	= 0x2b,
    BT_MP_INFO_OP_SCAN_TYPE	= 0x2d,
    BT_MP_INFO_OP_LNA_CONSTRAINT	= 0x32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_set_ant_phase {
    COEX_SET_ANT_INIT,
    COEX_SET_ANT_WONLY,
    COEX_SET_ANT_WOFF,
    COEX_SET_ANT_2G,
    COEX_SET_ANT_5G,
    COEX_SET_ANT_POWERON,
    COEX_SET_ANT_2G_WLBT,
    COEX_SET_ANT_2G_FREERUN,

    COEX_SET_ANT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_runreason {
    COEX_RSN_2GSCANSTART	= 0,
    COEX_RSN_5GSCANSTART	= 1,
    COEX_RSN_SCANFINISH	= 2,
    COEX_RSN_2GSWITCHBAND	= 3,
    COEX_RSN_5GSWITCHBAND	= 4,
    COEX_RSN_2GCONSTART	= 5,
    COEX_RSN_5GCONSTART	= 6,
    COEX_RSN_2GCONFINISH	= 7,
    COEX_RSN_5GCONFINISH	= 8,
    COEX_RSN_2GMEDIA	= 9,
    COEX_RSN_5GMEDIA	= 10,
    COEX_RSN_MEDIADISCON	= 11,
    COEX_RSN_BTINFO		= 12,
    COEX_RSN_LPS		= 13,
    COEX_RSN_WLSTATUS	= 14,
    COEX_RSN_BTSTATUS	= 15,

    COEX_RSN_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_lte_coex_table_type {
    COEX_CTT_WL_VS_LTE,
    COEX_CTT_BT_VS_LTE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_gnt_setup_state {
    COEX_GNT_SET_HW_PTA	= 0x0,
    COEX_GNT_SET_SW_LOW	= 0x1,
    COEX_GNT_SET_SW_HIGH	= 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_ext_ant_switch_pos_type {
    COEX_SWITCH_TO_BT,
    COEX_SWITCH_TO_WLG,
    COEX_SWITCH_TO_WLA,
    COEX_SWITCH_TO_NOCARE,
    COEX_SWITCH_TO_WLG_BT,

    COEX_SWITCH_TO_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_ext_ant_switch_ctrl_type {
    COEX_SWITCH_CTRL_BY_BBSW,
    COEX_SWITCH_CTRL_BY_PTA,
    COEX_SWITCH_CTRL_BY_ANTDIV,
    COEX_SWITCH_CTRL_BY_MAC,
    COEX_SWITCH_CTRL_BY_BT,
    COEX_SWITCH_CTRL_BY_FW,

    COEX_SWITCH_CTRL_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_algorithm {
    COEX_ALGO_NOPROFILE	= 0,
    COEX_ALGO_HFP		= 1,
    COEX_ALGO_HID		= 2,
    COEX_ALGO_A2DP		= 3,
    COEX_ALGO_PAN		= 4,
    COEX_ALGO_A2DP_HID	= 5,
    COEX_ALGO_A2DP_PAN	= 6,
    COEX_ALGO_PAN_HID	= 7,
    COEX_ALGO_A2DP_PAN_HID	= 8,

    COEX_ALGO_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_bt_profile {
    BPM_NOPROFILE		= 0,
    BPM_HFP			= BIT(0),
    BPM_HID			= BIT(1),
    BPM_A2DP		= BIT(2),
    BPM_PAN			= BIT(3),
    BPM_HID_HFP		= BPM_HID | BPM_HFP,
    BPM_A2DP_HFP		= BPM_A2DP | BPM_HFP,
    BPM_A2DP_HID		= BPM_A2DP | BPM_HID,
    BPM_A2DP_HID_HFP	= BPM_A2DP | BPM_HID | BPM_HFP,
    BPM_PAN_HFP		= BPM_PAN | BPM_HFP,
    BPM_PAN_HID		= BPM_PAN | BPM_HID,
    BPM_PAN_HID_HFP		= BPM_PAN | BPM_HID | BPM_HFP,
    BPM_PAN_A2DP		= BPM_PAN | BPM_A2DP,
    BPM_PAN_A2DP_HFP	= BPM_PAN | BPM_A2DP | BPM_HFP,
    BPM_PAN_A2DP_HID	= BPM_PAN | BPM_A2DP | BPM_HID,
    BPM_PAN_A2DP_HID_HFP	= BPM_PAN | BPM_A2DP | BPM_HID | BPM_HFP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_wl_link_mode {
    COEX_WLINK_2G1PORT	= 0x0,
    COEX_WLINK_5G		= 0x3,
    COEX_WLINK_2GFREE	= 0x7,
    COEX_WLINK_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_wl2bt_scoreboard {
    COEX_SCBD_ACTIVE	= BIT(0),
    COEX_SCBD_ONOFF		= BIT(1),
    COEX_SCBD_SCAN		= BIT(2),
    COEX_SCBD_UNDERTEST	= BIT(3),
    COEX_SCBD_RXGAIN	= BIT(4),
    COEX_SCBD_BT_RFK	= BIT(5),
    COEX_SCBD_WLBUSY	= BIT(6),
    COEX_SCBD_EXTFEM	= BIT(8),
    COEX_SCBD_TDMA		= BIT(9),
    COEX_SCBD_FIX2M		= BIT(10),
    COEX_SCBD_ALL		= GENMASK(15, 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_power_save_type {
    COEX_PS_WIFI_NATIVE	= 0,
    COEX_PS_LPS_ON		= 1,
    COEX_PS_LPS_OFF		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_rssi_state {
    COEX_RSSI_STATE_HIGH,
    COEX_RSSI_STATE_MEDIUM,
    COEX_RSSI_STATE_LOW,
    COEX_RSSI_STATE_STAY_HIGH,
    COEX_RSSI_STATE_STAY_MEDIUM,
    COEX_RSSI_STATE_STAY_LOW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_notify_type_ips {
    COEX_IPS_LEAVE		= 0x0,
    COEX_IPS_ENTER		= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_notify_type_lps {
    COEX_LPS_DISABLE	= 0x0,
    COEX_LPS_ENABLE		= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_notify_type_scan {
    COEX_SCAN_FINISH,
    COEX_SCAN_START,
    COEX_SCAN_START_2G,
    COEX_SCAN_START_5G,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_notify_type_switchband {
    COEX_NOT_SWITCH,
    COEX_SWITCH_TO_24G,
    COEX_SWITCH_TO_5G,
    COEX_SWITCH_TO_24G_NOFORSCAN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_notify_type_associate {
    COEX_ASSOCIATE_FINISH,
    COEX_ASSOCIATE_START,
    COEX_ASSOCIATE_5G_FINISH,
    COEX_ASSOCIATE_5G_START,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_notify_type_media_status {
    COEX_MEDIA_DISCONNECT,
    COEX_MEDIA_CONNECT,
    COEX_MEDIA_CONNECT_5G,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_bt_status {
    COEX_BTSTATUS_NCON_IDLE		= 0,
    COEX_BTSTATUS_CON_IDLE		= 1,
    COEX_BTSTATUS_INQ_PAGE		= 2,
    COEX_BTSTATUS_ACL_BUSY		= 3,
    COEX_BTSTATUS_SCO_BUSY		= 4,
    COEX_BTSTATUS_ACL_SCO_BUSY	= 5,

    COEX_BTSTATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_wl_tput_dir {
    COEX_WL_TPUT_TX			= 0x0,
    COEX_WL_TPUT_RX			= 0x1,
    COEX_WL_TPUT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_wl_priority_mask {
    COEX_WLPRI_RX_RSP	= 2,
    COEX_WLPRI_TX_RSP	= 3,
    COEX_WLPRI_TX_BEACON	= 4,
    COEX_WLPRI_TX_OFDM	= 11,
    COEX_WLPRI_TX_CCK	= 12,
    COEX_WLPRI_TX_BEACONQ	= 27,
    COEX_WLPRI_RX_CCK	= 28,
    COEX_WLPRI_RX_OFDM	= 29,
    COEX_WLPRI_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_commom_chip_setup {
    COEX_CSETUP_INIT_HW		= 0x0,
    COEX_CSETUP_ANT_SWITCH		= 0x1,
    COEX_CSETUP_GNT_FIX		= 0x2,
    COEX_CSETUP_GNT_DEBUG		= 0x3,
    COEX_CSETUP_RFE_TYPE		= 0x4,
    COEX_CSETUP_COEXINFO_HW		= 0x5,
    COEX_CSETUP_WL_TX_POWER		= 0x6,
    COEX_CSETUP_WL_RX_GAIN		= 0x7,
    COEX_CSETUP_WLAN_ACT_IPS	= 0x8,
    COEX_CSETUP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_indirect_reg_type {
    COEX_INDIRECT_1700		= 0x0,
    COEX_INDIRECT_7C0		= 0x1,
    COEX_INDIRECT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_pstdma_type {
    COEX_PSTDMA_FORCE_LPSOFF	= 0x0,
    COEX_PSTDMA_FORCE_LPSON		= 0x1,
    COEX_PSTDMA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coex_btrssi_type {
    COEX_BTRSSI_RATIO		= 0x0,
    COEX_BTRSSI_DBM			= 0x1,
    COEX_BTRSSI_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_table_para {
    pub bt: u32,
    pub wl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_tdma_para {
    pub para: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_5g_afh_map {
    pub wl_5g_ch: u32,
    pub bt_skip_ch: u8,
    pub bt_skip_span: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_rf_para {
    pub wl_pwr_dec_lvl: u8,
    pub bt_pwr_dec_lvl: u8,
    pub wl_low_gain_en: bool,
    pub bt_lna_lvl: u8,
}

extern "C" {
    pub fn rtw_coex_info_response(rtwdev: *mut rtw_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn rtw_coex_read_indirect_reg(rtwdev: *mut rtw_dev, addr: u16) -> u32;
}
extern "C" {
    pub fn rtw_coex_write_scbd(rtwdev: *mut rtw_dev, bitpos: u16, set: bool);
}
extern "C" {
    pub fn rtw_coex_query_bt_info(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_coex_bt_relink_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_bt_reenable_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_defreeze_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_wl_remain_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_bt_remain_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_wl_connecting_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_bt_multi_link_remain_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_wl_ccklock_work(work: *mut work_struct);
}
extern "C" {
    pub fn rtw_coex_power_on_setting(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_coex_power_off_setting(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_coex_init_hw_config(rtwdev: *mut rtw_dev, wifi_only: bool);
}
extern "C" {
    pub fn rtw_coex_ips_notify(rtwdev: *mut rtw_dev, type: u8);
}
extern "C" {
    pub fn rtw_coex_lps_notify(rtwdev: *mut rtw_dev, type: u8);
}
extern "C" {
    pub fn rtw_coex_scan_notify(rtwdev: *mut rtw_dev, type: u8);
}
extern "C" {
    pub fn rtw_coex_connect_notify(rtwdev: *mut rtw_dev, type: u8);
}
extern "C" {
    pub fn rtw_coex_media_status_notify(rtwdev: *mut rtw_dev, type: u8);
}
extern "C" {
    pub fn rtw_coex_bt_info_notify(rtwdev: *mut rtw_dev, buf: *mut u8, length: u8);
}
extern "C" {
    pub fn rtw_coex_bt_hid_info_notify(rtwdev: *mut rtw_dev, buf: *mut u8, length: u8);
}
extern "C" {
    pub fn rtw_coex_wl_fwdbginfo_notify(rtwdev: *mut rtw_dev, buf: *mut u8, length: u8);
}
extern "C" {
    pub fn rtw_coex_switchband_notify(rtwdev: *mut rtw_dev, type: u8);
}
extern "C" {
    pub fn rtw_coex_wl_status_change_notify(rtwdev: *mut rtw_dev, type: u32);
}
extern "C" {
    pub fn rtw_coex_wl_status_check(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_coex_query_bt_hid_list(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_coex_display_coex_info(rtwdev: *mut rtw_dev, m: *mut seq_file);
}
// The RTL8821AU firmware doesn't send C2H_BT_INFO by itself
// when bluetooth headphones are disconnected, so we have to
// ask for it regularly.
//
