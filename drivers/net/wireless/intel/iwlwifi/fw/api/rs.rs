//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/rs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2012-2014, 2018-2022, 2024-2026 Intel Corporation
// Copyright (C) 2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_rs_h__

//
// enum iwl_tlc_mng_cfg_flags - options for TLC config flags
// @IWL_TLC_MNG_CFG_FLAGS_STBC_MSK: enable STBC. For HE this enables STBC for
// bandwidths <= 80MHz
// @IWL_TLC_MNG_CFG_FLAGS_LDPC_MSK: enable LDPC
// @IWL_TLC_MNG_CFG_FLAGS_HE_STBC_160MHZ_MSK: enable STBC in HE at 160MHz
// bandwidth
// @IWL_TLC_MNG_CFG_FLAGS_HE_DCM_NSS_1_MSK: enable HE Dual Carrier Modulation
// for BPSK (MCS 0) with 1 spatial
// stream
// @IWL_TLC_MNG_CFG_FLAGS_HE_DCM_NSS_2_MSK: enable HE Dual Carrier Modulation
// for BPSK (MCS 0) with 2 spatial
// streams
// @IWL_TLC_MNG_CFG_FLAGS_EHT_EXTRA_LTF_MSK: enable support for EHT extra LTF
// @IWL_TLC_MNG_CFG_FLAGS_UHR_ELR_1_5_MBPS_MSK: support ELR 1.5 Mbps
// @IWL_TLC_MNG_CFG_FLAGS_UHR_ELR_3_MBPS_MSK: support ELR 3 Mbps
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tlc_mng_cfg_flags {
    IWL_TLC_MNG_CFG_FLAGS_STBC_MSK			= BIT(0),
    IWL_TLC_MNG_CFG_FLAGS_LDPC_MSK			= BIT(1),
    IWL_TLC_MNG_CFG_FLAGS_HE_STBC_160MHZ_MSK	= BIT(2),
    IWL_TLC_MNG_CFG_FLAGS_HE_DCM_NSS_1_MSK		= BIT(3),
    IWL_TLC_MNG_CFG_FLAGS_HE_DCM_NSS_2_MSK		= BIT(4),
    IWL_TLC_MNG_CFG_FLAGS_EHT_EXTRA_LTF_MSK		= BIT(6),
    IWL_TLC_MNG_CFG_FLAGS_UHR_ELR_1_5_MBPS_MSK	= BIT(7),
    IWL_TLC_MNG_CFG_FLAGS_UHR_ELR_3_MBPS_MSK	= BIT(8),
}

//
// enum iwl_tlc_mng_cfg_cw - channel width options
// @IWL_TLC_MNG_CH_WIDTH_20MHZ: 20MHZ channel
// @IWL_TLC_MNG_CH_WIDTH_40MHZ: 40MHZ channel
// @IWL_TLC_MNG_CH_WIDTH_80MHZ: 80MHZ channel
// @IWL_TLC_MNG_CH_WIDTH_160MHZ: 160MHZ channel
// @IWL_TLC_MNG_CH_WIDTH_320MHZ: 320MHZ channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tlc_mng_cfg_cw {
    IWL_TLC_MNG_CH_WIDTH_20MHZ,
    IWL_TLC_MNG_CH_WIDTH_40MHZ,
    IWL_TLC_MNG_CH_WIDTH_80MHZ,
    IWL_TLC_MNG_CH_WIDTH_160MHZ,
    IWL_TLC_MNG_CH_WIDTH_320MHZ,
}

//
// enum iwl_tlc_mng_cfg_chains - possible chains
// @IWL_TLC_MNG_CHAIN_A_MSK: chain A
// @IWL_TLC_MNG_CHAIN_B_MSK: chain B
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tlc_mng_cfg_chains {
    IWL_TLC_MNG_CHAIN_A_MSK = BIT(0),
    IWL_TLC_MNG_CHAIN_B_MSK = BIT(1),
}

//
// enum iwl_tlc_mng_cfg_mode - supported modes
// @IWL_TLC_MNG_MODE_CCK: enable CCK
// @IWL_TLC_MNG_MODE_OFDM_NON_HT: enable OFDM (non HT)
// @IWL_TLC_MNG_MODE_NON_HT: enable non HT
// @IWL_TLC_MNG_MODE_HT: enable HT
// @IWL_TLC_MNG_MODE_VHT: enable VHT
// @IWL_TLC_MNG_MODE_HE: enable HE
// @IWL_TLC_MNG_MODE_EHT: enable EHT
// @IWL_TLC_MNG_MODE_UHR: enable UHR
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tlc_mng_cfg_mode {
    IWL_TLC_MNG_MODE_CCK = 0,
    IWL_TLC_MNG_MODE_OFDM_NON_HT = IWL_TLC_MNG_MODE_CCK,
    IWL_TLC_MNG_MODE_NON_HT = IWL_TLC_MNG_MODE_CCK,
    IWL_TLC_MNG_MODE_HT,
    IWL_TLC_MNG_MODE_VHT,
    IWL_TLC_MNG_MODE_HE,
    IWL_TLC_MNG_MODE_EHT,
    IWL_TLC_MNG_MODE_UHR,
}

//
// enum iwl_tlc_mng_ht_rates - HT/VHT/HE rates
// @IWL_TLC_MNG_HT_RATE_MCS0: index of MCS0
// @IWL_TLC_MNG_HT_RATE_MCS1: index of MCS1
// @IWL_TLC_MNG_HT_RATE_MCS2: index of MCS2
// @IWL_TLC_MNG_HT_RATE_MCS3: index of MCS3
// @IWL_TLC_MNG_HT_RATE_MCS4: index of MCS4
// @IWL_TLC_MNG_HT_RATE_MCS5: index of MCS5
// @IWL_TLC_MNG_HT_RATE_MCS6: index of MCS6
// @IWL_TLC_MNG_HT_RATE_MCS7: index of MCS7
// @IWL_TLC_MNG_HT_RATE_MCS8: index of MCS8
// @IWL_TLC_MNG_HT_RATE_MCS9: index of MCS9
// @IWL_TLC_MNG_HT_RATE_MCS10: index of MCS10
// @IWL_TLC_MNG_HT_RATE_MCS11: index of MCS11
// @IWL_TLC_MNG_HT_RATE_MAX: maximal rate for HT/VHT
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tlc_mng_ht_rates {
    IWL_TLC_MNG_HT_RATE_MCS0 = 0,
    IWL_TLC_MNG_HT_RATE_MCS1,
    IWL_TLC_MNG_HT_RATE_MCS2,
    IWL_TLC_MNG_HT_RATE_MCS3,
    IWL_TLC_MNG_HT_RATE_MCS4,
    IWL_TLC_MNG_HT_RATE_MCS5,
    IWL_TLC_MNG_HT_RATE_MCS6,
    IWL_TLC_MNG_HT_RATE_MCS7,
    IWL_TLC_MNG_HT_RATE_MCS8,
    IWL_TLC_MNG_HT_RATE_MCS9,
    IWL_TLC_MNG_HT_RATE_MCS10,
    IWL_TLC_MNG_HT_RATE_MCS11,
    IWL_TLC_MNG_HT_RATE_MAX = IWL_TLC_MNG_HT_RATE_MCS11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IWL_TLC_MNG_NSS {
    IWL_TLC_NSS_1,
    IWL_TLC_NSS_2,
    IWL_TLC_NSS_MAX
}

//
// enum IWL_TLC_MCS_PER_BW - mcs index per BW
// @IWL_TLC_MCS_PER_BW_80: mcs for bw - 20Hhz, 40Hhz, 80Hhz
// @IWL_TLC_MCS_PER_BW_160: mcs for bw - 160Mhz
// @IWL_TLC_MCS_PER_BW_320: mcs for bw - 320Mhz
// @IWL_TLC_MCS_PER_BW_NUM_V3: number of entries up to version 3
// @IWL_TLC_MCS_PER_BW_NUM_V4: number of entries from version 4
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IWL_TLC_MCS_PER_BW {
    IWL_TLC_MCS_PER_BW_80,
    IWL_TLC_MCS_PER_BW_160,
    IWL_TLC_MCS_PER_BW_320,
    IWL_TLC_MCS_PER_BW_NUM_V3 = IWL_TLC_MCS_PER_BW_160 + 1,
    IWL_TLC_MCS_PER_BW_NUM_V4 = IWL_TLC_MCS_PER_BW_320 + 1,
}

//
// struct iwl_tlc_config_cmd_v3 - TLC configuration
// @sta_id: station id
// @reserved1: reserved
// @max_ch_width: max supported channel width from @enum iwl_tlc_mng_cfg_cw
// @mode: &enum iwl_tlc_mng_cfg_mode
// @chains: bitmask of &enum iwl_tlc_mng_cfg_chains
// @amsdu: TX amsdu is supported
// @flags: bitmask of &enum iwl_tlc_mng_cfg_flags
// @non_ht_rates: bitmap of supported legacy rates
// @ht_rates: bitmap of &enum iwl_tlc_mng_ht_rates, per &enum IWL_TLC_MCS_PER_BW
// <nss, channel-width> pair (0 - 80mhz width and below, 1 - 160mhz).
// @max_mpdu_len: max MPDU length, in bytes
// @sgi_ch_width_supp: bitmap of SGI support per channel width
// use BIT(@enum iwl_tlc_mng_cfg_cw)
// @reserved2: reserved
// @max_tx_op: max TXOP in uSecs for all AC (BK, BE, VO, VI),
// set zero for no limit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tlc_config_cmd_v3 {
    pub sta_id: u8,
    pub reserved1: [u8; 3],
    pub max_ch_width: u8,
    pub mode: u8,
    pub chains: u8,
    pub amsdu: u8,
    pub flags: __le16,
    pub non_ht_rates: __le16,
    pub ht_rates: [__le16; IWL_TLC_NSS_MAX][IWL_TLC_MCS_PER_BW_NUM_V3],
    pub max_mpdu_len: __le16,
    pub sgi_ch_width_supp: u8,
    pub reserved2: u8,
    pub max_tx_op: __le32,
    pub /: *mut *mut } __packed; / TLC_MNG_CONFIG_CMD_API_S_VER_3,
//
// struct iwl_tlc_config_cmd_v4 - TLC configuration
// @sta_id: station id
// @reserved1: reserved
// @max_ch_width: max supported channel width from &enum iwl_tlc_mng_cfg_cw
// @mode: &enum iwl_tlc_mng_cfg_mode
// @chains: bitmask of &enum iwl_tlc_mng_cfg_chains
// @sgi_ch_width_supp: bitmap of SGI support per channel width
// use BIT(&enum iwl_tlc_mng_cfg_cw)
// @flags: bitmask of &enum iwl_tlc_mng_cfg_flags
// @non_ht_rates: bitmap of supported legacy rates
// @ht_rates: bitmap of &enum iwl_tlc_mng_ht_rates, per <nss, channel-width>
// pair (0 - 80mhz width and below, 1 - 160mhz, 2 - 320mhz).
// @max_mpdu_len: max MPDU length, in bytes
// @max_tx_op: max TXOP in uSecs for all AC (BK, BE, VO, VI),
// set zero for no limit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tlc_config_cmd_v4 {
    pub sta_id: u8,
    pub reserved1: [u8; 3],
    pub max_ch_width: u8,
    pub mode: u8,
    pub chains: u8,
    pub sgi_ch_width_supp: u8,
    pub flags: __le16,
    pub non_ht_rates: __le16,
    pub ht_rates: [__le16; IWL_TLC_NSS_MAX][IWL_TLC_MCS_PER_BW_NUM_V4],
    pub max_mpdu_len: __le16,
    pub max_tx_op: __le16,
    pub /: *mut *mut } __packed; / TLC_MNG_CONFIG_CMD_API_S_VER_4,
//
// struct iwl_tlc_config_cmd - TLC configuration
// @sta_mask: station mask (in NAN we can have multiple logical stations of
// the same peer (with the same TLC configuration)).
// @phy_id: the phy id to used for this TLC configuration
// @max_ch_width: max supported channel width from &enum iwl_tlc_mng_cfg_cw
// @mode: &enum iwl_tlc_mng_cfg_mode
// @chains: bitmask of &enum iwl_tlc_mng_cfg_chains
// @sgi_ch_width_supp: bitmap of SGI support per channel width
// use BIT(&enum iwl_tlc_mng_cfg_cw)
// @flags: bitmask of &enum iwl_tlc_mng_cfg_flags
// @non_ht_rates: bitmap of supported legacy rates
// @ht_rates: bitmap of &enum iwl_tlc_mng_ht_rates, per <nss, channel-width>
// pair (0 - 80mhz width and below, 1 - 160mhz, 2 - 320mhz).
// @max_mpdu_len: max MPDU length, in bytes
// @max_tx_op: max TXOP in uSecs for all AC (BK, BE, VO, VI),
// set zero for no limit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tlc_config_cmd {
    pub sta_mask: __le32,
    pub phy_id: __le32,
    pub max_ch_width: u8,
    pub mode: u8,
    pub chains: u8,
    pub sgi_ch_width_supp: u8,
    pub flags: __le16,
    pub non_ht_rates: __le16,
    pub ht_rates: [__le32; IWL_TLC_NSS_MAX][IWL_TLC_MCS_PER_BW_NUM_V4],
    pub max_mpdu_len: __le16,
    pub max_tx_op: __le16,
    pub /: *mut *mut } __packed; / TLC_MNG_CONFIG_CMD_API_S_VER_6,
//
// enum iwl_tlc_update_flags - updated fields
// @IWL_TLC_NOTIF_FLAG_RATE: last initial rate update
// @IWL_TLC_NOTIF_FLAG_AMSDU: umsdu parameters update
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tlc_update_flags {
    IWL_TLC_NOTIF_FLAG_RATE  = BIT(0),
    IWL_TLC_NOTIF_FLAG_AMSDU = BIT(1),
}

//
// struct iwl_tlc_update_notif - TLC notification from FW
// @sta_id: station id
// @reserved: reserved
// @flags: bitmap of notifications reported
// @rate: current initial rate, format depends on the notification
// version
// @amsdu_size: Max AMSDU size, in bytes
// @amsdu_enabled: bitmap for per-TID AMSDU enablement
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tlc_update_notif {
    pub sta_id: u8,
    pub reserved: [u8; 3],
    pub flags: __le32,
    pub rate: __le32,
    pub amsdu_size: __le32,
    pub amsdu_enabled: __le32,
    pub /: *mut *mut } __packed; / TLC_MNG_UPDATE_NTFY_API_S_VER_2, _VER_3, _VER_4,
//
// enum iwl_tlc_debug_types - debug options
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tlc_debug_types {
//
// @IWL_TLC_DEBUG_FIXED_RATE: set fixed rate for rate scaling
//
    IWL_TLC_DEBUG_FIXED_RATE,
//
// @IWL_TLC_DEBUG_AGG_DURATION_LIM: time limit for a BA
// session, in usec
//
    IWL_TLC_DEBUG_AGG_DURATION_LIM,
//
// @IWL_TLC_DEBUG_AGG_FRAME_CNT_LIM: set max number of frames
// in an aggregation
//
    IWL_TLC_DEBUG_AGG_FRAME_CNT_LIM,
//
// @IWL_TLC_DEBUG_TPC_ENABLED: enable or disable tpc
//
    IWL_TLC_DEBUG_TPC_ENABLED,
//
// @IWL_TLC_DEBUG_TPC_STATS: get number of frames Tx'ed in each
// tpc step
//
    IWL_TLC_DEBUG_TPC_STATS,
//
// @IWL_TLC_DEBUG_RTS_DISABLE: disable RTS (bool true/false).
//
    IWL_TLC_DEBUG_RTS_DISABLE,
//
// @IWL_TLC_DEBUG_PARTIAL_FIXED_RATE: set partial fixed rate to fw
//
    IWL_TLC_DEBUG_PARTIAL_FIXED_RATE,
}

pub const MAX_DATA_IN_DHC_TLC_CMD: c_int = 10;
//
// struct iwl_dhc_tlc_cmd - fixed debug config
// @sta_id: bit 0 - enable/disable, bits 1 - 7 hold station id
// @reserved1: reserved
// @type: type id of %enum iwl_tlc_debug_types
// @data: data to send
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dhc_tlc_cmd {
    pub sta_id: u8,
    pub reserved1: [u8; 3],
    pub type: __le32,
    pub data: [__le32; MAX_DATA_IN_DHC_TLC_CMD],
    pub /: *mut *mut } __packed; / TLC_MNG_DEBUG_CMD_S,
pub const IWL_MAX_MCS_DISPLAY_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rate_mcs_info {
    pub mbps: [c_char; IWL_MAX_MCS_DISPLAY_SIZE],
    pub mcs: [c_char; IWL_MAX_MCS_DISPLAY_SIZE],
}

//
// These serve as indexes into
// struct iwl_rate_info fw_rate_idx_to_plcp[IWL_RATE_COUNT];
// TODO: avoid overlap between legacy and HT rates
//

// fw API values for legacy bit rates, both OFDM and CCK
//
// rate_n_flags bit fields version 1
//
// The 32-bit value has different layouts in the low 8 bites depending on the
// format. There are three formats, HT, VHT and legacy (11abg, with subformats
// for CCK and OFDM).
//
// High-throughput (HT) rate format
// bit 8 is 1, bit 26 is 0, bit 9 is 0 (OFDM)
// Very High-throughput (VHT) rate format
// bit 8 is 0, bit 26 is 1, bit 9 is 0 (OFDM)
// Legacy OFDM rate format for bits 7:0
// bit 8 is 0, bit 26 is 0, bit 9 is 0 (OFDM)
// Legacy CCK rate format for bits 7:0:
// bit 8 is 0, bit 26 is 0, bit 9 is 1 (CCK)
//
// Bit 8: (1) HT format, (0) legacy or VHT format
pub const RATE_MCS_HT_POS: c_int = 8;

// Bit 9: (1) CCK, (0) OFDM.  HT (bit 8) must be "0" for this bit to be valid
pub const RATE_MCS_CCK_POS_V1: c_int = 9;

// Bit 26: (1) VHT format, (0) legacy format in bits 8:0
pub const RATE_MCS_VHT_POS_V1: c_int = 26;

//
// High-throughput (HT) rate format for bits 7:0
//
// 2-0:  MCS rate base
// 0)   6 Mbps
// 1)  12 Mbps
// 2)  18 Mbps
// 3)  24 Mbps
// 4)  36 Mbps
// 5)  48 Mbps
// 6)  54 Mbps
// 7)  60 Mbps
// 4-3:  0)  Single stream (SISO)
// 1)  Dual stream (MIMO)
// 2)  Triple stream (MIMO)
// 5:  Value of 0x20 in bits 7:0 indicates 6 Mbps HT40 duplicate data
// (bits 7-6 are zero)
//
// Together the low 5 bits work out to the MCS index because we don't
// support MCSes above 15/23, and 0-7 have one stream, 8-15 have two
// streams and 16-23 have three streams. We could also support MCS 32
// which is the duplicate 20 MHz MCS (bit 5 set, all others zero.)
//
pub const RATE_HT_MCS_RATE_CODE_MSK_V1: c_uint = 0x7;
pub const RATE_HT_MCS_NSS_POS_V1: c_int = 3;

// Bit 10: (1) Use Green Field preamble
pub const RATE_HT_MCS_GF_POS: c_int = 10;

pub const RATE_HT_MCS_INDEX_MSK_V1: c_uint = 0x3f;
//
// Very High-throughput (VHT) rate format for bits 7:0
//
// 3-0:  VHT MCS (0-9)
// 5-4:  number of streams - 1:
// 0)  Single stream (SISO)
// 1)  Dual stream (MIMO)
// 2)  Triple stream (MIMO)
//
// Bit 4-5: (0) SISO, (1) MIMO2 (2) MIMO3
pub const RATE_VHT_MCS_RATE_CODE_MSK: c_uint = 0xf;
pub const RATE_VHT_MCS_NSS_MSK: c_uint = 0x30;
//
// Legacy OFDM rate format for bits 7:0
//
// 3-0:  0xD)   6 Mbps
// 0xF)   9 Mbps
// 0x5)  12 Mbps
// 0x7)  18 Mbps
// 0x9)  24 Mbps
// 0xB)  36 Mbps
// 0x1)  48 Mbps
// 0x3)  54 Mbps
// (bits 7-4 are 0)
//
// Legacy CCK rate format for bits 7:0:
// bit 8 is 0, bit 26 is 0, bit 9 is 1 (CCK):
//
// 6-0:   10)  1 Mbps
// 20)  2 Mbps
// 55)  5.5 Mbps
// 110)  11 Mbps
// (bit 7 is 0)
//
pub const RATE_LEGACY_RATE_MSK_V1: c_uint = 0xff;
// Bit 10 - OFDM HE
pub const RATE_MCS_HE_POS_V1: c_int = 10;

//
// Bit 11-12: (0) 20MHz, (1) 40MHz, (2) 80MHz, (3) 160MHz
// 0 and 1 are valid for HT and VHT, 2 and 3 only for VHT
//
pub const RATE_MCS_CHAN_WIDTH_POS: c_int = 11;

// Bit 13: (1) Short guard interval (0.4 usec), (0) normal GI (0.8 usec)
pub const RATE_MCS_SGI_POS_V1: c_int = 13;

// Bit 14-16: Antenna selection (1) Ant A, (2) Ant B, (4) Ant C
pub const RATE_MCS_ANT_POS: c_int = 14;

// Bit 17: (0) SS, (1) SS*2
pub const RATE_MCS_STBC_POS: c_int = 17;

// Bit 18: OFDM-HE dual carrier mode
pub const RATE_HE_DUAL_CARRIER_MODE: c_int = 18;

// Bit 19: (0) Beamforming is off, (1) Beamforming is on
pub const RATE_MCS_BF_POS: c_int = 19;

//
// Bit 20-21: HE LTF type and guard interval
// HE (ext) SU:
// 0			1xLTF+0.8us
// 1			2xLTF+0.8us
// 2			2xLTF+1.6us
// 3 & SGI (bit 13) clear	4xLTF+3.2us
// 3 & SGI (bit 13) set	4xLTF+0.8us
// HE MU:
// 0			4xLTF+0.8us
// 1			2xLTF+0.8us
// 2			2xLTF+1.6us
// 3			4xLTF+3.2us
// HE-EHT TRIG:
// 0			1xLTF+1.6us
// 1			2xLTF+1.6us
// 2			4xLTF+3.2us
// 3			(does not occur)
// EHT MU:
// 0			2xLTF+0.8us
// 1			2xLTF+1.6us
// 2			4xLTF+0.8us
// 3			4xLTF+3.2us
//
pub const RATE_MCS_HE_GI_LTF_POS: c_int = 20;

// Bit 22-23: HE type. (0) SU, (1) SU_EXT, (2) MU, (3) trigger based
pub const RATE_MCS_HE_TYPE_POS_V1: c_int = 22;

// Bit 24-25: (0) 20MHz (no dup), (1) 2x20MHz, (2) 4x20MHz, 3 8x20MHz
pub const RATE_MCS_DUP_POS_V1: c_int = 24;

// Bit 27: (1) LDPC enabled, (0) LDPC disabled
pub const RATE_MCS_LDPC_POS_V1: c_int = 27;

// Bit 28: (1) 106-tone RX (8 MHz RU), (0) normal bandwidth
pub const RATE_MCS_HE_106T_POS_V1: c_int = 28;

// Bit 30-31: (1) RTS, (2) CTS

// rate_n_flags bit field version 2 and 3
//
// The 32-bit value has different layouts in the low 8 bits depending on the
// format. There are three formats, HT, VHT and legacy (11abg, with subformats
// for CCK and OFDM).
//
// Bits 10-8: rate format
// (0) Legacy CCK (1) Legacy OFDM (2) High-throughput (HT)
// (3) Very High-throughput (VHT) (4) High-efficiency (HE)
// (5) Extremely High-throughput (EHT)
// (6) Ultra High Reliability (UHR) (v3 rate format only)
//
pub const RATE_MCS_MOD_TYPE_POS: c_int = 8;

//
// Legacy CCK rate format for bits 0:3:
//
// (0) 1 Mbps
// (1) 2 Mbps
// (2) 5.5 Mbps
// (3) 11 Mbps
//
// Legacy OFDM rate format for bis 3:0:
//
// (0) 6 Mbps
// (1) 9 Mbps
// (2) 12 Mbps
// (3) 18 Mbps
// (4) 24 Mbps
// (5) 36 Mbps
// (6) 48 Mbps
// (7) 54 Mbps
//
pub const RATE_LEGACY_RATE_MSK: c_uint = 0x7;
//
// HT, VHT, HE, EHT, UHR rate format
// Version 2: (not applicable for UHR)
// 3-0: MCS
// 4: NSS==2 indicator
// Version 3:
// 4-0: MCS
// 5: NSS==2 indicator
//
pub const RATE_HT_MCS_CODE_MSK: c_uint = 0x7;
pub const RATE_MCS_NSS_MSK_V2: c_uint = 0x10;
pub const RATE_MCS_NSS_MSK: c_uint = 0x20;
pub const RATE_MCS_CODE_MSK: c_uint = 0x1f;

// Bits 7-5: reserved
//
// Bits 13-11: (0) 20MHz, (1) 40MHz, (2) 80MHz, (3) 160MHz, (4) 320MHz
//

pub const RATE_MCS_CHAN_WIDTH_20_VAL: c_int = 0;

pub const RATE_MCS_CHAN_WIDTH_40_VAL: c_int = 1;

pub const RATE_MCS_CHAN_WIDTH_80_VAL: c_int = 2;

pub const RATE_MCS_CHAN_WIDTH_160_VAL: c_int = 3;

pub const RATE_MCS_CHAN_WIDTH_320_VAL: c_int = 4;

// Bit 15-14: Antenna selection:
// Bit 14: Ant A active
// Bit 15: Ant B active
//
// All relevant definitions are same as in v1
//
// Bit 16 (1) LDPC enables, (0) LDPC disabled
pub const RATE_MCS_LDPC_POS: c_int = 16;

// Bit 17: (0) SS, (1) SS*2 (same as v1)
// Bit 18: OFDM-HE dual carrier mode (same as v1)
// Bit 19: (0) Beamforming is off, (1) Beamforming is on (same as v1)
//
// Bit 22-20: HE LTF type and guard interval
// CCK:
// 0			long preamble
// 1			short preamble
// HT/VHT:
// 0			0.8us
// 1			0.4us
// HE (ext) SU:
// 0			1xLTF+0.8us
// 1			2xLTF+0.8us
// 2			2xLTF+1.6us
// 3			4xLTF+3.2us
// 4			4xLTF+0.8us
// HE MU:
// 0			4xLTF+0.8us
// 1			2xLTF+0.8us
// 2			2xLTF+1.6us
// 3			4xLTF+3.2us
// HE TRIG:
// 0			1xLTF+1.6us
// 1			2xLTF+1.6us
// 2			4xLTF+3.2us
//

pub const RATE_MCS_HE_SU_4_LTF: c_int = 3;
pub const RATE_MCS_HE_SU_4_LTF_08_GI: c_int = 4;
// Bit 24-23: HE type. (0) SU, (1) HE SU_EXT/UHR ELR, (2) MU, (3) trigger based
pub const RATE_MCS_HE_TYPE_POS: c_int = 23;

// Bit 25: duplicate channel enabled
//
// if this bit is set, duplicate is according to BW (bits 11-13):
//
// CCK:  2x 20MHz
// OFDM Legacy: N x 20Mhz, (N = BW \ 2 , either 2, 4, 8, 16)
// EHT: 2 x BW/2, (80 - 2x40, 160 - 2x80, 320 - 2x160)
//
pub const RATE_MCS_DUP_POS: c_int = 25;

// Bit 26: (1) 106-tone RX (8 MHz RU), (0) normal bandwidth
pub const RATE_MCS_HE_106T_POS: c_int = 26;

// Bit 27: EHT extra LTF:
// instead of 1 LTF for SISO use 2 LTFs,
// instead of 2 LTFs for NSTS=2 use 4 LTFs
pub const RATE_MCS_EHT_EXTRA_LTF_POS: c_int = 27;

// Bit 31-28: reserved
// Link Quality definitions
// # entries in rate scale table to support Tx retries
pub const LQ_MAX_RETRY_NUM: c_int = 16;
// Link quality command flags bit fields
// Bit 0: (0) Don't use RTS (1) Use RTS
pub const LQ_FLAG_USE_RTS_POS: c_int = 0;

// Bit 1-3: LQ command color. Used to match responses to LQ commands
pub const LQ_FLAG_COLOR_POS: c_int = 1;

// Bit 4-5: Tx RTS BW Signalling
// (0) No RTS BW signalling
// (1) Static BW signalling
// (2) Dynamic BW signalling
//
pub const LQ_FLAG_RTS_BW_SIG_POS: c_int = 4;

// Bit 6: (0) No dynamic BW selection (1) Allow dynamic BW selection
// Dyanmic BW selection allows Tx with narrower BW then requested in rates
//
pub const LQ_FLAG_DYNAMIC_BW_POS: c_int = 6;

// Single Stream Tx Parameters (lq_cmd->ss_params)
// Flags to control a smart FW decision about whether BFER/STBC/SISO will be
// used for single stream Tx.
//
// Bit 0-1: Max STBC streams allowed. Can be 0-3.
// (0) - No STBC allowed
// (1) - 2x1 STBC allowed (HT/VHT)
// (2) - 4x2 STBC allowed (HT/VHT)
// (3) - 3x2 STBC allowed (HT only)
// All our chips are at most 2 antennas so only (1) is valid for now.
//
pub const LQ_SS_STBC_ALLOWED_POS: c_int = 0;

// 2x1 STBC is allowed

// Bit 2: Beamformer (VHT only) is allowed
pub const LQ_SS_BFER_ALLOWED_POS: c_int = 2;

// Bit 3: Force BFER or STBC for testing
// If this is set:
// If BFER is allowed then force the ucode to choose BFER else
// If STBC is allowed then force the ucode to choose STBC over SISO
//
pub const LQ_SS_FORCE_POS: c_int = 3;

// Bit 31: ss_params field is valid. Used for FW backward compatibility
// with other drivers which don't support the ss_params API yet
//
pub const LQ_SS_PARAMS_VALID_POS: c_int = 31;

//
// struct iwl_lq_cmd - link quality command
// @sta_id: station to update
// @reduced_tpc: reduced transmit power control value
// @control: not used
// @flags: combination of LQ_FLAG_
// @mimo_delim: the first SISO index in rs_table, which separates MIMO
// and SISO rates
// @single_stream_ant_msk: best antenna for SISO (can be dual in CDD).
// Should be ANT_[ABC]
// @dual_stream_ant_msk: best antennas for MIMO, combination of ANT_[ABC]
// @initial_rate_index: first index from rs_table per AC category
// @agg_time_limit: aggregation max time threshold in usec/100, meaning
// value of 100 is one usec. Range is 100 to 8000
// @agg_disable_start_th: try-count threshold for starting aggregation.
// If a frame has higher try-count, it should not be selected for
// starting an aggregation sequence.
// @agg_frame_cnt_limit: max frame count in an aggregation.
// 0: no limit
// 1: no aggregation (one frame per aggregation)
// 2 - 0x3f: maximal number of frames (up to 3f == 63)
// @reserved2: reserved
// @rs_table: array of rates for each TX try, each is rate_n_flags,
// meaning it is a combination of RATE_MCS_* and IWL_RATE_*_PLCP
// @ss_params: single stream features. declare whether STBC or BFER are allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lq_cmd {
    pub sta_id: u8,
    pub reduced_tpc: u8,
    pub control: __le16,
// LINK_QUAL_GENERAL_PARAMS_API_S_VER_1
    pub flags: u8,
    pub mimo_delim: u8,
    pub single_stream_ant_msk: u8,
    pub dual_stream_ant_msk: u8,
    pub initial_rate_index: [u8; AC_NUM],
// LINK_QUAL_AGG_PARAMS_API_S_VER_1
    pub agg_time_limit: __le16,
    pub agg_disable_start_th: u8,
    pub agg_frame_cnt_limit: u8,
    pub reserved2: __le32,
    pub rs_table: [__le32; LQ_MAX_RETRY_NUM],
    pub ss_params: __le32,
}

extern "C" {
    pub fn rs_pretty_print_rate(buf: *mut c_char, bufsz: c_int, rate: u32) -> c_int;
}
extern "C" {
    pub fn iwl_he_is_sgi(rate_n_flags: u32) -> bool;
}
extern "C" {
    pub fn le32_to_cpu(_arg: rate) -> return;
}
extern "C" {
    pub fn cpu_to_le32(_arg: rate) -> return;
}
