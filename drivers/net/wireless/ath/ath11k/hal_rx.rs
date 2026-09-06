//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/hal_rx.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_wbm_rel_info {
    pub cookie: u32,
    pub err_rel_src: hal_wbm_rel_src_module,
    pub push_reason: hal_reo_dest_ring_push_reason,
    pub err_code: u32,
    pub first_msdu: bool,
    pub last_msdu: bool,
}

pub const HAL_INVALID_PEERID: c_uint = 0xffff;
pub const VHT_SIG_SU_NSS_MASK: c_uint = 0x7;
pub const HAL_RX_MAX_MCS: c_int = 12;
pub const HAL_RX_MAX_NSS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mon_status_tlv_hdr {
    pub hdr: u32,
    pub value: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_su_mu_coding {
    HAL_RX_SU_MU_CODING_BCC,
    HAL_RX_SU_MU_CODING_LDPC,
    HAL_RX_SU_MU_CODING_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_gi {
    HAL_RX_GI_0_8_US,
    HAL_RX_GI_0_4_US,
    HAL_RX_GI_1_6_US,
    HAL_RX_GI_3_2_US,
    HAL_RX_GI_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_bw {
    HAL_RX_BW_20MHZ,
    HAL_RX_BW_40MHZ,
    HAL_RX_BW_80MHZ,
    HAL_RX_BW_160MHZ,
    HAL_RX_BW_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_preamble {
    HAL_RX_PREAMBLE_11A,
    HAL_RX_PREAMBLE_11B,
    HAL_RX_PREAMBLE_11N,
    HAL_RX_PREAMBLE_11AC,
    HAL_RX_PREAMBLE_11AX,
    HAL_RX_PREAMBLE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_reception_type {
    HAL_RX_RECEPTION_TYPE_SU,
    HAL_RX_RECEPTION_TYPE_MU_MIMO,
    HAL_RX_RECEPTION_TYPE_MU_OFDMA,
    HAL_RX_RECEPTION_TYPE_MU_OFDMA_MIMO,
    HAL_RX_RECEPTION_TYPE_MAX,
}

pub const HAL_RX_FCS_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_mon_status {
    HAL_RX_MON_STATUS_PPDU_NOT_DONE,
    HAL_RX_MON_STATUS_PPDU_DONE,
    HAL_RX_MON_STATUS_BUF_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_user_status {
    pub ul_ofdma_user_v0_word0: u32,
    pub ul_ofdma_user_v0_word1: u32,
    pub ast_index: u32,
    pub tid: u32,
    pub tcp_msdu_count: u16,
    pub udp_msdu_count: u16,
    pub other_msdu_count: u16,
    pub frame_control: u16,
    pub frame_control_info_valid: u8,
    pub data_sequence_control_info_valid: u8,
    pub first_data_seq_ctrl: u16,
    pub preamble_type: u32,
    pub ht_flags: u16,
    pub vht_flags: u16,
    pub he_flags: u16,
    pub rs_flags: u8,
    pub mpdu_cnt_fcs_ok: u32,
    pub mpdu_cnt_fcs_err: u32,
    pub mpdu_fcs_ok_bitmap: [u32; 8],
    pub mpdu_ok_byte_count: u32,
    pub mpdu_err_byte_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_sw_mon_ring_entries {
    pub mon_dst_paddr: dma_addr_t,
    pub mon_status_paddr: dma_addr_t,
    pub mon_dst_sw_cookie: u32,
    pub mon_status_sw_cookie: u32,
    pub dst_buf_addr_info: *mut c_void,
    pub status_buf_addr_info: *mut c_void,
    pub ppdu_id: u16,
    pub status_buf_count: u8,
    pub msdu_cnt: u8,
    pub end_of_ppdu: bool,
    pub drop_ppdu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mon_ppdu_info {
    pub ppdu_id: u32,
    pub ppdu_ts: u32,
    pub num_mpdu_fcs_ok: u32,
    pub num_mpdu_fcs_err: u32,
    pub preamble_type: u32,
    pub chan_num: u16,
    pub tcp_msdu_count: u16,
    pub tcp_ack_msdu_count: u16,
    pub udp_msdu_count: u16,
    pub other_msdu_count: u16,
    pub peer_id: u16,
    pub rate: u8,
    pub mcs: u8,
    pub nss: u8,
    pub bw: u8,
    pub vht_flag_values1: u8,
    pub vht_flag_values2: u8,
    pub vht_flag_values3: [u8; 4],
    pub vht_flag_values4: u8,
    pub vht_flag_values5: u8,
    pub vht_flag_values6: u16,
    pub is_stbc: u8,
    pub gi: u8,
    pub ldpc: u8,
    pub beamformed: u8,
    pub rssi_comb: u8,
    pub rssi_chain_pri20: [u8; HAL_RX_MAX_NSS],
    pub tid: u16,
    pub ht_flags: u16,
    pub vht_flags: u16,
    pub he_flags: u16,
    pub he_mu_flags: u16,
    pub dcm: u8,
    pub ru_alloc: u8,
    pub reception_type: u8,
    pub tsft: u64,
    pub rx_duration: u64,
    pub frame_control: u16,
    pub ast_index: u32,
    pub rs_fcs_err: u8,
    pub rs_flags: u8,
    pub cck_flag: u8,
    pub ofdm_flag: u8,
    pub ulofdma_flag: u8,
    pub frame_control_info_valid: u8,
    pub he_per_user_1: u16,
    pub he_per_user_2: u16,
    pub he_per_user_position: u8,
    pub he_per_user_known: u8,
    pub he_flags1: u16,
    pub he_flags2: u16,
    pub he_RU: [u8; 4],
    pub he_data1: u16,
    pub he_data2: u16,
    pub he_data3: u16,
    pub he_data4: u16,
    pub he_data5: u16,
    pub he_data6: u16,
    pub ppdu_len: u32,
    pub prev_ppdu_id: u32,
    pub device_id: u32,
    pub first_data_seq_ctrl: u16,
    pub monitor_direct_used: u8,
    pub data_sequence_control_info_valid: u8,
    pub ltf_size: u8,
    pub rxpcu_filter_pass: u8,
    pub rssi_chain: [c_char; 8][8],
    pub userstats: hal_rx_user_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ppdu_start {
    pub info0: __le32,
    pub chan_num: __le32,
    pub ppdu_start_ts: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ppdu_end_user_stats {
    pub rsvd0: [__le32; 2],
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub info3: __le32,
    pub ht_ctrl: __le32,
    pub rsvd1: [__le32; 2],
    pub info4: __le32,
    pub info5: __le32,
    pub info6: __le32,
    pub info7: __le32,
    pub rsvd2: [__le32; 4],
    pub info8: __le32,
    pub rsvd3: __le32,
    pub info9: __le32,
    pub rsvd4: [__le32; 2],
    pub info10: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ppdu_end_user_stats_ext {
    pub info0: u32,
    pub info1: u32,
    pub info2: u32,
    pub info3: u32,
    pub info4: u32,
    pub info5: u32,
    pub info6: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ht_sig_info {
    pub info0: __le32,
    pub info1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_lsig_b_info {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_lsig_a_info {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_vht_sig_a_info {
    pub info0: __le32,
    pub info1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_vht_sig_a_gi_setting {
    HAL_RX_VHT_SIG_A_NORMAL_GI = 0,
    HAL_RX_VHT_SIG_A_SHORT_GI = 1,
    HAL_RX_VHT_SIG_A_SHORT_GI_AMBIGUITY = 3,
}

pub const HAL_RX_SU_MU_CODING_LDPC: c_uint = 0x01;
pub const HE_GI_0_8: c_int = 0;
pub const HE_GI_0_4: c_int = 1;
pub const HE_GI_1_6: c_int = 2;
pub const HE_GI_3_2: c_int = 3;
pub const HE_LTF_1_X: c_int = 0;
pub const HE_LTF_2_X: c_int = 1;
pub const HE_LTF_4_X: c_int = 2;
pub const HE_LTF_UNKNOWN: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_he_sig_a_su_info {
    pub info0: __le32,
    pub info1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_he_sig_a_mu_dl_info {
    pub info0: __le32,
    pub info1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_he_sig_b1_mu_info {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_he_sig_b2_mu_info {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_he_sig_b2_ofdma_info {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_phyrx_chain_rssi {
    pub rssi_2040: __le32,
    pub rssi_80: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_phyrx_rssi_legacy_info {
    pub rsvd: [__le32; 3],
    pub pre_rssi: [hal_rx_phyrx_chain_rssi; HAL_RX_MAX_NSS],
    pub preamble: [hal_rx_phyrx_chain_rssi; HAL_RX_MAX_NSS],
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mpdu_info_ipq8074 {
    pub rsvd0: __le32,
    pub info0: __le32,
    pub rsvd1: [__le32; 11],
    pub info1: __le32,
    pub rsvd2: [__le32; 9],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mpdu_info_qcn9074 {
    pub rsvd0: [__le32; 10],
    pub info0: __le32,
    pub rsvd1: [__le32; 2],
    pub info1: __le32,
    pub rsvd2: [__le32; 9],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mpdu_info_wcn6855 {
    pub rsvd0: [__le32; 8],
    pub info0: __le32,
    pub rsvd1: [__le32; 14],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mpdu_info {
    pub ipq8074: hal_rx_mpdu_info_ipq8074,
    pub qcn9074: hal_rx_mpdu_info_qcn9074,
    pub wcn6855: hal_rx_mpdu_info_wcn6855,
    pub u: },
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ppdu_end_duration {
    pub rsvd0: [__le32; 9],
    pub info0: __le32,
    pub rsvd1: [__le32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_rxpcu_classification_overview {
    pub rsvd0: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_msdu_desc_info {
    pub msdu_flags: u32,
    pub /: *mut *mut u16 msdu_len; / 14 bits for length,
}

pub const HAL_RX_NUM_MSDU_DESC: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_msdu_list {
    pub msdu_info: [hal_rx_msdu_desc_info; HAL_RX_NUM_MSDU_DESC],
    pub sw_cookie: [u32; HAL_RX_NUM_MSDU_DESC],
    pub rbm: [u8; HAL_RX_NUM_MSDU_DESC],
}

extern "C" {
    pub fn ath11k_hal_reo_process_status(reo_desc: *mut u8, status: *mut u8) -> c_int;
}
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_0: c_uint = 0xDDBEEF;
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_1: c_uint = 0xADBEEF;
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_2: c_uint = 0xBDBEEF;
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_3: c_uint = 0xCDBEEF;
