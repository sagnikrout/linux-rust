//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-ht.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// IEEE 802.11 HT definitions
//
// Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
// <jkmaline@cc.hut.fi>
// Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
// Copyright (c) 2005, Devicescape Software, Inc.
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
// Copyright (c) 2016 - 2017 Intel Deutschland GmbH
// Copyright (c) 2018 - 2025 Intel Corporation
//

// Maximal size of an A-MSDU that can be transported in a HT BA session
pub const IEEE80211_MAX_MPDU_LEN_HT_BA: c_int = 4095;
// Maximal size of an A-MSDU
pub const IEEE80211_MAX_MPDU_LEN_HT_3839: c_int = 3839;
pub const IEEE80211_MAX_MPDU_LEN_HT_7935: c_int = 7935;
pub const IEEE80211_HT_CTL_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_ht_chanwidth_values {
    IEEE80211_HT_CHANWIDTH_20MHZ = 0,
    IEEE80211_HT_CHANWIDTH_ANY = 1,
}

//
// struct ieee80211_bar - Block Ack Request frame format
// @frame_control: Frame Control
// @duration: Duration
// @ra: RA
// @ta: TA
// @control: BAR Control
// @start_seq_num: Starting Sequence Number (see Figure 9-37)
//
// This structure represents the "BlockAckReq frame format"
// as described in IEEE Std 802.11-2020 section 9.3.1.7.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_bar {
    pub frame_control: __le16,
    pub duration: __le16,
    pub ra: [__u8; ETH_ALEN],
    pub ta: [__u8; ETH_ALEN],
    pub control: __le16,
    pub start_seq_num: __le16,
    pub __packed: },
// 802.11 BAR control masks
pub const IEEE80211_BAR_CTRL_ACK_POLICY_NORMAL: c_uint = 0x0000;
pub const IEEE80211_BAR_CTRL_MULTI_TID: c_uint = 0x0002;
pub const IEEE80211_BAR_CTRL_CBMTID_COMPRESSED_BA: c_uint = 0x0004;
pub const IEEE80211_BAR_CTRL_TID_INFO_MASK: c_uint = 0xf000;
pub const IEEE80211_BAR_CTRL_TID_INFO_SHIFT: c_int = 12;
pub const IEEE80211_HT_MCS_MASK_LEN: c_int = 10;
//
// struct ieee80211_mcs_info - Supported MCS Set field
// @rx_mask: RX mask
// @rx_highest: highest supported RX rate. If set represents
// the highest supported RX data rate in units of 1 Mbps.
// If this field is 0 this value should not be used to
// consider the highest RX data rate supported.
// @tx_params: TX parameters
// @reserved: Reserved bits
//
// This structure represents the "Supported MCS Set field" as
// described in IEEE Std 802.11-2020 section 9.4.2.55.4.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mcs_info {
    pub rx_mask: [u8; IEEE80211_HT_MCS_MASK_LEN],
    pub rx_highest: __le16,
    pub tx_params: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// 802.11n HT capability MSC set
pub const IEEE80211_HT_MCS_RX_HIGHEST_MASK: c_uint = 0x3ff;
pub const IEEE80211_HT_MCS_TX_DEFINED: c_uint = 0x01;
pub const IEEE80211_HT_MCS_TX_RX_DIFF: c_uint = 0x02;
// value 0 == 1 stream etc
pub const IEEE80211_HT_MCS_TX_MAX_STREAMS_MASK: c_uint = 0x0C;
pub const IEEE80211_HT_MCS_TX_MAX_STREAMS_SHIFT: c_int = 2;
pub const IEEE80211_HT_MCS_TX_MAX_STREAMS: c_int = 4;
pub const IEEE80211_HT_MCS_TX_UNEQUAL_MODULATION: c_uint = 0x10;

//
// 802.11n D5.0 20.3.5 / 20.6 says:
// - indices 0 to 7 and 32 are single spatial stream
// - 8 to 31 are multiple spatial streams using equal modulation
// [8..15 for two streams, 16..23 for three and 24..31 for four]
// - remainder are multiple spatial streams using unequal modulation
//
pub const IEEE80211_HT_MCS_UNEQUAL_MODULATION_START: c_int = 33;

//
// struct ieee80211_ht_cap - HT capabilities element
// @cap_info: HT Capability Information
// @ampdu_params_info: A-MPDU Parameters
// @mcs: Supported MCS Set
// @extended_ht_cap_info: HT Extended Capabilities
// @tx_BF_cap_info: Transmit Beamforming Capabilities
// @antenna_selection_info: ASEL Capability
//
// This structure represents the payload of the "HT Capabilities
// element" as described in IEEE Std 802.11-2020 section 9.4.2.55.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_ht_cap {
    pub cap_info: __le16,
    pub ampdu_params_info: u8,
// 16 bytes MCS information
    pub mcs: ieee80211_mcs_info,
    pub extended_ht_cap_info: __le16,
    pub tx_BF_cap_info: __le32,
    pub antenna_selection_info: u8,
    pub __packed: },
// 802.11n HT capabilities masks (for cap_info)
pub const IEEE80211_HT_CAP_LDPC_CODING: c_uint = 0x0001;
pub const IEEE80211_HT_CAP_SUP_WIDTH_20_40: c_uint = 0x0002;
pub const IEEE80211_HT_CAP_SM_PS: c_uint = 0x000C;
pub const IEEE80211_HT_CAP_SM_PS_SHIFT: c_int = 2;
pub const IEEE80211_HT_CAP_GRN_FLD: c_uint = 0x0010;
pub const IEEE80211_HT_CAP_SGI_20: c_uint = 0x0020;
pub const IEEE80211_HT_CAP_SGI_40: c_uint = 0x0040;
pub const IEEE80211_HT_CAP_TX_STBC: c_uint = 0x0080;
pub const IEEE80211_HT_CAP_RX_STBC: c_uint = 0x0300;
pub const IEEE80211_HT_CAP_RX_STBC_SHIFT: c_int = 8;
pub const IEEE80211_HT_CAP_DELAY_BA: c_uint = 0x0400;
pub const IEEE80211_HT_CAP_MAX_AMSDU: c_uint = 0x0800;
pub const IEEE80211_HT_CAP_DSSSCCK40: c_uint = 0x1000;
pub const IEEE80211_HT_CAP_RESERVED: c_uint = 0x2000;
pub const IEEE80211_HT_CAP_40MHZ_INTOLERANT: c_uint = 0x4000;
pub const IEEE80211_HT_CAP_LSIG_TXOP_PROT: c_uint = 0x8000;
// 802.11n HT extended capabilities masks (for extended_ht_cap_info)
pub const IEEE80211_HT_EXT_CAP_PCO: c_uint = 0x0001;
pub const IEEE80211_HT_EXT_CAP_PCO_TIME: c_uint = 0x0006;
pub const IEEE80211_HT_EXT_CAP_PCO_TIME_SHIFT: c_int = 1;
pub const IEEE80211_HT_EXT_CAP_MCS_FB: c_uint = 0x0300;
pub const IEEE80211_HT_EXT_CAP_MCS_FB_SHIFT: c_int = 8;
pub const IEEE80211_HT_EXT_CAP_HTC_SUP: c_uint = 0x0400;
pub const IEEE80211_HT_EXT_CAP_RD_RESPONDER: c_uint = 0x0800;
// 802.11n HT capability AMPDU settings (for ampdu_params_info)
pub const IEEE80211_HT_AMPDU_PARM_FACTOR: c_uint = 0x03;
pub const IEEE80211_HT_AMPDU_PARM_DENSITY: c_uint = 0x1C;
pub const IEEE80211_HT_AMPDU_PARM_DENSITY_SHIFT: c_int = 2;
//
// Maximum length of AMPDU that the STA can receive in high-throughput (HT).
// Length = 2 ^ (13 + max_ampdu_length_exp) - 1 (octets)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_max_ampdu_length_exp {
    IEEE80211_HT_MAX_AMPDU_8K = 0,
    IEEE80211_HT_MAX_AMPDU_16K = 1,
    IEEE80211_HT_MAX_AMPDU_32K = 2,
    IEEE80211_HT_MAX_AMPDU_64K = 3
}

pub const IEEE80211_HT_MAX_AMPDU_FACTOR: c_int = 13;
// Minimum MPDU start spacing
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_min_mpdu_spacing {
    IEEE80211_HT_MPDU_DENSITY_NONE = 0,	/* No restriction */
    IEEE80211_HT_MPDU_DENSITY_0_25 = 1,	/* 1/4 usec */
    IEEE80211_HT_MPDU_DENSITY_0_5 = 2,	/* 1/2 usec */
    IEEE80211_HT_MPDU_DENSITY_1 = 3,	/* 1 usec */
    IEEE80211_HT_MPDU_DENSITY_2 = 4,	/* 2 usec */
    IEEE80211_HT_MPDU_DENSITY_4 = 5,	/* 4 usec */
    IEEE80211_HT_MPDU_DENSITY_8 = 6,	/* 8 usec */
    IEEE80211_HT_MPDU_DENSITY_16 = 7	/* 16 usec */
}

//
// struct ieee80211_ht_operation - HT operation IE
// @primary_chan: Primary Channel
// @ht_param: HT Operation Information parameters
// @operation_mode: HT Operation Information operation mode
// @stbc_param: HT Operation Information STBC params
// @basic_set: Basic HT-MCS Set
//
// This structure represents the payload of the "HT Operation
// element" as described in IEEE Std 802.11-2020 section 9.4.2.56.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_ht_operation {
    pub primary_chan: u8,
    pub ht_param: u8,
    pub operation_mode: __le16,
    pub stbc_param: __le16,
    pub basic_set: [u8; 16],
    pub __packed: },
// for ht_param
pub const IEEE80211_HT_PARAM_CHA_SEC_OFFSET: c_uint = 0x03;
pub const IEEE80211_HT_PARAM_CHA_SEC_NONE: c_uint = 0x00;
pub const IEEE80211_HT_PARAM_CHA_SEC_ABOVE: c_uint = 0x01;
pub const IEEE80211_HT_PARAM_CHA_SEC_BELOW: c_uint = 0x03;
pub const IEEE80211_HT_PARAM_CHAN_WIDTH_ANY: c_uint = 0x04;
pub const IEEE80211_HT_PARAM_RIFS_MODE: c_uint = 0x08;
// for operation_mode
pub const IEEE80211_HT_OP_MODE_PROTECTION: c_uint = 0x0003;
pub const IEEE80211_HT_OP_MODE_PROTECTION_NONE: c_int = 0;
pub const IEEE80211_HT_OP_MODE_PROTECTION_NONMEMBER: c_int = 1;
pub const IEEE80211_HT_OP_MODE_PROTECTION_20MHZ: c_int = 2;
pub const IEEE80211_HT_OP_MODE_PROTECTION_NONHT_MIXED: c_int = 3;
pub const IEEE80211_HT_OP_MODE_NON_GF_STA_PRSNT: c_uint = 0x0004;
pub const IEEE80211_HT_OP_MODE_NON_HT_STA_PRSNT: c_uint = 0x0010;
pub const IEEE80211_HT_OP_MODE_CCFS2_SHIFT: c_int = 5;
pub const IEEE80211_HT_OP_MODE_CCFS2_MASK: c_uint = 0x1fe0;
// for stbc_param
pub const IEEE80211_HT_STBC_PARAM_DUAL_BEACON: c_uint = 0x0040;
pub const IEEE80211_HT_STBC_PARAM_DUAL_CTS_PROT: c_uint = 0x0080;
pub const IEEE80211_HT_STBC_PARAM_STBC_BEACON: c_uint = 0x0100;
pub const IEEE80211_HT_STBC_PARAM_LSIG_TXOP_FULLPROT: c_uint = 0x0200;
pub const IEEE80211_HT_STBC_PARAM_PCO_ACTIVE: c_uint = 0x0400;
pub const IEEE80211_HT_STBC_PARAM_PCO_PHASE: c_uint = 0x0800;
// block-ack parameters
pub const IEEE80211_ADDBA_PARAM_AMSDU_MASK: c_uint = 0x0001;
pub const IEEE80211_ADDBA_PARAM_POLICY_MASK: c_uint = 0x0002;
pub const IEEE80211_ADDBA_PARAM_TID_MASK: c_uint = 0x003C;
pub const IEEE80211_ADDBA_PARAM_BUF_SIZE_MASK: c_uint = 0xFFC0;
pub const IEEE80211_DELBA_PARAM_TID_MASK: c_uint = 0xF000;
pub const IEEE80211_DELBA_PARAM_INITIATOR_MASK: c_uint = 0x0800;
//
// A-MPDU buffer sizes
// According to HT size varies from 8 to 64 frames
// HE adds the ability to have up to 256 frames.
// EHT adds the ability to have up to 1K frames.
//
pub const IEEE80211_MIN_AMPDU_BUF: c_uint = 0x8;
pub const IEEE80211_MAX_AMPDU_BUF_HT: c_uint = 0x40;
pub const IEEE80211_MAX_AMPDU_BUF_HE: c_uint = 0x100;
pub const IEEE80211_MAX_AMPDU_BUF_EHT: c_uint = 0x400;
// Spatial Multiplexing Power Save Modes (for capability)
pub const WLAN_HT_CAP_SM_PS_STATIC: c_int = 0;
pub const WLAN_HT_CAP_SM_PS_DYNAMIC: c_int = 1;
pub const WLAN_HT_CAP_SM_PS_INVALID: c_int = 2;
pub const WLAN_HT_CAP_SM_PS_DISABLED: c_int = 3;
// for SM power control field lower two bits
pub const WLAN_HT_SMPS_CONTROL_DISABLED: c_int = 0;
pub const WLAN_HT_SMPS_CONTROL_STATIC: c_int = 1;
pub const WLAN_HT_SMPS_CONTROL_DYNAMIC: c_int = 3;
// HT action codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_ht_actioncode {
    WLAN_HT_ACTION_NOTIFY_CHANWIDTH = 0,
    WLAN_HT_ACTION_SMPS = 1,
    WLAN_HT_ACTION_PSMP = 2,
    WLAN_HT_ACTION_PCO_PHASE = 3,
    WLAN_HT_ACTION_CSI = 4,
    WLAN_HT_ACTION_NONCOMPRESSED_BF = 5,
    WLAN_HT_ACTION_COMPRESSED_BF = 6,
    WLAN_HT_ACTION_ASEL_IDX_FEEDBACK = 7,
}

// BACK action code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_back_actioncode {
    WLAN_ACTION_ADDBA_REQ = 0,
    WLAN_ACTION_ADDBA_RESP = 1,
    WLAN_ACTION_DELBA = 2,
    WLAN_ACTION_NDP_ADDBA_REQ = 128,
    WLAN_ACTION_NDP_ADDBA_RESP = 129,
    WLAN_ACTION_NDP_DELBA = 130,
}

// BACK (block-ack) parties
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_back_parties {
    WLAN_BACK_RECIPIENT = 0,
    WLAN_BACK_INITIATOR = 1,
}
