//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/fw.h
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
pub const H2C_PKT_SIZE: c_int = 32;
pub const H2C_PKT_HDR_SIZE: c_int = 8;
// FW bin information
pub const FW_HDR_SIZE: c_int = 64;
pub const FW_HDR_CHKSUM_SIZE: c_int = 8;
pub const FW_NLO_INFO_CHECK_SIZE: c_int = 4;
pub const FIFO_PAGE_SIZE_SHIFT: c_int = 12;
pub const FIFO_PAGE_SIZE: c_int = 4096;
pub const FIFO_DUMP_ADDR: c_uint = 0x8000;
pub const DLFW_PAGE_SIZE_SHIFT_LEGACY: c_int = 12;
pub const DLFW_PAGE_SIZE_LEGACY: c_uint = 0x1000;
pub const DLFW_BLK_SIZE_SHIFT_LEGACY: c_int = 2;
pub const DLFW_BLK_SIZE_LEGACY: c_int = 4;
pub const FW_START_ADDR_LEGACY: c_uint = 0x1000;
pub const BCN_LOSS_CNT: c_int = 10;
pub const BCN_FILTER_NOTIFY_SIGNAL_CHANGE: c_int = 0;
pub const BCN_FILTER_CONNECTION_LOSS: c_int = 1;
pub const BCN_FILTER_CONNECTED: c_int = 2;
pub const BCN_FILTER_NOTIFY_BEACON_LOSS: c_int = 3;

pub const RTW_DEFAULT_CQM_HYST: c_int = 4;

pub const RTW_CHANNEL_TIME: c_int = 45;
pub const RTW_OFF_CHAN_TIME: c_int = 100;
pub const RTW_PASS_CHAN_TIME: c_int = 105;
pub const RTW_DFS_CHAN_TIME: c_int = 20;
pub const RTW_CH_INFO_SIZE: c_int = 4;
pub const RTW_EX_CH_INFO_SIZE: c_int = 3;
pub const RTW_EX_CH_INFO_HDR_SIZE: c_int = 2;
pub const RTW_SCAN_WIDTH: c_int = 0;
pub const RTW_PRI_CH_IDX: c_int = 1;
pub const RTW_OLD_PROBE_PG_CNT: c_int = 2;
pub const RTW_PROBE_PG_CNT: c_int = 4;
pub const RTW_DEBUG_DUMP_TIMES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_c2h_cmd_id {
    C2H_CCX_TX_RPT = 0x03,
    C2H_BT_INFO = 0x09,
    C2H_BT_MP_INFO = 0x0b,
    C2H_BT_HID_INFO = 0x45,
    C2H_RA_RPT = 0x0c,
    C2H_HW_FEATURE_REPORT = 0x19,
    C2H_WLAN_INFO = 0x27,
    C2H_WLAN_RFON = 0x32,
    C2H_BCN_FILTER_NOTIFY = 0x36,
    C2H_ADAPTIVITY = 0x37,
    C2H_SCAN_RESULT = 0x38,
    C2H_HW_FEATURE_DUMP = 0xfd,
    C2H_HALMAC = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_c2h_cmd_id_ext {
    C2H_SCAN_STATUS_RPT = 0x3,
    C2H_CCX_RPT = 0x0f,
    C2H_CHAN_SWITCH = 0x22,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_c2h_cmd {
    pub id: u8,
    pub seq: u8,
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_c2h_adaptivity {
    pub density: u8,
    pub igi: u8,
    pub l2h_th_init: u8,
    pub l2h: u8,
    pub h2l: u8,
    pub option: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_c2h_ra_rpt {
    pub rate_sgi: u8,
    pub mac_id: u8,
    pub byte2: u8,
    pub status: u8,
    pub byte4: u8,
    pub ra_ratio: u8,
    pub bw: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_h2c_register {
    pub w0: u32,
    pub w1: u32,
    pub __packed: },

// H2C_CMD_DEFAULT_PORT command

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_h2c_cmd {
    pub msg: __le32,
    pub msg_ext: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rsvd_packet_type {
    RSVD_BEACON,
    RSVD_DUMMY,
    RSVD_PS_POLL,
    RSVD_PROBE_RESP,
    RSVD_NULL,
    RSVD_QOS_NULL,
    RSVD_LPS_PG_DPK,
    RSVD_LPS_PG_INFO,
    RSVD_PROBE_REQ,
    RSVD_NLO_INFO,
    RSVD_CH_INFO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_fw_rf_type {
    FW_RF_1T2R = 0,
    FW_RF_2T4R = 1,
    FW_RF_2T2R = 2,
    FW_RF_2T3R = 3,
    FW_RF_1T1R = 4,
    FW_RF_2T2R_GREEN = 5,
    FW_RF_3T3R = 6,
    FW_RF_3T4R = 7,
    FW_RF_4T4R = 8,
    FW_RF_MAX_TYPE = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_fw_feature {
    FW_FEATURE_SIG = BIT(0),
    FW_FEATURE_LPS_C2H = BIT(1),
    FW_FEATURE_LCLK = BIT(2),
    FW_FEATURE_PG = BIT(3),
    FW_FEATURE_TX_WAKE = BIT(4),
    FW_FEATURE_BCN_FILTER = BIT(5),
    FW_FEATURE_NOTIFY_SCAN = BIT(6),
    FW_FEATURE_ADAPTIVITY = BIT(7),
    FW_FEATURE_SCAN_OFFLOAD = BIT(8),
    FW_FEATURE_MAX = BIT(31),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_fw_feature_ext {
    FW_FEATURE_EXT_OLD_PAGE_NUM = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_beacon_filter_offload_mode {
    BCN_FILTER_OFFLOAD_MODE_0 = 0,
    BCN_FILTER_OFFLOAD_MODE_1,
    BCN_FILTER_OFFLOAD_MODE_2,
    BCN_FILTER_OFFLOAD_MODE_3,

    BCN_FILTER_OFFLOAD_MODE_DEFAULT = BCN_FILTER_OFFLOAD_MODE_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex_info_req {
    pub seq: u8,
    pub op_code: u8,
    pub para1: u8,
    pub para2: u8,
    pub para3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_iqk_para {
    pub clear: u8,
    pub segment_iqk: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_lps_pg_dpk_hdr {
    pub dpk_path_ok: u16,
    pub dpk_txagc: [u8; 2],
    pub dpk_gs: [u16; 2],
    pub coef: [u32; 2][20],
    pub dpk_ch: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_lps_pg_info_hdr {
    pub macid: u8,
    pub mbssid: u8,
    pub pattern_count: u8,
    pub mu_tab_group_id: u8,
    pub sec_cam_count: u8,
    pub tx_bu_page_count: u8,
    pub rsvd: u16,
    pub sec_cam: [u8; MAX_PG_CAM_BACKUP_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_rsvd_page {
// associated with each vif
    pub vif_list: list_head,
    pub rtwvif: *mut rtw_vif,
// associated when build rsvd page
    pub build_list: list_head,
    pub skb: *mut sk_buff,
    pub type: rtw_rsvd_packet_type,
    pub page: u8,
    pub tim_offset: u16,
    pub add_txdesc: bool,
    pub ssid: *mut cfg80211_ssid,
    pub probe_req_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_keep_alive_pkt_type {
    KEEP_ALIVE_NULL_PKT = 0,
    KEEP_ALIVE_ARP_RSP = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_nlo_info_hdr {
    pub nlo_count: u8,
    pub hidden_ap_count: u8,
    pub rsvd1: [u8; 2],
    pub pattern_check: [u8; FW_NLO_INFO_CHECK_SIZE],
    pub rsvd2: [u8; 8],
    pub ssid_len: [u8; 16],
    pub chiper: [u8; 16],
    pub rsvd3: [u8; 16],
    pub location: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_packet_type {
    RTW_PACKET_PROBE_REQ = 0x00,

    RTW_PACKET_UNDEFINE = 0x7FFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fw_wow_keep_alive_para {
    pub adopt: bool,
    pub pkt_type: u8,
    pub /: *mut *mut u8 period; / unit: sec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fw_wow_disconnect_para {
    pub adopt: bool,
    pub /: *mut *mut u8 period; / unit: sec,
    pub retry_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_channel_type {
    RTW_CHANNEL_PASSIVE,
    RTW_CHANNEL_ACTIVE,
    RTW_CHANNEL_RADAR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_scan_extra_id {
    RTW_SCAN_EXTRA_ID_DFS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_scan_extra_info {
    RTW_SCAN_EXTRA_ACTION_SCAN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_scan_report_code {
    RTW_SCAN_REPORT_SUCCESS = 0x00,
    RTW_SCAN_REPORT_ERR_PHYDM = 0x01,
    RTW_SCAN_REPORT_ERR_ID = 0x02,
    RTW_SCAN_REPORT_ERR_TX = 0x03,
    RTW_SCAN_REPORT_CANCELED = 0x10,
    RTW_SCAN_REPORT_CANCELED_EXT = 0x11,
    RTW_SCAN_REPORT_FW_DISABLED = 0xF0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_scan_notify_id {
    RTW_SCAN_NOTIFY_ID_PRESWITCH = 0x00,
    RTW_SCAN_NOTIFY_ID_POSTSWITCH = 0x01,
    RTW_SCAN_NOTIFY_ID_PROBE_PRETX = 0x02,
    RTW_SCAN_NOTIFY_ID_PROBE_ISSUETX = 0x03,
    RTW_SCAN_NOTIFY_ID_NULL0_PRETX = 0x04,
    RTW_SCAN_NOTIFY_ID_NULL0_ISSUETX = 0x05,
    RTW_SCAN_NOTIFY_ID_NULL0_POSTTX = 0x06,
    RTW_SCAN_NOTIFY_ID_NULL1_PRETX = 0x07,
    RTW_SCAN_NOTIFY_ID_NULL1_ISSUETX = 0x08,
    RTW_SCAN_NOTIFY_ID_NULL1_POSTTX = 0x09,
    RTW_SCAN_NOTIFY_ID_DWELLEXT = 0x0A,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_scan_notify_status {
    RTW_SCAN_NOTIFY_STATUS_SUCCESS = 0x00,
    RTW_SCAN_NOTIFY_STATUS_FAILURE = 0x01,
    RTW_SCAN_NOTIFY_STATUS_RESOURCE = 0x02,
    RTW_SCAN_NOTIFY_STATUS_TIMEOUT = 0x03,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_ch_switch_option {
    pub periodic_option: u8,
    pub tsf_high: u32,
    pub tsf_low: u32,
    pub dest_ch_en: u8,
    pub absolute_time_en: u8,
    pub dest_ch: u8,
    pub normal_period: u8,
    pub normal_period_sel: u8,
    pub normal_cycle: u8,
    pub slow_period: u8,
    pub slow_period_sel: u8,
    pub nlo_en: u8,
    pub switch_en: bool,
    pub back_op_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fw_hdr {
    pub signature: __le16,
    pub category: u8,
    pub function: u8,
    pub /: *mut *mut __le16 version; / 0x04,
    pub subversion: u8,
    pub subindex: u8,
    pub /: *mut *mut __le32 rsvd; / 0x08,
    pub /: *mut *mut __le32 feature; / 0x0C,
    pub /: *mut *mut u8 month; / 0x10,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub /: *mut *mut __le16 year; / 0x14,
    pub rsvd3: __le16,
    pub /: *mut *mut u8 mem_usage; / 0x18,
    pub rsvd4: [u8; 3],
    pub /: *mut *mut __le16 h2c_fmt_ver; / 0x1C,
    pub rsvd5: __le16,
    pub /: *mut *mut __le32 dmem_addr; / 0x20,
    pub dmem_size: __le32,
    pub rsvd6: __le32,
    pub rsvd7: __le32,
    pub /: *mut *mut __le32 imem_size; / 0x30,
    pub emem_size: __le32,
    pub emem_addr: __le32,
    pub imem_addr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fw_hdr_legacy {
    pub signature: __le16,
    pub category: u8,
    pub function: u8,
    pub /: *mut *mut __le16 version; / 0x04,
    pub subversion1: u8,
    pub subversion2: u8,
    pub /: *mut *mut u8 month; / 0x08,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub size: __le16,
    pub rsvd2: __le16,
    pub /: *mut *mut __le32 idx; / 0x10,
    pub rsvd3: __le32,
    pub /: *mut *mut __le32 rsvd4; / 0x18,
    pub rsvd5: __le32,
    pub __packed: },

// C2H

// PKT H2C
pub const H2C_PKT_CMD_ID: c_uint = 0xFF;
pub const H2C_PKT_CATEGORY: c_uint = 0x01;
pub const H2C_PKT_GENERAL_INFO: c_uint = 0x0D;
pub const H2C_PKT_PHYDM_INFO: c_uint = 0x11;
pub const H2C_PKT_IQK: c_uint = 0x0E;
pub const H2C_PKT_CH_SWITCH: c_uint = 0x02;
pub const H2C_PKT_UPDATE_PKT: c_uint = 0x0C;
pub const H2C_PKT_SCAN_OFFLOAD: c_uint = 0x19;
pub const H2C_PKT_CH_SWITCH_LEN: c_uint = 0x20;
pub const H2C_PKT_UPDATE_PKT_LEN: c_uint = 0x4;

    pub H2C_PKT_CATEGORY): SET_PKT_H2C_CATEGORY(h2c_pkt,,
    pub H2C_PKT_CMD_ID): SET_PKT_H2C_CMD_ID(h2c_pkt,,
    pub sub_id): SET_PKT_H2C_SUB_CMD_ID(h2c_pkt,,

// Command H2C
pub const H2C_CMD_RSVD_PAGE: c_uint = 0x0;
pub const H2C_CMD_MEDIA_STATUS_RPT: c_uint = 0x01;
pub const H2C_CMD_SET_PWR_MODE: c_uint = 0x20;
pub const H2C_CMD_LPS_PG_INFO: c_uint = 0x2b;
pub const H2C_CMD_DEFAULT_PORT: c_uint = 0x2c;
pub const H2C_CMD_RA_INFO: c_uint = 0x40;
pub const H2C_CMD_RSSI_MONITOR: c_uint = 0x42;
pub const H2C_CMD_RA_INFO_HI: c_uint = 0x46;
pub const H2C_CMD_BCN_FILTER_OFFLOAD_P0: c_uint = 0x56;
pub const H2C_CMD_BCN_FILTER_OFFLOAD_P1: c_uint = 0x57;
pub const H2C_CMD_WL_PHY_INFO: c_uint = 0x58;
pub const H2C_CMD_SCAN: c_uint = 0x59;
pub const H2C_CMD_ADAPTIVITY: c_uint = 0x5A;
pub const H2C_CMD_COEX_TDMA_TYPE: c_uint = 0x60;
pub const H2C_CMD_QUERY_BT_INFO: c_uint = 0x61;
pub const H2C_CMD_FORCE_BT_TX_POWER: c_uint = 0x62;
pub const H2C_CMD_IGNORE_WLAN_ACTION: c_uint = 0x63;
pub const H2C_CMD_WL_CH_INFO: c_uint = 0x66;
pub const H2C_CMD_QUERY_BT_MP_INFO: c_uint = 0x67;
pub const H2C_CMD_BT_WIFI_CONTROL: c_uint = 0x69;
pub const H2C_CMD_WIFI_CALIBRATION: c_uint = 0x6d;
pub const H2C_CMD_QUERY_BT_HID_INFO: c_uint = 0x73;
pub const H2C_CMD_KEEP_ALIVE: c_uint = 0x03;
pub const H2C_CMD_DISCONNECT_DECISION: c_uint = 0x04;
pub const H2C_CMD_WOWLAN: c_uint = 0x80;
pub const H2C_CMD_REMOTE_WAKE_CTRL: c_uint = 0x81;
pub const H2C_CMD_AOAC_GLOBAL_INFO: c_uint = 0x82;
pub const H2C_CMD_NLO_INFO: c_uint = 0x8C;
pub const H2C_CMD_RECOVER_BT_DEV: c_uint = 0xD1;

    pub pkt_offset: u32,
    pub )skb->cb): *mut *mut pkt_offset = ((u32,
    pub pkt_offset): *mut *mut return (struct rtw_c2h_cmd )(skb->data +,
    pub feature): return !!(fw->feature &,
    pub feature): return !!(fw->feature_ext &,
    pub rtwdev): *mut void rtw_fw_dump_dbg_info(struct rtw_dev,
    pub skb): *mut sk_buff,
    pub skb): *mut *mut void rtw_fw_c2h_cmd_handle(struct rtw_dev rtwdev, struct sk_buff,
    pub rtwdev): *mut void rtw_fw_send_general_info(struct rtw_dev,
    pub rtwdev): *mut void rtw_fw_send_phydm_info(struct rtw_dev,
    pub rtwvif): *mut *mut void rtw_fw_default_port(struct rtw_dev rtwdev, struct rtw_vif,
    pub para): *mut *mut void rtw_fw_do_iqk(struct rtw_dev rtwdev, struct rtw_iqk_para,
    pub start): *mut *mut void rtw_fw_inform_rfk_status(struct rtw_dev rtwdev, bool,
    pub rtwdev): *mut void rtw_fw_set_pwr_mode(struct rtw_dev,
    pub rtwdev): *mut void rtw_fw_set_pg_info(struct rtw_dev,
    pub rtwdev): *mut void rtw_fw_query_bt_info(struct rtw_dev,
    pub bw): *mut *mut void rtw_fw_wl_ch_info(struct rtw_dev rtwdev, u8 link, u8 ch, u8,
    pub req): *mut rtw_coex_info_req,
    pub bt_pwr_dec_lvl): *mut *mut void rtw_fw_force_bt_tx_power(struct rtw_dev rtwdev, u8,
    pub enable): *mut *mut void rtw_fw_bt_ignore_wlan_action(struct rtw_dev rtwdev, bool,
    pub para5): u8 para1, u8 para2, u8 para3, u8 para4, u8,
    pub data): *mut *mut void rtw_fw_coex_query_hid_info(struct rtw_dev rtwdev, u8 sub_id, u8,
    pub data): *mut *mut void rtw_fw_bt_wifi_control(struct rtw_dev rtwdev, u8 op_code, u8,
    pub si): *mut *mut void rtw_fw_send_rssi_info(struct rtw_dev rtwdev, struct rtw_sta_info,
    pub reset_ra_mask): bool,
    pub conn): *mut *mut void rtw_fw_media_status_report(struct rtw_dev rtwdev, u8 mac_id, bool,
    pub rtwdev): *mut void rtw_fw_update_wl_phy_info(struct rtw_dev,
    pub vif): *mut ieee80211_vif,
    pub size): *mut *mut u8 buf, u32,
    pub rtwvif): *mut rtw_vif,
    pub rtwvif): *mut rtw_vif,
    pub rtwvif): *mut rtw_vif,
    pub rtwvif): *mut rtw_vif,
    pub rtwdev): *mut int rtw_fw_download_rsvd_page(struct rtw_dev,
    pub work): *mut void rtw_fw_update_beacon_work(struct work_struct,
    pub rtwdev): *mut void rtw_send_rsvd_page_h2c(struct rtw_dev,
    pub buf): *mut u32 offset, u32 size, u32,
    pub enable): *mut *mut void rtw_fw_set_remote_wake_ctrl_cmd(struct rtw_dev rtwdev, bool,
    pub enable): *mut *mut void rtw_fw_set_wowlan_ctrl_cmd(struct rtw_dev rtwdev, bool,
    pub enable): *mut *mut void rtw_fw_set_keep_alive_cmd(struct rtw_dev rtwdev, bool,
    pub enable): *mut *mut void rtw_fw_set_disconnect_decision_cmd(struct rtw_dev rtwdev, bool,
    pub group_key_enc): u8,
    pub enable): *mut *mut void rtw_fw_set_nlo_info(struct rtw_dev rtwdev, bool,
    pub rtwdev): *mut void rtw_fw_set_recover_bt_device(struct rtw_dev,
    pub ssid): *mut cfg80211_ssid,
    pub enable): *mut *mut void rtw_fw_channel_switch(struct rtw_dev rtwdev, bool,
    pub h2c): *mut *mut void rtw_fw_h2c_cmd_dbg(struct rtw_dev rtwdev, u8,
    pub rtwdev): *mut void rtw_fw_c2h_cmd_isr(struct rtw_dev,
    pub buffer): *mut u32,
    pub start): *mut *mut void rtw_fw_scan_notify(struct rtw_dev rtwdev, bool,
    pub rtwdev): *mut void rtw_fw_adaptivity(struct rtw_dev,
    pub backup): *mut *mut void rtw_store_op_chan(struct rtw_dev rtwdev, bool,
    pub rtwdev): *mut void rtw_clear_op_chan(struct rtw_dev,
    pub req): *mut ieee80211_scan_request,
    pub aborted): bool,
    pub enable): bool,
    pub skb): *mut *mut void rtw_hw_scan_status_report(struct rtw_dev rtwdev, struct sk_buff,
    pub skb): *mut *mut void rtw_hw_scan_chan_switch(struct rtw_dev rtwdev, struct sk_buff,
    pub rtwdev): *mut void rtw_hw_scan_abort(struct rtw_dev,
