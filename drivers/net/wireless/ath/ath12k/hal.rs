//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/hal.h
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

pub const HAL_DESC_REO_NON_QOS_TID: c_int = 16;
pub const HAL_INVALID_PEERID: c_uint = 0x3fff;
pub const VHT_SIG_SU_NSS_MASK: c_uint = 0x7;
pub const HAL_TX_ADDRX_EN: c_int = 1;
pub const HAL_TX_ADDRY_EN: c_int = 2;
pub const HAL_TX_ADDR_SEARCH_DEFAULT: c_int = 0;
pub const HAL_TX_ADDR_SEARCH_INDEX: c_int = 1;
pub const HAL_RX_MAX_MPDU: c_int = 256;

// TODO: 16 entries per radio times MAX_VAPS_SUPPORTED
pub const HAL_DSCP_TID_MAP_TBL_NUM_ENTRIES_MAX: c_int = 32;
pub const HAL_DSCP_TID_TBL_SIZE: c_int = 24;
pub const EHT_MAX_USER_INFO: c_int = 4;
pub const HAL_RX_MON_MAX_AGGR_SIZE: c_int = 128;
pub const HAL_MAX_UL_MU_USERS: c_int = 37;
pub const MAX_USER_POS: c_int = 8;
pub const MAX_MU_GROUP_ID: c_int = 64;
pub const MAX_MU_GROUP_SHOW: c_int = 16;

pub const HAL_LINK_DESC_ALIGN: c_int = 128;
pub const HAL_NUM_MPDUS_PER_LINK_DESC: c_int = 6;
pub const HAL_NUM_TX_MSDUS_PER_LINK_DESC: c_int = 7;
pub const HAL_NUM_RX_MSDUS_PER_LINK_DESC: c_int = 6;
pub const HAL_NUM_MPDU_LINKS_PER_QUEUE_DESC: c_int = 12;
pub const HAL_MAX_AVAIL_BLK_RES: c_int = 3;
pub const HAL_RING_BASE_ALIGN: c_int = 8;
pub const HAL_REO_QLUT_ADDR_ALIGN: c_int = 256;
pub const HAL_ADDR_LSB_REG_MASK: c_uint = 0xffffffff;
pub const HAL_ADDR_MSB_REG_SHIFT: c_int = 32;
pub const HAL_WBM2SW_REL_ERR_RING_NUM: c_int = 3;
pub const HAL_SHADOW_NUM_REGS_MAX: c_int = 40;
pub const HAL_WBM_IDLE_SCATTER_BUF_SIZE_MAX: c_int = 32704;
// TODO: Check with hw team on the supported scatter buf size
pub const HAL_WBM_IDLE_SCATTER_NEXT_PTR_SIZE: c_int = 8;

pub const HAL_AST_IDX_INVALID: c_uint = 0xFFFF;
pub const HAL_RX_MAX_MCS: c_int = 12;
pub const HAL_RX_MAX_MCS_HT: c_int = 31;
pub const HAL_RX_MAX_MCS_VHT: c_int = 9;
pub const HAL_RX_MAX_MCS_HE: c_int = 11;
pub const HAL_RX_MAX_MCS_BE: c_int = 15;
pub const HAL_RX_MAX_NSS: c_int = 8;
pub const HAL_RX_MAX_NUM_LEGACY_RATES: c_int = 12;

pub const HAL_RX_FCS_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_srng_ring_id {
    HAL_SRNG_RING_ID_REO2SW0 = 0,
    HAL_SRNG_RING_ID_REO2SW1,
    HAL_SRNG_RING_ID_REO2SW2,
    HAL_SRNG_RING_ID_REO2SW3,
    HAL_SRNG_RING_ID_REO2SW4,
    HAL_SRNG_RING_ID_REO2SW5,
    HAL_SRNG_RING_ID_REO2SW6,
    HAL_SRNG_RING_ID_REO2SW7,
    HAL_SRNG_RING_ID_REO2SW8,
    HAL_SRNG_RING_ID_REO2TCL,
    HAL_SRNG_RING_ID_REO2PPE,

    HAL_SRNG_RING_ID_SW2REO  = 16,
    HAL_SRNG_RING_ID_SW2REO1,
    HAL_SRNG_RING_ID_SW2REO2,
    HAL_SRNG_RING_ID_SW2REO3,

    HAL_SRNG_RING_ID_REO_CMD,
    HAL_SRNG_RING_ID_REO_STATUS,

    HAL_SRNG_RING_ID_SW2TCL1 = 24,
    HAL_SRNG_RING_ID_SW2TCL2,
    HAL_SRNG_RING_ID_SW2TCL3,
    HAL_SRNG_RING_ID_SW2TCL4,
    HAL_SRNG_RING_ID_SW2TCL5,
    HAL_SRNG_RING_ID_SW2TCL6,
    HAL_SRNG_RING_ID_PPE2TCL1 = 30,

    HAL_SRNG_RING_ID_SW2TCL_CMD = 40,
    HAL_SRNG_RING_ID_SW2TCL1_CMD,
    HAL_SRNG_RING_ID_TCL_STATUS,

    HAL_SRNG_RING_ID_CE0_SRC = 64,
    HAL_SRNG_RING_ID_CE1_SRC,
    HAL_SRNG_RING_ID_CE2_SRC,
    HAL_SRNG_RING_ID_CE3_SRC,
    HAL_SRNG_RING_ID_CE4_SRC,
    HAL_SRNG_RING_ID_CE5_SRC,
    HAL_SRNG_RING_ID_CE6_SRC,
    HAL_SRNG_RING_ID_CE7_SRC,
    HAL_SRNG_RING_ID_CE8_SRC,
    HAL_SRNG_RING_ID_CE9_SRC,
    HAL_SRNG_RING_ID_CE10_SRC,
    HAL_SRNG_RING_ID_CE11_SRC,
    HAL_SRNG_RING_ID_CE12_SRC,
    HAL_SRNG_RING_ID_CE13_SRC,
    HAL_SRNG_RING_ID_CE14_SRC,
    HAL_SRNG_RING_ID_CE15_SRC,

    HAL_SRNG_RING_ID_CE0_DST = 81,
    HAL_SRNG_RING_ID_CE1_DST,
    HAL_SRNG_RING_ID_CE2_DST,
    HAL_SRNG_RING_ID_CE3_DST,
    HAL_SRNG_RING_ID_CE4_DST,
    HAL_SRNG_RING_ID_CE5_DST,
    HAL_SRNG_RING_ID_CE6_DST,
    HAL_SRNG_RING_ID_CE7_DST,
    HAL_SRNG_RING_ID_CE8_DST,
    HAL_SRNG_RING_ID_CE9_DST,
    HAL_SRNG_RING_ID_CE10_DST,
    HAL_SRNG_RING_ID_CE11_DST,
    HAL_SRNG_RING_ID_CE12_DST,
    HAL_SRNG_RING_ID_CE13_DST,
    HAL_SRNG_RING_ID_CE14_DST,
    HAL_SRNG_RING_ID_CE15_DST,

    HAL_SRNG_RING_ID_CE0_DST_STATUS = 100,
    HAL_SRNG_RING_ID_CE1_DST_STATUS,
    HAL_SRNG_RING_ID_CE2_DST_STATUS,
    HAL_SRNG_RING_ID_CE3_DST_STATUS,
    HAL_SRNG_RING_ID_CE4_DST_STATUS,
    HAL_SRNG_RING_ID_CE5_DST_STATUS,
    HAL_SRNG_RING_ID_CE6_DST_STATUS,
    HAL_SRNG_RING_ID_CE7_DST_STATUS,
    HAL_SRNG_RING_ID_CE8_DST_STATUS,
    HAL_SRNG_RING_ID_CE9_DST_STATUS,
    HAL_SRNG_RING_ID_CE10_DST_STATUS,
    HAL_SRNG_RING_ID_CE11_DST_STATUS,
    HAL_SRNG_RING_ID_CE12_DST_STATUS,
    HAL_SRNG_RING_ID_CE13_DST_STATUS,
    HAL_SRNG_RING_ID_CE14_DST_STATUS,
    HAL_SRNG_RING_ID_CE15_DST_STATUS,

    HAL_SRNG_RING_ID_WBM_IDLE_LINK = 120,
    HAL_SRNG_RING_ID_WBM_SW0_RELEASE,
    HAL_SRNG_RING_ID_WBM_SW1_RELEASE,
    HAL_SRNG_RING_ID_WBM_PPE_RELEASE = 123,

    HAL_SRNG_RING_ID_WBM2SW0_RELEASE = 128,
    HAL_SRNG_RING_ID_WBM2SW1_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW2_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW3_RELEASE, /* RX ERROR RING */
    HAL_SRNG_RING_ID_WBM2SW4_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW5_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW6_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW7_RELEASE,

    HAL_SRNG_RING_ID_UMAC_ID_END = 159,

// Common DMAC rings shared by all LMACs
    HAL_SRNG_RING_ID_DMAC_CMN_ID_START = 160,
    HAL_SRNG_SW2RXDMA_BUF0 = HAL_SRNG_RING_ID_DMAC_CMN_ID_START,
    HAL_SRNG_SW2RXDMA_BUF1 = 161,
    HAL_SRNG_SW2RXDMA_BUF2 = 162,

    HAL_SRNG_SW2RXMON_BUF0 = 168,

    HAL_SRNG_SW2TXMON_BUF0 = 176,

    HAL_SRNG_RING_ID_DMAC_CMN_ID_END = 183,
    HAL_SRNG_RING_ID_PMAC1_ID_START = 184,

    HAL_SRNG_RING_ID_WMAC1_SW2RXMON_BUF0 = HAL_SRNG_RING_ID_PMAC1_ID_START,

    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA1_STATBUF,
    HAL_SRNG_RING_ID_WMAC1_RXDMA2SW0,
    HAL_SRNG_RING_ID_WMAC1_RXDMA2SW1,
    HAL_SRNG_RING_ID_WMAC1_RXMON2SW0 = HAL_SRNG_RING_ID_WMAC1_RXDMA2SW1,
    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA1_DESC,
    HAL_SRNG_RING_ID_RXDMA_DIR_BUF,
    HAL_SRNG_RING_ID_WMAC1_TXMON2SW0_BUF0,
    HAL_SRNG_RING_ID_WMAC1_SW2TXMON_BUF0,

    HAL_SRNG_RING_ID_PMAC1_ID_END,
}

// SRNG registers are split into two groups R0 and R2
pub const HAL_SRNG_REG_GRP_R0: c_int = 0;
pub const HAL_SRNG_REG_GRP_R2: c_int = 1;
pub const HAL_SRNG_NUM_REG_GRP: c_int = 2;
// TODO: number of PMACs
pub const HAL_SRNG_NUM_PMACS: c_int = 3;

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
    HAL_RX_BW_320MHZ,
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
    HAL_RX_PREAMBLE_11BA,
    HAL_RX_PREAMBLE_11BE,
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_legacy_rate {
    HAL_RX_LEGACY_RATE_LP_1_MBPS,
    HAL_RX_LEGACY_RATE_LP_2_MBPS,
    HAL_RX_LEGACY_RATE_LP_5_5_MBPS,
    HAL_RX_LEGACY_RATE_LP_11_MBPS,
    HAL_RX_LEGACY_RATE_SP_2_MBPS,
    HAL_RX_LEGACY_RATE_SP_5_5_MBPS,
    HAL_RX_LEGACY_RATE_SP_11_MBPS,
    HAL_RX_LEGACY_RATE_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_legacy_rates_ofdm {
    HAL_RX_LEGACY_RATE_OFDM_48_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_24_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_12_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_6_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_54_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_36_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_18_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_9_MBPS,
    HAL_RX_LEGACY_RATE_OFDM_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_ring_type {
    HAL_REO_DST,
    HAL_REO_EXCEPTION,
    HAL_REO_REINJECT,
    HAL_REO_CMD,
    HAL_REO_STATUS,
    HAL_TCL_DATA,
    HAL_TCL_CMD,
    HAL_TCL_STATUS,
    HAL_CE_SRC,
    HAL_CE_DST,
    HAL_CE_DST_STATUS,
    HAL_WBM_IDLE_LINK,
    HAL_SW2WBM_RELEASE,
    HAL_WBM2SW_RELEASE,
    HAL_RXDMA_BUF,
    HAL_RXDMA_DST,
    HAL_RXDMA_MONITOR_BUF,
    HAL_RXDMA_MONITOR_STATUS,
    HAL_RXDMA_MONITOR_DST,
    HAL_RXDMA_MONITOR_DESC,
    HAL_RXDMA_DIR_BUF,
    HAL_PPE2TCL,
    HAL_PPE_RELEASE,
    HAL_TX_MONITOR_BUF,
    HAL_TX_MONITOR_DST,
    HAL_MAX_RING_TYPES,
}

//
// enum hal_reo_cmd_type: Enum for REO command type
// @HAL_REO_CMD_GET_QUEUE_STATS: Get REO queue status/stats
// @HAL_REO_CMD_FLUSH_QUEUE: Flush all frames in REO queue
// @HAL_REO_CMD_FLUSH_CACHE: Flush descriptor entries in the cache
// @HAL_REO_CMD_UNBLOCK_CACHE: Unblock a descriptor's address that was blocked
// earlier with a 'REO_FLUSH_CACHE' command
// @HAL_REO_CMD_FLUSH_TIMEOUT_LIST: Flush buffers/descriptors from timeout list
// @HAL_REO_CMD_UPDATE_RX_QUEUE: Update REO queue settings
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_cmd_type {
    HAL_REO_CMD_GET_QUEUE_STATS     = 0,
    HAL_REO_CMD_FLUSH_QUEUE         = 1,
    HAL_REO_CMD_FLUSH_CACHE         = 2,
    HAL_REO_CMD_UNBLOCK_CACHE       = 3,
    HAL_REO_CMD_FLUSH_TIMEOUT_LIST  = 4,
    HAL_REO_CMD_UPDATE_RX_QUEUE     = 5,
}

//
// enum hal_reo_cmd_status: Enum for execution status of REO command
// @HAL_REO_CMD_SUCCESS: Command has successfully executed
// @HAL_REO_CMD_BLOCKED: Command could not be executed as the queue
// or cache was blocked
// @HAL_REO_CMD_FAILED: Command execution failed, could be due to
// invalid queue desc
// @HAL_REO_CMD_RESOURCE_BLOCKED: Command could not be executed because
// one or more descriptors were blocked
// @HAL_REO_CMD_DRAIN:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_cmd_status {
    HAL_REO_CMD_SUCCESS		= 0,
    HAL_REO_CMD_BLOCKED		= 1,
    HAL_REO_CMD_FAILED		= 2,
    HAL_REO_CMD_RESOURCE_BLOCKED	= 3,
    HAL_REO_CMD_DRAIN		= 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_tcl_encap_type {
    HAL_TCL_ENCAP_TYPE_RAW,
    HAL_TCL_ENCAP_TYPE_NATIVE_WIFI,
    HAL_TCL_ENCAP_TYPE_ETHERNET,
    HAL_TCL_ENCAP_TYPE_802_3 = 3,
    HAL_TCL_ENCAP_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_tcl_desc_type {
    HAL_TCL_DESC_TYPE_BUFFER,
    HAL_TCL_DESC_TYPE_EXT_DESC,
    HAL_TCL_DESC_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_dest_ring_buffer_type {
    HAL_REO_DEST_RING_BUFFER_TYPE_MSDU,
    HAL_REO_DEST_RING_BUFFER_TYPE_LINK_DESC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_dest_ring_push_reason {
    HAL_REO_DEST_RING_PUSH_REASON_ERR_DETECTED,
    HAL_REO_DEST_RING_PUSH_REASON_ROUTING_INSTRUCTION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_entr_rxdma_push_reason {
    HAL_REO_ENTR_RING_RXDMA_PUSH_REASON_ERR_DETECTED,
    HAL_REO_ENTR_RING_RXDMA_PUSH_REASON_ROUTING_INSTRUCTION,
    HAL_REO_ENTR_RING_RXDMA_PUSH_REASON_RX_FLUSH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_dest_ring_error_code {
    HAL_REO_DEST_RING_ERROR_CODE_DESC_ADDR_ZERO,
    HAL_REO_DEST_RING_ERROR_CODE_DESC_INVALID,
    HAL_REO_DEST_RING_ERROR_CODE_AMPDU_IN_NON_BA,
    HAL_REO_DEST_RING_ERROR_CODE_NON_BA_DUPLICATE,
    HAL_REO_DEST_RING_ERROR_CODE_BA_DUPLICATE,
    HAL_REO_DEST_RING_ERROR_CODE_FRAME_2K_JUMP,
    HAL_REO_DEST_RING_ERROR_CODE_BAR_2K_JUMP,
    HAL_REO_DEST_RING_ERROR_CODE_FRAME_OOR,
    HAL_REO_DEST_RING_ERROR_CODE_BAR_OOR,
    HAL_REO_DEST_RING_ERROR_CODE_NO_BA_SESSION,
    HAL_REO_DEST_RING_ERROR_CODE_FRAME_SN_EQUALS_SSN,
    HAL_REO_DEST_RING_ERROR_CODE_PN_CHECK_FAILED,
    HAL_REO_DEST_RING_ERROR_CODE_2K_ERR_FLAG_SET,
    HAL_REO_DEST_RING_ERROR_CODE_PN_ERR_FLAG_SET,
    HAL_REO_DEST_RING_ERROR_CODE_DESC_BLOCKED,
    HAL_REO_DEST_RING_ERROR_CODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_entr_rxdma_ecode {
    HAL_REO_ENTR_RING_RXDMA_ECODE_OVERFLOW_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_MPDU_LEN_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_FCS_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_DECRYPT_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_TKIP_MIC_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_UNECRYPTED_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_MSDU_LEN_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_MSDU_LIMIT_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_WIFI_PARSE_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_AMSDU_PARSE_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_SA_TIMEOUT_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_DA_TIMEOUT_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_FLOW_TIMEOUT_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_FLUSH_REQUEST_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_AMSDU_FRAG_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_MULTICAST_ECHO_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_AMSDU_MISMATCH_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_UNAUTH_WDS_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_GRPCAST_AMSDU_WDS_ERR,
    HAL_REO_ENTR_RING_RXDMA_ECODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_wbm_htt_tx_comp_status {
    HAL_WBM_REL_HTT_TX_COMP_STATUS_OK,
    HAL_WBM_REL_HTT_TX_COMP_STATUS_DROP,
    HAL_WBM_REL_HTT_TX_COMP_STATUS_TTL,
    HAL_WBM_REL_HTT_TX_COMP_STATUS_REINJ,
    HAL_WBM_REL_HTT_TX_COMP_STATUS_INSPECT,
    HAL_WBM_REL_HTT_TX_COMP_STATUS_MEC_NOTIFY,
    HAL_WBM_REL_HTT_TX_COMP_STATUS_VDEVID_MISMATCH,
    HAL_WBM_REL_HTT_TX_COMP_STATUS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_encrypt_type {
    HAL_ENCRYPT_TYPE_WEP_40,
    HAL_ENCRYPT_TYPE_WEP_104,
    HAL_ENCRYPT_TYPE_TKIP_NO_MIC,
    HAL_ENCRYPT_TYPE_WEP_128,
    HAL_ENCRYPT_TYPE_TKIP_MIC,
    HAL_ENCRYPT_TYPE_WAPI,
    HAL_ENCRYPT_TYPE_CCMP_128,
    HAL_ENCRYPT_TYPE_OPEN,
    HAL_ENCRYPT_TYPE_CCMP_256,
    HAL_ENCRYPT_TYPE_GCMP_128,
    HAL_ENCRYPT_TYPE_AES_GCMP_256,
    HAL_ENCRYPT_TYPE_WAPI_GCM_SM4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_tx_rate_stats_bw {
    HAL_TX_RATE_STATS_BW_20,
    HAL_TX_RATE_STATS_BW_40,
    HAL_TX_RATE_STATS_BW_80,
    HAL_TX_RATE_STATS_BW_160,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_tx_rate_stats_pkt_type {
    HAL_TX_RATE_STATS_PKT_TYPE_11A,
    HAL_TX_RATE_STATS_PKT_TYPE_11B,
    HAL_TX_RATE_STATS_PKT_TYPE_11N,
    HAL_TX_RATE_STATS_PKT_TYPE_11AC,
    HAL_TX_RATE_STATS_PKT_TYPE_11AX,
    HAL_TX_RATE_STATS_PKT_TYPE_11BA,
    HAL_TX_RATE_STATS_PKT_TYPE_11BE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_tx_rate_stats_sgi {
    HAL_TX_RATE_STATS_SGI_08US,
    HAL_TX_RATE_STATS_SGI_04US,
    HAL_TX_RATE_STATS_SGI_16US,
    HAL_TX_RATE_STATS_SGI_32US,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_wbm_idle_scatter_list {
    pub paddr: dma_addr_t,
    pub vaddr: *mut hal_wbm_link_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_srng_params {
    pub ring_base_paddr: dma_addr_t,
    pub ring_base_vaddr: *mut u32,
    pub num_entries: c_int,
    pub intr_batch_cntr_thres_entries: u32,
    pub intr_timer_thres_us: u32,
    pub flags: u32,
    pub max_buffer_len: u32,
    pub low_threshold: u32,
    pub high_threshold: u32,
    pub msi_addr: dma_addr_t,
    pub msi2_addr: dma_addr_t,
    pub msi_data: u32,
    pub msi2_data: u32,
// Add more params as needed
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_srng_dir {
    HAL_SRNG_DIR_SRC,
    HAL_SRNG_DIR_DST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_msdu_start_pkt_type {
    RX_MSDU_START_PKT_TYPE_11A,
    RX_MSDU_START_PKT_TYPE_11B,
    RX_MSDU_START_PKT_TYPE_11N,
    RX_MSDU_START_PKT_TYPE_11AC,
    RX_MSDU_START_PKT_TYPE_11AX,
    RX_MSDU_START_PKT_TYPE_11BA,
    RX_MSDU_START_PKT_TYPE_11BE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_msdu_start_sgi {
    RX_MSDU_START_SGI_0_8_US,
    RX_MSDU_START_SGI_0_4_US,
    RX_MSDU_START_SGI_1_6_US,
    RX_MSDU_START_SGI_3_2_US,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_msdu_start_recv_bw {
    RX_MSDU_START_RECV_BW_20MHZ,
    RX_MSDU_START_RECV_BW_40MHZ,
    RX_MSDU_START_RECV_BW_80MHZ,
    RX_MSDU_START_RECV_BW_160MHZ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_msdu_start_reception_type {
    RX_MSDU_START_RECEPTION_TYPE_SU,
    RX_MSDU_START_RECEPTION_TYPE_DL_MU_MIMO,
    RX_MSDU_START_RECEPTION_TYPE_DL_MU_OFDMA,
    RX_MSDU_START_RECEPTION_TYPE_DL_MU_OFDMA_MIMO,
    RX_MSDU_START_RECEPTION_TYPE_UL_MU_MIMO,
    RX_MSDU_START_RECEPTION_TYPE_UL_MU_OFDMA,
    RX_MSDU_START_RECEPTION_TYPE_UL_MU_OFDMA_MIMO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_desc_decap_type {
    RX_DESC_DECAP_TYPE_RAW,
    RX_DESC_DECAP_TYPE_NATIVE_WIFI,
    RX_DESC_DECAP_TYPE_ETHERNET2_DIX,
    RX_DESC_DECAP_TYPE_8023,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_user_status {
    pub ul_ofdma_user_v0_word0: u32,
    pub ul_ofdma_user_v0_word1: u32,
    pub ast_index: u32,
    pub tid: u32,
    pub tcp_msdu_count: u16,
    pub tcp_ack_msdu_count: u16,
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
    pub ldpc: u8,
    pub mpdu_cnt_fcs_ok: u32,
    pub mpdu_cnt_fcs_err: u32,
    pub mpdu_fcs_ok_bitmap: [u32; HAL_RX_NUM_WORDS_PER_PPDU_BITMAP],
    pub mpdu_ok_byte_count: u32,
    pub mpdu_err_byte_count: u32,
    pub ampdu_present: bool,
    pub ampdu_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_u_sig_info {
    pub ul_dl: bool,
    pub bw: u8,
    pub ppdu_type_comp_mode: u8,
    pub eht_sig_mcs: u8,
    pub num_eht_sig_sym: u8,
    pub usig: ieee80211_radiotap_eht_usig,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_tlv_aggr_info {
    pub in_progress: bool,
    pub cur_len: u16,
    pub tlv_tag: u16,
    pub buf: [u8; HAL_RX_MON_MAX_AGGR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_radiotap_eht {
    pub known: __le32,
    pub data: [__le32; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_eht_info {
    pub num_user_info: u8,
    pub eht: hal_rx_radiotap_eht,
    pub user_info: [u32; EHT_MAX_USER_INFO],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_msdu_desc_info {
    pub msdu_flags: u32,
    pub /: *mut *mut u16 msdu_len; / 14 bits for length,
}

// hal_mon_buf_ring
// Producer : SW
// Consumer : Monitor
//
// paddr_lo
// Lower 32-bit physical address of the buffer pointer from the source ring.
// paddr_hi
// bit range 7-0 : upper 8 bit of the physical address.
// bit range 31-8 : reserved.
// cookie
// Consumer: RxMon/TxMon 64 bit cookie of the buffers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_mon_buf_ring {
    pub paddr_lo: __le32,
    pub paddr_hi: __le32,
    pub cookie: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_mon_ppdu_info {
    pub ppdu_id: u32,
    pub last_ppdu_id: u32,
    pub ppdu_ts: u64,
    pub num_mpdu_fcs_ok: u32,
    pub num_mpdu_fcs_err: u32,
    pub preamble_type: u32,
    pub mpdu_len: u32,
    pub chan_num: u16,
    pub freq: u16,
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
    pub sgi: u8,
    pub ldpc: u8,
    pub beamformed: u8,
    pub rssi_comb: u8,
    pub tid: u16,
    pub fc_valid: u8,
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
    pub rssi_chain: [i8; 8][8],
    pub num_users: u32,
    pub mpdu_fcs_ok_bitmap: [u32; HAL_RX_NUM_WORDS_PER_PPDU_BITMAP],
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub addr4: [u8; ETH_ALEN],
    pub userstats: [hal_rx_user_status; HAL_MAX_UL_MU_USERS],
    pub userid: u8,
    pub first_msdu_in_mpdu: bool,
    pub is_ampdu: bool,
    pub medium_prot_type: u8,
    pub ppdu_continuation: bool,
    pub eht_usig: bool,
    pub u_sig_info: hal_rx_u_sig_info,
    pub is_eht: bool,
    pub eht_info: hal_rx_eht_info,
    pub tlv_aggr: hal_rx_tlv_aggr_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_rx_desc_data {
    pub rx_status: *mut ieee80211_rx_status,
    pub phy_meta_data: u32,
    pub err_bitmap: u32,
    pub enctype: u32,
    pub msdu_len: u16,
    pub peer_id: u16,
    pub seq_no: u16,
    pub addr2: *mut u8,
    pub pkt_type: u8,
    pub l3_pad_bytes: u8,
    pub decap_type: u8,
    pub bw: u8,
    pub rate_mcs: u8,
    pub nss: u8,
    pub sgi: u8,
    pub tid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_buffer_addr {
    pub info0: __le32,
    pub info1: __le32,
    pub __packed: },
// ath12k_buffer_addr
//
// buffer_addr_31_0
// Address (lower 32 bits) of the MSDU buffer or MSDU_EXTENSION
// descriptor or Link descriptor
//
// buffer_addr_39_32
// Address (upper 8 bits) of the MSDU buffer or MSDU_EXTENSION
// descriptor or Link descriptor
//
// return_buffer_manager (RBM)
// Consumer: WBM
// Producer: SW/FW
// Indicates to which buffer manager the buffer or MSDU_EXTENSION
// descriptor or link descriptor that is being pointed to shall be
// returned after the frame has been processed. It is used by WBM
// for routing purposes.
//
// Values are defined in enum %HAL_RX_BUF_RBM_
//
// sw_buffer_cookie
// Cookie field exclusively used by SW. HW ignores the contents,
// accept that it passes the programmed value on to other
// descriptors together with the physical address.
//
// Field can be used by SW to for example associate the buffers
// physical address with the virtual address.
//
// NOTE1:
// The three most significant bits can have a special meaning
// in case this struct is embedded in a TX_MPDU_DETAILS STRUCT,
// and field transmit_bw_restriction is set
//
// In case of NON punctured transmission:
// Sw_buffer_cookie[19:17] = 3'b000: 20 MHz TX only
// Sw_buffer_cookie[19:17] = 3'b001: 40 MHz TX only
// Sw_buffer_cookie[19:17] = 3'b010: 80 MHz TX only
// Sw_buffer_cookie[19:17] = 3'b011: 160 MHz TX only
// Sw_buffer_cookie[19:17] = 3'b101: 240 MHz TX only
// Sw_buffer_cookie[19:17] = 3'b100: 320 MHz TX only
// Sw_buffer_cookie[19:18] = 2'b11: reserved
//
// In case of punctured transmission:
// Sw_buffer_cookie[19:16] = 4'b0000: pattern 0 only
// Sw_buffer_cookie[19:16] = 4'b0001: pattern 1 only
// Sw_buffer_cookie[19:16] = 4'b0010: pattern 2 only
// Sw_buffer_cookie[19:16] = 4'b0011: pattern 3 only
// Sw_buffer_cookie[19:16] = 4'b0100: pattern 4 only
// Sw_buffer_cookie[19:16] = 4'b0101: pattern 5 only
// Sw_buffer_cookie[19:16] = 4'b0110: pattern 6 only
// Sw_buffer_cookie[19:16] = 4'b0111: pattern 7 only
// Sw_buffer_cookie[19:16] = 4'b1000: pattern 8 only
// Sw_buffer_cookie[19:16] = 4'b1001: pattern 9 only
// Sw_buffer_cookie[19:16] = 4'b1010: pattern 10 only
// Sw_buffer_cookie[19:16] = 4'b1011: pattern 11 only
// Sw_buffer_cookie[19:18] = 2'b11: reserved
//
// Note: a punctured transmission is indicated by the presence
// of TLV TX_PUNCTURE_SETUP embedded in the scheduler TLV
//
// Sw_buffer_cookie[20:17]: Tid: The TID field in the QoS control
// field
//
// Sw_buffer_cookie[16]: Mpdu_qos_control_valid: This field
// indicates MPDUs with a QoS control field.
//
    pub hal_ce_srng_dest_desc: struct,
    pub hal_ce_srng_dst_status_desc: struct,
    pub hal_ce_srng_src_desc: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_wbm_link_desc {
    pub buf_addr_info: ath12k_buffer_addr,
    pub __packed: },
// srng flags
pub const HAL_SRNG_FLAGS_MSI_SWAP: c_uint = 0x00000008;
pub const HAL_SRNG_FLAGS_RING_PTR_SWAP: c_uint = 0x00000010;
pub const HAL_SRNG_FLAGS_DATA_TLV_SWAP: c_uint = 0x00000020;
pub const HAL_SRNG_FLAGS_LOW_THRESH_INTR_EN: c_uint = 0x00010000;
pub const HAL_SRNG_FLAGS_MSI_INTR: c_uint = 0x00020000;
pub const HAL_SRNG_FLAGS_HIGH_THRESH_INTR_EN: c_uint = 0x00080000;
pub const HAL_SRNG_FLAGS_LMAC_RING: c_uint = 0x80000000;
// Common SRNG ring structure for source and destination rings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_srng {
// Unique SRNG ring ID
    pub ring_id: u8,
// Ring initialization done
    pub initialized: u8,
// Interrupt/MSI value assigned to this ring
    pub irq: c_int,
// Physical base address of the ring
    pub ring_base_paddr: dma_addr_t,
// Virtual base address of the ring
    pub ring_base_vaddr: *mut u32,
// Number of entries in ring
    pub num_entries: u32,
// Ring size
    pub ring_size: u32,
// Ring size mask
    pub ring_size_mask: u32,
// Size of ring entry
    pub entry_size: u32,
// Interrupt timer threshold - in micro seconds
    pub intr_timer_thres_us: u32,
// Interrupt batch counter threshold - in number of ring entries
    pub intr_batch_cntr_thres_entries: u32,
// MSI Address
    pub msi_addr: dma_addr_t,
// MSI data
    pub msi_data: u32,
// MSI2 Address
    pub msi2_addr: dma_addr_t,
// MSI2 data
    pub msi2_data: u32,
// Misc flags
    pub flags: u32,
// Lock for serializing ring index updates
    pub lock: spinlock_t,
    pub lock_key: lock_class_key,
// Start offset of SRNG register groups for this ring
// TBD: See if this is required - register address can be derived
// from ring ID
//
    pub hwreg_base: [u32; HAL_SRNG_NUM_REG_GRP],
    pub timestamp: u64,
// Source or Destination ring
    pub ring_dir: hal_srng_dir,
// SW tail pointer
    pub tp: u32,
// Shadow head pointer location to be updated by HW
    pub hp_addr: *mut volatile u32,
// Cached head pointer
    pub cached_hp: u32,
// Tail pointer location to be updated by SW - This
// will be a register address and need not be
// accessed through SW structure
//
    pub tp_addr: *mut u32,
// Current SW loop cnt
    pub loop_cnt: u32,
// max transfer size
    pub max_buffer_length: u16,
// head pointer at access end
    pub last_hp: u32,
    pub dst_ring: },
// SW head pointer
    pub hp: u32,
// SW reap head pointer
    pub reap_hp: u32,
// Shadow tail pointer location to be updated by HW
    pub tp_addr: *mut u32,
// Cached tail pointer
    pub cached_tp: u32,
// Head pointer location to be updated by SW - This
// will be a register address and need not be accessed
// through SW structure
//
    pub hp_addr: *mut u32,
// Low threshold - in number of ring entries
    pub low_threshold: u32,
// tail pointer at access end
    pub last_tp: u32,
    pub src_ring: },
    pub u: },
}

// hal_wbm_link_desc
//
// Producer: WBM
// Consumer: WBM
//
// buf_addr_info
// Details of the physical address of a buffer or MSDU
// link descriptor.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_wbm_rel_src_module {
    HAL_WBM_REL_SRC_MODULE_TQM,
    HAL_WBM_REL_SRC_MODULE_RXDMA,
    HAL_WBM_REL_SRC_MODULE_REO,
    HAL_WBM_REL_SRC_MODULE_FW,
    HAL_WBM_REL_SRC_MODULE_SW,
    HAL_WBM_REL_SRC_MODULE_MAX,
}

// hal_wbm_rel_desc_type
//
// msdu_buffer
// The address points to an MSDU buffer
//
// msdu_link_descriptor
// The address points to an Tx MSDU link descriptor
//
// mpdu_link_descriptor
// The address points to an MPDU link descriptor
//
// msdu_ext_descriptor
// The address points to an MSDU extension descriptor
//
// queue_ext_descriptor
// The address points to an TQM queue extension descriptor. WBM should
// treat this is the same way as a link descriptor.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_wbm_rel_desc_type {
    HAL_WBM_REL_DESC_TYPE_REL_MSDU,
    HAL_WBM_REL_DESC_TYPE_MSDU_LINK,
    HAL_WBM_REL_DESC_TYPE_MPDU_LINK,
    HAL_WBM_REL_DESC_TYPE_MSDU_EXT,
    HAL_WBM_REL_DESC_TYPE_QUEUE_EXT,
}

// Interrupt mitigation - Batch threshold in terms of number of frames
pub const HAL_SRNG_INT_BATCH_THRESHOLD_TX: c_int = 256;
pub const HAL_SRNG_INT_BATCH_THRESHOLD_RX: c_int = 128;
pub const HAL_SRNG_INT_BATCH_THRESHOLD_OTHER: c_int = 1;
// Interrupt mitigation - timer threshold in us
pub const HAL_SRNG_INT_TIMER_THRESHOLD_TX: c_int = 1000;
pub const HAL_SRNG_INT_TIMER_THRESHOLD_RX: c_int = 200;
pub const HAL_SRNG_INT_TIMER_THRESHOLD_OTHER: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_srng_mac_type {
    ATH12K_HAL_SRNG_UMAC,
    ATH12K_HAL_SRNG_DMAC,
    ATH12K_HAL_SRNG_PMAC
}

// HW SRNG configuration table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_srng_config {
    pub start_ring_id: c_int,
    pub max_rings: u16,
    pub entry_size: u16,
    pub reg_start: [u32; HAL_SRNG_NUM_REG_GRP],
    pub reg_size: [u16; HAL_SRNG_NUM_REG_GRP],
    pub mac_type: hal_srng_mac_type,
    pub ring_dir: hal_srng_dir,
    pub max_size: u32,
}

//
// enum hal_rx_buf_return_buf_manager - manager for returned rx buffers
//
// @HAL_RX_BUF_RBM_WBM_IDLE_BUF_LIST: Buffer returned to WBM idle buffer list
// @HAL_RX_BUF_RBM_WBM_DEV0_IDLE_DESC_LIST: Descriptor returned to WBM idle
// descriptor list, where the device 0 WBM is chosen in case of a multi-device config
// @HAL_RX_BUF_RBM_WBM_DEV1_IDLE_DESC_LIST: Descriptor returned to WBM idle
// descriptor list, where the device 1 WBM is chosen in case of a multi-device config
// @HAL_RX_BUF_RBM_WBM_DEV2_IDLE_DESC_LIST: Descriptor returned to WBM idle
// descriptor list, where the device 2 WBM is chosen in case of a multi-device config
// @HAL_RX_BUF_RBM_FW_BM: Buffer returned to FW
// @HAL_RX_BUF_RBM_SW0_BM: For ring 0 -- returned to host
// @HAL_RX_BUF_RBM_SW1_BM: For ring 1 -- returned to host
// @HAL_RX_BUF_RBM_SW2_BM: For ring 2 -- returned to host
// @HAL_RX_BUF_RBM_SW3_BM: For ring 3 -- returned to host
// @HAL_RX_BUF_RBM_SW4_BM: For ring 4 -- returned to host
// @HAL_RX_BUF_RBM_SW5_BM: For ring 5 -- returned to host
// @HAL_RX_BUF_RBM_SW6_BM: For ring 6 -- returned to host
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_buf_return_buf_manager {
    HAL_RX_BUF_RBM_WBM_IDLE_BUF_LIST,
    HAL_RX_BUF_RBM_WBM_DEV0_IDLE_DESC_LIST,
    HAL_RX_BUF_RBM_WBM_DEV1_IDLE_DESC_LIST,
    HAL_RX_BUF_RBM_WBM_DEV2_IDLE_DESC_LIST,
    HAL_RX_BUF_RBM_FW_BM,
    HAL_RX_BUF_RBM_SW0_BM,
    HAL_RX_BUF_RBM_SW1_BM,
    HAL_RX_BUF_RBM_SW2_BM,
    HAL_RX_BUF_RBM_SW3_BM,
    HAL_RX_BUF_RBM_SW4_BM,
    HAL_RX_BUF_RBM_SW5_BM,
    HAL_RX_BUF_RBM_SW6_BM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hal_reo_cmd {
    pub addr_lo: u32,
    pub flag: u32,
    pub upd0: u32,
    pub upd1: u32,
    pub upd2: u32,
    pub pn: [u32; 4],
    pub rx_queue_num: u16,
    pub min_rel: u16,
    pub min_fwd: u16,
    pub addr_hi: u8,
    pub ac_list: u8,
    pub blocking_idx: u8,
    pub ba_window_size: u16,
    pub pn_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_pn_type {
    HAL_PN_TYPE_NONE,
    HAL_PN_TYPE_WPA,
    HAL_PN_TYPE_WAPI_EVEN,
    HAL_PN_TYPE_WAPI_UNEVEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_ce_desc {
    HAL_CE_DESC_SRC,
    HAL_CE_DESC_DST,
    HAL_CE_DESC_DST_STATUS,
}

pub const HAL_HASH_ROUTING_RING_TCL: c_int = 0;
pub const HAL_HASH_ROUTING_RING_SW1: c_int = 1;
pub const HAL_HASH_ROUTING_RING_SW2: c_int = 2;
pub const HAL_HASH_ROUTING_RING_SW3: c_int = 3;
pub const HAL_HASH_ROUTING_RING_SW4: c_int = 4;
pub const HAL_HASH_ROUTING_RING_REL: c_int = 5;
pub const HAL_HASH_ROUTING_RING_FW: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_header {
    pub cmd_num: u16,
    pub cmd_status: hal_reo_cmd_status,
    pub cmd_exe_time: u16,
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_hal_params {
    pub rx_buf_rbm: hal_rx_buf_return_buf_manager,
    pub wbm2sw_cc_enable: u32,
}

pub const ATH12K_HW_REG_UNDEFINED: c_uint = 0xdeadbeaf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_regs {
    pub tcl1_ring_id: u32,
    pub tcl1_ring_misc: u32,
    pub tcl1_ring_tp_addr_lsb: u32,
    pub tcl1_ring_tp_addr_msb: u32,
    pub tcl1_ring_consumer_int_setup_ix0: u32,
    pub tcl1_ring_consumer_int_setup_ix1: u32,
    pub tcl1_ring_msi1_base_lsb: u32,
    pub tcl1_ring_msi1_base_msb: u32,
    pub tcl1_ring_msi1_data: u32,
    pub tcl_ring_base_lsb: u32,
    pub tcl1_ring_base_lsb: u32,
    pub tcl1_ring_base_msb: u32,
    pub tcl2_ring_base_lsb: u32,
    pub tcl_status_ring_base_lsb: u32,
    pub reo1_qdesc_addr: u32,
    pub reo1_qdesc_max_peerid: u32,
    pub wbm_idle_ring_base_lsb: u32,
    pub wbm_idle_ring_misc_addr: u32,
    pub wbm_r0_idle_list_cntl_addr: u32,
    pub wbm_r0_idle_list_size_addr: u32,
    pub wbm_scattered_ring_base_lsb: u32,
    pub wbm_scattered_ring_base_msb: u32,
    pub wbm_scattered_desc_head_info_ix0: u32,
    pub wbm_scattered_desc_head_info_ix1: u32,
    pub wbm_scattered_desc_tail_info_ix0: u32,
    pub wbm_scattered_desc_tail_info_ix1: u32,
    pub wbm_scattered_desc_ptr_hp_addr: u32,
    pub wbm_sw_release_ring_base_lsb: u32,
    pub wbm_sw1_release_ring_base_lsb: u32,
    pub wbm0_release_ring_base_lsb: u32,
    pub wbm1_release_ring_base_lsb: u32,
    pub pcie_qserdes_sysclk_en_sel: u32,
    pub pcie_pcs_osc_dtct_config_base: u32,
    pub umac_ce0_src_reg_base: u32,
    pub umac_ce0_dest_reg_base: u32,
    pub umac_ce1_src_reg_base: u32,
    pub umac_ce1_dest_reg_base: u32,
    pub ppe_rel_ring_base: u32,
    pub reo2_ring_base: u32,
    pub reo1_misc_ctrl_addr: u32,
    pub reo1_sw_cookie_cfg0: u32,
    pub reo1_sw_cookie_cfg1: u32,
    pub reo1_qdesc_lut_base0: u32,
    pub reo1_qdesc_lut_base1: u32,
    pub reo1_ring_base_lsb: u32,
    pub reo1_ring_base_msb: u32,
    pub reo1_ring_id: u32,
    pub reo1_ring_misc: u32,
    pub reo1_ring_hp_addr_lsb: u32,
    pub reo1_ring_hp_addr_msb: u32,
    pub reo1_ring_producer_int_setup: u32,
    pub reo1_ring_msi1_base_lsb: u32,
    pub reo1_ring_msi1_base_msb: u32,
    pub reo1_ring_msi1_data: u32,
    pub reo1_aging_thres_ix0: u32,
    pub reo1_aging_thres_ix1: u32,
    pub reo1_aging_thres_ix2: u32,
    pub reo1_aging_thres_ix3: u32,
    pub reo2_sw0_ring_base: u32,
    pub sw2reo_ring_base: u32,
    pub sw2reo1_ring_base: u32,
    pub reo_cmd_ring_base: u32,
    pub reo_status_ring_base: u32,
    pub gcc_gcc_pcie_hot_rst: u32,
    pub qrtr_node_id: u32,
}

// HAL context to be used to access SRNG APIs (currently used by data path
// and transport (CE) modules)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hal {
// HAL internal state for all SRNG rings.
//
    pub srng_list: [hal_srng; HAL_SRNG_RING_ID_MAX],
// SRNG configuration table
    pub srng_config: *mut hal_srng_config,
// Remote pointer memory for HW/FW updates
    pub vaddr: *mut u32,
    pub paddr: dma_addr_t,
    pub rdp: },
// Shared memory for ring pointer updates from host to FW
    pub vaddr: *mut u32,
    pub paddr: dma_addr_t,
    pub wrp: },
    pub dev: *mut device,
    pub ops: *const hal_ops,
    pub regs: *const ath12k_hw_regs,
    pub hal_params: *const ath12k_hw_hal_params,
// Available REO blocking resources bitmap
    pub avail_blk_resource: u8,
    pub current_blk_index: u8,
// shadow register configuration
    pub shadow_reg_addr: [u32; HAL_SHADOW_NUM_REGS_MAX],
    pub num_shadow_reg_configured: c_int,
    pub hal_desc_sz: u32,
    pub hal_wbm_release_ring_tx_size: u32,
    pub tcl_to_wbm_rbm_map: *const ath12k_hal_tcl_to_wbm_rbm_map,
}

// Maps WBM ring number and Return Buffer Manager Id per TCL ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hal_tcl_to_wbm_rbm_map {
    pub wbm_ring_num: u8,
    pub rbm_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_wbm_rel_bm_act {
    HAL_WBM_REL_BM_ACT_PUT_IN_IDLE,
    HAL_WBM_REL_BM_ACT_REL_MSDU,
}

// hal_wbm_rel_bm_act
//
// put_in_idle_list
// Put the buffer or descriptor back in the idle list. In case of MSDU or
// MDPU link descriptor, BM does not need to check to release any
// individual MSDU buffers.
//
// release_msdu_list
// This BM action can only be used in combination with desc_type being
// msdu_link_descriptor. Field first_msdu_index points out which MSDU
// pointer in the MSDU link descriptor is the first of an MPDU that is
// released. BM shall release all the MSDU buffers linked to this first
// MSDU buffer pointer. All related MSDU buffer pointer entries shall be
// set to value 0, which represents the 'NULL' pointer. When all MSDU
// buffer pointers in the MSDU link descriptor are 'NULL', the MSDU link
// descriptor itself shall also be released.
//
pub const RU_INVALID: c_int = 0;
pub const RU_26: c_int = 1;
pub const RU_52: c_int = 2;
pub const RU_106: c_int = 4;
pub const RU_242: c_int = 9;
pub const RU_484: c_int = 18;
pub const RU_996: c_int = 37;
pub const RU_2X996: c_int = 74;
pub const RU_3X996: c_int = 111;
pub const RU_4X996: c_int = 148;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_eht_ru_size {
    ATH12K_EHT_RU_26,
    ATH12K_EHT_RU_52,
    ATH12K_EHT_RU_106,
    ATH12K_EHT_RU_242,
    ATH12K_EHT_RU_484,
    ATH12K_EHT_RU_996,
    ATH12K_EHT_RU_996x2,
    ATH12K_EHT_RU_996x4,
    ATH12K_EHT_RU_52_26,
    ATH12K_EHT_RU_106_26,
    ATH12K_EHT_RU_484_242,
    ATH12K_EHT_RU_996_484,
    ATH12K_EHT_RU_996_484_242,
    ATH12K_EHT_RU_996x2_484,
    ATH12K_EHT_RU_996x3,
    ATH12K_EHT_RU_996x3_484,

// Keep last
    ATH12K_EHT_RU_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hw_version_map {
    pub hal_ops: *const hal_ops,
    pub hal_desc_sz: u32,
    pub tcl_to_wbm_rbm_map: *const ath12k_hal_tcl_to_wbm_rbm_map,
    pub hal_params: *const ath12k_hw_hal_params,
    pub hw_regs: *const ath12k_hw_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_ops {
    pub hal): *mut *mut int (create_srng_config)(struct ath12k_hal,
    pub len): *mut *mut *mut void (rx_desc_set_msdu_len)(struct hal_rx_desc desc, u16,
    pub hdr): *mut ieee80211_hdr,
    pub enctype): hal_encrypt_type,
    pub ldesc): *mut hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_msdu_src_link_id)(struct hal_rx_desc,
    pub ldesc): *mut hal_rx_desc,
    pub desc): *mut *mut u32 (rx_desc_get_mpdu_start_tag)(struct hal_rx_desc,
    pub desc): *mut *mut u32 (rx_desc_get_mpdu_ppdu_id)(struct hal_rx_desc,
    pub desc): *mut *mut u8 (rx_desc_get_l3_pad_bytes)(struct hal_rx_desc,
    pub desc): *mut *mut *mut u8 (rx_desc_get_msdu_payload)(struct hal_rx_desc,
    pub ring_num): *mut *mut hal_srng srng, int,
    pub srng): *mut hal_srng,
    pub srng): *mut *mut *mut void (srng_src_hw_init)(struct ath12k_base ab, struct hal_srng,
    pub srng): *mut *mut *mut void (srng_dst_hw_init)(struct ath12k_base ab, struct hal_srng,
    pub ring_num): c_int,
    pub mac_id): int ring_num, int,
    pub type): *mut *mut u32 (ce_get_desc_size)(enum hal_ce_desc,
    pub byte_swap_data): u8,
    pub paddr): dma_addr_t,
    pub desc): *mut *mut u32 (ce_dst_status_get_length)(struct hal_ce_srng_dst_status_desc,
    pub rbm): hal_rx_buf_return_buf_manager,
    pub id): *mut *mut *mut void (tx_set_dscp_tid_map)(struct ath12k_base ab, int,
    pub bank_id): u32 bank_config, u8,
    pub ab): *mut *mut void (reoq_lut_addr_read_enable)(struct ath12k_base,
    pub ab): *mut *mut void (reoq_lut_set_max_peerid)(struct ath12k_base,
    pub paddr): dma_addr_t,
    pub paddr): *mut *mut *mut void (write_reoq_lut_addr)(struct ath12k_base ab, dma_addr_t,
    pub end_offset): u32,
    pub srng): *mut hal_srng,
    pub ab): *mut *mut void (reo_shared_qaddr_cache_clear)(struct ath12k_base,
    pub ring_hash_map): *mut *mut *mut void (reo_hw_setup)(struct ath12k_base ab, u32,
    pub manager): dma_addr_t paddr, u32 cookie, u8,
    pub rbm): *mut u8,
    pub ab): *mut *mut void (cc_config)(struct ath12k_base,
    pub device_id): *mut *mut *mut (get_idle_link_rbm)(struct ath12k_hal hal, u8,
    pub num_msdus): *mut u16,
    pub msdu_cnt): *mut *mut u8 rbm, u32,
    pub len): *mut *mut *mut *mut void (reo_cmd_enc_tlv_hdr)(void tlv, u64 tag, u64,
    pub desc): *mut *mut *mut u16 (reo_status_dec_tlv_hdr)(void tlv, void,
    pub usrid): *mut *mut *mut *mut *mut *mut void (mon_rx_status_dec_tlv_hdr)(void tlv, u16 tag, u16 len, u16,
    pub (*get_tlv_hdr_align)(void): *mut u32,
}

pub const HAL_TLV_ALIGN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tlv_hdr {
    pub tl: __le32,
    pub value: [u8; ],
    pub __packed: },

pub const HAL_TLV_64_ALIGN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tlv_64_hdr {
    pub tl: __le64,
    pub value: [u8; ],
    pub __packed: },
    pub srng): *mut hal_srng,
    pub srng): *mut hal_srng,
    pub type): *mut *mut u32 ath12k_hal_ce_get_desc_size(struct ath12k_hal hal, enum hal_ce_desc,
    pub paddr): dma_addr_t,
    pub byte_swap_data): u8,
    pub ring_type): *mut *mut int ath12k_hal_srng_get_entrysize(struct ath12k_base ab, u32,
    pub ring_type): *mut *mut int ath12k_hal_srng_get_max_entries(struct ath12k_base ab, u32,
    pub params): *mut hal_srng_params,
    pub srng): *mut hal_srng,
    pub srng): *mut *mut *mut void ath12k_hal_srng_src_peek(struct ath12k_base ab, struct hal_srng,
    pub srng): *mut *mut *mut void ath12k_hal_srng_dst_peek(struct ath12k_base ab, struct hal_srng,
    pub sync_hw_ptr): bool,
    pub srng): *mut hal_srng,
    pub srng): *mut hal_srng,
    pub srng): *mut hal_srng,
    pub srng): *mut hal_srng,
    pub sync_hw_ptr): bool,
    pub srng): *mut hal_srng,
    pub srng): *mut *mut void ath12k_hal_srng_access_end(struct ath12k_base ab, struct hal_srng,
    pub params): *mut hal_srng_params,
    pub ath12k): *mut int ath12k_hal_srng_init(struct ath12k_base,
    pub ath12k): *mut void ath12k_hal_srng_deinit(struct ath12k_base,
    pub ab): *mut void ath12k_hal_dump_srng_stats(struct ath12k_base,
    pub len): *mut *mut *mut u32 cfg, u32,
    pub ring_num): c_int,
    pub ab): *mut void ath12k_hal_srng_shadow_config(struct ath12k_base,
    pub srng): *mut hal_srng,
    pub ab): *mut void ath12k_hal_reo_shared_qaddr_cache_clear(struct ath12k_base,
    pub rbm): dma_addr_t paddr, int,
    pub end_offset): u32,
    pub desc): *mut hal_ce_srng_dst_status_desc,
    pub id): *mut *mut void ath12k_hal_tx_set_dscp_tid_map(struct ath12k_base ab, int,
    pub bank_id): u32 bank_config, u8,
    pub ab): *mut void ath12k_hal_reoq_lut_addr_read_enable(struct ath12k_base,
    pub ab): *mut void ath12k_hal_reoq_lut_set_max_peerid(struct ath12k_base,
    pub paddr): *mut *mut void ath12k_hal_write_reoq_lut_addr(struct ath12k_base ab, dma_addr_t,
    pub paddr): *mut *mut ath12k_hal_write_ml_reoq_lut_addr(struct ath12k_base ab, dma_addr_t,
    pub srng): *mut *mut void ath12k_hal_reo_init_cmd_ring(struct ath12k_base ab, struct hal_srng,
    pub ring_hash_map): *mut *mut void ath12k_hal_reo_hw_setup(struct ath12k_base ab, u32,
    pub manager): dma_addr_t paddr, u32 cookie, u8,
    pub rbm): *mut u8,
    pub ab): *mut void ath12k_hal_cc_config(struct ath12k_base,
    pub device_id): *mut *mut ath12k_hal_get_idle_link_rbm(struct ath12k_hal hal, u8,
    pub num_msdus): *mut u16,
    pub msdu_cnt): *mut *mut u8 rbm, u32,
    pub len): *mut *mut *mut void ath12k_hal_encode_tlv64_hdr(void tlv, u64 tag, u64,
    pub len): *mut *mut *mut void ath12k_hal_encode_tlv32_hdr(void tlv, u64 tag, u64,
    pub usrid): *mut *mut *mut *mut *mut void ath12k_hal_decode_tlv64_hdr(void tlv, u16 tag, u16 len, u16,
    pub usrid): *mut *mut *mut *mut *mut void ath12k_hal_decode_tlv32_hdr(void tlv, u16 tag, u16 len, u16,
    pub ath12k_hal_get_tlv64_hdr_align(void): u32,
    pub ath12k_hal_get_tlv32_hdr_align(void): u32,
