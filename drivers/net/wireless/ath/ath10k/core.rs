//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/core.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2017 Qualcomm Atheros, Inc.
// Copyright (c) 2018-2019, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH10K_SCAN_ID: c_int = 0;

pub const ATH10K_NUM_CHANS: c_int = 41;
pub const ATH10K_MAX_5G_CHAN: c_int = 173;
// Antenna noise floor

pub const ATH10K_INVALID_RSSI: c_int = 128;
pub const ATH10K_MAX_NUM_MGMT_PENDING: c_int = 128;
// number of failed packets (20 packets with 16 sw reties each)

//
// Use insanely high numbers to make sure that the firmware implementation
// won't start, we have the same functionality already in hostapd. Unit
// is seconds.
//
pub const ATH10K_KEEPALIVE_MIN_IDLE: c_int = 3747;
pub const ATH10K_KEEPALIVE_MAX_IDLE: c_int = 3895;
pub const ATH10K_KEEPALIVE_MAX_UNRESPONSIVE: c_int = 3900;
// SMBIOS type containing Board Data File Name Extension
pub const ATH10K_SMBIOS_BDF_EXT_TYPE: c_uint = 0xF8;
// SMBIOS type structure length (excluding strings-set)
pub const ATH10K_SMBIOS_BDF_EXT_LENGTH: c_uint = 0x9;
// Offset pointing to Board Data File Name Extension
pub const ATH10K_SMBIOS_BDF_EXT_OFFSET: c_uint = 0x8;
// Board Data File Name Extension string length.
// String format: BDF_<Customer ID>_<Extension>\0
//
pub const ATH10K_SMBIOS_BDF_EXT_STR_LENGTH: c_uint = 0x20;
// The magic used by QCA spec

// Default Airtime weight multiplier (Tuned for multiclient performance)
pub const ATH10K_AIRTIME_WEIGHT_MULTIPLIER: c_int = 4;
pub const ATH10K_MAX_RETRY_COUNT: c_int = 30;

pub const ATH10K_RECOVERY_MAX_FAIL_COUNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_skb_flags {
    ATH10K_SKB_F_NO_HWCRYPT = BIT(0),
    ATH10K_SKB_F_DTIM_ZERO = BIT(1),
    ATH10K_SKB_F_DELIVER_CAB = BIT(2),
    ATH10K_SKB_F_MGMT = BIT(3),
    ATH10K_SKB_F_QOS = BIT(4),
    ATH10K_SKB_F_RAW_TX = BIT(5),
    ATH10K_SKB_F_NOACK_TID = BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_skb_cb {
    pub paddr: dma_addr_t,
    pub flags: u8,
    pub eid: u8,
    pub msdu_id: u16,
    pub airtime_est: u16,
    pub vif: *mut ieee80211_vif,
    pub txq: *mut ieee80211_txq,
    pub ucast_cipher: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_skb_rxcb {
    pub paddr: dma_addr_t,
    pub hlist: hlist_node,
    pub eid: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_phy_mode {
    ATH10K_PHY_MODE_LEGACY = 0,
    ATH10K_PHY_MODE_HT = 1,
    ATH10K_PHY_MODE_VHT = 2,
}

// Data rate 100KBPS based on IE Index
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_index_ht_data_rate_type {
    pub beacon_rate_index: u8,
    pub supported_rate: [u16; 4],
}

// Data rate 100KBPS based on IE Index
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_index_vht_data_rate_type {
    pub beacon_rate_index: u8,
    pub supported_VHT80_rate: [u16; 2],
    pub supported_VHT40_rate: [u16; 2],
    pub supported_VHT20_rate: [u16; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_bmi {
    pub done_sent: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_mem_chunk {
    pub vaddr: *mut c_void,
    pub paddr: dma_addr_t,
    pub len: u32,
    pub req_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_wmi {
    pub eid: ath10k_htc_ep_id,
    pub service_ready: completion,
    pub unified_ready: completion,
    pub barrier: completion,
    pub radar_confirm: completion,
    pub tx_credits_wq: wait_queue_head_t,
    pub WMI_SERVICE_MAX): DECLARE_BITMAP(svc_map,,
    pub cmd: *mut wmi_cmd_map,
    pub vdev_param: *mut wmi_vdev_param_map,
    pub pdev_param: *mut wmi_pdev_param_map,
    pub peer_param: *mut wmi_peer_param_map,
    pub ops: *const wmi_ops,
    pub peer_flags: *const wmi_peer_flags_map,
    pub mgmt_max_num_pending_tx: u32,
// Protected by data_lock
    pub mgmt_pending_tx: idr,
    pub num_mem_chunks: u32,
    pub rx_decap_mode: u32,
    pub mem_chunks: [ath10k_mem_chunk; WMI_MAX_MEM_REQS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_stats_peer {
    pub list: list_head,
    pub peer_macaddr: [u8; ETH_ALEN],
    pub peer_rssi: u32,
    pub peer_tx_rate: u32,
    pub /: *mut *mut u32 peer_rx_rate; / 10x only,
    pub rx_duration: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_extd_stats_peer {
    pub list: list_head,
    pub peer_macaddr: [u8; ETH_ALEN],
    pub rx_duration: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_stats_vdev {
    pub list: list_head,
    pub vdev_id: u32,
    pub beacon_snr: u32,
    pub data_snr: u32,
    pub num_tx_frames: [u32; 4],
    pub num_rx_frames: u32,
    pub num_tx_frames_retries: [u32; 4],
    pub num_tx_frames_failures: [u32; 4],
    pub num_rts_fail: u32,
    pub num_rts_success: u32,
    pub num_rx_err: u32,
    pub num_rx_discard: u32,
    pub num_tx_not_acked: u32,
    pub tx_rate_history: [u32; 10],
    pub beacon_rssi_history: [u32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_stats_vdev_extd {
    pub list: list_head,
    pub vdev_id: u32,
    pub ppdu_aggr_cnt: u32,
    pub ppdu_noack: u32,
    pub mpdu_queued: u32,
    pub ppdu_nonaggr_cnt: u32,
    pub mpdu_sw_requeued: u32,
    pub mpdu_suc_retry: u32,
    pub mpdu_suc_multitry: u32,
    pub mpdu_fail_retry: u32,
    pub tx_ftm_suc: u32,
    pub tx_ftm_suc_retry: u32,
    pub tx_ftm_fail: u32,
    pub rx_ftmr_cnt: u32,
    pub rx_ftmr_dup_cnt: u32,
    pub rx_iftmr_cnt: u32,
    pub rx_iftmr_dup_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_stats_pdev {
    pub list: list_head,
// PDEV stats
    pub ch_noise_floor: i32,
    pub /: *mut *mut u32 tx_frame_count; / Cycles spent transmitting frames,
    pub /: *mut *mut u32 rx_frame_count; / Cycles spent receiving frames,
    pub /: *mut *mut u32 rx_clear_count; / Total channel busy time, evidently,
    pub /: *mut *mut u32 cycle_count; / Total on-channel time,
    pub phy_err_count: u32,
    pub chan_tx_power: u32,
    pub ack_rx_bad: u32,
    pub rts_bad: u32,
    pub rts_good: u32,
    pub fcs_bad: u32,
    pub no_beacons: u32,
    pub mib_int_count: u32,
// PDEV TX stats
    pub comp_queued: i32,
    pub comp_delivered: i32,
    pub msdu_enqued: i32,
    pub mpdu_enqued: i32,
    pub wmm_drop: i32,
    pub local_enqued: i32,
    pub local_freed: i32,
    pub hw_queued: i32,
    pub hw_reaped: i32,
    pub underrun: i32,
    pub hw_paused: u32,
    pub tx_abort: i32,
    pub mpdus_requeued: i32,
    pub tx_ko: u32,
    pub data_rc: u32,
    pub self_triggers: u32,
    pub sw_retry_failure: u32,
    pub illgl_rate_phy_err: u32,
    pub pdev_cont_xretry: u32,
    pub pdev_tx_timeout: u32,
    pub pdev_resets: u32,
    pub phy_underrun: u32,
    pub txop_ovf: u32,
    pub seq_posted: u32,
    pub seq_failed_queueing: u32,
    pub seq_completed: u32,
    pub seq_restarted: u32,
    pub mu_seq_posted: u32,
    pub mpdus_sw_flush: u32,
    pub mpdus_hw_filter: u32,
    pub mpdus_truncated: u32,
    pub mpdus_ack_failed: u32,
    pub mpdus_expired: u32,
// PDEV RX stats
    pub mid_ppdu_route_change: i32,
    pub status_rcvd: i32,
    pub r0_frags: i32,
    pub r1_frags: i32,
    pub r2_frags: i32,
    pub r3_frags: i32,
    pub htt_msdus: i32,
    pub htt_mpdus: i32,
    pub loc_msdus: i32,
    pub loc_mpdus: i32,
    pub oversize_amsdu: i32,
    pub phy_errs: i32,
    pub phy_err_drop: i32,
    pub mpdu_errs: i32,
    pub rx_ovfl_errs: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_stats {
    pub extended: bool,
    pub pdevs: list_head,
    pub vdevs: list_head,
    pub peers: list_head,
    pub peers_extd: list_head,
}

pub const ATH10K_TPC_TABLE_TYPE_FLAG: c_int = 1;
pub const ATH10K_TPC_PREAM_TABLE_END: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_tpc_table {
    pub pream_idx: [u32; WMI_TPC_RATE_MAX],
    pub rate_code: [u8; WMI_TPC_RATE_MAX],
    pub WMI_TPC_BUF_SIZE]: *mut *mut char tpc_value[WMI_TPC_RATE_MAX][WMI_TPC_TX_N_CHAIN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_tpc_stats {
    pub reg_domain: u32,
    pub chan_freq: u32,
    pub phy_mode: u32,
    pub twice_antenna_reduction: u32,
    pub twice_max_rd_power: u32,
    pub twice_antenna_gain: i32,
    pub power_limit: u32,
    pub num_tx_chain: u32,
    pub ctl: u32,
    pub rate_max: u32,
    pub flag: [u8; WMI_TPC_FLAG],
    pub tpc_table: [ath10k_tpc_table; WMI_TPC_FLAG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_tpc_table_final {
    pub pream_idx: [u32; WMI_TPC_FINAL_RATE_MAX],
    pub rate_code: [u8; WMI_TPC_FINAL_RATE_MAX],
    pub WMI_TPC_BUF_SIZE]: *mut *mut char tpc_value[WMI_TPC_FINAL_RATE_MAX][WMI_TPC_TX_N_CHAIN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_tpc_stats_final {
    pub reg_domain: u32,
    pub chan_freq: u32,
    pub phy_mode: u32,
    pub twice_antenna_reduction: u32,
    pub twice_max_rd_power: u32,
    pub twice_antenna_gain: i32,
    pub power_limit: u32,
    pub num_tx_chain: u32,
    pub ctl: u32,
    pub rate_max: u32,
    pub flag: [u8; WMI_TPC_FLAG],
    pub tpc_table_final: [ath10k_tpc_table_final; WMI_TPC_FLAG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_dfs_stats {
    pub phy_errors: u32,
    pub pulses_total: u32,
    pub pulses_detected: u32,
    pub pulses_discarded: u32,
    pub radar_detected: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_radar_confirmation_state {
    ATH10K_RADAR_CONFIRMATION_IDLE = 0,
    ATH10K_RADAR_CONFIRMATION_INPROGRESS,
    ATH10K_RADAR_CONFIRMATION_STOPPED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_radar_found_info {
    pub pri_min: u32,
    pub pri_max: u32,
    pub width_min: u32,
    pub width_max: u32,
    pub sidx_min: u32,
    pub sidx_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_peer {
    pub list: list_head,
    pub vif: *mut ieee80211_vif,
    pub sta: *mut ieee80211_sta,
    pub removed: bool,
    pub vdev_id: c_int,
    pub addr: [u8; ETH_ALEN],
    pub ATH10K_MAX_NUM_PEER_IDS): DECLARE_BITMAP(peer_ids,,
// protected by ar->data_lock
    pub 1]: *mut *mut ieee80211_key_conf keys[WMI_MAX_KEY_INDEX +,
    pub tids_last_pn: [htt_rx_pn_t; ATH10K_TXRX_NUM_EXT_TIDS],
    pub tids_last_pn_valid: [bool; ATH10K_TXRX_NUM_EXT_TIDS],
    pub frag_tids_last_pn: [htt_rx_pn_t; ATH10K_TXRX_NUM_EXT_TIDS],
    pub frag_tids_seq: [u32; ATH10K_TXRX_NUM_EXT_TIDS],
    pub sec_type: htt_security_types,
    pub pn_len: c_int,
    pub rx_pn: [}; ATH10K_HTT_TXRX_PEER_SECURITY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_txq {
    pub list: list_head,
    pub num_fw_queued: c_ulong,
    pub num_push_allowed: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_pkt_rx_err {
    ATH10K_PKT_RX_ERR_FCS,
    ATH10K_PKT_RX_ERR_TKIP,
    ATH10K_PKT_RX_ERR_CRYPT,
    ATH10K_PKT_RX_ERR_PEER_IDX_INVAL,
    ATH10K_PKT_RX_ERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_ampdu_subfrm_num {
    ATH10K_AMPDU_SUBFRM_NUM_10,
    ATH10K_AMPDU_SUBFRM_NUM_20,
    ATH10K_AMPDU_SUBFRM_NUM_30,
    ATH10K_AMPDU_SUBFRM_NUM_40,
    ATH10K_AMPDU_SUBFRM_NUM_50,
    ATH10K_AMPDU_SUBFRM_NUM_60,
    ATH10K_AMPDU_SUBFRM_NUM_MORE,
    ATH10K_AMPDU_SUBFRM_NUM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_amsdu_subfrm_num {
    ATH10K_AMSDU_SUBFRM_NUM_1,
    ATH10K_AMSDU_SUBFRM_NUM_2,
    ATH10K_AMSDU_SUBFRM_NUM_3,
    ATH10K_AMSDU_SUBFRM_NUM_4,
    ATH10K_AMSDU_SUBFRM_NUM_MORE,
    ATH10K_AMSDU_SUBFRM_NUM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sta_tid_stats {
    pub rx_pkt_from_fw: c_ulong,
    pub rx_pkt_unchained: c_ulong,
    pub rx_pkt_drop_chained: c_ulong,
    pub rx_pkt_drop_filter: c_ulong,
    pub rx_pkt_err: [c_ulong; ATH10K_PKT_RX_ERR_MAX],
    pub rx_pkt_queued_for_mac: c_ulong,
    pub rx_pkt_ampdu: [c_ulong; ATH10K_AMPDU_SUBFRM_NUM_MAX],
    pub rx_pkt_amsdu: [c_ulong; ATH10K_AMSDU_SUBFRM_NUM_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_counter_type {
    ATH10K_COUNTER_TYPE_BYTES,
    ATH10K_COUNTER_TYPE_PKTS,
    ATH10K_COUNTER_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_stats_type {
    ATH10K_STATS_TYPE_SUCC,
    ATH10K_STATS_TYPE_FAIL,
    ATH10K_STATS_TYPE_RETRY,
    ATH10K_STATS_TYPE_AMPDU,
    ATH10K_STATS_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt_data_stats {
    pub legacy: [u64; ATH10K_COUNTER_TYPE_MAX][ATH10K_LEGACY_NUM],
    pub ht: [u64; ATH10K_COUNTER_TYPE_MAX][ATH10K_HT_MCS_NUM],
    pub vht: [u64; ATH10K_COUNTER_TYPE_MAX][ATH10K_VHT_MCS_NUM],
    pub bw: [u64; ATH10K_COUNTER_TYPE_MAX][ATH10K_BW_NUM],
    pub nss: [u64; ATH10K_COUNTER_TYPE_MAX][ATH10K_NSS_NUM],
    pub gi: [u64; ATH10K_COUNTER_TYPE_MAX][ATH10K_GI_NUM],
    pub rate_table: [u64; ATH10K_COUNTER_TYPE_MAX][ATH10K_RATE_TABLE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt_tx_stats {
    pub stats: [ath10k_htt_data_stats; ATH10K_STATS_TYPE_MAX],
    pub tx_duration: u64,
    pub ba_fails: u64,
    pub ack_fails: u64,
}

pub const ATH10K_TID_MAX: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_sta {
    pub arvif: *mut ath10k_vif,
// the following are protected by ar->data_lock
    pub /: *mut *mut *mut u32 changed; / IEEE80211_RC_,
    pub bw: u32,
    pub nss: u32,
    pub smps: u32,
    pub peer_id: u16,
    pub txrate: rate_info,
    pub tx_info: ieee80211_tx_info,
    pub tx_retries: u32,
    pub tx_failed: u32,
    pub last_tx_bitrate: u32,
    pub rx_rate_code: u32,
    pub rx_bitrate_kbps: u32,
    pub tx_rate_code: u32,
    pub tx_bitrate_kbps: u32,
    pub update_wk: work_struct,
    pub rx_duration: u64,
    pub tx_stats: *mut ath10k_htt_tx_stats,
    pub ucast_cipher: u32,

// protected by conf_mutex
    pub aggr_mode: bool,
// Protected with ar->data_lock
    pub 1]: ath10k_sta_tid_stats tid_stats[IEEE80211_NUM_TIDS +,

// Protected with ar->data_lock
    pub peer_ps_state: u32,
    pub tid_config_wk: work_struct,
    pub noack: [c_int; ATH10K_TID_MAX],
    pub retry_long: [c_int; ATH10K_TID_MAX],
    pub ampdu: [c_int; ATH10K_TID_MAX],
    pub rate_ctrl: [u8; ATH10K_TID_MAX],
    pub rate_code: [u32; ATH10K_TID_MAX],
    pub rtscts: [c_int; ATH10K_TID_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_beacon_state {
    ATH10K_BEACON_SCHEDULED = 0,
    ATH10K_BEACON_SENDING,
    ATH10K_BEACON_SENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_vif {
    pub list: list_head,
    pub vdev_id: u32,
    pub peer_id: u16,
    pub vdev_type: wmi_vdev_type,
    pub vdev_subtype: wmi_vdev_subtype,
    pub beacon_interval: u32,
    pub dtim_period: u32,
    pub beacon: *mut sk_buff,
// protected by data_lock
    pub beacon_state: ath10k_beacon_state,
    pub beacon_buf: *mut c_void,
    pub beacon_paddr: dma_addr_t,
    pub /: *mut *mut unsigned long tx_paused; / arbitrary values defined by target,
    pub ar: *mut ath10k,
    pub vif: *mut ieee80211_vif,
    pub is_started: bool,
    pub is_up: bool,
    pub spectral_enabled: bool,
    pub ps: bool,
    pub aid: u32,
    pub bssid: [u8; ETH_ALEN],
    pub 1]: *mut *mut ieee80211_key_conf wep_keys[WMI_MAX_KEY_INDEX +,
    pub def_wep_key_idx: i8,
    pub tx_seq_no: u16,
    pub uapsd: u32,
    pub sta: },
// 512 stations
    pub tim_bitmap: [u8; 64],
    pub tim_len: u8,
    pub ssid_len: u32,
    pub __nonstring: u8 ssid[IEEE80211_MAX_SSID_LEN],
    pub hidden_ssid: bool,
// P2P_IE with NoA attribute for P2P_GO case
    pub noa_len: u32,
    pub noa_data: *mut u8,
    pub ap: },
    pub u: },
    pub use_cts_prot: bool,
    pub nohwcrypt: bool,
    pub num_legacy_stations: c_int,
    pub txpower: c_int,
    pub ftm_responder: bool,
    pub wmm_params: wmi_wmm_params_all_arg,
    pub ap_csa_work: work_struct,
    pub connection_loss_work: delayed_work,
    pub bitrate_mask: cfg80211_bitrate_mask,
// For setting VHT peer fixed rate, protected by conf_mutex
    pub vht_num_rates: c_int,
    pub vht_pfr: u8,
    pub tid_conf_changed: [u32; ATH10K_TID_MAX],
    pub noack: [c_int; ATH10K_TID_MAX],
    pub retry_long: [c_int; ATH10K_TID_MAX],
    pub ampdu: [c_int; ATH10K_TID_MAX],
    pub rate_ctrl: [u8; ATH10K_TID_MAX],
    pub rate_code: [u32; ATH10K_TID_MAX],
    pub rtscts: [c_int; ATH10K_TID_MAX],
    pub tids_rst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_vif_iter {
    pub vdev_id: u32,
    pub arvif: *mut ath10k_vif,
}

// Copy Engine register dump, protected by ce-lock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ce_crash_data {
    pub base_addr: __le32,
    pub src_wr_idx: __le32,
    pub src_r_idx: __le32,
    pub dst_wr_idx: __le32,
    pub dst_r_idx: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ce_crash_hdr {
    pub ce_count: __le32,
    pub /: *mut *mut __le32 reserved[3]; / for future use,
    pub entries: [ath10k_ce_crash_data; ],
}

pub const MAX_MEM_DUMP_TYPE: c_int = 5;
// used for crash-dump storage, protected by data-lock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_crash_data {
    pub guid: guid_t,
    pub timestamp: timespec64,
    pub registers: [__le32; REG_DUMP_COUNT_QCA988X],
    pub ce_crash_data: [ath10k_ce_crash_data; CE_COUNT_MAX],
    pub ramdump_buf: *mut u8,
    pub ramdump_buf_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_debug {
    pub debugfs_phy: *mut dentry,
    pub fw_stats: ath10k_fw_stats,
    pub fw_stats_complete: completion,
    pub fw_stats_done: bool,
    pub htt_stats_mask: c_ulong,
    pub reset_htt_stats: c_ulong,
    pub htt_stats_dwork: delayed_work,
    pub dfs_stats: ath10k_dfs_stats,
    pub dfs_pool_stats: ath_dfs_pool_stats,
// used for tpc-dump storage, protected by data-lock
    pub tpc_stats: *mut ath10k_tpc_stats,
    pub tpc_stats_final: *mut ath10k_tpc_stats_final,
    pub tpc_complete: completion,
// protected by conf_mutex
    pub fw_dbglog_mask: u64,
    pub fw_dbglog_level: u32,
    pub reg_addr: u32,
    pub nf_cal_period: u32,
    pub cal_data: *mut c_void,
    pub enable_extd_tx_stats: u32,
    pub fw_dbglog_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_state {
    ATH10K_STATE_OFF = 0,
    ATH10K_STATE_ON,

// When doing firmware recovery the device is first powered down.
// mac80211 is supposed to call in to start() hook later on. It is
// however possible that driver unloading and firmware crash overlap.
// mac80211 can wait on conf_mutex in stop() while the device is
// stopped in ath10k_core_restart() work holding conf_mutex. The state
// RESTARTED means that the device is up and mac80211 has started hw
// reconfiguration. Once mac80211 is done with the reconfiguration we
// set the state to STATE_ON in reconfig_complete().
//
    ATH10K_STATE_RESTARTING,
    ATH10K_STATE_RESTARTED,

// The device has crashed while restarting hw. This state is like ON
// but commands are blocked in HTC and -ECOMM response is given. This
// prevents completion timeouts and makes the driver more responsive to
// userspace commands. This is also prevents recursive recovery.
//
    ATH10K_STATE_WEDGED,

// factory tests
    ATH10K_STATE_UTF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_firmware_mode {
// the default mode, standard 802.11 functionality
    ATH10K_FIRMWARE_MODE_NORMAL,

// factory tests etc
    ATH10K_FIRMWARE_MODE_UTF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_fw_features {
// wmi_mgmt_rx_hdr contains extra RSSI information
    ATH10K_FW_FEATURE_EXT_WMI_MGMT_RX = 0,

// Firmware from 10X branch. Deprecated, don't use in new code.
    ATH10K_FW_FEATURE_WMI_10X = 1,

// firmware support tx frame management over WMI, otherwise it's HTT
    ATH10K_FW_FEATURE_HAS_WMI_MGMT_TX = 2,

// Firmware does not support P2P
    ATH10K_FW_FEATURE_NO_P2P = 3,

// Firmware 10.2 feature bit. The ATH10K_FW_FEATURE_WMI_10X feature
// bit is required to be set as well. Deprecated, don't use in new
// code.
//
    ATH10K_FW_FEATURE_WMI_10_2 = 4,

// Some firmware revisions lack proper multi-interface client powersave
// implementation. Enabling PS could result in connection drops,
// traffic stalls, etc.
//
    ATH10K_FW_FEATURE_MULTI_VIF_PS_SUPPORT = 5,

// Some firmware revisions have an incomplete WoWLAN implementation
// despite WMI service bit being advertised. This feature flag is used
// to distinguish whether WoWLAN is really supported or not.
//
    ATH10K_FW_FEATURE_WOWLAN_SUPPORT = 6,

// Don't trust error code from otp.bin
    ATH10K_FW_FEATURE_IGNORE_OTP_RESULT = 7,

// Some firmware revisions pad 4th hw address to 4 byte boundary making
// it 8 bytes long in Native Wifi Rx decap.
//
    ATH10K_FW_FEATURE_NO_NWIFI_DECAP_4ADDR_PADDING = 8,

// Firmware supports bypassing PLL setting on init.
    ATH10K_FW_FEATURE_SUPPORTS_SKIP_CLOCK_INIT = 9,

// Raw mode support. If supported, FW supports receiving and transmitting
// frames in raw mode.
//
    ATH10K_FW_FEATURE_RAW_MODE_SUPPORT = 10,

// Firmware Supports Adaptive CCA
    ATH10K_FW_FEATURE_SUPPORTS_ADAPTIVE_CCA = 11,

// Firmware supports management frame protection
    ATH10K_FW_FEATURE_MFP_SUPPORT = 12,

// Firmware supports pull-push model where host shares it's software
// queue state with firmware and firmware generates fetch requests
// telling host which queues to dequeue tx from.
//
// Primary function of this is improved MU-MIMO performance with
// multiple clients.
//
    ATH10K_FW_FEATURE_PEER_FLOW_CONTROL = 13,

// Firmware supports BT-Coex without reloading firmware via pdev param.
// To support Bluetooth coexistence pdev param, WMI_COEX_GPIO_SUPPORT of
// extended resource config should be enabled always. This firmware IE
// is used to configure WMI_COEX_GPIO_SUPPORT.
//
    ATH10K_FW_FEATURE_BTCOEX_PARAM = 14,

// Unused flag and proven to be not working, enable this if you want
// to experiment sending NULL func data frames in HTT TX
//
    ATH10K_FW_FEATURE_SKIP_NULL_FUNC_WAR = 15,

// Firmware allow other BSS mesh broadcast/multicast frames without
// creating monitor interface. Appropriate rxfilters are programmed for
// mesh vdev by firmware itself. This feature flags will be used for
// not creating monitor vdev while configuring mesh node.
//
    ATH10K_FW_FEATURE_ALLOWS_MESH_BCAST = 16,

// Firmware does not support power save in station mode.
    ATH10K_FW_FEATURE_NO_PS = 17,

// Firmware allows management tx by reference instead of by value.
    ATH10K_FW_FEATURE_MGMT_TX_BY_REF = 18,

// Firmware load is done externally, not by bmi
    ATH10K_FW_FEATURE_NON_BMI = 19,

// Firmware sends only one chan_info event per channel
    ATH10K_FW_FEATURE_SINGLE_CHAN_INFO_PER_CHANNEL = 20,

// Firmware allows setting peer fixed rate
    ATH10K_FW_FEATURE_PEER_FIXED_RATE = 21,

// Firmware support IRAM recovery
    ATH10K_FW_FEATURE_IRAM_RECOVERY = 22,

// keep last
    ATH10K_FW_FEATURE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_dev_flags {
// Indicates that ath10k device is during CAC phase of DFS
    ATH10K_CAC_RUNNING,
    ATH10K_FLAG_CORE_REGISTERED,

// Device has crashed and needs to restart. This indicates any pending
// waiters should immediately cancel instead of waiting for a time out.
//
    ATH10K_FLAG_CRASH_FLUSH,

// Use Raw mode instead of native WiFi Tx/Rx encap mode.
// Raw mode supports both hardware and software crypto. Native WiFi only
// supports hardware crypto.
//
    ATH10K_FLAG_RAW_MODE,

// Disable HW crypto engine
    ATH10K_FLAG_HW_CRYPTO_DISABLED,

// Bluetooth coexistence enabled
    ATH10K_FLAG_BTCOEX,

// Per Station statistics service
    ATH10K_FLAG_PEER_STATS,

// protected by conf_mutex
    ATH10K_FLAG_NAPI_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_cal_mode {
    ATH10K_CAL_MODE_FILE,
    ATH10K_CAL_MODE_OTP,
    ATH10K_CAL_MODE_DT,
    ATH10K_CAL_MODE_NVMEM,
    ATH10K_PRE_CAL_MODE_FILE,
    ATH10K_PRE_CAL_MODE_DT,
    ATH10K_PRE_CAL_MODE_NVMEM,
    ATH10K_CAL_MODE_EEPROM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_crypt_mode {
// Only use hardware crypto engine
    ATH10K_CRYPT_MODE_HW,
// Only use software crypto engine
    ATH10K_CRYPT_MODE_SW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_scan_state {
    ATH10K_SCAN_IDLE,
    ATH10K_SCAN_STARTING,
    ATH10K_SCAN_RUNNING,
    ATH10K_SCAN_ABORTING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_tx_pause_reason {
    ATH10K_TX_PAUSE_Q_FULL,
    ATH10K_TX_PAUSE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_file {
    pub firmware: *const firmware,
    pub fw_version: [c_char; ETHTOOL_FWVERS_LEN],
    pub ATH10K_FW_FEATURE_COUNT): DECLARE_BITMAP(fw_features,,
    pub wmi_op_version: ath10k_fw_wmi_op_version,
    pub htt_op_version: ath10k_fw_htt_op_version,
    pub firmware_data: *const c_void,
    pub firmware_len: usize,
    pub otp_data: *const c_void,
    pub otp_len: usize,
    pub codeswap_data: *const c_void,
    pub codeswap_len: usize,
// The original idea of struct ath10k_fw_file was that it only
// contains struct firmware and pointers to various parts (actual
// firmware binary, otp, metadata etc) of the file. This seg_info
// is actually created separate but as this is used similarly as
// the other firmware components it's more convenient to have it
// here.
//
    pub firmware_swap_code_seg_info: *mut ath10k_swap_code_seg_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_components {
    pub board: *const firmware,
    pub board_data: *const c_void,
    pub board_len: usize,
    pub ext_board: *const firmware,
    pub ext_board_data: *const c_void,
    pub ext_board_len: usize,
    pub fw_file: ath10k_fw_file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_per_peer_tx_stats {
    pub succ_bytes: u32,
    pub retry_bytes: u32,
    pub failed_bytes: u32,
    pub ratecode: u8,
    pub flags: u8,
    pub peer_id: u16,
    pub succ_pkts: u16,
    pub retry_pkts: u16,
    pub failed_pkts: u16,
    pub duration: u16,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_dev_type {
    ATH10K_DEV_TYPE_LL,
    ATH10K_DEV_TYPE_HL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_bus_params {
    pub chip_id: u32,
    pub dev_type: ath10k_dev_type,
    pub link_can_suspend: bool,
    pub hl_msdu_ids: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k {
    pub ath_common: ath_common,
    pub hw: *mut ieee80211_hw,
    pub ops: *mut ieee80211_ops,
    pub dev: *mut device,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msa_region {
    pub paddr: dma_addr_t,
    pub mem_size: u32,
    pub vaddr: *mut c_void,
    pub msa: },
    pub mac_addr: [u8; ETH_ALEN],
    pub hw_rev: ath10k_hw_rev,
    pub dev_id: u16,
    pub chip_id: u32,
    pub target_version: u32,
    pub fw_version_major: u8,
    pub fw_version_minor: u32,
    pub fw_version_release: u16,
    pub fw_version_build: u16,
    pub fw_stats_req_mask: u32,
    pub phy_capability: u32,
    pub hw_min_tx_power: u32,
    pub hw_max_tx_power: u32,
    pub hw_eeprom_rd: u32,
    pub ht_cap_info: u32,
    pub vht_cap_info: u32,
    pub vht_supp_mcs: u32,
    pub num_rf_chains: u32,
    pub max_spatial_stream: u32,
// protected by conf_mutex
    pub low_2ghz_chan: u32,
    pub high_2ghz_chan: u32,
    pub low_5ghz_chan: u32,
    pub high_5ghz_chan: u32,
    pub ani_enabled: bool,
    pub sys_cap_info: u32,
// protected by data_lock
    pub hw_rfkill_on: bool,
// protected by conf_mutex
    pub ps_state_enable: u8,
    pub nlo_enabled: bool,
    pub p2p: bool,
    pub bus: ath10k_bus,
    pub ops: *const ath10k_hif_ops,
    pub hif: },
    pub target_suspend: completion,
    pub driver_recovery: completion,
    pub regs: *const ath10k_hw_regs,
    pub hw_ce_regs: *const ath10k_hw_ce_regs,
    pub hw_values: *const ath10k_hw_values,
    pub bmi: ath10k_bmi,
    pub wmi: ath10k_wmi,
    pub htc: ath10k_htc,
    pub htt: ath10k_htt,
    pub hw_params: ath10k_hw_params,
// contains the firmware images used with ATH10K_FIRMWARE_MODE_NORMAL
    pub normal_mode_fw: ath10k_fw_components,
// READ-ONLY images of the running firmware, which can be either
// normal or UTF. Do not modify, release etc!
//
    pub running_fw: *const ath10k_fw_components,
    pub board_name: *const c_char,
    pub pre_cal_file: *const firmware,
    pub cal_file: *const firmware,
    pub vendor: u32,
    pub device: u32,
    pub subsystem_vendor: u32,
    pub subsystem_device: u32,
    pub bmi_ids_valid: bool,
    pub qmi_ids_valid: bool,
    pub qmi_board_id: u32,
    pub qmi_chip_id: u32,
    pub bmi_board_id: u8,
    pub bmi_eboard_id: u8,
    pub bmi_chip_id: u8,
    pub ext_bid_supported: bool,
    pub bdf_ext: [c_char; ATH10K_SMBIOS_BDF_EXT_STR_LENGTH],
    pub id: },
    pub fw_api: c_int,
    pub bd_api: c_int,
    pub cal_mode: ath10k_cal_mode,
    pub started: completion,
    pub completed: completion,
    pub on_channel: completion,
    pub timeout: delayed_work,
    pub state: ath10k_scan_state,
    pub is_roc: bool,
    pub vdev_id: c_int,
    pub roc_freq: c_int,
    pub roc_notify: bool,
    pub scan: },
    pub sbands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub mac: },
// should never be NULL; needed for regular htt rx
    pub rx_channel: *mut ieee80211_channel,
// valid during scan; needed for mgmt rx during scan
    pub scan_channel: *mut ieee80211_channel,
// current operating channel definition
    pub chandef: cfg80211_chan_def,
// currently configured operating channel in firmware
    pub tgt_oper_chan: *mut ieee80211_channel,
    pub free_vdev_map: c_ulonglong,
    pub monitor_arvif: *mut ath10k_vif,
    pub monitor: bool,
    pub monitor_vdev_id: c_int,
    pub monitor_started: bool,
    pub filter_flags: c_uint,
    pub dev_flags: c_ulong,
    pub dfs_block_radar_events: bool,
// protected by conf_mutex
    pub radar_enabled: bool,
    pub num_started_vdevs: c_int,
// Protected by conf-mutex
    pub cfg_tx_chainmask: u8,
    pub cfg_rx_chainmask: u8,
    pub install_key_done: completion,
    pub last_wmi_vdev_start_status: c_int,
    pub vdev_setup_done: completion,
    pub vdev_delete_done: completion,
    pub peer_stats_info_complete: completion,
    pub workqueue: *mut workqueue_struct,
// Auxiliary workqueue
    pub workqueue_aux: *mut workqueue_struct,
    pub workqueue_tx_complete: *mut workqueue_struct,
// prevents concurrent FW reconfiguration
    pub conf_mutex: mutex,
// protects coredump data
    pub dump_mutex: mutex,
// protects shared structure data
    pub data_lock: spinlock_t,
// serialize wake_tx_queue calls per ac
    pub queue_lock: [spinlock_t; IEEE80211_NUM_ACS],
    pub arvifs: list_head,
    pub peers: list_head,
    pub peer_map: [*mut ath10k_peer; ATH10K_MAX_NUM_PEER_IDS],
    pub peer_mapping_wq: wait_queue_head_t,
// protected by conf_mutex
    pub num_peers: c_int,
    pub num_stations: c_int,
    pub max_num_peers: c_int,
    pub max_num_stations: c_int,
    pub max_num_vdevs: c_int,
    pub max_num_tdls_vdevs: c_int,
    pub num_active_peers: c_int,
    pub num_tids: c_int,
    pub svc_rdy_work: work_struct,
    pub svc_rdy_skb: *mut sk_buff,
    pub offchan_tx_work: work_struct,
    pub offchan_tx_queue: sk_buff_head,
    pub offchan_tx_completed: completion,
    pub offchan_tx_skb: *mut sk_buff,
    pub wmi_mgmt_tx_work: work_struct,
    pub wmi_mgmt_tx_queue: sk_buff_head,
    pub state: ath10k_state,
    pub register_work: work_struct,
    pub restart_work: work_struct,
    pub recovery_check_work: work_struct,
    pub bundle_tx_work: work_struct,
    pub tx_complete_work: work_struct,
    pub pending_recovery: core::sync::atomic::AtomicI32,
    pub recovery_count: c_uint,
// continuous recovery fail count
    pub fail_cont_count: core::sync::atomic::AtomicI32,
// cycle count is reported twice for each visited channel during scan.
// access protected by data_lock
//
    pub survey_last_rx_clear_count: u32,
    pub survey_last_cycle_count: u32,
    pub survey: [survey_info; ATH10K_NUM_CHANS],
// Channel info events are expected to come in pairs without and with
// COMPLETE flag set respectively for each channel visit during scan.
//
// However there are deviations from this rule. This flag is used to
// avoid reporting garbage data.
//
    pub ch_info_can_report_survey: bool,
    pub bss_survey_done: completion,
    pub dfs_detector: *mut dfs_pattern_detector,
    pub /: *mut *mut unsigned long tx_paused; / see ATH10K_TX_PAUSE_,

    pub debug: ath10k_debug,
// relay(fs) channel for spectral scan
    pub rfs_chan_spec_scan: *mut rchan,
// spectral_mode and spec_config are protected by conf_mutex
    pub mode: ath10k_spectral_mode,
    pub config: ath10k_spec_scan,
    pub spectral: },

    pub pktlog_filter: u32,

    pub fw_crash_data: *mut ath10k_fw_crash_data,
    pub coredump: },

// protected by conf_mutex
    pub utf_mode_fw: ath10k_fw_components,
    pub ftm_msgref: u8,
// protected by data_lock
    pub utf_monitor: bool,
    pub data_pos: u32,
    pub expected_seq: u32,
    pub eventdata: *mut u8,
    pub testmode: },
    pub cdev: led_classdev,
    pub label: [c_char; 48],
    pub gpio_state_pin: u32,
    pub leds: },
// protected by data_lock
    pub rx_crc_err_drop: u32,
    pub fw_crash_counter: u32,
    pub fw_warm_reset_counter: u32,
    pub fw_cold_reset_counter: u32,
    pub stats: },
    pub thermal: ath10k_thermal,
    pub wow: ath10k_wow,
    pub peer_tx_stats: ath10k_per_peer_tx_stats,
// NAPI
    pub napi_dev: *mut net_device,
    pub napi: napi_struct,
    pub set_coverage_class_work: work_struct,
// protected by conf_mutex
// writing also protected by data_lock
    pub coverage_class: i16,
    pub reg_phyclk: u32,
    pub reg_slottime_conf: u32,
    pub reg_slottime_orig: u32,
    pub reg_ack_cts_timeout_conf: u32,
    pub reg_ack_cts_timeout_orig: u32,
    pub fw_coverage: },
    pub ampdu_reference: u32,
    pub wmi_key_cipher: *const u8,
    pub ce_priv: *mut c_void,
    pub sta_tid_stats_mask: u32,
// protected by data_lock
    pub radar_conf_state: ath10k_radar_confirmation_state,
    pub last_radar_info: ath10k_radar_found_info,
    pub radar_confirmation_work: work_struct,
    pub bus_param: ath10k_bus_params,
    pub peer_delete_done: completion,
    pub coex_support: bool,
    pub coex_gpio_pin: c_int,
    pub tx_power_2g_limit: i32,
    pub tx_power_5g_limit: i32,
// must be last
    pub )): *mut u8 drv_priv[] __aligned(sizeof(void,
}

extern "C" {
    pub fn ath10k_core_napi_sync_disable(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_core_napi_enable(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_core_destroy(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_wait_for_suspend(ar: *mut ath10k, suspend_opt: u32) -> c_int;
}
extern "C" {
    pub fn ath10k_core_stop(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_core_start_recovery(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_core_unregister(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_core_fetch_board_file(ar: *mut ath10k, bd_ie_type: c_int) -> c_int;
}
extern "C" {
    pub fn ath10k_core_check_dt(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_core_free_board_files(ar: *mut ath10k);
}
