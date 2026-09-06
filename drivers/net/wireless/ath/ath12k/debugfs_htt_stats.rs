//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/debugfs_htt_stats.h
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

pub const ATH12K_HTT_STATS_MAGIC_VALUE: c_uint = 0xF0F0F0F0;
pub const ATH12K_HTT_STATS_SUBTYPE_MAX: c_int = 16;
pub const ATH12K_HTT_MAX_STRING_LEN: c_int = 256;

extern "C" {
    pub fn ath12k_debugfs_htt_stats_register(ar: *mut ath12k);
}

//
// DOC: target -> host extended statistics upload
//
// The following field definitions describe the format of the HTT
// target to host stats upload confirmation message.
// The message contains a cookie echoed from the HTT host->target stats
// upload request, which identifies which request the confirmation is
// for, and a single stats can span over multiple HTT stats indication
// due to the HTT message size limitation so every HTT ext stats
// indication will have tag-length-value stats information elements.
// The tag-length header for each HTT stats IND message also includes a
// status field, to indicate whether the request for the stat type in
// question was fully met, partially met, unable to be met, or invalid
// (if the stat type in question is disabled in the target).
// A Done bit 1's indicate the end of the of stats info elements.
//
// |31                         16|15    12|11|10 8|7   5|4       0|
// |--------------------------------------------------------------|
// |                   reserved                   |    msg type   |
// |--------------------------------------------------------------|
// |                         cookie LSBs                          |
// |--------------------------------------------------------------|
// |                         cookie MSBs                          |
// |--------------------------------------------------------------|
// |      stats entry length     | rsvd   | D|  S |   stat type   |
// |--------------------------------------------------------------|
// |                   type-specific stats info                   |
// |                      (see debugfs_htt_stats.h)               |
// |--------------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: Identifies this is a extended statistics upload confirmation
// message.
// Value: 0x1c
// - COOKIE_LSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: MSBs of the opaque cookie specified by the host-side requestor
// - COOKIE_MSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: MSBs of the opaque cookie specified by the host-side requestor
//
// Stats Information Element tag-length header fields:
// - STAT_TYPE
// Bits 7:0
// Purpose: identifies the type of statistics info held in the
// following information element
// Value: ath12k_dbg_htt_ext_stats_type
// - STATUS
// Bits 10:8
// Purpose: indicate whether the requested stats are present
// Value:
// 0 -> The requested stats have been delivered in full
// 1 -> The requested stats have been delivered in part
// 2 -> The requested stats could not be delivered (error case)
// 3 -> The requested stat type is either not recognized (invalid)
// - DONE
// Bits 11
// Purpose:
// Indicates the completion of the stats entry, this will be the last
// stats conf HTT segment for the requested stats type.
// Value:
// 0 -> the stats retrieval is ongoing
// 1 -> the stats retrieval is complete
// - LENGTH
// Bits 31:16
// Purpose: indicate the stats information size
// Value: This field specifies the number of bytes of stats information
// that follows the element tag-length header.
// It is expected but not required that this length is a multiple of
// 4 bytes.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_extd_stats_msg {
    pub info0: __le32,
    pub cookie: __le64,
    pub info1: __le32,
    pub data: [u8; ],
    pub __packed: },
// htt_dbg_ext_stats_type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_dbg_htt_ext_stats_type {
    ATH12K_DBG_HTT_EXT_STATS_RESET				= 0,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_TX			= 1,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_RX			= 2,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_TX_HWQ			= 3,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_TX_SCHED			= 4,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_ERROR			= 5,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_TQM			= 6,
    ATH12K_DBG_HTT_EXT_STATS_TX_DE_INFO			= 8,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_TX_RATE			= 9,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_RX_RATE			= 10,
    ATH12K_DBG_HTT_EXT_STATS_TX_SELFGEN_INFO		= 12,
    ATH12K_DBG_HTT_EXT_STATS_SRNG_INFO			= 15,
    ATH12K_DBG_HTT_EXT_STATS_SFM_INFO			= 16,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_TX_MU			= 17,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_CCA_STATS			= 19,
    ATH12K_DBG_HTT_EXT_STATS_TX_SOUNDING_INFO		= 22,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_OBSS_PD_STATS		= 23,
    ATH12K_DBG_HTT_EXT_STATS_LATENCY_PROF_STATS		= 25,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_UL_TRIG_STATS		= 26,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_UL_MUMIMO_TRIG_STATS	= 27,
    ATH12K_DBG_HTT_EXT_STATS_FSE_RX				= 28,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_RX_RATE_EXT		= 30,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_TX_RATE_TXBF		= 31,
    ATH12K_DBG_HTT_EXT_STATS_TXBF_OFDMA			= 32,
    ATH12K_DBG_HTT_EXT_STATS_DLPAGER_STATS			= 36,
    ATH12K_DBG_HTT_EXT_PHY_COUNTERS_AND_PHY_STATS		= 37,
    ATH12K_DBG_HTT_EXT_VDEVS_TXRX_STATS			= 38,
    ATH12K_DBG_HTT_EXT_PDEV_PER_STATS			= 40,
    ATH12K_DBG_HTT_EXT_AST_ENTRIES				= 41,
    ATH12K_DBG_HTT_EXT_STATS_SOC_ERROR			= 45,
    ATH12K_DBG_HTT_DBG_PDEV_PUNCTURE_STATS			= 46,
    ATH12K_DBG_HTT_EXT_STATS_PDEV_SCHED_ALGO		= 49,
    ATH12K_DBG_HTT_EXT_STATS_MANDATORY_MUOFDMA		= 51,
    ATH12K_DGB_HTT_EXT_STATS_PDEV_MBSSID_CTRL_FRAME		= 54,
    ATH12K_DBG_HTT_PDEV_TDMA_STATS				= 57,
    ATH12K_DBG_HTT_MLO_SCHED_STATS				= 63,
    ATH12K_DBG_HTT_PDEV_MLO_IPC_STATS			= 64,
    ATH12K_DBG_HTT_EXT_PDEV_RTT_RESP_STATS			= 65,
    ATH12K_DBG_HTT_EXT_PDEV_RTT_INITIATOR_STATS		= 66,
    ATH12K_DBG_HTT_EXT_CHAN_SWITCH_STATS			= 76,

// keep this last
    ATH12K_DBG_HTT_NUM_EXT_STATS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_dbg_htt_tlv_tag {
    HTT_STATS_TX_PDEV_CMN_TAG			= 0,
    HTT_STATS_TX_PDEV_UNDERRUN_TAG			= 1,
    HTT_STATS_TX_PDEV_SIFS_TAG			= 2,
    HTT_STATS_TX_PDEV_FLUSH_TAG			= 3,
    HTT_STATS_STRING_TAG				= 5,
    HTT_STATS_TX_HWQ_CMN_TAG                        = 6,
    HTT_STATS_TX_TQM_GEN_MPDU_TAG			= 11,
    HTT_STATS_TX_TQM_LIST_MPDU_TAG			= 12,
    HTT_STATS_TX_TQM_LIST_MPDU_CNT_TAG		= 13,
    HTT_STATS_TX_TQM_CMN_TAG			= 14,
    HTT_STATS_TX_TQM_PDEV_TAG			= 15,
    HTT_STATS_TX_DE_EAPOL_PACKETS_TAG		= 17,
    HTT_STATS_TX_DE_CLASSIFY_FAILED_TAG		= 18,
    HTT_STATS_TX_DE_CLASSIFY_STATS_TAG		= 19,
    HTT_STATS_TX_DE_CLASSIFY_STATUS_TAG		= 20,
    HTT_STATS_TX_DE_ENQUEUE_PACKETS_TAG		= 21,
    HTT_STATS_TX_DE_ENQUEUE_DISCARD_TAG		= 22,
    HTT_STATS_TX_DE_CMN_TAG				= 23,
    HTT_STATS_TX_PDEV_MU_MIMO_STATS_TAG		= 25,
    HTT_STATS_SFM_CMN_TAG				= 26,
    HTT_STATS_SRING_STATS_TAG			= 27,
    HTT_STATS_RX_PDEV_FW_STATS_TAG                  = 28,
    HTT_STATS_TX_PDEV_RATE_STATS_TAG		= 34,
    HTT_STATS_RX_PDEV_RATE_STATS_TAG		= 35,
    HTT_STATS_TX_PDEV_SCHEDULER_TXQ_STATS_TAG	= 36,
    HTT_STATS_TX_SCHED_CMN_TAG			= 37,
    HTT_STATS_SCHED_TXQ_CMD_POSTED_TAG		= 39,
    HTT_STATS_SFM_CLIENT_USER_TAG			= 41,
    HTT_STATS_SFM_CLIENT_TAG			= 42,
    HTT_STATS_TX_TQM_ERROR_STATS_TAG                = 43,
    HTT_STATS_SCHED_TXQ_CMD_REAPED_TAG		= 44,
    HTT_STATS_TX_SELFGEN_AC_ERR_STATS_TAG		= 46,
    HTT_STATS_TX_SELFGEN_CMN_STATS_TAG		= 47,
    HTT_STATS_TX_SELFGEN_AC_STATS_TAG		= 48,
    HTT_STATS_TX_SELFGEN_AX_STATS_TAG		= 49,
    HTT_STATS_TX_SELFGEN_AX_ERR_STATS_TAG		= 50,
    HTT_STATS_HW_INTR_MISC_TAG			= 54,
    HTT_STATS_HW_PDEV_ERRS_TAG			= 56,
    HTT_STATS_TX_DE_COMPL_STATS_TAG			= 65,
    HTT_STATS_WHAL_TX_TAG				= 66,
    HTT_STATS_TX_PDEV_SIFS_HIST_TAG			= 67,
    HTT_STATS_PDEV_CCA_1SEC_HIST_TAG		= 70,
    HTT_STATS_PDEV_CCA_100MSEC_HIST_TAG		= 71,
    HTT_STATS_PDEV_CCA_STAT_CUMULATIVE_TAG		= 72,
    HTT_STATS_PDEV_CCA_COUNTERS_TAG			= 73,
    HTT_STATS_TX_PDEV_MPDU_STATS_TAG		= 74,
    HTT_STATS_TX_SOUNDING_STATS_TAG			= 80,
    HTT_STATS_SCHED_TXQ_SCHED_ORDER_SU_TAG		= 86,
    HTT_STATS_SCHED_TXQ_SCHED_INELIGIBILITY_TAG	= 87,
    HTT_STATS_PDEV_OBSS_PD_TAG			= 88,
    HTT_STATS_HW_WAR_TAG				= 89,
    HTT_STATS_LATENCY_PROF_STATS_TAG		= 91,
    HTT_STATS_LATENCY_CTX_TAG			= 92,
    HTT_STATS_LATENCY_CNT_TAG			= 93,
    HTT_STATS_RX_PDEV_UL_TRIG_STATS_TAG		= 94,
    HTT_STATS_RX_PDEV_UL_OFDMA_USER_STATS_TAG	= 95,
    HTT_STATS_RX_PDEV_UL_MUMIMO_TRIG_STATS_TAG	= 97,
    HTT_STATS_RX_FSE_STATS_TAG			= 98,
    HTT_STATS_SCHED_TXQ_SUPERCYCLE_TRIGGER_TAG	= 100,
    HTT_STATS_PDEV_CTRL_PATH_TX_STATS_TAG		= 102,
    HTT_STATS_RX_PDEV_RATE_EXT_STATS_TAG		= 103,
    HTT_STATS_PDEV_TX_RATE_TXBF_STATS_TAG		= 108,
    HTT_STATS_TX_SELFGEN_AC_SCHED_STATUS_STATS_TAG	= 111,
    HTT_STATS_TX_SELFGEN_AX_SCHED_STATUS_STATS_TAG	= 112,
    HTT_STATS_DLPAGER_STATS_TAG			= 120,
    HTT_STATS_PHY_COUNTERS_TAG			= 121,
    HTT_STATS_PHY_STATS_TAG				= 122,
    HTT_STATS_PHY_RESET_COUNTERS_TAG		= 123,
    HTT_STATS_PHY_RESET_STATS_TAG			= 124,
    HTT_STATS_SOC_TXRX_STATS_COMMON_TAG		= 125,
    HTT_STATS_PER_RATE_STATS_TAG			= 128,
    HTT_STATS_MU_PPDU_DIST_TAG			= 129,
    HTT_STATS_TX_PDEV_MUMIMO_GRP_STATS_TAG		= 130,
    HTT_STATS_AST_ENTRY_TAG				= 132,
    HTT_STATS_TX_PDEV_RATE_STATS_BE_OFDMA_TAG	= 135,
    HTT_STATS_TX_SELFGEN_BE_ERR_STATS_TAG		= 137,
    HTT_STATS_TX_SELFGEN_BE_STATS_TAG		= 138,
    HTT_STATS_TX_SELFGEN_BE_SCHED_STATUS_STATS_TAG	= 139,
    HTT_STATS_TX_PDEV_HISTOGRAM_STATS_TAG		= 144,
    HTT_STATS_TXBF_OFDMA_AX_NDPA_STATS_TAG		= 147,
    HTT_STATS_TXBF_OFDMA_AX_NDP_STATS_TAG		= 148,
    HTT_STATS_TXBF_OFDMA_AX_BRP_STATS_TAG		= 149,
    HTT_STATS_TXBF_OFDMA_AX_STEER_STATS_TAG		= 150,
    HTT_STATS_DMAC_RESET_STATS_TAG			= 155,
    HTT_STATS_PHY_TPC_STATS_TAG			= 157,
    HTT_STATS_PDEV_PUNCTURE_STATS_TAG		= 158,
    HTT_STATS_PDEV_SCHED_ALGO_OFDMA_STATS_TAG	= 165,
    HTT_STATS_TXBF_OFDMA_AX_STEER_MPDU_STATS_TAG	= 172,
    HTT_STATS_PDEV_MBSSID_CTRL_FRAME_STATS_TAG	= 176,
    HTT_STATS_PDEV_TDMA_TAG				= 187,
    HTT_STATS_MLO_SCHED_STATS_TAG			= 190,
    HTT_STATS_PDEV_MLO_IPC_STATS_TAG		= 191,
    HTT_STATS_PDEV_RTT_RESP_STATS_TAG		= 194,
    HTT_STATS_PDEV_RTT_INIT_STATS_TAG		= 195,
    HTT_STATS_PDEV_RTT_HW_STATS_TAG			= 196,
    HTT_STATS_PDEV_RTT_TBR_SELFGEN_QUEUED_STATS_TAG	= 197,
    HTT_STATS_PDEV_RTT_TBR_CMD_RESULT_STATS_TAG	= 198,
    HTT_STATS_CHAN_SWITCH_STATS_TAG			= 213,

    HTT_STATS_MAX_TAG,
}

pub const ATH12K_HTT_TX_PDEV_MAX_SIFS_BURST_STATS: c_int = 9;
pub const ATH12K_HTT_TX_PDEV_MAX_FLUSH_REASON_STATS: c_int = 150;
// MU MIMO distribution stats is a 2-dimensional array
// with dimension one denoting stats for nr4[0] or nr8[1]
//
pub const ATH12K_HTT_STATS_NUM_NR_BINS: c_int = 2;
pub const ATH12K_HTT_STATS_MAX_NUM_MU_PPDU_PER_BURST: c_int = 10;
pub const ATH12K_HTT_TX_PDEV_MAX_SIFS_BURST_HIST_STATS: c_int = 10;
pub const ATH12K_HTT_STATS_MAX_NUM_SCHED_STATUS: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_tx_pdev_underrun_enum {
    HTT_STATS_TX_PDEV_NO_DATA_UNDERRUN		= 0,
    HTT_STATS_TX_PDEV_DATA_UNDERRUN_BETWEEN_MPDU	= 1,
    HTT_STATS_TX_PDEV_DATA_UNDERRUN_WITHIN_MPDU	= 2,
    HTT_TX_PDEV_MAX_URRN_STATS			= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_reset_cfg_param_alloc_pos {
    ATH12K_HTT_STATS_RESET_PARAM_CFG_32_BYTES = 1,
    ATH12K_HTT_STATS_RESET_PARAM_CFG_64_BYTES,
    ATH12K_HTT_STATS_RESET_PARAM_CFG_128_BYTES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_htt_stats_req {
    pub done: bool,
    pub override_cfg_param: bool,
    pub pdev_id: u8,
    pub type: ath12k_dbg_htt_ext_stats_type,
    pub cfg_param: [u32; 4],
    pub peer_addr: [u8; ETH_ALEN],
    pub htt_stats_rcvd: completion,
    pub buf_len: u32,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_stats_cmn_tlv {
    pub mac_id__word: __le32,
    pub hw_queued: __le32,
    pub hw_reaped: __le32,
    pub underrun: __le32,
    pub hw_paused: __le32,
    pub hw_flush: __le32,
    pub hw_filt: __le32,
    pub tx_abort: __le32,
    pub mpdu_requed: __le32,
    pub tx_xretry: __le32,
    pub data_rc: __le32,
    pub mpdu_dropped_xretry: __le32,
    pub illgl_rate_phy_err: __le32,
    pub cont_xretry: __le32,
    pub tx_timeout: __le32,
    pub pdev_resets: __le32,
    pub phy_underrun: __le32,
    pub txop_ovf: __le32,
    pub seq_posted: __le32,
    pub seq_failed_queueing: __le32,
    pub seq_completed: __le32,
    pub seq_restarted: __le32,
    pub mu_seq_posted: __le32,
    pub seq_switch_hw_paused: __le32,
    pub next_seq_posted_dsr: __le32,
    pub seq_posted_isr: __le32,
    pub seq_ctrl_cached: __le32,
    pub mpdu_count_tqm: __le32,
    pub msdu_count_tqm: __le32,
    pub mpdu_removed_tqm: __le32,
    pub msdu_removed_tqm: __le32,
    pub mpdus_sw_flush: __le32,
    pub mpdus_hw_filter: __le32,
    pub mpdus_truncated: __le32,
    pub mpdus_ack_failed: __le32,
    pub mpdus_expired: __le32,
    pub mpdus_seq_hw_retry: __le32,
    pub ack_tlv_proc: __le32,
    pub coex_abort_mpdu_cnt_valid: __le32,
    pub coex_abort_mpdu_cnt: __le32,
    pub num_total_ppdus_tried_ota: __le32,
    pub num_data_ppdus_tried_ota: __le32,
    pub local_ctrl_mgmt_enqued: __le32,
    pub local_ctrl_mgmt_freed: __le32,
    pub local_data_enqued: __le32,
    pub local_data_freed: __le32,
    pub mpdu_tried: __le32,
    pub isr_wait_seq_posted: __le32,
    pub tx_active_dur_us_low: __le32,
    pub tx_active_dur_us_high: __le32,
    pub remove_mpdus_max_retries: __le32,
    pub comp_delivered: __le32,
    pub ppdu_ok: __le32,
    pub self_triggers: __le32,
    pub tx_time_dur_data: __le32,
    pub seq_qdepth_repost_stop: __le32,
    pub mu_seq_min_msdu_repost_stop: __le32,
    pub seq_min_msdu_repost_stop: __le32,
    pub seq_txop_repost_stop: __le32,
    pub next_seq_cancel: __le32,
    pub fes_offsets_err_cnt: __le32,
    pub num_mu_peer_blacklisted: __le32,
    pub mu_ofdma_seq_posted: __le32,
    pub ul_mumimo_seq_posted: __le32,
    pub ul_ofdma_seq_posted: __le32,
    pub thermal_suspend_cnt: __le32,
    pub dfs_suspend_cnt: __le32,
    pub tx_abort_suspend_cnt: __le32,
    pub tgt_specific_opaque_txq_suspend_info: __le32,
    pub last_suspend_reason: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_stats_urrn_tlv {
    pub urrn_stats): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_stats_flush_tlv {
    pub flush_errs): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_stats_phy_err_tlv {
    pub phy_errs): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_stats_sifs_tlv {
    pub sifs_status): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_ctrl_path_tx_stats_tlv {
    pub fw_tx_mgmt_subtype: [__le32; ATH12K_HTT_STATS_SUBTYPE_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_stats_sifs_hist_tlv {
    pub sifs_hist_status): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_hw_mode {
    ATH12K_HTT_STATS_HWMODE_AC = 0,
    ATH12K_HTT_STATS_HWMODE_AX = 1,
    ATH12K_HTT_STATS_HWMODE_BE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_mu_ppdu_dist_stats_tlv {
    pub hw_mode: __le32,
    pub num_seq_term_status: [__le32; ATH12K_HTT_STATS_NUM_SCHED_STATUS_WORDS],
    pub num_ppdu_cmpl_per_burst: [__le32; ATH12K_HTT_STATS_MU_PPDU_PER_BURST_WORDS],
    pub num_seq_posted: [__le32; ATH12K_HTT_STATS_NUM_NR_BINS],
    pub num_ppdu_posted_per_burst: [__le32; ATH12K_HTT_STATS_MU_PPDU_PER_BURST_WORDS],
    pub __packed: },
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS: c_int = 12;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_GI_COUNTERS: c_int = 4;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_DCM_COUNTERS: c_int = 5;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_BW_COUNTERS: c_int = 4;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS: c_int = 8;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_PREAMBLE_TYPES: c_int = 7;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_LEGACY_CCK_STATS: c_int = 4;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_LEGACY_OFDM_STATS: c_int = 8;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_LTF: c_int = 4;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_EXTRA_MCS_COUNTERS: c_int = 2;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_EXTRA2_MCS_COUNTERS: c_int = 2;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_11AX_TRIGGER_TYPES: c_int = 6;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_PER_COUNTERS: c_int = 101;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_rate_stats_tlv {
    pub mac_id_word: __le32,
    pub tx_ldpc: __le32,
    pub rts_cnt: __le32,
    pub ack_rssi: __le32,
    pub tx_mcs: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub tx_su_mcs: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub tx_mu_mcs: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub tx_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub tx_bw: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub tx_stbc: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub tx_pream: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_PREAMBLE_TYPES],
    pub tx_dcm: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_DCM_COUNTERS],
    pub rts_success: __le32,
    pub tx_legacy_cck_rate: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_LEGACY_CCK_STATS],
    pub tx_legacy_ofdm_rate: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_LEGACY_OFDM_STATS],
    pub ac_mu_mimo_tx_ldpc: __le32,
    pub ax_mu_mimo_tx_ldpc: __le32,
    pub ofdma_tx_ldpc: __le32,
    pub tx_he_ltf: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_LTF],
    pub ac_mu_mimo_tx_mcs: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ax_mu_mimo_tx_mcs: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ofdma_tx_mcs: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ac_mu_mimo_tx_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ax_mu_mimo_tx_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ofdma_tx_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ac_mu_mimo_tx_bw: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub ax_mu_mimo_tx_bw: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub ofdma_tx_bw: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub trigger_type_11ax: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_11AX_TRIGGER_TYPES],
    pub tx_11ax_su_ext: __le32,
    pub tx_mcs_ext: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_EXTRA_MCS_COUNTERS],
    pub tx_stbc_ext: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_EXTRA_MCS_COUNTERS],
    pub ax_mu_mimo_tx_mcs_ext: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_EXTRA_MCS_COUNTERS],
    pub ofdma_tx_mcs_ext: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_EXTRA_MCS_COUNTERS],
    pub tx_mcs_ext_2: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_EXTRA2_MCS_COUNTERS],
    pub tx_bw_320mhz: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_histogram_stats_tlv {
    pub rate_retry_mcs_drop_cnt: __le32,
    pub mcs_drop_rate: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_MCS_DROP_COUNTERS],
    pub per_histogram_cnt: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_PER_COUNTERS],
    pub low_latency_rate_cnt: __le32,
    pub su_burst_rate_drop_cnt: __le32,
    pub su_burst_rate_drop_fail_cnt: __le32,
    pub __packed: },
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_LEGACY_CCK_STATS: c_int = 4;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_LEGACY_OFDM_STATS: c_int = 8;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS: c_int = 12;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_GI_COUNTERS: c_int = 4;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_DCM_COUNTERS: c_int = 5;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_BW_COUNTERS: c_int = 4;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_SPATIAL_STREAMS: c_int = 8;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_PREAMBLE_TYPES: c_int = 7;
pub const ATH12K_HTT_RX_PDEV_MAX_OFDMA_NUM_USER: c_int = 8;
pub const ATH12K_HTT_RX_PDEV_STATS_RXEVM_MAX_PILOTS_NSS: c_int = 16;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_RU_SIZE_COUNTERS: c_int = 6;
pub const ATH12K_HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER: c_int = 8;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_EXTRA_MCS_COUNTERS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_rx_pdev_rate_stats_tlv {
    pub mac_id_word: __le32,
    pub nsts: __le32,
    pub rx_ldpc: __le32,
    pub rts_cnt: __le32,
    pub rssi_mgmt: __le32,
    pub rssi_data: __le32,
    pub rssi_comb: __le32,
    pub rx_mcs: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_nss: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub rx_dcm: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_DCM_COUNTERS],
    pub rx_stbc: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_bw: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_BW_COUNTERS],
    pub rx_pream: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_PREAMBLE_TYPES],
    pub rssi_in_dbm: __le32,
    pub rx_11ax_su_ext: __le32,
    pub rx_11ac_mumimo: __le32,
    pub rx_11ax_mumimo: __le32,
    pub rx_11ax_ofdma: __le32,
    pub txbf: __le32,
    pub rx_legacy_cck_rate: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_LEGACY_CCK_STATS],
    pub rx_legacy_ofdm_rate: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_LEGACY_OFDM_STATS],
    pub rx_active_dur_us_low: __le32,
    pub rx_active_dur_us_high: __le32,
    pub rx_11ax_ul_ofdma: __le32,
    pub ul_ofdma_rx_mcs: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ul_ofdma_rx_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ul_ofdma_rx_bw: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub ul_ofdma_rx_stbc: __le32,
    pub ul_ofdma_rx_ldpc: __le32,
    pub rx_ulofdma_non_data_ppdu: [__le32; ATH12K_HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
    pub rx_ulofdma_data_ppdu: [__le32; ATH12K_HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
    pub rx_ulofdma_mpdu_ok: [__le32; ATH12K_HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
    pub rx_ulofdma_mpdu_fail: [__le32; ATH12K_HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
    pub nss_count: __le32,
    pub pilot_count: __le32,
    pub rx_pilot_evm_db_mean: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub per_chain_rssi_pkt_type: __le32,
    pub rx_su_ndpa: __le32,
    pub rx_11ax_su_txbf_mcs: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_mu_ndpa: __le32,
    pub rx_11ax_mu_txbf_mcs: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_br_poll: __le32,
    pub rx_11ax_dl_ofdma_mcs: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_11ax_dl_ofdma_ru: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_RU_SIZE_COUNTERS],
    pub rx_ulmumimo_non_data_ppdu: [__le32; ATH12K_HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulmumimo_data_ppdu: [__le32; ATH12K_HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulmumimo_mpdu_ok: [__le32; ATH12K_HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulmumimo_mpdu_fail: [__le32; ATH12K_HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulofdma_non_data_nusers: [__le32; ATH12K_HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
    pub rx_ulofdma_data_nusers: [__le32; ATH12K_HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
    pub rx_mcs_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_EXTRA_MCS_COUNTERS],
    pub __packed: },
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_BW_EXT_COUNTERS: c_int = 4;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS_EXT: c_int = 14;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_EXTRA2_MCS_COUNTERS: c_int = 2;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_BW_EXT2_COUNTERS: c_int = 5;
pub const ATH12K_HTT_RX_PDEV_STATS_NUM_PUNCTURED_MODE_COUNTERS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_rx_pdev_rate_ext_stats_tlv {
    pub rssi_mcast_in_dbm: __le32,
    pub rssi_mgmt_in_dbm: __le32,
    pub rx_mcs_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS_EXT],
    pub rx_stbc_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS_EXT],
    pub ul_ofdma_rx_mcs_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS_EXT],
    pub rx_11ax_su_txbf_mcs_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS_EXT],
    pub rx_11ax_mu_txbf_mcs_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS_EXT],
    pub rx_11ax_dl_ofdma_mcs_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS_EXT],
    pub rx_mcs_ext_2: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_EXTRA2_MCS_COUNTERS],
    pub rx_bw_ext: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_BW_EXT2_COUNTERS],
    pub rx_su_punctured_mode: [__le32; ATH12K_HTT_RX_PDEV_STATS_NUM_PUNCTURED_MODE_COUNTERS],
    pub __packed: },

pub const ATH12K_HTT_TX_PDEV_NUM_SCHED_ORDER_LOG: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_stats_tx_sched_cmn_tlv {
    pub mac_id__word: __le32,
    pub current_timestamp: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_stats_sched_per_txq_tlv {
    pub mac_id__word: __le32,
    pub sched_policy: __le32,
    pub last_sched_cmd_posted_timestamp: __le32,
    pub last_sched_cmd_compl_timestamp: __le32,
    pub sched_2_tac_lwm_count: __le32,
    pub sched_2_tac_ring_full: __le32,
    pub sched_cmd_post_failure: __le32,
    pub num_active_tids: __le32,
    pub num_ps_schedules: __le32,
    pub sched_cmds_pending: __le32,
    pub num_tid_register: __le32,
    pub num_tid_unregister: __le32,
    pub num_qstats_queried: __le32,
    pub qstats_update_pending: __le32,
    pub last_qstats_query_timestamp: __le32,
    pub num_tqm_cmdq_full: __le32,
    pub num_de_sched_algo_trigger: __le32,
    pub num_rt_sched_algo_trigger: __le32,
    pub num_tqm_sched_algo_trigger: __le32,
    pub notify_sched: __le32,
    pub dur_based_sendn_term: __le32,
    pub su_notify2_sched: __le32,
    pub su_optimal_queued_msdus_sched: __le32,
    pub su_delay_timeout_sched: __le32,
    pub su_min_txtime_sched_delay: __le32,
    pub su_no_delay: __le32,
    pub num_supercycles: __le32,
    pub num_subcycles_with_sort: __le32,
    pub num_subcycles_no_sort: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sched_txq_cmd_posted_tlv {
    pub sched_cmd_posted): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sched_txq_cmd_reaped_tlv {
    pub sched_cmd_reaped): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sched_txq_sched_order_su_tlv {
    pub sched_order_su): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sched_txq_sched_ineligibility_tlv {
    pub sched_ineligibility): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_sched_txq_supercycle_triggers_tlv_enum {
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_NONE = 0,
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_FORCED,
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_LESS_NUM_TIDQ_ENTRIES,
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_LESS_NUM_ACTIVE_TIDS,
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_MAX_ITR_REACHED,
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_DUR_THRESHOLD_REACHED,
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_TWT_TRIGGER,
    ATH12K_HTT_SCHED_SUPERCYCLE_TRIGGER_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sched_txq_supercycle_triggers_tlv {
    pub supercycle_triggers): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_hw_stats_pdev_errs_tlv {
    pub mac_id__word: __le32,
    pub tx_abort: __le32,
    pub tx_abort_fail_count: __le32,
    pub rx_abort: __le32,
    pub rx_abort_fail_count: __le32,
    pub warm_reset: __le32,
    pub cold_reset: __le32,
    pub tx_flush: __le32,
    pub tx_glb_reset: __le32,
    pub tx_txq_reset: __le32,
    pub rx_timeout_reset: __le32,
    pub mac_cold_reset_restore_cal: __le32,
    pub mac_cold_reset: __le32,
    pub mac_warm_reset: __le32,
    pub mac_only_reset: __le32,
    pub phy_warm_reset: __le32,
    pub phy_warm_reset_ucode_trig: __le32,
    pub mac_warm_reset_restore_cal: __le32,
    pub mac_sfm_reset: __le32,
    pub phy_warm_reset_m3_ssr: __le32,
    pub phy_warm_reset_reason_phy_m3: __le32,
    pub phy_warm_reset_reason_tx_hw_stuck: __le32,
    pub phy_warm_reset_reason_num_rx_frame_stuck: __le32,
    pub phy_warm_reset_reason_wal_rx_rec_rx_busy: __le32,
    pub phy_warm_reset_reason_wal_rx_rec_mac_hng: __le32,
    pub phy_warm_reset_reason_mac_conv_phy_reset: __le32,
    pub wal_rx_recovery_rst_mac_hang_cnt: __le32,
    pub wal_rx_recovery_rst_known_sig_cnt: __le32,
    pub wal_rx_recovery_rst_no_rx_cnt: __le32,
    pub wal_rx_recovery_rst_no_rx_consec_cnt: __le32,
    pub wal_rx_recovery_rst_rx_busy_cnt: __le32,
    pub wal_rx_recovery_rst_phy_mac_hang_cnt: __le32,
    pub rx_flush_cnt: __le32,
    pub phy_warm_reset_reason_tx_exp_cca_stuck: __le32,
    pub phy_warm_reset_reason_tx_consec_flsh_war: __le32,
    pub phy_warm_reset_reason_tx_hwsch_reset_war: __le32,
    pub phy_warm_reset_reason_hwsch_cca_wdog_war: __le32,
    pub fw_rx_rings_reset: __le32,
    pub rx_dest_drain_rx_descs_leak_prevented: __le32,
    pub rx_dest_drain_rx_descs_saved_cnt: __le32,
    pub rx_dest_drain_rxdma2reo_leak_detected: __le32,
    pub rx_dest_drain_rxdma2fw_leak_detected: __le32,
    pub rx_dest_drain_rxdma2wbm_leak_detected: __le32,
    pub rx_dest_drain_rxdma1_2sw_leak_detected: __le32,
    pub rx_dest_drain_rx_drain_ok_mac_idle: __le32,
    pub rx_dest_drain_ok_mac_not_idle: __le32,
    pub rx_dest_drain_prerequisite_invld: __le32,
    pub rx_dest_drain_skip_non_lmac_reset: __le32,
    pub rx_dest_drain_hw_fifo_notempty_post_wait: __le32,
    pub __packed: },
pub const ATH12K_HTT_STATS_MAX_HW_INTR_NAME_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_hw_stats_intr_misc_tlv {
    pub hw_intr_name: [u8; ATH12K_HTT_STATS_MAX_HW_INTR_NAME_LEN],
    pub mask: __le32,
    pub count: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_hw_stats_whal_tx_tlv {
    pub mac_id__word: __le32,
    pub last_unpause_ppdu_id: __le32,
    pub hwsch_unpause_wait_tqm_write: __le32,
    pub hwsch_dummy_tlv_skipped: __le32,
    pub hwsch_misaligned_offset_received: __le32,
    pub hwsch_reset_count: __le32,
    pub hwsch_dev_reset_war: __le32,
    pub hwsch_delayed_pause: __le32,
    pub hwsch_long_delayed_pause: __le32,
    pub sch_rx_ppdu_no_response: __le32,
    pub sch_selfgen_response: __le32,
    pub sch_rx_sifs_resp_trigger: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_hw_war_stats_tlv {
    pub mac_id__word: __le32,
    pub hw_wars): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_tqm_cmn_stats_tlv {
    pub mac_id__word: __le32,
    pub max_cmdq_id: __le32,
    pub list_mpdu_cnt_hist_intvl: __le32,
    pub add_msdu: __le32,
    pub q_empty: __le32,
    pub q_not_empty: __le32,
    pub drop_notification: __le32,
    pub desc_threshold: __le32,
    pub hwsch_tqm_invalid_status: __le32,
    pub missed_tqm_gen_mpdus: __le32,
    pub tqm_active_tids: __le32,
    pub tqm_inactive_tids: __le32,
    pub tqm_active_msduq_flows: __le32,
    pub msduq_timestamp_updates: __le32,
    pub msduq_updates_mpdu_head_info_cmd: __le32,
    pub msduq_updates_emp_to_nonemp_status: __le32,
    pub get_mpdu_head_info_cmds_by_query: __le32,
    pub get_mpdu_head_info_cmds_by_tac: __le32,
    pub gen_mpdu_cmds_by_query: __le32,
    pub high_prio_q_not_empty: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_tqm_error_stats_tlv {
    pub q_empty_failure: __le32,
    pub q_not_empty_failure: __le32,
    pub add_msdu_failure: __le32,
    pub tqm_cache_ctl_err: __le32,
    pub tqm_soft_reset: __le32,
    pub tqm_reset_num_in_use_link_descs: __le32,
    pub tqm_reset_num_lost_link_descs: __le32,
    pub tqm_reset_num_lost_host_tx_buf_cnt: __le32,
    pub tqm_reset_num_in_use_internal_tqm: __le32,
    pub tqm_reset_num_in_use_idle_link_rng: __le32,
    pub tqm_reset_time_to_tqm_hang_delta_ms: __le32,
    pub tqm_reset_recovery_time_ms: __le32,
    pub tqm_reset_num_peers_hdl: __le32,
    pub tqm_reset_cumm_dirty_hw_mpduq_cnt: __le32,
    pub tqm_reset_cumm_dirty_hw_msduq_proc: __le32,
    pub tqm_reset_flush_cache_cmd_su_cnt: __le32,
    pub tqm_reset_flush_cache_cmd_other_cnt: __le32,
    pub tqm_reset_flush_cache_cmd_trig_type: __le32,
    pub tqm_reset_flush_cache_cmd_trig_cfg: __le32,
    pub tqm_reset_flush_cmd_skp_status_null: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_tqm_gen_mpdu_stats_tlv {
    pub gen_mpdu_end_reason): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
pub const ATH12K_HTT_TX_TQM_MAX_LIST_MPDU_END_REASON: c_int = 16;
pub const ATH12K_HTT_TX_TQM_MAX_LIST_MPDU_CNT_HISTOGRAM_BINS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_tqm_list_mpdu_stats_tlv {
    pub list_mpdu_end_reason): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_tqm_list_mpdu_cnt_tlv {
    pub list_mpdu_cnt_hist): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_tqm_pdev_stats_tlv {
    pub msdu_count: __le32,
    pub mpdu_count: __le32,
    pub remove_msdu: __le32,
    pub remove_mpdu: __le32,
    pub remove_msdu_ttl: __le32,
    pub send_bar: __le32,
    pub bar_sync: __le32,
    pub notify_mpdu: __le32,
    pub sync_cmd: __le32,
    pub write_cmd: __le32,
    pub hwsch_trigger: __le32,
    pub ack_tlv_proc: __le32,
    pub gen_mpdu_cmd: __le32,
    pub gen_list_cmd: __le32,
    pub remove_mpdu_cmd: __le32,
    pub remove_mpdu_tried_cmd: __le32,
    pub mpdu_queue_stats_cmd: __le32,
    pub mpdu_head_info_cmd: __le32,
    pub msdu_flow_stats_cmd: __le32,
    pub remove_msdu_cmd: __le32,
    pub remove_msdu_ttl_cmd: __le32,
    pub flush_cache_cmd: __le32,
    pub update_mpduq_cmd: __le32,
    pub enqueue: __le32,
    pub enqueue_notify: __le32,
    pub notify_mpdu_at_head: __le32,
    pub notify_mpdu_state_valid: __le32,
    pub sched_udp_notify1: __le32,
    pub sched_udp_notify2: __le32,
    pub sched_nonudp_notify1: __le32,
    pub sched_nonudp_notify2: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_cmn_stats_tlv {
    pub mac_id__word: __le32,
    pub tcl2fw_entry_count: __le32,
    pub not_to_fw: __le32,
    pub invalid_pdev_vdev_peer: __le32,
    pub tcl_res_invalid_addrx: __le32,
    pub wbm2fw_entry_count: __le32,
    pub invalid_pdev: __le32,
    pub tcl_res_addrx_timeout: __le32,
    pub invalid_vdev: __le32,
    pub invalid_tcl_exp_frame_desc: __le32,
    pub vdev_id_mismatch_cnt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_eapol_packets_stats_tlv {
    pub m1_packets: __le32,
    pub m2_packets: __le32,
    pub m3_packets: __le32,
    pub m4_packets: __le32,
    pub g1_packets: __le32,
    pub g2_packets: __le32,
    pub rc4_packets: __le32,
    pub eap_packets: __le32,
    pub eapol_start_packets: __le32,
    pub eapol_logoff_packets: __le32,
    pub eapol_encap_asf_packets: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_classify_stats_tlv {
    pub arp_packets: __le32,
    pub igmp_packets: __le32,
    pub dhcp_packets: __le32,
    pub host_inspected: __le32,
    pub htt_included: __le32,
    pub htt_valid_mcs: __le32,
    pub htt_valid_nss: __le32,
    pub htt_valid_preamble_type: __le32,
    pub htt_valid_chainmask: __le32,
    pub htt_valid_guard_interval: __le32,
    pub htt_valid_retries: __le32,
    pub htt_valid_bw_info: __le32,
    pub htt_valid_power: __le32,
    pub htt_valid_key_flags: __le32,
    pub htt_valid_no_encryption: __le32,
    pub fse_entry_count: __le32,
    pub fse_priority_be: __le32,
    pub fse_priority_high: __le32,
    pub fse_priority_low: __le32,
    pub fse_traffic_ptrn_be: __le32,
    pub fse_traffic_ptrn_over_sub: __le32,
    pub fse_traffic_ptrn_bursty: __le32,
    pub fse_traffic_ptrn_interactive: __le32,
    pub fse_traffic_ptrn_periodic: __le32,
    pub fse_hwqueue_alloc: __le32,
    pub fse_hwqueue_created: __le32,
    pub fse_hwqueue_send_to_host: __le32,
    pub mcast_entry: __le32,
    pub bcast_entry: __le32,
    pub htt_update_peer_cache: __le32,
    pub htt_learning_frame: __le32,
    pub fse_invalid_peer: __le32,
    pub mec_notify: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_classify_failed_stats_tlv {
    pub ap_bss_peer_not_found: __le32,
    pub ap_bcast_mcast_no_peer: __le32,
    pub sta_delete_in_progress: __le32,
    pub ibss_no_bss_peer: __le32,
    pub invalid_vdev_type: __le32,
    pub invalid_ast_peer_entry: __le32,
    pub peer_entry_invalid: __le32,
    pub ethertype_not_ip: __le32,
    pub eapol_lookup_failed: __le32,
    pub qpeer_not_allow_data: __le32,
    pub fse_tid_override: __le32,
    pub ipv6_jumbogram_zero_length: __le32,
    pub qos_to_non_qos_in_prog: __le32,
    pub ap_bcast_mcast_eapol: __le32,
    pub unicast_on_ap_bss_peer: __le32,
    pub ap_vdev_invalid: __le32,
    pub incomplete_llc: __le32,
    pub eapol_duplicate_m3: __le32,
    pub eapol_duplicate_m4: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_classify_status_stats_tlv {
    pub eok: __le32,
    pub classify_done: __le32,
    pub lookup_failed: __le32,
    pub send_host_dhcp: __le32,
    pub send_host_mcast: __le32,
    pub send_host_unknown_dest: __le32,
    pub send_host: __le32,
    pub status_invalid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_enqueue_packets_stats_tlv {
    pub enqueued_pkts: __le32,
    pub to_tqm: __le32,
    pub to_tqm_bypass: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_enqueue_discard_stats_tlv {
    pub discarded_pkts: __le32,
    pub local_frames: __le32,
    pub is_ext_msdu: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_de_compl_stats_tlv {
    pub tcl_dummy_frame: __le32,
    pub tqm_dummy_frame: __le32,
    pub tqm_notify_frame: __le32,
    pub fw2wbm_enq: __le32,
    pub tqm_bypass_frame: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_tx_mumimo_grp_invalid_reason_code_stats {
    ATH12K_HTT_TX_MUMIMO_GRP_VALID,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_NUM_MU_USERS_EXCEEDED_MU_MAX_USERS,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_SCHED_ALGO_NOT_MU_COMPATIBLE_GID,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_NON_PRIMARY_GRP,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_ZERO_CANDIDATES,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_MORE_CANDIDATES,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_GROUP_SIZE_EXCEED_NSS,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_GROUP_INELIGIBLE,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_GROUP_EFF_MU_TPUT_OMBPS,
    ATH12K_HTT_TX_MUMIMO_GRP_INVALID_MAX_REASON_CODE,
}

pub const ATH12K_HTT_NUM_AC_WMM: c_uint = 0x4;
pub const ATH12K_HTT_MAX_NUM_SBT_INTR: c_int = 4;
pub const ATH12K_HTT_TX_NUM_AC_MUMIMO_USER_STATS: c_int = 4;
pub const ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS: c_int = 8;
pub const ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS: c_int = 8;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS: c_int = 7;
pub const ATH12K_HTT_TX_NUM_OFDMA_USER_STATS: c_int = 74;
pub const ATH12K_HTT_TX_NUM_UL_MUMIMO_USER_STATS: c_int = 8;
pub const ATH12K_HTT_STATS_NUM_MAX_MUMIMO_SZ: c_int = 8;
pub const ATH12K_HTT_STATS_MUMIMO_TPUT_NUM_BINS: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_cmn_stats_tlv {
    pub mac_id__word: __le32,
    pub su_bar: __le32,
    pub rts: __le32,
    pub cts2self: __le32,
    pub qos_null: __le32,
    pub delayed_bar_1: __le32,
    pub delayed_bar_2: __le32,
    pub delayed_bar_3: __le32,
    pub delayed_bar_4: __le32,
    pub delayed_bar_5: __le32,
    pub delayed_bar_6: __le32,
    pub delayed_bar_7: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_ac_stats_tlv {
    pub ac_su_ndpa: __le32,
    pub ac_su_ndp: __le32,
    pub ac_mu_mimo_ndpa: __le32,
    pub ac_mu_mimo_ndp: __le32,
    pub 1]: __le32 ac_mu_mimo_brpoll[ATH12K_HTT_TX_NUM_AC_MUMIMO_USER_STATS -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_ax_stats_tlv {
    pub ax_su_ndpa: __le32,
    pub ax_su_ndp: __le32,
    pub ax_mu_mimo_ndpa: __le32,
    pub ax_mu_mimo_ndp: __le32,
    pub 1]: __le32 ax_mu_mimo_brpoll[ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS -,
    pub ax_basic_trigger: __le32,
    pub ax_bsr_trigger: __le32,
    pub ax_mu_bar_trigger: __le32,
    pub ax_mu_rts_trigger: __le32,
    pub ax_ulmumimo_trigger: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_be_stats_tlv {
    pub be_su_ndpa: __le32,
    pub be_su_ndp: __le32,
    pub be_mu_mimo_ndpa: __le32,
    pub be_mu_mimo_ndp: __le32,
    pub 1]: __le32 be_mu_mimo_brpoll[ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS -,
    pub be_basic_trigger: __le32,
    pub be_bsr_trigger: __le32,
    pub be_mu_bar_trigger: __le32,
    pub be_mu_rts_trigger: __le32,
    pub be_ulmumimo_trigger: __le32,
    pub be_su_ndpa_queued: __le32,
    pub be_su_ndp_queued: __le32,
    pub be_mu_mimo_ndpa_queued: __le32,
    pub be_mu_mimo_ndp_queued: __le32,
    pub 1]: __le32 be_mu_mimo_brpoll_queued[ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS -,
    pub be_ul_mumimo_trigger: [__le32; ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_ac_err_stats_tlv {
    pub ac_su_ndp_err: __le32,
    pub ac_su_ndpa_err: __le32,
    pub ac_mu_mimo_ndpa_err: __le32,
    pub ac_mu_mimo_ndp_err: __le32,
    pub ac_mu_mimo_brp1_err: __le32,
    pub ac_mu_mimo_brp2_err: __le32,
    pub ac_mu_mimo_brp3_err: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_ax_err_stats_tlv {
    pub ax_su_ndp_err: __le32,
    pub ax_su_ndpa_err: __le32,
    pub ax_mu_mimo_ndpa_err: __le32,
    pub ax_mu_mimo_ndp_err: __le32,
    pub 1]: __le32 ax_mu_mimo_brp_err[ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS -,
    pub ax_basic_trigger_err: __le32,
    pub ax_bsr_trigger_err: __le32,
    pub ax_mu_bar_trigger_err: __le32,
    pub ax_mu_rts_trigger_err: __le32,
    pub ax_ulmumimo_trigger_err: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_be_err_stats_tlv {
    pub be_su_ndp_err: __le32,
    pub be_su_ndpa_err: __le32,
    pub be_mu_mimo_ndpa_err: __le32,
    pub be_mu_mimo_ndp_err: __le32,
    pub 1]: __le32 be_mu_mimo_brp_err[ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS -,
    pub be_basic_trigger_err: __le32,
    pub be_bsr_trigger_err: __le32,
    pub be_mu_bar_trigger_err: __le32,
    pub be_mu_rts_trigger_err: __le32,
    pub be_ulmumimo_trigger_err: __le32,
    pub be_mu_mimo_brp_err_num_cbf_rxd: [__le32; ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS],
    pub be_su_ndpa_flushed: __le32,
    pub be_su_ndp_flushed: __le32,
    pub be_mu_mimo_ndpa_flushed: __le32,
    pub be_mu_mimo_ndp_flushed: __le32,
    pub 1]: __le32 be_mu_mimo_brpoll_flushed[ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS -,
    pub be_ul_mumimo_trigger_err: [__le32; ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_tx_selfgen_sch_tsflag_error_stats {
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_FLUSH_RCVD_ERR,
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_FILT_SCHED_CMD_ERR,
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_RESP_MISMATCH_ERR,
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_RESP_CBF_MIMO_CTRL_MISMATCH_ERR,
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_RESP_CBF_BW_MISMATCH_ERR,
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_RETRY_COUNT_FAIL_ERR,
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_RESP_TOO_LATE_RECEIVED_ERR,
    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_SIFS_STALL_NO_NEXT_CMD_ERR,

    ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_ac_sched_status_stats_tlv {
    pub ac_su_ndpa_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ac_su_ndp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ac_su_ndp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub ac_mu_mimo_ndpa_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ac_mu_mimo_ndp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ac_mu_mimo_ndp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub ac_mu_mimo_brp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ac_mu_mimo_brp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_ax_sched_status_stats_tlv {
    pub ax_su_ndpa_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_su_ndp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_su_ndp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub ax_mu_mimo_ndpa_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_mu_mimo_ndp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_mu_mimo_ndp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub ax_mu_brp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_mu_brp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub ax_mu_bar_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_mu_bar_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub ax_basic_trig_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_basic_trig_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub ax_ulmumimo_trig_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub ax_ulmumimo_trig_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_selfgen_be_sched_status_stats_tlv {
    pub be_su_ndpa_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_su_ndp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_su_ndp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub be_mu_mimo_ndpa_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_mu_mimo_ndp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_mu_mimo_ndp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub be_mu_brp_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_mu_brp_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub be_mu_bar_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_mu_bar_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub be_basic_trig_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_basic_trig_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub be_ulmumimo_trig_sch_status: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_TX_ERR_STATUS],
    pub be_ulmumimo_trig_sch_flag_err: [__le32; ATH12K_HTT_TX_SELFGEN_SCH_TSFLAG_ERR_STATS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_stats_string_tlv {
    pub data): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sring_stats_tlv {
    pub mac_id__ring_id__arena__ep: __le32,
    pub base_addr_lsb: __le32,
    pub base_addr_msb: __le32,
    pub ring_size: __le32,
    pub elem_size: __le32,
    pub num_avail_words__num_valid_words: __le32,
    pub head_ptr__tail_ptr: __le32,
    pub consumer_empty__producer_full: __le32,
    pub prefetch_count__internal_tail_ptr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sfm_cmn_tlv {
    pub mac_id__word: __le32,
    pub buf_total: __le32,
    pub mem_empty: __le32,
    pub deallocate_bufs: __le32,
    pub num_records: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sfm_client_tlv {
    pub client_id: __le32,
    pub buf_min: __le32,
    pub buf_max: __le32,
    pub buf_busy: __le32,
    pub buf_alloc: __le32,
    pub buf_avail: __le32,
    pub num_users: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_sfm_client_user_tlv {
    pub dwords_used_by_user_n): DECLARE_FLEX_ARRAY(__le32,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_mu_mimo_sch_stats_tlv {
    pub mu_mimo_sch_posted: __le32,
    pub mu_mimo_sch_failed: __le32,
    pub mu_mimo_ppdu_posted: __le32,
    pub ac_mu_mimo_sch_nusers: [__le32; ATH12K_HTT_TX_NUM_AC_MUMIMO_USER_STATS],
    pub ax_mu_mimo_sch_nusers: [__le32; ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS],
    pub ax_ofdma_sch_nusers: [__le32; ATH12K_HTT_TX_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_nusers: [__le32; ATH12K_HTT_TX_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_bsr_nusers: [__le32; ATH12K_HTT_TX_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_bar_nusers: [__le32; ATH12K_HTT_TX_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_brp_nusers: [__le32; ATH12K_HTT_TX_NUM_OFDMA_USER_STATS],
    pub ax_ul_mumimo_nusers: [__le32; ATH12K_HTT_TX_NUM_UL_MUMIMO_USER_STATS],
    pub ax_ul_mumimo_brp_nusers: [__le32; ATH12K_HTT_TX_NUM_UL_MUMIMO_USER_STATS],
    pub ac_mu_mimo_per_grp_sz: [__le32; ATH12K_HTT_TX_NUM_AC_MUMIMO_USER_STATS],
    pub ax_mu_mimo_per_grp_sz: [__le32; ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS],
    pub be_mu_mimo_sch_nusers: [__le32; ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS],
    pub be_mu_mimo_per_grp_sz: [__le32; ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS],
    pub ac_mu_mimo_grp_sz_ext: [__le32; ATH12K_HTT_TX_NUM_AC_MUMIMO_USER_STATS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_mumimo_grp_stats_tlv {
    pub dl_mumimo_grp_best_grp_size: [__le32; ATH12K_HTT_STATS_NUM_MAX_MUMIMO_SZ],
    pub dl_mumimo_grp_best_num_usrs: [__le32; ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS],
    pub dl_mumimo_grp_eligible: [__le32; ATH12K_HTT_STATS_NUM_MAX_MUMIMO_SZ],
    pub dl_mumimo_grp_ineligible: [__le32; ATH12K_HTT_STATS_NUM_MAX_MUMIMO_SZ],
    pub dl_mumimo_grp_invalid: [__le32; ATH12K_HTT_TX_NUM_MUMIMO_GRP_INVALID_WORDS],
    pub dl_mumimo_grp_tputs: [__le32; ATH12K_HTT_STATS_MUMIMO_TPUT_NUM_BINS],
    pub ul_mumimo_grp_best_grp_size: [__le32; ATH12K_HTT_STATS_NUM_MAX_MUMIMO_SZ],
    pub ul_mumimo_grp_best_usrs: [__le32; ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS],
    pub ul_mumimo_grp_tputs: [__le32; ATH12K_HTT_STATS_MUMIMO_TPUT_NUM_BINS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_tx_sched_modes {
    ATH12K_HTT_STATS_TX_SCHED_MODE_MU_MIMO_AC = 0,
    ATH12K_HTT_STATS_TX_SCHED_MODE_MU_MIMO_AX,
    ATH12K_HTT_STATS_TX_SCHED_MODE_MU_OFDMA_AX,
    ATH12K_HTT_STATS_TX_SCHED_MODE_MU_OFDMA_BE,
    ATH12K_HTT_STATS_TX_SCHED_MODE_MU_MIMO_BE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_mpdu_stats_tlv {
    pub mpdus_queued_usr: __le32,
    pub mpdus_tried_usr: __le32,
    pub mpdus_failed_usr: __le32,
    pub mpdus_requeued_usr: __le32,
    pub err_no_ba_usr: __le32,
    pub mpdu_underrun_usr: __le32,
    pub ampdu_underrun_usr: __le32,
    pub user_index: __le32,
    pub tx_sched_mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_stats_cca_counters_tlv {
    pub tx_frame_usec: __le32,
    pub rx_frame_usec: __le32,
    pub rx_clear_usec: __le32,
    pub my_rx_frame_usec: __le32,
    pub usec_cnt: __le32,
    pub med_rx_idle_usec: __le32,
    pub med_tx_idle_global_usec: __le32,
    pub cca_obss_usec: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_cca_stats_hist_v1_tlv {
    pub chan_num: __le32,
    pub num_records: __le32,
    pub valid_cca_counters_bitmap: __le32,
    pub collection_interval: __le32,
    pub __packed: },
pub const ATH12K_HTT_TX_CV_CORR_MAX_NUM_COLUMNS: c_int = 8;
pub const ATH12K_HTT_TX_NUM_AC_MUMIMO_USER_STATS: c_int = 4;
pub const ATH12K_HTT_TX_NUM_AX_MUMIMO_USER_STATS: c_int = 8;
pub const ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS: c_int = 8;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_BW_COUNTERS: c_int = 4;
pub const ATH12K_HTT_TX_NUM_MCS_CNTRS: c_int = 12;
pub const ATH12K_HTT_TX_NUM_EXTRA_MCS_CNTRS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_txbf_sound_steer_modes {
    ATH12K_HTT_IMPL_STEER_STATS		= 0,
    ATH12K_HTT_EXPL_SUSIFS_STEER_STATS	= 1,
    ATH12K_HTT_EXPL_SURBO_STEER_STATS	= 2,
    ATH12K_HTT_EXPL_MUSIFS_STEER_STATS	= 3,
    ATH12K_HTT_EXPL_MURBO_STEER_STATS	= 4,
    ATH12K_HTT_TXBF_MAX_NUM_OF_MODES	= 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_sounding_tx_mode {
    ATH12K_HTT_TX_AC_SOUNDING_MODE		= 0,
    ATH12K_HTT_TX_AX_SOUNDING_MODE		= 1,
    ATH12K_HTT_TX_BE_SOUNDING_MODE		= 2,
    ATH12K_HTT_TX_CMN_SOUNDING_MODE		= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_sounding_stats_tlv {
    pub tx_sounding_mode: __le32,
    pub cbf_20: [__le32; ATH12K_HTT_TXBF_MAX_NUM_OF_MODES],
    pub cbf_40: [__le32; ATH12K_HTT_TXBF_MAX_NUM_OF_MODES],
    pub cbf_80: [__le32; ATH12K_HTT_TXBF_MAX_NUM_OF_MODES],
    pub cbf_160: [__le32; ATH12K_HTT_TXBF_MAX_NUM_OF_MODES],
    pub sounding: [__le32; ATH12K_HTT_TX_NUM_OF_SOUNDING_STATS_WORDS],
    pub cv_nc_mismatch_err: __le32,
    pub cv_fcs_err: __le32,
    pub cv_frag_idx_mismatch: __le32,
    pub cv_invalid_peer_id: __le32,
    pub cv_no_txbf_setup: __le32,
    pub cv_expiry_in_update: __le32,
    pub cv_pkt_bw_exceed: __le32,
    pub cv_dma_not_done_err: __le32,
    pub cv_update_failed: __le32,
    pub cv_total_query: __le32,
    pub cv_total_pattern_query: __le32,
    pub cv_total_bw_query: __le32,
    pub cv_invalid_bw_coding: __le32,
    pub cv_forced_sounding: __le32,
    pub cv_standalone_sounding: __le32,
    pub cv_nc_mismatch: __le32,
    pub cv_fb_type_mismatch: __le32,
    pub cv_ofdma_bw_mismatch: __le32,
    pub cv_bw_mismatch: __le32,
    pub cv_pattern_mismatch: __le32,
    pub cv_preamble_mismatch: __le32,
    pub cv_nr_mismatch: __le32,
    pub cv_in_use_cnt_exceeded: __le32,
    pub cv_found: __le32,
    pub cv_not_found: __le32,
    pub sounding_320: [__le32; ATH12K_HTT_TX_NUM_BE_MUMIMO_USER_STATS],
    pub cbf_320: [__le32; ATH12K_HTT_TXBF_MAX_NUM_OF_MODES],
    pub cv_ntbr_sounding: __le32,
    pub cv_found_upload_in_progress: __le32,
    pub cv_expired_during_query: __le32,
    pub cv_dma_timeout_error: __le32,
    pub cv_buf_ibf_uploads: __le32,
    pub cv_buf_ebf_uploads: __le32,
    pub cv_buf_received: __le32,
    pub cv_buf_fed_back: __le32,
    pub cv_total_query_ibf: __le32,
    pub cv_found_ibf: __le32,
    pub cv_not_found_ibf: __le32,
    pub cv_expired_during_query_ibf: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_obss_pd_stats_tlv {
    pub num_obss_tx_ppdu_success: __le32,
    pub num_obss_tx_ppdu_failure: __le32,
    pub num_sr_tx_transmissions: __le32,
    pub num_spatial_reuse_opportunities: __le32,
    pub num_non_srg_opportunities: __le32,
    pub num_non_srg_ppdu_tried: __le32,
    pub num_non_srg_ppdu_success: __le32,
    pub num_srg_opportunities: __le32,
    pub num_srg_ppdu_tried: __le32,
    pub num_srg_ppdu_success: __le32,
    pub num_psr_opportunities: __le32,
    pub num_psr_ppdu_tried: __le32,
    pub num_psr_ppdu_success: __le32,
    pub num_non_srg_tried_per_ac: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub num_non_srg_success_ac: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub num_srg_tried_per_ac: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub num_srg_success_per_ac: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub num_obss_min_dur_check_flush_cnt: __le32,
    pub num_sr_ppdu_abort_flush_cnt: __le32,
    pub __packed: },
pub const ATH12K_HTT_STATS_MAX_PROF_STATS_NAME_LEN: c_int = 32;
pub const ATH12K_HTT_LATENCY_PROFILE_NUM_MAX_HIST: c_int = 3;
pub const ATH12K_HTT_INTERRUPTS_LATENCY_PROFILE_MAX_HIST: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_latency_prof_stats_tlv {
    pub print_header: __le32,
    pub latency_prof_name: [i8; ATH12K_HTT_STATS_MAX_PROF_STATS_NAME_LEN],
    pub cnt: __le32,
    pub min: __le32,
    pub max: __le32,
    pub last: __le32,
    pub tot: __le32,
    pub avg: __le32,
    pub hist_intvl: __le32,
    pub hist: [__le32; ATH12K_HTT_LATENCY_PROFILE_NUM_MAX_HIST],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_latency_prof_ctx_tlv {
    pub duration: __le32,
    pub tx_msdu_cnt: __le32,
    pub tx_mpdu_cnt: __le32,
    pub tx_ppdu_cnt: __le32,
    pub rx_msdu_cnt: __le32,
    pub rx_mpdu_cnt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_latency_prof_cnt_tlv {
    pub prof_enable_cnt: __le32,
    pub __packed: },
pub const ATH12K_HTT_RX_NUM_MCS_CNTRS: c_int = 12;
pub const ATH12K_HTT_RX_NUM_GI_CNTRS: c_int = 4;
pub const ATH12K_HTT_RX_NUM_SPATIAL_STREAMS: c_int = 8;
pub const ATH12K_HTT_RX_NUM_BW_CNTRS: c_int = 4;
pub const ATH12K_HTT_RX_NUM_RU_SIZE_CNTRS: c_int = 6;
pub const ATH12K_HTT_RX_NUM_RU_SIZE_160MHZ_CNTRS: c_int = 7;
pub const ATH12K_HTT_RX_UL_MAX_UPLINK_RSSI_TRACK: c_int = 5;
pub const ATH12K_HTT_RX_NUM_REDUCED_CHAN_TYPES: c_int = 2;
pub const ATH12K_HTT_RX_NUM_EXTRA_MCS_CNTRS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE {
    ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE_26,
    ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE_52,
    ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE_106,
    ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE_242,
    ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE_484,
    ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE_996,
    ATH12K_HTT_TX_RX_PDEV_STATS_AX_RU_SIZE_996x2,
    ATH12K_HTT_TX_RX_PDEV_STATS_NUM_AX_RU_SIZE_CNTRS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_rx_pdev_ul_ofdma_user_stats_tlv {
    pub user_index: __le32,
    pub rx_ulofdma_non_data_ppdu: __le32,
    pub rx_ulofdma_data_ppdu: __le32,
    pub rx_ulofdma_mpdu_ok: __le32,
    pub rx_ulofdma_mpdu_fail: __le32,
    pub rx_ulofdma_non_data_nusers: __le32,
    pub rx_ulofdma_data_nusers: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_rx_pdev_ul_trigger_stats_tlv {
    pub mac_id__word: __le32,
    pub rx_11ax_ul_ofdma: __le32,
    pub ul_ofdma_rx_mcs: [__le32; ATH12K_HTT_RX_NUM_MCS_CNTRS],
    pub ul_ofdma_rx_gi: [__le32; ATH12K_HTT_RX_NUM_GI_CNTRS][ATH12K_HTT_RX_NUM_MCS_CNTRS],
    pub ul_ofdma_rx_nss: [__le32; ATH12K_HTT_RX_NUM_SPATIAL_STREAMS],
    pub ul_ofdma_rx_bw: [__le32; ATH12K_HTT_RX_NUM_BW_CNTRS],
    pub ul_ofdma_rx_stbc: __le32,
    pub ul_ofdma_rx_ldpc: __le32,
    pub data_ru_size_ppdu: [__le32; ATH12K_HTT_RX_NUM_RU_SIZE_160MHZ_CNTRS],
    pub non_data_ru_size_ppdu: [__le32; ATH12K_HTT_RX_NUM_RU_SIZE_160MHZ_CNTRS],
    pub uplink_sta_aid: [__le32; ATH12K_HTT_RX_UL_MAX_UPLINK_RSSI_TRACK],
    pub uplink_sta_target_rssi: [__le32; ATH12K_HTT_RX_UL_MAX_UPLINK_RSSI_TRACK],
    pub uplink_sta_fd_rssi: [__le32; ATH12K_HTT_RX_UL_MAX_UPLINK_RSSI_TRACK],
    pub uplink_sta_power_headroom: [__le32; ATH12K_HTT_RX_UL_MAX_UPLINK_RSSI_TRACK],
    pub red_bw: [__le32; ATH12K_HTT_RX_NUM_REDUCED_CHAN_TYPES][ATH12K_HTT_RX_NUM_BW_CNTRS],
    pub ul_ofdma_bsc_trig_rx_qos_null_only: __le32,
    pub __packed: },
pub const ATH12K_HTT_TX_UL_MUMIMO_USER_STATS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_rx_ul_mumimo_trig_stats_tlv {
    pub mac_id__word: __le32,
    pub rx_11ax_ul_mumimo: __le32,
    pub ul_mumimo_rx_mcs: [__le32; ATH12K_HTT_RX_NUM_MCS_CNTRS],
    pub ul_rx_gi: [__le32; ATH12K_HTT_RX_NUM_GI_CNTRS][ATH12K_HTT_RX_NUM_MCS_CNTRS],
    pub ul_mumimo_rx_nss: [__le32; ATH12K_HTT_RX_NUM_SPATIAL_STREAMS],
    pub ul_mumimo_rx_bw: [__le32; ATH12K_HTT_RX_NUM_BW_CNTRS],
    pub ul_mumimo_rx_stbc: __le32,
    pub ul_mumimo_rx_ldpc: __le32,
    pub ul_mumimo_rx_mcs_ext: [__le32; ATH12K_HTT_RX_NUM_EXTRA_MCS_CNTRS],
    pub ul_gi_ext: [__le32; ATH12K_HTT_RX_NUM_GI_CNTRS][ATH12K_HTT_RX_NUM_EXTRA_MCS_CNTRS],
    pub ul_rssi: [i8; ATH12K_HTT_RX_NUM_SPATIAL_STREAMS][ATH12K_HTT_RX_NUM_BW_CNTRS],
    pub tgt_rssi: [i8; ATH12K_HTT_TX_UL_MUMIMO_USER_STATS][ATH12K_HTT_RX_NUM_BW_CNTRS],
    pub fd: [i8; ATH12K_HTT_TX_UL_MUMIMO_USER_STATS][ATH12K_HTT_RX_NUM_SPATIAL_STREAMS],
    pub db: [i8; ATH12K_HTT_TX_UL_MUMIMO_USER_STATS][ATH12K_HTT_RX_NUM_SPATIAL_STREAMS],
    pub red_bw: [__le32; ATH12K_HTT_RX_NUM_REDUCED_CHAN_TYPES][ATH12K_HTT_RX_NUM_BW_CNTRS],
    pub mumimo_bsc_trig_rx_qos_null_only: __le32,
    pub __packed: },
pub const ATH12K_HTT_RX_NUM_MAX_PEAK_OCCUPANCY_INDEX: c_int = 10;
pub const ATH12K_HTT_RX_NUM_MAX_CURR_OCCUPANCY_INDEX: c_int = 10;
pub const ATH12K_HTT_RX_NUM_SQUARE_INDEX: c_int = 6;
pub const ATH12K_HTT_RX_NUM_MAX_PEAK_SEARCH_INDEX: c_int = 4;
pub const ATH12K_HTT_RX_NUM_MAX_PENDING_SEARCH_INDEX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_rx_fse_stats_tlv {
    pub fse_enable_cnt: __le32,
    pub fse_disable_cnt: __le32,
    pub fse_cache_invalidate_entry_cnt: __le32,
    pub fse_full_cache_invalidate_cnt: __le32,
    pub fse_num_cache_hits_cnt: __le32,
    pub fse_num_searches_cnt: __le32,
    pub fse_cache_occupancy_peak_cnt: [__le32; ATH12K_HTT_RX_NUM_MAX_PEAK_OCCUPANCY_INDEX],
    pub fse_cache_occupancy_curr_cnt: [__le32; ATH12K_HTT_RX_NUM_MAX_CURR_OCCUPANCY_INDEX],
    pub fse_search_stat_square_cnt: [__le32; ATH12K_HTT_RX_NUM_SQUARE_INDEX],
    pub fse_search_stat_peak_cnt: [__le32; ATH12K_HTT_RX_NUM_MAX_PEAK_SEARCH_INDEX],
    pub fse_search_stat_pending_cnt: [__le32; ATH12K_HTT_RX_NUM_MAX_PENDING_SEARCH_INDEX],
    pub __packed: },
pub const ATH12K_HTT_TX_BF_RATE_STATS_NUM_MCS_COUNTERS: c_int = 14;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_LEGACY_OFDM_STATS: c_int = 8;
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS: c_int = 8;
pub const ATH12K_HTT_TXBF_NUM_BW_CNTRS: c_int = 5;
pub const ATH12K_HTT_TXBF_NUM_REDUCED_CHAN_TYPES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_txrate_txbf_stats_tlv {
    pub tx_su_txbf_mcs: [__le32; ATH12K_HTT_TX_BF_RATE_STATS_NUM_MCS_COUNTERS],
    pub tx_su_ibf_mcs: [__le32; ATH12K_HTT_TX_BF_RATE_STATS_NUM_MCS_COUNTERS],
    pub tx_su_ol_mcs: [__le32; ATH12K_HTT_TX_BF_RATE_STATS_NUM_MCS_COUNTERS],
    pub tx_su_txbf_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub tx_su_ibf_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub tx_su_ol_nss: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub tx_su_txbf_bw: [__le32; ATH12K_HTT_TXBF_NUM_BW_CNTRS],
    pub tx_su_ibf_bw: [__le32; ATH12K_HTT_TXBF_NUM_BW_CNTRS],
    pub tx_su_ol_bw: [__le32; ATH12K_HTT_TXBF_NUM_BW_CNTRS],
    pub tx_legacy_ofdm_rate: [__le32; ATH12K_HTT_TX_PDEV_STATS_NUM_LEGACY_OFDM_STATS],
    pub txbf: [__le32; ATH12K_HTT_TXBF_NUM_REDUCED_CHAN_TYPES][ATH12K_HTT_TXBF_NUM_BW_CNTRS],
    pub ibf: [__le32; ATH12K_HTT_TXBF_NUM_REDUCED_CHAN_TYPES][ATH12K_HTT_TXBF_NUM_BW_CNTRS],
    pub ol: [__le32; ATH12K_HTT_TXBF_NUM_REDUCED_CHAN_TYPES][ATH12K_HTT_TXBF_NUM_BW_CNTRS],
    pub txbf_flag_set_mu_mode: __le32,
    pub txbf_flag_set_final_status: __le32,
    pub txbf_flag_not_set_verified_txbf_mode: __le32,
    pub txbf_flag_not_set_disable_p2p_access: __le32,
    pub txbf_flag_not_set_max_nss_in_he160: __le32,
    pub txbf_flag_not_set_disable_uldlofdma: __le32,
    pub txbf_flag_not_set_mcs_threshold_val: __le32,
    pub txbf_flag_not_set_final_status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_ndpa_stats_elem_t {
    pub ax_ofdma_ndpa_queued: __le32,
    pub ax_ofdma_ndpa_tried: __le32,
    pub ax_ofdma_ndpa_flush: __le32,
    pub ax_ofdma_ndpa_err: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_ndpa_stats_tlv {
    pub num_elems_ax_ndpa_arr: __le32,
    pub arr_elem_size_ax_ndpa: __le32,
    pub ax_ndpa): DECLARE_FLEX_ARRAY(struct ath12k_htt_txbf_ofdma_ax_ndpa_stats_elem_t,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_ndp_stats_elem_t {
    pub ax_ofdma_ndp_queued: __le32,
    pub ax_ofdma_ndp_tried: __le32,
    pub ax_ofdma_ndp_flush: __le32,
    pub ax_ofdma_ndp_err: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_ndp_stats_tlv {
    pub num_elems_ax_ndp_arr: __le32,
    pub arr_elem_size_ax_ndp: __le32,
    pub ax_ndp): DECLARE_FLEX_ARRAY(struct ath12k_htt_txbf_ofdma_ax_ndp_stats_elem_t,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_brp_stats_elem_t {
    pub ax_ofdma_brp_queued: __le32,
    pub ax_ofdma_brp_tried: __le32,
    pub ax_ofdma_brp_flushed: __le32,
    pub ax_ofdma_brp_err: __le32,
    pub ax_ofdma_num_cbf_rcvd: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_brp_stats_tlv {
    pub num_elems_ax_brp_arr: __le32,
    pub arr_elem_size_ax_brp: __le32,
    pub ax_brp): DECLARE_FLEX_ARRAY(struct ath12k_htt_txbf_ofdma_ax_brp_stats_elem_t,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_steer_stats_elem_t {
    pub num_ppdu_steer: __le32,
    pub num_ppdu_ol: __le32,
    pub num_usr_prefetch: __le32,
    pub num_usr_sound: __le32,
    pub num_usr_force_sound: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_steer_stats_tlv {
    pub num_elems_ax_steer_arr: __le32,
    pub arr_elem_size_ax_steer: __le32,
    pub ax_steer): DECLARE_FLEX_ARRAY(struct ath12k_htt_txbf_ofdma_ax_steer_stats_elem_t,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_txbf_ofdma_ax_steer_mpdu_stats_tlv {
    pub ax_ofdma_rbo_steer_mpdus_tried: __le32,
    pub ax_ofdma_rbo_steer_mpdus_failed: __le32,
    pub ax_ofdma_sifs_steer_mpdus_tried: __le32,
    pub ax_ofdma_sifs_steer_mpdus_failed: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_page_lock_state {
    ATH12K_HTT_STATS_PAGE_LOCKED	= 0,
    ATH12K_HTT_STATS_PAGE_UNLOCKED	= 1,
    ATH12K_NUM_PG_LOCK_STATE
}

pub const ATH12K_PAGER_MAX: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pgs_info {
    pub page_num: __le32,
    pub num_pgs: __le32,
    pub ts_lsb: __le32,
    pub ts_msb: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_dl_pager_stats_tlv {
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub pgs_info: [ath12k_htt_pgs_info; ATH12K_NUM_PG_LOCK_STATE][ATH12K_PAGER_MAX],
    pub __packed: },
pub const ATH12K_HTT_STATS_MAX_CHAINS: c_int = 8;
pub const ATH12K_HTT_MAX_RX_PKT_CNT: c_int = 8;
pub const ATH12K_HTT_MAX_RX_PKT_CRC_PASS_CNT: c_int = 8;
pub const ATH12K_HTT_MAX_PER_BLK_ERR_CNT: c_int = 20;
pub const ATH12K_HTT_MAX_RX_OTA_ERR_CNT: c_int = 14;
pub const ATH12K_HTT_MAX_CH_PWR_INFO_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_phy_stats_tlv {
    pub nf_chain: [a_sle32; ATH12K_HTT_STATS_MAX_CHAINS],
    pub false_radar_cnt: __le32,
    pub radar_cs_cnt: __le32,
    pub ani_level: a_sle32,
    pub fw_run_time: __le32,
    pub runtime_nf_chain: [a_sle32; ATH12K_HTT_STATS_MAX_CHAINS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_phy_counters_tlv {
    pub rx_ofdma_timing_err_cnt: __le32,
    pub rx_cck_fail_cnt: __le32,
    pub mactx_abort_cnt: __le32,
    pub macrx_abort_cnt: __le32,
    pub phytx_abort_cnt: __le32,
    pub phyrx_abort_cnt: __le32,
    pub phyrx_defer_abort_cnt: __le32,
    pub rx_gain_adj_lstf_event_cnt: __le32,
    pub rx_gain_adj_non_legacy_cnt: __le32,
    pub rx_pkt_cnt: [__le32; ATH12K_HTT_MAX_RX_PKT_CNT],
    pub rx_pkt_crc_pass_cnt: [__le32; ATH12K_HTT_MAX_RX_PKT_CRC_PASS_CNT],
    pub per_blk_err_cnt: [__le32; ATH12K_HTT_MAX_PER_BLK_ERR_CNT],
    pub rx_ota_err_cnt: [__le32; ATH12K_HTT_MAX_RX_OTA_ERR_CNT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_phy_reset_stats_tlv {
    pub pdev_id: __le32,
    pub chan_mhz: __le32,
    pub chan_band_center_freq1: __le32,
    pub chan_band_center_freq2: __le32,
    pub chan_phy_mode: __le32,
    pub chan_flags: __le32,
    pub chan_num: __le32,
    pub reset_cause: __le32,
    pub prev_reset_cause: __le32,
    pub phy_warm_reset_src: __le32,
    pub rx_gain_tbl_mode: __le32,
    pub xbar_val: __le32,
    pub force_calibration: __le32,
    pub phyrf_mode: __le32,
    pub phy_homechan: __le32,
    pub phy_tx_ch_mask: __le32,
    pub phy_rx_ch_mask: __le32,
    pub phybb_ini_mask: __le32,
    pub phyrf_ini_mask: __le32,
    pub phy_dfs_en_mask: __le32,
    pub phy_sscan_en_mask: __le32,
    pub phy_synth_sel_mask: __le32,
    pub phy_adfs_freq: __le32,
    pub cck_fir_settings: __le32,
    pub phy_dyn_pri_chan: __le32,
    pub cca_thresh: __le32,
    pub dyn_cca_status: __le32,
    pub rxdesense_thresh_hw: __le32,
    pub rxdesense_thresh_sw: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_phy_reset_counters_tlv {
    pub pdev_id: __le32,
    pub cf_active_low_fail_cnt: __le32,
    pub cf_active_low_pass_cnt: __le32,
    pub phy_off_through_vreg_cnt: __le32,
    pub force_calibration_cnt: __le32,
    pub rf_mode_switch_phy_off_cnt: __le32,
    pub temperature_recal_cnt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_phy_tpc_stats_tlv {
    pub pdev_id: __le32,
    pub tx_power_scale: __le32,
    pub tx_power_scale_db: __le32,
    pub min_negative_tx_power: __le32,
    pub reg_ctl_domain: __le32,
    pub max_reg_allowed_power: [__le32; ATH12K_HTT_STATS_MAX_CHAINS],
    pub max_reg_allowed_power_6ghz: [__le32; ATH12K_HTT_STATS_MAX_CHAINS],
    pub twice_max_rd_power: __le32,
    pub max_tx_power: __le32,
    pub home_max_tx_power: __le32,
    pub psd_power: __le32,
    pub eirp_power: __le32,
    pub power_type_6ghz: __le32,
    pub sub_band_cfreq: [__le32; ATH12K_HTT_MAX_CH_PWR_INFO_SIZE],
    pub sub_band_txpower: [__le32; ATH12K_HTT_MAX_CH_PWR_INFO_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_t2h_soc_txrx_stats_common_tlv {
    pub inv_peers_msdu_drop_count_hi: __le32,
    pub inv_peers_msdu_drop_count_lo: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_ast_entry_tlv {
    pub sw_peer_id: __le32,
    pub ast_index: __le32,
    pub mac_addr: htt_mac_addr,
    pub info: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_direction {
    ATH12K_HTT_STATS_DIRECTION_TX,
    ATH12K_HTT_STATS_DIRECTION_RX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_ppdu_type {
    ATH12K_HTT_STATS_PPDU_TYPE_MODE_SU,
    ATH12K_HTT_STATS_PPDU_TYPE_DL_MU_MIMO,
    ATH12K_HTT_STATS_PPDU_TYPE_UL_MU_MIMO,
    ATH12K_HTT_STATS_PPDU_TYPE_DL_MU_OFDMA,
    ATH12K_HTT_STATS_PPDU_TYPE_UL_MU_OFDMA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_param_type {
    ATH12K_HTT_STATS_PREAM_OFDM,
    ATH12K_HTT_STATS_PREAM_CCK,
    ATH12K_HTT_STATS_PREAM_HT,
    ATH12K_HTT_STATS_PREAM_VHT,
    ATH12K_HTT_STATS_PREAM_HE,
    ATH12K_HTT_STATS_PREAM_EHT,
    ATH12K_HTT_STATS_PREAM_RSVD1,
    ATH12K_HTT_STATS_PREAM_COUNT,
}

pub const ATH12K_HTT_PUNCT_STATS_MAX_SUBBAND_CNT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_puncture_stats_tlv {
    pub mac_id__word: __le32,
    pub direction: __le32,
    pub preamble: __le32,
    pub ppdu_type: __le32,
    pub subband_cnt: __le32,
    pub last_used_pattern_mask: __le32,
    pub num_subbands_used_cnt: [__le32; ATH12K_HTT_PUNCT_STATS_MAX_SUBBAND_CNT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_dmac_reset_stats_tlv {
    pub reset_count: __le32,
    pub reset_time_lo_ms: __le32,
    pub reset_time_hi_ms: __le32,
    pub disengage_time_lo_ms: __le32,
    pub disengage_time_hi_ms: __le32,
    pub engage_time_lo_ms: __le32,
    pub engage_time_hi_ms: __le32,
    pub disengage_count: __le32,
    pub engage_count: __le32,
    pub drain_dest_ring_mask: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_sched_algo_ofdma_stats_tlv {
    pub mac_id__word: __le32,
    pub rate_based_dlofdma_enabled_cnt: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub rate_based_dlofdma_disabled_cnt: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub rate_based_dlofdma_probing_cnt: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub rate_based_dlofdma_monitor_cnt: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub chan_acc_lat_based_dlofdma_enabled_cnt: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub chan_acc_lat_based_dlofdma_disabled_cnt: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub chan_acc_lat_based_dlofdma_monitor_cnt: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub downgrade_to_dl_su_ru_alloc_fail: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub candidate_list_single_user_disable_ofdma: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub dl_cand_list_dropped_high_ul_qos_weight: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub ax_dlofdma_disabled_due_to_pipelining: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub dlofdma_disabled_su_only_eligible: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub dlofdma_disabled_consec_no_mpdus_tried: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub dlofdma_disabled_consec_no_mpdus_success: [__le32; ATH12K_HTT_NUM_AC_WMM],
    pub __packed: },
pub const ATH12K_HTT_TX_PDEV_STATS_NUM_BW_CNTRS: c_int = 4;
pub const ATH12K_HTT_PDEV_STAT_NUM_SPATIAL_STREAMS: c_int = 8;
pub const ATH12K_HTT_TXBF_RATE_STAT_NUM_MCS_CNTRS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE {
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_26,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_52,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_52_26,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_106,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_106_26,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_242,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_484,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_484_242,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996_484,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996_484_242,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996x2,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996x2_484,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996x3,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996x3_484,
    ATH12K_HTT_TX_RX_PDEV_STATS_BE_RU_SIZE_996x4,
    ATH12K_HTT_TX_RX_PDEV_NUM_BE_RU_SIZE_CNTRS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATH12K_HTT_RC_MODE {
    ATH12K_HTT_RC_MODE_SU_OL,
    ATH12K_HTT_RC_MODE_SU_BF,
    ATH12K_HTT_RC_MODE_MU1_INTF,
    ATH12K_HTT_RC_MODE_MU2_INTF,
    ATH12K_HTT_RC_MODE_MU3_INTF,
    ATH12K_HTT_RC_MODE_MU4_INTF,
    ATH12K_HTT_RC_MODE_MU5_INTF,
    ATH12K_HTT_RC_MODE_MU6_INTF,
    ATH12K_HTT_RC_MODE_MU7_INTF,
    ATH12K_HTT_RC_MODE_2D_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_rc_mode {
    ATH12K_HTT_STATS_RC_MODE_DLSU     = 0,
    ATH12K_HTT_STATS_RC_MODE_DLMUMIMO = 1,
    ATH12K_HTT_STATS_RC_MODE_DLOFDMA  = 2,
    ATH12K_HTT_STATS_RC_MODE_ULMUMIMO = 3,
    ATH12K_HTT_STATS_RC_MODE_ULOFDMA  = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_ru_type {
    ATH12K_HTT_STATS_RU_TYPE_INVALID,
    ATH12K_HTT_STATS_RU_TYPE_SINGLE_RU_ONLY,
    ATH12K_HTT_STATS_RU_TYPE_SINGLE_AND_MULTI_RU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_rate_stats {
    pub ppdus_tried: __le32,
    pub ppdus_ack_failed: __le32,
    pub mpdus_tried: __le32,
    pub mpdus_failed: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_per_rate_stats_tlv {
    pub rc_mode: __le32,
    pub last_probed_mcs: __le32,
    pub last_probed_nss: __le32,
    pub last_probed_bw: __le32,
    pub per_bw: [ath12k_htt_tx_rate_stats; ATH12K_HTT_TX_PDEV_STATS_NUM_BW_CNTRS],
    pub per_nss: [ath12k_htt_tx_rate_stats; ATH12K_HTT_PDEV_STAT_NUM_SPATIAL_STREAMS],
    pub per_mcs: [ath12k_htt_tx_rate_stats; ATH12K_HTT_TXBF_RATE_STAT_NUM_MCS_CNTRS],
    pub per_bw320: ath12k_htt_tx_rate_stats,
    pub probe_cnt: [__le32; ATH12K_HTT_RC_MODE_2D_COUNT],
    pub ru_type: __le32,
    pub ru: [ath12k_htt_tx_rate_stats; ATH12K_HTT_TX_RX_PDEV_NUM_BE_RU_SIZE_CNTRS],
    pub __packed: },
pub const ATH12K_HTT_TX_PDEV_NUM_BE_MCS_CNTRS: c_int = 16;
pub const ATH12K_HTT_TX_PDEV_NUM_BE_BW_CNTRS: c_int = 5;
pub const ATH12K_HTT_TX_PDEV_NUM_EHT_SIG_MCS_CNTRS: c_int = 4;
pub const ATH12K_HTT_TX_PDEV_NUM_GI_CNTRS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_tx_pdev_rate_stats_be_ofdma_tlv {
    pub mac_id__word: __le32,
    pub be_ofdma_tx_ldpc: __le32,
    pub be_ofdma_tx_mcs: [__le32; ATH12K_HTT_TX_PDEV_NUM_BE_MCS_CNTRS],
    pub be_ofdma_tx_nss: [__le32; ATH12K_HTT_PDEV_STAT_NUM_SPATIAL_STREAMS],
    pub be_ofdma_tx_bw: [__le32; ATH12K_HTT_TX_PDEV_NUM_BE_BW_CNTRS],
    pub gi: [__le32; ATH12K_HTT_TX_PDEV_NUM_GI_CNTRS][ATH12K_HTT_TX_PDEV_NUM_BE_MCS_CNTRS],
    pub be_ofdma_tx_ru_size: [__le32; ATH12K_HTT_TX_RX_PDEV_NUM_BE_RU_SIZE_CNTRS],
    pub be_ofdma_eht_sig_mcs: [__le32; ATH12K_HTT_TX_PDEV_NUM_EHT_SIG_MCS_CNTRS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_mbssid_ctrl_frame_tlv {
    pub mac_id__word: __le32,
    pub basic_trigger_across_bss: __le32,
    pub basic_trigger_within_bss: __le32,
    pub bsr_trigger_across_bss: __le32,
    pub bsr_trigger_within_bss: __le32,
    pub mu_rts_across_bss: __le32,
    pub mu_rts_within_bss: __le32,
    pub ul_mumimo_trigger_across_bss: __le32,
    pub ul_mumimo_trigger_within_bss: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_tdma_stats_tlv {
    pub mac_id__word: __le32,
    pub num_tdma_active_schedules: __le32,
    pub num_tdma_reserved_schedules: __le32,
    pub num_tdma_restricted_schedules: __le32,
    pub num_tdma_unconfigured_schedules: __le32,
    pub num_tdma_slot_switches: __le32,
    pub num_tdma_edca_switches: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_mlo_sched_stats_tlv {
    pub pref_link_num_sec_link_sched: __le32,
    pub pref_link_num_pref_link_timeout: __le32,
    pub pref_link_num_pref_link_sch_delay_ipc: __le32,
    pub pref_link_num_pref_link_timeout_ipc: __le32,
    pub __packed: },
pub const ATH12K_HTT_HWMLO_MAX_LINKS: c_int = 6;
pub const ATH12K_HTT_MLO_MAX_IPC_RINGS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_pdev_mlo_ipc_stats_tlv {
    pub mlo_ipc_ring_cnt: [__le32; ATH12K_HTT_HWMLO_MAX_LINKS][ATH12K_HTT_MLO_MAX_IPC_RINGS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_stats_pdev_rtt_resp_stats_tlv {
    pub pdev_id: __le32,
    pub tx_11mc_ftm_suc: __le32,
    pub tx_11mc_ftm_suc_retry: __le32,
    pub tx_11mc_ftm_fail: __le32,
    pub rx_11mc_ftmr_cnt: __le32,
    pub rx_11mc_ftmr_dup_cnt: __le32,
    pub rx_11mc_iftmr_cnt: __le32,
    pub rx_11mc_iftmr_dup_cnt: __le32,
    pub ftmr_drop_11mc_resp_role_not_enabled_cnt: __le32,
    pub initiator_active_responder_rejected_cnt: __le32,
    pub responder_terminate_cnt: __le32,
    pub active_rsta_open: __le32,
    pub active_rsta_mac: __le32,
    pub active_rsta_mac_phy: __le32,
    pub num_assoc_ranging_peers: __le32,
    pub num_unassoc_ranging_peers: __le32,
    pub responder_alloc_cnt: __le32,
    pub responder_alloc_failure: __le32,
    pub pn_check_failure_cnt: __le32,
    pub pasn_m1_auth_recv_cnt: __le32,
    pub pasn_m1_auth_drop_cnt: __le32,
    pub pasn_m2_auth_recv_cnt: __le32,
    pub pasn_m2_auth_tx_fail_cnt: __le32,
    pub pasn_m3_auth_recv_cnt: __le32,
    pub pasn_m3_auth_drop_cnt: __le32,
    pub pasn_peer_create_request_cnt: __le32,
    pub pasn_peer_create_timeout_cnt: __le32,
    pub pasn_peer_created_cnt: __le32,
    pub sec_ranging_not_supported_mfp_not_setup: __le32,
    pub non_sec_ranging_discarded_for_assoc_peer: __le32,
    pub open_ranging_discarded_set_for_pasn_peer: __le32,
    pub unassoc_non_pasn_ranging_not_supported: __le32,
    pub num_req_bw_20_mhz: __le32,
    pub num_req_bw_40_mhz: __le32,
    pub num_req_bw_80_mhz: __le32,
    pub num_req_bw_160_mhz: __le32,
    pub tx_11az_ftm_successful: __le32,
    pub tx_11az_ftm_failed: __le32,
    pub rx_11az_ftmr_cnt: __le32,
    pub rx_11az_ftmr_dup_cnt: __le32,
    pub rx_11az_iftmr_dup_cnt: __le32,
    pub malformed_ftmr: __le32,
    pub ftmr_drop_ntb_resp_role_not_enabled_cnt: __le32,
    pub ftmr_drop_tb_resp_role_not_enabled_cnt: __le32,
    pub invalid_ftm_request_params: __le32,
    pub requested_bw_format_not_supported: __le32,
    pub ntb_unsec_unassoc_ranging_peer_alloc_failed: __le32,
    pub tb_unassoc_unsec_pasn_peer_creation_failed: __le32,
    pub num_ranging_sequences_processed: __le32,
    pub ntb_tx_ndp: __le32,
    pub ndp_rx_cnt: __le32,
    pub num_ntb_ranging_ndpas_recv: __le32,
    pub recv_lmr: __le32,
    pub invalid_ftmr_cnt: __le32,
    pub max_time_bw_meas_exp_cnt: __le32,
    pub __packed: },
pub const ATH12K_HTT_MAX_SCH_CMD_RESULT: c_int = 25;
pub const ATH12K_HTT_SCH_CMD_STATUS_CNT: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_stats_pdev_rtt_init_stats_tlv {
    pub pdev_id: __le32,
    pub tx_11mc_ftmr_cnt: __le32,
    pub tx_11mc_ftmr_fail: __le32,
    pub tx_11mc_ftmr_suc_retry: __le32,
    pub rx_11mc_ftm_cnt: __le32,
    pub tx_meas_req_count: __le32,
    pub init_role_not_enabled: __le32,
    pub initiator_terminate_cnt: __le32,
    pub tx_11az_ftmr_fail: __le32,
    pub tx_11az_ftmr_start: __le32,
    pub tx_11az_ftmr_stop: __le32,
    pub rx_11az_ftm_cnt: __le32,
    pub active_ista: __le32,
    pub invalid_preamble: __le32,
    pub invalid_chan_bw_format: __le32,
    pub mgmt_buff_alloc_fail_cnt: __le32,
    pub ftm_parse_failure: __le32,
    pub ranging_negotiation_successful_cnt: __le32,
    pub incompatible_ftm_params: __le32,
    pub sec_ranging_req_in_open_mode: __le32,
    pub ftmr_tx_failed_null_11az_peer: __le32,
    pub ftmr_retry_timeout: __le32,
    pub max_time_bw_meas_exp_cnt: __le32,
    pub tb_meas_duration_expiry_cnt: __le32,
    pub num_tb_ranging_requests: __le32,
    pub ntbr_triggered_successfully: __le32,
    pub ntbr_trigger_failed: __le32,
    pub invalid_or_no_vreg_idx: __le32,
    pub set_vreg_params_failed: __le32,
    pub sac_mismatch: __le32,
    pub pasn_m1_auth_recv_cnt: __le32,
    pub pasn_m1_auth_tx_fail_cnt: __le32,
    pub pasn_m2_auth_recv_cnt: __le32,
    pub pasn_m2_auth_drop_cnt: __le32,
    pub pasn_m3_auth_recv_cnt: __le32,
    pub pasn_m3_auth_tx_fail_cnt: __le32,
    pub pasn_peer_create_request_cnt: __le32,
    pub pasn_peer_create_timeout_cnt: __le32,
    pub pasn_peer_created_cnt: __le32,
    pub ntbr_ndpa_failed: __le32,
    pub ntbr_sequence_successful: __le32,
    pub ntbr_ndp_failed: __le32,
    pub sch_cmd_status_cnts: [__le32; ATH12K_HTT_SCH_CMD_STATUS_CNT],
    pub lmr_timeout: __le32,
    pub lmr_recv: __le32,
    pub num_trigger_frames_received: __le32,
    pub num_tb_ranging_ndpas_recv: __le32,
    pub ndp_rx_cnt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_stats_pdev_rtt_hw_stats_tlv {
    pub ista_ranging_ndpa_cnt: __le32,
    pub ista_ranging_ndp_cnt: __le32,
    pub ista_ranging_i2r_lmr_cnt: __le32,
    pub rtsa_ranging_resp_cnt: __le32,
    pub rtsa_ranging_ndp_cnt: __le32,
    pub rsta_ranging_lmr_cnt: __le32,
    pub tb_ranging_cts2s_rcvd_cnt: __le32,
    pub tb_ranging_ndp_rcvd_cnt: __le32,
    pub tb_ranging_lmr_rcvd_cnt: __le32,
    pub tb_ranging_tf_poll_resp_sent_cnt: __le32,
    pub tb_ranging_tf_sound_resp_sent_cnt: __le32,
    pub tb_ranging_tf_report_resp_sent_cnt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htt_stats_txsend_ftype {
    ATH12K_HTT_FTYPE_TF_POLL,
    ATH12K_HTT_FTYPE_TF_SOUND,
    ATH12K_HTT_FTYPE_TBR_NDPA,
    ATH12K_HTT_FTYPE_TBR_NDP,
    ATH12K_HTT_FTYPE_TBR_LMR,
    ATH12K_HTT_FTYPE_TF_RPRT,
    ATH12K_HTT_FTYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_stats_pdev_rtt_tbr_tlv {
    pub su_ftype: [__le32; ATH12K_HTT_FTYPE_MAX],
    pub mu_ftype: [__le32; ATH12K_HTT_FTYPE_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_stats_pdev_rtt_tbr_cmd_result_stats_tlv {
    pub tbr_num_sch_cmd_result_buckets: __le32,
    pub su_res: [__le32; ATH12K_HTT_FTYPE_MAX][ATH12K_HTT_MAX_SCH_CMD_RESULT],
    pub mu_res: [__le32; ATH12K_HTT_FTYPE_MAX][ATH12K_HTT_MAX_SCH_CMD_RESULT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_pdev_fw_stats_tlv {
    pub mac_id__word: __le32,
    pub ppdu_recvd: __le32,
    pub mpdu_cnt_fcs_ok: __le32,
    pub mpdu_cnt_fcs_err: __le32,
    pub tcp_msdu_cnt: __le32,
    pub tcp_ack_msdu_cnt: __le32,
    pub udp_msdu_cnt: __le32,
    pub other_msdu_cnt: __le32,
    pub fw_ring_mpdu_ind: __le32,
    pub fw_ring_mgmt_subtype: [__le32; ATH12K_HTT_STATS_SUBTYPE_MAX],
    pub fw_ring_ctrl_subtype: [__le32; ATH12K_HTT_STATS_SUBTYPE_MAX],
    pub fw_ring_mcast_data_msdu: __le32,
    pub fw_ring_bcast_data_msdu: __le32,
    pub fw_ring_ucast_data_msdu: __le32,
    pub fw_ring_null_data_msdu: __le32,
    pub fw_ring_mpdu_drop: __le32,
    pub ofld_local_data_ind_cnt: __le32,
    pub ofld_local_data_buf_recycle_cnt: __le32,
    pub drx_local_data_ind_cnt: __le32,
    pub drx_local_data_buf_recycle_cnt: __le32,
    pub local_nondata_ind_cnt: __le32,
    pub local_nondata_buf_recycle_cnt: __le32,
    pub fw_status_buf_ring_refill_cnt: __le32,
    pub fw_status_buf_ring_empty_cnt: __le32,
    pub fw_pkt_buf_ring_refill_cnt: __le32,
    pub fw_pkt_buf_ring_empty_cnt: __le32,
    pub fw_link_buf_ring_refill_cnt: __le32,
    pub fw_link_buf_ring_empty_cnt: __le32,
    pub host_pkt_buf_ring_refill_cnt: __le32,
    pub host_pkt_buf_ring_empty_cnt: __le32,
    pub mon_pkt_buf_ring_refill_cnt: __le32,
    pub mon_pkt_buf_ring_empty_cnt: __le32,
    pub mon_status_buf_ring_refill_cnt: __le32,
    pub mon_status_buf_ring_empty_cnt: __le32,
    pub mon_desc_buf_ring_refill_cnt: __le32,
    pub mon_desc_buf_ring_empty_cnt: __le32,
    pub mon_dest_ring_update_cnt: __le32,
    pub mon_dest_ring_full_cnt: __le32,
    pub rx_suspend_cnt: __le32,
    pub rx_suspend_fail_cnt: __le32,
    pub rx_resume_cnt: __le32,
    pub rx_resume_fail_cnt: __le32,
    pub rx_ring_switch_cnt: __le32,
    pub rx_ring_restore_cnt: __le32,
    pub rx_flush_cnt: __le32,
    pub rx_recovery_reset_cnt: __le32,
    pub rx_lwm_prom_filter_dis: __le32,
    pub rx_hwm_prom_filter_en: __le32,
    pub bytes_received_low_32: __le32,
    pub bytes_received_high_32: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_stats_cmn_tlv {
    pub mac_id__hwq_id__word: __le32,
    pub xretry: __le32,
    pub underrun_cnt: __le32,
    pub flush_cnt: __le32,
    pub filt_cnt: __le32,
    pub null_mpdu_bmap: __le32,
    pub user_ack_failure: __le32,
    pub ack_tlv_proc: __le32,
    pub sched_id_proc: __le32,
    pub null_mpdu_tx_count: __le32,
    pub mpdu_bmap_not_recvd: __le32,
    pub num_bar: __le32,
    pub rts: __le32,
    pub cts2self: __le32,
    pub qos_null: __le32,
    pub mpdu_tried_cnt: __le32,
    pub mpdu_queued_cnt: __le32,
    pub mpdu_ack_fail_cnt: __le32,
    pub mpdu_filt_cnt: __le32,
    pub false_mpdu_ack_count: __le32,
    pub txq_timeout: __le32,
    pub __packed: },
pub const ATH12K_HTT_CHAN_SWITCH_STATS_BUF_LEN: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_chan_switch_stats_tlv {
    pub chan_switch_freq: __le32,
    pub chan_switch_profile: __le32,
    pub chan_switch_time: __le32,
    pub cal_module_time: __le32,
    pub ini_module_time: __le32,
    pub tpc_module_time: __le32,
    pub misc_module_time: __le32,
    pub ctl_module_time: __le32,
    pub reserved: __le32,
    pub chan_stats: [}; ATH12K_HTT_CHAN_SWITCH_STATS_BUF_LEN],
    pub /: *mut *mut __le32 switch_count; / shows how many channel changes have occurred,
    pub __packed: },
