//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/debugfs_htt_stats.h
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

pub const HTT_STATS_MAGIC_VALUE: c_uint = 0xF0F0F0F0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_tlv_tag_t {
    HTT_STATS_TX_PDEV_CMN_TAG                           = 0,
    HTT_STATS_TX_PDEV_UNDERRUN_TAG                      = 1,
    HTT_STATS_TX_PDEV_SIFS_TAG                          = 2,
    HTT_STATS_TX_PDEV_FLUSH_TAG                         = 3,
    HTT_STATS_TX_PDEV_PHY_ERR_TAG                       = 4,
    HTT_STATS_STRING_TAG                                = 5,
    HTT_STATS_TX_HWQ_CMN_TAG                            = 6,
    HTT_STATS_TX_HWQ_DIFS_LATENCY_TAG                   = 7,
    HTT_STATS_TX_HWQ_CMD_RESULT_TAG                     = 8,
    HTT_STATS_TX_HWQ_CMD_STALL_TAG                      = 9,
    HTT_STATS_TX_HWQ_FES_STATUS_TAG                     = 10,
    HTT_STATS_TX_TQM_GEN_MPDU_TAG                       = 11,
    HTT_STATS_TX_TQM_LIST_MPDU_TAG                      = 12,
    HTT_STATS_TX_TQM_LIST_MPDU_CNT_TAG                  = 13,
    HTT_STATS_TX_TQM_CMN_TAG                            = 14,
    HTT_STATS_TX_TQM_PDEV_TAG                           = 15,
    HTT_STATS_TX_TQM_CMDQ_STATUS_TAG                    = 16,
    HTT_STATS_TX_DE_EAPOL_PACKETS_TAG                   = 17,
    HTT_STATS_TX_DE_CLASSIFY_FAILED_TAG                 = 18,
    HTT_STATS_TX_DE_CLASSIFY_STATS_TAG                  = 19,
    HTT_STATS_TX_DE_CLASSIFY_STATUS_TAG                 = 20,
    HTT_STATS_TX_DE_ENQUEUE_PACKETS_TAG                 = 21,
    HTT_STATS_TX_DE_ENQUEUE_DISCARD_TAG                 = 22,
    HTT_STATS_TX_DE_CMN_TAG                             = 23,
    HTT_STATS_RING_IF_TAG                               = 24,
    HTT_STATS_TX_PDEV_MU_MIMO_STATS_TAG                 = 25,
    HTT_STATS_SFM_CMN_TAG                               = 26,
    HTT_STATS_SRING_STATS_TAG                           = 27,
    HTT_STATS_RX_PDEV_FW_STATS_TAG                      = 28,
    HTT_STATS_RX_PDEV_FW_RING_MPDU_ERR_TAG              = 29,
    HTT_STATS_RX_PDEV_FW_MPDU_DROP_TAG                  = 30,
    HTT_STATS_RX_SOC_FW_STATS_TAG                       = 31,
    HTT_STATS_RX_SOC_FW_REFILL_RING_EMPTY_TAG           = 32,
    HTT_STATS_RX_SOC_FW_REFILL_RING_NUM_REFILL_TAG      = 33,
    HTT_STATS_TX_PDEV_RATE_STATS_TAG                    = 34,
    HTT_STATS_RX_PDEV_RATE_STATS_TAG                    = 35,
    HTT_STATS_TX_PDEV_SCHEDULER_TXQ_STATS_TAG           = 36,
    HTT_STATS_TX_SCHED_CMN_TAG                          = 37,
    HTT_STATS_TX_PDEV_MUMIMO_MPDU_STATS_TAG             = 38,
    HTT_STATS_SCHED_TXQ_CMD_POSTED_TAG                  = 39,
    HTT_STATS_RING_IF_CMN_TAG                           = 40,
    HTT_STATS_SFM_CLIENT_USER_TAG                       = 41,
    HTT_STATS_SFM_CLIENT_TAG                            = 42,
    HTT_STATS_TX_TQM_ERROR_STATS_TAG                    = 43,
    HTT_STATS_SCHED_TXQ_CMD_REAPED_TAG                  = 44,
    HTT_STATS_SRING_CMN_TAG                             = 45,
    HTT_STATS_TX_SELFGEN_AC_ERR_STATS_TAG               = 46,
    HTT_STATS_TX_SELFGEN_CMN_STATS_TAG                  = 47,
    HTT_STATS_TX_SELFGEN_AC_STATS_TAG                   = 48,
    HTT_STATS_TX_SELFGEN_AX_STATS_TAG                   = 49,
    HTT_STATS_TX_SELFGEN_AX_ERR_STATS_TAG               = 50,
    HTT_STATS_TX_HWQ_MUMIMO_SCH_STATS_TAG               = 51,
    HTT_STATS_TX_HWQ_MUMIMO_MPDU_STATS_TAG              = 52,
    HTT_STATS_TX_HWQ_MUMIMO_CMN_STATS_TAG               = 53,
    HTT_STATS_HW_INTR_MISC_TAG                          = 54,
    HTT_STATS_HW_WD_TIMEOUT_TAG                         = 55,
    HTT_STATS_HW_PDEV_ERRS_TAG                          = 56,
    HTT_STATS_COUNTER_NAME_TAG                          = 57,
    HTT_STATS_TX_TID_DETAILS_TAG                        = 58,
    HTT_STATS_RX_TID_DETAILS_TAG                        = 59,
    HTT_STATS_PEER_STATS_CMN_TAG                        = 60,
    HTT_STATS_PEER_DETAILS_TAG                          = 61,
    HTT_STATS_PEER_TX_RATE_STATS_TAG                    = 62,
    HTT_STATS_PEER_RX_RATE_STATS_TAG                    = 63,
    HTT_STATS_PEER_MSDU_FLOWQ_TAG                       = 64,
    HTT_STATS_TX_DE_COMPL_STATS_TAG                     = 65,
    HTT_STATS_WHAL_TX_TAG                               = 66,
    HTT_STATS_TX_PDEV_SIFS_HIST_TAG                     = 67,
    HTT_STATS_RX_PDEV_FW_STATS_PHY_ERR_TAG              = 68,
    HTT_STATS_TX_TID_DETAILS_V1_TAG                     = 69,
    HTT_STATS_PDEV_CCA_1SEC_HIST_TAG                    = 70,
    HTT_STATS_PDEV_CCA_100MSEC_HIST_TAG                 = 71,
    HTT_STATS_PDEV_CCA_STAT_CUMULATIVE_TAG              = 72,
    HTT_STATS_PDEV_CCA_COUNTERS_TAG                     = 73,
    HTT_STATS_TX_PDEV_MPDU_STATS_TAG                    = 74,
    HTT_STATS_PDEV_TWT_SESSIONS_TAG                     = 75,
    HTT_STATS_PDEV_TWT_SESSION_TAG                      = 76,
    HTT_STATS_RX_REFILL_RXDMA_ERR_TAG                   = 77,
    HTT_STATS_RX_REFILL_REO_ERR_TAG                     = 78,
    HTT_STATS_RX_REO_RESOURCE_STATS_TAG                 = 79,
    HTT_STATS_TX_SOUNDING_STATS_TAG                     = 80,
    HTT_STATS_TX_PDEV_TX_PPDU_STATS_TAG                 = 81,
    HTT_STATS_TX_PDEV_TRIED_MPDU_CNT_HIST_TAG           = 82,
    HTT_STATS_TX_HWQ_TRIED_MPDU_CNT_HIST_TAG            = 83,
    HTT_STATS_TX_HWQ_TXOP_USED_CNT_HIST_TAG             = 84,
    HTT_STATS_TX_DE_FW2WBM_RING_FULL_HIST_TAG           = 85,
    HTT_STATS_SCHED_TXQ_SCHED_ORDER_SU_TAG              = 86,
    HTT_STATS_SCHED_TXQ_SCHED_INELIGIBILITY_TAG         = 87,
    HTT_STATS_PDEV_OBSS_PD_TAG                          = 88,
    HTT_STATS_HW_WAR_TAG				    = 89,
    HTT_STATS_RING_BACKPRESSURE_STATS_TAG		    = 90,
    HTT_STATS_PEER_CTRL_PATH_TXRX_STATS_TAG		    = 101,
    HTT_STATS_PDEV_TX_RATE_TXBF_STATS_TAG		    = 108,
    HTT_STATS_TXBF_OFDMA_NDPA_STATS_TAG		    = 113,
    HTT_STATS_TXBF_OFDMA_NDP_STATS_TAG		    = 114,
    HTT_STATS_TXBF_OFDMA_BRP_STATS_TAG		    = 115,
    HTT_STATS_TXBF_OFDMA_STEER_STATS_TAG		    = 116,
    HTT_STATS_PHY_COUNTERS_TAG			    = 121,
    HTT_STATS_PHY_STATS_TAG				    = 122,
    HTT_STATS_PHY_RESET_COUNTERS_TAG		    = 123,
    HTT_STATS_PHY_RESET_STATS_TAG			    = 124,

    HTT_STATS_MAX_TAG,
}

pub const HTT_STATS_MAX_STRING_SZ32: c_int = 4;
pub const HTT_STATS_MACID_INVALID: c_uint = 0xff;
pub const HTT_TX_HWQ_MAX_DIFS_LATENCY_BINS: c_int = 10;
pub const HTT_TX_HWQ_MAX_CMD_RESULT_STATS: c_int = 13;
pub const HTT_TX_HWQ_MAX_CMD_STALL_STATS: c_int = 5;
pub const HTT_TX_HWQ_MAX_FES_RESULT_STATS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_tx_pdev_underrun_enum {
    HTT_STATS_TX_PDEV_NO_DATA_UNDERRUN           = 0,
    HTT_STATS_TX_PDEV_DATA_UNDERRUN_BETWEEN_MPDU = 1,
    HTT_STATS_TX_PDEV_DATA_UNDERRUN_WITHIN_MPDU  = 2,
    HTT_TX_PDEV_MAX_URRN_STATS                   = 3,
}

pub const HTT_TX_PDEV_MAX_FLUSH_REASON_STATS: c_int = 71;
pub const HTT_TX_PDEV_MAX_SIFS_BURST_STATS: c_int = 9;
pub const HTT_TX_PDEV_MAX_SIFS_BURST_HIST_STATS: c_int = 10;
pub const HTT_TX_PDEV_MAX_PHY_ERR_STATS: c_int = 18;
pub const HTT_TX_PDEV_SCHED_TX_MODE_MAX: c_int = 4;
pub const HTT_TX_PDEV_NUM_SCHED_ORDER_LOG: c_int = 20;
pub const HTT_RX_STATS_REFILL_MAX_RING: c_int = 4;
pub const HTT_RX_STATS_RXDMA_MAX_ERR: c_int = 16;
pub const HTT_RX_STATS_FW_DROP_REASON_MAX: c_int = 16;
// Bytes stored in little endian order
// Length should be multiple of DWORD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_stats_string_tlv {
// Can be variable length
    pub data): DECLARE_FLEX_ARRAY(u32,,
    pub __packed: },

// == TX PDEV STATS ==
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_cmn_tlv {
    pub mac_id__word: u32,
    pub hw_queued: u32,
    pub hw_reaped: u32,
    pub underrun: u32,
    pub hw_paused: u32,
    pub hw_flush: u32,
    pub hw_filt: u32,
    pub tx_abort: u32,
    pub mpdu_requeued: u32,
    pub tx_xretry: u32,
    pub data_rc: u32,
    pub mpdu_dropped_xretry: u32,
    pub illgl_rate_phy_err: u32,
    pub cont_xretry: u32,
    pub tx_timeout: u32,
    pub pdev_resets: u32,
    pub phy_underrun: u32,
    pub txop_ovf: u32,
    pub seq_posted: u32,
    pub seq_failed_queueing: u32,
    pub seq_completed: u32,
    pub seq_restarted: u32,
    pub mu_seq_posted: u32,
    pub seq_switch_hw_paused: u32,
    pub next_seq_posted_dsr: u32,
    pub seq_posted_isr: u32,
    pub seq_ctrl_cached: u32,
    pub mpdu_count_tqm: u32,
    pub msdu_count_tqm: u32,
    pub mpdu_removed_tqm: u32,
    pub msdu_removed_tqm: u32,
    pub mpdus_sw_flush: u32,
    pub mpdus_hw_filter: u32,
    pub mpdus_truncated: u32,
    pub mpdus_ack_failed: u32,
    pub mpdus_expired: u32,
    pub mpdus_seq_hw_retry: u32,
    pub ack_tlv_proc: u32,
    pub coex_abort_mpdu_cnt_valid: u32,
    pub coex_abort_mpdu_cnt: u32,
    pub num_total_ppdus_tried_ota: u32,
    pub num_data_ppdus_tried_ota: u32,
    pub local_ctrl_mgmt_enqued: u32,
    pub local_ctrl_mgmt_freed: u32,
    pub local_data_enqued: u32,
    pub local_data_freed: u32,
    pub mpdu_tried: u32,
    pub isr_wait_seq_posted: u32,
    pub tx_active_dur_us_low: u32,
    pub tx_active_dur_us_high: u32,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_urrn_tlv_v {
// HTT_TX_PDEV_MAX_URRN_STATS
    pub urrn_stats): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_flush_tlv_v {
// HTT_TX_PDEV_MAX_FLUSH_REASON_STATS
    pub flush_errs): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_sifs_tlv_v {
// HTT_TX_PDEV_MAX_SIFS_BURST_STATS
    pub sifs_status): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_phy_err_tlv_v {
// HTT_TX_PDEV_MAX_PHY_ERR_STATS
    pub phy_errs): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_sifs_hist_tlv_v {
// HTT_TX_PDEV_SIFS_BURST_HIST_STATS
    pub sifs_hist_status): DECLARE_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_tx_ppdu_stats_tlv_v {
    pub num_data_ppdus_legacy_su: u32,
    pub num_data_ppdus_ac_su: u32,
    pub num_data_ppdus_ax_su: u32,
    pub num_data_ppdus_ac_su_txbf: u32,
    pub num_data_ppdus_ax_su_txbf: u32,
}

// NOTE: Variable length TLV, use length spec to infer array size .
//
// Tried_mpdu_cnt_hist is the histogram of MPDUs tries per HWQ.
// The tries here is the count of the  MPDUS within a PPDU that the
// HW had attempted to transmit on  air, for the HWSCH Schedule
// command submitted by FW.It is not the retry attempts.
// The histogram bins are  0-29, 30-59, 60-89 and so on. The are
// 10 bins in this histogram. They are defined in FW using the
// following macros
// #define WAL_MAX_TRIED_MPDU_CNT_HISTOGRAM 9
// #define WAL_TRIED_MPDU_CNT_HISTOGRAM_INTERVAL 30
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_tried_mpdu_cnt_hist_tlv_v {
    pub hist_bin_size: u32,
    pub /: *mut *mut u32 tried_mpdu_cnt_hist[]; / HTT_TX_PDEV_TRIED_MPDU_CNT_HIST,
}

// == SOC ERROR STATS ==
// =============== PDEV ERROR STATS ==============
pub const HTT_STATS_MAX_HW_INTR_NAME_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_hw_stats_intr_misc_tlv {
// Stored as little endian
    pub hw_intr_name: [u8; HTT_STATS_MAX_HW_INTR_NAME_LEN],
    pub mask: u32,
    pub count: u32,
}

pub const HTT_STATS_MAX_HW_MODULE_NAME_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_hw_stats_wd_timeout_tlv {
// Stored as little endian
    pub hw_module_name: [u8; HTT_STATS_MAX_HW_MODULE_NAME_LEN],
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_hw_stats_pdev_errs_tlv {
    pub /: *mut *mut u32 mac_id__word; / BIT [ 7 : 0] : mac_id,
    pub tx_abort: u32,
    pub tx_abort_fail_count: u32,
    pub rx_abort: u32,
    pub rx_abort_fail_count: u32,
    pub warm_reset: u32,
    pub cold_reset: u32,
    pub tx_flush: u32,
    pub tx_glb_reset: u32,
    pub tx_txq_reset: u32,
    pub rx_timeout_reset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_hw_stats_whal_tx_tlv {
    pub mac_id__word: u32,
    pub last_unpause_ppdu_id: u32,
    pub hwsch_unpause_wait_tqm_write: u32,
    pub hwsch_dummy_tlv_skipped: u32,
    pub hwsch_misaligned_offset_received: u32,
    pub hwsch_reset_count: u32,
    pub hwsch_dev_reset_war: u32,
    pub hwsch_delayed_pause: u32,
    pub hwsch_long_delayed_pause: u32,
    pub sch_rx_ppdu_no_response: u32,
    pub sch_selfgen_response: u32,
    pub sch_rx_sifs_resp_trigger: u32,
}

// ============ PEER STATS ============

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_msdu_flow_stats_tlv {
    pub last_update_timestamp: u32,
    pub last_add_timestamp: u32,
    pub last_remove_timestamp: u32,
    pub total_processed_msdu_count: u32,
    pub cur_msdu_count_in_flowq: u32,
    pub sw_peer_id: u32,
    pub tx_flow_no__tid_num__drop_rule: u32,
    pub last_cycle_enqueue_count: u32,
    pub last_cycle_dequeue_count: u32,
    pub last_cycle_drop_count: u32,
    pub current_drop_th: u32,
}

pub const MAX_HTT_TID_NAME: c_int = 8;

// Tidq stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tid_stats_tlv {
// Stored as little endian
    pub tid_name: [u8; MAX_HTT_TID_NAME],
    pub sw_peer_id__tid_num: u32,
    pub num_sched_pending__num_ppdu_in_hwq: u32,
    pub tid_flags: u32,
    pub hw_queued: u32,
    pub hw_reaped: u32,
    pub mpdus_hw_filter: u32,
    pub qdepth_bytes: u32,
    pub qdepth_num_msdu: u32,
    pub qdepth_num_mpdu: u32,
    pub last_scheduled_tsmp: u32,
    pub pause_module_id: u32,
    pub block_module_id: u32,
    pub tid_tx_airtime: u32,
}

// Tidq stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tid_stats_v1_tlv {
// Stored as little endian
    pub tid_name: [u8; MAX_HTT_TID_NAME],
    pub sw_peer_id__tid_num: u32,
    pub num_sched_pending__num_ppdu_in_hwq: u32,
    pub tid_flags: u32,
    pub max_qdepth_bytes: u32,
    pub max_qdepth_n_msdus: u32,
    pub rsvd: u32,
    pub qdepth_bytes: u32,
    pub qdepth_num_msdu: u32,
    pub qdepth_num_mpdu: u32,
    pub last_scheduled_tsmp: u32,
    pub pause_module_id: u32,
    pub block_module_id: u32,
    pub tid_tx_airtime: u32,
    pub allow_n_flags: u32,
    pub sendn_frms_allowed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_tid_stats_tlv {
    pub sw_peer_id__tid_num: u32,
    pub tid_name: [u8; MAX_HTT_TID_NAME],
    pub dup_in_reorder: u32,
    pub dup_past_outside_window: u32,
    pub dup_past_within_window: u32,
    pub rxdesc_err_decrypt: u32,
    pub tid_rx_airtime: u32,
}

pub const HTT_MAX_COUNTER_NAME: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_counter_tlv {
    pub counter_name: [u8; HTT_MAX_COUNTER_NAME],
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_peer_stats_cmn_tlv {
    pub ppdu_cnt: u32,
    pub mpdu_cnt: u32,
    pub msdu_cnt: u32,
    pub pause_bitmap: u32,
    pub block_bitmap: u32,
    pub current_timestamp: u32,
    pub peer_tx_airtime: u32,
    pub peer_rx_airtime: u32,
    pub rssi: i32,
    pub peer_enqueued_count_low: u32,
    pub peer_enqueued_count_high: u32,
    pub peer_dequeued_count_low: u32,
    pub peer_dequeued_count_high: u32,
    pub peer_dropped_count_low: u32,
    pub peer_dropped_count_high: u32,
    pub ppdu_transmitted_bytes_low: u32,
    pub ppdu_transmitted_bytes_high: u32,
    pub peer_ttl_removed_count: u32,
    pub inactive_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_peer_details_tlv {
    pub peer_type: u32,
    pub sw_peer_id: u32,
    pub vdev_pdev_ast_idx: u32,
    pub mac_addr: htt_mac_addr,
    pub peer_flags: u32,
    pub qpeer_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_stats_param_type {
    HTT_STATS_PREAM_OFDM,
    HTT_STATS_PREAM_CCK,
    HTT_STATS_PREAM_HT,
    HTT_STATS_PREAM_VHT,
    HTT_STATS_PREAM_HE,
    HTT_STATS_PREAM_RSVD,
    HTT_STATS_PREAM_RSVD1,

    HTT_STATS_PREAM_COUNT,
}

pub const HTT_TX_PEER_STATS_NUM_MCS_COUNTERS: c_int = 12;
pub const HTT_TX_PEER_STATS_NUM_GI_COUNTERS: c_int = 4;
pub const HTT_TX_PEER_STATS_NUM_DCM_COUNTERS: c_int = 5;
pub const HTT_TX_PEER_STATS_NUM_BW_COUNTERS: c_int = 4;
pub const HTT_TX_PEER_STATS_NUM_SPATIAL_STREAMS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_peer_rate_stats_tlv {
    pub tx_ldpc: u32,
    pub rts_cnt: u32,
    pub ack_rssi: u32,
    pub tx_mcs: [u32; HTT_TX_PEER_STATS_NUM_MCS_COUNTERS],
    pub tx_su_mcs: [u32; HTT_TX_PEER_STATS_NUM_MCS_COUNTERS],
    pub tx_mu_mcs: [u32; HTT_TX_PEER_STATS_NUM_MCS_COUNTERS],
// element 0,1, ...7 -> NSS 1,2, ...8
    pub tx_nss: [u32; HTT_TX_PEER_STATS_NUM_SPATIAL_STREAMS],
// element 0: 20 MHz, 1: 40 MHz, 2: 80 MHz, 3: 160 and 80+80 MHz
    pub tx_bw: [u32; HTT_TX_PEER_STATS_NUM_BW_COUNTERS],
    pub tx_stbc: [u32; HTT_TX_PEER_STATS_NUM_MCS_COUNTERS],
    pub tx_pream: [u32; HTT_TX_PEER_STATS_NUM_PREAMBLE_TYPES],
// Counters to track number of tx packets in each GI
// (400us, 800us, 1600us & 3200us) in each mcs (0-11)
//
    pub tx_gi: [u32; HTT_TX_PEER_STATS_NUM_GI_COUNTERS][HTT_TX_PEER_STATS_NUM_MCS_COUNTERS],
// Counters to track packets in dcm mcs (MCS 0, 1, 3, 4)
    pub tx_dcm: [u32; HTT_TX_PEER_STATS_NUM_DCM_COUNTERS],
}

pub const HTT_RX_PEER_STATS_NUM_MCS_COUNTERS: c_int = 12;
pub const HTT_RX_PEER_STATS_NUM_GI_COUNTERS: c_int = 4;
pub const HTT_RX_PEER_STATS_NUM_DCM_COUNTERS: c_int = 5;
pub const HTT_RX_PEER_STATS_NUM_BW_COUNTERS: c_int = 4;
pub const HTT_RX_PEER_STATS_NUM_SPATIAL_STREAMS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_peer_rate_stats_tlv {
    pub nsts: u32,
// Number of rx ldpc packets
    pub rx_ldpc: u32,
// Number of rx rts packets
    pub rts_cnt: u32,
    pub /: *mut *mut u32 rssi_mgmt; / units = dB above noise floor,
    pub /: *mut *mut u32 rssi_data; / units = dB above noise floor,
    pub /: *mut *mut u32 rssi_comb; / units = dB above noise floor,
    pub rx_mcs: [u32; HTT_RX_PEER_STATS_NUM_MCS_COUNTERS],
// element 0,1, ...7 -> NSS 1,2, ...8
    pub rx_nss: [u32; HTT_RX_PEER_STATS_NUM_SPATIAL_STREAMS],
    pub rx_dcm: [u32; HTT_RX_PEER_STATS_NUM_DCM_COUNTERS],
    pub rx_stbc: [u32; HTT_RX_PEER_STATS_NUM_MCS_COUNTERS],
// element 0: 20 MHz, 1: 40 MHz, 2: 80 MHz, 3: 160 and 80+80 MHz
    pub rx_bw: [u32; HTT_RX_PEER_STATS_NUM_BW_COUNTERS],
    pub rx_pream: [u32; HTT_RX_PEER_STATS_NUM_PREAMBLE_TYPES],
// units = dB above noise floor
// Counters to track number of rx packets in each GI in each mcs (0-11)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_peer_stats_req_mode {
    HTT_PEER_STATS_REQ_MODE_NO_QUERY,
    HTT_PEER_STATS_REQ_MODE_QUERY_TQM,
    HTT_PEER_STATS_REQ_MODE_FLUSH_TQM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_peer_stats_tlv_enum {
    HTT_PEER_STATS_CMN_TLV       = 0,
    HTT_PEER_DETAILS_TLV         = 1,
    HTT_TX_PEER_RATE_STATS_TLV   = 2,
    HTT_RX_PEER_RATE_STATS_TLV   = 3,
    HTT_TX_TID_STATS_TLV         = 4,
    HTT_RX_TID_STATS_TLV         = 5,
    HTT_MSDU_FLOW_STATS_TLV      = 6,

    HTT_PEER_STATS_MAX_TLV       = 31,
}

// =========== MUMIMO HWQ stats ===========
// MU MIMO stats per hwQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_mu_mimo_sch_stats_tlv {
    pub mu_mimo_sch_posted: u32,
    pub mu_mimo_sch_failed: u32,
    pub mu_mimo_ppdu_posted: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_mu_mimo_mpdu_stats_tlv {
    pub mu_mimo_mpdus_queued_usr: u32,
    pub mu_mimo_mpdus_tried_usr: u32,
    pub mu_mimo_mpdus_failed_usr: u32,
    pub mu_mimo_mpdus_requeued_usr: u32,
    pub mu_mimo_err_no_ba_usr: u32,
    pub mu_mimo_mpdu_underrun_usr: u32,
    pub mu_mimo_ampdu_underrun_usr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_mu_mimo_cmn_stats_tlv {
    pub mac_id__hwq_id__word: u32,
}

// == TX HWQ STATS ==
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_stats_cmn_tlv {
    pub mac_id__hwq_id__word: u32,
// PPDU level stats
    pub xretry: u32,
    pub underrun_cnt: u32,
    pub flush_cnt: u32,
    pub filt_cnt: u32,
    pub null_mpdu_bmap: u32,
    pub user_ack_failure: u32,
    pub ack_tlv_proc: u32,
    pub sched_id_proc: u32,
    pub null_mpdu_tx_count: u32,
    pub mpdu_bmap_not_recvd: u32,
// Selfgen stats per hwQ
    pub num_bar: u32,
    pub rts: u32,
    pub cts2self: u32,
    pub qos_null: u32,
// MPDU level stats
    pub mpdu_tried_cnt: u32,
    pub mpdu_queued_cnt: u32,
    pub mpdu_ack_fail_cnt: u32,
    pub mpdu_filt_cnt: u32,
    pub false_mpdu_ack_count: u32,
    pub txq_timeout: u32,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_difs_latency_stats_tlv_v {
    pub hist_intvl: u32,
// histogram of ppdu post to hwsch - > cmd status received
    pub /: *mut *mut u32 difs_latency_hist[]; / HTT_TX_HWQ_MAX_DIFS_LATENCY_BINS,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_cmd_result_stats_tlv_v {
// Histogram of sched cmd result, HTT_TX_HWQ_MAX_CMD_RESULT_STATS
    pub cmd_result): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_cmd_stall_stats_tlv_v {
// Histogram of various pause conitions, HTT_TX_HWQ_MAX_CMD_STALL_STATS
    pub cmd_stall_status): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_fes_result_stats_tlv_v {
// Histogram of number of user fes result, HTT_TX_HWQ_MAX_FES_RESULT_STATS
    pub fes_result): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
//
// The hwq_tried_mpdu_cnt_hist is a  histogram of MPDUs tries per HWQ.
// The tries here is the count of the  MPDUS within a PPDU that the HW
// had attempted to transmit on  air, for the HWSCH Schedule command
// submitted by FW in this HWQ .It is not the retry attempts. The
// histogram bins are  0-29, 30-59, 60-89 and so on. The are 10 bins
// in this histogram.
// they are defined in FW using the following macros
// #define WAL_MAX_TRIED_MPDU_CNT_HISTOGRAM 9
// #define WAL_TRIED_MPDU_CNT_HISTOGRAM_INTERVAL 30
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_tried_mpdu_cnt_hist_tlv_v {
    pub hist_bin_size: u32,
// Histogram of number of mpdus on tried mpdu
    pub /: *mut *mut u32 tried_mpdu_cnt_hist[]; / HTT_TX_HWQ_TRIED_MPDU_CNT_HIST,
}

// NOTE: Variable length TLV, use length spec to infer array size
//
// The txop_used_cnt_hist is the histogram of txop per burst. After
// completing the burst, we identify the txop used in the burst and
// incr the corresponding bin.
// Each bin represents 1ms & we have 10 bins in this histogram.
// they are defined in FW using the following macros
// #define WAL_MAX_TXOP_USED_CNT_HISTOGRAM 10
// #define WAL_TXOP_USED_HISTOGRAM_INTERVAL 1000 ( 1 ms )
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_hwq_txop_used_cnt_hist_tlv_v {
// Histogram of txop used cnt,  HTT_TX_HWQ_TXOP_USED_CNT_HIST
    pub txop_used_cnt_hist): DECLARE_FLEX_ARRAY(u32,,
}

// == TX SELFGEN STATS ==
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_selfgen_cmn_stats_tlv {
    pub mac_id__word: u32,
    pub su_bar: u32,
    pub rts: u32,
    pub cts2self: u32,
    pub qos_null: u32,
    pub /: *mut *mut u32 delayed_bar_1; / MU user 1,
    pub /: *mut *mut u32 delayed_bar_2; / MU user 2,
    pub /: *mut *mut u32 delayed_bar_3; / MU user 3,
    pub /: *mut *mut u32 delayed_bar_4; / MU user 4,
    pub /: *mut *mut u32 delayed_bar_5; / MU user 5,
    pub /: *mut *mut u32 delayed_bar_6; / MU user 6,
    pub /: *mut *mut u32 delayed_bar_7; / MU user 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_selfgen_ac_stats_tlv {
// 11AC
    pub ac_su_ndpa: u32,
    pub ac_su_ndp: u32,
    pub ac_mu_mimo_ndpa: u32,
    pub ac_mu_mimo_ndp: u32,
    pub /: *mut *mut u32 ac_mu_mimo_brpoll_1; / MU user 1,
    pub /: *mut *mut u32 ac_mu_mimo_brpoll_2; / MU user 2,
    pub /: *mut *mut u32 ac_mu_mimo_brpoll_3; / MU user 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_selfgen_ax_stats_tlv {
// 11AX
    pub ax_su_ndpa: u32,
    pub ax_su_ndp: u32,
    pub ax_mu_mimo_ndpa: u32,
    pub ax_mu_mimo_ndp: u32,
    pub /: *mut *mut u32 ax_mu_mimo_brpoll_1; / MU user 1,
    pub /: *mut *mut u32 ax_mu_mimo_brpoll_2; / MU user 2,
    pub /: *mut *mut u32 ax_mu_mimo_brpoll_3; / MU user 3,
    pub /: *mut *mut u32 ax_mu_mimo_brpoll_4; / MU user 4,
    pub /: *mut *mut u32 ax_mu_mimo_brpoll_5; / MU user 5,
    pub /: *mut *mut u32 ax_mu_mimo_brpoll_6; / MU user 6,
    pub /: *mut *mut u32 ax_mu_mimo_brpoll_7; / MU user 7,
    pub ax_basic_trigger: u32,
    pub ax_bsr_trigger: u32,
    pub ax_mu_bar_trigger: u32,
    pub ax_mu_rts_trigger: u32,
    pub ax_ulmumimo_trigger: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_selfgen_ac_err_stats_tlv {
// 11AC error stats
    pub ac_su_ndp_err: u32,
    pub ac_su_ndpa_err: u32,
    pub ac_mu_mimo_ndpa_err: u32,
    pub ac_mu_mimo_ndp_err: u32,
    pub ac_mu_mimo_brp1_err: u32,
    pub ac_mu_mimo_brp2_err: u32,
    pub ac_mu_mimo_brp3_err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_selfgen_ax_err_stats_tlv {
// 11AX error stats
    pub ax_su_ndp_err: u32,
    pub ax_su_ndpa_err: u32,
    pub ax_mu_mimo_ndpa_err: u32,
    pub ax_mu_mimo_ndp_err: u32,
    pub ax_mu_mimo_brp1_err: u32,
    pub ax_mu_mimo_brp2_err: u32,
    pub ax_mu_mimo_brp3_err: u32,
    pub ax_mu_mimo_brp4_err: u32,
    pub ax_mu_mimo_brp5_err: u32,
    pub ax_mu_mimo_brp6_err: u32,
    pub ax_mu_mimo_brp7_err: u32,
    pub ax_basic_trigger_err: u32,
    pub ax_bsr_trigger_err: u32,
    pub ax_mu_bar_trigger_err: u32,
    pub ax_mu_rts_trigger_err: u32,
    pub ax_ulmumimo_trigger_err: u32,
}

// == TX MU STATS ==
pub const HTT_TX_PDEV_STATS_NUM_AC_MUMIMO_USER_STATS: c_int = 4;
pub const HTT_TX_PDEV_STATS_NUM_AX_MUMIMO_USER_STATS: c_int = 8;
pub const HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS: c_int = 74;
pub const HTT_TX_PDEV_STATS_NUM_UL_MUMIMO_USER_STATS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_mu_mimo_sch_stats_tlv {
// mu-mimo sw sched cmd stats
    pub mu_mimo_sch_posted: u32,
    pub mu_mimo_sch_failed: u32,
// MU PPDU stats per hwQ
    pub mu_mimo_ppdu_posted: u32,
//
// Counts the number of users in each transmission of
// the given TX mode.
//
// Index is the number of users - 1.
//
    pub ac_mu_mimo_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_AC_MUMIMO_USER_STATS],
    pub ax_mu_mimo_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_AX_MUMIMO_USER_STATS],
    pub ax_ofdma_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_basic_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_bsr_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_bar_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
    pub ax_ul_ofdma_brp_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// UL MU-MIMO
// ax_ul_mumimo_basic_sch_nusers[i] is the number of basic triggers sent
// for (i+1) users
//
    pub ax_ul_mumimo_basic_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_UL_MUMIMO_USER_STATS],
// ax_ul_mumimo_brp_sch_nusers[i] is the number of brp triggers sent
// for (i+1) users
//
    pub ax_ul_mumimo_brp_sch_nusers: [u32; HTT_TX_PDEV_STATS_NUM_UL_MUMIMO_USER_STATS],
    pub ac_mu_mimo_sch_posted_per_grp_sz: [u32; HTT_TX_PDEV_STATS_NUM_AC_MUMIMO_USER_STATS],
    pub ax_mu_mimo_sch_posted_per_grp_sz: [u32; HTT_TX_PDEV_STATS_NUM_AX_MUMIMO_USER_STATS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_mu_mimo_mpdu_stats_tlv {
    pub mu_mimo_mpdus_queued_usr: u32,
    pub mu_mimo_mpdus_tried_usr: u32,
    pub mu_mimo_mpdus_failed_usr: u32,
    pub mu_mimo_mpdus_requeued_usr: u32,
    pub mu_mimo_err_no_ba_usr: u32,
    pub mu_mimo_mpdu_underrun_usr: u32,
    pub mu_mimo_ampdu_underrun_usr: u32,
    pub ax_mu_mimo_mpdus_queued_usr: u32,
    pub ax_mu_mimo_mpdus_tried_usr: u32,
    pub ax_mu_mimo_mpdus_failed_usr: u32,
    pub ax_mu_mimo_mpdus_requeued_usr: u32,
    pub ax_mu_mimo_err_no_ba_usr: u32,
    pub ax_mu_mimo_mpdu_underrun_usr: u32,
    pub ax_mu_mimo_ampdu_underrun_usr: u32,
    pub ax_ofdma_mpdus_queued_usr: u32,
    pub ax_ofdma_mpdus_tried_usr: u32,
    pub ax_ofdma_mpdus_failed_usr: u32,
    pub ax_ofdma_mpdus_requeued_usr: u32,
    pub ax_ofdma_err_no_ba_usr: u32,
    pub ax_ofdma_mpdu_underrun_usr: u32,
    pub ax_ofdma_ampdu_underrun_usr: u32,
}

pub const HTT_STATS_TX_SCHED_MODE_MU_MIMO_AC: c_int = 1;
pub const HTT_STATS_TX_SCHED_MODE_MU_MIMO_AX: c_int = 2;
pub const HTT_STATS_TX_SCHED_MODE_MU_OFDMA_AX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_mpdu_stats_tlv {
// mpdu level stats
    pub mpdus_queued_usr: u32,
    pub mpdus_tried_usr: u32,
    pub mpdus_failed_usr: u32,
    pub mpdus_requeued_usr: u32,
    pub err_no_ba_usr: u32,
    pub mpdu_underrun_usr: u32,
    pub ampdu_underrun_usr: u32,
    pub user_index: u32,
    pub /: *mut *mut u32 tx_sched_mode; / HTT_STATS_TX_SCHED_MODE_xxx,
}

// == TX SCHED STATS ==
// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sched_txq_cmd_posted_tlv_v {
// HTT_TX_PDEV_SCHED_TX_MODE_MAX
    pub sched_cmd_posted): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sched_txq_cmd_reaped_tlv_v {
// HTT_TX_PDEV_SCHED_TX_MODE_MAX
    pub sched_cmd_reaped): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sched_txq_sched_order_su_tlv_v {
// HTT_TX_PDEV_NUM_SCHED_ORDER_LOG
    pub sched_order_su): DECLARE_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_sched_txq_sched_ineligibility_tlv_enum {
    HTT_SCHED_TID_SKIP_SCHED_MASK_DISABLED = 0,
    HTT_SCHED_TID_SKIP_NOTIFY_MPDU,
    HTT_SCHED_TID_SKIP_MPDU_STATE_INVALID,
    HTT_SCHED_TID_SKIP_SCHED_DISABLED,
    HTT_SCHED_TID_SKIP_TQM_BYPASS_CMD_PENDING,
    HTT_SCHED_TID_SKIP_SECOND_SU_SCHEDULE,

    HTT_SCHED_TID_SKIP_CMD_SLOT_NOT_AVAIL,
    HTT_SCHED_TID_SKIP_NO_ENQ,
    HTT_SCHED_TID_SKIP_LOW_ENQ,
    HTT_SCHED_TID_SKIP_PAUSED,
    HTT_SCHED_TID_SKIP_UL,
    HTT_SCHED_TID_REMOVE_PAUSED,
    HTT_SCHED_TID_REMOVE_NO_ENQ,
    HTT_SCHED_TID_REMOVE_UL,
    HTT_SCHED_TID_QUERY,
    HTT_SCHED_TID_SU_ONLY,
    HTT_SCHED_TID_ELIGIBLE,
    HTT_SCHED_INELIGIBILITY_MAX,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sched_txq_sched_ineligibility_tlv_v {
// indexed by htt_sched_txq_sched_ineligibility_tlv_enum
    pub sched_ineligibility): DECLARE_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_stats_sched_per_txq_tlv {
    pub mac_id__txq_id__word: u32,
    pub sched_policy: u32,
    pub last_sched_cmd_posted_timestamp: u32,
    pub last_sched_cmd_compl_timestamp: u32,
    pub sched_2_tac_lwm_count: u32,
    pub sched_2_tac_ring_full: u32,
    pub sched_cmd_post_failure: u32,
    pub num_active_tids: u32,
    pub num_ps_schedules: u32,
    pub sched_cmds_pending: u32,
    pub num_tid_register: u32,
    pub num_tid_unregister: u32,
    pub num_qstats_queried: u32,
    pub qstats_update_pending: u32,
    pub last_qstats_query_timestamp: u32,
    pub num_tqm_cmdq_full: u32,
    pub num_de_sched_algo_trigger: u32,
    pub num_rt_sched_algo_trigger: u32,
    pub num_tqm_sched_algo_trigger: u32,
    pub notify_sched: u32,
    pub dur_based_sendn_term: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_stats_tx_sched_cmn_tlv {
// BIT [ 7 :  0]   :- mac_id
// BIT [31 :  8]   :- reserved
//
    pub mac_id__word: u32,
// Current timestamp
    pub current_timestamp: u32,
}

// == TQM STATS ==
pub const HTT_TX_TQM_MAX_GEN_MPDU_END_REASON: c_int = 16;
pub const HTT_TX_TQM_MAX_LIST_MPDU_END_REASON: c_int = 16;
pub const HTT_TX_TQM_MAX_LIST_MPDU_CNT_HISTOGRAM_BINS: c_int = 16;
// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tqm_gen_mpdu_stats_tlv_v {
// HTT_TX_TQM_MAX_GEN_MPDU_END_REASON
    pub gen_mpdu_end_reason): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tqm_list_mpdu_stats_tlv_v {
// HTT_TX_TQM_MAX_LIST_MPDU_END_REASON
    pub list_mpdu_end_reason): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tqm_list_mpdu_cnt_tlv_v {
// HTT_TX_TQM_MAX_LIST_MPDU_CNT_HISTOGRAM_BINS
    pub list_mpdu_cnt_hist): DECLARE_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tqm_pdev_stats_tlv_v {
    pub msdu_count: u32,
    pub mpdu_count: u32,
    pub remove_msdu: u32,
    pub remove_mpdu: u32,
    pub remove_msdu_ttl: u32,
    pub send_bar: u32,
    pub bar_sync: u32,
    pub notify_mpdu: u32,
    pub sync_cmd: u32,
    pub write_cmd: u32,
    pub hwsch_trigger: u32,
    pub ack_tlv_proc: u32,
    pub gen_mpdu_cmd: u32,
    pub gen_list_cmd: u32,
    pub remove_mpdu_cmd: u32,
    pub remove_mpdu_tried_cmd: u32,
    pub mpdu_queue_stats_cmd: u32,
    pub mpdu_head_info_cmd: u32,
    pub msdu_flow_stats_cmd: u32,
    pub remove_msdu_cmd: u32,
    pub remove_msdu_ttl_cmd: u32,
    pub flush_cache_cmd: u32,
    pub update_mpduq_cmd: u32,
    pub enqueue: u32,
    pub enqueue_notify: u32,
    pub notify_mpdu_at_head: u32,
    pub notify_mpdu_state_valid: u32,
//
// On receiving TQM_FLOW_NOT_EMPTY_STATUS from TQM, (on MSDUs being enqueued
// the flow is non empty), if the number of MSDUs is greater than the threshold,
// notify is incremented. UDP_THRESH counters are for UDP MSDUs, and NONUDP are
// for non-UDP MSDUs.
// MSDUQ_SWNOTIFY_UDP_THRESH1 threshold    - sched_udp_notify1 is incremented
// MSDUQ_SWNOTIFY_UDP_THRESH2 threshold    - sched_udp_notify2 is incremented
// MSDUQ_SWNOTIFY_NONUDP_THRESH1 threshold - sched_nonudp_notify1 is incremented
// MSDUQ_SWNOTIFY_NONUDP_THRESH2 threshold - sched_nonudp_notify2 is incremented
//
// Notify signifies that we trigger the scheduler.
//
    pub sched_udp_notify1: u32,
    pub sched_udp_notify2: u32,
    pub sched_nonudp_notify1: u32,
    pub sched_nonudp_notify2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tqm_cmn_stats_tlv {
    pub mac_id__word: u32,
    pub max_cmdq_id: u32,
    pub list_mpdu_cnt_hist_intvl: u32,
// Global stats
    pub add_msdu: u32,
    pub q_empty: u32,
    pub q_not_empty: u32,
    pub drop_notification: u32,
    pub desc_threshold: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tqm_error_stats_tlv {
// Error stats
    pub q_empty_failure: u32,
    pub q_not_empty_failure: u32,
    pub add_msdu_failure: u32,
}

// == TQM CMDQ stats ==

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_tqm_cmdq_status_tlv {
    pub mac_id__cmdq_id__word: u32,
    pub sync_cmd: u32,
    pub write_cmd: u32,
    pub gen_mpdu_cmd: u32,
    pub mpdu_queue_stats_cmd: u32,
    pub mpdu_head_info_cmd: u32,
    pub msdu_flow_stats_cmd: u32,
    pub remove_mpdu_cmd: u32,
    pub remove_msdu_cmd: u32,
    pub flush_cache_cmd: u32,
    pub update_mpduq_cmd: u32,
    pub update_msduq_cmd: u32,
}

// == TX-DE STATS ==
// Structures for tx de stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_eapol_packets_stats_tlv {
    pub m1_packets: u32,
    pub m2_packets: u32,
    pub m3_packets: u32,
    pub m4_packets: u32,
    pub g1_packets: u32,
    pub g2_packets: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_classify_failed_stats_tlv {
    pub ap_bss_peer_not_found: u32,
    pub ap_bcast_mcast_no_peer: u32,
    pub sta_delete_in_progress: u32,
    pub ibss_no_bss_peer: u32,
    pub invalid_vdev_type: u32,
    pub invalid_ast_peer_entry: u32,
    pub peer_entry_invalid: u32,
    pub ethertype_not_ip: u32,
    pub eapol_lookup_failed: u32,
    pub qpeer_not_allow_data: u32,
    pub fse_tid_override: u32,
    pub ipv6_jumbogram_zero_length: u32,
    pub qos_to_non_qos_in_prog: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_classify_stats_tlv {
    pub arp_packets: u32,
    pub igmp_packets: u32,
    pub dhcp_packets: u32,
    pub host_inspected: u32,
    pub htt_included: u32,
    pub htt_valid_mcs: u32,
    pub htt_valid_nss: u32,
    pub htt_valid_preamble_type: u32,
    pub htt_valid_chainmask: u32,
    pub htt_valid_guard_interval: u32,
    pub htt_valid_retries: u32,
    pub htt_valid_bw_info: u32,
    pub htt_valid_power: u32,
    pub htt_valid_key_flags: u32,
    pub htt_valid_no_encryption: u32,
    pub fse_entry_count: u32,
    pub fse_priority_be: u32,
    pub fse_priority_high: u32,
    pub fse_priority_low: u32,
    pub fse_traffic_ptrn_be: u32,
    pub fse_traffic_ptrn_over_sub: u32,
    pub fse_traffic_ptrn_bursty: u32,
    pub fse_traffic_ptrn_interactive: u32,
    pub fse_traffic_ptrn_periodic: u32,
    pub fse_hwqueue_alloc: u32,
    pub fse_hwqueue_created: u32,
    pub fse_hwqueue_send_to_host: u32,
    pub mcast_entry: u32,
    pub bcast_entry: u32,
    pub htt_update_peer_cache: u32,
    pub htt_learning_frame: u32,
    pub fse_invalid_peer: u32,
//
// mec_notify is HTT TX WBM multicast echo check notification
// from firmware to host.  FW sends SA addresses to host for all
// multicast/broadcast packets received on STA side.
//
    pub mec_notify: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_classify_status_stats_tlv {
    pub eok: u32,
    pub classify_done: u32,
    pub lookup_failed: u32,
    pub send_host_dhcp: u32,
    pub send_host_mcast: u32,
    pub send_host_unknown_dest: u32,
    pub send_host: u32,
    pub status_invalid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_enqueue_packets_stats_tlv {
    pub enqueued_pkts: u32,
    pub to_tqm: u32,
    pub to_tqm_bypass: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_enqueue_discard_stats_tlv {
    pub discarded_pkts: u32,
    pub local_frames: u32,
    pub is_ext_msdu: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_compl_stats_tlv {
    pub tcl_dummy_frame: u32,
    pub tqm_dummy_frame: u32,
    pub tqm_notify_frame: u32,
    pub fw2wbm_enq: u32,
    pub tqm_bypass_frame: u32,
}

//
// The htt_tx_de_fw2wbm_ring_full_hist_tlv is a histogram of time we waited
// for the fw2wbm ring buffer.  we are requesting a buffer in FW2WBM release
// ring,which may fail, due to non availability of buffer. Hence we sleep for
// 200us & again request for it. This is a histogram of time we wait, with
// bin of 200ms & there are 10 bin (2 seconds max)
// They are defined by the following macros in FW
// #define ENTRIES_PER_BIN_COUNT 1000  // per bin 1000 * 200us = 200ms
// #define RING_FULL_BIN_ENTRIES (WAL_TX_DE_FW2WBM_ALLOC_TIMEOUT_COUNT
// ENTRIES_PER_BIN_COUNT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_fw2wbm_ring_full_hist_tlv {
    pub fw2wbm_ring_full_hist): DECLARE_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_de_cmn_stats_tlv {
    pub mac_id__word: u32,
// Global Stats
    pub tcl2fw_entry_count: u32,
    pub not_to_fw: u32,
    pub invalid_pdev_vdev_peer: u32,
    pub tcl_res_invalid_addrx: u32,
    pub wbm2fw_entry_count: u32,
    pub invalid_pdev: u32,
}

// == RING-IF STATS ==
pub const HTT_STATS_LOW_WM_BINS: c_int = 5;
pub const HTT_STATS_HIGH_WM_BINS: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ring_if_stats_tlv {
    pub /: *mut *mut u32 base_addr; / DWORD aligned base memory address of the ring,
    pub elem_size: u32,
    pub num_elems__prefetch_tail_idx: u32,
    pub head_idx__tail_idx: u32,
    pub shadow_head_idx__shadow_tail_idx: u32,
    pub num_tail_incr: u32,
    pub lwm_thresh__hwm_thresh: u32,
    pub overrun_hit_count: u32,
    pub underrun_hit_count: u32,
    pub prod_blockwait_count: u32,
    pub cons_blockwait_count: u32,
    pub low_wm_hit_count: [u32; HTT_STATS_LOW_WM_BINS],
    pub high_wm_hit_count: [u32; HTT_STATS_HIGH_WM_BINS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ring_if_cmn_tlv {
    pub mac_id__word: u32,
    pub num_records: u32,
}

// == SFM STATS ==
// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sfm_client_user_tlv_v {
// Number of DWORDS used per user and per client
    pub dwords_used_by_user_n): DECLARE_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sfm_client_tlv {
// Client ID
    pub client_id: u32,
// Minimum number of buffers
    pub buf_min: u32,
// Maximum number of buffers
    pub buf_max: u32,
// Number of Busy buffers
    pub buf_busy: u32,
// Number of Allocated buffers
    pub buf_alloc: u32,
// Number of Available/Usable buffers
    pub buf_avail: u32,
// Number of users
    pub num_users: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sfm_cmn_tlv {
    pub mac_id__word: u32,
// Indicates the total number of 128 byte buffers
// in the CMEM that are available for buffer sharing
//
    pub buf_total: u32,
// Indicates for certain client or all the clients
// there is no dowrd saved in SFM, refer to SFM_R1_MEM_EMPTY
//
    pub mem_empty: u32,
// DEALLOCATE_BUFFERS, refer to register SFM_R0_DEALLOCATE_BUFFERS
    pub deallocate_bufs: u32,
// Number of Records
    pub num_records: u32,
}

// == SRNG STATS ==

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sring_stats_tlv {
    pub mac_id__ring_id__arena__ep: u32,
    pub /: *mut *mut u32 base_addr_lsb; / DWORD aligned base memory address of the ring,
    pub base_addr_msb: u32,
    pub ring_size: u32,
    pub elem_size: u32,
    pub num_avail_words__num_valid_words: u32,
    pub head_ptr__tail_ptr: u32,
    pub consumer_empty__producer_full: u32,
    pub prefetch_count__internal_tail_ptr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_sring_cmn_tlv {
    pub num_records: u32,
}

// == PDEV TX RATE CTRL STATS ==
pub const HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS: c_int = 12;
pub const HTT_TX_PDEV_STATS_NUM_GI_COUNTERS: c_int = 4;
pub const HTT_TX_PDEV_STATS_NUM_DCM_COUNTERS: c_int = 5;
pub const HTT_TX_PDEV_STATS_NUM_BW_COUNTERS: c_int = 4;
pub const HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS: c_int = 8;

pub const HTT_TX_PDEV_STATS_NUM_LEGACY_CCK_STATS: c_int = 4;
pub const HTT_TX_PDEV_STATS_NUM_LEGACY_OFDM_STATS: c_int = 8;
pub const HTT_TX_PDEV_STATS_NUM_LTF: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_pdev_rate_stats_tlv {
    pub mac_id__word: u32,
    pub tx_ldpc: u32,
    pub rts_cnt: u32,
// RSSI value of last ack packet (units = dB above noise floor)
    pub ack_rssi: u32,
    pub tx_mcs: [u32; HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub tx_su_mcs: [u32; HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub tx_mu_mcs: [u32; HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
// element 0,1, ...7 -> NSS 1,2, ...8
    pub tx_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
// element 0: 20 MHz, 1: 40 MHz, 2: 80 MHz, 3: 160 and 80+80 MHz
    pub tx_bw: [u32; HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub tx_stbc: [u32; HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub tx_pream: [u32; HTT_TX_PDEV_STATS_NUM_PREAMBLE_TYPES],
// Counters to track number of tx packets
// in each GI (400us, 800us, 1600us & 3200us) in each mcs (0-11)
//
    pub tx_gi: [u32; HTT_TX_PDEV_STATS_NUM_GI_COUNTERS][HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
// Counters to track packets in dcm mcs (MCS 0, 1, 3, 4)
    pub tx_dcm: [u32; HTT_TX_PDEV_STATS_NUM_DCM_COUNTERS],
// Number of CTS-acknowledged RTS packets
    pub rts_success: u32,
//
// Counters for legacy 11a and 11b transmissions.
//
// The index corresponds to:
//
// CCK: 0: 1 Mbps, 1: 2 Mbps, 2: 5.5 Mbps, 3: 11 Mbps
//
// OFDM: 0: 6 Mbps, 1: 9 Mbps, 2: 12 Mbps, 3: 18 Mbps,
// 4: 24 Mbps, 5: 36 Mbps, 6: 48 Mbps, 7: 54 Mbps
//
    pub tx_legacy_cck_rate: [u32; HTT_TX_PDEV_STATS_NUM_LEGACY_CCK_STATS],
    pub tx_legacy_ofdm_rate: [u32; HTT_TX_PDEV_STATS_NUM_LEGACY_OFDM_STATS],
    pub ac_mu_mimo_tx_ldpc: u32,
    pub ax_mu_mimo_tx_ldpc: u32,
    pub ofdma_tx_ldpc: u32,
//
// Counters for 11ax HE LTF selection during TX.
//
// The index corresponds to:
//
// 0: unused, 1: 1x LTF, 2: 2x LTF, 3: 4x LTF
//
    pub tx_he_ltf: [u32; HTT_TX_PDEV_STATS_NUM_LTF],
    pub ac_mu_mimo_tx_mcs: [u32; HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ax_mu_mimo_tx_mcs: [u32; HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ofdma_tx_mcs: [u32; HTT_TX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ac_mu_mimo_tx_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ax_mu_mimo_tx_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ofdma_tx_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ac_mu_mimo_tx_bw: [u32; HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub ax_mu_mimo_tx_bw: [u32; HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub ofdma_tx_bw: [u32; HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
}

// == PDEV RX RATE CTRL STATS ==
pub const HTT_RX_PDEV_STATS_NUM_LEGACY_CCK_STATS: c_int = 4;
pub const HTT_RX_PDEV_STATS_NUM_LEGACY_OFDM_STATS: c_int = 8;
pub const HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS: c_int = 12;
pub const HTT_RX_PDEV_STATS_NUM_GI_COUNTERS: c_int = 4;
pub const HTT_RX_PDEV_STATS_NUM_DCM_COUNTERS: c_int = 5;
pub const HTT_RX_PDEV_STATS_NUM_BW_COUNTERS: c_int = 4;
pub const HTT_RX_PDEV_STATS_NUM_SPATIAL_STREAMS: c_int = 8;

pub const HTT_RX_PDEV_MAX_OFDMA_NUM_USER: c_int = 8;
pub const HTT_RX_PDEV_STATS_RXEVM_MAX_PILOTS_PER_NSS: c_int = 16;
pub const HTT_RX_PDEV_STATS_NUM_RU_SIZE_COUNTERS: c_int = 6;
pub const HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_pdev_rate_stats_tlv {
    pub mac_id__word: u32,
    pub nsts: u32,
    pub rx_ldpc: u32,
    pub rts_cnt: u32,
    pub /: *mut *mut u32 rssi_mgmt; / units = dB above noise floor,
    pub /: *mut *mut u32 rssi_data; / units = dB above noise floor,
    pub /: *mut *mut u32 rssi_comb; / units = dB above noise floor,
    pub rx_mcs: [u32; HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
// element 0,1, ...7 -> NSS 1,2, ...8
    pub rx_nss: [u32; HTT_RX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub rx_dcm: [u32; HTT_RX_PDEV_STATS_NUM_DCM_COUNTERS],
    pub rx_stbc: [u32; HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
// element 0: 20 MHz, 1: 40 MHz, 2: 80 MHz, 3: 160 and 80+80 MHz
    pub rx_bw: [u32; HTT_RX_PDEV_STATS_NUM_BW_COUNTERS],
    pub rx_pream: [u32; HTT_RX_PDEV_STATS_NUM_PREAMBLE_TYPES],
// units = dB above noise floor
// Counters to track number of rx packets
// in each GI in each mcs (0-11)
//
    pub rx_gi: [u32; HTT_RX_PDEV_STATS_NUM_GI_COUNTERS][HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub /: *mut *mut s32 rssi_in_dbm; / rx Signal Strength value in dBm unit,
    pub rx_11ax_su_ext: u32,
    pub rx_11ac_mumimo: u32,
    pub rx_11ax_mumimo: u32,
    pub rx_11ax_ofdma: u32,
    pub txbf: u32,
    pub rx_legacy_cck_rate: [u32; HTT_RX_PDEV_STATS_NUM_LEGACY_CCK_STATS],
    pub rx_legacy_ofdm_rate: [u32; HTT_RX_PDEV_STATS_NUM_LEGACY_OFDM_STATS],
    pub rx_active_dur_us_low: u32,
    pub rx_active_dur_us_high: u32,
    pub rx_11ax_ul_ofdma: u32,
    pub ul_ofdma_rx_mcs: [u32; HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub ul_ofdma_rx_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub ul_ofdma_rx_bw: [u32; HTT_TX_PDEV_STATS_NUM_BW_COUNTERS],
    pub ul_ofdma_rx_stbc: u32,
    pub ul_ofdma_rx_ldpc: u32,
// record the stats for each user index
    pub /: *mut *mut u32 rx_ulofdma_non_data_ppdu[HTT_RX_PDEV_MAX_OFDMA_NUM_USER]; / ppdu level,
    pub /: *mut *mut u32 rx_ulofdma_data_ppdu[HTT_RX_PDEV_MAX_OFDMA_NUM_USER]; / ppdu level,
    pub /: *mut *mut u32 rx_ulofdma_mpdu_ok[HTT_RX_PDEV_MAX_OFDMA_NUM_USER]; / mpdu level,
    pub /: *mut *mut u32 rx_ulofdma_mpdu_fail[HTT_RX_PDEV_MAX_OFDMA_NUM_USER]; / mpdu level,
    pub nss_count: u32,
    pub pilot_count: u32,
// RxEVM stats in dB
// rx_pilot_evm_db_mean:
// EVM mean across pilots, computed as
// mean(10*log10(rx_pilot_evm_linear)) = mean(rx_pilot_evm_db)
//
    pub rx_pilot_evm_db_mean: [i32; HTT_RX_PDEV_STATS_NUM_SPATIAL_STREAMS],
    pub /: *mut *mut [HTT_RX_PDEV_MAX_OFDMA_NUM_USER]; / dBm units,
// per_chain_rssi_pkt_type:
// This field shows what type of rx frame the per-chain RSSI was computed
// on, by recording the frame type and sub-type as bit-fields within this
// field:
// BIT [3 : 0]    :- IEEE80211_FC0_TYPE
// BIT [7 : 4]    :- IEEE80211_FC0_SUBTYPE
// BIT [31 : 8]   :- Reserved
//
    pub per_chain_rssi_pkt_type: u32,
    pub rx_su_ndpa: u32,
    pub rx_11ax_su_txbf_mcs: [u32; HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_mu_ndpa: u32,
    pub rx_11ax_mu_txbf_mcs: [u32; HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_br_poll: u32,
    pub rx_11ax_dl_ofdma_mcs: [u32; HTT_RX_PDEV_STATS_NUM_MCS_COUNTERS],
    pub rx_11ax_dl_ofdma_ru: [u32; HTT_RX_PDEV_STATS_NUM_RU_SIZE_COUNTERS],
    pub rx_ulmumimo_non_data_ppdu: [u32; HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulmumimo_data_ppdu: [u32; HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulmumimo_mpdu_ok: [u32; HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulmumimo_mpdu_fail: [u32; HTT_RX_PDEV_MAX_ULMUMIMO_NUM_USER],
    pub rx_ulofdma_non_data_nusers: [u32; HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
    pub rx_ulofdma_data_nusers: [u32; HTT_RX_PDEV_MAX_OFDMA_NUM_USER],
}

// == RX PDEV/SOC STATS ==
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_soc_fw_stats_tlv {
    pub fw_reo_ring_data_msdu: u32,
    pub fw_to_host_data_msdu_bcmc: u32,
    pub fw_to_host_data_msdu_uc: u32,
    pub ofld_remote_data_buf_recycle_cnt: u32,
    pub ofld_remote_free_buf_indication_cnt: u32,
    pub ofld_buf_to_host_data_msdu_uc: u32,
    pub reo_fw_ring_to_host_data_msdu_uc: u32,
    pub wbm_sw_ring_reap: u32,
    pub wbm_forward_to_host_cnt: u32,
    pub wbm_target_recycle_cnt: u32,
    pub target_refill_ring_recycle_cnt: u32,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_soc_fw_refill_ring_empty_tlv_v {
// HTT_RX_STATS_REFILL_MAX_RING
    pub refill_ring_empty_cnt): DECLARE_FLEX_ARRAY(u32,,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_soc_fw_refill_ring_num_refill_tlv_v {
// HTT_RX_STATS_REFILL_MAX_RING
    pub refill_ring_num_refill): DECLARE_FLEX_ARRAY(u32,,
}

// RXDMA error code from WBM released packets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_rxdma_error_code_enum {
    HTT_RX_RXDMA_OVERFLOW_ERR                           = 0,
    HTT_RX_RXDMA_MPDU_LENGTH_ERR                        = 1,
    HTT_RX_RXDMA_FCS_ERR                                = 2,
    HTT_RX_RXDMA_DECRYPT_ERR                            = 3,
    HTT_RX_RXDMA_TKIP_MIC_ERR                           = 4,
    HTT_RX_RXDMA_UNECRYPTED_ERR                         = 5,
    HTT_RX_RXDMA_MSDU_LEN_ERR                           = 6,
    HTT_RX_RXDMA_MSDU_LIMIT_ERR                         = 7,
    HTT_RX_RXDMA_WIFI_PARSE_ERR                         = 8,
    HTT_RX_RXDMA_AMSDU_PARSE_ERR                        = 9,
    HTT_RX_RXDMA_SA_TIMEOUT_ERR                         = 10,
    HTT_RX_RXDMA_DA_TIMEOUT_ERR                         = 11,
    HTT_RX_RXDMA_FLOW_TIMEOUT_ERR                       = 12,
    HTT_RX_RXDMA_FLUSH_REQUEST                          = 13,
    HTT_RX_RXDMA_ERR_CODE_RVSD0                         = 14,
    HTT_RX_RXDMA_ERR_CODE_RVSD1                         = 15,

// This MAX_ERR_CODE should not be used in any host/target messages,
// so that even though it is defined within a host/target interface
// definition header file, it isn't actually part of the host/target
// interface, and thus can be modified.
//
    HTT_RX_RXDMA_MAX_ERR_CODE
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_soc_fw_refill_ring_num_rxdma_err_tlv_v {
    pub /: *mut *mut DECLARE_FLEX_ARRAY(u32, rxdma_err); / HTT_RX_RXDMA_MAX_ERR_CODE,
}

// REO error code from WBM released packets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_reo_error_code_enum {
    HTT_RX_REO_QUEUE_DESC_ADDR_ZERO                     = 0,
    HTT_RX_REO_QUEUE_DESC_NOT_VALID                     = 1,
    HTT_RX_AMPDU_IN_NON_BA                              = 2,
    HTT_RX_NON_BA_DUPLICATE                             = 3,
    HTT_RX_BA_DUPLICATE                                 = 4,
    HTT_RX_REGULAR_FRAME_2K_JUMP                        = 5,
    HTT_RX_BAR_FRAME_2K_JUMP                            = 6,
    HTT_RX_REGULAR_FRAME_OOR                            = 7,
    HTT_RX_BAR_FRAME_OOR                                = 8,
    HTT_RX_BAR_FRAME_NO_BA_SESSION                      = 9,
    HTT_RX_BAR_FRAME_SN_EQUALS_SSN                      = 10,
    HTT_RX_PN_CHECK_FAILED                              = 11,
    HTT_RX_2K_ERROR_HANDLING_FLAG_SET                   = 12,
    HTT_RX_PN_ERROR_HANDLING_FLAG_SET                   = 13,
    HTT_RX_QUEUE_DESCRIPTOR_BLOCKED_SET                 = 14,
    HTT_RX_REO_ERR_CODE_RVSD                            = 15,

// This MAX_ERR_CODE should not be used in any host/target messages,
// so that even though it is defined within a host/target interface
// definition header file, it isn't actually part of the host/target
// interface, and thus can be modified.
//
    HTT_RX_REO_MAX_ERR_CODE
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_soc_fw_refill_ring_num_reo_err_tlv_v {
    pub /: *mut *mut DECLARE_FLEX_ARRAY(u32, reo_err); / HTT_RX_REO_MAX_ERR_CODE,
}

// == RX PDEV STATS ==
pub const HTT_STATS_SUBTYPE_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_pdev_fw_stats_tlv {
    pub mac_id__word: u32,
    pub ppdu_recvd: u32,
    pub mpdu_cnt_fcs_ok: u32,
    pub mpdu_cnt_fcs_err: u32,
    pub tcp_msdu_cnt: u32,
    pub tcp_ack_msdu_cnt: u32,
    pub udp_msdu_cnt: u32,
    pub other_msdu_cnt: u32,
    pub fw_ring_mpdu_ind: u32,
    pub fw_ring_mgmt_subtype: [u32; HTT_STATS_SUBTYPE_MAX],
    pub fw_ring_ctrl_subtype: [u32; HTT_STATS_SUBTYPE_MAX],
    pub fw_ring_mcast_data_msdu: u32,
    pub fw_ring_bcast_data_msdu: u32,
    pub fw_ring_ucast_data_msdu: u32,
    pub fw_ring_null_data_msdu: u32,
    pub fw_ring_mpdu_drop: u32,
    pub ofld_local_data_ind_cnt: u32,
    pub ofld_local_data_buf_recycle_cnt: u32,
    pub drx_local_data_ind_cnt: u32,
    pub drx_local_data_buf_recycle_cnt: u32,
    pub local_nondata_ind_cnt: u32,
    pub local_nondata_buf_recycle_cnt: u32,
    pub fw_status_buf_ring_refill_cnt: u32,
    pub fw_status_buf_ring_empty_cnt: u32,
    pub fw_pkt_buf_ring_refill_cnt: u32,
    pub fw_pkt_buf_ring_empty_cnt: u32,
    pub fw_link_buf_ring_refill_cnt: u32,
    pub fw_link_buf_ring_empty_cnt: u32,
    pub host_pkt_buf_ring_refill_cnt: u32,
    pub host_pkt_buf_ring_empty_cnt: u32,
    pub mon_pkt_buf_ring_refill_cnt: u32,
    pub mon_pkt_buf_ring_empty_cnt: u32,
    pub mon_status_buf_ring_refill_cnt: u32,
    pub mon_status_buf_ring_empty_cnt: u32,
    pub mon_desc_buf_ring_refill_cnt: u32,
    pub mon_desc_buf_ring_empty_cnt: u32,
    pub mon_dest_ring_update_cnt: u32,
    pub mon_dest_ring_full_cnt: u32,
    pub rx_suspend_cnt: u32,
    pub rx_suspend_fail_cnt: u32,
    pub rx_resume_cnt: u32,
    pub rx_resume_fail_cnt: u32,
    pub rx_ring_switch_cnt: u32,
    pub rx_ring_restore_cnt: u32,
    pub rx_flush_cnt: u32,
    pub rx_recovery_reset_cnt: u32,
}

pub const HTT_STATS_PHY_ERR_MAX: c_int = 43;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_pdev_fw_stats_phy_err_tlv {
    pub mac_id__word: u32,
    pub total_phy_err_cnt: u32,
// Counts of different types of phy errs
// The mapping of PHY error types to phy_err array elements is HW dependent.
// The only currently-supported mapping is shown below:
//
// 0 phyrx_err_phy_off Reception aborted due to receiving a PHY_OFF TLV
// 1 phyrx_err_synth_off
// 2 phyrx_err_ofdma_timing
// 3 phyrx_err_ofdma_signal_parity
// 4 phyrx_err_ofdma_rate_illegal
// 5 phyrx_err_ofdma_length_illegal
// 6 phyrx_err_ofdma_restart
// 7 phyrx_err_ofdma_service
// 8 phyrx_err_ppdu_ofdma_power_drop
// 9 phyrx_err_cck_blokker
// 10 phyrx_err_cck_timing
// 11 phyrx_err_cck_header_crc
// 12 phyrx_err_cck_rate_illegal
// 13 phyrx_err_cck_length_illegal
// 14 phyrx_err_cck_restart
// 15 phyrx_err_cck_service
// 16 phyrx_err_cck_power_drop
// 17 phyrx_err_ht_crc_err
// 18 phyrx_err_ht_length_illegal
// 19 phyrx_err_ht_rate_illegal
// 20 phyrx_err_ht_zlf
// 21 phyrx_err_false_radar_ext
// 22 phyrx_err_green_field
// 23 phyrx_err_bw_gt_dyn_bw
// 24 phyrx_err_leg_ht_mismatch
// 25 phyrx_err_vht_crc_error
// 26 phyrx_err_vht_siga_unsupported
// 27 phyrx_err_vht_lsig_len_invalid
// 28 phyrx_err_vht_ndp_or_zlf
// 29 phyrx_err_vht_nsym_lt_zero
// 30 phyrx_err_vht_rx_extra_symbol_mismatch
// 31 phyrx_err_vht_rx_skip_group_id0
// 32 phyrx_err_vht_rx_skip_group_id1to62
// 33 phyrx_err_vht_rx_skip_group_id63
// 34 phyrx_err_ofdm_ldpc_decoder_disabled
// 35 phyrx_err_defer_nap
// 36 phyrx_err_fdomain_timeout
// 37 phyrx_err_lsig_rel_check
// 38 phyrx_err_bt_collision
// 39 phyrx_err_unsupported_mu_feedback
// 40 phyrx_err_ppdu_tx_interrupt_rx
// 41 phyrx_err_unsupported_cbf
// 42 phyrx_err_other
//
    pub phy_err: [u32; HTT_STATS_PHY_ERR_MAX],
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_pdev_fw_ring_mpdu_err_tlv_v {
// Num error MPDU for each RxDMA error type
    pub /: *mut *mut DECLARE_FLEX_ARRAY(u32, fw_ring_mpdu_err); / HTT_RX_STATS_RXDMA_MAX_ERR,
}

// NOTE: Variable length TLV, use length spec to infer array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_pdev_fw_mpdu_drop_tlv_v {
// Num MPDU dropped
    pub /: *mut *mut DECLARE_FLEX_ARRAY(u32, fw_mpdu_drop); / HTT_RX_STATS_FW_DROP_REASON_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pdev_stats_cca_counters_tlv {
// Below values are obtained from the HW Cycles counter registers
    pub tx_frame_usec: u32,
    pub rx_frame_usec: u32,
    pub rx_clear_usec: u32,
    pub my_rx_frame_usec: u32,
    pub usec_cnt: u32,
    pub med_rx_idle_usec: u32,
    pub med_tx_idle_global_usec: u32,
    pub cca_obss_usec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pdev_cca_stats_hist_v1_tlv {
    pub chan_num: u32,
// num of CCA records (Num of htt_pdev_stats_cca_counters_tlv)
    pub num_records: u32,
    pub valid_cca_counters_bitmap: u32,
    pub collection_interval: u32,
// This will be followed by an array which contains the CCA stats
// collected in the last N intervals,
// if the indication is for last N intervals CCA stats.
// Then the pdev_cca_stats[0] element contains the oldest CCA stats
// and pdev_cca_stats[N-1] will have the most recent CCA stats.
// htt_pdev_stats_cca_counters_tlv cca_hist_tlv[1];
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pdev_stats_twt_session_tlv {
    pub vdev_id: u32,
    pub peer_mac: htt_mac_addr,
    pub flow_id_flags: u32,
// TWT_DIALOG_ID_UNAVAILABLE is used
// when TWT session is not initiated by host
//
    pub dialog_id: u32,
    pub wake_dura_us: u32,
    pub wake_intvl_us: u32,
    pub sp_offset_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pdev_stats_twt_sessions_tlv {
    pub pdev_id: u32,
    pub num_sessions: u32,
    pub twt_session: [htt_pdev_stats_twt_session_tlv; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_reo_resource_sample_id_enum {
// Global link descriptor queued in REO
    HTT_RX_REO_RESOURCE_GLOBAL_LINK_DESC_COUNT_0           = 0,
    HTT_RX_REO_RESOURCE_GLOBAL_LINK_DESC_COUNT_1           = 1,
    HTT_RX_REO_RESOURCE_GLOBAL_LINK_DESC_COUNT_2           = 2,
// Number of queue descriptors of this aging group
    HTT_RX_REO_RESOURCE_BUFFERS_USED_AC0                   = 3,
    HTT_RX_REO_RESOURCE_BUFFERS_USED_AC1                   = 4,
    HTT_RX_REO_RESOURCE_BUFFERS_USED_AC2                   = 5,
    HTT_RX_REO_RESOURCE_BUFFERS_USED_AC3                   = 6,
// Total number of MSDUs buffered in AC
    HTT_RX_REO_RESOURCE_AGING_NUM_QUEUES_AC0               = 7,
    HTT_RX_REO_RESOURCE_AGING_NUM_QUEUES_AC1               = 8,
    HTT_RX_REO_RESOURCE_AGING_NUM_QUEUES_AC2               = 9,
    HTT_RX_REO_RESOURCE_AGING_NUM_QUEUES_AC3               = 10,

    HTT_RX_REO_RESOURCE_STATS_MAX                          = 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_reo_resource_stats_tlv_v {
// Variable based on the Number of records. HTT_RX_REO_RESOURCE_STATS_MAX
    pub sample_id: u32,
    pub total_max: u32,
    pub total_avg: u32,
    pub total_sample: u32,
    pub non_zeros_avg: u32,
    pub non_zeros_sample: u32,
    pub last_non_zeros_max: u32,
    pub last_non_zeros_min: u32,
    pub last_non_zeros_avg: u32,
    pub last_non_zeros_sample: u32,
}

// == TX SOUNDING STATS ==
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_txbf_sound_steer_modes {
    HTT_IMPLICIT_TXBF_STEER_STATS                = 0,
    HTT_EXPLICIT_TXBF_SU_SIFS_STEER_STATS        = 1,
    HTT_EXPLICIT_TXBF_SU_RBO_STEER_STATS         = 2,
    HTT_EXPLICIT_TXBF_MU_SIFS_STEER_STATS        = 3,
    HTT_EXPLICIT_TXBF_MU_RBO_STEER_STATS         = 4,
    HTT_TXBF_MAX_NUM_OF_MODES                    = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_stats_sounding_tx_mode {
    HTT_TX_AC_SOUNDING_MODE                      = 0,
    HTT_TX_AX_SOUNDING_MODE                      = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_sounding_stats_tlv {
    pub /: *mut *mut u32 tx_sounding_mode; / HTT_TX_XX_SOUNDING_MODE,
// Counts number of soundings for all steering modes in each bw
    pub cbf_20: [u32; HTT_TXBF_MAX_NUM_OF_MODES],
    pub cbf_40: [u32; HTT_TXBF_MAX_NUM_OF_MODES],
    pub cbf_80: [u32; HTT_TXBF_MAX_NUM_OF_MODES],
    pub cbf_160: [u32; HTT_TXBF_MAX_NUM_OF_MODES],
//
// The sounding array is a 2-D array stored as an 1-D array of
// u32. The stats for a particular user/bw combination is
// referenced with the following:
//
// sounding[(user* max_bw) + bw]
//
// ... where max_bw == 4 for 160mhz
//
    pub sounding: [u32; HTT_TX_NUM_OF_SOUNDING_STATS_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pdev_obss_pd_stats_tlv {
    pub num_obss_tx_ppdu_success: u32,
    pub num_obss_tx_ppdu_failure: u32,
    pub num_sr_tx_transmissions: u32,
    pub num_spatial_reuse_opportunities: u32,
    pub num_non_srg_opportunities: u32,
    pub num_non_srg_ppdu_tried: u32,
    pub num_non_srg_ppdu_success: u32,
    pub num_srg_opportunities: u32,
    pub num_srg_ppdu_tried: u32,
    pub num_srg_ppdu_success: u32,
    pub num_psr_opportunities: u32,
    pub num_psr_ppdu_tried: u32,
    pub num_psr_ppdu_success: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ring_backpressure_stats_tlv {
    pub pdev_id: u32,
    pub current_head_idx: u32,
    pub current_tail_idx: u32,
    pub num_htt_msgs_sent: u32,
// Time in milliseconds for which the ring has been in
// its current backpressure condition
//
    pub backpressure_time_ms: u32,
// backpressure_hist - histogram showing how many times
// different degrees of backpressure duration occurred:
// Index 0 indicates the number of times ring was
// continuously in backpressure state for 100 - 200ms.
// Index 1 indicates the number of times ring was
// continuously in backpressure state for 200 - 300ms.
// Index 2 indicates the number of times ring was
// continuously in backpressure state for 300 - 400ms.
// Index 3 indicates the number of times ring was
// continuously in backpressure state for 400 - 500ms.
// Index 4 indicates the number of times ring was
// continuously in backpressure state beyond 500ms.
//
    pub backpressure_hist: [u32; 5],
}

pub const HTT_TX_TXBF_RATE_STATS_NUM_MCS_COUNTERS: c_int = 14;
pub const HTT_TX_TXBF_RATE_STATS_NUM_BW_COUNTERS: c_int = 5;
pub const HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pdev_txrate_txbf_stats_tlv {
// SU TxBF TX MCS stats
    pub tx_su_txbf_mcs: [u32; HTT_TX_TXBF_RATE_STATS_NUM_MCS_COUNTERS],
// Implicit BF TX MCS stats
    pub tx_su_ibf_mcs: [u32; HTT_TX_TXBF_RATE_STATS_NUM_MCS_COUNTERS],
// Open loop TX MCS stats
    pub tx_su_ol_mcs: [u32; HTT_TX_TXBF_RATE_STATS_NUM_MCS_COUNTERS],
// SU TxBF TX NSS stats
    pub tx_su_txbf_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
// Implicit BF TX NSS stats
    pub tx_su_ibf_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
// Open loop TX NSS stats
    pub tx_su_ol_nss: [u32; HTT_TX_PDEV_STATS_NUM_SPATIAL_STREAMS],
// SU TxBF TX BW stats
    pub tx_su_txbf_bw: [u32; HTT_TX_TXBF_RATE_STATS_NUM_BW_COUNTERS],
// Implicit BF TX BW stats
    pub tx_su_ibf_bw: [u32; HTT_TX_TXBF_RATE_STATS_NUM_BW_COUNTERS],
// Open loop TX BW stats
    pub tx_su_ol_bw: [u32; HTT_TX_TXBF_RATE_STATS_NUM_BW_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_txbf_ofdma_ndpa_stats_tlv {
// 11AX HE OFDMA NDPA frame queued to the HW
    pub ax_ofdma_ndpa_queued: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA NDPA frame sent over the air
    pub ax_ofdma_ndpa_tried: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA NDPA frame flushed by HW
    pub ax_ofdma_ndpa_flushed: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA NDPA frame completed with error(s)
    pub ax_ofdma_ndpa_err: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_txbf_ofdma_ndp_stats_tlv {
// 11AX HE OFDMA NDP frame queued to the HW
    pub ax_ofdma_ndp_queued: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA NDPA frame sent over the air
    pub ax_ofdma_ndp_tried: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA NDPA frame flushed by HW
    pub ax_ofdma_ndp_flushed: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA NDPA frame completed with error(s)
    pub ax_ofdma_ndp_err: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_txbf_ofdma_brp_stats_tlv {
// 11AX HE OFDMA MU BRPOLL frame queued to the HW
    pub ax_ofdma_brpoll_queued: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA MU BRPOLL frame sent over the air
    pub ax_ofdma_brpoll_tried: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA MU BRPOLL frame flushed by HW
    pub ax_ofdma_brpoll_flushed: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA MU BRPOLL frame completed with error(s)
    pub ax_ofdma_brp_err: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// Number of CBF(s) received when 11AX HE OFDMA MU BRPOLL frame
// completed with error(s).
//
    pub 1]: u32 ax_ofdma_brp_err_num_cbf_rcvd[HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_txbf_ofdma_steer_stats_tlv {
// 11AX HE OFDMA PPDUs that were sent over the air with steering (TXBF + OFDMA)
    pub ax_ofdma_num_ppdu_steer: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA PPDUs that were sent over the air in open loop
    pub ax_ofdma_num_ppdu_ol: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA number of users for which CBF prefetch was
// initiated to PHY HW during TX.
//
    pub ax_ofdma_num_usrs_prefetch: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA number of users for which sounding was initiated during TX
    pub ax_ofdma_num_usrs_sound: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
// 11AX HE OFDMA number of users for which sounding was forced during TX
    pub ax_ofdma_num_usrs_force_sound: [u32; HTT_TX_PDEV_STATS_NUM_OFDMA_USER_STATS],
}

pub const HTT_MAX_RX_PKT_CNT: c_int = 8;
pub const HTT_MAX_RX_PKT_CRC_PASS_CNT: c_int = 8;
pub const HTT_MAX_PER_BLK_ERR_CNT: c_int = 20;
pub const HTT_MAX_RX_OTA_ERR_CNT: c_int = 14;
pub const HTT_STATS_MAX_CHAINS: c_int = 8;
pub const ATH11K_STATS_MGMT_FRM_TYPE_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_phy_counters_tlv {
// number of RXTD OFDMA OTA error counts except power surge and drop
    pub rx_ofdma_timing_err_cnt: u32,
// rx_cck_fail_cnt:
// number of cck error counts due to rx reception failure because of
// timing error in cck
//
    pub rx_cck_fail_cnt: u32,
// number of times tx abort initiated by mac
    pub mactx_abort_cnt: u32,
// number of times rx abort initiated by mac
    pub macrx_abort_cnt: u32,
// number of times tx abort initiated by phy
    pub phytx_abort_cnt: u32,
// number of times rx abort initiated by phy
    pub phyrx_abort_cnt: u32,
// number of rx deferred count initiated by phy
    pub phyrx_defer_abort_cnt: u32,
// number of sizing events generated at LSTF
    pub rx_gain_adj_lstf_event_cnt: u32,
// number of sizing events generated at non-legacy LTF
    pub rx_gain_adj_non_legacy_cnt: u32,
// rx_pkt_cnt -
// Received EOP (end-of-packet) count per packet type;
// [0] = 11a; [1] = 11b; [2] = 11n; [3] = 11ac; [4] = 11ax; [5] = GF
// [6-7]=RSVD
//
    pub rx_pkt_cnt: [u32; HTT_MAX_RX_PKT_CNT],
// rx_pkt_crc_pass_cnt -
// Received EOP (end-of-packet) count per packet type;
// [0] = 11a; [1] = 11b; [2] = 11n; [3] = 11ac; [4] = 11ax; [5] = GF
// [6-7]=RSVD
//
    pub rx_pkt_crc_pass_cnt: [u32; HTT_MAX_RX_PKT_CRC_PASS_CNT],
// per_blk_err_cnt -
// Error count per error source;
// [0] = unknown; [1] = LSIG; [2] = HTSIG; [3] = VHTSIG; [4] = HESIG;
// [5] = RXTD_OTA; [6] = RXTD_FATAL; [7] = DEMF; [8] = ROBE;
// [9] = PMI; [10] = TXFD; [11] = TXTD; [12] = PHYRF
// [13-19]=RSVD
//
    pub per_blk_err_cnt: [u32; HTT_MAX_PER_BLK_ERR_CNT],
// rx_ota_err_cnt -
// RXTD OTA (over-the-air) error count per error reason;
// [0] = voting fail; [1] = weak det fail; [2] = strong sig fail;
// [3] = cck fail; [4] = power surge; [5] = power drop;
// [6] = btcf timing timeout error; [7] = btcf packet detect error;
// [8] = coarse timing timeout error
// [9-13]=RSVD
//
    pub rx_ota_err_cnt: [u32; HTT_MAX_RX_OTA_ERR_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_phy_stats_tlv {
// per chain hw noise floor values in dBm
    pub nf_chain: [i32; HTT_STATS_MAX_CHAINS],
// number of false radars detected
    pub false_radar_cnt: u32,
// number of channel switches happened due to radar detection
    pub radar_cs_cnt: u32,
// ani_level -
// ANI level (noise interference) corresponds to the channel
// the desense levels range from -5 to 15 in dB units,
// higher values indicating more noise interference.
//
    pub ani_level: i32,
// running time in minutes since FW boot
    pub fw_run_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_phy_reset_counters_tlv {
    pub pdev_id: u32,
    pub cf_active_low_fail_cnt: u32,
    pub cf_active_low_pass_cnt: u32,
    pub phy_off_through_vreg_cnt: u32,
    pub force_calibration_cnt: u32,
    pub rf_mode_switch_phy_off_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_phy_reset_stats_tlv {
    pub pdev_id: u32,
    pub chan_mhz: u32,
    pub chan_band_center_freq1: u32,
    pub chan_band_center_freq2: u32,
    pub chan_phy_mode: u32,
    pub chan_flags: u32,
    pub chan_num: u32,
    pub reset_cause: u32,
    pub prev_reset_cause: u32,
    pub phy_warm_reset_src: u32,
    pub rx_gain_tbl_mode: u32,
    pub xbar_val: u32,
    pub force_calibration: u32,
    pub phyrf_mode: u32,
    pub phy_homechan: u32,
    pub phy_tx_ch_mask: u32,
    pub phy_rx_ch_mask: u32,
    pub phybb_ini_mask: u32,
    pub phyrf_ini_mask: u32,
    pub phy_dfs_en_mask: u32,
    pub phy_sscan_en_mask: u32,
    pub phy_synth_sel_mask: u32,
    pub phy_adfs_freq: u32,
    pub cck_fir_settings: u32,
    pub phy_dyn_pri_chan: u32,
    pub cca_thresh: u32,
    pub dyn_cca_status: u32,
    pub rxdesense_thresh_hw: u32,
    pub rxdesense_thresh_sw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_peer_ctrl_path_txrx_stats_tlv {
// peer mac address
    pub peer_mac_addr: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
// Num of tx mgmt frames with subtype on peer level
    pub peer_tx_mgmt_subtype: [u32; ATH11K_STATS_MGMT_FRM_TYPE_MAX],
// Num of rx mgmt frames with subtype on peer level
    pub peer_rx_mgmt_subtype: [u32; ATH11K_STATS_MGMT_FRM_TYPE_MAX],
}

extern "C" {
    pub fn ath11k_debugfs_htt_stats_init(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_debugfs_htt_stats_req(ar: *mut ath11k) -> c_int;
}

