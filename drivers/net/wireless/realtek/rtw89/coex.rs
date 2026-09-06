//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/coex.h
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
// Copyright(c) 2019-2020  Realtek Corporation
//

pub const BTC_H2C_MAXLEN: c_int = 2020;
pub const BTC_TLV_SLOT_ID_LEN_V7: c_int = 1;
pub const BTC_SLOT_REQ_TH: c_int = 2;
pub const BTC_FREQ_W2G: c_int = 2412;
pub const BTC_FREQ_W5G: c_int = 5005;
pub const BTC_FREQ_W6G: c_int = 5955;
pub const BTC_FREQ_B2G: c_int = 2402;
pub const BTC_FREQ_B6G: c_int = 5125;
pub const BTC_FREQ_W2G_CH14: c_int = 2484;

pub const BTC_LO_5G_DNM: c_int = 125;
pub const BTC_LO_6G_DNM: c_int = 150;

pub const BTC_LO_5G_NMR: c_int = 200;
pub const BTC_LO_6G_NMR: c_int = 200;

pub const BTC_CH_W2G_MAX: c_int = 14;

pub const BTC_CH_W5G_MAX: c_int = 165;

pub const BTC_CH_W6G_MAX: c_int = 233;

pub const BTC_CH_B2G_MAX: c_int = 78;

pub const BTC_CH_B5G_MAX: c_int = 1300;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_mode {
    BTC_MODE_NORMAL,
    BTC_MODE_WL,
    BTC_MODE_BT,
    BTC_MODE_WLOFF,
    BTC_MODE_COTX,
    BTC_MODE_MECHANISM_INIT,
    BTC_MODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wl_rfk_type {
    BTC_WRFKT_IQK = 0,
    BTC_WRFKT_LCK = 1,
    BTC_WRFKT_DPK = 2,
    BTC_WRFKT_TXGAPK = 3,
    BTC_WRFKT_DACK = 4,
    BTC_WRFKT_RXDCK = 5,
    BTC_WRFKT_TSSI = 6,
    BTC_WRFKT_CHLK = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wl_rfk_state {
    BTC_WRFK_STOP = 0,
    BTC_WRFK_START = 1,
    BTC_WRFK_ONESHOT_START = 2,
    BTC_WRFK_ONESHOT_STOP = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_pri {
    BTC_PRI_MASK_RX_RESP = 0,
    BTC_PRI_MASK_TX_RESP,
    BTC_PRI_MASK_BEACON,
    BTC_PRI_MASK_RX_CCK,
    BTC_PRI_MASK_TX_MNGQ,
    BTC_PRI_MASK_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_bt_trs {
    BTC_BT_SS_GROUP = 0x0,
    BTC_BT_TX_GROUP = 0x2,
    BTC_BT_RX_GROUP = 0x3,
    BTC_BT_MAX_GROUP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_rssi_st {
    BTC_RSSI_ST_LOW = 0x0,
    BTC_RSSI_ST_HIGH,
    BTC_RSSI_ST_STAY_LOW,
    BTC_RSSI_ST_STAY_HIGH,
    BTC_RSSI_ST_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_fddt_en {
    BTC_FDDT_DISABLE,
    BTC_FDDT_ENABLE,
}

// Antenna TX/RX path position masks and helpers
pub const BTC_ANT_TX_MASK: c_uint = 0xf0;
pub const BTC_ANT_RX_MASK: c_uint = 0x0f;
pub const BTC_ANT_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_ant {
    BTC_ANT_SHARED = 0,
    BTC_ANT_DEDICATED,
    BTC_ANTTYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_bt_btg {
    BTC_BT_ALONE = 0,
    BTC_BT_BTG
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_switch {
    BTC_SWITCH_INTERNAL = 0,
    BTC_SWITCH_EXTERNAL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_ant_switch_type {
    BTC_SWITCH_V1_NONE = 0, /* independent antenna */
    BTC_SWITCH_V1_INTERNAL, /* internal-switch: BTGA structure */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_pkt_type {
    PACKET_DHCP,
    PACKET_ARP,
    PACKET_EAPOL,
    PACKET_EAPOL_END,
    PACKET_ICMP,
    PACKET_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_bt_mailbox_id {
    BTC_BTINFO_REPLY = 0x23,
    BTC_BTINFO_AUTO = 0x27
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_role_state {
    BTC_ROLE_START,
    BTC_ROLE_STOP,
    BTC_ROLE_CHG_TYPE,
    BTC_ROLE_MSTS_STA_CONN_START,
    BTC_ROLE_MSTS_STA_CONN_END,
    BTC_ROLE_MSTS_STA_DIS_CONN,
    BTC_ROLE_MSTS_AP_START,
    BTC_ROLE_MSTS_AP_STOP,
    BTC_ROLE_STATE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_rfctrl {
    BTC_RFCTRL_WL_OFF,
    BTC_RFCTRL_WL_ON,
    BTC_RFCTRL_LPS_WL_ON,
    BTC_RFCTRL_FW_CTRL,
    BTC_RFCTRL_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_lps_state {
    BTC_LPS_OFF = 0,
    BTC_LPS_RF_OFF = 1,
    BTC_LPS_RF_ON = 2
}

pub const R_BTC_BB_BTG_RX: c_uint = 0x980;
pub const R_BTC_BB_PRE_AGC_S1: c_uint = 0x476C;
pub const R_BTC_BB_PRE_AGC_S0: c_uint = 0x4688;

pub const BTC_REG_NOTFOUND: c_uint = 0xff;
pub const R_BTC_ZB_COEX_TBL_0: c_uint = 0xE328;
pub const R_BTC_ZB_COEX_TBL_1: c_uint = 0xE32c;
pub const R_BTC_ZB_BREAK_TBL: c_uint = 0xE350;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_ant_div_pos {
    BTC_ANT_DIV_MAIN = 0,
    BTC_ANT_DIV_AUX = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_get_reg_status {
    BTC_CSTATUS_TXDIV_POS = 0,
    BTC_CSTATUS_RXDIV_POS = 1,
    BTC_CSTATUS_BB_GNT_MUX = 2,
    BTC_CSTATUS_BB_GNT_MUX_MON = 3,
    BTC_CSTATUS_BB_PRE_AGC = 4,
    BTC_CSTATUS_BB_PRE_AGC_MON = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_preagc_type {
    BTC_PREAGC_DISABLE,
    BTC_PREAGC_ENABLE,
    BTC_PREAGC_BB_FWCTRL,
    BTC_PREAGC_NOTFOUND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_btgctrl_type {
    BTC_BTGCTRL_DISABLE,
    BTC_BTGCTRL_ENABLE,
    BTC_BTGCTRL_BB_GNT_FWCTRL,
    BTC_BTGCTRL_BB_GNT_NOTFOUND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wa_type {
    BTC_WA_5G_HI_CH_RX = BIT(0),
    BTC_WA_NULL_AP = BIT(1),
    BTC_WA_HFP_ZB = BIT(2),  /* HFP PTA req bit4 define issue */
    BTC_WA_HFP_LAG = BIT(3),  /* 52BT WL break BT Rx lag issue */
    BTC_WA_INIT_SCAN = BIT(4)  /* 52A/C/D init scan move to wl slot WA */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_3cx_type {
    BTC_3CX_NONE = 0,
    BTC_3CX_BT2 = BIT(0),
    BTC_3CX_ZB = BIT(1),
    BTC_3CX_LTE = BIT(2),
    BTC_3CX_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_fddt_type {
    BTC_FDDT_TYPE_STOP,
    BTC_FDDT_TYPE_AUTO,
    BTC_FDDT_TYPE_FIX_TDD,
    BTC_FDDT_TYPE_FIX_FULL_FDD,
    BTC_FDDT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_chip_feature {
    BTC_FEAT_PTA_ONOFF_CTRL  = BIT(0), /* on/off ctrl by HW (not 0x73[2]) */
    BTC_FEAT_NONBTG_GWL_THRU = BIT(1), /* non-BTG GNT_WL!=0 if GNT_BT = 1 */
    BTC_FEAT_WLAN_ACT_MUX = BIT(2), /* separate wlan_act/gnt mux */
    BTC_FEAT_NEW_BBAPI_FLOW = BIT(3), /* new btg_ctrl/pre_agc_ctrl */
    BTC_FEAT_MLO_SUPPORT = BIT(4),
    BTC_FEAT_H2C_MACRO = BIT(5),
    BTC_FEAT_DUAL_BT = BIT(6),
    BTC_FEAT_BT_6G = BIT(7),
    BTC_FEAT_MULTI_PTA = BIT(8),
    BTC_FEAT_DUAL_BTGA = BIT(9) /* the future A-Die */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_efuse_ant_function_map {
    BTC_EFMAP_NONE = 0,
    BTC_EFMAP_BT0 = BIT(0),
    BTC_EFMAP_BT1 = BIT(1),
    BTC_EFMAP_ZB = BIT(2), /* ZB or thread */
    BTC_EFMAP_24GP = BIT(3),
    BTC_EFMAP_ULL = BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_extsoc_interface {
    BTC_EXTSOC_INTF_NONE = 0,
    BTC_EXTSOC_INTF_PTA = BIT(0),
    BTC_EXTSOC_INTF_MBX = BIT(1),
    BTC_EXTSOC_INTF_SWIO = BIT(2),
    BTC_EXTSOC_INTF_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_esoc_type {
    BTC_ESOC_NONE,
    BTC_ESOC_8761,
    BTC_ESOC_8771,
    BTC_ESOC_SILAB_MG21,
    BTC_ESOC_NORDI_NRF52840,
    BTC_ESOC_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wl_mode {
    BTC_WL_MODE_11B = 0,
    BTC_WL_MODE_11A = 1,
    BTC_WL_MODE_11G = 2,
    BTC_WL_MODE_HT = 3,
    BTC_WL_MODE_VHT = 4,
    BTC_WL_MODE_HE = 5,
    BTC_WL_MODE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_mlo_rf_combin {
    BTC_MLO_RF_2_PLUS_0 = 0,
    BTC_MLO_RF_0_PLUS_2 = 1,
    BTC_MLO_RF_1_PLUS_1 = 2,
    BTC_MLO_RF_2_PLUS_2 = 3,
}

extern "C" {
    pub fn rtw89_btc_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_btc_ntfy_poweron(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_btc_ntfy_poweroff(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_btc_ntfy_init(rtwdev: *mut rtw89_dev, mode: u8);
}
extern "C" {
    pub fn rtw89_btc_ntfy_scan_start(rtwdev: *mut rtw89_dev, phy_idx: u8, band: u8);
}
extern "C" {
    pub fn rtw89_btc_ntfy_scan_finish(rtwdev: *mut rtw89_dev, phy_idx: u8);
}
extern "C" {
    pub fn rtw89_btc_ntfy_switch_band(rtwdev: *mut rtw89_dev, phy_idx: u8, band: u8);
}
extern "C" {
    pub fn rtw89_btc_ntfy_eapol_packet_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_btc_ntfy_arp_packet_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_btc_ntfy_dhcp_packet_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_btc_ntfy_icmp_packet_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_btc_ntfy_radio_state(rtwdev: *mut rtw89_dev, rf_state: btc_rfctrl);
}
extern "C" {
    pub fn rtw89_btc_ntfy_wl_sta(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_btc_dump_info(rtwdev: *mut rtw89_dev, buf: *mut c_char, bufsz: usize) -> isize;
}
extern "C" {
    pub fn rtw89_coex_act1_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_coex_bt_devinfo_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_coex_rfk_chk_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_coex_power_on(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_btc_set_policy(rtwdev: *mut rtw89_dev, policy_type: u16);
}
extern "C" {
    pub fn rtw89_btc_set_policy_v1(rtwdev: *mut rtw89_dev, policy_type: u16);
}
extern "C" {
    pub fn rtw89_coex_recognize_ver(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_btc_ntfy_preserve_bt_time(rtwdev: *mut rtw89_dev, ms: u32);
}
extern "C" {
    pub fn rtw89_btc_ntfy_conn_rfk(rtwdev: *mut rtw89_dev, state: bool);
}
extern "C" {
    pub fn rtw89_btc_phymap(_arg: rtwdev, _arg: phy_idx, _arg: BIT(path), _arg: chanctx_idx) -> return;
}
// return bt req len in TU
