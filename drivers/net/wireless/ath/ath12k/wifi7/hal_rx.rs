//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/wifi7/hal_rx.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_wbm_rel_info {
    pub cookie: u32,
    pub err_rel_src: hal_wbm_rel_src_module,
    pub push_reason: hal_reo_dest_ring_push_reason,
    pub err_code: u32,
    pub first_msdu: bool,
    pub last_msdu: bool,
    pub continuation: bool,
    pub rx_desc: *mut c_void,
    pub hw_cc_done: bool,
    pub peer_metadata: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mon_status_tlv_hdr {
    pub hdr: u32,
    pub value: [u8; ],
}

pub const HAL_TLV_STATUS_PPDU_NOT_DONE: c_int = 0;
pub const HAL_TLV_STATUS_PPDU_DONE: c_int = 1;
pub const HAL_TLV_STATUS_BUF_DONE: c_int = 2;
pub const HAL_TLV_STATUS_PPDU_NON_STD_DONE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_mon_status {
    HAL_RX_MON_STATUS_PPDU_NOT_DONE,
    HAL_RX_MON_STATUS_PPDU_DONE,
    HAL_RX_MON_STATUS_BUF_DONE,
    HAL_RX_MON_STATUS_BUF_ADDR,
    HAL_RX_MON_STATUS_MPDU_START,
    HAL_RX_MON_STATUS_MPDU_END,
    HAL_RX_MON_STATUS_MSDU_END,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ppdu_start {
    pub info0: __le32,
    pub info1: __le32,
    pub ppdu_start_ts_31_0: __le32,
    pub ppdu_start_ts_63_32: __le32,
    pub rsvd: [__le32; 2],
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
    pub usr_resp_ref: __le32,
    pub info6: __le32,
    pub rsvd3: [__le32; 4],
    pub info7: __le32,
    pub rsvd4: __le32,
    pub info8: __le32,
    pub rsvd5: [__le32; 2],
    pub usr_resp_ref_ext: __le32,
    pub rsvd6: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ppdu_end_user_stats_ext {
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub info3: __le32,
    pub info4: __le32,
    pub info5: __le32,
    pub info6: __le32,
    pub rsvd: __le32,
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

pub const HE_GI_0_8: c_int = 0;
pub const HE_GI_0_4: c_int = 1;
pub const HE_GI_1_6: c_int = 2;
pub const HE_GI_3_2: c_int = 3;
pub const HE_LTF_1_X: c_int = 0;
pub const HE_LTF_2_X: c_int = 1;
pub const HE_LTF_4_X: c_int = 2;

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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_ul_reception_type {
    HAL_RECEPTION_TYPE_ULOFMDA,
    HAL_RECEPTION_TYPE_ULMIMO,
    HAL_RECEPTION_TYPE_OTHER,
    HAL_RECEPTION_TYPE_FRAMELESS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_phyrx_rssi_legacy_info {
    pub info0: __le32,
    pub rsvd0: [__le32; 39],
    pub info1: __le32,
    pub info2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mpdu_start {
    pub rsvd0: [__le32; 9],
    pub info0: __le32,
    pub info1: __le32,
    pub rsvd1: [__le32; 2],
    pub info2: __le32,
    pub rsvd2: [__le32; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_msdu_end {
    pub info0: __le32,
    pub rsvd0: [__le32; 9],
    pub info00: __le16,
    pub info01: __le16,
    pub rsvd00: [__le32; 8],
    pub info1: __le32,
    pub rsvd1: [__le32; 10],
    pub info2: __le32,
    pub rsvd2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_ppdu_end_duration {
    pub rsvd0: [__le32; 9],
    pub info0: __le32,
    pub rsvd1: [__le32; 18],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_rxpcu_classification_overview {
    pub rsvd0: u32,
    pub __packed: },
pub const HAL_RX_NUM_MSDU_DESC: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_msdu_list {
    pub msdu_info: [hal_rx_msdu_desc_info; HAL_RX_NUM_MSDU_DESC],
    pub paddr: [u64; HAL_RX_NUM_MSDU_DESC],
    pub sw_cookie: [u32; HAL_RX_NUM_MSDU_DESC],
    pub rbm: [u8; HAL_RX_NUM_MSDU_DESC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_frame_bitmap_ack {
    pub reserved: __le32,
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub reserved1: [__le32; 10],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_resp_req_info {
    pub info0: __le32,
    pub reserved: [__le32; 1],
    pub info1: __le32,
    pub info2: __le32,
    pub reserved1: [__le32; 2],
    pub info3: __le32,
    pub info4: __le32,
    pub info5: __le32,
    pub reserved2: [__le32; 5],
    pub __packed: },
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_0: c_uint = 0xDDBEEF;
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_1: c_uint = 0xADBEEF;
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_2: c_uint = 0xBDBEEF;
pub const REO_QUEUE_DESC_MAGIC_DEBUG_PATTERN_3: c_uint = 0xCDBEEF;
// HE Radiotap data1 Mask
pub const HE_SU_FORMAT_TYPE: c_uint = 0x0000;
pub const HE_EXT_SU_FORMAT_TYPE: c_uint = 0x0001;
pub const HE_MU_FORMAT_TYPE: c_uint = 0x0002;
pub const HE_TRIG_FORMAT_TYPE: c_uint = 0x0003;
pub const HE_BEAM_CHANGE_KNOWN: c_uint = 0x0008;
pub const HE_DL_UL_KNOWN: c_uint = 0x0010;
pub const HE_MCS_KNOWN: c_uint = 0x0020;
pub const HE_DCM_KNOWN: c_uint = 0x0040;
pub const HE_CODING_KNOWN: c_uint = 0x0080;
pub const HE_LDPC_EXTRA_SYMBOL_KNOWN: c_uint = 0x0100;
pub const HE_STBC_KNOWN: c_uint = 0x0200;
pub const HE_DATA_BW_RU_KNOWN: c_uint = 0x4000;
pub const HE_DOPPLER_KNOWN: c_uint = 0x8000;
pub const HE_BSS_COLOR_KNOWN: c_uint = 0x0004;
// HE Radiotap data2 Mask
pub const HE_GI_KNOWN: c_uint = 0x0002;
pub const HE_TXBF_KNOWN: c_uint = 0x0010;
pub const HE_PE_DISAMBIGUITY_KNOWN: c_uint = 0x0020;
pub const HE_TXOP_KNOWN: c_uint = 0x0040;
pub const HE_LTF_SYMBOLS_KNOWN: c_uint = 0x0004;
pub const HE_PRE_FEC_PADDING_KNOWN: c_uint = 0x0008;
pub const HE_MIDABLE_PERIODICITY_KNOWN: c_uint = 0x0080;
// HE radiotap data3 shift values
pub const HE_BEAM_CHANGE_SHIFT: c_int = 6;
pub const HE_DL_UL_SHIFT: c_int = 7;
pub const HE_TRANSMIT_MCS_SHIFT: c_int = 8;
pub const HE_DCM_SHIFT: c_int = 12;
pub const HE_CODING_SHIFT: c_int = 13;
pub const HE_LDPC_EXTRA_SYMBOL_SHIFT: c_int = 14;
pub const HE_STBC_SHIFT: c_int = 15;
// HE radiotap data4 shift values
pub const HE_STA_ID_SHIFT: c_int = 4;
// HE radiotap data5
pub const HE_GI_SHIFT: c_int = 4;
pub const HE_LTF_SIZE_SHIFT: c_int = 6;
pub const HE_LTF_SYM_SHIFT: c_int = 8;
pub const HE_TXBF_SHIFT: c_int = 14;
pub const HE_PE_DISAMBIGUITY_SHIFT: c_int = 15;
pub const HE_PRE_FEC_PAD_SHIFT: c_int = 12;
// HE radiotap data6
pub const HE_DOPPLER_SHIFT: c_int = 4;
pub const HE_TXOP_SHIFT: c_int = 8;
// HE radiotap HE-MU flags1
pub const HE_SIG_B_MCS_KNOWN: c_uint = 0x0010;
pub const HE_SIG_B_DCM_KNOWN: c_uint = 0x0040;
pub const HE_SIG_B_SYM_NUM_KNOWN: c_uint = 0x8000;
pub const HE_RU_0_KNOWN: c_uint = 0x0100;
pub const HE_RU_1_KNOWN: c_uint = 0x0200;
pub const HE_RU_2_KNOWN: c_uint = 0x0400;
pub const HE_RU_3_KNOWN: c_uint = 0x0800;
pub const HE_DCM_FLAG_1_SHIFT: c_int = 5;
pub const HE_SPATIAL_REUSE_MU_KNOWN: c_uint = 0x0100;
pub const HE_SIG_B_COMPRESSION_FLAG_1_KNOWN: c_uint = 0x4000;
// HE radiotap HE-MU flags2
pub const HE_SIG_B_COMPRESSION_FLAG_2_SHIFT: c_int = 3;
pub const HE_BW_KNOWN: c_uint = 0x0004;
pub const HE_NUM_SIG_B_SYMBOLS_SHIFT: c_int = 4;
pub const HE_SIG_B_COMPRESSION_FLAG_2_KNOWN: c_uint = 0x0100;
pub const HE_NUM_SIG_B_FLAG_2_SHIFT: c_int = 9;
pub const HE_LTF_FLAG_2_SYMBOLS_SHIFT: c_int = 12;
pub const HE_LTF_KNOWN: c_uint = 0x8000;
// HE radiotap per_user_1
pub const HE_STA_SPATIAL_SHIFT: c_int = 11;
pub const HE_TXBF_SHIFT: c_int = 14;
pub const HE_RESERVED_SET_TO_1_SHIFT: c_int = 19;
pub const HE_STA_CODING_SHIFT: c_int = 20;
// HE radiotap per_user_2
pub const HE_STA_MCS_SHIFT: c_int = 4;
pub const HE_STA_DCM_SHIFT: c_int = 5;
// HE radiotap per user known
pub const HE_USER_FIELD_POSITION_KNOWN: c_uint = 0x01;
pub const HE_STA_ID_PER_USER_KNOWN: c_uint = 0x02;
pub const HE_STA_NSTS_KNOWN: c_uint = 0x04;
pub const HE_STA_TX_BF_KNOWN: c_uint = 0x08;
pub const HE_STA_SPATIAL_CONFIG_KNOWN: c_uint = 0x10;
pub const HE_STA_MCS_KNOWN: c_uint = 0x20;
pub const HE_STA_DCM_KNOWN: c_uint = 0x40;
pub const HE_STA_CODING_KNOWN: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_phyrx_common_user_info {
    pub rsvd: [__le32; 2],
    pub info0: __le32,
    pub rsvd1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_ndp_cmn_eb {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_usig_overflow {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_non_mu_mimo {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_mu_mimo {
    pub info0: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hal_eht_sig_user_field {
    pub mu_mimo: hal_eht_sig_mu_mimo,
    pub n_mu_mimo: hal_eht_sig_non_mu_mimo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_non_ofdma_cmn_eb {
    pub info0: __le32,
    pub user_field: hal_eht_sig_user_field,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_ofdma_cmn_eb1 {
    pub info0: __le64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_ofdma_cmn_eb2 {
    pub info0: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_eht_sig_ofdma_cmn_eb {
    pub eb1: hal_eht_sig_ofdma_cmn_eb1,
    pub eb2: hal_eht_sig_ofdma_cmn_eb2,
    pub user_field: hal_eht_sig_user_field,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_eht_bw {
    HAL_EHT_BW_20,
    HAL_EHT_BW_40,
    HAL_EHT_BW_80,
    HAL_EHT_BW_160,
    HAL_EHT_BW_320_1,
    HAL_EHT_BW_320_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_mon_usig_cmn {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_mon_usig_tb {
    pub info0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_mon_usig_mu {
    pub info0: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hal_mon_usig_non_cmn {
    pub tb: hal_mon_usig_tb,
    pub mu: hal_mon_usig_mu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_mon_usig_hdr {
    pub cmn: hal_mon_usig_cmn,
    pub non_cmn: hal_mon_usig_non_cmn,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_receive_user_info {
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub info3: __le32,
    pub user_fd_rssi_seg0: __le32,
    pub user_fd_rssi_seg1: __le32,
    pub user_fd_rssi_seg2: __le32,
    pub user_fd_rssi_seg3: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_mon_reception_type {
    HAL_RECEPTION_TYPE_SU,
    HAL_RECEPTION_TYPE_DL_MU_MIMO,
    HAL_RECEPTION_TYPE_DL_MU_OFMA,
    HAL_RECEPTION_TYPE_DL_MU_OFDMA_MIMO,
    HAL_RECEPTION_TYPE_UL_MU_MIMO,
    HAL_RECEPTION_TYPE_UL_MU_OFDMA,
    HAL_RECEPTION_TYPE_UL_MU_OFDMA_MIMO,
}

// Different allowed RU in 11BE

// MRUs spanning above 80Mhz
// HAL_EHT_RU_996_484 = HAL_EHT_RU_484 + HAL_EHT_RU_996 + 4 (reserved)
//

pub const NUM_RU_BITS_PER80: c_int = 16;
pub const NUM_RU_BITS_PER20: c_int = 4;
// Different per_80Mhz band in 320Mhz bandwidth
pub const HAL_80_0: c_int = 0;
pub const HAL_80_1: c_int = 1;
pub const HAL_80_2: c_int = 2;
pub const HAL_80_3: c_int = 3;

// MRU-996+484

// MRU-996x2+484

// MRU-996x3+484

    pub status): *mut hal_reo_status,
    pub status): *mut hal_reo_status,
    pub status): *mut hal_reo_status,
    pub status): *mut hal_reo_status,
    pub status): *mut hal_reo_status,
    pub status): *mut hal_reo_status,
    pub status): *mut hal_reo_status,
    pub rbm): *mut hal_rx_buf_return_buf_manager,
    pub action): hal_wbm_rel_bm_act,
    pub manager): dma_addr_t paddr, u32 cookie, u8,
    pub rbm): *mut *mut u32 cookie, u8,
    pub desc_bank): *mut *mut dma_addr_t paddr, u32,
    pub rel_info): *mut hal_rx_wbm_rel_info,
    pub cookie): *mut *mut dma_addr_t paddr, u32,
    pub msdu_cnt): *mut *mut u8 rbm, u32,
    pub num_msdus): *mut u16,
    pub srng): *mut hal_srng,
    pub srng): *mut hal_srng,
    pub ab): *mut void ath12k_wifi7_hal_reo_shared_qaddr_cache_clear(struct ath12k_base,
    pub ring_hash_map): *mut *mut void ath12k_wifi7_hal_reo_hw_setup(struct ath12k_base ab, u32,
    pub type): u32 start_seq, enum hal_pn_type,
