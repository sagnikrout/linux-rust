//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/hif_api_cmd.h
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


// SPDX-License-Identifier: GPL-2.0-only or Apache-2.0
//
// WF200 hardware interface definitions
//
// Copyright (c) 2018-2020, Silicon Laboratories Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_requests_ids {
    HIF_REQ_ID_RESET                = 0x0a,
    HIF_REQ_ID_READ_MIB             = 0x05,
    HIF_REQ_ID_WRITE_MIB            = 0x06,
    HIF_REQ_ID_START_SCAN           = 0x07,
    HIF_REQ_ID_STOP_SCAN            = 0x08,
    HIF_REQ_ID_TX                   = 0x04,
    HIF_REQ_ID_JOIN                 = 0x0b,
    HIF_REQ_ID_SET_PM_MODE          = 0x10,
    HIF_REQ_ID_SET_BSS_PARAMS       = 0x11,
    HIF_REQ_ID_ADD_KEY              = 0x0c,
    HIF_REQ_ID_REMOVE_KEY           = 0x0d,
    HIF_REQ_ID_EDCA_QUEUE_PARAMS    = 0x13,
    HIF_REQ_ID_START                = 0x17,
    HIF_REQ_ID_BEACON_TRANSMIT      = 0x18,
    HIF_REQ_ID_UPDATE_IE            = 0x1b,
    HIF_REQ_ID_MAP_LINK             = 0x1c,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_confirmations_ids {
    HIF_CNF_ID_RESET                = 0x0a,
    HIF_CNF_ID_READ_MIB             = 0x05,
    HIF_CNF_ID_WRITE_MIB            = 0x06,
    HIF_CNF_ID_START_SCAN           = 0x07,
    HIF_CNF_ID_STOP_SCAN            = 0x08,
    HIF_CNF_ID_TX                   = 0x04,
    HIF_CNF_ID_MULTI_TRANSMIT       = 0x1e,
    HIF_CNF_ID_JOIN                 = 0x0b,
    HIF_CNF_ID_SET_PM_MODE          = 0x10,
    HIF_CNF_ID_SET_BSS_PARAMS       = 0x11,
    HIF_CNF_ID_ADD_KEY              = 0x0c,
    HIF_CNF_ID_REMOVE_KEY           = 0x0d,
    HIF_CNF_ID_EDCA_QUEUE_PARAMS    = 0x13,
    HIF_CNF_ID_START                = 0x17,
    HIF_CNF_ID_BEACON_TRANSMIT      = 0x18,
    HIF_CNF_ID_UPDATE_IE            = 0x1b,
    HIF_CNF_ID_MAP_LINK             = 0x1c,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_indications_ids {
    HIF_IND_ID_RX                   = 0x84,
    HIF_IND_ID_SCAN_CMPL            = 0x86,
    HIF_IND_ID_JOIN_COMPLETE        = 0x8f,
    HIF_IND_ID_SET_PM_MODE_CMPL     = 0x89,
    HIF_IND_ID_SUSPEND_RESUME_TX    = 0x8c,
    HIF_IND_ID_EVENT                = 0x85
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_reset {
    pub reset_stat:1: u8,
    pub reset_all_int:1: u8,
    pub reserved1:6: u8,
    pub reserved2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_reset {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_read_mib {
    pub mib_id: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_read_mib {
    pub status: __le32,
    pub mib_id: __le16,
    pub length: __le16,
    pub mib_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_write_mib {
    pub mib_id: __le16,
    pub length: __le16,
    pub mib_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_write_mib {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_update_ie {
    pub beacon:1: u8,
    pub probe_resp:1: u8,
    pub probe_req:1: u8,
    pub reserved1:5: u8,
    pub reserved2: u8,
    pub num_ies: __le16,
    pub ie: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_update_ie {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ssid_def {
    pub ssid_length: __le32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub __packed: },
pub const HIF_API_MAX_NB_SSIDS: c_int = 2;
pub const HIF_API_MAX_NB_CHANNELS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_start_scan_alt {
    pub band: u8,
    pub maintain_current_bss:1: u8,
    pub periodic:1: u8,
    pub reserved1:6: u8,
    pub disallow_ps:1: u8,
    pub reserved2:1: u8,
    pub short_preamble:1: u8,
    pub reserved3:5: u8,
    pub max_transmit_rate: u8,
    pub periodic_interval: __le16,
    pub reserved4: u8,
    pub periodic_rssi_thr: i8,
    pub num_of_probe_requests: u8,
    pub probe_delay: u8,
    pub num_of_ssids: u8,
    pub num_of_channels: u8,
    pub min_channel_time: __le32,
    pub max_channel_time: __le32,
    pub /: *mut *mut __le32 tx_power_level; / signed value,
    pub ssid_def: [wfx_hif_ssid_def; HIF_API_MAX_NB_SSIDS],
    pub channel_list: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_start_scan {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_stop_scan {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_pm_mode_status {
    HIF_PM_MODE_ACTIVE                         = 0x0,
    HIF_PM_MODE_PS                             = 0x1,
    HIF_PM_MODE_UNDETERMINED                   = 0x2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_scan_cmpl {
    pub status: __le32,
    pub pm_mode: u8,
    pub num_channels_completed: u8,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_queue_id {
    HIF_QUEUE_ID_BACKGROUND                    = 0x0,
    HIF_QUEUE_ID_BESTEFFORT                    = 0x1,
    HIF_QUEUE_ID_VIDEO                         = 0x2,
    HIF_QUEUE_ID_VOICE                         = 0x3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_frame_format {
    HIF_FRAME_FORMAT_NON_HT                    = 0x0,
    HIF_FRAME_FORMAT_MIXED_FORMAT_HT           = 0x1,
    HIF_FRAME_FORMAT_GF_HT_11N                 = 0x2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_tx {
// packet_id is not interpreted by the device, so it is not necessary to declare it little
// endian
//
    pub packet_id: u32,
    pub max_tx_rate: u8,
    pub queue_id:2: u8,
    pub peer_sta_id:4: u8,
    pub reserved1:2: u8,
    pub more:1: u8,
    pub fc_offset:3: u8,
    pub after_dtim:1: u8,
    pub reserved2:3: u8,
    pub start_exp:1: u8,
    pub reserved3:3: u8,
    pub retry_policy_index:4: u8,
    pub reserved4: __le32,
    pub expire_time: __le32,
    pub frame_format:4: u8,
    pub fec_coding:1: u8,
    pub short_gi:1: u8,
    pub reserved5:1: u8,
    pub stbc:1: u8,
    pub reserved6: u8,
    pub aggregation:1: u8,
    pub reserved7:7: u8,
    pub reserved8: u8,
    pub frame: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_qos_ackplcy {
    HIF_QOS_ACKPLCY_NORMAL                         = 0x0,
    HIF_QOS_ACKPLCY_TXNOACK                        = 0x1,
    HIF_QOS_ACKPLCY_NOEXPACK                       = 0x2,
    HIF_QOS_ACKPLCY_BLCKACK                        = 0x3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_tx {
    pub status: __le32,
// packet_id is copied from struct wfx_hif_req_tx without been interpreted by the device, so
// it is not necessary to declare it little endian
//
    pub packet_id: u32,
    pub txed_rate: u8,
    pub ack_failures: u8,
    pub aggr:1: u8,
    pub requeue:1: u8,
    pub ack_policy:2: u8,
    pub txop_limit:1: u8,
    pub reserved1:3: u8,
    pub reserved2: u8,
    pub media_delay: __le32,
    pub tx_queue_delay: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_multi_transmit {
    pub num_tx_confs: u8,
    pub reserved: [u8; 3],
    pub tx_conf_payload: [wfx_hif_cnf_tx; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_ri_flags_encrypt {
    HIF_RI_FLAGS_UNENCRYPTED                   = 0x0,
    HIF_RI_FLAGS_WEP_ENCRYPTED                 = 0x1,
    HIF_RI_FLAGS_TKIP_ENCRYPTED                = 0x2,
    HIF_RI_FLAGS_AES_ENCRYPTED                 = 0x3,
    HIF_RI_FLAGS_WAPI_ENCRYPTED                = 0x4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_rx {
    pub status: __le32,
    pub channel_number: u8,
    pub reserved1: u8,
    pub rxed_rate: u8,
    pub rcpi_rssi: u8,
    pub encryp:3: u8,
    pub in_aggr:1: u8,
    pub first_aggr:1: u8,
    pub last_aggr:1: u8,
    pub defrag:1: u8,
    pub beacon:1: u8,
    pub tim:1: u8,
    pub bitmap:1: u8,
    pub match_ssid:1: u8,
    pub match_bssid:1: u8,
    pub more:1: u8,
    pub reserved2:1: u8,
    pub ht:1: u8,
    pub stbc:1: u8,
    pub match_uc_addr:1: u8,
    pub match_mc_addr:1: u8,
    pub match_bc_addr:1: u8,
    pub key_type:1: u8,
    pub key_index:4: u8,
    pub reserved3:1: u8,
    pub peer_sta_id:4: u8,
    pub reserved4:2: u8,
    pub reserved5:1: u8,
    pub frame: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_edca_queue_params {
    pub queue_id: u8,
    pub reserved1: u8,
    pub aifsn: u8,
    pub reserved2: u8,
    pub cw_min: __le16,
    pub cw_max: __le16,
    pub tx_op_limit: __le16,
    pub allowed_medium_time: __le16,
    pub reserved3: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_edca_queue_params {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_join {
    pub infrastructure_bss_mode:1: u8,
    pub reserved1:7: u8,
    pub band: u8,
    pub channel_number: u8,
    pub reserved2: u8,
    pub bssid: [u8; ETH_ALEN],
    pub atim_window: __le16,
    pub short_preamble:1: u8,
    pub reserved3:7: u8,
    pub probe_for_join: u8,
    pub reserved4: u8,
    pub reserved5:2: u8,
    pub force_no_beacon:1: u8,
    pub force_with_ind:1: u8,
    pub reserved6:4: u8,
    pub ssid_length: __le32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub beacon_interval: __le32,
    pub basic_rate_set: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_join {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_join_complete {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_set_bss_params {
    pub lost_count_only:1: u8,
    pub reserved:7: u8,
    pub beacon_lost_count: u8,
    pub aid: __le16,
    pub operational_rate_set: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_set_bss_params {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_set_pm_mode {
    pub enter_psm:1: u8,
    pub reserved:6: u8,
    pub fast_psm:1: u8,
    pub fast_psm_idle_period: u8,
    pub ap_psm_change_period: u8,
    pub min_auto_ps_poll_period: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_set_pm_mode {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_set_pm_mode_cmpl {
    pub status: __le32,
    pub pm_mode: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_start {
    pub mode: u8,
    pub band: u8,
    pub channel_number: u8,
    pub reserved1: u8,
    pub reserved2: __le32,
    pub beacon_interval: __le32,
    pub dtim_period: u8,
    pub short_preamble:1: u8,
    pub reserved3:7: u8,
    pub reserved4: u8,
    pub ssid_length: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub basic_rate_set: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_start {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_beacon_transmit {
    pub enable_beaconing: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_beacon_transmit {
    pub status: __le32,
    pub __packed: },
pub const HIF_LINK_ID_MAX: c_int = 14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_map_link {
    pub mac_addr: [u8; ETH_ALEN],
    pub unmap:1: u8,
    pub mfpc:1: u8,
    pub reserved:6: u8,
    pub peer_sta_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_map_link {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_suspend_resume_tx {
    pub resume:1: u8,
    pub reserved1:2: u8,
    pub bc_mc_only:1: u8,
    pub reserved2:4: u8,
    pub reserved3: u8,
    pub peer_sta_set: __le16,
    pub __packed: },
pub const MAX_KEY_ENTRIES: c_int = 24;
pub const HIF_API_WEP_KEY_DATA_SIZE: c_int = 16;
pub const HIF_API_TKIP_KEY_DATA_SIZE: c_int = 16;
pub const HIF_API_RX_MIC_KEY_SIZE: c_int = 8;
pub const HIF_API_TX_MIC_KEY_SIZE: c_int = 8;
pub const HIF_API_AES_KEY_DATA_SIZE: c_int = 16;
pub const HIF_API_WAPI_KEY_DATA_SIZE: c_int = 16;
pub const HIF_API_MIC_KEY_DATA_SIZE: c_int = 16;
pub const HIF_API_IGTK_KEY_DATA_SIZE: c_int = 16;
pub const HIF_API_RX_SEQUENCE_COUNTER_SIZE: c_int = 8;
pub const HIF_API_IPN_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_key_type {
    HIF_KEY_TYPE_WEP_DEFAULT                   = 0x0,
    HIF_KEY_TYPE_WEP_PAIRWISE                  = 0x1,
    HIF_KEY_TYPE_TKIP_GROUP                    = 0x2,
    HIF_KEY_TYPE_TKIP_PAIRWISE                 = 0x3,
    HIF_KEY_TYPE_AES_GROUP                     = 0x4,
    HIF_KEY_TYPE_AES_PAIRWISE                  = 0x5,
    HIF_KEY_TYPE_WAPI_GROUP                    = 0x6,
    HIF_KEY_TYPE_WAPI_PAIRWISE                 = 0x7,
    HIF_KEY_TYPE_IGTK_GROUP                    = 0x8,
    HIF_KEY_TYPE_NONE                          = 0x9
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_wep_pairwise_key {
    pub peer_address: [u8; ETH_ALEN],
    pub reserved: u8,
    pub key_length: u8,
    pub key_data: [u8; HIF_API_WEP_KEY_DATA_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_wep_group_key {
    pub key_id: u8,
    pub key_length: u8,
    pub reserved: [u8; 2],
    pub key_data: [u8; HIF_API_WEP_KEY_DATA_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_tkip_pairwise_key {
    pub peer_address: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub tkip_key_data: [u8; HIF_API_TKIP_KEY_DATA_SIZE],
    pub rx_mic_key: [u8; HIF_API_RX_MIC_KEY_SIZE],
    pub tx_mic_key: [u8; HIF_API_TX_MIC_KEY_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_tkip_group_key {
    pub tkip_key_data: [u8; HIF_API_TKIP_KEY_DATA_SIZE],
    pub rx_mic_key: [u8; HIF_API_RX_MIC_KEY_SIZE],
    pub key_id: u8,
    pub reserved: [u8; 3],
    pub rx_sequence_counter: [u8; HIF_API_RX_SEQUENCE_COUNTER_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_aes_pairwise_key {
    pub peer_address: [u8; ETH_ALEN],
    pub reserved: [u8; 2],
    pub aes_key_data: [u8; HIF_API_AES_KEY_DATA_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_aes_group_key {
    pub aes_key_data: [u8; HIF_API_AES_KEY_DATA_SIZE],
    pub key_id: u8,
    pub reserved: [u8; 3],
    pub rx_sequence_counter: [u8; HIF_API_RX_SEQUENCE_COUNTER_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_wapi_pairwise_key {
    pub peer_address: [u8; ETH_ALEN],
    pub key_id: u8,
    pub reserved: u8,
    pub wapi_key_data: [u8; HIF_API_WAPI_KEY_DATA_SIZE],
    pub mic_key_data: [u8; HIF_API_MIC_KEY_DATA_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_wapi_group_key {
    pub wapi_key_data: [u8; HIF_API_WAPI_KEY_DATA_SIZE],
    pub mic_key_data: [u8; HIF_API_MIC_KEY_DATA_SIZE],
    pub key_id: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_igtk_group_key {
    pub igtk_key_data: [u8; HIF_API_IGTK_KEY_DATA_SIZE],
    pub key_id: u8,
    pub reserved: [u8; 3],
    pub ipn: [u8; HIF_API_IPN_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_add_key {
    pub type: u8,
    pub entry_index: u8,
    pub int_id:2: u8,
    pub reserved1:6: u8,
    pub reserved2: u8,
    pub wep_pairwise_key: wfx_hif_wep_pairwise_key,
    pub wep_group_key: wfx_hif_wep_group_key,
    pub tkip_pairwise_key: wfx_hif_tkip_pairwise_key,
    pub tkip_group_key: wfx_hif_tkip_group_key,
    pub aes_pairwise_key: wfx_hif_aes_pairwise_key,
    pub aes_group_key: wfx_hif_aes_group_key,
    pub wapi_pairwise_key: wfx_hif_wapi_pairwise_key,
    pub wapi_group_key: wfx_hif_wapi_group_key,
    pub igtk_group_key: wfx_hif_igtk_group_key,
    pub key: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_add_key {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_remove_key {
    pub entry_index: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_remove_key {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_event_ind {
    HIF_EVENT_IND_BSSLOST                      = 0x1,
    HIF_EVENT_IND_BSSREGAINED                  = 0x2,
    HIF_EVENT_IND_RCPI_RSSI                    = 0x3,
    HIF_EVENT_IND_PS_MODE_ERROR                = 0x4,
    HIF_EVENT_IND_INACTIVITY                   = 0x5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_ps_mode_error {
    HIF_PS_ERROR_NO_ERROR                      = 0,
    HIF_PS_ERROR_AP_NOT_RESP_TO_POLL           = 1,
    HIF_PS_ERROR_AP_NOT_RESP_TO_UAPSD_TRIGGER  = 2,
    HIF_PS_ERROR_AP_SENT_UNICAST_IN_DOZE       = 3,
    HIF_PS_ERROR_AP_NO_DATA_AFTER_TIM          = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_event {
    pub event_id: __le32,
    pub rcpi_rssi: u8,
    pub ps_mode_error: __le32,
    pub peer_sta_set: __le32,
    pub event_data: },
    pub __packed: },
