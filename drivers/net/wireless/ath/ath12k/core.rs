//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/core.h
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
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH12K_TX_MGMT_NUM_PENDING_MAX: c_int = 512;
pub const ATH12K_TX_MGMT_TARGET_MAX_SUPPORT_WMI: c_int = 64;
// Pending management packets threshold for dropping probe responses

// SMBIOS type containing Board Data File Name Extension
pub const ATH12K_SMBIOS_BDF_EXT_TYPE: c_uint = 0xF8;
// SMBIOS type structure length (excluding strings-set)
pub const ATH12K_SMBIOS_BDF_EXT_LENGTH: c_uint = 0x9;
// The magic used by QCA spec

pub const ATH12K_INVALID_HW_MAC_ID: c_uint = 0xFF;

pub const ATH12K_MON_TIMER_INTERVAL: c_int = 10;

pub const ATH12K_RESET_MAX_FAIL_COUNT_FIRST: c_int = 3;
pub const ATH12K_RESET_MAX_FAIL_COUNT_FINAL: c_int = 5;

pub const ATH12K_INVALID_GROUP_ID: c_uint = 0xFF;
pub const ATH12K_INVALID_DEVICE_ID: c_uint = 0xFF;
pub const ATH12K_MAX_MLO_PEERS: c_int = 256;
pub const ATH12K_MLO_PEER_ID_INVALID: c_uint = 0xFFFF;
pub const ATH12K_MLO_PEER_ID_PENDING: c_uint = 0xFFFE;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_bdf_search {
    ATH12K_BDF_SEARCH_DEFAULT,
    ATH12K_BDF_SEARCH_BUS_AND_BOARD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wme_ac {
    WME_AC_BE,
    WME_AC_BK,
    WME_AC_VI,
    WME_AC_VO,
    WME_NUM_AC
}

pub const ATH12K_HT_MCS_MAX: c_int = 7;
pub const ATH12K_VHT_MCS_MAX: c_int = 9;
pub const ATH12K_HE_MCS_MAX: c_int = 11;
pub const ATH12K_EHT_MCS_MAX: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_crypt_mode {
// Only use hardware crypto engine
    ATH12K_CRYPT_MODE_HW,
// Only use software crypto
    ATH12K_CRYPT_MODE_SW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_skb_flags {
    ATH12K_SKB_HW_80211_ENCAP = BIT(0),
    ATH12K_SKB_CIPHER_SET = BIT(1),
    ATH12K_SKB_MLO_STA = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_skb_cb {
    pub paddr: dma_addr_t,
    pub ar: *mut ath12k,
    pub vif: *mut ieee80211_vif,
    pub paddr_ext_desc: dma_addr_t,
    pub cipher: u32,
    pub flags: u8,
    pub link_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_skb_rxcb {
    pub paddr: dma_addr_t,
    pub is_first_msdu: bool,
    pub is_last_msdu: bool,
    pub is_continuation: bool,
    pub is_mcbc: bool,
    pub is_eapol: bool,
    pub rx_desc: *mut hal_rx_desc,
    pub err_rel_src: u8,
    pub err_code: u8,
    pub hw_link_id: u8,
    pub unmapped: u8,
    pub is_frag: u8,
    pub tid: u8,
    pub peer_id: u16,
    pub is_end_of_ppdu: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_hw_rev {
    ATH12K_HW_QCN9274_HW10,
    ATH12K_HW_QCN9274_HW20,
    ATH12K_HW_WCN7850_HW20,
    ATH12K_HW_IPQ5332_HW10,
    ATH12K_HW_QCC2072_HW10,
    ATH12K_HW_IPQ5424_HW10,
}

pub const ATH12K_IRQ_NUM_MAX: c_int = 57;
pub const ATH12K_EXT_IRQ_NUM_MAX: c_int = 16;
pub const ATH12K_MAX_TCL_RING_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ext_irq_grp {
    pub ab: *mut ath12k_base,
    pub irqs: [u32; ATH12K_EXT_IRQ_NUM_MAX],
    pub num_irq: u32,
    pub grp_id: u32,
    pub timestamp: u64,
    pub napi_enabled: bool,
    pub napi: napi_struct,
    pub napi_ndev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_smbios_cc_type {
// disable country code setting from SMBIOS
    ATH12K_SMBIOS_CC_DISABLE = 0,

// set country code by ANSI country name, based on ISO3166-1 alpha2
    ATH12K_SMBIOS_CC_ISO = 1,

// worldwide regdomain
    ATH12K_SMBIOS_CC_WW = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_smbios_bdf {
    pub hdr: dmi_header,
    pub features_disabled: u8,
// enum ath12k_smbios_cc_type
    pub country_code_flag: u8,
// To set specific country, you need to set country code
// flag=ATH12K_SMBIOS_CC_ISO first, then if country is United
// States, then country code value = 0x5553 ("US",'U' = 0x55, 'S'=
// 0x53). To set country to INDONESIA, then country code value =
// 0x4944 ("IN", 'I'=0x49, 'D'=0x44). If country code flag =
// ATH12K_SMBIOS_CC_WW, then you can use worldwide regulatory
// setting.
//
    pub cc_code: u16,
    pub bdf_enabled: u8,
    pub bdf_ext: [u8; ],
    pub __packed: },
pub const HEHANDLE_CAP_PHYINFO_SIZE: c_int = 3;
pub const HECAP_PHYINFO_SIZE: c_int = 9;
pub const HECAP_MACINFO_SIZE: c_int = 5;
pub const HECAP_TXRX_MCS_NSS_SIZE: c_int = 2;
pub const HECAP_PPET16_PPET8_MAX_SIZE: c_int = 25;
pub const HE_PPET16_PPET8_SIZE: c_int = 8;
// 802.11ax PPE (PPDU packet Extension) threshold
#[repr(C)]
#[derive(Copy, Clone)]
pub struct he_ppe_threshold {
    pub numss_m1: u32,
    pub ru_mask: u32,
    pub ppet16_ppet8_ru3_ru0: [u32; HE_PPET16_PPET8_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_he {
    pub hecap_macinfo: [u8; HECAP_MACINFO_SIZE],
    pub hecap_rxmcsnssmap: u32,
    pub hecap_txmcsnssmap: u32,
    pub hecap_phyinfo: [u32; HEHANDLE_CAP_PHYINFO_SIZE],
    pub hecap_ppet: he_ppe_threshold,
    pub heop_param: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_scan_state {
    ATH12K_SCAN_IDLE,
    ATH12K_SCAN_STARTING,
    ATH12K_SCAN_RUNNING,
    ATH12K_SCAN_ABORTING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_11d_state {
    ATH12K_11D_IDLE,
    ATH12K_11D_PREPARING,
    ATH12K_11D_RUNNING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_hw_group_flags {
    ATH12K_GROUP_FLAG_REGISTERED,
    ATH12K_GROUP_FLAG_UNREGISTER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_dev_flags {
    ATH12K_FLAG_CAC_RUNNING,
    ATH12K_FLAG_CRASH_FLUSH,
    ATH12K_FLAG_RAW_MODE,
    ATH12K_FLAG_HW_CRYPTO_DISABLED,
    ATH12K_FLAG_RECOVERY,
    ATH12K_FLAG_UNREGISTERING,
    ATH12K_FLAG_REGISTERED,
    ATH12K_FLAG_QMI_FAIL,
    ATH12K_FLAG_HTC_SUSPEND_COMPLETE,
    ATH12K_FLAG_CE_IRQ_ENABLED,
    ATH12K_FLAG_EXT_IRQ_ENABLED,
    ATH12K_FLAG_QMI_FW_READY_COMPLETE,
    ATH12K_FLAG_FTM_SEGMENTED,
    ATH12K_FLAG_FIXED_MEM_REGION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_tx_conf {
    pub changed: bool,
    pub ac: u16,
    pub tx_queue_params: ieee80211_tx_queue_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_key_conf {
    pub cmd: set_key_cmd,
    pub list: list_head,
    pub sta: *mut ieee80211_sta,
    pub key: *mut ieee80211_key_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_vif_cache {
    pub tx_conf: ath12k_tx_conf,
    pub key_conf: ath12k_key_conf,
    pub bss_conf_changed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_rekey_data {
    pub kck: [u8; NL80211_KCK_LEN],
    pub kek: [u8; NL80211_KCK_LEN],
    pub replay_ctr: u64,
    pub enable_offload: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_link_vif {
    pub vdev_id: u32,
    pub beacon_interval: u32,
    pub dtim_period: u32,
    pub ar: *mut ath12k,
    pub beacon_prot: bool,
    pub wmm_params: wmi_wmm_params_all_arg,
    pub list: list_head,
    pub is_created: bool,
    pub is_started: bool,
    pub is_up: bool,
    pub bssid: [u8; ETH_ALEN],
    pub bitrate_mask: cfg80211_bitrate_mask,
    pub connection_loss_work: delayed_work,
    pub num_legacy_stations: c_int,
    pub rtscts_prot_mode: c_int,
    pub txpower: c_int,
    pub rsnie_present: bool,
    pub wpaie_present: bool,
    pub vdev_stats_id: u8,
    pub punct_bitmap: u32,
    pub link_id: u8,
    pub ahvif: *mut ath12k_vif,
    pub rekey_data: ath12k_rekey_data,
    pub link_stats: ath12k_link_stats,
    pub /: *mut *mut spinlock_t link_stats_lock; / Protects updates to link_stats,
    pub current_cntdown_counter: u8,
// only used in station mode
    pub is_sta_assoc_link: bool,
    pub reg_tpc_info: ath12k_reg_tpc_power_info,
    pub group_key_valid: bool,
    pub group_key: wmi_vdev_install_key_arg,
    pub pairwise_key_done: bool,
    pub num_stations: u16,
    pub is_csa_in_progress: bool,
    pub bcn_tx_work: wiphy_work,
    pub set_wds_vdev_param: bool,
    pub nawds_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_vif {
    pub dp_vif: ath12k_dp_vif,
    pub vdev_type: wmi_vdev_type,
    pub vdev_subtype: wmi_vdev_subtype,
    pub vif: *mut ieee80211_vif,
    pub ah: *mut ath12k_hw,
    pub uapsd: u32,
    pub sta: },
// 127 stations; wmi limit
    pub tim_bitmap: [u8; 16],
    pub tim_len: u8,
    pub ssid_len: u32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub hidden_ssid: bool,
// P2P_IE with NoA attribute for P2P_GO case
    pub noa_len: u32,
    pub noa_data: *mut u8,
    pub ap: },
    pub u: },
    pub aid: u32,
    pub ps: bool,
    pub deflink: ath12k_link_vif,
    pub link: [*mut ath12k_link_vif __rcu; ATH12K_NUM_MAX_LINKS],
    pub cache: [*mut ath12k_vif_cache; IEEE80211_MLD_MAX_NUM_LINKS],
// indicates bitmap of link vif created in FW
    pub links_map: u32,
// Must be last - ends in a flexible-array member.
//
// FIXME: Driver should not copy struct ieee80211_chanctx_conf,
// especially because it has a flexible array. Find a better way.
//
    pub chanctx: ieee80211_chanctx_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_vif_iter {
    pub vdev_id: u32,
    pub ar: *mut ath12k,
    pub arvif: *mut ath12k_link_vif,
}

pub const ATH12K_HE_MCS_NUM: c_int = 12;
pub const ATH12K_VHT_MCS_NUM: c_int = 10;
pub const ATH12K_BW_NUM: c_int = 5;
pub const ATH12K_NSS_NUM: c_int = 4;
pub const ATH12K_LEGACY_NUM: c_int = 12;
pub const ATH12K_GI_NUM: c_int = 4;
pub const ATH12K_HT_MCS_NUM: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_pkt_rx_err {
    ATH12K_PKT_RX_ERR_FCS,
    ATH12K_PKT_RX_ERR_TKIP,
    ATH12K_PKT_RX_ERR_CRYPT,
    ATH12K_PKT_RX_ERR_PEER_IDX_INVAL,
    ATH12K_PKT_RX_ERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_ampdu_subfrm_num {
    ATH12K_AMPDU_SUBFRM_NUM_10,
    ATH12K_AMPDU_SUBFRM_NUM_20,
    ATH12K_AMPDU_SUBFRM_NUM_30,
    ATH12K_AMPDU_SUBFRM_NUM_40,
    ATH12K_AMPDU_SUBFRM_NUM_50,
    ATH12K_AMPDU_SUBFRM_NUM_60,
    ATH12K_AMPDU_SUBFRM_NUM_MORE,
    ATH12K_AMPDU_SUBFRM_NUM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_amsdu_subfrm_num {
    ATH12K_AMSDU_SUBFRM_NUM_1,
    ATH12K_AMSDU_SUBFRM_NUM_2,
    ATH12K_AMSDU_SUBFRM_NUM_3,
    ATH12K_AMSDU_SUBFRM_NUM_4,
    ATH12K_AMSDU_SUBFRM_NUM_MORE,
    ATH12K_AMSDU_SUBFRM_NUM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_counter_type {
    ATH12K_COUNTER_TYPE_BYTES,
    ATH12K_COUNTER_TYPE_PKTS,
    ATH12K_COUNTER_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_stats_type {
    ATH12K_STATS_TYPE_SUCC,
    ATH12K_STATS_TYPE_FAIL,
    ATH12K_STATS_TYPE_RETRY,
    ATH12K_STATS_TYPE_AMPDU,
    ATH12K_STATS_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_data_stats {
    pub legacy: [u64; ATH12K_COUNTER_TYPE_MAX][ATH12K_LEGACY_NUM],
    pub ht: [u64; ATH12K_COUNTER_TYPE_MAX][ATH12K_HT_MCS_NUM],
    pub vht: [u64; ATH12K_COUNTER_TYPE_MAX][ATH12K_VHT_MCS_NUM],
    pub he: [u64; ATH12K_COUNTER_TYPE_MAX][ATH12K_HE_MCS_NUM],
    pub bw: [u64; ATH12K_COUNTER_TYPE_MAX][ATH12K_BW_NUM],
    pub nss: [u64; ATH12K_COUNTER_TYPE_MAX][ATH12K_NSS_NUM],
    pub gi: [u64; ATH12K_COUNTER_TYPE_MAX][ATH12K_GI_NUM],
    pub transmit_type: [u64; ATH12K_COUNTER_TYPE_MAX][HAL_RX_RECEPTION_TYPE_MAX],
    pub ru_loc: [u64; ATH12K_COUNTER_TYPE_MAX][HAL_RX_RU_ALLOC_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_stats {
    pub stats: [ath12k_htt_data_stats; ATH12K_STATS_TYPE_MAX],
    pub tx_duration: u64,
    pub ba_fails: u64,
    pub ack_fails: u64,
    pub ru_start: u16,
    pub ru_tones: u16,
    pub mu_group: [u32; MAX_MU_GROUP_ID],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_per_ppdu_tx_stats {
    pub succ_pkts: u16,
    pub failed_pkts: u16,
    pub retry_pkts: u16,
    pub succ_bytes: u32,
    pub failed_bytes: u32,
    pub retry_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_link_sta {
    pub arvif: *mut ath12k_link_vif,
    pub ahsta: *mut ath12k_sta,
// link address similar to ieee80211_link_sta
    pub addr: [u8; ETH_ALEN],
    pub tcl_metadata: u16,
    pub ast_hash: u16,
    pub ast_idx: u16,
// the following are protected by ar->data_lock
    pub /: *mut *mut *mut u32 changed; / IEEE80211_RC_,
    pub bw: u32,
    pub nss: u32,
    pub smps: u32,
    pub update_wk: wiphy_work,
    pub link_id: u8,
    pub bw_prev: u32,
    pub peer_nss: u32,
    pub rssi_beacon: i8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
// For now the assoc link will be considered primary
    pub is_assoc_link: bool,
// for firmware use only
    pub link_idx: u8,
// peer addr based rhashtable list pointer
    pub rhash_addr: rhash_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_sta {
    pub ahvif: *mut ath12k_vif,
    pub pn_type: hal_pn_type,
    pub deflink: ath12k_link_sta,
    pub link: [*mut ath12k_link_sta __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
// indicates bitmap of link sta created in FW
    pub links_map: u16,
    pub assoc_link_id: u8,
    pub ml_peer_id: u16,
    pub free_logical_link_idx_map: u16,
    pub state: ieee80211_sta_state,
    pub enable_4addr: bool,
}

pub const ATH12K_HALF_20MHZ_BW: c_int = 10;
pub const ATH12K_2GHZ_MIN_CENTER: c_int = 2412;
pub const ATH12K_2GHZ_MAX_CENTER: c_int = 2484;
pub const ATH12K_5GHZ_MIN_CENTER: c_int = 4900;
pub const ATH12K_5GHZ_MAX_CENTER: c_int = 5920;
pub const ATH12K_6GHZ_MIN_CENTER: c_int = 5935;
pub const ATH12K_6GHZ_MAX_CENTER: c_int = 7115;

pub const ATH12K_NUM_CHANS: c_int = 102;
pub const ATH12K_MAX_5GHZ_CHAN: c_int = 177;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_hw_state {
    ATH12K_HW_STATE_OFF,
    ATH12K_HW_STATE_ON,
    ATH12K_HW_STATE_RESTARTING,
    ATH12K_HW_STATE_RESTARTED,
    ATH12K_HW_STATE_WEDGED,
    ATH12K_HW_STATE_TM,
// Add other states as required
}

// Antenna noise floor

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ftm_event_obj {
    pub data_pos: u32,
    pub expected_seq: u32,
    pub eventdata: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_fw_stats {
    pub pdev_id: u32,
    pub stats_id: u32,
    pub pdevs: list_head,
    pub vdevs: list_head,
    pub bcn: list_head,
    pub num_vdev_recvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dbg_htt_stats {
    pub type: ath12k_dbg_htt_ext_stats_type,
    pub cfg_param: [u32; 4],
    pub reset: u8,
    pub stats_req: *mut debug_htt_stats_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_debug {
    pub debugfs_pdev: *mut dentry,
    pub debugfs_pdev_symlink: *mut dentry,
    pub debugfs_pdev_symlink_default: *mut dentry,
    pub htt_stats: ath12k_dbg_htt_stats,
    pub tpc_stats_type: wmi_halphy_ctrl_path_stats_id,
    pub tpc_request: bool,
    pub tpc_complete: completion,
    pub tpc_stats: *mut wmi_tpc_stats_arg,
    pub rx_filter: u32,
    pub extd_rx_stats: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pdev_rssi_offsets {
    pub temp_offset: i32,
    pub min_nf_dbm: i8,
// Cache the sum here to avoid calculating it every time in hot path
// noise_floor = min_nf_dbm + temp_offset
//
    pub noise_floor: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k {
    pub ab: *mut ath12k_base,
    pub pdev: *mut ath12k_pdev,
    pub ah: *mut ath12k_hw,
    pub wmi: *mut ath12k_wmi_pdev,
    pub dp: ath12k_pdev_dp,
    pub mac_addr: [u8; ETH_ALEN],
    pub ht_cap_info: u32,
    pub vht_cap_info: u32,
    pub ar_he: ath12k_he,
    pub supports_6ghz: bool,
    pub started: completion,
    pub completed: completion,
    pub on_channel: completion,
    pub timeout: delayed_work,
    pub state: ath12k_scan_state,
    pub is_roc: bool,
    pub roc_freq: c_int,
    pub roc_notify: bool,
    pub vdev_clean_wk: wiphy_work,
    pub arvif: *mut ath12k_link_vif,
    pub scan: },
    pub sbands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub mac: },
    pub dev_flags: c_ulong,
    pub filter_flags: c_uint,
    pub min_tx_power: u32,
    pub max_tx_power: u32,
    pub txpower_limit_2g: u32,
    pub txpower_limit_5g: u32,
    pub txpower_scale: u32,
    pub power_scale: u32,
    pub chan_tx_pwr: u32,
    pub rts_threshold: u32,
    pub num_stations: u32,
    pub max_num_stations: u32,
// protects the radio specific data like debug stats, ppdu_stats_info stats,
// vdev_stop_status info, scan data, ath12k_sta info, ath12k_link_vif info,
// channel context data, test mode data, regd_channel_update_queue,
// peer_delete_waits.
//
    pub data_lock: spinlock_t,
    pub arvifs: list_head,
// should never be NULL; needed for regular htt rx
    pub rx_channel: *mut ieee80211_channel,
// valid during scan; needed for mgmt rx during scan
    pub scan_channel: *mut ieee80211_channel,
    pub cfg_tx_chainmask: u8,
    pub cfg_rx_chainmask: u8,
    pub num_rx_chains: u8,
    pub num_tx_chains: u8,
// pdev_idx starts from 0 whereas pdev->pdev_id starts with 1
    pub pdev_idx: u8,
    pub lmac_id: u8,
    pub hw_link_id: u8,
    pub radio_idx: u8,
    pub peer_assoc_done: completion,
    pub peer_delete_waits: list_head,
    pub install_key_status: c_int,
    pub install_key_done: completion,
    pub last_wmi_vdev_start_status: c_int,
    pub vdev_setup_done: completion,
    pub vdev_delete_done: completion,
    pub num_peers: c_int,
    pub max_num_peers: c_int,
    pub num_started_vdevs: u32,
    pub num_created_vdevs: u32,
    pub allocated_vdev_map: c_ulonglong,
    pub txmgmt_idr: idr,
// protects txmgmt_idr data
    pub txmgmt_idr_lock: spinlock_t,
    pub num_pending_mgmt_tx: core::sync::atomic::AtomicI32,
    pub txmgmt_empty_waitq: wait_queue_head_t,
// cycle count is reported twice for each visited channel during scan.
// access protected by data_lock
//
    pub survey_last_rx_clear_count: u32,
    pub survey_last_cycle_count: u32,
// Channel info events are expected to come in pairs without and with
// COMPLETE flag set respectively for each channel visit during scan.
//
// However there are deviations from this rule. This flag is used to
// avoid reporting garbage data.
//
    pub ch_info_can_report_survey: bool,
    pub bss_survey_done: completion,
    pub regd_update_work: work_struct,
    pub regd_channel_update_work: work_struct,
    pub regd_channel_update_queue: list_head,
    pub wmi_mgmt_tx_work: wiphy_work,
    pub wmi_mgmt_tx_queue: sk_buff_head,
    pub wow: ath12k_wow,
    pub target_suspend: completion,
    pub target_suspend_ack: bool,
    pub cached_stats: ath12k_per_peer_tx_stats,
    pub last_ppdu_id: u32,
    pub cached_ppdu_id: u32,

    pub debug: ath12k_debug,

    pub dfs_block_radar_events: bool,
    pub monitor_vdev_created: bool,
    pub monitor_started: bool,
    pub monitor_vdev_id: c_int,
    pub freq_range: wiphy_radio_freq_range,
    pub nlo_enabled: bool,
// Protected by wiphy::mtx lock.
    pub vdev_id_11d_scan: u32,
    pub completed_11d_scan: completion,
    pub state_11d: ath12k_11d_state,
    pub alpha2: [u8; REG_ALPHA2_LEN],
    pub regdom_set_by_user: bool,
    pub regd_update_completed: completion,
    pub fw_stats_complete: completion,
    pub fw_stats_done: completion,
    pub mlo_setup_done: completion,
    pub mlo_setup_status: u32,
    pub ftm_msgref: u8,
    pub fw_stats: ath12k_fw_stats,
    pub last_tx_power_update: c_ulong,
    pub max_allowed_tx_power: i8,
    pub rssi_info: ath12k_pdev_rssi_offsets,
    pub thermal: ath12k_thermal,
// Protected by ar->data_lock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_incumbent_signal_interference {
    pub center_freq: u32,
    pub width: nl80211_chan_width,
    pub chan_bw_interference_bitmap: u32,
    pub handling_in_progress: bool,
    pub incumbent_signal_interference: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw {
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
// Protect the write operation of the hardware state ath12k_hw::state
// between hardware start<=>reconfigure<=>stop transitions.
//
    pub hw_mutex: mutex,
    pub state: ath12k_hw_state,
// protects survey[] shared across radios of this hw.
    pub survey_lock: spinlock_t,
    pub survey: [survey_info; ATH12K_NUM_CHANS],
    pub regd_updated: bool,
    pub use_6ghz_regd: bool,
    pub host_alloc_ml_id: bool,
    pub peer_ml_id_done: completion,
    pub num_radio: u8,
    pub ATH12K_MAX_MLO_PEERS): DECLARE_BITMAP(free_ml_peer_id_map,,
    pub dp_hw: ath12k_dp_hw,
// Keep last
    pub )): *mut ath12k radio[] __aligned(sizeof(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_band_cap {
    pub phy_id: u32,
    pub max_bw_supported: u32,
    pub ht_cap_info: u32,
    pub he_cap_info: [u32; 2],
    pub he_mcs: u32,
    pub he_cap_phy_info: [u32; PSOC_HOST_MAX_PHY_SIZE],
    pub he_ppet: ath12k_wmi_ppe_threshold_arg,
    pub he_6ghz_capa: u16,
    pub eht_cap_mac_info: [u32; WMI_MAX_EHTCAP_MAC_SIZE],
    pub eht_cap_phy_info: [u32; WMI_MAX_EHTCAP_PHY_SIZE],
    pub eht_mcs_20_only: u32,
    pub eht_mcs_80: u32,
    pub eht_mcs_160: u32,
    pub eht_mcs_320: u32,
    pub eht_ppet: ath12k_wmi_ppe_threshold_arg,
    pub eht_cap_info_internal: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pdev_cap {
    pub supported_bands: u32,
    pub ampdu_density: u32,
    pub vht_cap: u32,
    pub vht_mcs: u32,
    pub he_mcs: u32,
    pub tx_chain_mask: u32,
    pub rx_chain_mask: u32,
    pub tx_chain_mask_shift: u32,
    pub rx_chain_mask_shift: u32,
    pub band: [ath12k_band_cap; NUM_NL80211_BANDS],
    pub eml_cap: u32,
    pub mld_cap: u32,
    pub nss_ratio_enabled: bool,
    pub nss_ratio_info: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlo_timestamp {
    pub info: u32,
    pub sync_timestamp_lo_us: u32,
    pub sync_timestamp_hi_us: u32,
    pub mlo_offset_lo: u32,
    pub mlo_offset_hi: u32,
    pub mlo_offset_clks: u32,
    pub mlo_comp_clks: u32,
    pub mlo_comp_timer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pdev {
    pub ar: *mut ath12k,
    pub pdev_id: u32,
    pub hw_link_id: u32,
    pub cap: ath12k_pdev_cap,
    pub mac_addr: [u8; ETH_ALEN],
    pub timestamp: mlo_timestamp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_fw_pdev {
    pub pdev_id: u32,
    pub phy_id: u32,
    pub supported_bands: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_board_data {
    pub fw: *const firmware,
    pub data: *const c_void,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_reg_freq {
    pub start_freq: u32,
    pub end_freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_mlo_memory {
    pub chunk: [target_mem_chunk; ATH12K_QMI_WLANFW_MAX_NUM_MEM_SEG_V01],
    pub mlo_mem_size: c_int,
    pub init_done: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_link {
    pub device_id: u8,
    pub pdev_idx: u8,
}

// Holds info on the group of devices that are registered as a single
// wiphy, protected with struct ath12k_hw_group::mutex.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_group {
// Keep dp_hw_grp as the first member to allow efficient
// usage of cache lines for DP fields
//
    pub dp_hw_grp: ath12k_dp_hw_group,
    pub hw_links: [ath12k_hw_link; ATH12K_GROUP_MAX_RADIO],
    pub list: list_head,
    pub id: u8,
    pub num_devices: u8,
    pub num_probed: u8,
    pub num_started: u8,
    pub flags: c_ulong,
    pub ab: [*mut ath12k_base; ATH12K_MAX_DEVICES],
// protects access to this struct
    pub mutex: mutex,
// Holds information of wiphy (hw) registration.
//
// In Multi/Single Link Operation case, all pdevs are registered as
// a single wiphy. In other (legacy/Non-MLO) cases, each pdev is
// registered as separate wiphys.
//
    pub ah: [*mut ath12k_hw; ATH12K_GROUP_MAX_RADIO],
    pub num_hw: u8,
    pub mlo_capable: bool,
    pub wsi_node: [*mut device_node; ATH12K_MAX_DEVICES],
    pub mlo_mem: ath12k_mlo_memory,
    pub hw_link_id_init_done: bool,
}

// Holds WSI info specific to each device, excluding WSI group info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_wsi_info {
    pub index: u32,
    pub hw_link_id_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_profile_params {
    pub tx_comp_ring_size: u32,
    pub rxdma_monitor_buf_ring_size: u32,
    pub rxdma_monitor_dst_ring_size: u32,
    pub num_pool_tx_desc: u32,
    pub rx_desc_count: u32,
    pub rx_release_ring_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_mem_profile_based_param {
    pub num_vdevs: u32,
    pub max_client_single: u32,
    pub max_client_dbs: u32,
    pub max_client_dbs_sbs: u32,
    pub dp_params: ath12k_dp_profile_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_device_family {
    ATH12K_DEVICE_FAMILY_START,
    ATH12K_DEVICE_FAMILY_WIFI7 = ATH12K_DEVICE_FAMILY_START,
    ATH12K_DEVICE_FAMILY_MAX,
}

// Master structure to hold the hw data which may be used in core module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_base {
    pub hw_rev: ath12k_hw_rev,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub qmi: ath12k_qmi,
    pub wmi_ab: ath12k_wmi_base,
    pub fw_ready: completion,
    pub device_id: u8,
    pub num_radios: c_int,
// HW channel counters frequency value in hertz common to all MACs
    pub cc_freq_hz: u32,
    pub dump_data: *mut ath12k_dump_file_data,
    pub ath12k_coredump_len: usize,
    pub dump_work: work_struct,
    pub htc: ath12k_htc,
    pub dp: *mut ath12k_dp,
    pub mem: *mut void __iomem,
    pub mem_len: c_ulong,
    pub mem_ce: *mut void __iomem,
    pub ce_remap_base_addr: u32,
    pub cmem_offset: u32,
    pub ce_remap: bool,
    pub bus: ath12k_bus,
    pub ops: *const ath12k_hif_ops,
    pub hif: },
    pub wakeup_completed: completion,
    pub wmi_conf_rx_decap_mode: u32,
    pub wow: },
    pub ce: ath12k_ce,
    pub rx_replenish_retry: timer_list,
    pub hal: ath12k_hal,
// To synchronize core_start/core_stop
    pub core_lock: mutex,
// Protects data like peers
    pub base_lock: spinlock_t,
// Single pdev device (struct ath12k_hw_params::single_pdev_only):
//
// Firmware maintains data for all bands but advertises a single
// phy to the host which is stored as a single element in this
// array.
//
// Other devices:
//
// This array will contain as many elements as the number of
// radios.
//
    pub pdevs: [ath12k_pdev; MAX_RADIOS],
// struct ath12k_hw_params::single_pdev_only devices use this to
// store phy specific data
//
    pub fw_pdev: [ath12k_fw_pdev; MAX_RADIOS],
    pub fw_pdev_count: u8,
    pub pdevs_active: [*mut ath12k_pdev __rcu; MAX_RADIOS],
    pub hal_reg_cap: [ath12k_wmi_hal_reg_capabilities_ext_arg; MAX_RADIOS],
    pub free_vdev_map: c_ulonglong,
    pub free_vdev_stats_id_map: c_ulonglong,
    pub peer_mapping_wq: wait_queue_head_t,
    pub mac_addr: [u8; ETH_ALEN],
    pub wmi_ready: bool,
    pub wlan_init_status: u32,
    pub irq_num: [c_int; ATH12K_IRQ_NUM_MAX],
    pub ext_irq_grp: [ath12k_ext_irq_grp; ATH12K_EXT_IRQ_GRP_NUM_MAX],
    pub napi: *mut napi_struct,
    pub target_caps: ath12k_wmi_target_cap_arg,
    pub ext_service_bitmap: [u32; WMI_SERVICE_EXT_BM_SIZE],
    pub pdevs_macaddr_valid: bool,
    pub hw_params: *const ath12k_hw_params,
    pub cal_file: *const firmware,
// Below regd's are protected by ab->data_lock
// This is the regd set for every radio
// by the firmware during initialization
//
    pub default_regd: [*mut ieee80211_regdomain; MAX_RADIOS],
// This regd is set during dynamic country setting
// This may or may not be used during the runtime
//
    pub new_regd: [*mut ieee80211_regdomain; MAX_RADIOS],
    pub reg_info: [*mut ath12k_reg_info; MAX_RADIOS],
// Current DFS Regulatory
    pub dfs_region: ath12k_dfs_region,

    pub debugfs_soc: *mut dentry,

    pub dev_flags: c_ulong,
    pub driver_recovery: completion,
    pub workqueue: *mut workqueue_struct,
    pub restart_work: work_struct,
    pub workqueue_aux: *mut workqueue_struct,
    pub reset_work: work_struct,
    pub reset_count: core::sync::atomic::AtomicI32,
    pub recovery_count: core::sync::atomic::AtomicI32,
    pub is_reset: bool,
    pub reset_complete: completion,
// continuous recovery fail count
    pub fail_cont_count: core::sync::atomic::AtomicI32,
    pub reset_fail_timeout: c_ulong,
    pub update_11d_work: work_struct,
    pub new_alpha2: [u8; 2],
// protected by data_lock
    pub fw_crash_counter: u32,
    pub stats: },
    pub pktlog_defs_checksum: u32,
    pub db_caps: *mut ath12k_dbring_cap,
    pub num_db_cap: u32,
    pub htc_suspend: completion,
    pub fw_soc_drop_count: u64,
    pub static_window_map: bool,
    pub rfkill_work: work_struct,
// true means radio is on
    pub rfkill_radio_on: bool,
    pub bdf_search: ath12k_bdf_search,
    pub vendor: u32,
    pub device: u32,
    pub subsystem_vendor: u32,
    pub subsystem_device: u32,
    pub id: },
    pub api_version: u32,
    pub fw: *const firmware,
    pub amss_data: *const u8,
    pub amss_len: usize,
    pub amss_dualmac_data: *const u8,
    pub amss_dualmac_len: usize,
    pub m3_data: *const u8,
    pub m3_len: usize,
    pub aux_uc_data: *const u8,
    pub aux_uc_len: usize,
    pub ATH12K_FW_FEATURE_COUNT): DECLARE_BITMAP(fw_features,,
    pub fw_features_valid: bool,
    pub fw: },
    pub restart_completed: completion,

    pub started: bool,
    pub func_bit: u32,
    pub acpi_tas_enable: bool,
    pub acpi_bios_sar_enable: bool,
    pub acpi_disable_11be: bool,
    pub acpi_disable_rfkill: bool,
    pub acpi_cca_enable: bool,
    pub acpi_band_edge_enable: bool,
    pub acpi_enable_bdf: bool,
    pub bit_flag: u32,
    pub bdf_string: [c_char; ATH12K_ACPI_BDF_MAX_LEN],
    pub tas_cfg: [u8; ATH12K_ACPI_DSM_TAS_CFG_SIZE],
    pub tas_sar_power_table: [u8; ATH12K_ACPI_DSM_TAS_DATA_SIZE],
    pub bios_sar_data: [u8; ATH12K_ACPI_DSM_BIOS_SAR_DATA_SIZE],
    pub geo_offset_data: [u8; ATH12K_ACPI_DSM_GEO_OFFSET_DATA_SIZE],
    pub cca_data: [u8; ATH12K_ACPI_DSM_CCA_DATA_SIZE],
    pub band_edge_power: [u8; ATH12K_ACPI_DSM_BAND_EDGE_DATA_SIZE],
    pub acpi: },

    pub panic_nb: notifier_block,
    pub ag: *mut ath12k_hw_group,
    pub wsi_info: ath12k_wsi_info,
    pub fw_mode: ath12k_qmi_firmware_mode,
    pub ftm_event_obj: ath12k_ftm_event_obj,
    pub hw_group_ref: bool,
// Denote whether MLO is possible within the device
    pub single_chip_mlo_support: bool,
    pub reg_freq_2ghz: ath12k_reg_freq,
    pub reg_freq_5ghz: ath12k_reg_freq,
    pub reg_freq_6ghz: ath12k_reg_freq,
    pub profile_param: *const ath12k_mem_profile_based_param,
    pub target_mem_mode: ath12k_qmi_mem_mode,
// FIXME: Define this field in a ag equivalent object available
// during the initial phase of probe later.
//
    pub ath12k_ops: *const ieee80211_ops,
    pub rhead_sta_addr: *mut rhashtable,
    pub rhash_sta_addr_param: rhashtable_params,
// must be last
    pub )): *mut u8 drv_priv[] __aligned(sizeof(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pdev_map {
    pub ab: *mut ath12k_base,
    pub pdev_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_fw_stats_vdev {
    pub list: list_head,
    pub vdev_id: u32,
    pub beacon_snr: u32,
    pub data_snr: u32,
    pub num_tx_frames: [u32; WLAN_MAX_AC],
    pub num_rx_frames: u32,
    pub num_tx_frames_retries: [u32; WLAN_MAX_AC],
    pub num_tx_frames_failures: [u32; WLAN_MAX_AC],
    pub num_rts_fail: u32,
    pub num_rts_success: u32,
    pub num_rx_err: u32,
    pub num_rx_discard: u32,
    pub num_tx_not_acked: u32,
    pub tx_rate_history: [u32; MAX_TX_RATE_VALUES],
    pub beacon_rssi_history: [u32; MAX_TX_RATE_VALUES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_fw_stats_bcn {
    pub list: list_head,
    pub vdev_id: u32,
    pub tx_bcn_succ_cnt: u32,
    pub tx_bcn_outage_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_fw_stats_pdev {
    pub list: list_head,
// PDEV stats
    pub ch_noise_floor: i32,
    pub tx_frame_count: u32,
    pub rx_frame_count: u32,
    pub rx_clear_count: u32,
    pub cycle_count: u32,
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
    pub tx_abort: i32,
    pub mpdus_requed: i32,
    pub tx_ko: u32,
    pub data_rc: u32,
    pub self_triggers: u32,
    pub sw_retry_failure: u32,
    pub illgl_rate_phy_err: u32,
    pub pdev_cont_xretry: u32,
    pub pdev_tx_timeout: u32,
    pub pdev_resets: u32,
    pub stateless_tid_alloc_failure: u32,
    pub phy_underrun: u32,
    pub txop_ovf: u32,
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
}

extern "C" {
    pub fn ath12k_core_qmi_firmware_ready(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_hw_group_cleanup(ag: *mut ath12k_hw_group);
}
extern "C" {
    pub fn ath12k_core_pre_init(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_init(ath12k: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_deinit(ath12k: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_core_free(ath12k: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_core_free_bdf(ab: *mut ath12k_base, bd: *mut ath12k_board_data);
}
extern "C" {
    pub fn ath12k_core_fetch_regdb(ab: *mut ath12k_base, bd: *mut ath12k_board_data) -> c_int;
}
extern "C" {
    pub fn ath12k_core_check_dt(ath12k: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_check_smbios(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_halt(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_core_resume_early(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_resume(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_suspend(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_suspend_late(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_core_hw_group_unassign(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_get_num_partner_link(ar: *mut ath12k) -> u8;
}
extern "C" {
    pub fn ath12k_core_get_max_station_per_radio(ab: *mut ath12k_base) -> u32;
}
extern "C" {
    pub fn ath12k_core_get_max_peers_per_radio(ab: *mut ath12k_base) -> u32;
}
extern "C" {
    pub fn ath12k_core_hw_group_set_mlo_capable(ag: *mut ath12k_hw_group);
}
extern "C" {
    pub fn ath12k_fw_stats_init(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_fw_stats_bcn_free(head: *mut list_head);
}
extern "C" {
    pub fn ath12k_fw_stats_free(stats: *mut ath12k_fw_stats);
}
extern "C" {
    pub fn ath12k_fw_stats_reset(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_core_get_memory_mode(ab: *mut ath12k_base) -> ath12k_qmi_mem_mode;
}
extern "C" {
    pub fn container_of()ahsta: *mut (void, ieee80211_sta: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn container_of()ahvif: *mut (void, ieee80211_vif: struct, _arg: drv_priv) -> return;
}

// The @ab->dp NULL check or assertion is intentionally omitted because
// @ab->dp is guaranteed to be non-NULL after a successful probe and
// remains valid until teardown. Invoking this before allocation or
// after teardown is considered invalid usage.
//
extern "C" {
    pub fn container_of(_arg: dp, ath12k: struct, _arg: dp) -> return;
}
