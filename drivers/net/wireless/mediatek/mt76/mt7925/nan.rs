//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7925/nan.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2025-2026 MediaTek Inc.

pub const NAN_MAX_SOCIAL_CHANNELS: c_int = 3;
pub const NAN_ANCHOR_MASTER_RANK_NUM: c_int = 8;
pub const NAN_5G_LOW_DISC_CHANNEL: c_int = 44;
pub const NAN_5G_HIGH_DISC_CHANNEL: c_int = 149;
pub const NAN_MAX_MASTER_PREFERENCE: c_int = 255;
pub const NAN_DEFAULT_DW_INTERVAL: c_int = 1;
pub const NAN_DEFAULT_DISC_BCN_INTERVAL: c_int = 100;
pub const NAN_TOTAL_DW: c_int = 16;
pub const NAN_SUPPORTED_2G_FAW_CH_NUM: c_int = 4;
pub const NAN_SUPPORTED_5G_FAW_CH_NUM: c_int = 4;
pub const NAN_TIMELINE_MGMT_SIZE: c_int = 2;

pub const NAN_NUM_AVAIL_DB: c_int = 2;
pub const NAN_NDC_ATTRIBUTE_ID_LENGTH: c_int = 6;
pub const NAN_MAX_CONN_CFG: c_int = 8;
pub const NAN_MAX_NDP_CXT: c_int = 4;

// NAN Availability Attribute
pub const NAN_AVAIL_ATTR_ID_OFFSET: c_int = 0;
pub const NAN_AVAIL_ATTR_LEN_OFFSET: c_int = 1;
pub const NAN_AVAIL_SEQ_ID_OFFSET: c_int = 3;
pub const NAN_AVAIL_ATTR_CTRL_OFFSET: c_int = 4;
// NAN Availability Attribute - Attribute Control Field

pub const UNII1_LOWER_BOUND: c_int = 36;
pub const UNII1_UPPER_BOUND: c_int = 50;
pub const UNII3_LOWER_BOUND: c_int = 149;
pub const UNII3_UPPER_BOUND: c_int = 165;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nan_uni_cmd_tag {
    NAN_UNI_CMD_SET_MASTER_PREFERENCE	= 0,
    NAN_UNI_CMD_ENABLE_REQUEST		= 7,
    NAN_UNI_CMD_DISABLE_REQUEST		= 8,
    NAN_UNI_CMD_UPDATE_AVAILABILITY		= 9,
    NAN_UNI_CMD_UPDATE_CRB			= 10,
    NAN_UNI_CMD_MANAGE_PEER_SCH_RECORD	= 12,
    NAN_UNI_CMD_MAP_STA_RECORD		= 13,
    NAN_UNI_CMD_UPDATE_AVAILABILITY_CTRL	= 20,
    NAN_UNI_CMD_UPDATE_PEER_CAPABILITY	= 21,
    NAN_UNI_CMD_CHANGE_NMI_ADDRESS		= 24,
    NAN_UNI_CMD_SET_DW_INTERVAL		= 26,
    NAN_UNI_CMD_SET_SYNC_RSSI		= 39,
    NAN_UNI_CMD_SET_CLUSTER_ID		= 40,
    NAN_UNI_CMD_KEY_MANAGEMENT		= 53,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nan_uni_event_tag {
    NAN_UNI_EVENT_ID_DE_EVENT_IND		= 19,
    NAN_UNI_EVENT_REPORT_DW_END		= 60,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nan_disc_event_type {
    NAN_EVENT_ID_DISC_MAC_ADDR		= 0,
    NAN_EVENT_ID_JOINED_CLUSTER		= 2,
}

// NAN 4.0 Table 79. Device Capability attribute format, Supported Bands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nan_supported_bands {
    NAN_SUPPORTED_BAND_ID_2P4G = 2,
    NAN_SUPPORTED_BAND_ID_5G = 4,
    NAN_PROPRIETARY_BAND_ID_6G = 6,
    NAN_SUPPORTED_BAND_ID_6G = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nan_peer_supported_bands {
    NAN_SUPPORTED_BN_2G = 0,
    NAN_SUPPORTED_BN_5G_LOW,
    NAN_SUPPORTED_BN_5G_HIGH,
    NAN_SUPPORTED_BN_6G,
    NAN_SUPPORTED_BN_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_social_ch_scan_params {
    pub dwell_time: [u8; NAN_MAX_SOCIAL_CHANNELS],
    pub scan_period: [__le16; NAN_MAX_SOCIAL_CHANNELS],
    pub __packed: },
// Firmware-reported NAN device information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nan_dev_info_evt {
    pub is_enabled: u8,
    pub my_addr: [u8; ETH_ALEN],
    pub en_fw_election: u8,
    pub nan_dev_role: __le32,
    pub nan_dev_state: __le32,
    pub mst_preference: u8,
    pub random_factor: u8,
    pub cnt_hop: u8,
    pub cluster_id: [u8; ETH_ALEN],
    pub anchor_mst_addr: [u8; ETH_ALEN],
    pub am_preference: u8,
    pub am_random_factor: u8,
    pub parent_mac: [u8; ETH_ALEN],
    pub parent_am_preference: u8,
    pub parent_am_factor: u8,
    pub ambtt: __le32,
    pub tsf: [__le32; 2],
    pub pn_igtk: [u8; 6],
    pub pn_bigtk: [u8; 6],
}

// Firmware NAN discovery window event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nan_rpt_dw_evt {
    pub device_info: nan_dev_info_evt,
    pub expected_tsf_h: __le32,
    pub expected_tsf_l: __le32,
    pub actual_tsf_h: __le32,
    pub actual_tsf_l: __le32,
    pub channel: __le16,
    pub dw_num: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_conf_dw {
    pub config_2dot4g_dw_band: u8,
    pub dw_2dot4g_interval_val: __le32,
    pub config_5g_dw_band: u8,
    pub dw_5g_interval_val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_enable_req_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub master_pref: u8,
    pub cluster_low: __le16,
    pub cluster_high: __le16,
    pub config_support_5g: u8,
    pub support_5g_val: u8,
    pub config_sid_beacon: u8,
    pub sid_beacon_val: u8,
    pub config_2dot4g_rssi_close: u8,
    pub rssi_close_2dot4g_val: u8,
    pub config_2dot4g_rssi_middle: u8,
    pub rssi_middle_2dot4g_val: u8,
    pub config_2dot4g_rssi_proximity: u8,
    pub rssi_proximity_2dot4g_val: u8,
    pub config_hop_count_limit: u8,
    pub hop_count_limit_val: u8,
    pub config_2dot4g_support: u8,
    pub support_2dot4g_val: u8,
    pub config_2dot4g_beacons: u8,
    pub beacon_2dot4g_val: u8,
    pub config_2dot4g_sdf: u8,
    pub sdf_2dot4g_val: u8,
    pub config_5g_beacons: u8,
    pub beacon_5g_val: u8,
    pub config_5g_sdf: u8,
    pub sdf_5g_val: u8,
    pub config_5g_rssi_close: u8,
    pub rssi_close_5g_val: u8,
    pub config_5g_rssi_middle: u8,
    pub rssi_middle_5g_val: u8,
    pub config_5g_rssi_close_proximity: u8,
    pub rssi_close_proximity_5g_val: u8,
    pub config_rssi_window_size: u8,
    pub rssi_window_size_val: u8,
    pub config_oui: u8,
    pub oui_val: __le32,
    pub config_intf_addr: u8,
    pub intf_addr_val: [u8; ETH_ALEN],
    pub config_cluster_attribute_val: u8,
    pub config_scan_params: u8,
    pub scan_params_val: mt7925_nan_social_ch_scan_params,
    pub config_random_factor_force: u8,
    pub random_factor_force_val: u8,
    pub config_hop_count_force: u8,
    pub hop_count_force_val: u8,
    pub config_24g_channel: u8,
    pub channel_24g_val: __le32,
    pub config_5g_channel: u8,
    pub channel_5g_val: __le32,
    pub config_dw: mt7925_nan_conf_dw,
    pub config_disc_mac_addr_randomization: u8,
    pub disc_mac_addr_rand_interval_sec: __le32,
    pub discovery_indication_cfg: u8,
    pub config_subscribe_sid_beacon: u8,
    pub subscribe_sid_beacon_val: __le32,
    pub enable_log_slot_statistics: u8,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_common_hdr {
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_master_preference_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub master_preference: u8,
    pub reserved: [u8; 3],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_dw_interval_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub dw_interval: u8,
    pub vendor_ioctl: u8,
    pub disc_bcn_interval: __le16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_cluster_id_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub cluster_id: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sync_rssi_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub rssi_close_2g: i8,
    pub rssi_middle_2g: i8,
    pub rssi_close_5g: i8,
    pub rssi_middle_5g: i8,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_de_event {
    pub event_type: u8,
    pub cluster_id: [u8; ETH_ALEN],
    pub anchor_master_rank: [u8; NAN_ANCHOR_MASTER_RANK_NUM],
    pub own_nmi: [u8; ETH_ALEN],
    pub master_nmi: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_nmi_addr_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub nmi_addr: [u8; ETH_ALEN],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_avail_ctrl_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub avail_ctrl: __le16,
    pub seq_id: u8,
    pub reserved: [u8; 1],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_ch_timeline {
    pub is_valid: u8,
    pub reserved: [u8; 3],
    pub ch_info: __le32,
    pub num: __le32,
    pub avail_map: [__le32; NAN_TOTAL_DW],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_avail_entry_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub map_id: u8,
    pub is_cond_avail: u8,
    pub timeline_idx: u8,
    pub is_multi_map: u8,
    pub ch_list: [mt7925_nan_ch_timeline; NAN_TIMELINE_MGMT_CHNL_LIST_NUM],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sched_manage_peer_rec_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub sch_idx: __le32,
    pub is_activate: u8,
    pub nmi_addr: [u8; ETH_ALEN],
    pub reserved: [u8; 1],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sched_update_peer_cap_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub sch_idx: __le32,
    pub supported_bands: u8,
    pub max_chnl_switch_time: __le16,
    pub peer_supported_bands: u8,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sched_timeline {
    pub map_id: u8,
    pub local_map_id: u8,
    pub reserved: [u8; 2],
    pub avail_map: [__le32; NAN_TOTAL_DW],
    pub 4]: *mut *mut u8 avail_block[NAN_TOTAL_DW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sched_faw_ndc_timeline {
    pub avail_map: [__le32; NAN_TOTAL_DW],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sched_ndc_ctrl {
    pub is_valid: u8,
    pub ndc_id: [u8; NAN_NDC_ATTRIBUTE_ID_LENGTH],
    pub ndc_idx: u8,
    pub timeline: [mt7925_nan_sched_timeline; NAN_NUM_AVAIL_DB],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sched_update_crb_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub sch_idx: __le32,
    pub flags: u8,
    pub is_use_ranging: u8,
    pub reserved: [u8; 2],
    pub comm_ranging_timeline: [mt7925_nan_sched_timeline; NAN_TIMELINE_MGMT_SIZE],
    pub comm_faw_timeline: [mt7925_nan_sched_timeline; NAN_TIMELINE_MGMT_SIZE],
    pub comm_ndc_ctrl: mt7925_nan_sched_ndc_ctrl,
    pub faw_ndc_timeline: [mt7925_nan_sched_faw_ndc_timeline; NAN_TIMELINE_MGMT_SIZE],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_nan_sched_map_sta_rec_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub nmi_addr: [u8; ETH_ALEN],
    pub sta_rec_idx: u8,
    pub ndp_ctx_id: u8,
    pub role_idx: __le32,
    pub ndi_addr: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub __aligned(4): } __packed,
    pub conf): *mut cfg80211_nan_conf,
    pub dev): *mut mt792x_dev,
    pub conf): *mut cfg80211_nan_conf,
    pub skb): *mut *mut void mt7925_nan_mcu_event(struct mt792x_dev dev, struct sk_buff,
    pub addr): *const *const int mt7925_nan_set_nmi_addr(struct mt792x_dev dev, u8,
    pub vif): *mut ieee80211_vif,
    pub sta): *mut ieee80211_sta,
    pub sta): *mut ieee80211_sta,
    pub sta): *mut ieee80211_sta,
