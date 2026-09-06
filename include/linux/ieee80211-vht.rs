//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-vht.h
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
// IEEE 802.11 VHT definitions
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

pub const IEEE80211_MAX_MPDU_LEN_VHT_3895: c_int = 3895;
pub const IEEE80211_MAX_MPDU_LEN_VHT_7991: c_int = 7991;
pub const IEEE80211_MAX_MPDU_LEN_VHT_11454: c_int = 11454;
//
// enum ieee80211_vht_opmode_bits - VHT operating mode field bits
// @IEEE80211_OPMODE_NOTIF_CHANWIDTH_MASK: channel width mask
// @IEEE80211_OPMODE_NOTIF_CHANWIDTH_20MHZ: 20 MHz channel width
// @IEEE80211_OPMODE_NOTIF_CHANWIDTH_40MHZ: 40 MHz channel width
// @IEEE80211_OPMODE_NOTIF_CHANWIDTH_80MHZ: 80 MHz channel width
// @IEEE80211_OPMODE_NOTIF_CHANWIDTH_160MHZ: 160 MHz or 80+80 MHz channel width
// @IEEE80211_OPMODE_NOTIF_BW_160_80P80: 160 / 80+80 MHz indicator flag
// @IEEE80211_OPMODE_NOTIF_RX_NSS_MASK: number of spatial streams mask
// (the NSS value is the value of this field + 1)
// @IEEE80211_OPMODE_NOTIF_RX_NSS_SHIFT: number of spatial streams shift
// @IEEE80211_OPMODE_NOTIF_RX_NSS_TYPE_BF: indicates streams in SU-MIMO PPDU
// using a beamforming steering matrix
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_vht_opmode_bits {
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_MASK	= 0x03,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_20MHZ	= 0,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_40MHZ	= 1,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_80MHZ	= 2,
    IEEE80211_OPMODE_NOTIF_CHANWIDTH_160MHZ	= 3,
    IEEE80211_OPMODE_NOTIF_BW_160_80P80	= 0x04,
    IEEE80211_OPMODE_NOTIF_RX_NSS_MASK	= 0x70,
    IEEE80211_OPMODE_NOTIF_RX_NSS_SHIFT	= 4,
    IEEE80211_OPMODE_NOTIF_RX_NSS_TYPE_BF	= 0x80,
}

//
// Maximum length of AMPDU that the STA can receive in VHT.
// Length = 2 ^ (13 + max_ampdu_length_exp) - 1 (octets)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_vht_max_ampdu_length_exp {
    IEEE80211_VHT_MAX_AMPDU_8K = 0,
    IEEE80211_VHT_MAX_AMPDU_16K = 1,
    IEEE80211_VHT_MAX_AMPDU_32K = 2,
    IEEE80211_VHT_MAX_AMPDU_64K = 3,
    IEEE80211_VHT_MAX_AMPDU_128K = 4,
    IEEE80211_VHT_MAX_AMPDU_256K = 5,
    IEEE80211_VHT_MAX_AMPDU_512K = 6,
    IEEE80211_VHT_MAX_AMPDU_1024K = 7
}

//
// struct ieee80211_vht_mcs_info - VHT MCS information
// @rx_mcs_map: RX MCS map 2 bits for each stream, total 8 streams
// @rx_highest: Indicates highest long GI VHT PPDU data rate
// STA can receive. Rate expressed in units of 1 Mbps.
// If this field is 0 this value should not be used to
// consider the highest RX data rate supported.
// The top 3 bits of this field indicate the Maximum NSTS,total
// (a beamformee capability.)
// @tx_mcs_map: TX MCS map 2 bits for each stream, total 8 streams
// @tx_highest: Indicates highest long GI VHT PPDU data rate
// STA can transmit. Rate expressed in units of 1 Mbps.
// If this field is 0 this value should not be used to
// consider the highest TX data rate supported.
// The top 2 bits of this field are reserved, the
// 3rd bit from the top indiciates VHT Extended NSS BW
// Capability.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_vht_mcs_info {
    pub rx_mcs_map: __le16,
    pub rx_highest: __le16,
    pub tx_mcs_map: __le16,
    pub tx_highest: __le16,
    pub __packed: },
// for rx_highest
pub const IEEE80211_VHT_MAX_NSTS_TOTAL_SHIFT: c_int = 13;

// for tx_highest

//
// enum ieee80211_vht_mcs_support - VHT MCS support definitions
// @IEEE80211_VHT_MCS_SUPPORT_0_7: MCSes 0-7 are supported for the
// number of streams
// @IEEE80211_VHT_MCS_SUPPORT_0_8: MCSes 0-8 are supported
// @IEEE80211_VHT_MCS_SUPPORT_0_9: MCSes 0-9 are supported
// @IEEE80211_VHT_MCS_NOT_SUPPORTED: This number of streams isn't supported
//
// These definitions are used in each 2-bit subfield of the @rx_mcs_map
// and @tx_mcs_map fields of &struct ieee80211_vht_mcs_info, which are
// both split into 8 subfields by number of streams. These values indicate
// which MCSes are supported for the number of streams the value appears
// for.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_vht_mcs_support {
    IEEE80211_VHT_MCS_SUPPORT_0_7	= 0,
    IEEE80211_VHT_MCS_SUPPORT_0_8	= 1,
    IEEE80211_VHT_MCS_SUPPORT_0_9	= 2,
    IEEE80211_VHT_MCS_NOT_SUPPORTED	= 3,
}

//
// struct ieee80211_vht_cap - VHT capabilities
//
// This structure is the "VHT capabilities element" as
// described in 802.11ac D3.0 8.4.2.160
// @vht_cap_info: VHT capability info
// @supp_mcs: VHT MCS supported rates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_vht_cap {
    pub vht_cap_info: __le32,
    pub supp_mcs: ieee80211_vht_mcs_info,
    pub __packed: },
//
// enum ieee80211_vht_chanwidth - VHT channel width
// @IEEE80211_VHT_CHANWIDTH_USE_HT: use the HT operation IE to
// determine the channel width (20 or 40 MHz)
// @IEEE80211_VHT_CHANWIDTH_80MHZ: 80 MHz bandwidth
// @IEEE80211_VHT_CHANWIDTH_160MHZ: 160 MHz bandwidth
// @IEEE80211_VHT_CHANWIDTH_80P80MHZ: 80+80 MHz bandwidth
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_vht_chanwidth {
    IEEE80211_VHT_CHANWIDTH_USE_HT		= 0,
    IEEE80211_VHT_CHANWIDTH_80MHZ		= 1,
    IEEE80211_VHT_CHANWIDTH_160MHZ		= 2,
    IEEE80211_VHT_CHANWIDTH_80P80MHZ	= 3,
}

//
// struct ieee80211_vht_operation - VHT operation IE
//
// This structure is the "VHT operation element" as
// described in 802.11ac D3.0 8.4.2.161
// @chan_width: Operating channel width
// @center_freq_seg0_idx: center freq segment 0 index
// @center_freq_seg1_idx: center freq segment 1 index
// @basic_mcs_set: VHT Basic MCS rate set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_vht_operation {
    pub chan_width: u8,
    pub center_freq_seg0_idx: u8,
    pub center_freq_seg1_idx: u8,
    pub basic_mcs_set: __le16,
    pub __packed: },
// 802.11ac VHT Capabilities
pub const IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_3895: c_uint = 0x00000000;
pub const IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_7991: c_uint = 0x00000001;
pub const IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_11454: c_uint = 0x00000002;
pub const IEEE80211_VHT_CAP_MAX_MPDU_MASK: c_uint = 0x00000003;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160MHZ: c_uint = 0x00000004;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160_80PLUS80MHZ: c_uint = 0x00000008;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_MASK: c_uint = 0x0000000C;
pub const IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_SHIFT: c_int = 2;
pub const IEEE80211_VHT_CAP_RXLDPC: c_uint = 0x00000010;
pub const IEEE80211_VHT_CAP_SHORT_GI_80: c_uint = 0x00000020;
pub const IEEE80211_VHT_CAP_SHORT_GI_160: c_uint = 0x00000040;
pub const IEEE80211_VHT_CAP_TXSTBC: c_uint = 0x00000080;
pub const IEEE80211_VHT_CAP_RXSTBC_1: c_uint = 0x00000100;
pub const IEEE80211_VHT_CAP_RXSTBC_2: c_uint = 0x00000200;
pub const IEEE80211_VHT_CAP_RXSTBC_3: c_uint = 0x00000300;
pub const IEEE80211_VHT_CAP_RXSTBC_4: c_uint = 0x00000400;
pub const IEEE80211_VHT_CAP_RXSTBC_MASK: c_uint = 0x00000700;
pub const IEEE80211_VHT_CAP_RXSTBC_SHIFT: c_int = 8;
pub const IEEE80211_VHT_CAP_SU_BEAMFORMER_CAPABLE: c_uint = 0x00000800;
pub const IEEE80211_VHT_CAP_SU_BEAMFORMEE_CAPABLE: c_uint = 0x00001000;
pub const IEEE80211_VHT_CAP_BEAMFORMEE_STS_SHIFT: c_int = 13;

pub const IEEE80211_VHT_CAP_SOUNDING_DIMENSIONS_SHIFT: c_int = 16;

pub const IEEE80211_VHT_CAP_MU_BEAMFORMER_CAPABLE: c_uint = 0x00080000;
pub const IEEE80211_VHT_CAP_MU_BEAMFORMEE_CAPABLE: c_uint = 0x00100000;
pub const IEEE80211_VHT_CAP_VHT_TXOP_PS: c_uint = 0x00200000;
pub const IEEE80211_VHT_CAP_HTC_VHT: c_uint = 0x00400000;
pub const IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_SHIFT: c_int = 23;

pub const IEEE80211_VHT_CAP_VHT_LINK_ADAPTATION_VHT_UNSOL_MFB: c_uint = 0x08000000;
pub const IEEE80211_VHT_CAP_VHT_LINK_ADAPTATION_VHT_MRQ_MFB: c_uint = 0x0c000000;
pub const IEEE80211_VHT_CAP_RX_ANTENNA_PATTERN: c_uint = 0x10000000;
pub const IEEE80211_VHT_CAP_TX_ANTENNA_PATTERN: c_uint = 0x20000000;
pub const IEEE80211_VHT_CAP_EXT_NSS_BW_SHIFT: c_int = 30;
pub const IEEE80211_VHT_CAP_EXT_NSS_BW_MASK: c_uint = 0xc0000000;
//
// ieee80211_get_vht_max_nss - return max NSS for a given bandwidth/MCS
// @cap: VHT capabilities of the peer
// @bw: bandwidth to use
// @mcs: MCS index to use
// @ext_nss_bw_capable: indicates whether or not the local transmitter
// (rate scaling algorithm) can deal with the new logic
// (dot11VHTExtendedNSSBWCapable)
// @max_vht_nss: current maximum NSS as advertised by the STA in
// operating mode notification, can be 0 in which case the
// capability data will be used to derive this (from MCS support)
// Return: The maximum NSS that can be used for the given bandwidth/MCS
// combination
//
// Due to the VHT Extended NSS Bandwidth Support, the maximum NSS can
// vary for a given BW/MCS. This function parses the data.
//
// Note: This function is exported by cfg80211.
//
    pub max_vht_nss): c_uint,
// VHT action codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_vht_actioncode {
    WLAN_VHT_ACTION_COMPRESSED_BF = 0,
    WLAN_VHT_ACTION_GROUPID_MGMT = 1,
    WLAN_VHT_ACTION_OPMODE_NOTIF = 2,
}
