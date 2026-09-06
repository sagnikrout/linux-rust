//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-he.h
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
// IEEE 802.11 HE definitions
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_twt_setup_cmd {
    TWT_SETUP_CMD_REQUEST,
    TWT_SETUP_CMD_SUGGEST,
    TWT_SETUP_CMD_DEMAND,
    TWT_SETUP_CMD_GROUPING,
    TWT_SETUP_CMD_ACCEPT,
    TWT_SETUP_CMD_ALTERNATE,
    TWT_SETUP_CMD_DICTATE,
    TWT_SETUP_CMD_REJECT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_twt_params {
    pub req_type: __le16,
    pub twt: __le64,
    pub min_twt_dur: u8,
    pub mantissa: __le16,
    pub channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_twt_setup {
    pub dialog_token: u8,
    pub element_id: u8,
    pub length: u8,
    pub control: u8,
    pub params: [u8; ],
    pub __packed: },
//
// struct ieee80211_he_cap_elem - HE capabilities element
// @mac_cap_info: HE MAC Capabilities Information
// @phy_cap_info: HE PHY Capabilities Information
//
// This structure represents the fixed fields of the payload of the
// "HE capabilities element" as described in IEEE Std 802.11ax-2021
// sections 9.4.2.248.2 and 9.4.2.248.3.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_cap_elem {
    pub mac_cap_info: [u8; 6],
    pub phy_cap_info: [u8; 11],
    pub __packed: },
pub const IEEE80211_TX_RX_MCS_NSS_DESC_MAX_LEN: c_int = 5;
//
// enum ieee80211_he_mcs_support - HE MCS support definitions
// @IEEE80211_HE_MCS_SUPPORT_0_7: MCSes 0-7 are supported for the
// number of streams
// @IEEE80211_HE_MCS_SUPPORT_0_9: MCSes 0-9 are supported
// @IEEE80211_HE_MCS_SUPPORT_0_11: MCSes 0-11 are supported
// @IEEE80211_HE_MCS_NOT_SUPPORTED: This number of streams isn't supported
//
// These definitions are used in each 2-bit subfield of the rx_mcs_
// and tx_mcs_* fields of &struct ieee80211_he_mcs_nss_supp, which are
// both split into 8 subfields by number of streams. These values indicate
// which MCSes are supported for the number of streams the value appears
// for.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_he_mcs_support {
    IEEE80211_HE_MCS_SUPPORT_0_7	= 0,
    IEEE80211_HE_MCS_SUPPORT_0_9	= 1,
    IEEE80211_HE_MCS_SUPPORT_0_11	= 2,
    IEEE80211_HE_MCS_NOT_SUPPORTED	= 3,
}

//
// struct ieee80211_he_mcs_nss_supp - HE Tx/Rx HE MCS NSS Support Field
//
// This structure holds the data required for the Tx/Rx HE MCS NSS Support Field
// described in P802.11ax_D2.0 section 9.4.2.237.4
//
// @rx_mcs_80: Rx MCS map 2 bits for each stream, total 8 streams, for channel
// widths less than 80MHz.
// @tx_mcs_80: Tx MCS map 2 bits for each stream, total 8 streams, for channel
// widths less than 80MHz.
// @rx_mcs_160: Rx MCS map 2 bits for each stream, total 8 streams, for channel
// width 160MHz.
// @tx_mcs_160: Tx MCS map 2 bits for each stream, total 8 streams, for channel
// width 160MHz.
// @rx_mcs_80p80: Rx MCS map 2 bits for each stream, total 8 streams, for
// channel width 80p80MHz.
// @tx_mcs_80p80: Tx MCS map 2 bits for each stream, total 8 streams, for
// channel width 80p80MHz.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_mcs_nss_supp {
    pub rx_mcs_80: __le16,
    pub tx_mcs_80: __le16,
    pub rx_mcs_160: __le16,
    pub tx_mcs_160: __le16,
    pub rx_mcs_80p80: __le16,
    pub tx_mcs_80p80: __le16,
    pub __packed: },
//
// struct ieee80211_he_operation - HE Operation element
// @he_oper_params: HE Operation Parameters + BSS Color Information
// @he_mcs_nss_set: Basic HE-MCS And NSS Set
// @optional: Optional fields VHT Operation Information, Max Co-Hosted
// BSSID Indicator, and 6 GHz Operation Information
//
// This structure represents the payload of the "HE Operation
// element" as described in IEEE Std 802.11ax-2021 section 9.4.2.249.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_operation {
    pub he_oper_params: __le32,
    pub he_mcs_nss_set: __le16,
    pub optional: [u8; ],
    pub __packed: },
//
// struct ieee80211_he_spr - Spatial Reuse Parameter Set element
// @he_sr_control: SR Control
// @optional: Optional fields Non-SRG OBSS PD Max Offset, SRG OBSS PD
// Min Offset, SRG OBSS PD Max Offset, SRG BSS Color
// Bitmap, and SRG Partial BSSID Bitmap
//
// This structure represents the payload of the "Spatial Reuse
// Parameter Set element" as described in IEEE Std 802.11ax-2021
// section 9.4.2.252.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_spr {
    pub he_sr_control: u8,
    pub optional: [u8; ],
    pub __packed: },
//
// struct ieee80211_he_mu_edca_param_ac_rec - MU AC Parameter Record field
// @aifsn: ACI/AIFSN
// @ecw_min_max: ECWmin/ECWmax
// @mu_edca_timer: MU EDCA Timer
//
// This structure represents the "MU AC Parameter Record" as described
// in IEEE Std 802.11ax-2021 section 9.4.2.251, Figure 9-788p.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_mu_edca_param_ac_rec {
    pub aifsn: u8,
    pub ecw_min_max: u8,
    pub mu_edca_timer: u8,
    pub __packed: },
//
// struct ieee80211_mu_edca_param_set - MU EDCA Parameter Set element
// @mu_qos_info: QoS Info
// @ac_be: MU AC_BE Parameter Record
// @ac_bk: MU AC_BK Parameter Record
// @ac_vi: MU AC_VI Parameter Record
// @ac_vo: MU AC_VO Parameter Record
//
// This structure represents the payload of the "MU EDCA Parameter Set
// element" as described in IEEE Std 802.11ax-2021 section 9.4.2.251.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mu_edca_param_set {
    pub mu_qos_info: u8,
    pub ac_be: ieee80211_he_mu_edca_param_ac_rec,
    pub ac_bk: ieee80211_he_mu_edca_param_ac_rec,
    pub ac_vi: ieee80211_he_mu_edca_param_ac_rec,
    pub ac_vo: ieee80211_he_mu_edca_param_ac_rec,
    pub __packed: },
// 802.11ax HE MAC capabilities
pub const IEEE80211_HE_MAC_CAP0_HTC_HE: c_uint = 0x01;
pub const IEEE80211_HE_MAC_CAP0_TWT_REQ: c_uint = 0x02;
pub const IEEE80211_HE_MAC_CAP0_TWT_RES: c_uint = 0x04;
pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_NOT_SUPP: c_uint = 0x00;
pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_LEVEL_1: c_uint = 0x08;
pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_LEVEL_2: c_uint = 0x10;
pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_LEVEL_3: c_uint = 0x18;
pub const IEEE80211_HE_MAC_CAP0_DYNAMIC_FRAG_MASK: c_uint = 0x18;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_1: c_uint = 0x00;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_2: c_uint = 0x20;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_4: c_uint = 0x40;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_8: c_uint = 0x60;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_16: c_uint = 0x80;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_32: c_uint = 0xa0;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_64: c_uint = 0xc0;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_UNLIMITED: c_uint = 0xe0;
pub const IEEE80211_HE_MAC_CAP0_MAX_NUM_FRAG_MSDU_MASK: c_uint = 0xe0;
pub const IEEE80211_HE_MAC_CAP1_MIN_FRAG_SIZE_UNLIMITED: c_uint = 0x00;
pub const IEEE80211_HE_MAC_CAP1_MIN_FRAG_SIZE_128: c_uint = 0x01;
pub const IEEE80211_HE_MAC_CAP1_MIN_FRAG_SIZE_256: c_uint = 0x02;
pub const IEEE80211_HE_MAC_CAP1_MIN_FRAG_SIZE_512: c_uint = 0x03;
pub const IEEE80211_HE_MAC_CAP1_MIN_FRAG_SIZE_MASK: c_uint = 0x03;
pub const IEEE80211_HE_MAC_CAP1_TF_MAC_PAD_DUR_0US: c_uint = 0x00;
pub const IEEE80211_HE_MAC_CAP1_TF_MAC_PAD_DUR_8US: c_uint = 0x04;
pub const IEEE80211_HE_MAC_CAP1_TF_MAC_PAD_DUR_16US: c_uint = 0x08;
pub const IEEE80211_HE_MAC_CAP1_TF_MAC_PAD_DUR_MASK: c_uint = 0x0c;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_1: c_uint = 0x00;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_2: c_uint = 0x10;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_3: c_uint = 0x20;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_4: c_uint = 0x30;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_5: c_uint = 0x40;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_6: c_uint = 0x50;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_7: c_uint = 0x60;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_8: c_uint = 0x70;
pub const IEEE80211_HE_MAC_CAP1_MULTI_TID_AGG_RX_QOS_MASK: c_uint = 0x70;
// Link adaptation is split between byte HE_MAC_CAP1 and
// HE_MAC_CAP2. It should be set only if IEEE80211_HE_MAC_CAP0_HTC_HE
// in which case the following values apply:
// 0 = No feedback.
// 1 = reserved.
// 2 = Unsolicited feedback.
// 3 = both
//
pub const IEEE80211_HE_MAC_CAP1_LINK_ADAPTATION: c_uint = 0x80;
pub const IEEE80211_HE_MAC_CAP2_LINK_ADAPTATION: c_uint = 0x01;
pub const IEEE80211_HE_MAC_CAP2_ALL_ACK: c_uint = 0x02;
pub const IEEE80211_HE_MAC_CAP2_TRS: c_uint = 0x04;
pub const IEEE80211_HE_MAC_CAP2_BSR: c_uint = 0x08;
pub const IEEE80211_HE_MAC_CAP2_BCAST_TWT: c_uint = 0x10;
pub const IEEE80211_HE_MAC_CAP2_32BIT_BA_BITMAP: c_uint = 0x20;
pub const IEEE80211_HE_MAC_CAP2_MU_CASCADING: c_uint = 0x40;
pub const IEEE80211_HE_MAC_CAP2_ACK_EN: c_uint = 0x80;
pub const IEEE80211_HE_MAC_CAP3_OMI_CONTROL: c_uint = 0x02;
pub const IEEE80211_HE_MAC_CAP3_OFDMA_RA: c_uint = 0x04;
// The maximum length of an A-MDPU is defined by the combination of the Maximum
// A-MDPU Length Exponent field in the HT capabilities, VHT capabilities and the
// same field in the HE capabilities.
//
pub const IEEE80211_HE_MAC_CAP3_MAX_AMPDU_LEN_EXP_EXT_0: c_uint = 0x00;
pub const IEEE80211_HE_MAC_CAP3_MAX_AMPDU_LEN_EXP_EXT_1: c_uint = 0x08;
pub const IEEE80211_HE_MAC_CAP3_MAX_AMPDU_LEN_EXP_EXT_2: c_uint = 0x10;
pub const IEEE80211_HE_MAC_CAP3_MAX_AMPDU_LEN_EXP_EXT_3: c_uint = 0x18;
pub const IEEE80211_HE_MAC_CAP3_MAX_AMPDU_LEN_EXP_MASK: c_uint = 0x18;
pub const IEEE80211_HE_MAC_CAP3_AMSDU_FRAG: c_uint = 0x20;
pub const IEEE80211_HE_MAC_CAP3_FLEX_TWT_SCHED: c_uint = 0x40;
pub const IEEE80211_HE_MAC_CAP3_RX_CTRL_FRAME_TO_MULTIBSS: c_uint = 0x80;
pub const IEEE80211_HE_MAC_CAP4_BSRP_BQRP_A_MPDU_AGG: c_uint = 0x01;
pub const IEEE80211_HE_MAC_CAP4_QTP: c_uint = 0x02;
pub const IEEE80211_HE_MAC_CAP4_BQR: c_uint = 0x04;
pub const IEEE80211_HE_MAC_CAP4_PSR_RESP: c_uint = 0x08;
pub const IEEE80211_HE_MAC_CAP4_NDP_FB_REP: c_uint = 0x10;
pub const IEEE80211_HE_MAC_CAP4_OPS: c_uint = 0x20;
pub const IEEE80211_HE_MAC_CAP4_AMSDU_IN_AMPDU: c_uint = 0x40;
// Multi TID agg TX is split between byte #4 and #5
// The value is a combination of B39,B40,B41
//
pub const IEEE80211_HE_MAC_CAP4_MULTI_TID_AGG_TX_QOS_B39: c_uint = 0x80;
pub const IEEE80211_HE_MAC_CAP5_MULTI_TID_AGG_TX_QOS_B40: c_uint = 0x01;
pub const IEEE80211_HE_MAC_CAP5_MULTI_TID_AGG_TX_QOS_B41: c_uint = 0x02;
pub const IEEE80211_HE_MAC_CAP5_SUBCHAN_SELECTIVE_TRANSMISSION: c_uint = 0x04;
pub const IEEE80211_HE_MAC_CAP5_UL_2x996_TONE_RU: c_uint = 0x08;
pub const IEEE80211_HE_MAC_CAP5_OM_CTRL_UL_MU_DATA_DIS_RX: c_uint = 0x10;
pub const IEEE80211_HE_MAC_CAP5_HE_DYNAMIC_SM_PS: c_uint = 0x20;
pub const IEEE80211_HE_MAC_CAP5_PUNCTURED_SOUNDING: c_uint = 0x40;
pub const IEEE80211_HE_MAC_CAP5_HT_VHT_TRIG_FRAME_RX: c_uint = 0x80;
pub const IEEE80211_HE_VHT_MAX_AMPDU_FACTOR: c_int = 20;
pub const IEEE80211_HE_HT_MAX_AMPDU_FACTOR: c_int = 16;
pub const IEEE80211_HE_6GHZ_MAX_AMPDU_FACTOR: c_int = 13;
// 802.11ax HE PHY capabilities
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_40MHZ_IN_2G: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_40MHZ_80MHZ_IN_5G: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_160MHZ_IN_5G: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_80PLUS80_MHZ_IN_5G: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_MASK_ALL: c_uint = 0x1e;
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_RU_MAPPING_IN_2G: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_RU_MAPPING_IN_5G: c_uint = 0x40;
pub const IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_MASK: c_uint = 0xfe;
pub const IEEE80211_HE_PHY_CAP1_PREAMBLE_PUNC_RX_80MHZ_ONLY_SECOND_20MHZ: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP1_PREAMBLE_PUNC_RX_80MHZ_ONLY_SECOND_40MHZ: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP1_PREAMBLE_PUNC_RX_160MHZ_ONLY_SECOND_20MHZ: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP1_PREAMBLE_PUNC_RX_160MHZ_ONLY_SECOND_40MHZ: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP1_PREAMBLE_PUNC_RX_MASK: c_uint = 0x0f;
pub const IEEE80211_HE_PHY_CAP1_DEVICE_CLASS_A: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP1_LDPC_CODING_IN_PAYLOAD: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP1_HE_LTF_AND_GI_FOR_HE_PPDUS_0_8US: c_uint = 0x40;
// Midamble RX/TX Max NSTS is split between byte #2 and byte #3
pub const IEEE80211_HE_PHY_CAP1_MIDAMBLE_RX_TX_MAX_NSTS: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP2_MIDAMBLE_RX_TX_MAX_NSTS: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP2_NDP_4x_LTF_AND_3_2US: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP2_STBC_TX_UNDER_80MHZ: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP2_STBC_RX_UNDER_80MHZ: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP2_DOPPLER_TX: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP2_DOPPLER_RX: c_uint = 0x20;
// Note that the meaning of UL MU below is different between an AP and a non-AP
// sta, where in the AP case it indicates support for Rx and in the non-AP sta
// case it indicates support for Tx.
//
pub const IEEE80211_HE_PHY_CAP2_UL_MU_FULL_MU_MIMO: c_uint = 0x40;
pub const IEEE80211_HE_PHY_CAP2_UL_MU_PARTIAL_MU_MIMO: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_TX_NO_DCM: c_uint = 0x00;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_TX_BPSK: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_TX_QPSK: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_TX_16_QAM: c_uint = 0x03;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_TX_MASK: c_uint = 0x03;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_TX_NSS_1: c_uint = 0x00;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_TX_NSS_2: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_RX_NO_DCM: c_uint = 0x00;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_RX_BPSK: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_RX_QPSK: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_RX_16_QAM: c_uint = 0x18;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_CONST_RX_MASK: c_uint = 0x18;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_RX_NSS_1: c_uint = 0x00;
pub const IEEE80211_HE_PHY_CAP3_DCM_MAX_RX_NSS_2: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP3_RX_PARTIAL_BW_SU_IN_20MHZ_MU: c_uint = 0x40;
pub const IEEE80211_HE_PHY_CAP3_SU_BEAMFORMER: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP4_SU_BEAMFORMEE: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP4_MU_BEAMFORMER: c_uint = 0x02;
// Minimal allowed value of Max STS under 80MHz is 3
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_UNDER_80MHZ_4: c_uint = 0x0c;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_UNDER_80MHZ_5: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_UNDER_80MHZ_6: c_uint = 0x14;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_UNDER_80MHZ_7: c_uint = 0x18;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_UNDER_80MHZ_8: c_uint = 0x1c;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_UNDER_80MHZ_MASK: c_uint = 0x1c;
// Minimal allowed value of Max STS above 80MHz is 3
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_ABOVE_80MHZ_4: c_uint = 0x60;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_ABOVE_80MHZ_5: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_ABOVE_80MHZ_6: c_uint = 0xa0;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_ABOVE_80MHZ_7: c_uint = 0xc0;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_ABOVE_80MHZ_8: c_uint = 0xe0;
pub const IEEE80211_HE_PHY_CAP4_BEAMFORMEE_MAX_STS_ABOVE_80MHZ_MASK: c_uint = 0xe0;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_1: c_uint = 0x00;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_2: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_3: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_4: c_uint = 0x03;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_5: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_6: c_uint = 0x05;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_7: c_uint = 0x06;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_8: c_uint = 0x07;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_UNDER_80MHZ_MASK: c_uint = 0x07;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_1: c_uint = 0x00;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_2: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_3: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_4: c_uint = 0x18;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_5: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_6: c_uint = 0x28;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_7: c_uint = 0x30;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_8: c_uint = 0x38;
pub const IEEE80211_HE_PHY_CAP5_BEAMFORMEE_NUM_SND_DIM_ABOVE_80MHZ_MASK: c_uint = 0x38;
pub const IEEE80211_HE_PHY_CAP5_NG16_SU_FEEDBACK: c_uint = 0x40;
pub const IEEE80211_HE_PHY_CAP5_NG16_MU_FEEDBACK: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP6_CODEBOOK_SIZE_42_SU: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP6_CODEBOOK_SIZE_75_MU: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP6_TRIG_SU_BEAMFORMING_FB: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP6_TRIG_MU_BEAMFORMING_PARTIAL_BW_FB: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP6_TRIG_CQI_FB: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP6_PARTIAL_BW_EXT_RANGE: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP6_PARTIAL_BANDWIDTH_DL_MUMIMO: c_uint = 0x40;
pub const IEEE80211_HE_PHY_CAP6_PPE_THRESHOLD_PRESENT: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP7_PSR_BASED_SR: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP7_POWER_BOOST_FACTOR_SUPP: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP7_HE_SU_MU_PPDU_4XLTF_AND_08_US_GI: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_1: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_2: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_3: c_uint = 0x18;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_4: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_5: c_uint = 0x28;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_6: c_uint = 0x30;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_7: c_uint = 0x38;
pub const IEEE80211_HE_PHY_CAP7_MAX_NC_MASK: c_uint = 0x38;
pub const IEEE80211_HE_PHY_CAP7_STBC_TX_ABOVE_80MHZ: c_uint = 0x40;
pub const IEEE80211_HE_PHY_CAP7_STBC_RX_ABOVE_80MHZ: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP8_HE_ER_SU_PPDU_4XLTF_AND_08_US_GI: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP8_20MHZ_IN_40MHZ_HE_PPDU_IN_2G: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP8_20MHZ_IN_160MHZ_HE_PPDU: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP8_80MHZ_IN_160MHZ_HE_PPDU: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP8_HE_ER_SU_1XLTF_AND_08_US_GI: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP8_MIDAMBLE_RX_TX_2X_AND_1XLTF: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP8_DCM_MAX_RU_242: c_uint = 0x00;
pub const IEEE80211_HE_PHY_CAP8_DCM_MAX_RU_484: c_uint = 0x40;
pub const IEEE80211_HE_PHY_CAP8_DCM_MAX_RU_996: c_uint = 0x80;
pub const IEEE80211_HE_PHY_CAP8_DCM_MAX_RU_2x996: c_uint = 0xc0;
pub const IEEE80211_HE_PHY_CAP8_DCM_MAX_RU_MASK: c_uint = 0xc0;
pub const IEEE80211_HE_PHY_CAP9_LONGER_THAN_16_SIGB_OFDM_SYM: c_uint = 0x01;
pub const IEEE80211_HE_PHY_CAP9_NON_TRIGGERED_CQI_FEEDBACK: c_uint = 0x02;
pub const IEEE80211_HE_PHY_CAP9_TX_1024_QAM_LESS_THAN_242_TONE_RU: c_uint = 0x04;
pub const IEEE80211_HE_PHY_CAP9_RX_1024_QAM_LESS_THAN_242_TONE_RU: c_uint = 0x08;
pub const IEEE80211_HE_PHY_CAP9_RX_FULL_BW_SU_USING_MU_WITH_COMP_SIGB: c_uint = 0x10;
pub const IEEE80211_HE_PHY_CAP9_RX_FULL_BW_SU_USING_MU_WITH_NON_COMP_SIGB: c_uint = 0x20;
pub const IEEE80211_HE_PHY_CAP9_NOMINAL_PKT_PADDING_0US: c_uint = 0x0;
pub const IEEE80211_HE_PHY_CAP9_NOMINAL_PKT_PADDING_8US: c_uint = 0x1;
pub const IEEE80211_HE_PHY_CAP9_NOMINAL_PKT_PADDING_16US: c_uint = 0x2;
pub const IEEE80211_HE_PHY_CAP9_NOMINAL_PKT_PADDING_RESERVED: c_uint = 0x3;
pub const IEEE80211_HE_PHY_CAP9_NOMINAL_PKT_PADDING_POS: c_int = 6;
pub const IEEE80211_HE_PHY_CAP9_NOMINAL_PKT_PADDING_MASK: c_uint = 0xc0;
pub const IEEE80211_HE_PHY_CAP10_HE_MU_M1RU_MAX_LTF: c_uint = 0x01;
// 802.11ax HE TX/RX MCS NSS Support

pub const IEEE80211_TX_RX_MCS_NSS_SUPP_TX_BITMAP_MASK: c_uint = 0x07c0;
pub const IEEE80211_TX_RX_MCS_NSS_SUPP_RX_BITMAP_MASK: c_uint = 0xf800;
// TX/RX HE MCS Support field Highest MCS subfield encoding
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_he_highest_mcs_supported_subfield_enc {
    HIGHEST_MCS_SUPPORTED_MCS7 = 0,
    HIGHEST_MCS_SUPPORTED_MCS8,
    HIGHEST_MCS_SUPPORTED_MCS9,
    HIGHEST_MCS_SUPPORTED_MCS10,
    HIGHEST_MCS_SUPPORTED_MCS11,
}

// Calculate 802.11ax HE capabilities IE Tx/Rx HE MCS NSS Support Field size
    pub 4: u8 count =,
    pub 4: count +=,
    pub 4: count +=,
    pub count: return,
// 802.11ax HE PPE Thresholds

pub const IEEE80211_PPE_THRES_RU_INDEX_BITMASK_MASK: c_uint = 0x78;

//
// Calculate 802.11ax HE capabilities IE PPE field size
// Input: Header byte of ppe_thres (first byte), and HE capa IE's PHY cap u8
//
    pub n: u8,
    pub 0: return,
//
// Each pair is 6 bits, and we need to add the 7 "header" bits to the
// total size.
//
    pub 7: *mut *mut *mut n = (n  IEEE80211_PPE_THRES_INFO_PPET_SIZE  2) +,
    pub 8): n = DIV_ROUND_UP(n,,
    pub n: return,
    pub )data: *const *const ieee80211_he_cap_elem he_cap_ie_elem = (void,
    pub sizeof(*he_cap_ie_elem): *mut u8 needed =,
    pub false: return,
    pub ieee80211_he_mcs_nss_size(he_cap_ie_elem): needed +=,
    pub false: return,
    pub false: return,
    pub needed: return len >=,
// HE Operation defines
pub const IEEE80211_HE_OPERATION_DFLT_PE_DURATION_MASK: c_uint = 0x00000007;
pub const IEEE80211_HE_OPERATION_TWT_REQUIRED: c_uint = 0x00000008;
pub const IEEE80211_HE_OPERATION_RTS_THRESHOLD_MASK: c_uint = 0x00003ff0;
pub const IEEE80211_HE_OPERATION_RTS_THRESHOLD_OFFSET: c_int = 4;
pub const IEEE80211_HE_OPERATION_VHT_OPER_INFO: c_uint = 0x00004000;
pub const IEEE80211_HE_OPERATION_CO_HOSTED_BSS: c_uint = 0x00008000;
pub const IEEE80211_HE_OPERATION_ER_SU_DISABLE: c_uint = 0x00010000;
pub const IEEE80211_HE_OPERATION_6GHZ_OP_INFO: c_uint = 0x00020000;
pub const IEEE80211_HE_OPERATION_BSS_COLOR_MASK: c_uint = 0x3f000000;
pub const IEEE80211_HE_OPERATION_BSS_COLOR_OFFSET: c_int = 24;
pub const IEEE80211_HE_OPERATION_PARTIAL_BSS_COLOR: c_uint = 0x40000000;
pub const IEEE80211_HE_OPERATION_BSS_COLOR_DISABLED: c_uint = 0x80000000;
pub const IEEE80211_6GHZ_CTRL_REG_LPI_AP: c_int = 0;
pub const IEEE80211_6GHZ_CTRL_REG_SP_AP: c_int = 1;
pub const IEEE80211_6GHZ_CTRL_REG_VLP_AP: c_int = 2;
pub const IEEE80211_6GHZ_CTRL_REG_INDOOR_LPI_AP: c_int = 3;
pub const IEEE80211_6GHZ_CTRL_REG_INDOOR_SP_AP_OLD: c_int = 4;
pub const IEEE80211_6GHZ_CTRL_REG_AP_ROLE_NOT_RELEVANT: c_int = 7;
pub const IEEE80211_6GHZ_CTRL_REG_INDOOR_SP_AP: c_int = 8;
//
// struct ieee80211_he_6ghz_oper - HE 6 GHz operation Information field
// @primary: primary channel
// @control: control flags
// @ccfs0: channel center frequency segment 0
// @ccfs1: channel center frequency segment 1
// @minrate: minimum rate (in 1 Mbps units)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_6ghz_oper {
    pub primary: u8,
pub const IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH: c_uint = 0x3;
pub const IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_20MHZ: c_int = 0;
pub const IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_40MHZ: c_int = 1;
pub const IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_80MHZ: c_int = 2;
pub const IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_160MHZ: c_int = 3;
pub const IEEE80211_HE_6GHZ_OPER_CTRL_DUP_BEACON: c_uint = 0x4;
pub const IEEE80211_HE_6GHZ_OPER_CTRL_REG_INFO: c_uint = 0x78;
    pub control: u8,
    pub ccfs0: u8,
    pub ccfs1: u8,
    pub minrate: u8,
    pub __packed: },
//
// enum ieee80211_reg_conn_bits - represents Regulatory connectivity field bits.
//
// This enumeration defines bit flags used to represent regulatory connectivity
// field bits.
//
// @IEEE80211_REG_CONN_LPI_VALID: Indicates whether the LPI bit is valid.
// @IEEE80211_REG_CONN_LPI_VALUE: Represents the value of the LPI bit.
// @IEEE80211_REG_CONN_SP_VALID: Indicates whether the SP bit is valid.
// @IEEE80211_REG_CONN_SP_VALUE: Represents the value of the SP bit.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_reg_conn_bits {
    IEEE80211_REG_CONN_LPI_VALID = BIT(0),
    IEEE80211_REG_CONN_LPI_VALUE = BIT(1),
    IEEE80211_REG_CONN_SP_VALID = BIT(2),
    IEEE80211_REG_CONN_SP_VALUE = BIT(3),
}

// transmit power interpretation type of transmit power envelope element
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_tx_power_intrpt_type {
    IEEE80211_TPE_LOCAL_EIRP,
    IEEE80211_TPE_LOCAL_EIRP_PSD,
    IEEE80211_TPE_REG_CLIENT_EIRP,
    IEEE80211_TPE_REG_CLIENT_EIRP_PSD,
}

// category type of transmit power envelope element
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_tx_power_category_6ghz {
    IEEE80211_TPE_CAT_6GHZ_DEFAULT = 0,
    IEEE80211_TPE_CAT_6GHZ_SUBORDINATE = 1,
}

//
// For IEEE80211_TPE_LOCAL_EIRP / IEEE80211_TPE_REG_CLIENT_EIRP,
// setting to 63.5 dBm means no constraint.
//
pub const IEEE80211_TPE_MAX_TX_PWR_NO_CONSTRAINT: c_int = 127;
//
// For IEEE80211_TPE_LOCAL_EIRP_PSD / IEEE80211_TPE_REG_CLIENT_EIRP_PSD,
// setting to 127 indicates no PSD limit for the 20 MHz channel.
//
pub const IEEE80211_TPE_PSD_NO_LIMIT: c_int = 127;
//
// struct ieee80211_tx_pwr_env - Transmit Power Envelope
// @info: Transmit Power Information field
// @variable: Maximum Transmit Power field
//
// This structure represents the payload of the "Transmit Power
// Envelope element" as described in IEEE Std 802.11ax-2021 section
// 9.4.2.161
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tx_pwr_env {
    pub info: u8,
    pub variable: [u8; ],
    pub __packed: },
pub const IEEE80211_TX_PWR_ENV_INFO_COUNT: c_uint = 0x7;
pub const IEEE80211_TX_PWR_ENV_INFO_INTERPRET: c_uint = 0x38;
pub const IEEE80211_TX_PWR_ENV_INFO_CATEGORY: c_uint = 0xC0;
pub const IEEE80211_TX_PWR_ENV_EXT_COUNT: c_uint = 0xF;
    pub )data: *const *const ieee80211_tx_pwr_env env = (void,
    pub category: u8 count, interpret,,
    pub sizeof(*env): *mut u8 needed =,
    pub /: *mut *mut u8 N; / also called N in the spec,
    pub false: return,
    pub IEEE80211_TX_PWR_ENV_INFO_COUNT): count = u8_get_bits(env->info,,
    pub IEEE80211_TX_PWR_ENV_INFO_INTERPRET): interpret = u8_get_bits(env->info,,
    pub IEEE80211_TX_PWR_ENV_INFO_CATEGORY): category = u8_get_bits(env->info,,
    pub false: return,
    pub false: return,
// count == 0 encodes 1 value for 20 MHz, etc.
    pub 1: needed += count +,
    pub false: return,
// there can be extension fields not accounted for in 'count'
    pub true: return,
    pub false: return,
    pub 1: N = count ? 1 << (count - 1) :,
    pub N: needed +=,
    pub false: return,
    pub K: needed += 1 +,
    pub false: return,
    pub true: return,
    pub false: return,
//
// ieee80211_he_oper_size - calculate 802.11ax HE Operations IE size
// @he_oper_ie: byte data of the He Operations IE, stating from the byte
// after the ext ID byte. It is assumed that he_oper_ie has at least
// sizeof(struct ieee80211_he_operation) bytes, the caller must have
// validated this.
// @return the actual size of the IE data (not including header), or 0 on error
//
    pub )he_oper_ie: *const *const ieee80211_he_operation he_oper = (void,
    pub ieee80211_he_operation): u8 oper_len = sizeof(struct,
    pub he_oper_params: u32,
// Make sure the input is not NULL
    pub 0: return,
// Calc required length
    pub le32_to_cpu(he_oper->he_oper_params): he_oper_params =,
    pub 3: oper_len +=,
    pub ieee80211_he_6ghz_oper): oper_len += sizeof(struct,
// Add the first byte (extension ID) to the total length
    pub oper_len: return,
//
// ieee80211_he_6ghz_oper - obtain 6 GHz operation field
// @he_oper: HE operation element (must be pre-validated for size)
// but may be %NULL
//
// Return: a pointer to the 6 GHz operation field, or %NULL
//
    pub ret: *const u8,
    pub he_oper_params: u32,
    pub NULL: return,
    pub )&he_oper->optional: *const ret = (void,
    pub le32_to_cpu(he_oper->he_oper_params): he_oper_params =,
    pub NULL: return,
    pub 3: ret +=,
    pub )ret: *const return (void,
// HE Spatial Reuse defines

//
// ieee80211_he_spr_size - calculate 802.11ax HE Spatial Reuse IE size
// @he_spr_ie: byte data of the He Spatial Reuse IE, stating from the byte
// after the ext ID byte. It is assumed that he_spr_ie has at least
// sizeof(struct ieee80211_he_spr) bytes, the caller must have validated
// this
// @return the actual size of the IE data (not including header), or 0 on error
//
    pub )he_spr_ie: *const *const ieee80211_he_spr he_spr = (void,
    pub ieee80211_he_spr): u8 spr_len = sizeof(struct,
    pub he_spr_params: u8,
// Make sure the input is not NULL
    pub 0: return,
// Calc required length
    pub he_spr->he_sr_control: he_spr_params =,
    pub 18: spr_len +=,
// Add the first byte (extension ID) to the total length
    pub spr_len: return,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_6ghz_capa {
// uses IEEE80211_HE_6GHZ_CAP_* below
    pub capa: __le16,
    pub __packed: },
// HE 6 GHz band capabilities
// uses enum ieee80211_min_mpdu_spacing values
pub const IEEE80211_HE_6GHZ_CAP_MIN_MPDU_START: c_uint = 0x0007;
// uses enum ieee80211_vht_max_ampdu_length_exp values
pub const IEEE80211_HE_6GHZ_CAP_MAX_AMPDU_LEN_EXP: c_uint = 0x0038;
// uses IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_* values
pub const IEEE80211_HE_6GHZ_CAP_MAX_MPDU_LEN: c_uint = 0x00c0;
// WLAN_HT_CAP_SM_PS_* values
pub const IEEE80211_HE_6GHZ_CAP_SM_PS: c_uint = 0x0600;
pub const IEEE80211_HE_6GHZ_CAP_RD_RESPONDER: c_uint = 0x0800;
pub const IEEE80211_HE_6GHZ_CAP_RX_ANTPAT_CONS: c_uint = 0x1000;
pub const IEEE80211_HE_6GHZ_CAP_TX_ANTPAT_CONS: c_uint = 0x2000;
