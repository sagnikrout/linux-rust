//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/scan.h
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
//
// Copyright (C) 2012-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_scan_h__
// Scan Commands, Responses, Notifications
//
// enum iwl_scan_subcmd_ids - scan commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_subcmd_ids {
//
// @CHANNEL_SURVEY_NOTIF: &struct iwl_umac_scan_channel_survey_notif
//
    CHANNEL_SURVEY_NOTIF = 0xFB,
//
// @OFFLOAD_MATCH_INFO_NOTIF: &struct iwl_scan_offload_match_info
//
    OFFLOAD_MATCH_INFO_NOTIF = 0xFC,
}

// Max number of IEs for direct SSID scans in a command
pub const PROBE_OPTION_MAX: c_int = 20;
pub const SCAN_SHORT_SSID_MAX_SIZE: c_int = 8;
pub const SCAN_BSSID_MAX_SIZE: c_int = 16;
//
// struct iwl_ssid_ie - directed scan network information element
//
// Up to 20 of these may appear in REPLY_SCAN_CMD,
// selected by "type" bit field in struct iwl_scan_channel;
// each channel may select different ssids from among the 20 entries.
// SSID IEs get transmitted in reverse order of entry.
//
// @id: element ID
// @len: element length
// @ssid: element (SSID) data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ssid_ie {
    pub id: u8,
    pub len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub /: *mut *mut } __packed; / SCAN_DIRECT_SSID_IE_API_S_VER_1,
// scan offload
pub const IWL_SCAN_MAX_BLACKLIST_LEN: c_int = 64;
pub const IWL_SCAN_SHORT_BLACKLIST_LEN: c_int = 16;
pub const IWL_SCAN_MAX_PROFILES: c_int = 11;
pub const IWL_SCAN_MAX_PROFILES_V2: c_int = 8;
pub const SCAN_OFFLOAD_PROBE_REQ_SIZE: c_int = 512;
pub const SCAN_NUM_BAND_PROBE_DATA_V_1: c_int = 2;
pub const SCAN_NUM_BAND_PROBE_DATA_V_2: c_int = 3;
// Default watchdog (in MS) for scheduled scan iteration

pub const CAN_ABORT_STATUS: c_int = 1;
pub const IWL_FULL_SCAN_MULTIPLIER: c_int = 5;
pub const IWL_FAST_SCHED_SCAN_ITERATIONS: c_int = 3;
pub const IWL_MAX_SCHED_SCAN_PLANS: c_int = 2;
pub const IWL_MAX_NUM_NOISE_RESULTS: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_framework_client {
    SCAN_CLIENT_SCHED_SCAN		= BIT(0),
    SCAN_CLIENT_NETDETECT		= BIT(1),
    SCAN_CLIENT_ASSET_TRACKING	= BIT(2),
}

//
// struct iwl_scan_offload_blocklist - SCAN_OFFLOAD_BLACKLIST_S
// @ssid:		MAC address to filter out
// @reported_rssi:	AP rssi reported to the host
// @client_bitmap: clients ignore this entry  - enum scan_framework_client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_blocklist {
    pub ssid: [u8; ETH_ALEN],
    pub reported_rssi: u8,
    pub client_bitmap: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_offload_network_type {
    IWL_NETWORK_TYPE_BSS	= 1,
    IWL_NETWORK_TYPE_IBSS	= 2,
    IWL_NETWORK_TYPE_ANY	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_offload_band_selection {
    IWL_SCAN_OFFLOAD_SELECT_2_4	= 0x4,
    IWL_SCAN_OFFLOAD_SELECT_5_2	= 0x8,
    IWL_SCAN_OFFLOAD_SELECT_ANY	= 0xc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_offload_auth_alg {
    IWL_AUTH_ALGO_UNSUPPORTED  = 0x00,
    IWL_AUTH_ALGO_NONE         = 0x01,
    IWL_AUTH_ALGO_PSK          = 0x02,
    IWL_AUTH_ALGO_8021X        = 0x04,
    IWL_AUTH_ALGO_SAE          = 0x08,
    IWL_AUTH_ALGO_8021X_SHA384 = 0x10,
    IWL_AUTH_ALGO_OWE          = 0x20,
}

//
// struct iwl_scan_offload_profile - SCAN_OFFLOAD_PROFILE_S
// @ssid_index:		index to ssid list in fixed part
// @unicast_cipher:	encryption algorithm to match - bitmap
// @auth_alg:		authentication algorithm to match - bitmap
// @network_type:	enum iwl_scan_offload_network_type
// @band_selection:	enum iwl_scan_offload_band_selection
// @client_bitmap:	clients waiting for match - enum scan_framework_client
// @reserved:		reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_profile {
    pub ssid_index: u8,
    pub unicast_cipher: u8,
    pub auth_alg: u8,
    pub network_type: u8,
    pub band_selection: u8,
    pub client_bitmap: u8,
    pub reserved: [u8; 2],
    pub __packed: },
//
// struct iwl_scan_offload_profile_cfg_data - scan offload profile configs
// @blocklist_len:	length of blocklist
// @num_profiles:	num of profiles in the list
// @match_notify:	clients waiting for match found notification
// @pass_match:		clients waiting for the results
// @active_clients:	active clients bitmap - enum scan_framework_client
// @any_beacon_notify:	clients waiting for match notification without match
// @reserved:		reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_profile_cfg_data {
    pub blocklist_len: u8,
    pub num_profiles: u8,
    pub match_notify: u8,
    pub pass_match: u8,
    pub active_clients: u8,
    pub any_beacon_notify: u8,
    pub reserved: [u8; 2],
    pub __packed: },
//
// struct iwl_scan_offload_profile_cfg_v1 - scan offload profile config
// @profiles:	profiles to search for match
// @data:	the rest of the data for profile_cfg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_profile_cfg_v1 {
    pub profiles: [iwl_scan_offload_profile; IWL_SCAN_MAX_PROFILES],
    pub data: iwl_scan_offload_profile_cfg_data,
    pub SCAN_OFFLOAD_PROFILES_CFG_API_S_VER_1-2*/: *mut *mut } __packed; /,
//
// struct iwl_scan_offload_profile_cfg - scan offload profile config
// @profiles:	profiles to search for match
// @data:	the rest of the data for profile_cfg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_profile_cfg {
    pub profiles: [iwl_scan_offload_profile; IWL_SCAN_MAX_PROFILES_V2],
    pub data: iwl_scan_offload_profile_cfg_data,
    pub SCAN_OFFLOAD_PROFILES_CFG_API_S_VER_3*/: *mut *mut } __packed; /,
//
// struct iwl_scan_schedule_lmac - schedule of scan offload
// @delay:		delay between iterations, in seconds.
// @iterations:		num of scan iterations
// @full_scan_mul:	number of partial scans before each full scan
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_schedule_lmac {
    pub delay: __le16,
    pub iterations: u8,
    pub full_scan_mul: u8,
    pub /: *mut *mut } __packed; / SCAN_SCHEDULE_API_S,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_offload_complete_status {
    IWL_SCAN_OFFLOAD_COMPLETED	= 1,
    IWL_SCAN_OFFLOAD_ABORTED	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_ebs_status {
    IWL_SCAN_EBS_SUCCESS,
    IWL_SCAN_EBS_FAILED,
    IWL_SCAN_EBS_CHAN_NOT_FOUND,
    IWL_SCAN_EBS_INACTIVE,
}

//
// struct iwl_scan_req_tx_cmd - SCAN_REQ_TX_CMD_API_S
// @tx_flags: combination of TX_CMD_FLG_
// @rate_n_flags: rate for *all* Tx attempts, if TX_CMD_FLG_STA_RATE_MSK is
// cleared. Combination of RATE_MCS_
// @sta_id: index of destination station in FW station table
// @reserved: for alignment and future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_tx_cmd {
    pub tx_flags: __le32,
    pub rate_n_flags: __le32,
    pub sta_id: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_channel_flags_lmac {
    IWL_UNIFIED_SCAN_CHANNEL_FULL		= BIT(27),
    IWL_UNIFIED_SCAN_CHANNEL_PARTIAL	= BIT(28),
}

//
// struct iwl_scan_channel_cfg_lmac - SCAN_CHANNEL_CFG_S_VER2
// @flags:		bits 1-20: directed scan to i'th ssid
// other bits &enum iwl_scan_channel_flags_lmac
// @channel_num:	channel number 1-13 etc
// @iter_count:		scan iteration on this channel
// @iter_interval:	interval in seconds between iterations on one channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_channel_cfg_lmac {
    pub flags: __le32,
    pub channel_num: __le16,
    pub iter_count: __le16,
    pub iter_interval: __le32,
    pub __packed: },
//
// struct iwl_scan_probe_segment - PROBE_SEGMENT_API_S_VER_1
// @offset: offset in the data block
// @len: length of the segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_probe_segment {
    pub offset: __le16,
    pub len: __le16,
    pub __packed: },
//
// struct iwl_scan_probe_req_v1 - PROBE_REQUEST_FRAME_API_S_VER_2
// @mac_header: first (and common) part of the probe
// @band_data: band specific data
// @common_data: last (and common) part of the probe
// @buf: raw data block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_probe_req_v1 {
    pub mac_header: iwl_scan_probe_segment,
    pub band_data: [iwl_scan_probe_segment; SCAN_NUM_BAND_PROBE_DATA_V_1],
    pub common_data: iwl_scan_probe_segment,
    pub buf: [u8; SCAN_OFFLOAD_PROBE_REQ_SIZE],
    pub __packed: },
//
// struct iwl_scan_probe_req - PROBE_REQUEST_FRAME_API_S_VER_v2
// @mac_header: first (and common) part of the probe
// @band_data: band specific data
// @common_data: last (and common) part of the probe
// @buf: raw data block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_probe_req {
    pub mac_header: iwl_scan_probe_segment,
    pub band_data: [iwl_scan_probe_segment; SCAN_NUM_BAND_PROBE_DATA_V_2],
    pub common_data: iwl_scan_probe_segment,
    pub buf: [u8; SCAN_OFFLOAD_PROBE_REQ_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_channel_flags {
    IWL_SCAN_CHANNEL_FLAG_EBS		= BIT(0),
    IWL_SCAN_CHANNEL_FLAG_EBS_ACCURATE	= BIT(1),
    IWL_SCAN_CHANNEL_FLAG_CACHE_ADD		= BIT(2),
    IWL_SCAN_CHANNEL_FLAG_EBS_FRAG		= BIT(3),
    IWL_SCAN_CHANNEL_FLAG_FORCE_EBS         = BIT(4),
    IWL_SCAN_CHANNEL_FLAG_ENABLE_CHAN_ORDER = BIT(5),
    IWL_SCAN_CHANNEL_FLAG_6G_PSC_NO_FILTER  = BIT(6),
}

//
// struct iwl_scan_channel_opt - CHANNEL_OPTIMIZATION_API_S
// @flags: enum iwl_scan_channel_flags
// @non_ebs_ratio: defines the ratio of number of scan iterations where EBS is
// involved.
// 1 - EBS is disabled.
// 2 - every second scan will be full scan(and so on).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_channel_opt {
    pub flags: __le16,
    pub non_ebs_ratio: __le16,
    pub __packed: },
//
// enum iwl_mvm_lmac_scan_flags - LMAC scan flags
// @IWL_MVM_LMAC_SCAN_FLAG_PASS_ALL: pass all beacons and probe responses
// without filtering.
// @IWL_MVM_LMAC_SCAN_FLAG_PASSIVE: force passive scan on all channels
// @IWL_MVM_LMAC_SCAN_FLAG_PRE_CONNECTION: single channel scan
// @IWL_MVM_LMAC_SCAN_FLAG_ITER_COMPLETE: send iteration complete notification
// @IWL_MVM_LMAC_SCAN_FLAG_MULTIPLE_SSIDS: multiple SSID matching
// @IWL_MVM_LMAC_SCAN_FLAG_FRAGMENTED: all passive scans will be fragmented
// @IWL_MVM_LMAC_SCAN_FLAGS_RRM_ENABLED: insert WFA vendor-specific TPC report
// and DS parameter set IEs into probe requests.
// @IWL_MVM_LMAC_SCAN_FLAG_EXTENDED_DWELL: use extended dwell time on channels
// 1, 6 and 11.
// @IWL_MVM_LMAC_SCAN_FLAG_MATCH: Send match found notification on matches
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_lmac_scan_flags {
    IWL_MVM_LMAC_SCAN_FLAG_PASS_ALL		= BIT(0),
    IWL_MVM_LMAC_SCAN_FLAG_PASSIVE		= BIT(1),
    IWL_MVM_LMAC_SCAN_FLAG_PRE_CONNECTION	= BIT(2),
    IWL_MVM_LMAC_SCAN_FLAG_ITER_COMPLETE	= BIT(3),
    IWL_MVM_LMAC_SCAN_FLAG_MULTIPLE_SSIDS	= BIT(4),
    IWL_MVM_LMAC_SCAN_FLAG_FRAGMENTED	= BIT(5),
    IWL_MVM_LMAC_SCAN_FLAGS_RRM_ENABLED	= BIT(6),
    IWL_MVM_LMAC_SCAN_FLAG_EXTENDED_DWELL	= BIT(7),
    IWL_MVM_LMAC_SCAN_FLAG_MATCH		= BIT(9),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_priority {
    IWL_SCAN_PRIORITY_LOW,
    IWL_SCAN_PRIORITY_MEDIUM,
    IWL_SCAN_PRIORITY_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_priority_ext {
    IWL_SCAN_PRIORITY_EXT_0_LOWEST,
    IWL_SCAN_PRIORITY_EXT_1,
    IWL_SCAN_PRIORITY_EXT_2,
    IWL_SCAN_PRIORITY_EXT_3,
    IWL_SCAN_PRIORITY_EXT_4,
    IWL_SCAN_PRIORITY_EXT_5,
    IWL_SCAN_PRIORITY_EXT_6,
    IWL_SCAN_PRIORITY_EXT_7_HIGHEST,
}

//
// struct iwl_scan_req_lmac - SCAN_REQUEST_CMD_API_S_VER_1
// @reserved1: for alignment and future use
// @n_channels: num of channels to scan
// @active_dwell: dwell time for active channels
// @passive_dwell: dwell time for passive channels
// @fragmented_dwell: dwell time for fragmented passive scan
// @extended_dwell: dwell time for channels 1, 6 and 11 (in certain cases)
// @reserved2: for alignment and future use
// @rx_chain_select: PHY_RX_CHAIN_* flags
// @scan_flags: &enum iwl_mvm_lmac_scan_flags
// @max_out_time: max time (in TU) to be out of associated channel
// @suspend_time: pause scan this long (TUs) when returning to service channel
// @flags: RXON flags
// @filter_flags: RXON filter
// @tx_cmd: tx command for active scan; for 2GHz and for 5GHz
// @direct_scan: list of SSIDs for directed active scan
// @scan_prio: enum iwl_scan_priority
// @iter_num: number of scan iterations
// @delay: delay in seconds before first iteration
// @schedule: two scheduling plans. The first one is finite, the second one can
// be infinite.
// @channel_opt: channel optimization options, for full and partial scan
// @data: channel configuration and probe request packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_lmac {
// SCAN_REQUEST_FIXED_PART_API_S_VER_7
    pub reserved1: __le32,
    pub n_channels: u8,
    pub active_dwell: u8,
    pub passive_dwell: u8,
    pub fragmented_dwell: u8,
    pub extended_dwell: u8,
    pub reserved2: u8,
    pub rx_chain_select: __le16,
    pub scan_flags: __le32,
    pub max_out_time: __le32,
    pub suspend_time: __le32,
// RX_ON_FLAGS_API_S_VER_1
    pub flags: __le32,
    pub filter_flags: __le32,
    pub tx_cmd: [iwl_scan_req_tx_cmd; 2],
    pub direct_scan: [iwl_ssid_ie; PROBE_OPTION_MAX],
    pub scan_prio: __le32,
// SCAN_REQ_PERIODIC_PARAMS_API_S
    pub iter_num: __le32,
    pub delay: __le32,
    pub schedule: [iwl_scan_schedule_lmac; IWL_MAX_SCHED_SCAN_PLANS],
    pub channel_opt: [iwl_scan_channel_opt; 2],
    pub data: [u8; ],
    pub __packed: },
//
// struct iwl_scan_results_notif - scan results for one channel -
// SCAN_RESULT_NTF_API_S_VER_3
// @channel: which channel the results are from
// @band: 0 for 5.2 GHz, 1 for 2.4 GHz
// @probe_status: SCAN_PROBE_STATUS_*, indicates success of probe request
// @num_probe_not_sent: # of request that weren't sent due to not enough time
// @duration: duration spent in channel, in usecs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_results_notif {
    pub channel: u8,
    pub band: u8,
    pub probe_status: u8,
    pub num_probe_not_sent: u8,
    pub duration: __le32,
    pub __packed: },
//
// struct iwl_lmac_scan_complete_notif - notifies end of scanning (all channels)
// SCAN_COMPLETE_NTF_API_S_VER_3
// @scanned_channels: number of channels scanned (and number of valid results)
// @status: one of SCAN_COMP_STATUS_
// @bt_status: BT on/off status
// @last_channel: last channel that was scanned
// @tsf_low: TSF timer (lower half) in usecs
// @tsf_high: TSF timer (higher half) in usecs
// @results: an array of scan results, only "scanned_channels" of them are valid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lmac_scan_complete_notif {
    pub scanned_channels: u8,
    pub status: u8,
    pub bt_status: u8,
    pub last_channel: u8,
    pub tsf_low: __le32,
    pub tsf_high: __le32,
    pub results: [iwl_scan_results_notif; ],
    pub __packed: },
//
// struct iwl_periodic_scan_complete - PERIODIC_SCAN_COMPLETE_NTF_API_S_VER_2
// @last_schedule_line: last schedule line executed (fast or regular)
// @last_schedule_iteration: last scan iteration executed before scan abort
// @status: &enum iwl_scan_offload_complete_status
// @ebs_status: EBS success status &enum iwl_scan_ebs_status
// @time_after_last_iter: time in seconds elapsed after last iteration
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_periodic_scan_complete {
    pub last_schedule_line: u8,
    pub last_schedule_iteration: u8,
    pub status: u8,
    pub ebs_status: u8,
    pub time_after_last_iter: __le32,
    pub reserved: __le32,
    pub __packed: },
// UMAC Scan API
// The maximum of either of these cannot exceed 8, because we use an
// 8-bit mask (see enum iwl_scan_status).
//
pub const IWL_MAX_UMAC_SCANS: c_int = 4;
pub const IWL_MAX_LMAC_SCANS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_config_flags {
    SCAN_CONFIG_FLAG_ACTIVATE			= BIT(0),
    SCAN_CONFIG_FLAG_DEACTIVATE			= BIT(1),
    SCAN_CONFIG_FLAG_FORBID_CHUB_REQS		= BIT(2),
    SCAN_CONFIG_FLAG_ALLOW_CHUB_REQS		= BIT(3),
    SCAN_CONFIG_FLAG_SET_TX_CHAINS			= BIT(8),
    SCAN_CONFIG_FLAG_SET_RX_CHAINS			= BIT(9),
    SCAN_CONFIG_FLAG_SET_AUX_STA_ID			= BIT(10),
    SCAN_CONFIG_FLAG_SET_ALL_TIMES			= BIT(11),
    SCAN_CONFIG_FLAG_SET_EFFECTIVE_TIMES		= BIT(12),
    SCAN_CONFIG_FLAG_SET_CHANNEL_FLAGS		= BIT(13),
    SCAN_CONFIG_FLAG_SET_LEGACY_RATES		= BIT(14),
    SCAN_CONFIG_FLAG_SET_MAC_ADDR			= BIT(15),
    SCAN_CONFIG_FLAG_SET_FRAGMENTED			= BIT(16),
    SCAN_CONFIG_FLAG_CLEAR_FRAGMENTED		= BIT(17),
    SCAN_CONFIG_FLAG_SET_CAM_MODE			= BIT(18),
    SCAN_CONFIG_FLAG_CLEAR_CAM_MODE			= BIT(19),
    SCAN_CONFIG_FLAG_SET_PROMISC_MODE		= BIT(20),
    SCAN_CONFIG_FLAG_CLEAR_PROMISC_MODE		= BIT(21),
    SCAN_CONFIG_FLAG_SET_LMAC2_FRAGMENTED		= BIT(22),
    SCAN_CONFIG_FLAG_CLEAR_LMAC2_FRAGMENTED		= BIT(23),

// Bits 26-31 are for num of channels in channel_array

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_config_rates {
// OFDM basic rates
    SCAN_CONFIG_RATE_6M	= BIT(0),
    SCAN_CONFIG_RATE_9M	= BIT(1),
    SCAN_CONFIG_RATE_12M	= BIT(2),
    SCAN_CONFIG_RATE_18M	= BIT(3),
    SCAN_CONFIG_RATE_24M	= BIT(4),
    SCAN_CONFIG_RATE_36M	= BIT(5),
    SCAN_CONFIG_RATE_48M	= BIT(6),
    SCAN_CONFIG_RATE_54M	= BIT(7),
// CCK basic rates
    SCAN_CONFIG_RATE_1M	= BIT(8),
    SCAN_CONFIG_RATE_2M	= BIT(9),
    SCAN_CONFIG_RATE_5M	= BIT(10),
    SCAN_CONFIG_RATE_11M	= BIT(11),

// Bits 16-27 are for supported rates

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_channel_flags {
    IWL_CHANNEL_FLAG_EBS				= BIT(0),
    IWL_CHANNEL_FLAG_ACCURATE_EBS			= BIT(1),
    IWL_CHANNEL_FLAG_EBS_ADD			= BIT(2),
    IWL_CHANNEL_FLAG_PRE_SCAN_PASSIVE2ACTIVE	= BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_uhb_chan_cfg_flags {
    IWL_UHB_CHAN_CFG_FLAG_UNSOLICITED_PROBE_RES = BIT(24),
    IWL_UHB_CHAN_CFG_FLAG_PSC_CHAN_NO_LISTEN    = BIT(25),
    IWL_UHB_CHAN_CFG_FLAG_FORCE_PASSIVE         = BIT(26),
}

//
// struct iwl_scan_dwell - scan dwell configuration
// @active:		default dwell time for active scan
// @passive:		default dwell time for passive scan
// @fragmented:		default dwell time for fragmented scan
// @extended:		default dwell time for channels 1, 6 and 11
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_dwell {
    pub active: u8,
    pub passive: u8,
    pub fragmented: u8,
    pub extended: u8,
    pub __packed: },
//
// struct iwl_scan_config_v1 - scan configuration command
// @flags:			enum scan_config_flags
// @tx_chains:			valid_tx antenna - ANT_* definitions
// @rx_chains:			valid_rx antenna - ANT_* definitions
// @legacy_rates:		default legacy rates - enum scan_config_rates
// @out_of_channel_time:	default max out of serving channel time
// @suspend_time:		default max suspend time
// @dwell:			dwells for the scan
// @mac_addr:			default mac address to be used in probes
// @bcast_sta_id:		the index of the station in the fw
// @channel_flags:		default channel flags - enum iwl_channel_flags
// scan_config_channel_flag
// @channel_array:		default supported channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_config_v1 {
    pub flags: __le32,
    pub tx_chains: __le32,
    pub rx_chains: __le32,
    pub legacy_rates: __le32,
    pub out_of_channel_time: __le32,
    pub suspend_time: __le32,
    pub dwell: iwl_scan_dwell,
    pub mac_addr: [u8; ETH_ALEN],
    pub bcast_sta_id: u8,
    pub channel_flags: u8,
    pub channel_array: [u8; ],
    pub /: *mut *mut } __packed; / SCAN_CONFIG_DB_CMD_API_S,
pub const SCAN_TWO_LMACS: c_int = 2;
pub const SCAN_LB_LMAC_IDX: c_int = 0;
pub const SCAN_HB_LMAC_IDX: c_int = 1;
//
// struct iwl_scan_config_v2 - scan configuration command
// @flags:			enum scan_config_flags
// @tx_chains:			valid_tx antenna - ANT_* definitions
// @rx_chains:			valid_rx antenna - ANT_* definitions
// @legacy_rates:		default legacy rates - enum scan_config_rates
// @out_of_channel_time:	default max out of serving channel time
// @suspend_time:		default max suspend time
// @dwell:			dwells for the scan
// @mac_addr:			default mac address to be used in probes
// @bcast_sta_id:		the index of the station in the fw
// @channel_flags:		default channel flags - enum iwl_channel_flags
// scan_config_channel_flag
// @channel_array:		default supported channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_config_v2 {
    pub flags: __le32,
    pub tx_chains: __le32,
    pub rx_chains: __le32,
    pub legacy_rates: __le32,
    pub out_of_channel_time: [__le32; SCAN_TWO_LMACS],
    pub suspend_time: [__le32; SCAN_TWO_LMACS],
    pub dwell: iwl_scan_dwell,
    pub mac_addr: [u8; ETH_ALEN],
    pub bcast_sta_id: u8,
    pub channel_flags: u8,
    pub channel_array: [u8; ],
    pub /: *mut *mut } __packed; / SCAN_CONFIG_DB_CMD_API_S_2,
//
// struct iwl_scan_config - scan configuration command
// @enable_cam_mode: whether to enable CAM mode.
// @enable_promiscouos_mode: whether to enable promiscouos mode
// @bcast_sta_id: the index of the station in the fw. Deprecated starting with
// API version 5.
// @reserved: reserved
// @tx_chains: valid_tx antenna - ANT_* definitions
// @rx_chains: valid_rx antenna - ANT_* definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_config {
    pub enable_cam_mode: u8,
    pub enable_promiscouos_mode: u8,
    pub bcast_sta_id: u8,
    pub reserved: u8,
    pub tx_chains: __le32,
    pub rx_chains: __le32,
    pub /: *mut *mut } __packed; / SCAN_CONFIG_DB_CMD_API_S_5,
//
// enum iwl_umac_scan_flags - UMAC scan flags
// @IWL_UMAC_SCAN_FLAG_PREEMPTIVE: scan process triggered by this scan request
// can be preempted by other scan requests with higher priority.
// The low priority scan will be resumed when the higher proirity scan is
// completed.
// @IWL_UMAC_SCAN_FLAG_START_NOTIF: notification will be sent to the driver
// when scan starts.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_umac_scan_flags {
    IWL_UMAC_SCAN_FLAG_PREEMPTIVE		= BIT(0),
    IWL_UMAC_SCAN_FLAG_START_NOTIF		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_umac_scan_uid_offsets {
    IWL_UMAC_SCAN_UID_TYPE_OFFSET		= 0,
    IWL_UMAC_SCAN_UID_SEQ_OFFSET		= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_umac_scan_general_flags {
    IWL_UMAC_SCAN_GEN_FLAGS_PERIODIC		= BIT(0),
    IWL_UMAC_SCAN_GEN_FLAGS_OVER_BT			= BIT(1),
    IWL_UMAC_SCAN_GEN_FLAGS_PASS_ALL		= BIT(2),
    IWL_UMAC_SCAN_GEN_FLAGS_PASSIVE			= BIT(3),
    IWL_UMAC_SCAN_GEN_FLAGS_PRE_CONNECT		= BIT(4),
    IWL_UMAC_SCAN_GEN_FLAGS_ITER_COMPLETE		= BIT(5),
    IWL_UMAC_SCAN_GEN_FLAGS_MULTIPLE_SSID		= BIT(6),
    IWL_UMAC_SCAN_GEN_FLAGS_FRAGMENTED		= BIT(7),
    IWL_UMAC_SCAN_GEN_FLAGS_RRM_ENABLED		= BIT(8),
    IWL_UMAC_SCAN_GEN_FLAGS_MATCH			= BIT(9),
    IWL_UMAC_SCAN_GEN_FLAGS_EXTENDED_DWELL		= BIT(10),
// Extended dwell is obselete when adaptive dwell is used, making this
// bit reusable. Hence, probe request defer is used only when adaptive
// dwell is supported.
    IWL_UMAC_SCAN_GEN_FLAGS_PROB_REQ_DEFER_SUPP	= BIT(10),
    IWL_UMAC_SCAN_GEN_FLAGS_LMAC2_FRAGMENTED	= BIT(11),
    IWL_UMAC_SCAN_GEN_FLAGS_ADAPTIVE_DWELL		= BIT(13),
    IWL_UMAC_SCAN_GEN_FLAGS_MAX_CHNL_TIME		= BIT(14),
    IWL_UMAC_SCAN_GEN_FLAGS_PROB_REQ_HIGH_TX_RATE	= BIT(15),
}

//
// enum iwl_umac_scan_general_flags2 - UMAC scan general flags #2
// @IWL_UMAC_SCAN_GEN_FLAGS2_NOTIF_PER_CHNL: Whether to send a complete
// notification per channel or not.
// @IWL_UMAC_SCAN_GEN_FLAGS2_ALLOW_CHNL_REORDER: Whether to allow channel
// reorder optimization or not.
// @IWL_UMAC_SCAN_GEN_FLAGS2_COLLECT_CHANNEL_STATS: Enable channel statistics
// collection when #IWL_UMAC_SCAN_GEN_FLAGS_V2_FORCE_PASSIVE is set.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_umac_scan_general_flags2 {
    IWL_UMAC_SCAN_GEN_FLAGS2_NOTIF_PER_CHNL		= BIT(0),
    IWL_UMAC_SCAN_GEN_FLAGS2_ALLOW_CHNL_REORDER	= BIT(1),
    IWL_UMAC_SCAN_GEN_FLAGS2_COLLECT_CHANNEL_STATS	= BIT(3),
}

//
// enum iwl_umac_scan_general_flags_v2 - UMAC scan general flags version 2
//
// The FW flags were reordered and hence the driver introduce version 2
//
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_PERIODIC: periodic or scheduled
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_PASS_ALL: pass all probe responses and beacons
// during scan iterations
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_NTFY_ITER_COMPLETE: send complete notification
// on every iteration instead of only once after the last iteration
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_FRAGMENTED_LMAC1: fragmented scan LMAC1
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_FRAGMENTED_LMAC2: fragmented scan LMAC2
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_MATCH: does this scan check for profile matching
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_USE_ALL_RX_CHAINS: use all valid chains for RX
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_ADAPTIVE_DWELL: works with adaptive dwell
// for active channel
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_PREEMPTIVE: can be preempted by other requests
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_NTF_START: send notification of scan start
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_MULTI_SSID: matching on multiple SSIDs
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_FORCE_PASSIVE: all the channels scanned
// as passive
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_TRIGGER_UHB_SCAN: at the end of 2.4GHz and
// 5.2Ghz bands scan, trigger scan on 6GHz band to discover
// the reported collocated APs
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_6GHZ_PASSIVE_SCAN: at the end of 2.4GHz and 5GHz
// bands scan, if not APs were discovered, allow scan to conitnue and scan
// 6GHz PSC channels in order to discover country information.
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_6GHZ_PASSIVE_SCAN_FILTER_IN: in case
// &IWL_UMAC_SCAN_GEN_FLAGS_V2_6GHZ_PASSIVE_SCAN is enabled and scan is
// activated over 6GHz PSC channels, filter in beacons and probe responses.
// @IWL_UMAC_SCAN_GEN_FLAGS_V2_OCE: if set, send probe requests in a minimum
// rate of 5.5Mpbs, filter in broadcast probe responses and set the max
// channel time indication field in the FILS request parameters element
// (if included by the driver in the probe request IEs).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_umac_scan_general_flags_v2 {
    IWL_UMAC_SCAN_GEN_FLAGS_V2_PERIODIC             = BIT(0),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_PASS_ALL             = BIT(1),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_NTFY_ITER_COMPLETE   = BIT(2),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_FRAGMENTED_LMAC1     = BIT(3),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_FRAGMENTED_LMAC2     = BIT(4),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_MATCH                = BIT(5),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_USE_ALL_RX_CHAINS    = BIT(6),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_ADAPTIVE_DWELL       = BIT(7),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_PREEMPTIVE           = BIT(8),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_NTF_START            = BIT(9),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_MULTI_SSID           = BIT(10),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_FORCE_PASSIVE        = BIT(11),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_TRIGGER_UHB_SCAN     = BIT(12),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_6GHZ_PASSIVE_SCAN    = BIT(13),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_6GHZ_PASSIVE_SCAN_FILTER_IN = BIT(14),
    IWL_UMAC_SCAN_GEN_FLAGS_V2_OCE                  = BIT(15),
}

//
// enum iwl_umac_scan_general_params_flags2 - UMAC scan general flags2
//
// @IWL_UMAC_SCAN_GEN_PARAMS_FLAGS2_RESPECT_P2P_GO_LB: scan event scheduling
// should be aware of a P2P GO operation on the 2GHz band.
// @IWL_UMAC_SCAN_GEN_PARAMS_FLAGS2_RESPECT_P2P_GO_HB: scan event scheduling
// should be aware of a P2P GO operation on the 5GHz or 6GHz band.
// @IWL_UMAC_SCAN_GEN_PARAMS_FLAGS2_DONT_TOGGLE_ANT: don't toggle between
// valid antennas, and use the same antenna as in previous scan
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_umac_scan_general_params_flags2 {
    IWL_UMAC_SCAN_GEN_PARAMS_FLAGS2_RESPECT_P2P_GO_LB = BIT(0),
    IWL_UMAC_SCAN_GEN_PARAMS_FLAGS2_RESPECT_P2P_GO_HB = BIT(1),
    IWL_UMAC_SCAN_GEN_PARAMS_FLAGS2_DONT_TOGGLE_ANT   = BIT(2),
}

//
// struct iwl_scan_channel_cfg_umac - scan channel config
// @flags:		bitmap - 0-19:	directed scan to i'th ssid.
// @channel_num:	channel number 1-13 etc.
// @v1:			command version 1
// @v1.iter_count:	repetition count for the channel.
// @v1.iter_interval:	interval between two scan iterations on one channel.
// @v2:			command versions 2-4
// @v2.band:		band of channel: 0 for 2GHz, 1 for 5GHz
// @v2.iter_count:	repetition count for the channel.
// @v2.iter_interval:	interval between two scan iterations on one channel.
// @v5:			command versions 5 and up
// @v5.iter_count:	repetition count for the channel.
// @v5.iter_interval:	interval between two scan iterations on one channel.
// @v5.psd_20:		highest PSD value for all APs known so far
// on this channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_channel_cfg_umac {
pub const IWL_CHAN_CFG_FLAGS_BAND_POS: c_int = 30;
    pub flags: __le32,
    pub channel_num: u8,
// All versions are of the same size, so use a union without adjusting
// the command size later
//
    pub iter_count: u8,
    pub iter_interval: __le16,
    pub /: *mut *mut } __packed v1; / SCAN_CHANNEL_CONFIG_API_S_VER_1,
    pub band: u8,
    pub iter_count: u8,
    pub iter_interval: u8,
    pub SCAN_CHANNEL_CONFIG_API_S_VER_2: *mut *mut } __packed v2; /,
// SCAN_CHANNEL_CONFIG_API_S_VER_3
// SCAN_CHANNEL_CONFIG_API_S_VER_4
//
    pub psd_20: u8,
    pub iter_count: u8,
    pub iter_interval: u8,
    pub /: *mut *mut } __packed v5; / SCAN_CHANNEL_CONFIG_API_S_VER_5,
    pub __packed: },
    pub __packed: },
//
// struct iwl_scan_umac_schedule - scan schedule parameters
// @interval: interval in seconds between scan iterations
// @iter_count: num of scan iterations for schedule plan, 0xff for infinite loop
// @reserved: for alignment and future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_umac_schedule {
    pub interval: __le16,
    pub iter_count: u8,
    pub reserved: u8,
    pub /: *mut *mut } __packed; / SCAN_SCHED_PARAM_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_umac_tail_v1 {
// SCAN_PERIODIC_PARAMS_API_S_VER_1
    pub schedule: [iwl_scan_umac_schedule; IWL_MAX_SCHED_SCAN_PLANS],
    pub delay: __le16,
    pub reserved: __le16,
// SCAN_PROBE_PARAMS_API_S_VER_1
    pub preq: iwl_scan_probe_req_v1,
    pub direct_scan: [iwl_ssid_ie; PROBE_OPTION_MAX],
    pub __packed: },
//
// struct iwl_scan_req_umac_tail_v2 - the rest of the UMAC scan request command
// parameters following channels configuration array.
// @schedule: two scheduling plans.
// @delay: delay in TUs before starting the first scan iteration
// @reserved: for future use and alignment
// @preq: probe request with IEs blocks
// @direct_scan: list of SSIDs for directed active scan
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_umac_tail_v2 {
// SCAN_PERIODIC_PARAMS_API_S_VER_1
    pub schedule: [iwl_scan_umac_schedule; IWL_MAX_SCHED_SCAN_PLANS],
    pub delay: __le16,
    pub reserved: __le16,
// SCAN_PROBE_PARAMS_API_S_VER_2
    pub preq: iwl_scan_probe_req,
    pub direct_scan: [iwl_ssid_ie; PROBE_OPTION_MAX],
    pub __packed: },
//
// struct iwl_scan_umac_chan_param - scan channel parameters
// @flags: channel flags &enum iwl_scan_channel_flags
// @count: num of channels in scan request
// @reserved: for future use and alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_umac_chan_param {
    pub flags: u8,
    pub count: u8,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; /SCAN_CHANNEL_PARAMS_API_S_VER_1,
//
// struct iwl_scan_req_umac - scan request command
// @flags: &enum iwl_umac_scan_flags
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @ooc_priority: out of channel priority - &enum iwl_scan_priority
// @general_flags: &enum iwl_umac_scan_general_flags
// @reserved: reserved
// @scan_start_mac_id: report the scan start TSF time according to this mac TSF
// @v1: version 1 command data
// @v6: version 6 command data
// @v7: version 7 command data
// @v8: version 8 command data
// @v9: version 9 command data
// @v1.extended_dwell: dwell time for channels 1, 6 and 11
// @v1.active_dwell: dwell time for active scan per LMAC
// @v1.passive_dwell: dwell time for passive scan per LMAC
// @v1.fragmented_dwell: dwell time for fragmented passive scan
// @v7.adwell_default_n_aps: for adaptive dwell the default number of APs
// per channel
// @v7.adwell_default_n_aps_social: for adaptive dwell the default
// number of APs per social (1,6,11) channel
// @v8.general_flags2: &enum iwl_umac_scan_general_flags2
// @v7.adwell_max_budget: for adaptive dwell the maximal budget of TU to be
// added to total scan time
// @v1.max_out_time: max out of serving channel time, per LMAC - for CDB
// there are 2 LMACs
// @v1.suspend_time: max suspend time, per LMAC - for CDB there are 2 LMACs
// @v1.scan_priority: scan internal prioritization &enum iwl_scan_priority
// @v8.num_of_fragments: Number of fragments needed for full coverage per band.
// Relevant only for fragmented scan.
// @v1.channel: &struct iwl_scan_umac_chan_param
// @v1.data: &struct iwl_scan_channel_cfg_umac and
// &struct iwl_scan_req_umac_tail
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_umac {
    pub flags: __le32,
    pub uid: __le32,
    pub ooc_priority: __le32,
    pub general_flags: __le16,
    pub reserved: u8,
    pub scan_start_mac_id: u8,
    pub extended_dwell: u8,
    pub active_dwell: u8,
    pub passive_dwell: u8,
    pub fragmented_dwell: u8,
    pub max_out_time: __le32,
    pub suspend_time: __le32,
    pub scan_priority: __le32,
    pub channel: iwl_scan_umac_chan_param,
    pub data: [u8; ],
    pub /: *mut *mut } v1; / SCAN_REQUEST_CMD_UMAC_API_S_VER_1,
    pub extended_dwell: u8,
    pub active_dwell: u8,
    pub passive_dwell: u8,
    pub fragmented_dwell: u8,
    pub max_out_time: [__le32; SCAN_TWO_LMACS],
    pub suspend_time: [__le32; SCAN_TWO_LMACS],
    pub scan_priority: __le32,
    pub channel: iwl_scan_umac_chan_param,
    pub data: [u8; ],
    pub /: *mut *mut } v6; / SCAN_REQUEST_CMD_UMAC_API_S_VER_6,
    pub active_dwell: u8,
    pub passive_dwell: u8,
    pub fragmented_dwell: u8,
    pub adwell_default_n_aps: u8,
    pub adwell_default_n_aps_social: u8,
    pub reserved3: u8,
    pub adwell_max_budget: __le16,
    pub max_out_time: [__le32; SCAN_TWO_LMACS],
    pub suspend_time: [__le32; SCAN_TWO_LMACS],
    pub scan_priority: __le32,
    pub channel: iwl_scan_umac_chan_param,
    pub data: [u8; ],
    pub /: *mut *mut } v7; / SCAN_REQUEST_CMD_UMAC_API_S_VER_7,
    pub active_dwell: [u8; SCAN_TWO_LMACS],
    pub reserved2: u8,
    pub adwell_default_n_aps: u8,
    pub adwell_default_n_aps_social: u8,
    pub general_flags2: u8,
    pub adwell_max_budget: __le16,
    pub max_out_time: [__le32; SCAN_TWO_LMACS],
    pub suspend_time: [__le32; SCAN_TWO_LMACS],
    pub scan_priority: __le32,
    pub passive_dwell: [u8; SCAN_TWO_LMACS],
    pub num_of_fragments: [u8; SCAN_TWO_LMACS],
    pub channel: iwl_scan_umac_chan_param,
    pub data: [u8; ],
    pub /: *mut *mut } v8; / SCAN_REQUEST_CMD_UMAC_API_S_VER_8,
    pub active_dwell: [u8; SCAN_TWO_LMACS],
    pub adwell_default_hb_n_aps: u8,
    pub adwell_default_lb_n_aps: u8,
    pub adwell_default_n_aps_social: u8,
    pub general_flags2: u8,
    pub adwell_max_budget: __le16,
    pub max_out_time: [__le32; SCAN_TWO_LMACS],
    pub suspend_time: [__le32; SCAN_TWO_LMACS],
    pub scan_priority: __le32,
    pub passive_dwell: [u8; SCAN_TWO_LMACS],
    pub num_of_fragments: [u8; SCAN_TWO_LMACS],
    pub channel: iwl_scan_umac_chan_param,
    pub data: [u8; ],
    pub /: *mut *mut } v9; / SCAN_REQUEST_CMD_UMAC_API_S_VER_9,
}

pub const IWL_SCAN_REQ_UMAC_SIZE_V7: c_int = 48;
pub const IWL_SCAN_REQ_UMAC_SIZE_V6: c_int = 44;
pub const IWL_SCAN_REQ_UMAC_SIZE_V1: c_int = 36;
//
// struct iwl_scan_probe_params_v3 - scan probe parameters
// @preq: scan probe request params
// @ssid_num: number of valid SSIDs in direct scan array
// @short_ssid_num: number of valid short SSIDs in short ssid array
// @bssid_num: number of valid bssid in bssids array
// @reserved: reserved
// @direct_scan: list of ssids
// @short_ssid: array of short ssids
// @bssid_array: array of bssids
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_probe_params_v3 {
    pub preq: iwl_scan_probe_req,
    pub ssid_num: u8,
    pub short_ssid_num: u8,
    pub bssid_num: u8,
    pub reserved: u8,
    pub direct_scan: [iwl_ssid_ie; PROBE_OPTION_MAX],
    pub short_ssid: [__le32; SCAN_SHORT_SSID_MAX_SIZE],
    pub bssid_array: [u8; SCAN_BSSID_MAX_SIZE][ETH_ALEN],
    pub /: *mut *mut } __packed; / SCAN_PROBE_PARAMS_API_S_VER_3,
//
// struct iwl_scan_probe_params_v4 - scan probe parameters
// @preq: scan probe request params
// @short_ssid_num: number of valid short SSIDs in short ssid array
// @bssid_num: number of valid bssid in bssids array
// @reserved: reserved
// @direct_scan: list of ssids
// @short_ssid: array of short ssids
// @bssid_array: array of bssids
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_probe_params_v4 {
    pub preq: iwl_scan_probe_req,
    pub short_ssid_num: u8,
    pub bssid_num: u8,
    pub reserved: __le16,
    pub direct_scan: [iwl_ssid_ie; PROBE_OPTION_MAX],
    pub short_ssid: [__le32; SCAN_SHORT_SSID_MAX_SIZE],
    pub bssid_array: [u8; SCAN_BSSID_MAX_SIZE][ETH_ALEN],
    pub /: *mut *mut } __packed; / SCAN_PROBE_PARAMS_API_S_VER_4,
pub const SCAN_MAX_NUM_CHANS_V3: c_int = 67;
pub const SCAN_MAX_NUM_CHANS_V4: c_int = 68;
//
// struct iwl_scan_channel_params_v4 - channel params
// @flags: channel flags &enum iwl_scan_channel_flags
// @count: num of channels in scan request
// @num_of_aps_override: override the number of APs the FW uses to calculate
// dwell time when adaptive dwell is used
// @reserved: for future use and alignment
// @channel_config: array of explicit channel configurations
// for 2.4Ghz and 5.2Ghz bands
// @adwell_ch_override_bitmap: when using adaptive dwell, override the number
// of APs value with &num_of_aps_override for the channel.
// To cast channel to index, use &iwl_mvm_scan_ch_and_band_to_idx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_channel_params_v4 {
    pub flags: u8,
    pub count: u8,
    pub num_of_aps_override: u8,
    pub reserved: u8,
    pub channel_config: [iwl_scan_channel_cfg_umac; SCAN_MAX_NUM_CHANS_V3],
    pub adwell_ch_override_bitmap: [u8; 16],
    pub also: *mut *mut } __packed; / SCAN_CHANNEL_PARAMS_API_S_VER_4,
//
// struct iwl_scan_channel_params_v7 - channel params
// @flags: channel flags &enum iwl_scan_channel_flags
// @count: num of channels in scan request
// @n_aps_override: override the number of APs the FW uses to calculate dwell
// time when adaptive dwell is used.
// Channel k will use n_aps_override[i] when BIT(20 + i) is set in
// channel_config[k].flags
// @channel_config: array of explicit channel configurations
// for 2.4Ghz and 5.2Ghz bands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_channel_params_v7 {
    pub flags: u8,
    pub count: u8,
    pub n_aps_override: [u8; 2],
    pub channel_config: [iwl_scan_channel_cfg_umac; SCAN_MAX_NUM_CHANS_V3],
    pub /: *mut *mut } __packed; / SCAN_CHANNEL_PARAMS_API_S_VER_6,
//
// struct iwl_scan_channel_params_v8 - channel params
// @flags: channel flags &enum iwl_scan_channel_flags
// @count: num of channels in scan request
// @n_aps_override: override the number of APs the FW uses to calculate dwell
// time when adaptive dwell is used.
// Channel k will use n_aps_override[i] when BIT(20 + i) is set in
// channel_config[k].flags
// @channel_config: array of explicit channel configurations
// for 2.4Ghz and 5.2Ghz bands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_channel_params_v8 {
    pub flags: u8,
    pub count: u8,
    pub n_aps_override: [u8; 2],
    pub channel_config: [iwl_scan_channel_cfg_umac; SCAN_MAX_NUM_CHANS_V4],
    pub /: *mut *mut } __packed; / SCAN_CHANNEL_PARAMS_API_S_VER_8,
//
// struct iwl_scan_general_params_v11 - channel params
// @flags: &enum iwl_umac_scan_general_flags_v2
// @reserved: reserved for future
// @scan_start_mac_or_link_id: report the scan start TSF time according to this
// mac (up to verion 11) or link (starting with version 12) TSF
// @active_dwell: dwell time for active scan per LMAC
// @adwell_default_2g: adaptive dwell default number of APs
// for 2.4GHz channel
// @adwell_default_5g: adaptive dwell default number of APs
// for 5GHz channels
// @adwell_default_social_chn: adaptive dwell default number of
// APs per social channel
// @flags2: for version 11 see &enum iwl_umac_scan_general_params_flags2.
// Otherwise reserved.
// @adwell_max_budget: the maximal number of TUs that adaptive dwell
// can add to the total scan time
// @max_out_of_time: max out of serving channel time, per LMAC
// @suspend_time: max suspend time, per LMAC
// @scan_priority: priority of the request
// @passive_dwell: continues dwell time for passive channel
// (without adaptive dwell)
// @num_of_fragments: number of fragments needed for full fragmented
// scan coverage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_general_params_v11 {
    pub flags: __le16,
    pub reserved: u8,
    pub scan_start_mac_or_link_id: u8,
    pub active_dwell: [u8; SCAN_TWO_LMACS],
    pub adwell_default_2g: u8,
    pub adwell_default_5g: u8,
    pub adwell_default_social_chn: u8,
    pub flags2: u8,
    pub adwell_max_budget: __le16,
    pub max_out_of_time: [__le32; SCAN_TWO_LMACS],
    pub suspend_time: [__le32; SCAN_TWO_LMACS],
    pub scan_priority: __le32,
    pub passive_dwell: [u8; SCAN_TWO_LMACS],
    pub num_of_fragments: [u8; SCAN_TWO_LMACS],
    pub /: *mut *mut *mut *mut } __packed; / SCAN_GENERAL_PARAMS_API_S_VER_12, _VER_11 and _VER_10,
//
// struct iwl_scan_periodic_parms_v1 - periodicity parameters
// @schedule: can scheduling parameter
// @delay: initial delay of the periodic scan in seconds
// @reserved: reserved for future
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_periodic_parms_v1 {
    pub schedule: [iwl_scan_umac_schedule; IWL_MAX_SCHED_SCAN_PLANS],
    pub delay: __le16,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / SCAN_PERIODIC_PARAMS_API_S_VER_1,
//
// struct iwl_scan_req_params_v12 - scan request parameters (v12)
// @general_params: &struct iwl_scan_general_params_v11
// @channel_params: &struct iwl_scan_channel_params_v4
// @periodic_params: &struct iwl_scan_periodic_parms_v1
// @probe_params: &struct iwl_scan_probe_params_v3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_params_v12 {
    pub general_params: iwl_scan_general_params_v11,
    pub channel_params: iwl_scan_channel_params_v4,
    pub periodic_params: iwl_scan_periodic_parms_v1,
    pub probe_params: iwl_scan_probe_params_v3,
    pub /: *mut *mut } __packed; / SCAN_REQUEST_PARAMS_API_S_VER_12,
//
// struct iwl_scan_req_params_v17 - scan request parameters (v17)
// @general_params: &struct iwl_scan_general_params_v11
// @channel_params: &struct iwl_scan_channel_params_v7
// @periodic_params: &struct iwl_scan_periodic_parms_v1
// @probe_params: &struct iwl_scan_probe_params_v4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_params_v17 {
    pub general_params: iwl_scan_general_params_v11,
    pub channel_params: iwl_scan_channel_params_v7,
    pub periodic_params: iwl_scan_periodic_parms_v1,
    pub probe_params: iwl_scan_probe_params_v4,
    pub /: *mut *mut } __packed; / SCAN_REQUEST_PARAMS_API_S_VER_17 - 14,
//
// struct iwl_scan_req_params_v18 - scan request parameters (v18)
// @general_params: &struct iwl_scan_general_params_v11
// @channel_params: &struct iwl_scan_channel_params_v8
// @periodic_params: &struct iwl_scan_periodic_parms_v1
// @probe_params: &struct iwl_scan_probe_params_v4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_params_v18 {
    pub general_params: iwl_scan_general_params_v11,
    pub channel_params: iwl_scan_channel_params_v8,
    pub periodic_params: iwl_scan_periodic_parms_v1,
    pub probe_params: iwl_scan_probe_params_v4,
    pub /: *mut *mut } __packed; / SCAN_REQUEST_PARAMS_API_S_VER_18,
//
// struct iwl_scan_req_umac_v12 - scan request command (v12)
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @ooc_priority: out of channel priority - &enum iwl_scan_priority
// @scan_params: scan parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_umac_v12 {
    pub uid: __le32,
    pub ooc_priority: __le32,
    pub scan_params: iwl_scan_req_params_v12,
    pub /: *mut *mut } __packed; / SCAN_REQUEST_CMD_UMAC_API_S_VER_12,
//
// struct iwl_scan_req_umac_v17 - scan request command (v17)
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @ooc_priority: out of channel priority - &enum iwl_scan_priority
// @scan_params: scan parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_umac_v17 {
    pub uid: __le32,
    pub ooc_priority: __le32,
    pub scan_params: iwl_scan_req_params_v17,
    pub /: *mut *mut } __packed; / SCAN_REQUEST_CMD_UMAC_API_S_VER_17 - 14,
//
// struct iwl_scan_req_umac_v18 - scan request command (v18)
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @ooc_priority: out of channel priority - &enum iwl_scan_priority
// @scan_params: scan parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_req_umac_v18 {
    pub uid: __le32,
    pub ooc_priority: __le32,
    pub scan_params: iwl_scan_req_params_v18,
    pub /: *mut *mut } __packed; / SCAN_REQUEST_CMD_UMAC_API_S_VER_18,
//
// struct iwl_umac_scan_abort - scan abort command
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @flags: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_scan_abort {
    pub uid: __le32,
    pub flags: __le32,
    pub /: *mut *mut } __packed; / SCAN_ABORT_CMD_UMAC_API_S_VER_1,
//
// enum iwl_umac_scan_abort_status - scan abort status
//
// @IWL_UMAC_SCAN_ABORT_STATUS_SUCCESS: scan was successfully aborted
// @IWL_UMAC_SCAN_ABORT_STATUS_IN_PROGRESS: scan abort is in progress
// @IWL_UMAC_SCAN_ABORT_STATUS_NOT_FOUND: nothing to abort
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_umac_scan_abort_status {
    IWL_UMAC_SCAN_ABORT_STATUS_SUCCESS = 0,
    IWL_UMAC_SCAN_ABORT_STATUS_IN_PROGRESS,
    IWL_UMAC_SCAN_ABORT_STATUS_NOT_FOUND,
}

//
// struct iwl_umac_scan_start - scan start notification
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @reserved: for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_scan_start {
    pub uid: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / SCAN_START_UMAC_API_S_VER_1,
//
// struct iwl_umac_scan_complete - scan complete notification
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @last_schedule: last scheduling line
// @last_iter: last scan iteration number
// @status: &enum iwl_scan_offload_complete_status
// @ebs_status: &enum iwl_scan_ebs_status
// @time_from_last_iter: time elapsed from last iteration
// @reserved: for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_scan_complete {
    pub uid: __le32,
    pub last_schedule: u8,
    pub last_iter: u8,
    pub status: u8,
    pub ebs_status: u8,
    pub time_from_last_iter: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / SCAN_COMPLETE_NTF_UMAC_API_S_VER_1,
pub const SCAN_OFFLOAD_MATCHING_CHANNELS_LEN_V1: c_int = 5;
pub const SCAN_OFFLOAD_MATCHING_CHANNELS_LEN: c_int = 7;
//
// struct iwl_scan_offload_profile_match_v1 - match information
// @bssid: matched bssid
// @reserved: reserved
// @channel: channel where the match occurred
// @energy: energy
// @matching_feature: feature matches
// @matching_channels: bitmap of channels that matched, referencing
// the channels passed in the scan offload request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_profile_match_v1 {
    pub bssid: [u8; ETH_ALEN],
    pub reserved: __le16,
    pub channel: u8,
    pub energy: u8,
    pub matching_feature: u8,
    pub matching_channels: [u8; SCAN_OFFLOAD_MATCHING_CHANNELS_LEN_V1],
    pub /: *mut *mut } __packed; / SCAN_OFFLOAD_PROFILE_MATCH_RESULTS_S_VER_1,
//
// struct iwl_scan_offload_profiles_query_v1 - match results query response
// @matched_profiles: bitmap of matched profiles, referencing the
// matches passed in the scan offload request
// @last_scan_age: age of the last offloaded scan
// @n_scans_done: number of offloaded scans done
// @gp2_d0u: GP2 when D0U occurred
// @gp2_invoked: GP2 when scan offload was invoked
// @resume_while_scanning: not used
// @self_recovery: obsolete
// @reserved: reserved
// @matches: array of match information, one for each match
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_profiles_query_v1 {
    pub matched_profiles: __le32,
    pub last_scan_age: __le32,
    pub n_scans_done: __le32,
    pub gp2_d0u: __le32,
    pub gp2_invoked: __le32,
    pub resume_while_scanning: u8,
    pub self_recovery: u8,
    pub reserved: __le16,
    pub matches: [iwl_scan_offload_profile_match_v1; ],
    pub /: *mut *mut } __packed; / SCAN_OFFLOAD_PROFILES_QUERY_RSP_S_VER_2,
//
// struct iwl_scan_offload_profile_match - match information
// @bssid: matched bssid
// @reserved: reserved
// @channel: channel where the match occurred
// @energy: energy
// @matching_feature: feature matches
// @matching_channels: bitmap of channels that matched, referencing
// the channels passed in the scan offload request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_profile_match {
    pub bssid: [u8; ETH_ALEN],
    pub reserved: __le16,
    pub channel: u8,
    pub energy: u8,
    pub matching_feature: u8,
    pub matching_channels: [u8; SCAN_OFFLOAD_MATCHING_CHANNELS_LEN],
    pub /: *mut *mut } __packed; / SCAN_OFFLOAD_PROFILE_MATCH_RESULTS_S_VER_2,
//
// struct iwl_scan_offload_match_info - match results information
// @matched_profiles: bitmap of matched profiles, referencing the
// matches passed in the scan offload request
// @last_scan_age: age of the last offloaded scan
// @n_scans_done: number of offloaded scans done
// @gp2_d0u: GP2 when D0U occurred
// @gp2_invoked: GP2 when scan offload was invoked
// @resume_while_scanning: not used
// @self_recovery: obsolete
// @reserved: reserved
// @matches: array of match information, one for each match
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_offload_match_info {
    pub matched_profiles: __le32,
    pub last_scan_age: __le32,
    pub n_scans_done: __le32,
    pub gp2_d0u: __le32,
    pub gp2_invoked: __le32,
    pub resume_while_scanning: u8,
    pub self_recovery: u8,
    pub reserved: __le16,
    pub matches: [iwl_scan_offload_profile_match; IWL_SCAN_MAX_PROFILES_V2],
    pub and: *mut *mut } __packed; / SCAN_OFFLOAD_PROFILES_QUERY_RSP_S_VER_3,
// SCAN_OFFLOAD_MATCH_INFO_NOTIFICATION_S_VER_1
//
// struct iwl_umac_scan_iter_complete_notif - notifies end of scanning iteration
// @uid: scan id, &enum iwl_umac_scan_uid_offsets
// @scanned_channels: number of channels scanned and number of valid elements in
// results array
// @status: one of SCAN_COMP_STATUS_
// @bt_status: BT on/off status
// @last_channel: last channel that was scanned
// @start_tsf: TSF timer in usecs of the scan start time for the mac specified
// in &struct iwl_scan_req_umac.
// @results: array of scan results, length in @scanned_channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_scan_iter_complete_notif {
    pub uid: __le32,
    pub scanned_channels: u8,
    pub status: u8,
    pub bt_status: u8,
    pub last_channel: u8,
    pub start_tsf: __le64,
    pub results: [iwl_scan_results_notif; ],
    pub /: *mut *mut } __packed; / SCAN_ITER_COMPLETE_NTF_UMAC_API_S_VER_2,
//
// struct iwl_umac_scan_channel_survey_notif - data for survey
// @channel: the channel scanned
// @band: band of channel
// @noise: noise floor measurements in negative dBm, invalid 0xff
// @reserved: for future use and alignment
// @active_time: time in ms the radio was turned on (on the channel)
// @busy_time: time in ms the channel was sensed busy, 0 for a clean channel
// @tx_time: time the radio spent transmitting data
// @rx_time: time the radio spent receiving data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_scan_channel_survey_notif {
    pub channel: __le32,
    pub band: __le32,
    pub noise: [u8; IWL_MAX_NUM_NOISE_RESULTS],
    pub reserved: [u8; 2],
    pub active_time: __le32,
    pub busy_time: __le32,
    pub tx_time: __le32,
    pub rx_time: __le32,
    pub /: *mut *mut } __packed; / SCAN_CHANNEL_SURVEY_NTF_API_S_VER_1,
