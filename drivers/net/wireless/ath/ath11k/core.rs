//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/core.h
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
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH11K_TX_MGMT_NUM_PENDING_MAX: c_int = 512;
pub const ATH11K_TX_MGMT_TARGET_MAX_SUPPORT_WMI: c_int = 64;
// Pending management packets threshold for dropping probe responses

pub const ATH11K_INVALID_HW_MAC_ID: c_uint = 0xFF;

// SMBIOS type containing Board Data File Name Extension
pub const ATH11K_SMBIOS_BDF_EXT_TYPE: c_uint = 0xF8;
// SMBIOS type structure length (excluding strings-set)
pub const ATH11K_SMBIOS_BDF_EXT_LENGTH: c_uint = 0x9;
// The magic used by QCA spec

pub const ATH11K_MON_TIMER_INTERVAL: c_int = 10;

pub const ATH11K_RESET_MAX_FAIL_COUNT_FIRST: c_int = 3;
pub const ATH11K_RESET_MAX_FAIL_COUNT_FINAL: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_supported_bw {
    ATH11K_BW_20	= 0,
    ATH11K_BW_40	= 1,
    ATH11K_BW_80	= 2,
    ATH11K_BW_160	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_bdf_search {
    ATH11K_BDF_SEARCH_DEFAULT,
    ATH11K_BDF_SEARCH_BUS_AND_BOARD,
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

pub const ATH11K_HT_MCS_MAX: c_int = 7;
pub const ATH11K_VHT_MCS_MAX: c_int = 9;
pub const ATH11K_HE_MCS_MAX: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_crypt_mode {
// Only use hardware crypto engine
    ATH11K_CRYPT_MODE_HW,
// Only use software crypto
    ATH11K_CRYPT_MODE_SW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_skb_flags {
    ATH11K_SKB_HW_80211_ENCAP = BIT(0),
    ATH11K_SKB_CIPHER_SET = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_skb_cb {
    pub paddr: dma_addr_t,
    pub eid: u8,
    pub flags: u8,
    pub cipher: u32,
    pub ar: *mut ath11k,
    pub vif: *mut ieee80211_vif,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_skb_rxcb {
    pub paddr: dma_addr_t,
    pub is_first_msdu: bool,
    pub is_last_msdu: bool,
    pub is_continuation: bool,
    pub is_mcbc: bool,
    pub is_eapol: bool,
    pub rx_desc: *mut hal_rx_desc,
    pub err_rel_src: u8,
    pub err_code: u8,
    pub mac_id: u8,
    pub unmapped: u8,
    pub is_frag: u8,
    pub tid: u8,
    pub peer_id: u16,
    pub seq_no: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_hw_rev {
    ATH11K_HW_IPQ8074,
    ATH11K_HW_QCA6390_HW20,
    ATH11K_HW_IPQ6018_HW10,
    ATH11K_HW_QCN9074_HW10,
    ATH11K_HW_WCN6855_HW20,
    ATH11K_HW_WCN6855_HW21,
    ATH11K_HW_WCN6750_HW10,
    ATH11K_HW_IPQ5018_HW10,
    ATH11K_HW_QCA2066_HW21,
    ATH11K_HW_QCA6698AQ_HW21,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_firmware_mode {
// the default mode, standard 802.11 functionality
    ATH11K_FIRMWARE_MODE_NORMAL,

// factory tests etc
    ATH11K_FIRMWARE_MODE_FTM,

// Cold boot calibration
    ATH11K_FIRMWARE_MODE_COLD_BOOT = 7,
}

pub const ATH11K_IRQ_NUM_MAX: c_int = 52;
pub const ATH11K_EXT_IRQ_NUM_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_ext_irq_grp {
    pub ab: *mut ath11k_base,
    pub irqs: [u32; ATH11K_EXT_IRQ_NUM_MAX],
    pub num_irq: u32,
    pub grp_id: u32,
    pub timestamp: u64,
    pub napi_enabled: bool,
    pub napi: napi_struct,
    pub napi_ndev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_smbios_cc_type {
// disable country code setting from SMBIOS
    ATH11K_SMBIOS_CC_DISABLE = 0,

// set country code by ANSI country name, based on ISO3166-1 alpha2
    ATH11K_SMBIOS_CC_ISO = 1,

// worldwide regdomain
    ATH11K_SMBIOS_CC_WW = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_smbios_bdf {
    pub hdr: dmi_header,
    pub features_disabled: u8,
// enum ath11k_smbios_cc_type
    pub country_code_flag: u8,
// To set specific country, you need to set country code
// flag=ATH11K_SMBIOS_CC_ISO first, then if country is United
// States, then country code value = 0x5553 ("US",'U' = 0x55, 'S'=
// 0x53). To set country to INDONESIA, then country code value =
// 0x4944 ("IN", 'I'=0x49, 'D'=0x44). If country code flag =
// ATH11K_SMBIOS_CC_WW, then you can use worldwide regulatory
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
pub struct ath11k_he {
    pub hecap_macinfo: [u8; HECAP_MACINFO_SIZE],
    pub hecap_rxmcsnssmap: u32,
    pub hecap_txmcsnssmap: u32,
    pub hecap_phyinfo: [u32; HEHANDLE_CAP_PHYINFO_SIZE],
    pub hecap_ppet: he_ppe_threshold,
    pub heop_param: u32,
}

pub const MAX_RADIOS: c_int = 3;
// ipq5018 hw param macros
pub const MAX_RADIOS_5018: c_int = 1;
pub const CE_CNT_5018: c_int = 6;
pub const TARGET_CE_CNT_5018: c_int = 9;
pub const SVC_CE_MAP_LEN_5018: c_int = 17;
pub const RXDMA_PER_PDEV_5018: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_scan_state {
    ATH11K_SCAN_IDLE,
    ATH11K_SCAN_STARTING,
    ATH11K_SCAN_RUNNING,
    ATH11K_SCAN_ABORTING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_11d_state {
    ATH11K_11D_IDLE,
    ATH11K_11D_PREPARING,
    ATH11K_11D_RUNNING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_dev_flags {
    ATH11K_CAC_RUNNING,
    ATH11K_FLAG_CORE_REGISTERED,
    ATH11K_FLAG_CRASH_FLUSH,
    ATH11K_FLAG_RAW_MODE,
    ATH11K_FLAG_HW_CRYPTO_DISABLED,
    ATH11K_FLAG_BTCOEX,
    ATH11K_FLAG_RECOVERY,
    ATH11K_FLAG_UNREGISTERING,
    ATH11K_FLAG_REGISTERED,
    ATH11K_FLAG_QMI_FAIL,
    ATH11K_FLAG_HTC_SUSPEND_COMPLETE,
    ATH11K_FLAG_CE_IRQ_ENABLED,
    ATH11K_FLAG_EXT_IRQ_ENABLED,
    ATH11K_FLAG_FIXED_MEM_RGN,
    ATH11K_FLAG_DEVICE_INIT_DONE,
    ATH11K_FLAG_MULTI_MSI_VECTORS,
    ATH11K_FLAG_FTM_SEGMENTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_monitor_flags {
    ATH11K_FLAG_MONITOR_CONF_ENABLED,
    ATH11K_FLAG_MONITOR_STARTED,
    ATH11K_FLAG_MONITOR_VDEV_CREATED,
}

pub const ATH11K_IPV6_UC_TYPE: c_int = 0;
pub const ATH11K_IPV6_AC_TYPE: c_int = 1;
pub const ATH11K_IPV6_MAX_COUNT: c_int = 16;
pub const ATH11K_IPV4_MAX_COUNT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_arp_ns_offload {
    pub ipv4_addr: [u8; ATH11K_IPV4_MAX_COUNT][4],
    pub ipv4_count: u32,
    pub ipv6_count: u32,
    pub ipv6_addr: [u8; ATH11K_IPV6_MAX_COUNT][16],
    pub self_ipv6_addr: [u8; ATH11K_IPV6_MAX_COUNT][16],
    pub ipv6_type: [u8; ATH11K_IPV6_MAX_COUNT],
    pub ipv6_valid: [bool; ATH11K_IPV6_MAX_COUNT],
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_rekey_data {
    pub kck: [u8; NL80211_KCK_LEN],
    pub kek: [u8; NL80211_KCK_LEN],
    pub replay_ctr: u64,
    pub enable_offload: bool,
}

//
// struct ath11k_chan_power_info - TPE containing power info per channel chunk
// @chan_cfreq: channel center freq (MHz)
// e.g.
// channel 37/20 MHz,  it is 6135
// channel 37/40 MHz,  it is 6125
// channel 37/80 MHz,  it is 6145
// channel 37/160 MHz, it is 6185
// @tx_power: transmit power (dBm)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_chan_power_info {
    pub chan_cfreq: u16,
    pub tx_power: i8,
}

// ath11k only deals with 160 MHz, so 8 subchannels
pub const ATH11K_NUM_PWR_LEVELS: c_int = 8;
//
// struct ath11k_reg_tpc_power_info - regulatory TPC power info
// @is_psd_power: is PSD power or not
// @eirp_power: Maximum EIRP power (dBm), valid only if power is PSD
// @ap_power_type: type of power (SP/LPI/VLP)
// @num_pwr_levels: number of power levels
// @reg_max: Array of maximum TX power (dBm) per PSD value
// @tpe: TPE values processed from TPE IE
// @chan_power_info: power info to send to firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_reg_tpc_power_info {
    pub is_psd_power: bool,
    pub eirp_power: u8,
    pub ap_power_type: wmi_reg_6ghz_ap_type,
    pub num_pwr_levels: u8,
    pub reg_max: [u8; ATH11K_NUM_PWR_LEVELS],
    pub tpe: [i8; ATH11K_NUM_PWR_LEVELS],
    pub chan_power_info: [ath11k_chan_power_info; ATH11K_NUM_PWR_LEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_vif {
    pub vdev_id: u32,
    pub vdev_type: wmi_vdev_type,
    pub vdev_subtype: wmi_vdev_subtype,
    pub beacon_interval: u32,
    pub dtim_period: u32,
    pub ast_hash: u16,
    pub ast_idx: u16,
    pub tcl_metadata: u16,
    pub hal_addr_search_flags: u8,
    pub search_type: u8,
    pub ar: *mut ath11k,
    pub vif: *mut ieee80211_vif,
    pub wmm_params: wmi_wmm_params_all_arg,
    pub muedca_params: wmi_wmm_params_all_arg,
    pub list: list_head,
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
    pub is_started: bool,
    pub is_up: bool,
    pub ftm_responder: bool,
    pub spectral_enabled: bool,
    pub ps: bool,
    pub aid: u32,
    pub bssid: [u8; ETH_ALEN],
    pub bitrate_mask: cfg80211_bitrate_mask,
    pub connection_loss_work: delayed_work,
    pub bcn_tx_work: work_struct,
    pub num_legacy_stations: c_int,
    pub rtscts_prot_mode: c_int,
    pub txpower: c_int,
    pub rsnie_present: bool,
    pub wpaie_present: bool,
    pub bcca_zero_sent: bool,
    pub do_not_send_tmpl: bool,
    pub arp_ns_offload: ath11k_arp_ns_offload,
    pub rekey_data: ath11k_rekey_data,
    pub num_stations: u32,
    pub reinstall_group_keys: bool,
    pub reg_tpc_info: ath11k_reg_tpc_power_info,
// Must be last - ends in a flexible-array member.
//
// FIXME: Driver should not copy struct ieee80211_chanctx_conf,
// especially because it has a flexible array. Find a better way.
//
    pub chanctx: ieee80211_chanctx_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_vif_iter {
    pub vdev_id: u32,
    pub arvif: *mut ath11k_vif,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_rx_peer_stats {
    pub num_msdu: u64,
    pub num_mpdu_fcs_ok: u64,
    pub num_mpdu_fcs_err: u64,
    pub tcp_msdu_count: u64,
    pub udp_msdu_count: u64,
    pub other_msdu_count: u64,
    pub ampdu_msdu_count: u64,
    pub non_ampdu_msdu_count: u64,
    pub stbc_count: u64,
    pub beamformed_count: u64,
    pub 1]: u64 mcs_count[HAL_RX_MAX_MCS +,
    pub nss_count: [u64; HAL_RX_MAX_NSS],
    pub bw_count: [u64; HAL_RX_BW_MAX],
    pub gi_count: [u64; HAL_RX_GI_MAX],
    pub coding_count: [u64; HAL_RX_SU_MU_CODING_MAX],
    pub 1]: u64 tid_count[IEEE80211_NUM_TIDS +,
    pub pream_cnt: [u64; HAL_RX_PREAMBLE_MAX],
    pub reception_type: [u64; HAL_RX_RECEPTION_TYPE_MAX],
    pub rx_duration: u64,
    pub dcm_count: u64,
    pub ru_alloc_cnt: [u64; HAL_RX_RU_ALLOC_TYPE_MAX],
}

pub const ATH11K_HE_MCS_NUM: c_int = 12;
pub const ATH11K_VHT_MCS_NUM: c_int = 10;
pub const ATH11K_BW_NUM: c_int = 4;
pub const ATH11K_NSS_NUM: c_int = 4;
pub const ATH11K_LEGACY_NUM: c_int = 12;
pub const ATH11K_GI_NUM: c_int = 4;
pub const ATH11K_HT_MCS_NUM: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_pkt_rx_err {
    ATH11K_PKT_RX_ERR_FCS,
    ATH11K_PKT_RX_ERR_TKIP,
    ATH11K_PKT_RX_ERR_CRYPT,
    ATH11K_PKT_RX_ERR_PEER_IDX_INVAL,
    ATH11K_PKT_RX_ERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_ampdu_subfrm_num {
    ATH11K_AMPDU_SUBFRM_NUM_10,
    ATH11K_AMPDU_SUBFRM_NUM_20,
    ATH11K_AMPDU_SUBFRM_NUM_30,
    ATH11K_AMPDU_SUBFRM_NUM_40,
    ATH11K_AMPDU_SUBFRM_NUM_50,
    ATH11K_AMPDU_SUBFRM_NUM_60,
    ATH11K_AMPDU_SUBFRM_NUM_MORE,
    ATH11K_AMPDU_SUBFRM_NUM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_amsdu_subfrm_num {
    ATH11K_AMSDU_SUBFRM_NUM_1,
    ATH11K_AMSDU_SUBFRM_NUM_2,
    ATH11K_AMSDU_SUBFRM_NUM_3,
    ATH11K_AMSDU_SUBFRM_NUM_4,
    ATH11K_AMSDU_SUBFRM_NUM_MORE,
    ATH11K_AMSDU_SUBFRM_NUM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_counter_type {
    ATH11K_COUNTER_TYPE_BYTES,
    ATH11K_COUNTER_TYPE_PKTS,
    ATH11K_COUNTER_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_stats_type {
    ATH11K_STATS_TYPE_SUCC,
    ATH11K_STATS_TYPE_FAIL,
    ATH11K_STATS_TYPE_RETRY,
    ATH11K_STATS_TYPE_AMPDU,
    ATH11K_STATS_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htt_data_stats {
    pub legacy: [u64; ATH11K_COUNTER_TYPE_MAX][ATH11K_LEGACY_NUM],
    pub ht: [u64; ATH11K_COUNTER_TYPE_MAX][ATH11K_HT_MCS_NUM],
    pub vht: [u64; ATH11K_COUNTER_TYPE_MAX][ATH11K_VHT_MCS_NUM],
    pub he: [u64; ATH11K_COUNTER_TYPE_MAX][ATH11K_HE_MCS_NUM],
    pub bw: [u64; ATH11K_COUNTER_TYPE_MAX][ATH11K_BW_NUM],
    pub nss: [u64; ATH11K_COUNTER_TYPE_MAX][ATH11K_NSS_NUM],
    pub gi: [u64; ATH11K_COUNTER_TYPE_MAX][ATH11K_GI_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htt_tx_stats {
    pub stats: [ath11k_htt_data_stats; ATH11K_STATS_TYPE_MAX],
    pub tx_duration: u64,
    pub ba_fails: u64,
    pub ack_fails: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_per_ppdu_tx_stats {
    pub succ_pkts: u16,
    pub failed_pkts: u16,
    pub retry_pkts: u16,
    pub succ_bytes: u32,
    pub failed_bytes: u32,
    pub retry_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_per_peer_cfr_capture {
    pub cfr_method: ath11k_cfr_capture_method,
    pub cfr_bw: ath11k_cfr_capture_bw,
    pub cfr_enable: u32,
    pub cfr_period: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_sta {
    pub arvif: *mut ath11k_vif,
// the following are protected by ar->data_lock
    pub /: *mut *mut *mut u32 changed; / IEEE80211_RC_,
    pub bw: u32,
    pub nss: u32,
    pub smps: u32,
    pub pn_type: hal_pn_type,
    pub update_wk: work_struct,
    pub set_4addr_wk: work_struct,
    pub txrate: rate_info,
    pub peer_nss: u32,
    pub last_txrate: rate_info,
    pub rx_duration: u64,
    pub tx_duration: u64,
    pub rssi_comb: u8,
    pub avg_rssi: ewma_avg_rssi,
    pub rssi_beacon: i8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
    pub tx_stats: *mut ath11k_htt_tx_stats,
    pub rx_stats: *mut ath11k_rx_peer_stats,

// protected by conf_mutex
    pub aggr_mode: bool,

    pub use_4addr_set: bool,
    pub tcl_metadata: u16,
// Protected with ar->data_lock
    pub peer_ps_state: ath11k_wmi_peer_ps_state,
    pub ps_start_time: u64,
    pub ps_start_jiffies: u64,
    pub ps_total_duration: u64,
    pub peer_current_ps_valid: bool,
    pub bw_prev: u32,

    pub cfr_capture: ath11k_per_peer_cfr_capture,

}

pub const ATH11K_MIN_5G_FREQ: c_int = 4150;
pub const ATH11K_MIN_6G_FREQ: c_int = 5925;
pub const ATH11K_MAX_6G_FREQ: c_int = 7115;
pub const ATH11K_NUM_CHANS: c_int = 102;
pub const ATH11K_MAX_5G_CHAN: c_int = 177;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_state {
    ATH11K_STATE_OFF,
    ATH11K_STATE_ON,
    ATH11K_STATE_RESTARTING,
    ATH11K_STATE_RESTARTED,
    ATH11K_STATE_WEDGED,
    ATH11K_STATE_FTM,
// Add other states as required
}

// Antenna noise floor

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_fw_stats {
    pub debugfs_fwstats: *mut dentry,
    pub pdev_id: u32,
    pub stats_id: u32,
    pub pdevs: list_head,
    pub vdevs: list_head,
    pub bcn: list_head,
    pub num_vdev_recvd: u32,
    pub num_bcn_recvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbg_htt_stats {
    pub type: u8,
    pub reset: u8,
    pub stats_req: *mut debug_htt_stats_req,
// protects shared stats req buffer
    pub lock: spinlock_t,
}

pub const MAX_MODULE_ID_BITMAP_WORDS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_debug {
    pub debugfs_pdev: *mut dentry,
    pub htt_stats: ath11k_dbg_htt_stats,
    pub extd_tx_stats: u32,
    pub extd_rx_stats: u32,
    pub pktlog_filter: u32,
    pub pktlog_mode: u32,
    pub pktlog_peer_valid: u32,
    pub pktlog_peer_addr: [u8; ETH_ALEN],
    pub rx_filter: u32,
    pub mem_offset: u32,
    pub module_id_bitmap: [u32; MAX_MODULE_ID_BITMAP_WORDS],
    pub dbr_debug: [*mut ath11k_debug_dbr; WMI_DIRECT_BUF_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_per_peer_tx_stats {
    pub succ_bytes: u32,
    pub retry_bytes: u32,
    pub failed_bytes: u32,
    pub succ_pkts: u16,
    pub retry_pkts: u16,
    pub failed_pkts: u16,
    pub duration: u32,
    pub ba_fails: u8,
    pub is_ampdu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k {
    pub ab: *mut ath11k_base,
    pub pdev: *mut ath11k_pdev,
    pub hw: *mut ieee80211_hw,
    pub wmi: *mut ath11k_pdev_wmi,
    pub dp: ath11k_pdev_dp,
    pub mac_addr: [u8; ETH_ALEN],
    pub ar_he: ath11k_he,
    pub state: ath11k_state,
    pub supports_6ghz: bool,
    pub started: completion,
    pub completed: completion,
    pub on_channel: completion,
    pub timeout: delayed_work,
    pub state: ath11k_scan_state,
    pub is_roc: bool,
    pub vdev_id: c_int,
    pub roc_freq: c_int,
    pub roc_notify: bool,
    pub scan: },
    pub sbands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub mac: },
    pub dev_flags: c_ulong,
    pub filter_flags: c_uint,
    pub monitor_flags: c_ulong,
    pub min_tx_power: u32,
    pub max_tx_power: u32,
    pub txpower_limit_2g: u32,
    pub txpower_limit_5g: u32,
    pub txpower_scale: u32,
    pub power_scale: u32,
    pub chan_tx_pwr: u32,
    pub num_stations: u32,
    pub max_num_stations: u32,
// To synchronize concurrent synchronous mac80211 callback operations,
// concurrent debugfs configuration and concurrent FW statistics events.
//
    pub conf_mutex: mutex,
// protects the radio specific data like debug stats, ppdu_stats_info stats,
// vdev_stop_status info, scan data, ath11k_sta info, ath11k_vif info,
// channel context data, survey info, test mode data, channel_update_queue.
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
    pub peer_assoc_done: completion,
    pub peer_delete_done: completion,
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
    pub survey: [survey_info; ATH11K_NUM_CHANS],
    pub bss_survey_done: completion,
    pub regd_update_work: work_struct,
    pub channel_update_work: work_struct,
// protected with data_lock
    pub channel_update_queue: list_head,
    pub wmi_mgmt_tx_work: work_struct,
    pub wmi_mgmt_tx_queue: sk_buff_head,
    pub wow: ath11k_wow,
    pub target_suspend: completion,
    pub target_suspend_ack: bool,
    pub peer_tx_stats: ath11k_per_peer_tx_stats,
    pub ppdu_stats_info: list_head,
    pub ppdu_stat_list_depth: u32,
    pub cached_stats: ath11k_per_peer_tx_stats,
    pub last_ppdu_id: u32,
    pub cached_ppdu_id: u32,
    pub monitor_vdev_id: c_int,
    pub fw_mode_reset: completion,
    pub ftm_msgref: u8,

    pub debug: ath11k_debug,

    pub spectral: ath11k_spectral,

    pub dfs_block_radar_events: bool,
    pub thermal: ath11k_thermal,
    pub vdev_id_11d_scan: u32,
    pub completed_11d_scan: completion,
    pub state_11d: ath11k_11d_state,
    pub regdom_set_by_user: bool,
    pub hw_rate_code: c_int,
    pub twt_enabled: u8,
    pub nlo_enabled: bool,
    pub 1]: u8 alpha2[REG_ALPHA2_LEN +,
    pub fw_stats: ath11k_fw_stats,
    pub fw_stats_complete: completion,
    pub fw_stats_done: completion,
// protected by conf_mutex
    pub ps_state_enable: bool,
    pub ps_timekeeper_enable: bool,
    pub max_allowed_tx_power: i8,

    pub cfr: ath11k_cfr,

    pub cfr_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_band_cap {
    pub phy_id: u32,
    pub max_bw_supported: u32,
    pub ht_cap_info: u32,
    pub he_cap_info: [u32; 2],
    pub he_mcs: u32,
    pub he_cap_phy_info: [u32; PSOC_HOST_MAX_PHY_SIZE],
    pub he_ppet: ath11k_ppe_threshold,
    pub he_6ghz_capa: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_pdev_cap {
    pub supported_bands: u32,
    pub ampdu_density: u32,
    pub vht_cap: u32,
    pub vht_mcs: u32,
    pub he_mcs: u32,
    pub tx_chain_mask: u32,
    pub rx_chain_mask: u32,
    pub tx_chain_mask_shift: u32,
    pub rx_chain_mask_shift: u32,
    pub band: [ath11k_band_cap; NUM_NL80211_BANDS],
    pub nss_ratio_enabled: bool,
    pub nss_ratio_info: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_pdev {
    pub ar: *mut ath11k,
    pub pdev_id: u32,
    pub cap: ath11k_pdev_cap,
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_board_data {
    pub fw: *const firmware,
    pub data: *const c_void,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_pci_ops {
    pub ab): *mut *mut int (wakeup)(struct ath11k_base,
    pub ab): *mut *mut void (release)(struct ath11k_base,
    pub vector): *mut *mut *mut int (get_msi_irq)(struct ath11k_base ab, unsigned int,
    pub value): *mut *mut *mut void (window_write32)(struct ath11k_base ab, u32 offset, u32,
    pub offset): *mut *mut *mut u32 (window_read32)(struct ath11k_base ab, u32,
}

// IPQ8074 HW channel counters frequency value in hertz
pub const IPQ8074_CC_FREQ_HERTZ: c_int = 320000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_bp_stats {
// Head Pointer reported by the last HTT Backpressure event for the ring
    pub hp: u16,
// Tail Pointer reported by the last HTT Backpressure event for the ring
    pub tp: u16,
// Number of Backpressure events received for the ring
    pub count: u32,
// Last recorded event timestamp
    pub jiffies: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dp_ring_bp_stats {
    pub umac_ring_bp_stats: [ath11k_bp_stats; HTT_SW_UMAC_RING_IDX_MAX],
    pub lmac_ring_bp_stats: [ath11k_bp_stats; HTT_SW_LMAC_RING_IDX_MAX][MAX_RADIOS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_soc_dp_tx_err_stats {
// TCL Ring Descriptor unavailable
    pub desc_na: [u32; DP_TCL_NUM_RING_MAX],
// Other failures during dp_tx due to mem allocation failure
// idr unavailable etc.
//
    pub misc_fail: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_soc_dp_stats {
    pub err_ring_pkts: u32,
    pub invalid_rbm: u32,
    pub rxdma_error: [u32; HAL_REO_ENTR_RING_RXDMA_ECODE_MAX],
    pub reo_error: [u32; HAL_REO_DEST_RING_ERROR_CODE_MAX],
    pub hal_reo_error: [u32; DP_REO_DST_RING_MAX],
    pub tx_err: ath11k_soc_dp_tx_err_stats,
    pub bp_stats: ath11k_dp_ring_bp_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_msi_user {
    pub name: *mut c_char,
    pub num_vectors: c_int,
    pub base_vector: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_msi_config {
    pub total_vectors: c_int,
    pub total_users: c_int,
    pub users: *mut ath11k_msi_user,
    pub hw_rev: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_pm_policy {
    ATH11K_PM_DEFAULT,
    ATH11K_PM_WOW,
}

// Master structure to hold the hw data which may be used in core module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_base {
    pub hw_rev: ath11k_hw_rev,
    pub fw_mode: ath11k_firmware_mode,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub qmi: ath11k_qmi,
    pub wmi_ab: ath11k_wmi_base,
    pub fw_ready: completion,
    pub num_radios: c_int,
// HW channel counters frequency value in hertz common to all MACs
    pub cc_freq_hz: u32,
    pub dump_data: *mut ath11k_dump_file_data,
    pub ath11k_coredump_len: usize,
    pub dump_work: work_struct,
    pub htc: ath11k_htc,
    pub dp: ath11k_dp,
    pub mem: *mut void __iomem,
    pub mem_ce: *mut void __iomem,
    pub mem_len: c_ulong,
    pub bus: ath11k_bus,
    pub ops: *const ath11k_hif_ops,
    pub hif: },
    pub wakeup_completed: completion,
    pub wow: },
    pub ce: ath11k_ce,
    pub rx_replenish_retry: timer_list,
    pub hal: ath11k_hal,
// To synchronize core_start/core_stop
    pub core_lock: mutex,
// Protects data like peers
    pub base_lock: spinlock_t,
    pub pdevs: [ath11k_pdev; MAX_RADIOS],
    pub supported_bands: WMI_HOST_WLAN_BAND,
    pub pdev_id: u32,
    pub target_pdev_ids: [}; MAX_RADIOS],
    pub target_pdev_count: u8,
    pub pdevs_active: [*mut ath11k_pdev __rcu; MAX_RADIOS],
    pub hal_reg_cap: [ath11k_hal_reg_capabilities_ext; MAX_RADIOS],
    pub free_vdev_map: c_ulonglong,
// To synchronize rhash tbl write operation
    pub tbl_mtx_lock: mutex,
// The rhashtable containing struct ath11k_peer keyed by mac addr
    pub rhead_peer_addr: *mut rhashtable,
    pub rhash_peer_addr_param: rhashtable_params,
// The rhashtable containing struct ath11k_peer keyed by id
    pub rhead_peer_id: *mut rhashtable,
    pub rhash_peer_id_param: rhashtable_params,
    pub peers: list_head,
    pub peer_mapping_wq: wait_queue_head_t,
    pub mac_addr: [u8; ETH_ALEN],
    pub irq_num: [c_int; ATH11K_IRQ_NUM_MAX],
    pub ext_irq_grp: [ath11k_ext_irq_grp; ATH11K_EXT_IRQ_GRP_NUM_MAX],
    pub target_caps: ath11k_targ_cap,
    pub ext_service_bitmap: [u32; WMI_SERVICE_EXT_BM_SIZE],
    pub pdevs_macaddr_valid: bool,
    pub hw_params: ath11k_hw_params,
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
    pub reg_info_store: *mut cur_regulatory_info,
// Current DFS Regulatory
    pub dfs_region: ath11k_dfs_region,

    pub debugfs_soc: *mut dentry,

    pub soc_stats: ath11k_soc_dp_stats,
    pub dev_flags: c_ulong,
    pub driver_recovery: completion,
    pub workqueue: *mut workqueue_struct,
    pub restart_work: work_struct,
    pub update_11d_work: work_struct,
    pub new_alpha2: [u8; 3],
    pub workqueue_aux: *mut workqueue_struct,
    pub reset_work: work_struct,
    pub reset_count: core::sync::atomic::AtomicI32,
    pub recovery_count: core::sync::atomic::AtomicI32,
    pub recovery_start_count: core::sync::atomic::AtomicI32,
    pub is_reset: bool,
    pub reset_complete: completion,
    pub reconfigure_complete: completion,
    pub recovery_start: completion,
// continuous recovery fail count
    pub fail_cont_count: core::sync::atomic::AtomicI32,
    pub reset_fail_timeout: c_ulong,
// protected by data_lock
    pub fw_crash_counter: u32,
    pub stats: },
    pub pktlog_defs_checksum: u32,
    pub db_caps: *mut ath11k_dbring_cap,
    pub num_db_cap: u32,
// To synchronize 11d scan vdev id
    pub vdev_id_11d_lock: mutex,
    pub mon_reap_timer: timer_list,
    pub htc_suspend: completion,
    pub bdf_search: ath11k_bdf_search,
    pub vendor: u32,
    pub device: u32,
    pub subsystem_vendor: u32,
    pub subsystem_device: u32,
    pub id: },
    pub config: *const ath11k_msi_config,
    pub ep_base_data: u32,
    pub irqs: [u32; 32],
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub msi: },
    pub ops: *const ath11k_pci_ops,
    pub pci: },
    pub api_version: u32,
    pub fw: *const firmware,
    pub amss_data: *const u8,
    pub amss_len: usize,
    pub m3_data: *const u8,
    pub m3_len: usize,
    pub ATH11K_FW_FEATURE_COUNT): DECLARE_BITMAP(fw_features,,
    pub fw: },
    pub restart_completed: completion,

    pub data_pos: u32,
    pub expected_seq: u32,
    pub eventdata: *mut u8,
    pub testmode: },

    pub pm_policy: ath11k_pm_policy,
    pub actual_pm_policy: ath11k_pm_policy,
    pub pm_nb: notifier_block,
// must be last
    pub )): *mut u8 drv_priv[] __aligned(sizeof(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_fw_stats_pdev {
    pub list: list_head,
// PDEV stats
    pub ch_noise_floor: i32,
// Cycles spent transmitting frames
    pub tx_frame_count: u32,
// Cycles spent receiving frames
    pub rx_frame_count: u32,
// Total channel busy time, evidently
    pub rx_clear_count: u32,
// Total on-channel time
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
// Num HTT cookies queued to dispatch list
    pub comp_queued: i32,
// Num HTT cookies dispatched
    pub comp_delivered: i32,
// Num MSDU queued to WAL
    pub msdu_enqued: i32,
// Num MPDU queue to WAL
    pub mpdu_enqued: i32,
// Num MSDUs dropped by WMM limit
    pub wmm_drop: i32,
// Num Local frames queued
    pub local_enqued: i32,
// Num Local frames done
    pub local_freed: i32,
// Num queued to HW
    pub hw_queued: i32,
// Num PPDU reaped from HW
    pub hw_reaped: i32,
// Num underruns
    pub underrun: i32,
// Num hw paused
    pub hw_paused: u32,
// Num PPDUs cleaned up in TX abort
    pub tx_abort: i32,
// Num MPDUs requeued by SW
    pub mpdus_requeued: i32,
// excessive retries
    pub tx_ko: u32,
    pub tx_xretry: u32,
// data hw rate code
    pub data_rc: u32,
// Scheduler self triggers
    pub self_triggers: u32,
// frames dropped due to excessive sw retries
    pub sw_retry_failure: u32,
// illegal rate phy errors
    pub illgl_rate_phy_err: u32,
// wal pdev continuous xretry
    pub pdev_cont_xretry: u32,
// wal pdev tx timeouts
    pub pdev_tx_timeout: u32,
// wal pdev resets
    pub pdev_resets: u32,
// frames dropped due to non-availability of stateless TIDs
    pub stateless_tid_alloc_failure: u32,
// PhY/BB underrun
    pub phy_underrun: u32,
// MPDU is more than txop limit
    pub txop_ovf: u32,
// Num sequences posted
    pub seq_posted: u32,
// Num sequences failed in queueing
    pub seq_failed_queueing: u32,
// Num sequences completed
    pub seq_completed: u32,
// Num sequences restarted
    pub seq_restarted: u32,
// Num of MU sequences posted
    pub mu_seq_posted: u32,
// Num MPDUs flushed by SW, HWPAUSED, SW TXABORT
// (Reset,channel change)
//
    pub mpdus_sw_flush: i32,
// Num MPDUs filtered by HW, all filter condition (TTL expired)
    pub mpdus_hw_filter: i32,
// Num MPDUs truncated by PDG (TXOP, TBTT,
// PPDU_duration based on rate, dyn_bw)
//
    pub mpdus_truncated: i32,
// Num MPDUs that was tried but didn't receive ACK or BA
    pub mpdus_ack_failed: i32,
// Num MPDUs that was dropped du to expiry.
    pub mpdus_expired: i32,
// PDEV RX stats
// Cnts any change in ring routing mid-ppdu
    pub mid_ppdu_route_change: i32,
// Total number of statuses processed
    pub status_rcvd: i32,
// Extra frags on rings 0-3
    pub r0_frags: i32,
    pub r1_frags: i32,
    pub r2_frags: i32,
    pub r3_frags: i32,
// MSDUs / MPDUs delivered to HTT
    pub htt_msdus: i32,
    pub htt_mpdus: i32,
// MSDUs / MPDUs delivered to local stack
    pub loc_msdus: i32,
    pub loc_mpdus: i32,
// AMSDUs that have more MSDUs than the status ring size
    pub oversize_amsdu: i32,
// Number of PHY errors
    pub phy_errs: i32,
// Number of PHY errors drops
    pub phy_err_drop: i32,
// Number of mpdu errors - FCS, MIC, ENC etc.
    pub mpdu_errs: i32,
// Num overflow errors
    pub rx_ovfl_errs: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_fw_stats_vdev {
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
pub struct ath11k_fw_stats_bcn {
    pub list: list_head,
    pub vdev_id: u32,
    pub tx_bcn_succ_cnt: u32,
    pub tx_bcn_outage_cnt: u32,
}

extern "C" {
    pub fn ath11k_fw_stats_init(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_fw_stats_pdevs_free(head: *mut list_head);
}
extern "C" {
    pub fn ath11k_fw_stats_vdevs_free(head: *mut list_head);
}
extern "C" {
    pub fn ath11k_fw_stats_bcn_free(head: *mut list_head);
}
extern "C" {
    pub fn ath11k_fw_stats_free(stats: *mut ath11k_fw_stats);
}
extern "C" {
    pub fn ath11k_core_qmi_firmware_ready(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_pre_init(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_init(ath11k: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_deinit(ath11k: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_core_free(ath11k: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_core_fetch_regdb(ab: *mut ath11k_base, bd: *mut ath11k_board_data) -> c_int;
}
extern "C" {
    pub fn ath11k_core_free_bdf(ab: *mut ath11k_base, bd: *mut ath11k_board_data);
}
extern "C" {
    pub fn ath11k_core_check_dt(ath11k: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_check_smbios(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_halt(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_core_resume_early(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_resume(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_suspend(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_suspend_late(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_core_pre_reconfigure_recovery(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_core_coldboot_cal_support(ab: *mut ath11k_base) -> bool;
}
extern "C" {
    pub fn ath11k_core_pm_notifier_unregister(ab: *mut ath11k_base);
}
