//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-eht.h
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
// IEEE 802.11 EHT definitions
//
// Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
// <jkmaline@cc.hut.fi>
// Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
// Copyright (c) 2005, Devicescape Software, Inc.
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
// Copyright (c) 2016 - 2017 Intel Deutschland GmbH
// Copyright (c) 2018 - 2026 Intel Corporation
//

// need HE definitions for the inlines here

pub const IEEE80211_TTLM_MAX_CNT: c_int = 2;
pub const IEEE80211_TTLM_CONTROL_DIRECTION: c_uint = 0x03;
pub const IEEE80211_TTLM_CONTROL_DEF_LINK_MAP: c_uint = 0x04;
pub const IEEE80211_TTLM_CONTROL_SWITCH_TIME_PRESENT: c_uint = 0x08;
pub const IEEE80211_TTLM_CONTROL_EXPECTED_DUR_PRESENT: c_uint = 0x10;
pub const IEEE80211_TTLM_CONTROL_LINK_MAP_SIZE: c_uint = 0x20;
pub const IEEE80211_TTLM_DIRECTION_DOWN: c_int = 0;
pub const IEEE80211_TTLM_DIRECTION_UP: c_int = 1;
pub const IEEE80211_TTLM_DIRECTION_BOTH: c_int = 2;
//
// struct ieee80211_ttlm_elem - TID-To-Link Mapping element
//
// Defined in section 9.4.2.314 in P802.11be_D4
//
// @control: the first part of control field
// @optional: the second part of control field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_ttlm_elem {
    pub control: u8,
    pub optional: [u8; ],
    pub __packed: },
pub const IEEE80211_EHT_MCS_NSS_RX: c_uint = 0x0f;
pub const IEEE80211_EHT_MCS_NSS_TX: c_uint = 0xf0;
//
// struct ieee80211_eht_mcs_nss_supp_20mhz_only - EHT 20MHz only station max
// supported NSS for per MCS.
//
// For each field below, bits 0 - 3 indicate the maximal number of spatial
// streams for Rx, and bits 4 - 7 indicate the maximal number of spatial streams
// for Tx.
//
// @rx_tx_mcs7_max_nss: indicates the maximum number of spatial streams
// supported for reception and the maximum number of spatial streams
// supported for transmission for MCS 0 - 7.
// @rx_tx_mcs9_max_nss: indicates the maximum number of spatial streams
// supported for reception and the maximum number of spatial streams
// supported for transmission for MCS 8 - 9.
// @rx_tx_mcs11_max_nss: indicates the maximum number of spatial streams
// supported for reception and the maximum number of spatial streams
// supported for transmission for MCS 10 - 11.
// @rx_tx_mcs13_max_nss: indicates the maximum number of spatial streams
// supported for reception and the maximum number of spatial streams
// supported for transmission for MCS 12 - 13.
// @rx_tx_max_nss: array of the previous fields for easier loop access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_eht_mcs_nss_supp_20mhz_only {
    pub rx_tx_mcs7_max_nss: u8,
    pub rx_tx_mcs9_max_nss: u8,
    pub rx_tx_mcs11_max_nss: u8,
    pub rx_tx_mcs13_max_nss: u8,
}

//
// struct ieee80211_eht_mcs_nss_supp_bw - EHT max supported NSS per MCS (except
// 20MHz only stations).
//
// For each field below, bits 0 - 3 indicate the maximal number of spatial
// streams for Rx, and bits 4 - 7 indicate the maximal number of spatial streams
// for Tx.
//
// @rx_tx_mcs9_max_nss: indicates the maximum number of spatial streams
// supported for reception and the maximum number of spatial streams
// supported for transmission for MCS 0 - 9.
// @rx_tx_mcs11_max_nss: indicates the maximum number of spatial streams
// supported for reception and the maximum number of spatial streams
// supported for transmission for MCS 10 - 11.
// @rx_tx_mcs13_max_nss: indicates the maximum number of spatial streams
// supported for reception and the maximum number of spatial streams
// supported for transmission for MCS 12 - 13.
// @rx_tx_max_nss: array of the previous fields for easier loop access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_eht_mcs_nss_supp_bw {
    pub rx_tx_mcs9_max_nss: u8,
    pub rx_tx_mcs11_max_nss: u8,
    pub rx_tx_mcs13_max_nss: u8,
}

//
// struct ieee80211_eht_cap_elem_fixed - EHT capabilities fixed data
//
// This structure is the "EHT Capabilities element" fixed fields as
// described in P802.11be_D2.0 section 9.4.2.313.
//
// @mac_cap_info: MAC capabilities, see IEEE80211_EHT_MAC_CAP
// @phy_cap_info: PHY capabilities, see IEEE80211_EHT_PHY_CAP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_eht_cap_elem_fixed {
    pub mac_cap_info: [u8; 2],
    pub phy_cap_info: [u8; 9],
    pub __packed: },
//
// struct ieee80211_eht_cap_elem - EHT capabilities element
// @fixed: fixed parts, see &ieee80211_eht_cap_elem_fixed
// @optional: optional parts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_eht_cap_elem {
    pub fixed: ieee80211_eht_cap_elem_fixed,
//
// Followed by:
// Supported EHT-MCS And NSS Set field: 4, 3, 6 or 9 octets.
// EHT PPE Thresholds field: variable length.
//
    pub optional: [u8; ],
    pub __packed: },
pub const IEEE80211_EHT_OPER_INFO_PRESENT: c_uint = 0x01;
pub const IEEE80211_EHT_OPER_DISABLED_SUBCHANNEL_BITMAP_PRESENT: c_uint = 0x02;
pub const IEEE80211_EHT_OPER_EHT_DEF_PE_DURATION: c_uint = 0x04;
pub const IEEE80211_EHT_OPER_GROUP_ADDRESSED_BU_IND_LIMIT: c_uint = 0x08;
pub const IEEE80211_EHT_OPER_GROUP_ADDRESSED_BU_IND_EXP_MASK: c_uint = 0x30;
pub const IEEE80211_EHT_OPER_MCS15_DISABLE: c_uint = 0x40;
//
// struct ieee80211_eht_operation - eht operation element
//
// This structure is the "EHT Operation Element" fields as
// described in P802.11be_D2.0 section 9.4.2.311
//
// @params: EHT operation element parameters. See &IEEE80211_EHT_OPER_
// @basic_mcs_nss: indicates the EHT-MCSs for each number of spatial streams in
// EHT PPDUs that are supported by all EHT STAs in the BSS in transmit and
// receive.
// @optional: optional parts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_eht_operation {
    pub params: u8,
    pub basic_mcs_nss: ieee80211_eht_mcs_nss_supp_20mhz_only,
    pub optional: [u8; ],
    pub __packed: },
//
// struct ieee80211_eht_operation_info - eht operation information
//
// @control: EHT operation information control.
// @ccfs0: defines a channel center frequency for a 20, 40, 80, 160, or 320 MHz
// EHT BSS.
// @ccfs1: defines a channel center frequency for a 160 or 320 MHz EHT BSS.
// @optional: optional parts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_eht_operation_info {
    pub control: u8,
    pub ccfs0: u8,
    pub ccfs1: u8,
    pub optional: [u8; ],
    pub __packed: },
// EHT MAC capabilities as defined in P802.11be_D2.0 section 9.4.2.313.2
pub const IEEE80211_EHT_MAC_CAP0_EPCS_PRIO_ACCESS: c_uint = 0x01;
pub const IEEE80211_EHT_MAC_CAP0_OM_CONTROL: c_uint = 0x02;
pub const IEEE80211_EHT_MAC_CAP0_TRIG_TXOP_SHARING_MODE1: c_uint = 0x04;
pub const IEEE80211_EHT_MAC_CAP0_TRIG_TXOP_SHARING_MODE2: c_uint = 0x08;
pub const IEEE80211_EHT_MAC_CAP0_RESTRICTED_TWT: c_uint = 0x10;
pub const IEEE80211_EHT_MAC_CAP0_SCS_TRAFFIC_DESC: c_uint = 0x20;
pub const IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_MASK: c_uint = 0xc0;
pub const IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_3895: c_int = 0;
pub const IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_7991: c_int = 1;
pub const IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_11454: c_int = 2;
pub const IEEE80211_EHT_MAC_CAP1_MAX_AMPDU_LEN_MASK: c_uint = 0x01;
pub const IEEE80211_EHT_MAC_CAP1_EHT_TRS: c_uint = 0x02;
pub const IEEE80211_EHT_MAC_CAP1_TXOP_RET: c_uint = 0x04;
pub const IEEE80211_EHT_MAC_CAP1_TWO_BQRS: c_uint = 0x08;
pub const IEEE80211_EHT_MAC_CAP1_EHT_LINK_ADAPT_MASK: c_uint = 0x30;
pub const IEEE80211_EHT_MAC_CAP1_UNSOL_EPCS_PRIO_ACCESS: c_uint = 0x40;
// EHT PHY capabilities as defined in P802.11be_D2.0 section 9.4.2.313.3
pub const IEEE80211_EHT_PHY_CAP0_320MHZ_IN_6GHZ: c_uint = 0x02;
pub const IEEE80211_EHT_PHY_CAP0_242_TONE_RU_GT20MHZ: c_uint = 0x04;
pub const IEEE80211_EHT_PHY_CAP0_NDP_4_EHT_LFT_32_GI: c_uint = 0x08;
pub const IEEE80211_EHT_PHY_CAP0_PARTIAL_BW_UL_MU_MIMO: c_uint = 0x10;
pub const IEEE80211_EHT_PHY_CAP0_SU_BEAMFORMER: c_uint = 0x20;
pub const IEEE80211_EHT_PHY_CAP0_SU_BEAMFORMEE: c_uint = 0x40;
// EHT beamformee number of spatial streams <= 80MHz is split
pub const IEEE80211_EHT_PHY_CAP0_BEAMFORMEE_SS_80MHZ_MASK: c_uint = 0x80;
pub const IEEE80211_EHT_PHY_CAP1_BEAMFORMEE_SS_80MHZ_MASK: c_uint = 0x03;
pub const IEEE80211_EHT_PHY_CAP1_BEAMFORMEE_SS_160MHZ_MASK: c_uint = 0x1c;
pub const IEEE80211_EHT_PHY_CAP1_BEAMFORMEE_SS_320MHZ_MASK: c_uint = 0xe0;
pub const IEEE80211_EHT_PHY_CAP2_SOUNDING_DIM_80MHZ_MASK: c_uint = 0x07;
pub const IEEE80211_EHT_PHY_CAP2_SOUNDING_DIM_160MHZ_MASK: c_uint = 0x38;
// EHT number of sounding dimensions for 320MHz is split
pub const IEEE80211_EHT_PHY_CAP2_SOUNDING_DIM_320MHZ_MASK: c_uint = 0xc0;
pub const IEEE80211_EHT_PHY_CAP3_SOUNDING_DIM_320MHZ_MASK: c_uint = 0x01;
pub const IEEE80211_EHT_PHY_CAP3_NG_16_SU_FEEDBACK: c_uint = 0x02;
pub const IEEE80211_EHT_PHY_CAP3_NG_16_MU_FEEDBACK: c_uint = 0x04;
pub const IEEE80211_EHT_PHY_CAP3_CODEBOOK_4_2_SU_FDBK: c_uint = 0x08;
pub const IEEE80211_EHT_PHY_CAP3_CODEBOOK_7_5_MU_FDBK: c_uint = 0x10;
pub const IEEE80211_EHT_PHY_CAP3_TRIG_SU_BF_FDBK: c_uint = 0x20;
pub const IEEE80211_EHT_PHY_CAP3_TRIG_MU_BF_PART_BW_FDBK: c_uint = 0x40;
pub const IEEE80211_EHT_PHY_CAP3_TRIG_CQI_FDBK: c_uint = 0x80;
pub const IEEE80211_EHT_PHY_CAP4_PART_BW_DL_MU_MIMO: c_uint = 0x01;
pub const IEEE80211_EHT_PHY_CAP4_PSR_SR_SUPP: c_uint = 0x02;
pub const IEEE80211_EHT_PHY_CAP4_POWER_BOOST_FACT_SUPP: c_uint = 0x04;
pub const IEEE80211_EHT_PHY_CAP4_EHT_MU_PPDU_4_EHT_LTF_08_GI: c_uint = 0x08;
pub const IEEE80211_EHT_PHY_CAP4_MAX_NC_MASK: c_uint = 0xf0;
pub const IEEE80211_EHT_PHY_CAP5_NON_TRIG_CQI_FEEDBACK: c_uint = 0x01;
pub const IEEE80211_EHT_PHY_CAP5_TX_LESS_242_TONE_RU_SUPP: c_uint = 0x02;
pub const IEEE80211_EHT_PHY_CAP5_RX_LESS_242_TONE_RU_SUPP: c_uint = 0x04;
pub const IEEE80211_EHT_PHY_CAP5_PPE_THRESHOLD_PRESENT: c_uint = 0x08;
pub const IEEE80211_EHT_PHY_CAP5_COMMON_NOMINAL_PKT_PAD_MASK: c_uint = 0x30;
pub const IEEE80211_EHT_PHY_CAP5_COMMON_NOMINAL_PKT_PAD_0US: c_int = 0;
pub const IEEE80211_EHT_PHY_CAP5_COMMON_NOMINAL_PKT_PAD_8US: c_int = 1;
pub const IEEE80211_EHT_PHY_CAP5_COMMON_NOMINAL_PKT_PAD_16US: c_int = 2;
pub const IEEE80211_EHT_PHY_CAP5_COMMON_NOMINAL_PKT_PAD_20US: c_int = 3;
// Maximum number of supported EHT LTF is split
pub const IEEE80211_EHT_PHY_CAP5_MAX_NUM_SUPP_EHT_LTF_MASK: c_uint = 0xc0;
pub const IEEE80211_EHT_PHY_CAP5_SUPP_EXTRA_EHT_LTF: c_uint = 0x40;
pub const IEEE80211_EHT_PHY_CAP6_MAX_NUM_SUPP_EHT_LTF_MASK: c_uint = 0x07;
pub const IEEE80211_EHT_PHY_CAP6_MCS15_SUPP_80MHZ: c_uint = 0x10;
pub const IEEE80211_EHT_PHY_CAP6_MCS15_SUPP_160MHZ: c_uint = 0x20;
pub const IEEE80211_EHT_PHY_CAP6_MCS15_SUPP_320MHZ: c_uint = 0x40;
pub const IEEE80211_EHT_PHY_CAP6_MCS15_SUPP_MASK: c_uint = 0x78;
pub const IEEE80211_EHT_PHY_CAP6_EHT_DUP_6GHZ_SUPP: c_uint = 0x80;
pub const IEEE80211_EHT_PHY_CAP7_20MHZ_STA_RX_NDP_WIDER_BW: c_uint = 0x01;
pub const IEEE80211_EHT_PHY_CAP7_NON_OFDMA_UL_MU_MIMO_80MHZ: c_uint = 0x02;
pub const IEEE80211_EHT_PHY_CAP7_NON_OFDMA_UL_MU_MIMO_160MHZ: c_uint = 0x04;
pub const IEEE80211_EHT_PHY_CAP7_NON_OFDMA_UL_MU_MIMO_320MHZ: c_uint = 0x08;
pub const IEEE80211_EHT_PHY_CAP7_MU_BEAMFORMER_80MHZ: c_uint = 0x10;
pub const IEEE80211_EHT_PHY_CAP7_MU_BEAMFORMER_160MHZ: c_uint = 0x20;
pub const IEEE80211_EHT_PHY_CAP7_MU_BEAMFORMER_320MHZ: c_uint = 0x40;
pub const IEEE80211_EHT_PHY_CAP7_TB_SOUNDING_FDBK_RATE_LIMIT: c_uint = 0x80;
pub const IEEE80211_EHT_PHY_CAP8_RX_1024QAM_WIDER_BW_DL_OFDMA: c_uint = 0x01;
pub const IEEE80211_EHT_PHY_CAP8_RX_4096QAM_WIDER_BW_DL_OFDMA: c_uint = 0x02;
//
// EHT operation channel width as defined in P802.11be_D2.0 section 9.4.2.311
//
pub const IEEE80211_EHT_OPER_CHAN_WIDTH: c_uint = 0x7;
pub const IEEE80211_EHT_OPER_CHAN_WIDTH_20MHZ: c_int = 0;
pub const IEEE80211_EHT_OPER_CHAN_WIDTH_40MHZ: c_int = 1;
pub const IEEE80211_EHT_OPER_CHAN_WIDTH_80MHZ: c_int = 2;
pub const IEEE80211_EHT_OPER_CHAN_WIDTH_160MHZ: c_int = 3;
pub const IEEE80211_EHT_OPER_CHAN_WIDTH_320MHZ: c_int = 4;
// Calculate 802.11be EHT capabilities IE Tx/Rx EHT MCS NSS Support Field size
    pub 0: u8 count =,
// on 2.4 GHz, if it supports 40 MHz, the result is 3
    pub 3: return,
// on 2.4 GHz, these three bits are reserved, so should be 0
    pub 3: count +=,
    pub 3: count +=,
    pub 3: count +=,
    pub count: return,
    pub 4: return from_ap ? 3 :,
// 802.11be EHT PPE Thresholds
pub const IEEE80211_EHT_PPE_THRES_NSS_POS: c_int = 0;
pub const IEEE80211_EHT_PPE_THRES_NSS_MASK: c_uint = 0xf;
pub const IEEE80211_EHT_PPE_THRES_RU_INDEX_BITMASK_MASK: c_uint = 0x1f0;
pub const IEEE80211_EHT_PPE_THRES_INFO_PPET_SIZE: c_int = 3;
pub const IEEE80211_EHT_PPE_THRES_INFO_HEADER_SIZE: c_int = 9;
//
// Calculate 802.11be EHT capabilities IE EHT field size
//
    pub n: u32,
    pub 0: return,
    pub IEEE80211_EHT_PPE_THRES_NSS_MASK): *mut *mut n = 1 + u16_get_bits(ppe_thres_hdr,,
//
// Each pair is 6 bits, and we need to add the 9 "header" bits to the
// total size.
//
    pub 8): return DIV_ROUND_UP(n,,
    pub )data: *const *const ieee80211_eht_cap_elem_fixed elem = (void,
    pub ieee80211_eht_cap_elem_fixed): u8 needed = sizeof(struct,
    pub false: return,
    pub false: return,
    pub ppe_thres_hdr: u16,
    pub false: return,
    pub needed): ppe_thres_hdr = get_unaligned_le16(data +,
    pub needed: return len >=,
    pub )data: *const *const ieee80211_eht_operation elem = (void,
    pub sizeof(*elem): *mut u8 needed =,
    pub false: return,
    pub 3: needed +=,
    pub 2: needed +=,
    pub needed: return len >=,
// must validate ieee80211_eht_oper_size_ok() first
    pub NULL: return,
    pub )eht_oper->optional: *const return (void,
// must validate ieee80211_eht_oper_size_ok() first
    pub info: *const ieee80211_eht_operation_info,
    pub ieee80211_eht_oper_info(eht_oper): info =,
    pub 0: return,
    pub 0: return,
    pub get_unaligned_le16(info->optional): return,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_bandwidth_indication {
    pub params: u8,
    pub info: ieee80211_eht_operation_info,
    pub __packed: },
    pub )data: *const *const ieee80211_bandwidth_indication bwi = (void,
    pub false: return,
    pub false: return,
    pub true: return,
// Protected EHT action codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_protected_eht_actioncode {
    WLAN_PROTECTED_EHT_ACTION_TTLM_REQ = 0,
    WLAN_PROTECTED_EHT_ACTION_TTLM_RES = 1,
    WLAN_PROTECTED_EHT_ACTION_TTLM_TEARDOWN = 2,
    WLAN_PROTECTED_EHT_ACTION_EPCS_ENABLE_REQ = 3,
    WLAN_PROTECTED_EHT_ACTION_EPCS_ENABLE_RESP = 4,
    WLAN_PROTECTED_EHT_ACTION_EPCS_ENABLE_TEARDOWN = 5,
    WLAN_PROTECTED_EHT_ACTION_EML_OP_MODE_NOTIF = 6,
    WLAN_PROTECTED_EHT_ACTION_LINK_RECOMMEND = 7,
    WLAN_PROTECTED_EHT_ACTION_ML_OP_UPDATE_REQ = 8,
    WLAN_PROTECTED_EHT_ACTION_ML_OP_UPDATE_RESP = 9,
    WLAN_PROTECTED_EHT_ACTION_LINK_RECONFIG_NOTIF = 10,
    WLAN_PROTECTED_EHT_ACTION_LINK_RECONFIG_REQ = 11,
    WLAN_PROTECTED_EHT_ACTION_LINK_RECONFIG_RESP = 12,
}

// multi-link device
pub const IEEE80211_MLD_MAX_NUM_LINKS: c_int = 15;
pub const IEEE80211_ML_CONTROL_TYPE: c_uint = 0x0007;
pub const IEEE80211_ML_CONTROL_TYPE_BASIC: c_int = 0;
pub const IEEE80211_ML_CONTROL_TYPE_PREQ: c_int = 1;
pub const IEEE80211_ML_CONTROL_TYPE_RECONF: c_int = 2;
pub const IEEE80211_ML_CONTROL_TYPE_TDLS: c_int = 3;
pub const IEEE80211_ML_CONTROL_TYPE_PRIO_ACCESS: c_int = 4;
pub const IEEE80211_ML_CONTROL_PRESENCE_MASK: c_uint = 0xfff0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_multi_link_elem {
    pub control: __le16,
    pub variable: [u8; ],
    pub __packed: },
pub const IEEE80211_MLC_BASIC_PRES_LINK_ID: c_uint = 0x0010;
pub const IEEE80211_MLC_BASIC_PRES_BSS_PARAM_CH_CNT: c_uint = 0x0020;
pub const IEEE80211_MLC_BASIC_PRES_MED_SYNC_DELAY: c_uint = 0x0040;
pub const IEEE80211_MLC_BASIC_PRES_EML_CAPA: c_uint = 0x0080;
pub const IEEE80211_MLC_BASIC_PRES_MLD_CAPA_OP: c_uint = 0x0100;
pub const IEEE80211_MLC_BASIC_PRES_MLD_ID: c_uint = 0x0200;
pub const IEEE80211_MLC_BASIC_PRES_EXT_MLD_CAPA_OP: c_uint = 0x0400;
pub const IEEE80211_MLC_BASIC_PRES_ENH_CRIT_UPD: c_uint = 0x0800;
pub const IEEE80211_MED_SYNC_DELAY_DURATION: c_uint = 0x00ff;
pub const IEEE80211_MED_SYNC_DELAY_SYNC_OFDM_ED_THRESH: c_uint = 0x0f00;
pub const IEEE80211_MED_SYNC_DELAY_SYNC_MAX_NUM_TXOPS: c_uint = 0xf000;
//
// Described in P802.11be_D3.0
// dot11MSDTimerDuration should default to 5484 (i.e. 171.375)
// dot11MSDOFDMEDthreshold defaults to -72 (i.e. 0)
// dot11MSDTXOPMAX defaults to 1
//
pub const IEEE80211_MED_SYNC_DELAY_DEFAULT: c_uint = 0x10ac;
pub const IEEE80211_EML_CAP_EMLSR_SUPP: c_uint = 0x0001;
pub const IEEE80211_EML_CAP_EML_PADDING_DELAY: c_uint = 0x000e;
// Described Tables 9-417i & 9-417k in 802.11be-2024, which have the same values
pub const IEEE80211_EML_CAP_EML_PADDING_DELAY_0US: c_int = 0;
pub const IEEE80211_EML_CAP_EML_PADDING_DELAY_32US: c_int = 1;
pub const IEEE80211_EML_CAP_EML_PADDING_DELAY_64US: c_int = 2;
pub const IEEE80211_EML_CAP_EML_PADDING_DELAY_128US: c_int = 3;
pub const IEEE80211_EML_CAP_EML_PADDING_DELAY_256US: c_int = 4;
pub const IEEE80211_EML_CAP_EML_TRANSITION_DELAY: c_uint = 0x0070;
// Described in Table 9-417j in 802.11be-2024
pub const IEEE80211_EML_CAP_EMLSR_TRANSITION_DELAY_0US: c_int = 0;
pub const IEEE80211_EML_CAP_EMLSR_TRANSITION_DELAY_16US: c_int = 1;
pub const IEEE80211_EML_CAP_EMLSR_TRANSITION_DELAY_32US: c_int = 2;
pub const IEEE80211_EML_CAP_EMLSR_TRANSITION_DELAY_64US: c_int = 3;
pub const IEEE80211_EML_CAP_EMLSR_TRANSITION_DELAY_128US: c_int = 4;
pub const IEEE80211_EML_CAP_EMLSR_TRANSITION_DELAY_256US: c_int = 5;
// Described in Table 9-417l in 802.11be-2024
pub const IEEE80211_EML_CAP_EMLMR_TRANSITION_DELAY_0US: c_int = 0;
pub const IEEE80211_EML_CAP_EMLMR_TRANSITION_DELAY_32US: c_int = 1;
pub const IEEE80211_EML_CAP_EMLMR_TRANSITION_DELAY_64US: c_int = 2;
pub const IEEE80211_EML_CAP_EMLMR_TRANSITION_DELAY_128US: c_int = 3;
pub const IEEE80211_EML_CAP_EMLMR_TRANSITION_DELAY_256US: c_int = 4;
pub const IEEE80211_EML_CAP_EMLMR_SUPPORT: c_uint = 0x0080;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT: c_uint = 0x7800;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_0: c_int = 0;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_128US: c_int = 1;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_256US: c_int = 2;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_512US: c_int = 3;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_1TU: c_int = 4;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_2TU: c_int = 5;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_4TU: c_int = 6;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_8TU: c_int = 7;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_16TU: c_int = 8;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_32TU: c_int = 9;
pub const IEEE80211_EML_CAP_TRANSITION_TIMEOUT_64TU: c_int = 10;
pub const IEEE80211_MLD_CAP_OP_MAX_SIMUL_LINKS: c_uint = 0x000f;
pub const IEEE80211_MLD_CAP_OP_SRS_SUPPORT: c_uint = 0x0010;
pub const IEEE80211_MLD_CAP_OP_TID_TO_LINK_MAP_NEG_SUPP: c_uint = 0x0060;
pub const IEEE80211_MLD_CAP_OP_TID_TO_LINK_MAP_NEG_NO_SUPP: c_int = 0;
pub const IEEE80211_MLD_CAP_OP_TID_TO_LINK_MAP_NEG_SUPP_SAME: c_int = 1;
pub const IEEE80211_MLD_CAP_OP_TID_TO_LINK_MAP_NEG_RESERVED: c_int = 2;
pub const IEEE80211_MLD_CAP_OP_TID_TO_LINK_MAP_NEG_SUPP_DIFF: c_int = 3;
pub const IEEE80211_MLD_CAP_OP_FREQ_SEP_TYPE_IND: c_uint = 0x0f80;
pub const IEEE80211_MLD_CAP_OP_AAR_SUPPORT: c_uint = 0x1000;
pub const IEEE80211_MLD_CAP_OP_LINK_RECONF_SUPPORT: c_uint = 0x2000;
pub const IEEE80211_MLD_CAP_OP_ALIGNED_TWT_SUPPORT: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mle_basic_common_info {
    pub len: u8,
    pub mld_mac_addr: [u8; ETH_ALEN],
    pub variable: [u8; ],
    pub __packed: },
pub const IEEE80211_MLC_PREQ_PRES_MLD_ID: c_uint = 0x0010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mle_preq_common_info {
    pub len: u8,
    pub variable: [u8; ],
    pub __packed: },
pub const IEEE80211_MLC_RECONF_PRES_MLD_MAC_ADDR: c_uint = 0x0010;
pub const IEEE80211_MLC_RECONF_PRES_EML_CAPA: c_uint = 0x0020;
pub const IEEE80211_MLC_RECONF_PRES_MLD_CAPA_OP: c_uint = 0x0040;
pub const IEEE80211_MLC_RECONF_PRES_EXT_MLD_CAPA_OP: c_uint = 0x0080;
// no fixed fields in RECONF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mle_tdls_common_info {
    pub len: u8,
    pub ap_mld_mac_addr: [u8; ETH_ALEN],
    pub __packed: },
pub const IEEE80211_MLC_PRIO_ACCESS_PRES_AP_MLD_MAC_ADDR: c_uint = 0x0010;

pub const IEEE80211_EML_EMLSR_PAD_DELAY: c_uint = 0x07;
pub const IEEE80211_EML_EMLSR_TRANS_DELAY: c_uint = 0x38;
pub const IEEE80211_EML_EMLMR_RX_MCS_MAP: c_uint = 0xf0;
pub const IEEE80211_EML_EMLMR_TX_MCS_MAP: c_uint = 0x0f;
// no fixed fields in PRIO_ACCESS
//
// ieee80211_mle_common_size - check multi-link element common size
// @data: multi-link element, must already be checked for size using
// ieee80211_mle_size_ok()
// Return: the size of the multi-link element's "common" subfield
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
//
// The length is the first octet pointed by mle->variable so no
// need to add anything
//
    pub 0: return,
    pub mle->variable[0]: *mut *mut return sizeof(mle) +,
//
// ieee80211_mle_get_link_id - returns the link ID
// @data: the basic multi link element
// Return: the link ID, or -1 if not present
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
// common points now at the beginning of ieee80211_mle_basic_common_info
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub -1: return,
    pub common: *mut return,
//
// ieee80211_mle_get_bss_param_ch_cnt - returns the BSS parameter change count
// @data: pointer to the basic multi link element
// Return: the BSS Parameter Change Count field value, or -1 if not present
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
// common points now at the beginning of ieee80211_mle_basic_common_info
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub -1: return,
    pub 1: common +=,
    pub common: *mut return,
//
// ieee80211_mle_get_eml_med_sync_delay - returns the medium sync delay
// @data: pointer to the multi-link element
// Return: the medium synchronization delay field value from the multi-link
// element, or the default value (%IEEE80211_MED_SYNC_DELAY_DEFAULT)
// if not present
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
// common points now at the beginning of ieee80211_mle_basic_common_info
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub IEEE80211_MED_SYNC_DELAY_DEFAULT: return,
    pub 1: common +=,
    pub 1: common +=,
    pub get_unaligned_le16(common): return,
//
// ieee80211_mle_get_eml_cap - returns the EML capability
// @data: pointer to the multi-link element
// Return: the EML capability field value from the multi-link element,
// or 0 if not present
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
// common points now at the beginning of ieee80211_mle_basic_common_info
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub 0: return,
    pub 1: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub get_unaligned_le16(common): return,
//
// ieee80211_mle_get_mld_capa_op - returns the MLD capabilities and operations.
// @data: pointer to the multi-link element
// Return: the MLD capabilities and operations field value from the multi-link
// element, or 0 if not present
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
//
// common points now at the beginning of
// ieee80211_mle_basic_common_info
//
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub 0: return,
    pub 1: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub get_unaligned_le16(common): return,
// Defined in Figure 9-1074t in P802.11be_D7.0
pub const IEEE80211_EHT_ML_EXT_MLD_CAPA_OP_PARAM_UPDATE: c_uint = 0x0001;
pub const IEEE80211_EHT_ML_EXT_MLD_CAPA_OP_RECO_MAX_LINKS_MASK: c_uint = 0x001e;
pub const IEEE80211_EHT_ML_EXT_MLD_CAPA_NSTR_UPDATE: c_uint = 0x0020;
pub const IEEE80211_EHT_ML_EXT_MLD_CAPA_EMLSR_ENA_ON_ONE_LINK: c_uint = 0x0040;
pub const IEEE80211_EHT_ML_EXT_MLD_CAPA_BTM_MLD_RECO_MULTI_AP: c_uint = 0x0080;
// defined by UHR Draft P802.11bn_D1.3 Figure 9-1147
pub const IEEE80211_UHR_ML_EXT_MLD_CAPA_ML_PM: c_uint = 0x0100;
//
// ieee80211_mle_get_ext_mld_capa_op - returns the extended MLD capabilities
// and operations.
// @data: pointer to the multi-link element
// Return: the extended MLD capabilities and operations field value from
// the multi-link element, or 0 if not present
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
//
// common points now at the beginning of
// ieee80211_mle_basic_common_info
//
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub 0: return,
    pub 1: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 1: common +=,
    pub get_unaligned_le16(common): return,
//
// ieee80211_mle_get_enh_crit_upd_info - returns the enhanced critical
// updates information
// @data: pointer to the multi-link element
// Return: the enhanced critical updates information field, or %NULL
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
//
// common points now at the beginning of
// ieee80211_mle_basic_common_info
//
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub NULL: return,
    pub 1: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub )common: *const return (void,
//
// ieee80211_mle_get_mld_id - returns the MLD ID
// @data: pointer to the multi-link element
// Return: The MLD ID in the given multi-link element, or 0 if not present
//
// The element is assumed to be of the correct type (BASIC) and big enough,
// this must be checked using ieee80211_mle_type_ok().
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub le16_to_cpu(mle->control): u16 control =,
    pub mle->variable: *const *const u8 common =,
//
// common points now at the beginning of
// ieee80211_mle_basic_common_info
//
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub 0: return,
    pub 1: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub common: *mut return,
//
// ieee80211_mle_size_ok - validate multi-link element size
// @data: pointer to the element data
// @len: length of the containing element
// Return: whether or not the multi-link element size is OK
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub sizeof(*mle): *mut u8 fixed =,
    pub 0: u8 common =,
    pub common_len: u8,
    pub control: u16,
    pub false: return,
    pub le16_to_cpu(mle->control): control =,
    pub ieee80211_mle_basic_common_info): common += sizeof(struct,
    pub 1: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 1: common +=,
    pub 2: common +=,
    pub 1: common +=,
    pub ieee80211_mle_preq_common_info): common += sizeof(struct,
    pub 1: common +=,
    pub 1: common +=,
    pub ETH_ALEN: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub 2: common +=,
    pub ieee80211_mle_tdls_common_info): common += sizeof(struct,
    pub 1: common = ETH_ALEN +,
// we don't know this type
    pub true: return,
    pub false: return,
    pub mle->variable[0]: common_len =,
    pub fixed: return common_len >= common && common_len <= len -,
//
// ieee80211_mle_type_ok - validate multi-link element type and size
// @data: pointer to the element data
// @type: expected type of the element
// @len: length of the containing element
// Return: whether or not the multi-link element type matches and size is OK
//
    pub )data: *const *const ieee80211_multi_link_elem mle = (void,
    pub control: u16,
    pub false: return,
    pub le16_to_cpu(mle->control): control =,
    pub true: return,
    pub false: return,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_mle_subelems {
    IEEE80211_MLE_SUBELEM_PER_STA_PROFILE		= 0,
    IEEE80211_MLE_SUBELEM_FRAGMENT		        = 254,
}

pub const IEEE80211_MLE_STA_CONTROL_LINK_ID: c_uint = 0x000f;
pub const IEEE80211_MLE_STA_CONTROL_COMPLETE_PROFILE: c_uint = 0x0010;
pub const IEEE80211_MLE_STA_CONTROL_STA_MAC_ADDR_PRESENT: c_uint = 0x0020;
pub const IEEE80211_MLE_STA_CONTROL_BEACON_INT_PRESENT: c_uint = 0x0040;
pub const IEEE80211_MLE_STA_CONTROL_TSF_OFFS_PRESENT: c_uint = 0x0080;
pub const IEEE80211_MLE_STA_CONTROL_DTIM_INFO_PRESENT: c_uint = 0x0100;
pub const IEEE80211_MLE_STA_CONTROL_NSTR_LINK_PAIR_PRESENT: c_uint = 0x0200;
pub const IEEE80211_MLE_STA_CONTROL_NSTR_BITMAP_SIZE: c_uint = 0x0400;
pub const IEEE80211_MLE_STA_CONTROL_BSS_PARAM_CHANGE_CNT_PRESENT: c_uint = 0x0800;
pub const IEEE80211_MLE_STA_CONTROL_ENH_CRIT_UPD_PRESENT: c_uint = 0x1000;
pub const IEEE80211_MLE_STA_CONTROL_AP_CONDUCTED_TX_PWR_PRESENT: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mle_per_sta_profile {
    pub control: __le16,
    pub sta_info_len: u8,
    pub variable: [u8; ],
    pub __packed: },
//
// ieee80211_mle_basic_sta_prof_size_ok - validate basic multi-link element sta
// profile size
// @data: pointer to the sub element data
// @len: length of the containing sub element
// Return: %true if the STA profile is large enough, %false otherwise
//
    pub )data: *const *const ieee80211_mle_per_sta_profile prof = (void,
    pub control: u16,
    pub sizeof(*prof): *mut u8 fixed =,
    pub 1: u8 info_len =,
    pub false: return,
    pub le16_to_cpu(prof->control): control =,
    pub 6: info_len +=,
    pub 2: info_len +=,
    pub 8: info_len +=,
    pub 2: info_len +=,
    pub 2: info_len +=,
    pub 1: info_len +=,
    pub 1: info_len +=,
    pub 1: info_len +=,
    pub 1: info_len +=,
    pub len: fixed + prof->sta_info_len - 1 <=,
//
// ieee80211_mle_basic_sta_prof_bss_param_ch_cnt - get per-STA profile BSS
// parameter change count
// @prof: the per-STA profile, having been checked with
// ieee80211_mle_basic_sta_prof_size_ok() for the correct length
//
// Return: The BSS parameter change count value if present, 0 otherwise.
//
    pub le16_to_cpu(prof->control): u16 control =,
    pub prof->variable: *const *const u8 pos =,
    pub 0: return,
    pub 6: pos +=,
    pub 2: pos +=,
    pub 8: pos +=,
    pub 2: pos +=,
    pub 2: pos +=,
    pub 1: pos +=,
    pub pos: *mut return,
//
// ieee80211_mle_basic_sta_prof_enh_crit_upd - get per-STA profile enhanced
// critical updates field
// @prof: the per-STA profile, having been checked with
// ieee80211_mle_basic_sta_prof_size_ok() for the correct length
//
// Return: The enhanced critical updates field if present, %NULL otherwise.
//
    pub le16_to_cpu(prof->control): u16 control =,
    pub prof->variable: *const *const u8 pos =,
    pub NULL: return,
    pub 6: pos +=,
    pub 2: pos +=,
    pub 8: pos +=,
    pub 2: pos +=,
    pub 2: pos +=,
    pub 1: pos +=,
    pub 1: pos +=,
    pub )pos: *const return (void,
pub const IEEE80211_MLE_STA_RECONF_CONTROL_LINK_ID: c_uint = 0x000f;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_COMPLETE_PROFILE: c_uint = 0x0010;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_STA_MAC_ADDR_PRESENT: c_uint = 0x0020;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_AP_REM_TIMER_PRESENT: c_uint = 0x0040;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE: c_uint = 0x0780;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE_AP_REM: c_int = 0;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE_OP_PARAM_UPDATE: c_int = 1;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE_ADD_LINK: c_int = 2;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE_DEL_LINK: c_int = 3;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE_NSTR_STATUS: c_int = 4;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE_UHR_OMP_UPD: c_int = 5;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_PARAMS_PRESENT: c_uint = 0x0800;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_NSTR_BMAP_SIZE: c_uint = 0x1000;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_NSTR_IND_BMAP_PRES: c_uint = 0x2000;
pub const IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_DSO_INFO_PRESENT: c_uint = 0x4000;
//
// ieee80211_mle_reconf_sta_prof_size_ok - validate reconfiguration multi-link
// element sta profile size.
// @data: pointer to the sub element data
// @len: length of the containing sub element
// Return: %true if the STA profile is large enough, %false otherwise
//
    pub )data: *const *const ieee80211_mle_per_sta_profile prof = (void,
    pub control: u16,
    pub sizeof(*prof): *mut u8 fixed =,
    pub 1: u8 info_len =,
    pub false: return,
    pub le16_to_cpu(prof->control): control =,
    pub ETH_ALEN: info_len +=,
    pub 2: info_len +=,
    pub 2: info_len +=,
    pub len: fixed + prof->sta_info_len - 1 <=,
pub const IEEE80211_MLE_STA_EPCS_CONTROL_LINK_ID: c_uint = 0x000f;
pub const IEEE80211_EPCS_ENA_RESP_BODY_LEN: c_int = 3;
    pub )data: *const *const ieee80211_ttlm_elem t2l = (void,
    pub 0: *mut *mut u8 control, fixed = sizeof(t2l), elem_len =,
    pub false: return,
    pub t2l->control: control =,
    pub 2: elem_len +=,
    pub 3: elem_len +=,
    pub bm_size: u8,
    pub 1: elem_len +=,
    pub false: return,
    pub 1: bm_size =,
    pub 2: bm_size =,
    pub bm_size: *mut *mut elem_len += hweight8(t2l->optional[0]),
    pub elem_len: return len >= fixed +,
//
// ieee80211_emlsr_pad_delay_in_us - Fetch the EMLSR Padding delay
// in microseconds
// @eml_cap: EML capabilities field value from common info field of
// the Multi-link element
// Return: the EMLSR Padding delay (in microseconds) encoded in the
// EML Capabilities field
//
    pub IEEE80211_EML_CAP_EMLSR_SUPP): u16_get_bits(eml_cap,,
    pub 0: return,
// IEEE Std 802.11be-2024 Table 9-417i—Encoding of the EMLSR
// Padding Delay subfield.
//
    pub 0: return,
    pub 1)): *mut *mut return 32  (1 << (pad_delay -,
//
// ieee80211_emlsr_trans_delay_in_us - Fetch the EMLSR Transition
// delay in microseconds
// @eml_cap: EML capabilities field value from common info field of
// the Multi-link element
// Return: the EMLSR Transition delay (in microseconds) encoded in the
// EML Capabilities field
//
    pub IEEE80211_EML_CAP_EMLSR_SUPP): u16_get_bits(eml_cap,,
    pub 0: return,
// IEEE Std 802.11be-2024 Table 9-417j—Encoding of the EMLSR
// Transition Delay subfield.
//
// invalid values also just use 0
    pub 0: return,
    pub 1)): *mut *mut return 16  (1 << (trans_delay -,
//
// ieee80211_emlmr_pad_delay_in_us - Fetch the EMLMR Padding delay
// in microseconds
// @eml_cap: EML capabilities field value from common info field of
// the Multi-link element
// Return: the EMLMR Padding delay (in microseconds) encoded in the
// EML Capabilities field
//
    pub IEEE80211_EML_CAP_EMLMR_SUPPORT): u16_get_bits(eml_cap,,
    pub 0: return,
// IEEE Std 802.11be-2024 Table 9-417k—Encoding of the EMLMR
// Padding Delay subfield.
//
    pub 0: return,
    pub 1)): *mut *mut return 32  (1 << (pad_delay -,
//
// ieee80211_emlmr_trans_delay_in_us - Fetch the EMLMR Transition
// delay in microseconds
// @eml_cap: EML capabilities field value from common info field of
// the Multi-link element
// Return: the EMLMR Transition delay (in microseconds) encoded in the
// EML Capabilities field
//
    pub IEEE80211_EML_CAP_EMLMR_SUPPORT): u16_get_bits(eml_cap,,
    pub 0: return,
// IEEE Std 802.11be-2024 Table 9-417l—Encoding of the EMLMR
// Transition Delay subfield.
//
// invalid values also just use 0
    pub 0: return,
    pub 1)): *mut *mut return 32  (1 << (trans_delay -,
//
// ieee80211_eml_trans_timeout_in_us - Fetch the EML Transition
// timeout value in microseconds
// @eml_cap: EML capabilities field value from common info field of
// the Multi-link element
// Return: the EML Transition timeout (in microseconds) encoded in
// the EML Capabilities field
//
// IEEE Std 802.11be-2024 Table 9-417m—Encoding of the
// Transition Timeout subfield.
//
// invalid values also just use 0
    pub 0: return,
    pub 1)): *mut *mut return 128  (1 << (timeout -,

