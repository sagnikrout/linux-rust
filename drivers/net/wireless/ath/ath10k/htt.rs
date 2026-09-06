//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/htt.h
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_dbg_stats_type {
    HTT_DBG_STATS_WAL_PDEV_TXRX = 1 << 0,
    HTT_DBG_STATS_RX_REORDER    = 1 << 1,
    HTT_DBG_STATS_RX_RATE_INFO  = 1 << 2,
    HTT_DBG_STATS_TX_PPDU_LOG   = 1 << 3,
    HTT_DBG_STATS_TX_RATE_INFO  = 1 << 4,
// bits 5-23 currently reserved

    HTT_DBG_NUM_STATS /* keep this last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_h2t_msg_type {
    HTT_H2T_MSG_TYPE_VERSION_REQ        = 0,
    HTT_H2T_MSG_TYPE_TX_FRM             = 1,
    HTT_H2T_MSG_TYPE_RX_RING_CFG        = 2,
    HTT_H2T_MSG_TYPE_STATS_REQ          = 3,
    HTT_H2T_MSG_TYPE_SYNC               = 4,
    HTT_H2T_MSG_TYPE_AGGR_CFG           = 5,
    HTT_H2T_MSG_TYPE_FRAG_DESC_BANK_CFG = 6,

// This command is used for sending management frames in HTT < 3.0.
// HTT >= 3.0 uses TX_FRM for everything.
//
    HTT_H2T_MSG_TYPE_MGMT_TX            = 7,
    HTT_H2T_MSG_TYPE_TX_FETCH_RESP      = 11,

    HTT_H2T_NUM_MSGS /* keep this last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_cmd_hdr {
    pub msg_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ver_req {
    pub htt_cmd_hdr)]: u8 pad[sizeof(u32) - sizeof(struct,
    pub __packed: },
//
// HTT tx MSDU descriptor
//
// The HTT tx MSDU descriptor is created by the host HTT SW for each
// tx MSDU.  The HTT tx MSDU descriptor contains the information that
// the target firmware needs for the FW's tx processing, particularly
// for creating the HW msdu descriptor.
// The same HTT tx descriptor is used for HL and LL systems, though
// a few fields within the tx descriptor are used only by LL or
// only by HL.
// The HTT tx descriptor is defined in two manners: by a struct with
// bitfields, and by a series of [dword offset, bit mask, bit shift]
// definitions.
// The target should use the struct def, for simplicity and clarity,
// but the host shall use the bit-mast + bit-shift defs, to be endian-
// neutral.  Specifically, the host shall use the get/set macros built
// around the mask + shift defs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_data_tx_desc_frag {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct double_word_addr {
    pub paddr: __le32,
    pub len: __le32,
    pub dword_addr: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct triple_word_addr {
    pub paddr_lo: __le32,
    pub paddr_hi: __le16,
    pub len_16: __le16,
    pub tword_addr: } __packed,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_msdu_ext_desc {
    pub tso_flag: [__le32; 3],
    pub ip_identification: __le16,
    pub flags: u8,
    pub reserved: u8,
    pub frags: [htt_data_tx_desc_frag; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_msdu_ext_desc_64 {
    pub tso_flag: [__le32; 5],
    pub ip_identification: __le16,
    pub flags: u8,
    pub reserved: u8,
    pub frags: [htt_data_tx_desc_frag; 6],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_data_tx_desc_flags0 {
    HTT_DATA_TX_DESC_FLAGS0_MAC_HDR_PRESENT = 1 << 0,
    HTT_DATA_TX_DESC_FLAGS0_NO_AGGR         = 1 << 1,
    HTT_DATA_TX_DESC_FLAGS0_NO_ENCRYPT      = 1 << 2,
    HTT_DATA_TX_DESC_FLAGS0_NO_CLASSIFY     = 1 << 3,
    HTT_DATA_TX_DESC_FLAGS0_RSVD0           = 1 << 4
pub const HTT_DATA_TX_DESC_FLAGS0_PKT_TYPE_MASK: c_uint = 0xE0;
pub const HTT_DATA_TX_DESC_FLAGS0_PKT_TYPE_LSB: c_int = 5;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_data_tx_desc_flags1 {
pub const HTT_DATA_TX_DESC_FLAGS1_VDEV_ID_BITS: c_int = 6;
pub const HTT_DATA_TX_DESC_FLAGS1_VDEV_ID_MASK: c_uint = 0x003F;
pub const HTT_DATA_TX_DESC_FLAGS1_VDEV_ID_LSB: c_int = 0;
pub const HTT_DATA_TX_DESC_FLAGS1_EXT_TID_BITS: c_int = 5;
pub const HTT_DATA_TX_DESC_FLAGS1_EXT_TID_MASK: c_uint = 0x07C0;
pub const HTT_DATA_TX_DESC_FLAGS1_EXT_TID_LSB: c_int = 6;
    HTT_DATA_TX_DESC_FLAGS1_POSTPONED        = 1 << 11,
    HTT_DATA_TX_DESC_FLAGS1_MORE_IN_BATCH    = 1 << 12,
    HTT_DATA_TX_DESC_FLAGS1_CKSUM_L3_OFFLOAD = 1 << 13,
    HTT_DATA_TX_DESC_FLAGS1_CKSUM_L4_OFFLOAD = 1 << 14,
    HTT_DATA_TX_DESC_FLAGS1_TX_COMPLETE      = 1 << 15
}

pub const HTT_TX_CREDIT_DELTA_ABS_M: c_uint = 0xffff0000;
pub const HTT_TX_CREDIT_DELTA_ABS_S: c_int = 16;

pub const HTT_TX_CREDIT_SIGN_BIT_M: c_uint = 0x00000100;
pub const HTT_TX_CREDIT_SIGN_BIT_S: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_data_tx_ext_tid {
    HTT_DATA_TX_EXT_TID_NON_QOS_MCAST_BCAST = 16,
    HTT_DATA_TX_EXT_TID_MGMT                = 17,
    HTT_DATA_TX_EXT_TID_INVALID             = 31
}

pub const HTT_INVALID_PEERID: c_uint = 0xFFFF;
//
// htt_data_tx_desc - used for data tx path
//
// Note: vdev_id irrelevant for pkt_type == raw and no_classify == 1.
// ext_tid: for qos-data frames (0-15), see %HTT_DATA_TX_EXT_TID_
// for special kinds of tids
// postponed: only for HL hosts. indicates if this is a resend
// (HL hosts manage queues on the host )
// more_in_batch: only for HL hosts. indicates if more packets are
// pending. this allows target to wait and aggregate
// freq: 0 means home channel of given vdev. intended for offchannel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_data_tx_desc {
    pub /: *mut *mut u8 flags0; / %HTT_DATA_TX_DESC_FLAGS0_,
    pub /: *mut *mut __le16 flags1; / %HTT_DATA_TX_DESC_FLAGS1_,
    pub len: __le16,
    pub id: __le16,
    pub frags_paddr: __le32,
    pub peerid: __le32,
    pub peerid: __le16,
    pub freq: __le16,
    pub offchan_tx: } __packed,
    pub __packed: },
    pub /: *mut *mut u8 prefetch[0]; / start of frame, for FW classification engine,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_data_tx_desc_64 {
    pub /: *mut *mut u8 flags0; / %HTT_DATA_TX_DESC_FLAGS0_,
    pub /: *mut *mut __le16 flags1; / %HTT_DATA_TX_DESC_FLAGS1_,
    pub len: __le16,
    pub id: __le16,
    pub frags_paddr: __le64,
    pub peerid: __le32,
    pub peerid: __le16,
    pub freq: __le16,
    pub offchan_tx: } __packed,
    pub __packed: },
    pub /: *mut *mut u8 prefetch[0]; / start of frame, for FW classification engine,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_ring_flags {
    HTT_RX_RING_FLAGS_MAC80211_HDR = 1 << 0,
    HTT_RX_RING_FLAGS_MSDU_PAYLOAD = 1 << 1,
    HTT_RX_RING_FLAGS_PPDU_START   = 1 << 2,
    HTT_RX_RING_FLAGS_PPDU_END     = 1 << 3,
    HTT_RX_RING_FLAGS_MPDU_START   = 1 << 4,
    HTT_RX_RING_FLAGS_MPDU_END     = 1 << 5,
    HTT_RX_RING_FLAGS_MSDU_START   = 1 << 6,
    HTT_RX_RING_FLAGS_MSDU_END     = 1 << 7,
    HTT_RX_RING_FLAGS_RX_ATTENTION = 1 << 8,
    HTT_RX_RING_FLAGS_FRAG_INFO    = 1 << 9,
    HTT_RX_RING_FLAGS_UNICAST_RX   = 1 << 10,
    HTT_RX_RING_FLAGS_MULTICAST_RX = 1 << 11,
    HTT_RX_RING_FLAGS_CTRL_RX      = 1 << 12,
    HTT_RX_RING_FLAGS_MGMT_RX      = 1 << 13,
    HTT_RX_RING_FLAGS_NULL_RX      = 1 << 14,
    HTT_RX_RING_FLAGS_PHY_DATA_RX  = 1 << 15
}

pub const HTT_RX_RING_SIZE_MIN: c_int = 128;
pub const HTT_RX_RING_SIZE_MAX: c_int = 2048;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_rx_desc_offsets {
// the following offsets are in 4-byte units
    pub mac80211_hdr_offset: __le16,
    pub msdu_payload_offset: __le16,
    pub ppdu_start_offset: __le16,
    pub ppdu_end_offset: __le16,
    pub mpdu_start_offset: __le16,
    pub mpdu_end_offset: __le16,
    pub msdu_start_offset: __le16,
    pub msdu_end_offset: __le16,
    pub rx_attention_offset: __le16,
    pub frag_info_offset: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_setup_ring32 {
    pub fw_idx_shadow_reg_paddr: __le32,
    pub rx_ring_base_paddr: __le32,
    pub /: *mut *mut __le16 rx_ring_len; / in 4-byte words,
    pub /: *mut *mut __le16 rx_ring_bufsize; / rx skb size - in bytes,
    pub /: *mut *mut __le16 flags; / %HTT_RX_RING_FLAGS_,
    pub fw_idx_init_val: __le16,
    pub offsets: htt_rx_ring_rx_desc_offsets,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_setup_ring64 {
    pub fw_idx_shadow_reg_paddr: __le64,
    pub rx_ring_base_paddr: __le64,
    pub /: *mut *mut __le16 rx_ring_len; / in 4-byte words,
    pub /: *mut *mut __le16 rx_ring_bufsize; / rx skb size - in bytes,
    pub /: *mut *mut __le16 flags; / %HTT_RX_RING_FLAGS_,
    pub fw_idx_init_val: __le16,
    pub offsets: htt_rx_ring_rx_desc_offsets,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_setup_hdr {
    pub /: *mut *mut u8 num_rings; / supported values: 1, 2,
    pub rsvd0: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_setup_32 {
    pub hdr: htt_rx_ring_setup_hdr,
    pub rings: [htt_rx_ring_setup_ring32; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_setup_64 {
    pub hdr: htt_rx_ring_setup_hdr,
    pub rings: [htt_rx_ring_setup_ring64; ],
    pub __packed: },
//
// htt_stats_req - request target to send specified statistics
//
// @msg_type: hardcoded %HTT_H2T_MSG_TYPE_STATS_REQ
// @upload_types: see %htt_dbg_stats_type. this is 24bit field actually
// so make sure its little-endian.
// @reset_types: see %htt_dbg_stats_type. this is 24bit field actually
// so make sure its little-endian.
// @cfg_val: stat_type specific configuration
// @stat_type: see %htt_dbg_stats_type
// @cookie_lsb: used for confirmation message from target->host
// @cookie_msb: ditto as %cookie
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_stats_req {
    pub upload_types: [u8; 3],
    pub rsvd0: u8,
    pub reset_types: [u8; 3],
    pub mpdu_bytes: u8,
    pub mpdu_num_msdus: u8,
    pub msdu_bytes: u8,
    pub __packed: },
    pub stat_type: u8,
    pub cookie_lsb: __le32,
    pub cookie_msb: __le32,
    pub __packed: },
pub const HTT_STATS_REQ_CFG_STAT_TYPE_INVALID: c_uint = 0xff;

//
// htt_oob_sync_req - request out-of-band sync
//
// The HTT SYNC tells the target to suspend processing of subsequent
// HTT host-to-target messages until some other target agent locally
// informs the target HTT FW that the current sync counter is equal to
// or greater than (in a modulo sense) the sync counter specified in
// the SYNC message.
//
// This allows other host-target components to synchronize their operation
// with HTT, e.g. to ensure that tx frames don't get transmitted until a
// security key has been downloaded to and activated by the target.
// In the absence of any explicit synchronization counter value
// specification, the target HTT FW will use zero as the default current
// sync value.
//
// The HTT target FW will suspend its host->target message processing as long
// as 0 < (in-band sync counter - out-of-band sync counter) & 0xff < 128.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_oob_sync_req {
    pub sync_count: u8,
    pub rsvd0: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_aggr_conf {
    pub max_num_ampdu_subframes: u8,
// amsdu_subframes is limited by 0x1F mask
    pub max_num_amsdu_subframes: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_aggr_conf_v2 {
    pub max_num_ampdu_subframes: u8,
// amsdu_subframes is limited by 0x1F mask
    pub max_num_amsdu_subframes: u8,
    pub reserved: u8,
    pub __packed: },
pub const HTT_MGMT_FRM_HDR_DOWNLOAD_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_mgmt_tx_desc_qca99x0 {
    pub rate: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_mgmt_tx_desc {
    pub htt_cmd_hdr)]: u8 pad[sizeof(u32) - sizeof(struct,
    pub msdu_paddr: __le32,
    pub desc_id: __le32,
    pub len: __le32,
    pub vdev_id: __le32,
    pub hdr: [u8; HTT_MGMT_FRM_HDR_DOWNLOAD_LEN],
    pub qca99x0: htt_mgmt_tx_desc_qca99x0,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_mgmt_tx_status {
    HTT_MGMT_TX_STATUS_OK    = 0,
    HTT_MGMT_TX_STATUS_RETRY = 1,
    HTT_MGMT_TX_STATUS_DROP  = 2
}

// === target -> host messages ===============================================
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_main_t2h_msg_type {
    HTT_MAIN_T2H_MSG_TYPE_VERSION_CONF             = 0x0,
    HTT_MAIN_T2H_MSG_TYPE_RX_IND                   = 0x1,
    HTT_MAIN_T2H_MSG_TYPE_RX_FLUSH                 = 0x2,
    HTT_MAIN_T2H_MSG_TYPE_PEER_MAP                 = 0x3,
    HTT_MAIN_T2H_MSG_TYPE_PEER_UNMAP               = 0x4,
    HTT_MAIN_T2H_MSG_TYPE_RX_ADDBA                 = 0x5,
    HTT_MAIN_T2H_MSG_TYPE_RX_DELBA                 = 0x6,
    HTT_MAIN_T2H_MSG_TYPE_TX_COMPL_IND             = 0x7,
    HTT_MAIN_T2H_MSG_TYPE_PKTLOG                   = 0x8,
    HTT_MAIN_T2H_MSG_TYPE_STATS_CONF               = 0x9,
    HTT_MAIN_T2H_MSG_TYPE_RX_FRAG_IND              = 0xa,
    HTT_MAIN_T2H_MSG_TYPE_SEC_IND                  = 0xb,
    HTT_MAIN_T2H_MSG_TYPE_TX_INSPECT_IND           = 0xd,
    HTT_MAIN_T2H_MSG_TYPE_MGMT_TX_COMPL_IND        = 0xe,
    HTT_MAIN_T2H_MSG_TYPE_TX_CREDIT_UPDATE_IND     = 0xf,
    HTT_MAIN_T2H_MSG_TYPE_RX_PN_IND                = 0x10,
    HTT_MAIN_T2H_MSG_TYPE_RX_OFFLOAD_DELIVER_IND   = 0x11,
    HTT_MAIN_T2H_MSG_TYPE_TEST,
// keep this last
    HTT_MAIN_T2H_NUM_MSGS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_10x_t2h_msg_type {
    HTT_10X_T2H_MSG_TYPE_VERSION_CONF              = 0x0,
    HTT_10X_T2H_MSG_TYPE_RX_IND                    = 0x1,
    HTT_10X_T2H_MSG_TYPE_RX_FLUSH                  = 0x2,
    HTT_10X_T2H_MSG_TYPE_PEER_MAP                  = 0x3,
    HTT_10X_T2H_MSG_TYPE_PEER_UNMAP                = 0x4,
    HTT_10X_T2H_MSG_TYPE_RX_ADDBA                  = 0x5,
    HTT_10X_T2H_MSG_TYPE_RX_DELBA                  = 0x6,
    HTT_10X_T2H_MSG_TYPE_TX_COMPL_IND              = 0x7,
    HTT_10X_T2H_MSG_TYPE_PKTLOG                    = 0x8,
    HTT_10X_T2H_MSG_TYPE_STATS_CONF                = 0x9,
    HTT_10X_T2H_MSG_TYPE_RX_FRAG_IND               = 0xa,
    HTT_10X_T2H_MSG_TYPE_SEC_IND                   = 0xb,
    HTT_10X_T2H_MSG_TYPE_RC_UPDATE_IND             = 0xc,
    HTT_10X_T2H_MSG_TYPE_TX_INSPECT_IND            = 0xd,
    HTT_10X_T2H_MSG_TYPE_TEST                      = 0xe,
    HTT_10X_T2H_MSG_TYPE_CHAN_CHANGE               = 0xf,
    HTT_10X_T2H_MSG_TYPE_AGGR_CONF                 = 0x11,
    HTT_10X_T2H_MSG_TYPE_STATS_NOUPLOAD            = 0x12,
    HTT_10X_T2H_MSG_TYPE_MGMT_TX_COMPL_IND         = 0x13,
// keep this last
    HTT_10X_T2H_NUM_MSGS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_tlv_t2h_msg_type {
    HTT_TLV_T2H_MSG_TYPE_VERSION_CONF              = 0x0,
    HTT_TLV_T2H_MSG_TYPE_RX_IND                    = 0x1,
    HTT_TLV_T2H_MSG_TYPE_RX_FLUSH                  = 0x2,
    HTT_TLV_T2H_MSG_TYPE_PEER_MAP                  = 0x3,
    HTT_TLV_T2H_MSG_TYPE_PEER_UNMAP                = 0x4,
    HTT_TLV_T2H_MSG_TYPE_RX_ADDBA                  = 0x5,
    HTT_TLV_T2H_MSG_TYPE_RX_DELBA                  = 0x6,
    HTT_TLV_T2H_MSG_TYPE_TX_COMPL_IND              = 0x7,
    HTT_TLV_T2H_MSG_TYPE_PKTLOG                    = 0x8,
    HTT_TLV_T2H_MSG_TYPE_STATS_CONF                = 0x9,
    HTT_TLV_T2H_MSG_TYPE_RX_FRAG_IND               = 0xa,
    HTT_TLV_T2H_MSG_TYPE_SEC_IND                   = 0xb,
    HTT_TLV_T2H_MSG_TYPE_RC_UPDATE_IND             = 0xc, /* deprecated */
    HTT_TLV_T2H_MSG_TYPE_TX_INSPECT_IND            = 0xd,
    HTT_TLV_T2H_MSG_TYPE_MGMT_TX_COMPL_IND         = 0xe,
    HTT_TLV_T2H_MSG_TYPE_TX_CREDIT_UPDATE_IND      = 0xf,
    HTT_TLV_T2H_MSG_TYPE_RX_PN_IND                 = 0x10,
    HTT_TLV_T2H_MSG_TYPE_RX_OFFLOAD_DELIVER_IND    = 0x11,
    HTT_TLV_T2H_MSG_TYPE_RX_IN_ORD_PADDR_IND       = 0x12,
// 0x13 reservd
    HTT_TLV_T2H_MSG_TYPE_WDI_IPA_OP_RESPONSE       = 0x14,
    HTT_TLV_T2H_MSG_TYPE_CHAN_CHANGE               = 0x15,
    HTT_TLV_T2H_MSG_TYPE_RX_OFLD_PKT_ERR           = 0x16,
    HTT_TLV_T2H_MSG_TYPE_TEST,
// keep this last
    HTT_TLV_T2H_NUM_MSGS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_10_4_t2h_msg_type {
    HTT_10_4_T2H_MSG_TYPE_VERSION_CONF           = 0x0,
    HTT_10_4_T2H_MSG_TYPE_RX_IND                 = 0x1,
    HTT_10_4_T2H_MSG_TYPE_RX_FLUSH               = 0x2,
    HTT_10_4_T2H_MSG_TYPE_PEER_MAP               = 0x3,
    HTT_10_4_T2H_MSG_TYPE_PEER_UNMAP             = 0x4,
    HTT_10_4_T2H_MSG_TYPE_RX_ADDBA               = 0x5,
    HTT_10_4_T2H_MSG_TYPE_RX_DELBA               = 0x6,
    HTT_10_4_T2H_MSG_TYPE_TX_COMPL_IND           = 0x7,
    HTT_10_4_T2H_MSG_TYPE_PKTLOG                 = 0x8,
    HTT_10_4_T2H_MSG_TYPE_STATS_CONF             = 0x9,
    HTT_10_4_T2H_MSG_TYPE_RX_FRAG_IND            = 0xa,
    HTT_10_4_T2H_MSG_TYPE_SEC_IND                = 0xb,
    HTT_10_4_T2H_MSG_TYPE_RC_UPDATE_IND          = 0xc,
    HTT_10_4_T2H_MSG_TYPE_TX_INSPECT_IND         = 0xd,
    HTT_10_4_T2H_MSG_TYPE_MGMT_TX_COMPL_IND      = 0xe,
    HTT_10_4_T2H_MSG_TYPE_CHAN_CHANGE            = 0xf,
    HTT_10_4_T2H_MSG_TYPE_TX_CREDIT_UPDATE_IND   = 0x10,
    HTT_10_4_T2H_MSG_TYPE_RX_PN_IND              = 0x11,
    HTT_10_4_T2H_MSG_TYPE_RX_OFFLOAD_DELIVER_IND = 0x12,
    HTT_10_4_T2H_MSG_TYPE_TEST                   = 0x13,
    HTT_10_4_T2H_MSG_TYPE_EN_STATS               = 0x14,
    HTT_10_4_T2H_MSG_TYPE_AGGR_CONF              = 0x15,
    HTT_10_4_T2H_MSG_TYPE_TX_FETCH_IND           = 0x16,
    HTT_10_4_T2H_MSG_TYPE_TX_FETCH_CONFIRM       = 0x17,
    HTT_10_4_T2H_MSG_TYPE_STATS_NOUPLOAD         = 0x18,
// 0x19 to 0x2f are reserved
    HTT_10_4_T2H_MSG_TYPE_TX_MODE_SWITCH_IND     = 0x30,
    HTT_10_4_T2H_MSG_TYPE_PEER_STATS	     = 0x31,
// keep this last
    HTT_10_4_T2H_NUM_MSGS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_t2h_msg_type {
    HTT_T2H_MSG_TYPE_VERSION_CONF,
    HTT_T2H_MSG_TYPE_RX_IND,
    HTT_T2H_MSG_TYPE_RX_FLUSH,
    HTT_T2H_MSG_TYPE_PEER_MAP,
    HTT_T2H_MSG_TYPE_PEER_UNMAP,
    HTT_T2H_MSG_TYPE_RX_ADDBA,
    HTT_T2H_MSG_TYPE_RX_DELBA,
    HTT_T2H_MSG_TYPE_TX_COMPL_IND,
    HTT_T2H_MSG_TYPE_PKTLOG,
    HTT_T2H_MSG_TYPE_STATS_CONF,
    HTT_T2H_MSG_TYPE_RX_FRAG_IND,
    HTT_T2H_MSG_TYPE_SEC_IND,
    HTT_T2H_MSG_TYPE_RC_UPDATE_IND,
    HTT_T2H_MSG_TYPE_TX_INSPECT_IND,
    HTT_T2H_MSG_TYPE_MGMT_TX_COMPLETION,
    HTT_T2H_MSG_TYPE_TX_CREDIT_UPDATE_IND,
    HTT_T2H_MSG_TYPE_RX_PN_IND,
    HTT_T2H_MSG_TYPE_RX_OFFLOAD_DELIVER_IND,
    HTT_T2H_MSG_TYPE_RX_IN_ORD_PADDR_IND,
    HTT_T2H_MSG_TYPE_WDI_IPA_OP_RESPONSE,
    HTT_T2H_MSG_TYPE_CHAN_CHANGE,
    HTT_T2H_MSG_TYPE_RX_OFLD_PKT_ERR,
    HTT_T2H_MSG_TYPE_AGGR_CONF,
    HTT_T2H_MSG_TYPE_STATS_NOUPLOAD,
    HTT_T2H_MSG_TYPE_TEST,
    HTT_T2H_MSG_TYPE_EN_STATS,
    HTT_T2H_MSG_TYPE_TX_FETCH_IND,
    HTT_T2H_MSG_TYPE_TX_FETCH_CONFIRM,
    HTT_T2H_MSG_TYPE_TX_MODE_SWITCH_IND,
    HTT_T2H_MSG_TYPE_PEER_STATS,
// keep this last
    HTT_T2H_NUM_MSGS
}

//
// htt_resp_hdr - header for target-to-host messages
//
// msg_type: see htt_t2h_msg_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_resp_hdr {
    pub msg_type: u8,
    pub __packed: },
pub const HTT_RESP_HDR_MSG_TYPE_OFFSET: c_int = 0;
pub const HTT_RESP_HDR_MSG_TYPE_MASK: c_uint = 0xff;
pub const HTT_RESP_HDR_MSG_TYPE_LSB: c_int = 0;
// htt_ver_resp - response sent for htt_ver_req
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ver_resp {
    pub minor: u8,
    pub major: u8,
    pub rsvd0: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_mgmt_tx_completion {
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub flags: u8,
    pub desc_id: __le32,
    pub status: __le32,
    pub ppdu_id: __le32,
    pub info: __le32,
    pub __packed: },

pub const HTT_RX_INDICATION_INFO1_FLUSH_START_SEQNO_MASK: c_uint = 0x0000003F;
pub const HTT_RX_INDICATION_INFO1_FLUSH_START_SEQNO_LSB: c_int = 0;
pub const HTT_RX_INDICATION_INFO1_FLUSH_END_SEQNO_MASK: c_uint = 0x00000FC0;
pub const HTT_RX_INDICATION_INFO1_FLUSH_END_SEQNO_LSB: c_int = 6;
pub const HTT_RX_INDICATION_INFO1_RELEASE_START_SEQNO_MASK: c_uint = 0x0003F000;
pub const HTT_RX_INDICATION_INFO1_RELEASE_START_SEQNO_LSB: c_int = 12;
pub const HTT_RX_INDICATION_INFO1_RELEASE_END_SEQNO_MASK: c_uint = 0x00FC0000;
pub const HTT_RX_INDICATION_INFO1_RELEASE_END_SEQNO_LSB: c_int = 18;
pub const HTT_RX_INDICATION_INFO1_NUM_MPDU_RANGES_MASK: c_uint = 0xFF000000;
pub const HTT_RX_INDICATION_INFO1_NUM_MPDU_RANGES_LSB: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_indication_hdr {
    pub /: *mut *mut u8 info0; / %HTT_RX_INDICATION_INFO0_,
    pub peer_id: __le16,
    pub /: *mut *mut __le32 info1; / %HTT_RX_INDICATION_INFO1_,
    pub __packed: },

pub const HTT_RX_INDICATION_INFO1_VHT_SIG_A1_MASK: c_uint = 0x00FFFFFF;
pub const HTT_RX_INDICATION_INFO1_VHT_SIG_A1_LSB: c_int = 0;
pub const HTT_RX_INDICATION_INFO1_PREAMBLE_TYPE_MASK: c_uint = 0xFF000000;
pub const HTT_RX_INDICATION_INFO1_PREAMBLE_TYPE_LSB: c_int = 24;
pub const HTT_RX_INDICATION_INFO2_VHT_SIG_A1_MASK: c_uint = 0x00FFFFFF;
pub const HTT_RX_INDICATION_INFO2_VHT_SIG_A1_LSB: c_int = 0;
pub const HTT_RX_INDICATION_INFO2_SERVICE_MASK: c_uint = 0xFF000000;
pub const HTT_RX_INDICATION_INFO2_SERVICE_LSB: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_legacy_rate {
    HTT_RX_OFDM_48 = 0,
    HTT_RX_OFDM_24 = 1,
    HTT_RX_OFDM_12,
    HTT_RX_OFDM_6,
    HTT_RX_OFDM_54,
    HTT_RX_OFDM_36,
    HTT_RX_OFDM_18,
    HTT_RX_OFDM_9,

// long preamble
    HTT_RX_CCK_11_LP = 0,
    HTT_RX_CCK_5_5_LP = 1,
    HTT_RX_CCK_2_LP,
    HTT_RX_CCK_1_LP,
// short preamble
    HTT_RX_CCK_11_SP,
    HTT_RX_CCK_5_5_SP,
    HTT_RX_CCK_2_SP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_legacy_rate_type {
    HTT_RX_LEGACY_RATE_OFDM = 0,
    HTT_RX_LEGACY_RATE_CCK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_preamble_type {
    HTT_RX_LEGACY        = 0x4,
    HTT_RX_HT            = 0x8,
    HTT_RX_HT_WITH_TXBF  = 0x9,
    HTT_RX_VHT           = 0xC,
    HTT_RX_VHT_WITH_TXBF = 0xD,
}

//
// Fields: phy_err_valid, phy_err_code, tsf,
// usec_timestamp, sub_usec_timestamp
// ..are valid only if end_valid == 1.
//
// Fields: rssi_chains, legacy_rate_type,
// legacy_rate_cck, preamble_type, service,
// vht_sig_
// ..are valid only if start_valid == 1;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_indication_ppdu {
    pub combined_rssi: u8,
    pub sub_usec_timestamp: u8,
    pub phy_err_code: u8,
    pub /: *mut *mut u8 info0; / HTT_RX_INDICATION_INFO0_,
    pub pri20_db: u8,
    pub ext20_db: u8,
    pub ext40_db: u8,
    pub ext80_db: u8,
    pub rssi_chains: [} __packed; 4],
    pub tsf: __le32,
    pub usec_timestamp: __le32,
    pub /: *mut *mut __le32 info1; / HTT_RX_INDICATION_INFO1_,
    pub /: *mut *mut __le32 info2; / HTT_RX_INDICATION_INFO2_,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_mpdu_status {
    HTT_RX_IND_MPDU_STATUS_UNKNOWN = 0x0,
    HTT_RX_IND_MPDU_STATUS_OK,
    HTT_RX_IND_MPDU_STATUS_ERR_FCS,
    HTT_RX_IND_MPDU_STATUS_ERR_DUP,
    HTT_RX_IND_MPDU_STATUS_ERR_REPLAY,
    HTT_RX_IND_MPDU_STATUS_ERR_INV_PEER,
// only accept EAPOL frames
    HTT_RX_IND_MPDU_STATUS_UNAUTH_PEER,
    HTT_RX_IND_MPDU_STATUS_OUT_OF_SYNC,
// Non-data in promiscuous mode
    HTT_RX_IND_MPDU_STATUS_MGMT_CTRL,
    HTT_RX_IND_MPDU_STATUS_TKIP_MIC_ERR,
    HTT_RX_IND_MPDU_STATUS_DECRYPT_ERR,
    HTT_RX_IND_MPDU_STATUS_MPDU_LENGTH_ERR,
    HTT_RX_IND_MPDU_STATUS_ENCRYPT_REQUIRED_ERR,
    HTT_RX_IND_MPDU_STATUS_PRIVACY_ERR,

//
// MISC: discard for unspecified reasons.
// Leave this enum value last.
//
    HTT_RX_IND_MPDU_STATUS_ERR_MISC = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_indication_mpdu_range {
    pub mpdu_count: u8,
    pub /: *mut *mut u8 mpdu_range_status; / %htt_rx_mpdu_status,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_indication_prefix {
    pub fw_rx_desc_bytes: __le16,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_indication {
    pub hdr: htt_rx_indication_hdr,
    pub ppdu: htt_rx_indication_ppdu,
    pub prefix: htt_rx_indication_prefix,
//
// the following fields are both dynamically sized, so
// take care addressing them
//
// the size of this is %fw_rx_desc_bytes
    pub fw_desc: fw_rx_desc_base,
//
// %mpdu_ranges starts after &%prefix + roundup(%fw_rx_desc_bytes, 4)
// and has %num_mpdu_ranges elements.
//
    pub mpdu_ranges: [htt_rx_indication_mpdu_range; ],
    pub __packed: },
// High latency version of the RX indication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_indication_hl {
    pub hdr: htt_rx_indication_hdr,
    pub ppdu: htt_rx_indication_ppdu,
    pub prefix: htt_rx_indication_prefix,
    pub fw_desc: fw_rx_desc_hl,
    pub mpdu_ranges: [htt_rx_indication_mpdu_range; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_hl_rx_desc {
    pub info: __le32,
    pub pn_31_0: __le32,
    pub pn_47_32: __le16,
    pub pn_63_48: __le16,
    pub pn16: },
    pub pn_63_32: __le32,
    pub u0: },
    pub pn_95_64: __le32,
    pub pn_127_96: __le32,
    pub __packed: },
    pub rx_ind: *mut *mut void ptr =,
    pub 4): + roundup(__le16_to_cpu(rx_ind->prefix.fw_rx_desc_bytes),,
    pub ptr: return,
    pub rx_ind: *mut *mut void ptr =,
    pub sizeof(rx_ind->fw_desc): +,
    pub ptr: return,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_flush_mpdu_status {
    HTT_RX_FLUSH_MPDU_DISCARD = 0,
    HTT_RX_FLUSH_MPDU_REORDER = 1,
}

//
// htt_rx_flush - discard or reorder given range of mpdus
//
// Note: host must check if all sequence numbers between
// [seq_num_start, seq_num_end-1] are valid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_flush {
    pub peer_id: __le16,
    pub tid: u8,
    pub rsvd0: u8,
    pub /: *mut *mut u8 mpdu_status; / %htt_rx_flush_mpdu_status,
    pub /: *mut *mut u8 seq_num_start; / it is 6 LSBs of 802.11 seq no,
    pub /: *mut *mut u8 seq_num_end; / it is 6 LSBs of 802.11 seq no,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_peer_map {
    pub vdev_id: u8,
    pub peer_id: __le16,
    pub addr: [u8; 6],
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_peer_unmap {
    pub rsvd0: u8,
    pub peer_id: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_txrx_sec_cast_type {
    HTT_TXRX_SEC_MCAST = 0,
    HTT_TXRX_SEC_UCAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_pn_check_type {
    HTT_RX_NON_PN_CHECK = 0,
    HTT_RX_PN_CHECK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_tkip_demic_type {
    HTT_RX_NON_TKIP_MIC = 0,
    HTT_RX_TKIP_MIC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_security_types {
    HTT_SECURITY_NONE,
    HTT_SECURITY_WEP128,
    HTT_SECURITY_WEP104,
    HTT_SECURITY_WEP40,
    HTT_SECURITY_TKIP,
    HTT_SECURITY_TKIP_NOMIC,
    HTT_SECURITY_AES_CCMP,
    HTT_SECURITY_WAPI,

    HTT_NUM_SECURITY_TYPES /* keep this last! */
}

pub const ATH10K_HTT_TXRX_PEER_SECURITY_MAX: c_int = 2;
pub const ATH10K_TXRX_NUM_EXT_TIDS: c_int = 19;
pub const ATH10K_TXRX_NON_QOS_TID: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_security_flags {
pub const HTT_SECURITY_TYPE_MASK: c_uint = 0x7F;
pub const HTT_SECURITY_TYPE_LSB: c_int = 0;
    HTT_SECURITY_IS_UNICAST = 1 << 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_security_indication {
// dont use bitfields; undefined behaviour
    pub /: *mut *mut u8 flags; / %htt_security_flags,
    pub __packed: },
    pub __packed: },
    pub peer_id: __le16,
    pub michael_key: [u8; 8],
    pub wapi_rsc: [u8; 16],
    pub __packed: },
pub const HTT_RX_BA_INFO0_TID_MASK: c_uint = 0x000F;
pub const HTT_RX_BA_INFO0_TID_LSB: c_int = 0;
pub const HTT_RX_BA_INFO0_PEER_ID_MASK: c_uint = 0xFFF0;
pub const HTT_RX_BA_INFO0_PEER_ID_LSB: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_addba {
    pub window_size: u8,
    pub /: *mut *mut __le16 info0; / %HTT_RX_BA_INFO0_,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_delba {
    pub rsvd0: u8,
    pub /: *mut *mut __le16 info0; / %HTT_RX_BA_INFO0_,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_data_tx_status {
    HTT_DATA_TX_STATUS_OK            = 0,
    HTT_DATA_TX_STATUS_DISCARD       = 1,
    HTT_DATA_TX_STATUS_NO_ACK        = 2,
    HTT_DATA_TX_STATUS_POSTPONE      = 3 /* HL only */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_data_tx_flags {
pub const HTT_DATA_TX_STATUS_MASK: c_uint = 0x07;
pub const HTT_DATA_TX_STATUS_LSB: c_int = 0;
pub const HTT_DATA_TX_TID_MASK: c_uint = 0x78;
pub const HTT_DATA_TX_TID_LSB: c_int = 3;
    HTT_DATA_TX_TID_INVALID = 1 << 7
}

pub const HTT_TX_COMPL_INV_MSDU_ID: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_append_retries {
    pub msdu_id: __le16,
    pub tx_retries: u8,
    pub flag: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_data_tx_completion_ext {
    pub a_retries: htt_append_retries,
    pub t_stamp: __le32,
    pub msdus_rssi: [__le16; ],
    pub __packed: },
//
// @brief target -> host TX completion indication message definition
//
// @details
// The following diagram shows the format of the TX completion indication sent
// from the target to the host
//
// |31 28|27|26|25|24|23        16| 15 |14 11|10   8|7          0|
// |-------------------------------------------------------------|
// header:  |rsvd |A2|TP|A1|A0|     num    | t_i| tid |status|  msg_type  |
// |-------------------------------------------------------------|
// payload: |            MSDU1 ID          |         MSDU0 ID             |
// |-------------------------------------------------------------|
// :            MSDU3 ID          :         MSDU2 ID             :
// |-------------------------------------------------------------|
// |          struct htt_tx_compl_ind_append_retries             |
// |- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -|
// |          struct htt_tx_compl_ind_append_tx_tstamp           |
// |- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -|
// |           MSDU1 ACK RSSI     |        MSDU0 ACK RSSI        |
// |-------------------------------------------------------------|
// :           MSDU3 ACK RSSI     :        MSDU2 ACK RSSI        :
// |- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -|
// -msg_type
// Bits 7:0
// Purpose: identifies this as HTT TX completion indication
// -status
// Bits 10:8
// Purpose: the TX completion status of payload fragmentations descriptors
// Value: could be HTT_TX_COMPL_IND_STAT_OK or HTT_TX_COMPL_IND_STAT_DISCARD
// -tid
// Bits 14:11
// Purpose: the tid associated with those fragmentation descriptors. It is
// valid or not, depending on the tid_invalid bit.
// Value: 0 to 15
// -tid_invalid
// Bits 15:15
// Purpose: this bit indicates whether the tid field is valid or not
// Value: 0 indicates valid, 1 indicates invalid
// -num
// Bits 23:16
// Purpose: the number of payload in this indication
// Value: 1 to 255
// -A0 = append
// Bits 24:24
// Purpose: append the struct htt_tx_compl_ind_append_retries which contains
// the number of tx retries for one MSDU at the end of this message
// Value: 0 indicates no appending, 1 indicates appending
// -A1 = append1
// Bits 25:25
// Purpose: Append the struct htt_tx_compl_ind_append_tx_tstamp which
// contains the timestamp info for each TX msdu id in payload.
// Value: 0 indicates no appending, 1 indicates appending
// -TP = MSDU tx power presence
// Bits 26:26
// Purpose: Indicate whether the TX_COMPL_IND includes a tx power report
// for each MSDU referenced by the TX_COMPL_IND message.
// The order of the per-MSDU tx power reports matches the order
// of the MSDU IDs.
// Value: 0 indicates not appending, 1 indicates appending
// -A2 = append2
// Bits 27:27
// Purpose: Indicate whether data ACK RSSI is appended for each MSDU in
// TX_COMP_IND message.  The order of the per-MSDU ACK RSSI report
// matches the order of the MSDU IDs.
// The ACK RSSI values are valid when status is COMPLETE_OK (and
// this append2 bit is set).
// Value: 0 indicates not appending, 1 indicates appending
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_data_tx_completion {
    pub flags: u8,
    pub __packed: },
    pub __packed: },
    pub num_msdus: u8,
    pub /: *mut *mut u8 flags2; / HTT_TX_CMPL_FLAG_DATA_RSSI,
    pub /: *mut *mut __le16 msdus[]; / variable length based on %num_msdus,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_data_tx_ppdu_dur {
    pub /: *mut *mut __le32 info0; / HTT_TX_PPDU_DUR_INFO0_,
    pub /: *mut *mut __le32 tx_duration; / in usecs,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_data_tx_compl_ppdu_dur {
    pub /: *mut *mut __le32 info0; / HTT_TX_COMPL_PPDU_DUR_INFO0_,
    pub ppdu_dur: [htt_data_tx_ppdu_dur; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_compl_ind_base {
    pub hdr: u32,
    pub more*/]: *mut *mut u16 payload[1/or,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rc_tx_done_params {
    pub rate_code: u32,
    pub rate_code_flags: u32,
    pub flags: u32,
    pub /: *mut *mut u32 num_enqued; / 1 for non-AMPDU,
    pub num_retries: u32,
    pub /: *mut *mut u32 num_failed; / for AMPDU,
    pub ack_rssi: u32,
    pub time_stamp: u32,
    pub is_probe: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rc_update {
    pub vdev_id: u8,
    pub peer_id: __le16,
    pub addr: [u8; 6],
    pub num_elems: u8,
    pub rsvd0: u8,
    pub /: *mut *mut htt_rc_tx_done_params params[]; / variable length %num_elems,
    pub __packed: },
// see htt_rx_indication for similar fields and descriptions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_fragment_indication {
    pub /: *mut *mut u8 info0; / %HTT_RX_FRAG_IND_INFO0_,
    pub __packed: },
    pub __packed: },
    pub peer_id: __le16,
    pub /: *mut *mut __le32 info1; / %HTT_RX_FRAG_IND_INFO1_,
    pub fw_rx_desc_bytes: __le16,
    pub rsvd0: __le16,
    pub fw_msdu_rx_desc: [u8; ],
    pub __packed: },

pub const HTT_RX_FRAG_IND_INFO0_HEADER_LEN: c_int = 16;
pub const HTT_RX_FRAG_IND_INFO0_EXT_TID_MASK: c_uint = 0x1F;
pub const HTT_RX_FRAG_IND_INFO0_EXT_TID_LSB: c_int = 0;
pub const HTT_RX_FRAG_IND_INFO0_FLUSH_VALID_MASK: c_uint = 0x20;
pub const HTT_RX_FRAG_IND_INFO0_FLUSH_VALID_LSB: c_int = 5;
pub const HTT_RX_FRAG_IND_INFO1_FLUSH_SEQ_NUM_START_MASK: c_uint = 0x0000003F;
pub const HTT_RX_FRAG_IND_INFO1_FLUSH_SEQ_NUM_START_LSB: c_int = 0;
pub const HTT_RX_FRAG_IND_INFO1_FLUSH_SEQ_NUM_END_MASK: c_uint = 0x00000FC0;
pub const HTT_RX_FRAG_IND_INFO1_FLUSH_SEQ_NUM_END_LSB: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_pn_ind {
    pub peer_id: __le16,
    pub tid: u8,
    pub seqno_start: u8,
    pub seqno_end: u8,
    pub pn_ie_count: u8,
    pub reserved: u8,
    pub pn_ies: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_offload_msdu {
    pub msdu_len: __le16,
    pub peer_id: __le16,
    pub vdev_id: u8,
    pub tid: u8,
    pub fw_desc: u8,
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_offload_ind {
    pub reserved: u8,
    pub msdu_count: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_in_ord_msdu_desc {
    pub msdu_paddr: __le32,
    pub msdu_len: __le16,
    pub fw_desc: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_in_ord_msdu_desc_ext {
    pub msdu_paddr: __le64,
    pub msdu_len: __le16,
    pub fw_desc: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_in_ord_ind {
    pub info: u8,
    pub peer_id: __le16,
    pub vdev_id: u8,
    pub reserved: u8,
    pub msdu_count: __le16,
    pub __packed: },
    pub __packed: },
pub const HTT_RX_IN_ORD_IND_INFO_TID_MASK: c_uint = 0x0000001f;
pub const HTT_RX_IN_ORD_IND_INFO_TID_LSB: c_int = 0;
pub const HTT_RX_IN_ORD_IND_INFO_OFFLOAD_MASK: c_uint = 0x00000020;
pub const HTT_RX_IN_ORD_IND_INFO_OFFLOAD_LSB: c_int = 5;
pub const HTT_RX_IN_ORD_IND_INFO_FRAG_MASK: c_uint = 0x00000040;
pub const HTT_RX_IN_ORD_IND_INFO_FRAG_LSB: c_int = 6;
//
// target -> host test message definition
//
// The following field definitions describe the format of the test
// message sent from the target to the host.
// The message consists of a 4-octet header, followed by a variable
// number of 32-bit integer values, followed by a variable number
// of 8-bit character values.
//
// |31                         16|15           8|7            0|
// |-----------------------------------------------------------|
// |          num chars          |   num ints   |   msg type   |
// |-----------------------------------------------------------|
// |                           int 0                           |
// |-----------------------------------------------------------|
// |                           int 1                           |
// |-----------------------------------------------------------|
// |                            ...                            |
// |-----------------------------------------------------------|
// |    char 3    |    char 2    |    char 1    |    char 0    |
// |-----------------------------------------------------------|
// |              |              |      ...     |    char 4    |
// |-----------------------------------------------------------|
// - MSG_TYPE
// Bits 7:0
// Purpose: identifies this as a test message
// Value: HTT_MSG_TYPE_TEST
// - NUM_INTS
// Bits 15:8
// Purpose: indicate how many 32-bit integers follow the message header
// - NUM_CHARS
// Bits 31:16
// Purpose: indicate how many 8-bit characters follow the series of integers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_test {
    pub num_ints: u8,
    pub num_chars: __le16,
// payload consists of 2 lists:
// a) num_ints * sizeof(__le32)
// b) num_chars * sizeof(u8) aligned to 4bytes
//
    pub payload: [u8; ],
    pub __packed: },
    pub )rx_test->payload: *mut return (__le32,
    pub sizeof(__le32)): *mut *mut return rx_test->payload + (rx_test->num_ints,
//
// target -> host packet log message
//
// The following field definitions describe the format of the packet log
// message sent from the target to the host.
// The message consists of a 4-octet header,followed by a variable number
// of 32-bit character values.
//
// |31          24|23          16|15           8|7            0|
// |-----------------------------------------------------------|
// |              |              |              |   msg type   |
// |-----------------------------------------------------------|
// |                        payload                            |
// |-----------------------------------------------------------|
// - MSG_TYPE
// Bits 7:0
// Purpose: identifies this as a test message
// Value: HTT_MSG_TYPE_PACKETLOG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pktlog_msg {
    pub pad: [u8; 3],
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_dbg_stats_rx_reorder_stats {
// Non QoS MPDUs received
    pub deliver_non_qos: __le32,
// MPDUs received in-order
    pub deliver_in_order: __le32,
// Flush due to reorder timer expired
    pub deliver_flush_timeout: __le32,
// Flush due to move out of window
    pub deliver_flush_oow: __le32,
// Flush due to DELBA
    pub deliver_flush_delba: __le32,
// MPDUs dropped due to FCS error
    pub fcs_error: __le32,
// MPDUs dropped due to monitor mode non-data packet
    pub mgmt_ctrl: __le32,
// MPDUs dropped due to invalid peer
    pub invalid_peer: __le32,
// MPDUs dropped due to duplication (non aggregation)
    pub dup_non_aggr: __le32,
// MPDUs dropped due to processed before
    pub dup_past: __le32,
// MPDUs dropped due to duplicate in reorder queue
    pub dup_in_reorder: __le32,
// Reorder timeout happened
    pub reorder_timeout: __le32,
// invalid bar ssn
    pub invalid_bar_ssn: __le32,
// reorder reset due to bar ssn
    pub ssn_reset: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_dbg_stats_wal_tx_stats {
// Num HTT cookies queued to dispatch list
    pub comp_queued: __le32,
// Num HTT cookies dispatched
    pub comp_delivered: __le32,
// Num MSDU queued to WAL
    pub msdu_enqued: __le32,
// Num MPDU queue to WAL
    pub mpdu_enqued: __le32,
// Num MSDUs dropped by WMM limit
    pub wmm_drop: __le32,
// Num Local frames queued
    pub local_enqued: __le32,
// Num Local frames done
    pub local_freed: __le32,
// Num queued to HW
    pub hw_queued: __le32,
// Num PPDU reaped from HW
    pub hw_reaped: __le32,
// Num underruns
    pub underrun: __le32,
// Num PPDUs cleaned up in TX abort
    pub tx_abort: __le32,
// Num MPDUs requeued by SW
    pub mpdus_requeued: __le32,
// excessive retries
    pub tx_ko: __le32,
// data hw rate code
    pub data_rc: __le32,
// Scheduler self triggers
    pub self_triggers: __le32,
// frames dropped due to excessive sw retries
    pub sw_retry_failure: __le32,
// illegal rate phy errors
    pub illgl_rate_phy_err: __le32,
// wal pdev continuous xretry
    pub pdev_cont_xretry: __le32,
// wal pdev continuous xretry
    pub pdev_tx_timeout: __le32,
// wal pdev resets
    pub pdev_resets: __le32,
    pub phy_underrun: __le32,
// MPDU is more than txop limit
    pub txop_ovf: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_dbg_stats_wal_rx_stats {
// Cnts any change in ring routing mid-ppdu
    pub mid_ppdu_route_change: __le32,
// Total number of statuses processed
    pub status_rcvd: __le32,
// Extra frags on rings 0-3
    pub r0_frags: __le32,
    pub r1_frags: __le32,
    pub r2_frags: __le32,
    pub r3_frags: __le32,
// MSDUs / MPDUs delivered to HTT
    pub htt_msdus: __le32,
    pub htt_mpdus: __le32,
// MSDUs / MPDUs delivered to local stack
    pub loc_msdus: __le32,
    pub loc_mpdus: __le32,
// AMSDUs that have more MSDUs than the status ring size
    pub oversize_amsdu: __le32,
// Number of PHY errors
    pub phy_errs: __le32,
// Number of PHY errors drops
    pub phy_err_drop: __le32,
// Number of mpdu errors - FCS, MIC, ENC etc.
    pub mpdu_errs: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_dbg_stats_wal_peer_stats {
    pub /: *mut *mut __le32 dummy; / REMOVE THIS ONCE REAL PEER STAT COUNTERS ARE ADDED,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_dbg_stats_wal_pdev_txrx {
    pub tx_stats: htt_dbg_stats_wal_tx_stats,
    pub rx_stats: htt_dbg_stats_wal_rx_stats,
    pub peer_stats: htt_dbg_stats_wal_peer_stats,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_dbg_stats_rx_rate_info {
    pub mcs: [__le32; 10],
    pub sgi: [__le32; 10],
    pub nss: [__le32; 4],
    pub stbc: [__le32; 10],
    pub bw: [__le32; 3],
    pub pream: [__le32; 6],
    pub ldpc: __le32,
    pub txbf: __le32,
}

//
// htt_dbg_stats_status -
// present -     The requested stats have been delivered in full.
// This indicates that either the stats information was contained
// in its entirety within this message, or else this message
// completes the delivery of the requested stats info that was
// partially delivered through earlier STATS_CONF messages.
// partial -     The requested stats have been delivered in part.
// One or more subsequent STATS_CONF messages with the same
// cookie value will be sent to deliver the remainder of the
// information.
// error -       The requested stats could not be delivered, for example due
// to a shortage of memory to construct a message holding the
// requested stats.
// invalid -     The requested stat type is either not recognized, or the
// target is configured to not gather the stats type in question.
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// series_done - This special value indicates that no further stats info
// elements are present within a series of stats info elems
// (within a stats upload confirmation message).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_dbg_stats_status {
    HTT_DBG_STATS_STATUS_PRESENT     = 0,
    HTT_DBG_STATS_STATUS_PARTIAL     = 1,
    HTT_DBG_STATS_STATUS_ERROR       = 2,
    HTT_DBG_STATS_STATUS_INVALID     = 3,
    HTT_DBG_STATS_STATUS_SERIES_DONE = 7
}

//
// host -> target FRAG DESCRIPTOR/MSDU_EXT DESC bank
//
// The following field definitions describe the format of the HTT host
// to target frag_desc/msdu_ext bank configuration message.
// The message contains the based address and the min and max id of the
// MSDU_EXT/FRAG_DESC that will be used by the HTT to map MSDU DESC and
// MSDU_EXT/FRAG_DESC.
// HTT will use id in HTT descriptor instead sending the frag_desc_ptr.
// For QCA988X HW the firmware will use fragment_desc_ptr but in WIFI2.0
// the hardware does the mapping/translation.
//
// Total banks that can be configured is configured to 16.
//
// This should be called before any TX has be initiated by the HTT
//
// |31                         16|15           8|7   5|4       0|
// |------------------------------------------------------------|
// | DESC_SIZE    |  NUM_BANKS   | RES |SWP|pdev|    msg type   |
// |------------------------------------------------------------|
// |                     BANK0_BASE_ADDRESS                     |
// |------------------------------------------------------------|
// |                            ...                             |
// |------------------------------------------------------------|
// |                    BANK15_BASE_ADDRESS                     |
// |------------------------------------------------------------|
// |       BANK0_MAX_ID          |       BANK0_MIN_ID           |
// |------------------------------------------------------------|
// |                            ...                             |
// |------------------------------------------------------------|
// |       BANK15_MAX_ID         |       BANK15_MIN_ID          |
// |------------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Value: 0x6
// - BANKx_BASE_ADDRESS
// Bits 31:0
// Purpose: Provide a mechanism to specify the base address of the MSDU_EXT
// bank physical/bus address.
// - BANKx_MIN_ID
// Bits 15:0
// Purpose: Provide a mechanism to specify the min index that needs to
// mapped.
// - BANKx_MAX_ID
// Bits 31:16
// Purpose: Provide a mechanism to specify the max index that needs to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_frag_desc_bank_id {
    pub bank_min_id: __le16,
    pub bank_max_id: __le16,
    pub __packed: },
// real is 16 but it wouldn't fit in the max htt message size
// so we use a conservatively safe value for now
//
pub const HTT_FRAG_DESC_BANK_MAX: c_int = 4;
pub const HTT_FRAG_DESC_BANK_CFG_INFO_PDEV_ID_MASK: c_uint = 0x03;
pub const HTT_FRAG_DESC_BANK_CFG_INFO_PDEV_ID_LSB: c_int = 0;

pub const HTT_FRAG_DESC_BANK_CFG_INFO_Q_STATE_DEPTH_TYPE_LSB: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_q_depth_type {
    HTT_Q_DEPTH_TYPE_BYTES = 0,
    HTT_Q_DEPTH_TYPE_MSDUS = 1,
}

pub const HTT_TX_Q_STATE_NUM_TIDS: c_int = 8;
pub const HTT_TX_Q_STATE_ENTRY_SIZE: c_int = 1;
pub const HTT_TX_Q_STATE_ENTRY_MULTIPLIER: c_int = 0;
//
// struct htt_q_state_conf - part of htt_frag_desc_bank_cfg for host q state config
//
// Defines host q state format and behavior. See htt_q_state.
//
// @paddr: Queue physical address
// @num_peers: Number of supported peers
// @num_tids: Number of supported TIDs
// @record_size: Defines the size of each host q entry in bytes. In practice
// however firmware (at least 10.4.3-00191) ignores this host
// configuration value and uses hardcoded value of 1.
// @record_multiplier: This is valid only when q depth type is MSDUs. It
// defines the exponent for the power of 2 multiplication.
// @pad: struct padding for 32-bit alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_q_state_conf {
    pub paddr: __le32,
    pub num_peers: __le16,
    pub num_tids: __le16,
    pub record_size: u8,
    pub record_multiplier: u8,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_frag_desc_bank_cfg32 {
    pub /: *mut *mut u8 info; / HTT_FRAG_DESC_BANK_CFG_INFO_,
    pub num_banks: u8,
    pub desc_size: u8,
    pub bank_base_addrs: [__le32; HTT_FRAG_DESC_BANK_MAX],
    pub bank_id: [htt_frag_desc_bank_id; HTT_FRAG_DESC_BANK_MAX],
    pub q_state: htt_q_state_conf,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_frag_desc_bank_cfg64 {
    pub /: *mut *mut u8 info; / HTT_FRAG_DESC_BANK_CFG_INFO_,
    pub num_banks: u8,
    pub desc_size: u8,
    pub bank_base_addrs: [__le64; HTT_FRAG_DESC_BANK_MAX],
    pub bank_id: [htt_frag_desc_bank_id; HTT_FRAG_DESC_BANK_MAX],
    pub q_state: htt_q_state_conf,
    pub __packed: },
pub const HTT_TX_Q_STATE_ENTRY_COEFFICIENT: c_int = 128;
pub const HTT_TX_Q_STATE_ENTRY_FACTOR_MASK: c_uint = 0x3f;
pub const HTT_TX_Q_STATE_ENTRY_FACTOR_LSB: c_int = 0;
pub const HTT_TX_Q_STATE_ENTRY_EXP_MASK: c_uint = 0xc0;
pub const HTT_TX_Q_STATE_ENTRY_EXP_LSB: c_int = 6;
//
// struct htt_q_state - shared between host and firmware via DMA
//
// This structure is used for the host to expose it's software queue state to
// firmware so that its rate control can schedule fetch requests for optimized
// performance. This is most notably used for MU-MIMO aggregation when multiple
// MU clients are connected.
//
// @count: Each element defines the host queue depth. When q depth type was
// configured as HTT_Q_DEPTH_TYPE_BYTES then each entry is defined as:
// FACTOR * 128 * 8^EXP (see HTT_TX_Q_STATE_ENTRY_FACTOR_MASK and
// HTT_TX_Q_STATE_ENTRY_EXP_MASK). When q depth type was configured as
// HTT_Q_DEPTH_TYPE_MSDUS the number of packets is scaled by 2
// record_multiplier (see htt_q_state_conf).
// @map: Used by firmware to quickly check which host queues are not empty. It
// is a bitmap simply saying.
// @seq: Used by firmware to quickly check if the host queues were updated
// since it last checked.
//
// FIXME: Is the q_state map[] size calculation really correct?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_q_state {
    pub count: [u8; HTT_TX_Q_STATE_NUM_TIDS][HTT_TX_Q_STATE_NUM_PEERS],
    pub 32]: u32 map[HTT_TX_Q_STATE_NUM_TIDS][(HTT_TX_Q_STATE_NUM_PEERS + 31) /,
    pub seq: __le32,
    pub __packed: },
pub const HTT_TX_FETCH_RECORD_INFO_PEER_ID_MASK: c_uint = 0x0fff;
pub const HTT_TX_FETCH_RECORD_INFO_PEER_ID_LSB: c_int = 0;
pub const HTT_TX_FETCH_RECORD_INFO_TID_MASK: c_uint = 0xf000;
pub const HTT_TX_FETCH_RECORD_INFO_TID_LSB: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_fetch_record {
    pub /: *mut *mut __le16 info; / HTT_TX_FETCH_IND_RECORD_INFO_,
    pub num_msdus: __le16,
    pub num_bytes: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_fetch_ind {
    pub pad0: u8,
    pub fetch_seq_num: __le16,
    pub token: __le32,
    pub num_resp_ids: __le16,
    pub num_records: __le16,
// ath10k_htt_get_tx_fetch_ind_resp_ids()
    pub resp_ids): DECLARE_FLEX_ARRAY(__le32,,
    pub records): DECLARE_FLEX_ARRAY(struct htt_tx_fetch_record,,
    pub __packed: },
    pub __packed: },
    pub )&ind->records[le16_to_cpu(ind->num_records)]: *mut return (void,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_fetch_resp {
    pub pad0: u8,
    pub resp_id: __le16,
    pub fetch_seq_num: __le16,
    pub num_records: __le16,
    pub token: __le32,
    pub records: [htt_tx_fetch_record; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_fetch_confirm {
    pub pad0: u8,
    pub num_resp_ids: __le16,
    pub resp_ids: [__le32; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_tx_mode_switch_mode {
    HTT_TX_MODE_SWITCH_PUSH = 0,
    HTT_TX_MODE_SWITCH_PUSH_PULL = 1,
}

pub const HTT_TX_MODE_SWITCH_IND_INFO0_NUM_RECORDS_MASK: c_uint = 0xfffe;
pub const HTT_TX_MODE_SWITCH_IND_INFO0_NUM_RECORDS_LSB: c_int = 1;
pub const HTT_TX_MODE_SWITCH_IND_INFO1_MODE_MASK: c_uint = 0x0003;
pub const HTT_TX_MODE_SWITCH_IND_INFO1_MODE_LSB: c_int = 0;
pub const HTT_TX_MODE_SWITCH_IND_INFO1_THRESHOLD_MASK: c_uint = 0xfffc;
pub const HTT_TX_MODE_SWITCH_IND_INFO1_THRESHOLD_LSB: c_int = 2;
pub const HTT_TX_MODE_SWITCH_RECORD_INFO0_PEER_ID_MASK: c_uint = 0x0fff;
pub const HTT_TX_MODE_SWITCH_RECORD_INFO0_PEER_ID_LSB: c_int = 0;
pub const HTT_TX_MODE_SWITCH_RECORD_INFO0_TID_MASK: c_uint = 0xf000;
pub const HTT_TX_MODE_SWITCH_RECORD_INFO0_TID_LSB: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_mode_switch_record {
    pub /: *mut *mut __le16 info0; / HTT_TX_MODE_SWITCH_RECORD_INFO0_,
    pub num_max_msdus: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_mode_switch_ind {
    pub pad0: u8,
    pub /: *mut *mut __le16 info0; / HTT_TX_MODE_SWITCH_IND_INFO0_,
    pub /: *mut *mut __le16 info1; / HTT_TX_MODE_SWITCH_IND_INFO1_,
    pub pad1: [u8; 2],
    pub records: [htt_tx_mode_switch_record; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_channel_change {
    pub pad: [u8; 3],
    pub freq: __le32,
    pub center_freq1: __le32,
    pub center_freq2: __le32,
    pub phymode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_per_peer_tx_stats_ind {
    pub succ_bytes: __le32,
    pub retry_bytes: __le32,
    pub failed_bytes: __le32,
    pub ratecode: u8,
    pub flags: u8,
    pub peer_id: __le16,
    pub succ_pkts: __le16,
    pub retry_pkts: __le16,
    pub failed_pkts: __le16,
    pub tx_duration: __le16,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_peer_tx_stats {
    pub num_ppdu: u8,
    pub ppdu_len: u8,
    pub version: u8,
    pub payload: [u8; ],
    pub __packed: },
pub const ATH10K_10_2_TX_STATS_OFFSET: c_int = 136;
pub const PEER_STATS_FOR_NO_OF_PPDUS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_10_2_peer_tx_stats {
    pub ratecode: [u8; PEER_STATS_FOR_NO_OF_PPDUS],
    pub success_pkts: [u8; PEER_STATS_FOR_NO_OF_PPDUS],
    pub success_bytes: [__le16; PEER_STATS_FOR_NO_OF_PPDUS],
    pub retry_pkts: [u8; PEER_STATS_FOR_NO_OF_PPDUS],
    pub retry_bytes: [__le16; PEER_STATS_FOR_NO_OF_PPDUS],
    pub failed_pkts: [u8; PEER_STATS_FOR_NO_OF_PPDUS],
    pub failed_bytes: [__le16; PEER_STATS_FOR_NO_OF_PPDUS],
    pub flags: [u8; PEER_STATS_FOR_NO_OF_PPDUS],
    pub tx_duration: __le32,
    pub tx_ppdu_cnt: u8,
    pub peer_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union htt_rx_pn_t {
// WEP: 24-bit PN
    pub pn24: u32,
// TKIP or CCMP: 48-bit PN
    pub pn48: u64,
// WAPI: 128-bit PN
    pub pn128: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_cmd {
    pub hdr: htt_cmd_hdr,
    pub ver_req: htt_ver_req,
    pub mgmt_tx: htt_mgmt_tx_desc,
    pub data_tx: htt_data_tx_desc,
    pub rx_setup_32: htt_rx_ring_setup_32,
    pub rx_setup_64: htt_rx_ring_setup_64,
    pub stats_req: htt_stats_req,
    pub oob_sync_req: htt_oob_sync_req,
    pub aggr_conf: htt_aggr_conf,
    pub aggr_conf_v2: htt_aggr_conf_v2,
    pub frag_desc_bank_cfg32: htt_frag_desc_bank_cfg32,
    pub frag_desc_bank_cfg64: htt_frag_desc_bank_cfg64,
    pub tx_fetch_resp: htt_tx_fetch_resp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_resp {
    pub hdr: htt_resp_hdr,
    pub ver_resp: htt_ver_resp,
    pub mgmt_tx_completion: htt_mgmt_tx_completion,
    pub data_tx_completion: htt_data_tx_completion,
    pub rx_ind: htt_rx_indication,
    pub rx_ind_hl: htt_rx_indication_hl,
    pub rx_frag_ind: htt_rx_fragment_indication,
    pub peer_map: htt_rx_peer_map,
    pub peer_unmap: htt_rx_peer_unmap,
    pub rx_flush: htt_rx_flush,
    pub rx_addba: htt_rx_addba,
    pub rx_delba: htt_rx_delba,
    pub security_indication: htt_security_indication,
    pub rc_update: htt_rc_update,
    pub rx_test: htt_rx_test,
    pub pktlog_msg: htt_pktlog_msg,
    pub rx_pn_ind: htt_rx_pn_ind,
    pub rx_offload_ind: htt_rx_offload_ind,
    pub rx_in_ord_ind: htt_rx_in_ord_ind,
    pub tx_fetch_ind: htt_tx_fetch_ind,
    pub tx_fetch_confirm: htt_tx_fetch_confirm,
    pub tx_mode_switch_ind: htt_tx_mode_switch_ind,
    pub chan_change: htt_channel_change,
    pub peer_tx_stats: htt_peer_tx_stats,
    pub __packed: },
    pub __packed: },
// host side structures follow
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_done {
    pub msdu_id: u16,
    pub status: u16,
    pub ack_rssi: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_tx_compl_state {
    HTT_TX_COMPL_STATE_NONE,
    HTT_TX_COMPL_STATE_ACK,
    HTT_TX_COMPL_STATE_NOACK,
    HTT_TX_COMPL_STATE_DISCARD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_peer_map_event {
    pub vdev_id: u8,
    pub peer_id: u16,
    pub addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_peer_unmap_event {
    pub peer_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt_txbuf_32 {
    pub frags: [htt_data_tx_desc_frag; 2],
    pub htc_hdr: ath10k_htc_hdr,
    pub cmd_hdr: htt_cmd_hdr,
    pub cmd_tx: htt_data_tx_desc,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt_txbuf_64 {
    pub frags: [htt_data_tx_desc_frag; 2],
    pub htc_hdr: ath10k_htc_hdr,
    pub cmd_hdr: htt_cmd_hdr,
    pub cmd_tx: htt_data_tx_desc_64,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt {
    pub ar: *mut ath10k,
    pub eid: ath10k_htc_ep_id,
    pub rx_indication_head: sk_buff_head,
    pub target_version_major: u8,
    pub target_version_minor: u8,
    pub target_version_received: completion,
    pub max_num_amsdu: u8,
    pub max_num_ampdu: u8,
    pub t2h_msg_types: *const htt_t2h_msg_type,
    pub t2h_msg_types_max: u32,
//
// Ring of network buffer objects - This ring is
// used exclusively by the host SW. This ring
// mirrors the dev_addrs_ring that is shared
// between the host SW and the MAC HW. The host SW
// uses this netbufs ring to locate the network
// buffer objects whose data buffers the HW has
// filled.
//
    pub netbufs_ring: *mut sk_buff,
// This is used only with firmware supporting IN_ORD_IND.
//
// With Full Rx Reorder the HTT Rx Ring is more of a temporary
// buffer ring from which buffer addresses are copied by the
// firmware to MAC Rx ring. Firmware then delivers IN_ORD_IND
// pointing to specific (re-ordered) buffers.
//
// FIXME: With kernel generic hashing functions there's a lot
// of hash collisions for sk_buffs.
//
    pub in_ord_rx: bool,
    pub 4): DECLARE_HASHTABLE(skb_table,,
//
// Ring of buffer addresses -
// This ring holds the "physical" device address of the
// rx buffers the host SW provides for the MAC HW to
// fill.
//
    pub paddrs_ring_64: *mut __le64,
    pub paddrs_ring_32: *mut __le32,
}

//
// Base address of ring, as a "physical" device address
// rather than a CPU address.
//
// how many elems in the ring (power of 2)
// size - 1
// how many rx buffers to keep in the ring
// how many rx buffers (full+empty) are in the ring
//
// alloc_idx - where HTT SW has deposited empty buffers
// This is allocated in consistent mem, so that the FW can
// read this variable, and program the HW's FW_IDX reg with
// the value of this shadow register.
//
// where HTT SW has processed bufs filled by rx MAC DMA
//
// refill_retry_timer - timer triggered when the ring is
// not refilled to the level expected
//
// Protects access to all rx ring buffer state variables
// Protects access to pending_tx, num_pending_tx
// FIFO for storing tx done status {ack, no-ack, discard} and msdu id
// set if host-fw communication goes haywire
// used to avoid further failures
//
// This is used to group tx/rx completions separately and process them
// in batches to reduce cache stalls
//
// rx_status template
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt_tx_ops {
    pub htt): *mut *mut int (htt_send_rx_ring_cfg)(struct ath10k_htt,
    pub htt): *mut *mut int (htt_send_frag_desc_bank_cfg)(struct ath10k_htt,
    pub htt): *mut *mut int (htt_alloc_frag_desc)(struct ath10k_htt,
    pub htt): *mut *mut void (htt_free_frag_desc)(struct ath10k_htt,
    pub msdu): *mut sk_buff,
    pub htt): *mut *mut int (htt_alloc_txbuff)(struct ath10k_htt,
    pub htt): *mut *mut void (htt_free_txbuff)(struct ath10k_htt,
    pub max_subfrms_amsdu): u8,
    pub htt): *mut *mut void (htt_flush_tx)(struct ath10k_htt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt_rx_ops {
    pub htt): *mut *mut size_t (htt_get_rx_ring_size)(struct ath10k_htt,
    pub vaddr): *mut *mut *mut void (htt_config_paddrs_ring)(struct ath10k_htt htt, void,
    pub idx): c_int,
    pub htt): *mut *mut *mut void (htt_get_vaddr_ring)(struct ath10k_htt,
    pub idx): *mut *mut *mut void (htt_reset_paddrs_ring)(struct ath10k_htt htt, int,
    pub skb): *mut sk_buff,
}

// the driver strongly assumes that the rx header status be 64 bytes long,
// so all possible rx_desc structures must respect this assumption.
//
pub const RX_HTT_HDR_STATUS_LEN: c_int = 64;
// The rx descriptor structure layout is programmed via rx ring setup
// so that FW knows how to transfer the rx descriptor to the host.
// Unfortunately, though, QCA6174's firmware doesn't currently behave correctly
// when modifying the structure layout of the rx descriptor beyond what it expects
// (even if it correctly programmed during the rx ring setup).
// Therefore we must keep two different memory layouts, abstract the rx descriptor
// representation and use ath10k_rx_desc_ops
// for correctly accessing rx descriptor data.
//
// base struct used for abstracting the rx descriptor representation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_desc {
// This field is filled on the host using the msdu buffer
// from htt_rx_indication
//
    pub fw_desc: fw_rx_desc_base,
    pub pad: u32,
    pub __packed: },
    pub __packed: },
// rx descriptor for wcn3990 and possibly extensible for newer cards
// Buffers like this are placed on the rx ring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_desc_v2 {
    pub base: htt_rx_desc,
    pub attention: rx_attention,
    pub frag_info: rx_frag_info,
    pub mpdu_start: rx_mpdu_start,
    pub msdu_start: rx_msdu_start,
    pub msdu_end: rx_msdu_end,
    pub mpdu_end: rx_mpdu_end,
    pub ppdu_start: rx_ppdu_start,
    pub ppdu_end: rx_ppdu_end,
    pub __packed: },
    pub rx_hdr_status: [u8; RX_HTT_HDR_STATUS_LEN],
    pub msdu_payload: [u8; ],
}

// QCA6174, QCA988x, QCA99x0 dedicated rx descriptor to make sure their firmware
// works correctly. We keep a single rx descriptor for all these three
// families of cards because from tests it seems to be the most stable solution,
// e.g. having a rx descriptor only for QCA6174 seldom caused firmware crashes
// during some tests.
// Buffers like this are placed on the rx ring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_desc_v1 {
    pub base: htt_rx_desc,
    pub attention: rx_attention,
    pub frag_info: rx_frag_info_v1,
    pub mpdu_start: rx_mpdu_start,
    pub msdu_start: rx_msdu_start_v1,
    pub msdu_end: rx_msdu_end_v1,
    pub mpdu_end: rx_mpdu_end,
    pub ppdu_start: rx_ppdu_start,
    pub ppdu_end: rx_ppdu_end_v1,
    pub __packed: },
    pub rx_hdr_status: [u8; RX_HTT_HDR_STATUS_LEN],
    pub msdu_payload: [u8; ],
}

// rx_desc abstraction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htt_rx_desc_ops {
// These fields are mandatory, they must be specified in any instance
// sizeof() of the rx_desc structure used by this hw
    pub rx_desc_size: usize,
// offset of msdu_payload inside the rx_desc structure used by this hw
    pub rx_desc_msdu_payload_offset: usize,
// These fields are options.
// When a field is not provided the default implementation gets used
// (see the ath10k_rx_desc_* operations below for more info about the defaults)
//
    pub rxd): *mut *mut bool (rx_desc_get_msdu_limit_error)(struct htt_rx_desc,
    pub rxd): *mut *mut int (rx_desc_get_l3_pad_bytes)(struct htt_rx_desc,
// Safely cast from a void* buffer containing an rx descriptor
// to the proper rx_desc structure
//
    pub buff): *mut *mut *mut htt_rx_desc (rx_desc_from_raw_buffer)(void,
    pub offs): *mut *mut void (rx_desc_get_offsets)(struct htt_rx_ring_rx_desc_offsets,
    pub rxd): *mut *mut *mut rx_attention (rx_desc_get_attention)(htt_rx_desc,
    pub rxd): *mut *mut *mut rx_frag_info_common (rx_desc_get_frag_info)(htt_rx_desc,
    pub rxd): *mut *mut *mut rx_mpdu_start (rx_desc_get_mpdu_start)(htt_rx_desc,
    pub rxd): *mut *mut *mut rx_mpdu_end (rx_desc_get_mpdu_end)(htt_rx_desc,
    pub rxd): *mut *mut *mut rx_msdu_start_common (rx_desc_get_msdu_start)(htt_rx_desc,
    pub rxd): *mut *mut *mut rx_msdu_end_common (rx_desc_get_msdu_end)(htt_rx_desc,
    pub rxd): *mut *mut *mut rx_ppdu_start (rx_desc_get_ppdu_start)(htt_rx_desc,
    pub rxd): *mut *mut *mut rx_ppdu_end_common (rx_desc_get_ppdu_end)(htt_rx_desc,
    pub rxd): *mut *mut *mut u8 (rx_desc_get_rx_hdr_status)(struct htt_rx_desc,
    pub rxd): *mut *mut *mut u8 (rx_desc_get_msdu_payload)(struct htt_rx_desc,
}

// The default implementation of all these getters is using the old rx_desc,
// so that it is easier to define the ath10k_htt_rx_desc_ops instances.
// But probably, if new wireless cards must be supported, it would be better
// to switch the default implementation to the new rx_desc, since this would
// make the extension easier .
//

pub const HTT_RX_DESC_HL_INFO_SEQ_NUM_MASK: c_uint = 0x00000fff;
pub const HTT_RX_DESC_HL_INFO_SEQ_NUM_LSB: c_int = 0;
pub const HTT_RX_DESC_HL_INFO_ENCRYPTED_MASK: c_uint = 0x00001000;
pub const HTT_RX_DESC_HL_INFO_ENCRYPTED_LSB: c_int = 12;
pub const HTT_RX_DESC_HL_INFO_CHAN_INFO_PRESENT_MASK: c_uint = 0x00002000;
pub const HTT_RX_DESC_HL_INFO_CHAN_INFO_PRESENT_LSB: c_int = 13;
pub const HTT_RX_DESC_HL_INFO_MCAST_BCAST_MASK: c_uint = 0x00010000;
pub const HTT_RX_DESC_HL_INFO_MCAST_BCAST_LSB: c_int = 16;
pub const HTT_RX_DESC_HL_INFO_KEY_ID_OCT_MASK: c_uint = 0x01fe0000;
pub const HTT_RX_DESC_HL_INFO_KEY_ID_OCT_LSB: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_desc_base_hl {
    pub /: *mut *mut __le32 info; / HTT_RX_DESC_HL_INFO_,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_chan_info {
    pub primary_chan_center_freq_mhz: __le16,
    pub contig_chan1_center_freq_mhz: __le16,
    pub contig_chan2_center_freq_mhz: __le16,
    pub phy_mode: u8,
    pub reserved: u8,
    pub __packed: },
pub const HTT_RX_DESC_ALIGN: c_int = 8;
pub const HTT_MAC_ADDR_LEN: c_int = 6;
//
// FIX THIS
// Should be: sizeof(struct htt_host_rx_desc) + max rx MSDU size,
// rounded up to a cache line size.
//
pub const HTT_RX_BUF_SIZE: c_int = 2048;
// The HTT_RX_MSDU_SIZE can't be statically computed anymore,
// because it depends on the underlying device rx_desc representation
//
    pub (int)hw->rx_desc_ops->rx_desc_size: return HTT_RX_BUF_SIZE -,
// Refill a bunch of RX buffers for each refill round so that FW/HW can handle
// aggregated traffic more nicely.
//
pub const ATH10K_HTT_MAX_NUM_REFILL: c_int = 100;
//
// DMA_MAP expects the buffer to be an integral number of cache lines.
// Rather than checking the actual cache line size, this code makes a
// conservative estimate of what the cache line size could be.
//

// These values are default in most firmware revisions and apparently are a
// sweet spot performance wise.
//
pub const ATH10K_HTT_MAX_NUM_AMSDU_DEFAULT: c_int = 3;
pub const ATH10K_HTT_MAX_NUM_AMPDU_DEFAULT: c_int = 64;
    pub htt): *mut int ath10k_htt_connect(struct ath10k_htt,
    pub ar): *mut int ath10k_htt_init(struct ath10k,
    pub htt): *mut int ath10k_htt_setup(struct ath10k_htt,
    pub htt): *mut int ath10k_htt_tx_start(struct ath10k_htt,
    pub htt): *mut void ath10k_htt_tx_stop(struct ath10k_htt,
    pub htt): *mut void ath10k_htt_tx_destroy(struct ath10k_htt,
    pub htt): *mut void ath10k_htt_tx_free(struct ath10k_htt,
    pub htt): *mut int ath10k_htt_rx_alloc(struct ath10k_htt,
    pub ar): *mut int ath10k_htt_rx_ring_refill(struct ath10k,
    pub htt): *mut void ath10k_htt_rx_free(struct ath10k_htt,
    pub skb): *mut *mut void ath10k_htt_htc_tx_complete(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_htt_htc_t2h_msg_handler(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut bool ath10k_htt_t2h_msg_handler(struct ath10k ar, struct sk_buff,
    pub htt): *mut int ath10k_htt_h2t_ver_req_msg(struct ath10k_htt,
    pub cookie): u64,
    pub skb): *mut *mut void ath10k_htt_hif_tx_complete(struct ath10k ar, struct sk_buff,
    pub num_records): usize,
    pub ar): *mut void ath10k_htt_op_ep_tx_credits(struct ath10k,
    pub txq): *mut ieee80211_txq,
    pub txq): *mut ieee80211_txq,
    pub ar): *mut void ath10k_htt_tx_txq_sync(struct ath10k,
    pub htt): *mut void ath10k_htt_tx_dec_pending(struct ath10k_htt,
    pub htt): *mut int ath10k_htt_tx_inc_pending(struct ath10k_htt,
    pub htt): *mut void ath10k_htt_tx_mgmt_dec_pending(struct ath10k_htt,
    pub is_presp): bool,
    pub skb): *mut *mut int ath10k_htt_tx_alloc_msdu_id(struct ath10k_htt htt, struct sk_buff,
    pub msdu_id): *mut *mut void ath10k_htt_tx_free_msdu_id(struct ath10k_htt htt, u16,
    pub msdu): *mut *mut int ath10k_htt_mgmt_tx(struct ath10k_htt htt, struct sk_buff,
    pub skb): *mut sk_buff,
    pub budget): *mut *mut int ath10k_htt_txrx_compl_task(struct ath10k ar, int,
    pub budget): *mut *mut int ath10k_htt_rx_hl_indication(struct ath10k ar, int,
    pub htt): *mut void ath10k_htt_set_tx_ops(struct ath10k_htt,
    pub htt): *mut void ath10k_htt_set_rx_ops(struct ath10k_htt,
