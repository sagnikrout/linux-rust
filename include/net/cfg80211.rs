//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/cfg80211.h
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
// 802.11 device and configuration interface
//
// Copyright 2006-2010	Johannes Berg <johannes@sipsolutions.net>
// Copyright 2013-2014 Intel Mobile Communications GmbH
// Copyright 2015-2017	Intel Deutschland GmbH
// Copyright (C) 2018-2026 Intel Corporation
//

//
// DOC: Introduction
//
// cfg80211 is the configuration API for 802.11 devices in Linux. It bridges
// userspace and drivers, and offers some utility functionality associated
// with 802.11. cfg80211 must, directly or indirectly via mac80211, be used
// by all modern wireless drivers in Linux, so that they offer a consistent
// API through nl80211. For backward compatibility, cfg80211 also offers
// wireless extensions to userspace, but hides them from drivers completely.
//
// Additionally, cfg80211 contains code to help enforce regulatory spectrum
// use restrictions.
//
// DOC: Device registration
//
// In order for a driver to use cfg80211, it must register the hardware device
// with cfg80211. This happens through a number of hardware capability structs
// described below.
//
// The fundamental structure for each device is the 'wiphy', of which each
// instance describes a physical wireless device connected to the system. Each
// such wiphy can have zero, one, or many virtual interfaces associated with
// it, which need to be identified as such by pointing the network interface's
// @ieee80211_ptr pointer to a &struct wireless_dev which further describes
// the wireless part of the interface. Normally this struct is embedded in the
// network interface's private data area. Drivers can optionally allow creating
// or destroying virtual interfaces on the fly, but without at least one or the
// ability to create some the wireless device isn't useful.
//
// Each wiphy structure contains device capability information, and also has
// a pointer to the various operations the driver offers. The definitions and
// structures here describe these capabilities in detail.
//
// wireless hardware capability structures
//
// enum ieee80211_channel_flags - channel flags
//
// Channel flags set by the regulatory control code.
//
// @IEEE80211_CHAN_DISABLED: This channel is disabled.
// @IEEE80211_CHAN_NO_IR: do not initiate radiation, this includes
// sending probe requests or beaconing.
// @IEEE80211_CHAN_PSD: Power spectral density (in dBm) is set for this
// channel.
// @IEEE80211_CHAN_RADAR: Radar detection is required on this channel.
// @IEEE80211_CHAN_NO_HT40PLUS: extension channel above this channel
// is not permitted.
// @IEEE80211_CHAN_NO_HT40MINUS: extension channel below this channel
// is not permitted.
// @IEEE80211_CHAN_NO_OFDM: OFDM is not allowed on this channel.
// @IEEE80211_CHAN_NO_80MHZ: If the driver supports 80 MHz on the band,
// this flag indicates that an 80 MHz channel cannot use this
// channel as the control or any of the secondary channels.
// This may be due to the driver or due to regulatory bandwidth
// restrictions.
// @IEEE80211_CHAN_NO_160MHZ: If the driver supports 160 MHz on the band,
// this flag indicates that an 160 MHz channel cannot use this
// channel as the control or any of the secondary channels.
// This may be due to the driver or due to regulatory bandwidth
// restrictions.
// @IEEE80211_CHAN_INDOOR_ONLY: see %NL80211_FREQUENCY_ATTR_INDOOR_ONLY
// @IEEE80211_CHAN_IR_CONCURRENT: see %NL80211_FREQUENCY_ATTR_IR_CONCURRENT
// @IEEE80211_CHAN_NO_20MHZ: 20 MHz bandwidth is not permitted
// on this channel.
// @IEEE80211_CHAN_NO_10MHZ: 10 MHz bandwidth is not permitted
// on this channel.
// @IEEE80211_CHAN_NO_HE: HE operation is not permitted on this channel.
// @IEEE80211_CHAN_NO_320MHZ: If the driver supports 320 MHz on the band,
// this flag indicates that a 320 MHz channel cannot use this
// channel as the control or any of the secondary channels.
// This may be due to the driver or due to regulatory bandwidth
// restrictions.
// @IEEE80211_CHAN_NO_EHT: EHT operation is not permitted on this channel.
// @IEEE80211_CHAN_DFS_CONCURRENT: See %NL80211_RRF_DFS_CONCURRENT
// @IEEE80211_CHAN_NO_6GHZ_VLP_CLIENT: Client connection with VLP AP
// not permitted using this channel
// @IEEE80211_CHAN_NO_6GHZ_AFC_CLIENT: Client connection with AFC AP
// not permitted using this channel
// @IEEE80211_CHAN_CAN_MONITOR: This channel can be used for monitor
// mode even in the presence of other (regulatory) restrictions,
// even if it is otherwise disabled.
// @IEEE80211_CHAN_ALLOW_6GHZ_VLP_AP: Allow using this channel for AP operation
// with very low power (VLP), even if otherwise set to NO_IR.
// @IEEE80211_CHAN_ALLOW_20MHZ_ACTIVITY: Allow activity on a 20 MHz channel,
// even if otherwise set to NO_IR.
// @IEEE80211_CHAN_S1G_NO_PRIMARY: Prevents the channel for use as an S1G
// primary channel. Does not prevent the wider operating channel
// described by the chandef from being used. In order for a 2MHz primary
// to be used, both 1MHz subchannels shall not contain this flag.
// @IEEE80211_CHAN_NO_4MHZ: 4 MHz bandwidth is not permitted on this channel.
// @IEEE80211_CHAN_NO_8MHZ: 8 MHz bandwidth is not permitted on this channel.
// @IEEE80211_CHAN_NO_16MHZ: 16 MHz bandwidth is not permitted on this channel.
// @IEEE80211_CHAN_NO_UHR: UHR operation is not permitted on this channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_channel_flags {
    IEEE80211_CHAN_DISABLED			= BIT(0),
    IEEE80211_CHAN_NO_IR			= BIT(1),
    IEEE80211_CHAN_PSD			= BIT(2),
    IEEE80211_CHAN_RADAR			= BIT(3),
    IEEE80211_CHAN_NO_HT40PLUS		= BIT(4),
    IEEE80211_CHAN_NO_HT40MINUS		= BIT(5),
    IEEE80211_CHAN_NO_OFDM			= BIT(6),
    IEEE80211_CHAN_NO_80MHZ			= BIT(7),
    IEEE80211_CHAN_NO_160MHZ		= BIT(8),
    IEEE80211_CHAN_INDOOR_ONLY		= BIT(9),
    IEEE80211_CHAN_IR_CONCURRENT		= BIT(10),
    IEEE80211_CHAN_NO_20MHZ			= BIT(11),
    IEEE80211_CHAN_NO_10MHZ			= BIT(12),
    IEEE80211_CHAN_NO_HE			= BIT(13),
// can use free bits here
    IEEE80211_CHAN_NO_UHR			= BIT(18),
    IEEE80211_CHAN_NO_320MHZ		= BIT(19),
    IEEE80211_CHAN_NO_EHT			= BIT(20),
    IEEE80211_CHAN_DFS_CONCURRENT		= BIT(21),
    IEEE80211_CHAN_NO_6GHZ_VLP_CLIENT	= BIT(22),
    IEEE80211_CHAN_NO_6GHZ_AFC_CLIENT	= BIT(23),
    IEEE80211_CHAN_CAN_MONITOR		= BIT(24),
    IEEE80211_CHAN_ALLOW_6GHZ_VLP_AP	= BIT(25),
    IEEE80211_CHAN_ALLOW_20MHZ_ACTIVITY     = BIT(26),
    IEEE80211_CHAN_S1G_NO_PRIMARY		= BIT(27),
    IEEE80211_CHAN_NO_4MHZ			= BIT(28),
    IEEE80211_CHAN_NO_8MHZ			= BIT(29),
    IEEE80211_CHAN_NO_16MHZ			= BIT(30),
}

pub const IEEE80211_DFS_MIN_CAC_TIME_MS: c_int = 60000;

//
// struct ieee80211_channel - channel definition
//
// This structure describes a single channel for use
// with cfg80211.
//
// @center_freq: center frequency in MHz
// @freq_offset: offset from @center_freq, in KHz
// @hw_value: hardware-specific value for the channel
// @flags: channel flags from &enum ieee80211_channel_flags.
// @orig_flags: channel flags at registration time, used by regulatory
// code to support devices with additional restrictions
// @band: band this channel belongs to.
// @max_antenna_gain: maximum antenna gain in dBi
// @max_power: maximum transmission power (in dBm)
// @max_reg_power: maximum regulatory transmission power (in dBm)
// @beacon_found: helper to regulatory code to indicate when a beacon
// has been found on this channel. Use regulatory_hint_found_beacon()
// to enable this, this is useful only on 5 GHz band.
// @orig_mag: internal use
// @orig_mpwr: internal use
// @dfs_state: current state of this channel. Only relevant if radar is required
// on this channel.
// @dfs_state_entered: timestamp (jiffies) when the dfs state was entered.
// @dfs_cac_ms: DFS CAC time in milliseconds, this is valid for DFS channels.
// @cac_start_time: timestamp (CLOCK_BOOTTIME, nanoseconds) when CAC was
// started on this channel. Zero when CAC is not in progress.
// @psd: power spectral density (in dBm)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_channel {
    pub band: nl80211_band,
    pub center_freq: u32,
    pub freq_offset: u16,
    pub hw_value: u16,
    pub flags: u32,
    pub max_antenna_gain: c_int,
    pub max_power: c_int,
    pub max_reg_power: c_int,
    pub beacon_found: bool,
    pub orig_flags: u32,
    pub orig_mpwr: int orig_mag,,
    pub dfs_state: nl80211_dfs_state,
    pub dfs_state_entered: c_ulong,
    pub dfs_cac_ms: c_uint,
    pub cac_start_time: u64,
    pub psd: i8,
}

//
// enum ieee80211_rate_flags - rate flags
//
// Hardware/specification flags for rates. These are structured
// in a way that allows using the same bitrate structure for
// different bands/PHY modes.
//
// @IEEE80211_RATE_SHORT_PREAMBLE: Hardware can send with short
// preamble on this bitrate; only relevant in 2.4GHz band and
// with CCK rates.
// @IEEE80211_RATE_MANDATORY_A: This bitrate is a mandatory rate
// when used with 802.11a (on the 5 GHz band); filled by the
// core code when registering the wiphy.
// @IEEE80211_RATE_MANDATORY_B: This bitrate is a mandatory rate
// when used with 802.11b (on the 2.4 GHz band); filled by the
// core code when registering the wiphy.
// @IEEE80211_RATE_MANDATORY_G: This bitrate is a mandatory rate
// when used with 802.11g (on the 2.4 GHz band); filled by the
// core code when registering the wiphy.
// @IEEE80211_RATE_ERP_G: This is an ERP rate in 802.11g mode.
// @IEEE80211_RATE_SUPPORTS_5MHZ: Rate can be used in 5 MHz mode
// @IEEE80211_RATE_SUPPORTS_10MHZ: Rate can be used in 10 MHz mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_rate_flags {
    IEEE80211_RATE_SHORT_PREAMBLE	= BIT(0),
    IEEE80211_RATE_MANDATORY_A	= BIT(1),
    IEEE80211_RATE_MANDATORY_B	= BIT(2),
    IEEE80211_RATE_MANDATORY_G	= BIT(3),
    IEEE80211_RATE_ERP_G		= BIT(4),
    IEEE80211_RATE_SUPPORTS_5MHZ	= BIT(5),
    IEEE80211_RATE_SUPPORTS_10MHZ	= BIT(6),
}

//
// enum ieee80211_bss_type - BSS type filter
//
// @IEEE80211_BSS_TYPE_ESS: Infrastructure BSS
// @IEEE80211_BSS_TYPE_PBSS: Personal BSS
// @IEEE80211_BSS_TYPE_IBSS: Independent BSS
// @IEEE80211_BSS_TYPE_MBSS: Mesh BSS
// @IEEE80211_BSS_TYPE_ANY: Wildcard value for matching any BSS type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_bss_type {
    IEEE80211_BSS_TYPE_ESS,
    IEEE80211_BSS_TYPE_PBSS,
    IEEE80211_BSS_TYPE_IBSS,
    IEEE80211_BSS_TYPE_MBSS,
    IEEE80211_BSS_TYPE_ANY
}

//
// enum ieee80211_privacy - BSS privacy filter
//
// @IEEE80211_PRIVACY_ON: privacy bit set
// @IEEE80211_PRIVACY_OFF: privacy bit clear
// @IEEE80211_PRIVACY_ANY: Wildcard value for matching any privacy setting
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_privacy {
    IEEE80211_PRIVACY_ON,
    IEEE80211_PRIVACY_OFF,
    IEEE80211_PRIVACY_ANY
}

//
// struct ieee80211_rate - bitrate definition
//
// This structure describes a bitrate that an 802.11 PHY can
// operate with. The two values @hw_value and @hw_value_short
// are only for driver use when pointers to this structure are
// passed around.
//
// @flags: rate-specific flags from &enum ieee80211_rate_flags
// @bitrate: bitrate in units of 100 Kbps
// @hw_value: driver/hardware value for this rate
// @hw_value_short: driver/hardware value for this rate when
// short preamble is used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_rate {
    pub flags: u32,
    pub bitrate: u16,
    pub hw_value_short: u16 hw_value,,
}

//
// struct ieee80211_he_obss_pd - AP settings for spatial reuse
//
// @enable: is the feature enabled.
// @sr_ctrl: The SR Control field of SRP element.
// @non_srg_max_offset: non-SRG maximum tx power offset
// @min_offset: minimal tx power offset an associated station shall use
// @max_offset: maximum tx power offset an associated station shall use
// @bss_color_bitmap: bitmap that indicates the BSS color values used by
// members of the SRG
// @partial_bssid_bitmap: bitmap that indicates the partial BSSID values
// used by members of the SRG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_he_obss_pd {
    pub enable: bool,
    pub sr_ctrl: u8,
    pub non_srg_max_offset: u8,
    pub min_offset: u8,
    pub max_offset: u8,
    pub bss_color_bitmap: [u8; 8],
    pub partial_bssid_bitmap: [u8; 8],
}

//
// struct cfg80211_he_bss_color - AP settings for BSS coloring
//
// @color: the current color.
// @enabled: HE BSS color is used
// @partial: define the AID equation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_he_bss_color {
    pub color: u8,
    pub enabled: bool,
    pub partial: bool,
}

//
// struct ieee80211_sta_ht_cap - STA's HT capabilities
//
// This structure describes most essential parameters needed
// to describe 802.11n HT capabilities for an STA.
//
// @ht_supported: is HT supported by the STA
// @cap: HT capabilities map as described in 802.11n spec
// @ampdu_factor: Maximum A-MPDU length factor
// @ampdu_density: Minimum A-MPDU spacing
// @mcs: Supported MCS rates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_ht_cap {
    pub /: *mut *mut u16 cap; / use IEEE80211_HT_CAP_,
    pub ht_supported: bool,
    pub ampdu_factor: u8,
    pub ampdu_density: u8,
    pub mcs: ieee80211_mcs_info,
}

//
// struct ieee80211_sta_vht_cap - STA's VHT capabilities
//
// This structure describes most essential parameters needed
// to describe 802.11ac VHT capabilities for an STA.
//
// @vht_supported: is VHT supported by the STA
// @cap: VHT capabilities map as described in 802.11ac spec
// @vht_mcs: Supported VHT MCS rates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_vht_cap {
    pub vht_supported: bool,
    pub /: *mut *mut u32 cap; / use IEEE80211_VHT_CAP_,
    pub vht_mcs: ieee80211_vht_mcs_info,
}

pub const IEEE80211_HE_PPE_THRES_MAX_LEN: c_int = 25;
//
// struct ieee80211_sta_he_cap - STA's HE capabilities
//
// This structure describes most essential parameters needed
// to describe 802.11ax HE capabilities for a STA.
//
// @has_he: true iff HE data is valid.
// @he_cap_elem: Fixed portion of the HE capabilities element.
// @he_mcs_nss_supp: The supported NSS/MCS combinations.
// @ppe_thres: Holds the PPE Thresholds data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_he_cap {
    pub has_he: bool,
    pub he_cap_elem: ieee80211_he_cap_elem,
    pub he_mcs_nss_supp: ieee80211_he_mcs_nss_supp,
    pub ppe_thres: [u8; IEEE80211_HE_PPE_THRES_MAX_LEN],
}

//
// struct ieee80211_eht_mcs_nss_supp - EHT max supported NSS per MCS
//
// See P802.11be_D1.3 Table 9-401k - "Subfields of the Supported EHT-MCS
// and NSS Set field"
//
// @only_20mhz: MCS/NSS support for 20 MHz-only STA.
// @bw: MCS/NSS support for 80, 160 and 320 MHz
// @bw._80: MCS/NSS support for BW <= 80 MHz
// @bw._160: MCS/NSS support for BW = 160 MHz
// @bw._320: MCS/NSS support for BW = 320 MHz
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_eht_mcs_nss_supp {
    pub only_20mhz: ieee80211_eht_mcs_nss_supp_20mhz_only,
    pub _80: ieee80211_eht_mcs_nss_supp_bw,
    pub _160: ieee80211_eht_mcs_nss_supp_bw,
    pub _320: ieee80211_eht_mcs_nss_supp_bw,
    pub bw: } __packed,
    pub __packed: },
    pub __packed: },
pub const IEEE80211_EHT_PPE_THRES_MAX_LEN: c_int = 32;
//
// struct ieee80211_sta_eht_cap - STA's EHT capabilities
//
// This structure describes most essential parameters needed
// to describe 802.11be EHT capabilities for a STA.
//
// @has_eht: true iff EHT data is valid.
// @eht_cap_elem: Fixed portion of the eht capabilities element.
// @eht_mcs_nss_supp: The supported NSS/MCS combinations.
// @eht_ppe_thres: Holds the PPE Thresholds data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_eht_cap {
    pub has_eht: bool,
    pub eht_cap_elem: ieee80211_eht_cap_elem_fixed,
    pub eht_mcs_nss_supp: ieee80211_eht_mcs_nss_supp,
    pub eht_ppe_thres: [u8; IEEE80211_EHT_PPE_THRES_MAX_LEN],
}

//
// struct ieee80211_sta_uhr_cap - STA's UHR capabilities
// @has_uhr: true iff UHR is supported and data is valid
// @mac: fixed MAC capabilities
// @phy: fixed PHY capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_uhr_cap {
    pub has_uhr: bool,
    pub mac: ieee80211_uhr_cap_mac,
    pub phy: ieee80211_uhr_cap_phy,
}

// sparse defines __CHECKER__; see Documentation/dev-tools/sparse.rst

//
// This is used to mark the sband->iftype_data pointer which is supposed
// to be an array with special access semantics (per iftype), but a lot
// of code got it wrong in the past, so with this marking sparse will be
// noisy when the pointer is used directly.
//

//
// struct ieee80211_sband_iftype_data - sband data per interface type
//
// This structure encapsulates sband data that is relevant for the
// interface types defined in @types_mask.  Each type in the
// @types_mask must be unique across all instances of iftype_data.
//
// @types_mask: interface types mask
// @he_cap: holds the HE capabilities
// @he_6ghz_capa: HE 6 GHz capabilities, must be filled in for a
// 6 GHz band channel (and 0 may be valid value).
// @eht_cap: STA's EHT capabilities
// @uhr_cap: STA's UHR capabilities
// @vendor_elems: vendor element(s) to advertise
// @vendor_elems.data: vendor element(s) data
// @vendor_elems.len: vendor element(s) length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sband_iftype_data {
    pub types_mask: u16,
    pub he_cap: ieee80211_sta_he_cap,
    pub he_6ghz_capa: ieee80211_he_6ghz_capa,
    pub eht_cap: ieee80211_sta_eht_cap,
    pub uhr_cap: ieee80211_sta_uhr_cap,
    pub data: *const u8,
    pub len: c_uint,
    pub vendor_elems: },
}

//
// enum ieee80211_edmg_bw_config - allowed channel bandwidth configurations
//
// @IEEE80211_EDMG_BW_CONFIG_4: 2.16GHz
// @IEEE80211_EDMG_BW_CONFIG_5: 2.16GHz and 4.32GHz
// @IEEE80211_EDMG_BW_CONFIG_6: 2.16GHz, 4.32GHz and 6.48GHz
// @IEEE80211_EDMG_BW_CONFIG_7: 2.16GHz, 4.32GHz, 6.48GHz and 8.64GHz
// @IEEE80211_EDMG_BW_CONFIG_8: 2.16GHz and 2.16GHz + 2.16GHz
// @IEEE80211_EDMG_BW_CONFIG_9: 2.16GHz, 4.32GHz and 2.16GHz + 2.16GHz
// @IEEE80211_EDMG_BW_CONFIG_10: 2.16GHz, 4.32GHz, 6.48GHz and 2.16GHz+2.16GHz
// @IEEE80211_EDMG_BW_CONFIG_11: 2.16GHz, 4.32GHz, 6.48GHz, 8.64GHz and
// 2.16GHz+2.16GHz
// @IEEE80211_EDMG_BW_CONFIG_12: 2.16GHz, 2.16GHz + 2.16GHz and
// 4.32GHz + 4.32GHz
// @IEEE80211_EDMG_BW_CONFIG_13: 2.16GHz, 4.32GHz, 2.16GHz + 2.16GHz and
// 4.32GHz + 4.32GHz
// @IEEE80211_EDMG_BW_CONFIG_14: 2.16GHz, 4.32GHz, 6.48GHz, 2.16GHz + 2.16GHz
// and 4.32GHz + 4.32GHz
// @IEEE80211_EDMG_BW_CONFIG_15: 2.16GHz, 4.32GHz, 6.48GHz, 8.64GHz,
// 2.16GHz + 2.16GHz and 4.32GHz + 4.32GHz
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_edmg_bw_config {
    IEEE80211_EDMG_BW_CONFIG_4	= 4,
    IEEE80211_EDMG_BW_CONFIG_5	= 5,
    IEEE80211_EDMG_BW_CONFIG_6	= 6,
    IEEE80211_EDMG_BW_CONFIG_7	= 7,
    IEEE80211_EDMG_BW_CONFIG_8	= 8,
    IEEE80211_EDMG_BW_CONFIG_9	= 9,
    IEEE80211_EDMG_BW_CONFIG_10	= 10,
    IEEE80211_EDMG_BW_CONFIG_11	= 11,
    IEEE80211_EDMG_BW_CONFIG_12	= 12,
    IEEE80211_EDMG_BW_CONFIG_13	= 13,
    IEEE80211_EDMG_BW_CONFIG_14	= 14,
    IEEE80211_EDMG_BW_CONFIG_15	= 15,
}

//
// struct ieee80211_edmg - EDMG configuration
//
// This structure describes most essential parameters needed
// to describe 802.11ay EDMG configuration
//
// @channels: bitmap that indicates the 2.16 GHz channel(s)
// that are allowed to be used for transmissions.
// Bit 0 indicates channel 1, bit 1 indicates channel 2, etc.
// Set to 0 indicate EDMG not supported.
// @bw_config: Channel BW Configuration subfield encodes
// the allowed channel bandwidth configurations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_edmg {
    pub channels: u8,
    pub bw_config: ieee80211_edmg_bw_config,
}

//
// struct ieee80211_sta_s1g_cap - STA's S1G capabilities
//
// This structure describes most essential parameters needed
// to describe 802.11ah S1G capabilities for a STA.
//
// @s1g: is STA an S1G STA
// @cap: S1G capabilities information
// @nss_mcs: Supported NSS MCS set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_s1g_cap {
    pub s1g: bool,
    pub /: *mut *mut u8 cap[10]; / use S1G_CAPAB_,
    pub nss_mcs: [u8; 5],
}

//
// struct ieee80211_supported_band - frequency band definition
//
// This structure describes a frequency band a wiphy
// is able to operate in.
//
// @channels: Array of channels the hardware can operate with
// in this band.
// @band: the band this structure represents
// @n_channels: Number of channels in @channels
// @bitrates: Array of bitrates the hardware can operate with
// in this band. Must be sorted to give a valid "supported
// rates" IE, i.e. CCK rates first, then OFDM.
// @n_bitrates: Number of bitrates in @bitrates
// @ht_cap: HT capabilities in this band
// @vht_cap: VHT capabilities in this band
// @s1g_cap: S1G capabilities in this band
// @edmg_cap: EDMG capabilities in this band
// @s1g_cap: S1G capabilities in this band (S1G band only, of course)
// @n_iftype_data: number of iftype data entries
// @iftype_data: interface type data entries.  Note that the bits in
// @types_mask inside this structure cannot overlap (i.e. only
// one occurrence of each type is allowed across all instances of
// iftype_data).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_supported_band {
    pub channels: *mut ieee80211_channel,
    pub bitrates: *mut ieee80211_rate,
    pub band: nl80211_band,
    pub n_channels: c_int,
    pub n_bitrates: c_int,
    pub ht_cap: ieee80211_sta_ht_cap,
    pub vht_cap: ieee80211_sta_vht_cap,
    pub s1g_cap: ieee80211_sta_s1g_cap,
    pub edmg_cap: ieee80211_edmg,
    pub n_iftype_data: u16,
    pub iftype_data: *const ieee80211_sband_iftype_data __iftd,
}

//
// _ieee80211_set_sband_iftype_data - set sband iftype data array
// @sband: the sband to initialize
// @iftd: the iftype data array pointer
// @n_iftd: the length of the iftype data array
//
// Set the sband iftype data array; use this where the length cannot
// be derived from the ARRAY_SIZE() of the argument, but prefer
// ieee80211_set_sband_iftype_data() where it can be used.
//
// ieee80211_set_sband_iftype_data - set sband iftype data array
// @sband: the sband to initialize
// @iftd: the iftype data array
//

//
// for_each_sband_iftype_data - iterate sband iftype data entries
// @sband: the sband whose iftype_data array to iterate
// @i: iterator counter
// @iftd: iftype data pointer to set
//

//
// ieee80211_get_sband_iftype_data - return sband data for a given iftype
// @sband: the sband to search for the STA on
// @iftype: enum nl80211_iftype
//
// Return: pointer to struct ieee80211_sband_iftype_data, or NULL is none found
//
// ieee80211_get_he_iftype_cap - return HE capabilities for an sband's iftype
// @sband: the sband to search for the iftype on
// @iftype: enum nl80211_iftype
//
// Return: pointer to the struct ieee80211_sta_he_cap, or NULL is none found
//
// ieee80211_get_he_6ghz_capa - return HE 6 GHz capabilities
// @sband: the sband to search for the STA on
// @iftype: the iftype to search for
//
// Return: the 6GHz capabilities
//
// ieee80211_get_eht_iftype_cap - return EHT capabilities for an sband's iftype
// @sband: the sband to search for the iftype on
// @iftype: enum nl80211_iftype
//
// Return: pointer to the struct ieee80211_sta_eht_cap, or NULL is none found
//
// ieee80211_get_uhr_iftype_cap - return UHR capabilities for an sband's iftype
// @sband: the sband to search for the iftype on
// @iftype: enum nl80211_iftype
//
// Return: pointer to the struct ieee80211_sta_uhr_cap, or NULL is none found
//
// wiphy_read_of_freq_limits - read frequency limits from device tree
//
// @wiphy: the wireless device to get extra limits for
//
// Some devices may have extra limitations specified in DT. This may be useful
// for chipsets that normally support more bands but are limited due to board
// design (e.g. by antennas or external power amplifier).
//
// This function reads info from DT and uses it to *modify* channels (disable
// unavailable ones). It's usually a *bad* idea to use it in drivers with
// shared channel data as DT limitations are device specific. You should make
// sure to call it only if channels in wiphy are copied and can be modified
// without affecting other devices.
//
// As this function access device node it has to be called after set_wiphy_dev.
// It also modifies channels so they have to be set first.
// If using this helper, call it before wiphy_register().
//

extern "C" {
    pub fn wiphy_read_of_freq_limits(wiphy: *mut wiphy);
}

//
// Wireless hardware/device configuration structures and methods
//
// DOC: Actions and configuration
//
// Each wireless device and each virtual interface offer a set of configuration
// operations and other actions that are invoked by userspace. Each of these
// actions is described in the operations structure, and the parameters these
// operations use are described separately.
//
// Additionally, some operations are asynchronous and expect to get status
// information via some functions that drivers need to call.
//
// Scanning and BSS list handling with its associated functionality is described
// in a separate chapter.
//

//
// struct vif_params - describes virtual interface parameters
// @flags: monitor interface flags, unchanged if 0, otherwise
// %MONITOR_FLAG_CHANGED will be set
// @use_4addr: use 4-address frames
// @macaddr: address to use for this virtual interface.
// If this parameter is set to zero address the driver may
// determine the address as needed.
// This feature is only fully supported by drivers that enable the
// %NL80211_FEATURE_MAC_ON_CREATE flag.  Others may support creating
// only p2p devices with specified MAC.
// @vht_mumimo_groups: MU-MIMO groupID, used for monitoring MU-MIMO packets
// belonging to that MU-MIMO groupID; %NULL if not changed
// @vht_mumimo_follow_addr: MU-MIMO follow address, used for monitoring
// MU-MIMO packets going to the specified station; %NULL if not changed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vif_params {
    pub flags: u32,
    pub use_4addr: c_int,
    pub macaddr: [u8; ETH_ALEN],
    pub vht_mumimo_groups: *const u8,
    pub vht_mumimo_follow_addr: *const u8,
}

//
// struct key_params - key information
//
// Information about a key
//
// @key: key material
// @key_len: length of key material
// @cipher: cipher suite selector
// @seq: sequence counter (IV/PN), must be in little endian,
// length given by @seq_len.
// @seq_len: length of @seq.
// @vlan_id: vlan_id for VLAN group key (if nonzero)
// @mode: key install mode (RX_TX, NO_TX or SET_TX)
// @ltf_keyseed: LTF key seed material
// @ltf_keyseed_len: length of LTF key seed material
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_params {
    pub key: *const u8,
    pub seq: *const u8,
    pub key_len: c_int,
    pub seq_len: c_int,
    pub vlan_id: u16,
    pub cipher: u32,
    pub mode: nl80211_key_mode,
    pub ltf_keyseed: *const u8,
    pub ltf_keyseed_len: usize,
}

//
// struct cfg80211_chan_def - channel definition
// @chan: the (control) channel
// @npca_chan: the NPCA primary channel
// Note that if DBE is in use, this channel may appear to be
// inside the primary half of the chandef. Implementations
// can use the position of this channel to understand how
// NPCA is used.
// @width: channel width
// @center_freq1: center frequency of first segment
// @center_freq2: center frequency of second segment
// (only with 80+80 MHz)
// @edmg: define the EDMG channels configuration.
// If edmg is requested (i.e. the .channels member is non-zero),
// chan will define the primary channel and all other
// parameters are ignored.
// @freq1_offset: offset from @center_freq1, in KHz
// @punctured: mask of the punctured 20 MHz subchannels, with
// bits turned on being disabled (punctured); numbered
// from lower to higher frequency (like in the spec)
// @npca_punctured: NPCA puncturing bitmap, like @punctured but for
// NPCA transmissions. If NPCA is used (@npca_chan is not %NULL)
// this will be a superset of the @punctured bimap.
// Note that if DBE is used, this bitmap is also shifted to be in
// accordance with the overall chandef bandwidth.
// @s1g_primary_2mhz: Indicates if the control channel pointed to
// by 'chan' exists as a 1MHz primary subchannel within an
// S1G 2MHz primary channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_chan_def {
    pub chan: *mut ieee80211_channel,
    pub npca_chan: *mut ieee80211_channel,
    pub width: nl80211_chan_width,
    pub center_freq1: u32,
    pub center_freq2: u32,
    pub edmg: ieee80211_edmg,
    pub freq1_offset: u16,
    pub npca_punctured: u16 punctured,,
    pub s1g_primary_2mhz: bool,
}

//
// cfg80211_bitrate_mask - masks for bitrate control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_bitrate_mask {
    pub legacy: u32,
    pub ht_mcs: [u8; IEEE80211_HT_MCS_MASK_LEN],
    pub vht_mcs: [u16; NL80211_VHT_NSS_MAX],
    pub he_mcs: [u16; NL80211_HE_NSS_MAX],
    pub eht_mcs: [u16; NL80211_EHT_NSS_MAX],
    pub gi: nl80211_txrate_gi,
    pub he_gi: nl80211_he_gi,
    pub eht_gi: nl80211_eht_gi,
    pub he_ltf: nl80211_he_ltf,
    pub eht_ltf: nl80211_eht_ltf,
    pub control: [}; NUM_NL80211_BANDS],
}

//
// struct cfg80211_tid_cfg - TID specific configuration
// @config_override: Flag to notify driver to reset TID configuration
// of the peer.
// @tids: bitmap of TIDs to modify
// @mask: bitmap of attributes indicating which parameter changed,
// similar to &nl80211_tid_config_supp.
// @noack: noack configuration value for the TID
// @retry_long: retry count value
// @retry_short: retry count value
// @ampdu: Enable/Disable MPDU aggregation
// @rtscts: Enable/Disable RTS/CTS
// @amsdu: Enable/Disable MSDU aggregation
// @txrate_type: Tx bitrate mask type
// @txrate_mask: Tx bitrate to be applied for the TID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_tid_cfg {
    pub config_override: bool,
    pub tids: u8,
    pub mask: u64,
    pub noack: nl80211_tid_config,
    pub retry_short: u8 retry_long,,
    pub ampdu: nl80211_tid_config,
    pub rtscts: nl80211_tid_config,
    pub amsdu: nl80211_tid_config,
    pub txrate_type: nl80211_tx_rate_setting,
    pub txrate_mask: cfg80211_bitrate_mask,
}

//
// struct cfg80211_tid_config - TID configuration
// @peer: Station's MAC address
// @n_tid_conf: Number of TID specific configurations to be applied
// @tid_conf: Configuration change info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_tid_config {
    pub peer: *const u8,
    pub n_tid_conf: u32,
    pub __counted_by(n_tid_conf): cfg80211_tid_cfg tid_conf[],
}

//
// struct cfg80211_fils_aad - FILS AAD data
// @macaddr: STA MAC address
// @kek: FILS KEK
// @kek_len: FILS KEK length
// @snonce: STA Nonce
// @anonce: AP Nonce
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_fils_aad {
    pub macaddr: *const u8,
    pub kek: *const u8,
    pub kek_len: u8,
    pub snonce: *const u8,
    pub anonce: *const u8,
}

//
// struct cfg80211_set_hw_timestamp - enable/disable HW timestamping
// @macaddr: peer MAC address. NULL to enable/disable HW timestamping for all
// addresses.
// @enable: if set, enable HW timestamping for the specified MAC address.
// Otherwise disable HW timestamping for the specified MAC address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_set_hw_timestamp {
    pub macaddr: *const u8,
    pub enable: bool,
}

//
// cfg80211_get_chandef_type - return old channel type from chandef
// @chandef: the channel definition
//
// Return: The old channel type (NOHT, HT20, HT40+/-) from a given
// chandef, which must have a bandwidth allowing this conversion.
//
// cfg80211_chandef_create - create channel definition using channel type
// @chandef: the channel definition struct to fill
// @channel: the control channel
// @chantype: the channel type
//
// Given a channel type, create a channel definition.
//
// cfg80211_chandef_identical - check if two channel definitions are identical
// @chandef1: first channel definition
// @chandef2: second channel definition
//
// Return: %true if the channels defined by the channel definitions are
// identical, %false otherwise.
//
// cfg80211_chandef_is_edmg - check if chandef represents an EDMG channel
//
// @chandef: the channel definition
//
// Return: %true if EDMG defined, %false otherwise.
//
// cfg80211_chandef_is_s1g - check if chandef represents an S1G channel
// @chandef: the channel definition
//
// Return: %true if S1G.
//
// cfg80211_chandef_compatible - check if two channel definitions are compatible
// @chandef1: first channel definition
// @chandef2: second channel definition
//
// Return: %NULL if the given channel definitions are incompatible,
// chandef1 or chandef2 otherwise.
//
// nl80211_chan_width_to_mhz - get the channel width in MHz
// @chan_width: the channel width from &enum nl80211_chan_width
//
// Return: channel width in MHz if the chan_width from &enum nl80211_chan_width
// is valid. -1 otherwise.
//
extern "C" {
    pub fn nl80211_chan_width_to_mhz(chan_width: nl80211_chan_width) -> c_int;
}
//
// cfg80211_chandef_get_width - return chandef width in MHz
// @c: chandef to return bandwidth for
// Return: channel width in MHz for the given chandef; note that it returns
// 80 for 80+80 configurations
//
extern "C" {
    pub fn nl80211_chan_width_to_mhz(_arg: c->width) -> return;
}
//
// cfg80211_chandef_valid - check if a channel definition is valid
// @chandef: the channel definition to check
// Return: %true if the channel definition is valid. %false otherwise.
//
extern "C" {
    pub fn cfg80211_chandef_valid(chandef: *const cfg80211_chan_def) -> bool;
}
//
// cfg80211_chandef_usable - check if secondary channels can be used
// @wiphy: the wiphy to validate against
// @chandef: the channel definition to check
// @prohibited_flags: the regulatory channel flags that must not be set
// Return: %true if secondary channels are usable. %false otherwise.
//
// cfg80211_chandef_dfs_required - checks if radar detection is required
// @wiphy: the wiphy to validate against
// @chandef: the channel definition to check
// @iftype: the interface type as specified in &enum nl80211_iftype
// Returns:
// 1 if radar detection is required, 0 if it is not, < 0 on error
//
// cfg80211_chandef_dfs_usable - checks if chandef is DFS usable and we
// can/need start CAC on such channel
// @wiphy: the wiphy to validate against
// @chandef: the channel definition to check
//
// Return: true if all channels available and at least
// one channel requires CAC (NL80211_DFS_USABLE)
//
// cfg80211_chandef_dfs_cac_time - get the DFS CAC time (in ms) for given
// channel definition
// @wiphy: the wiphy to validate against
// @chandef: the channel definition to check
//
// Returns: DFS CAC time (in ms) which applies for this channel definition
//
// cfg80211_chandef_primary - calculate primary 40/80/160 MHz freq
// @chandef: chandef to calculate for
// @primary_chan_width: primary channel width to calculate center for
// @punctured: punctured sub-channel bitmap, will be recalculated
// according to the new bandwidth, can be %NULL
//
// Returns: the primary 40/80/160 MHz channel center frequency, or -1
// for errors, updating the punctured bitmap
//
// cfg80211_chandef_npca_valid - check that NPCA information is valid
// @wiphy: the wiphy to check for, for channel pointer lookup
// @chandef: the BSS channel chandef to check against
// @npca: NPCA information, can be %NULL in which case this
// always returns %true
//
// Note that DBE must not have been configured into the chandef yet
// before checking NPCA, i.e. @chandef must represent the BSS channel.
//
// Returns: %true if the NPCA channel and puncturing bitmap are valid
// according to the chandef, %false otherwise
//
// cfg80211_chandef_add_npca - parse and add NPCA information to chandef
// @wiphy: the wiphy this will be used for, for channel pointer lookup
// @chandef: the chandef to modify, must be a valid chandef without NPCA
// @npca: the NPCA information, can be %NULL
//
// Returns: 0 if the NPCA information was added and the resulting
// chandef is valid, a negative error code on errors
//
// cfg80211_chandef_add_dbe - parse and add DBE information to chandef
// @chandef: the chandef to expand
// @dbe: the DBE information, must be size-checked if not %NULL
//
// Returns: 0 for success, a negative error code otherwise
//
// nl80211_send_chandef - sends the channel definition.
// @msg: the msg to send channel definition
// @chandef: the channel definition to check
//
// Returns: 0 if sent the channel definition to msg, < 0 on error
//
extern "C" {
    pub fn nl80211_send_chandef(msg: *mut sk_buff, chandef: *const cfg80211_chan_def) -> c_int;
}
//
// ieee80211_chandef_max_power - maximum transmission power for the chandef
//
// In some regulations, the transmit power may depend on the configured channel
// bandwidth which may be defined as dBm/MHz. This function returns the actual
// max_power for non-standard (20 MHz) channels.
//
// @chandef: channel definition for the channel
//
// Returns: maximum allowed transmission power in dBm for the chandef
//
// cfg80211_chandef_s1g_pri_width - return S1G primary width in MHz
//
// An S1G interface may have a primary channel width of either 1
// or 2MHz depending on whether chandef::s1g_primary_2mhz is set.
//
// Note: There is _always_ a 1MHz primary subchannel, regardless
// of the primary width. So chandef::chan always points to this
// 1MHz primary channel.
//
// @chandef: the chandef to use
//
// Returns: width in MHz of the S1G primary channel in use
//
// cfg80211_any_usable_channels - check for usable channels
// @wiphy: the wiphy to check for
// @band_mask: which bands to check on
// @prohibited_flags: which channels to not consider usable,
// %IEEE80211_CHAN_DISABLED is always taken into account
//
// Return: %true if usable channels found, %false otherwise
//
// enum survey_info_flags - survey information flags
//
// @SURVEY_INFO_NOISE_DBM: noise (in dBm) was filled in
// @SURVEY_INFO_IN_USE: channel is currently being used
// @SURVEY_INFO_TIME: active time (in ms) was filled in
// @SURVEY_INFO_TIME_BUSY: busy time was filled in
// @SURVEY_INFO_TIME_EXT_BUSY: extension channel busy time was filled in
// @SURVEY_INFO_TIME_RX: receive time was filled in
// @SURVEY_INFO_TIME_TX: transmit time was filled in
// @SURVEY_INFO_TIME_SCAN: scan time was filled in
// @SURVEY_INFO_TIME_BSS_RX: local BSS receive time was filled in
//
// Used by the driver to indicate which info in &struct survey_info
// it has filled in during the get_survey().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum survey_info_flags {
    SURVEY_INFO_NOISE_DBM		= BIT(0),
    SURVEY_INFO_IN_USE		= BIT(1),
    SURVEY_INFO_TIME		= BIT(2),
    SURVEY_INFO_TIME_BUSY		= BIT(3),
    SURVEY_INFO_TIME_EXT_BUSY	= BIT(4),
    SURVEY_INFO_TIME_RX		= BIT(5),
    SURVEY_INFO_TIME_TX		= BIT(6),
    SURVEY_INFO_TIME_SCAN		= BIT(7),
    SURVEY_INFO_TIME_BSS_RX		= BIT(8),
}

//
// struct survey_info - channel survey response
//
// @channel: the channel this survey record reports, may be %NULL for a single
// record to report global statistics
// @filled: bitflag of flags from &enum survey_info_flags
// @noise: channel noise in dBm. This and all following fields are
// optional
// @time: amount of time in ms the radio was turn on (on the channel)
// @time_busy: amount of time the primary channel was sensed busy
// @time_ext_busy: amount of time the extension channel was sensed busy
// @time_rx: amount of time the radio spent receiving data
// @time_tx: amount of time the radio spent transmitting data
// @time_scan: amount of time the radio spent for scanning
// @time_bss_rx: amount of time the radio spent receiving data on a local BSS
//
// Used by dump_survey() to report back per-channel survey information.
//
// This structure can later be expanded with things like
// channel duty cycle etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct survey_info {
    pub channel: *mut ieee80211_channel,
    pub time: u64,
    pub time_busy: u64,
    pub time_ext_busy: u64,
    pub time_rx: u64,
    pub time_tx: u64,
    pub time_scan: u64,
    pub time_bss_rx: u64,
    pub filled: u32,
    pub noise: i8,
}

pub const CFG80211_MAX_NUM_AKM_SUITES: c_int = 10;
//
// struct cfg80211_crypto_settings - Crypto settings
// @wpa_versions: indicates which, if any, WPA versions are enabled
// (from enum nl80211_wpa_versions)
// @cipher_group: group key cipher suite (or 0 if unset)
// @n_ciphers_pairwise: number of AP supported unicast ciphers
// @ciphers_pairwise: unicast key cipher suites
// @n_akm_suites: number of AKM suites
// @akm_suites: AKM suites
// @control_port: Whether user space controls IEEE 802.1X port, i.e.,
// sets/clears %NL80211_STA_FLAG_AUTHORIZED. If true, the driver is
// required to assume that the port is unauthorized until authorized by
// user space. Otherwise, port is marked authorized by default.
// @control_port_ethertype: the control port protocol that should be
// allowed through even on unauthorized ports
// @control_port_no_encrypt: TRUE to prevent encryption of control port
// protocol frames.
// @control_port_over_nl80211: TRUE if userspace expects to exchange control
// port frames over NL80211 instead of the network interface.
// @control_port_no_preauth: disables pre-auth rx over the nl80211 control
// port for mac80211
// @psk: PSK (for devices supporting 4-way-handshake offload)
// @sae_pwd: password for SAE authentication (for devices supporting SAE
// offload)
// @sae_pwd_len: length of SAE password (for devices supporting SAE offload)
// @sae_pwe: The mechanisms allowed for SAE PWE derivation:
//
// NL80211_SAE_PWE_UNSPECIFIED
// Not-specified, used to indicate userspace did not specify any
// preference. The driver should follow its internal policy in
// such a scenario.
//
// NL80211_SAE_PWE_HUNT_AND_PECK
// Allow hunting-and-pecking loop only
//
// NL80211_SAE_PWE_HASH_TO_ELEMENT
// Allow hash-to-element only
//
// NL80211_SAE_PWE_BOTH
// Allow either hunting-and-pecking loop or hash-to-element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_crypto_settings {
    pub wpa_versions: u32,
    pub cipher_group: u32,
    pub n_ciphers_pairwise: c_int,
    pub ciphers_pairwise: [u32; NL80211_MAX_NR_CIPHER_SUITES],
    pub n_akm_suites: c_int,
    pub akm_suites: [u32; CFG80211_MAX_NUM_AKM_SUITES],
    pub control_port: bool,
    pub control_port_ethertype: __be16,
    pub control_port_no_encrypt: bool,
    pub control_port_over_nl80211: bool,
    pub control_port_no_preauth: bool,
    pub psk: *const u8,
    pub sae_pwd: *const u8,
    pub sae_pwd_len: u8,
    pub sae_pwe: nl80211_sae_pwe_mechanism,
}

//
// struct cfg80211_mbssid_config - AP settings for multi bssid
//
// @tx_wdev: pointer to the transmitted interface in the MBSSID set
// @tx_link_id: link ID of the transmitted profile in an MLD.
// @index: index of this AP in the multi bssid group.
// @ema: set to true if the beacons should be sent out in EMA mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_mbssid_config {
    pub tx_wdev: *mut wireless_dev,
    pub tx_link_id: u8,
    pub index: u8,
    pub ema: bool,
}

//
// struct cfg80211_mbssid_elems - Multiple BSSID elements
//
// @cnt: Number of elements in array %elems.
//
// @elem: Array of multiple BSSID element(s) to be added into Beacon frames.
// @elem.data: Data for multiple BSSID elements.
// @elem.len: Length of data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_mbssid_elems {
    pub cnt: u8,
    pub data: *const u8,
    pub len: usize,
    pub __counted_by(cnt): } elem[],
}

//
// struct cfg80211_rnr_elems - Reduced neighbor report (RNR) elements
//
// @cnt: Number of elements in array %elems.
//
// @elem: Array of RNR element(s) to be added into Beacon frames.
// @elem.data: Data for RNR elements.
// @elem.len: Length of data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_rnr_elems {
    pub cnt: u8,
    pub data: *const u8,
    pub len: usize,
    pub __counted_by(cnt): } elem[],
}

//
// struct cfg80211_beacon_data - beacon data
// @link_id: the link ID for the AP MLD link sending this beacon
// @head: head portion of beacon (before TIM IE)
// or %NULL if not changed
// @tail: tail portion of beacon (after TIM IE)
// or %NULL if not changed
// @head_len: length of @head
// @tail_len: length of @tail
// @beacon_ies: extra information element(s) to add into Beacon frames or %NULL
// @beacon_ies_len: length of beacon_ies in octets
// @proberesp_ies: extra information element(s) to add into Probe Response
// frames or %NULL
// @proberesp_ies_len: length of proberesp_ies in octets
// @assocresp_ies: extra information element(s) to add into (Re)Association
// Response frames or %NULL
// @assocresp_ies_len: length of assocresp_ies in octets
// @probe_resp_len: length of probe response template (@probe_resp)
// @probe_resp: probe response template (AP mode only)
// @mbssid_ies: multiple BSSID elements
// @rnr_ies: reduced neighbor report elements
// @ftm_responder: enable FTM responder functionality; -1 for no change
// (which also implies no change in LCI/civic location data)
// @lci: Measurement Report element content, starting with Measurement Token
// (measurement type 8)
// @civicloc: Measurement Report element content, starting with Measurement
// Token (measurement type 11)
// @lci_len: LCI data length
// @civicloc_len: Civic location data length
// @he_bss_color: BSS Color settings
// @he_bss_color_valid: indicates whether bss color
// attribute is present in beacon data or not.
// @ht_required: stations must support HT
// @vht_required: stations must support VHT
// @ht_oper: HT operation element (or %NULL if HT isn't enabled)
// @vht_oper: VHT operation element (or %NULL if VHT isn't enabled)
// @he_oper: HE operation IE (or %NULL if HE isn't enabled)
// @eht_oper: EHT operation IE (or %NULL if EHT isn't enabled)
// @uhr_oper: UHR operation (or %NULL if UHR isn't enabled)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_beacon_data {
    pub link_id: c_uint,
    pub tail: *const *const u8 head,,
    pub beacon_ies: *const u8,
    pub proberesp_ies: *const u8,
    pub assocresp_ies: *const u8,
    pub probe_resp: *const u8,
    pub lci: *const u8,
    pub civicloc: *const u8,
    pub mbssid_ies: *mut cfg80211_mbssid_elems,
    pub rnr_ies: *mut cfg80211_rnr_elems,
    pub ftm_responder: i8,
    pub tail_len: size_t head_len,,
    pub beacon_ies_len: usize,
    pub proberesp_ies_len: usize,
    pub assocresp_ies_len: usize,
    pub probe_resp_len: usize,
    pub lci_len: usize,
    pub civicloc_len: usize,
    pub he_bss_color: cfg80211_he_bss_color,
    pub he_bss_color_valid: bool,
    pub vht_required: bool ht_required,,
    pub ht_oper: *const ieee80211_ht_operation,
    pub vht_oper: *const ieee80211_vht_operation,
    pub he_oper: *const ieee80211_he_operation,
    pub eht_oper: *const ieee80211_eht_operation,
    pub uhr_oper: *const ieee80211_uhr_operation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_address {
    pub addr: [u8; ETH_ALEN],
}

//
// struct cfg80211_acl_data - Access control list data
//
// @acl_policy: ACL policy to be applied on the station's
// entry specified by mac_addr
// @n_acl_entries: Number of MAC address entries passed
// @mac_addrs: List of MAC addresses of stations to be used for ACL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_acl_data {
    pub acl_policy: nl80211_acl_policy,
    pub n_acl_entries: c_int,
// Keep it last
    pub __counted_by(n_acl_entries): mac_address mac_addrs[],
}

//
// struct cfg80211_fils_discovery - FILS discovery parameters from
// IEEE Std 802.11ai-2016, Annex C.3 MIB detail.
//
// @update: Set to true if the feature configuration should be updated.
// @min_interval: Minimum packet interval in TUs (0 - 10000)
// @max_interval: Maximum packet interval in TUs (0 - 10000)
// @tmpl_len: Template length
// @tmpl: Template data for FILS discovery frame including the action
// frame headers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_fils_discovery {
    pub update: bool,
    pub min_interval: u32,
    pub max_interval: u32,
    pub tmpl_len: usize,
    pub tmpl: *const u8,
}

//
// struct cfg80211_unsol_bcast_probe_resp - Unsolicited broadcast probe
// response parameters in 6GHz.
//
// @update: Set to true if the feature configuration should be updated.
// @interval: Packet interval in TUs. Maximum allowed is 20 TU, as mentioned
// in IEEE P802.11ax/D6.0 26.17.2.3.2 - AP behavior for fast passive
// scanning
// @tmpl_len: Template length
// @tmpl: Template data for probe response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_unsol_bcast_probe_resp {
    pub update: bool,
    pub interval: u32,
    pub tmpl_len: usize,
    pub tmpl: *const u8,
}

//
// struct cfg80211_s1g_short_beacon - S1G short beacon data.
//
// @update: Set to true if the feature configuration should be updated.
// @short_head: Short beacon head.
// @short_tail: Short beacon tail.
// @short_head_len: Short beacon head len.
// @short_tail_len: Short beacon tail len.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_s1g_short_beacon {
    pub update: bool,
    pub short_head: *const u8,
    pub short_tail: *const u8,
    pub short_head_len: usize,
    pub short_tail_len: usize,
}

//
// struct cfg80211_ap_settings - AP configuration
//
// Used to configure an AP interface.
//
// @chandef: defines the channel to use
// @beacon: beacon data
// @beacon_interval: beacon interval
// @dtim_period: DTIM period
// @ssid: SSID to be used in the BSS (note: may be %NULL if not provided from
// user space)
// @ssid_len: length of @ssid
// @hidden_ssid: whether to hide the SSID in Beacon/Probe Response frames
// @crypto: crypto settings
// @privacy: the BSS uses privacy
// @auth_type: Authentication type (algorithm)
// @inactivity_timeout: time in seconds to determine station's inactivity.
// @p2p_ctwindow: P2P CT Window
// @p2p_opp_ps: P2P opportunistic PS
// @acl: ACL configuration used by the drivers which has support for
// MAC address based access control
// @pbss: If set, start as a PCP instead of AP. Relevant for DMG
// networks.
// @beacon_rate: bitrate to be used for beacons
// @ht_cap: HT capabilities (or %NULL if HT isn't enabled)
// @vht_cap: VHT capabilities (or %NULL if VHT isn't enabled)
// @he_cap: HE capabilities (or %NULL if HE isn't enabled)
// @eht_cap: EHT capabilities (or %NULL if EHT isn't enabled)
// @twt_responder: Enable Target Wait Time
// @flags: flags, as defined in &enum nl80211_ap_settings_flags
// @he_obss_pd: OBSS Packet Detection settings
// @fils_discovery: FILS discovery transmission parameters
// @unsol_bcast_probe_resp: Unsolicited broadcast probe response parameters
// @mbssid_config: AP settings for multiple bssid
// @s1g_long_beacon_period: S1G long beacon period
// @s1g_short_beacon: S1G short beacon data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ap_settings {
    pub chandef: cfg80211_chan_def,
    pub beacon: cfg80211_beacon_data,
    pub dtim_period: int beacon_interval,,
    pub ssid: *const u8,
    pub ssid_len: usize,
    pub hidden_ssid: nl80211_hidden_ssid,
    pub crypto: cfg80211_crypto_settings,
    pub privacy: bool,
    pub auth_type: nl80211_auth_type,
    pub inactivity_timeout: c_int,
    pub p2p_ctwindow: u8,
    pub p2p_opp_ps: bool,
    pub acl: *const cfg80211_acl_data,
    pub pbss: bool,
    pub beacon_rate: cfg80211_bitrate_mask,
    pub ht_cap: *const ieee80211_ht_cap,
    pub vht_cap: *const ieee80211_vht_cap,
    pub he_cap: *const ieee80211_he_cap_elem,
    pub eht_cap: *const ieee80211_eht_cap_elem,
    pub twt_responder: bool,
    pub flags: u32,
    pub he_obss_pd: ieee80211_he_obss_pd,
    pub fils_discovery: cfg80211_fils_discovery,
    pub unsol_bcast_probe_resp: cfg80211_unsol_bcast_probe_resp,
    pub mbssid_config: cfg80211_mbssid_config,
    pub s1g_long_beacon_period: u8,
    pub s1g_short_beacon: cfg80211_s1g_short_beacon,
}

//
// struct cfg80211_ap_update - AP configuration update
//
// Subset of &struct cfg80211_ap_settings, for updating a running AP.
//
// @beacon: beacon data
// @fils_discovery: FILS discovery transmission parameters
// @unsol_bcast_probe_resp: Unsolicited broadcast probe response parameters
// @s1g_short_beacon: S1G short beacon data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ap_update {
    pub beacon: cfg80211_beacon_data,
    pub fils_discovery: cfg80211_fils_discovery,
    pub unsol_bcast_probe_resp: cfg80211_unsol_bcast_probe_resp,
    pub s1g_short_beacon: cfg80211_s1g_short_beacon,
}

//
// struct cfg80211_csa_settings - channel switch settings
//
// Used for channel switch
//
// @chandef: defines the channel to use after the switch
// @beacon_csa: beacon data while performing the switch
// @counter_offsets_beacon: offsets of the counters within the beacon (tail)
// @counter_offsets_presp: offsets of the counters within the probe response
// @n_counter_offsets_beacon: number of csa counters the beacon (tail)
// @n_counter_offsets_presp: number of csa counters in the probe response
// @beacon_after: beacon data to be used on the new channel
// @unsol_bcast_probe_resp: Unsolicited broadcast probe response parameters
// @radar_required: whether radar detection is required on the new channel
// @block_tx: whether transmissions should be blocked while changing
// @count: number of beacons until switch
// @link_id: defines the link on which channel switch is expected during
// MLO. 0 in case of non-MLO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_csa_settings {
    pub chandef: cfg80211_chan_def,
    pub beacon_csa: cfg80211_beacon_data,
    pub counter_offsets_beacon: *const u16,
    pub counter_offsets_presp: *const u16,
    pub n_counter_offsets_beacon: c_uint,
    pub n_counter_offsets_presp: c_uint,
    pub beacon_after: cfg80211_beacon_data,
    pub unsol_bcast_probe_resp: cfg80211_unsol_bcast_probe_resp,
    pub radar_required: bool,
    pub block_tx: bool,
    pub count: u8,
    pub link_id: u8,
}

//
// struct cfg80211_color_change_settings - color change settings
//
// Used for bss color change
//
// @beacon_color_change: beacon data while performing the color countdown
// @counter_offset_beacon: offsets of the counters within the beacon (tail)
// @counter_offset_presp: offsets of the counters within the probe response
// @beacon_next: beacon data to be used after the color change
// @unsol_bcast_probe_resp: Unsolicited broadcast probe response parameters
// @count: number of beacons until the color change
// @color: the color used after the change
// @link_id: defines the link on which color change is expected during MLO.
// 0 in case of non-MLO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_color_change_settings {
    pub beacon_color_change: cfg80211_beacon_data,
    pub counter_offset_beacon: u16,
    pub counter_offset_presp: u16,
    pub beacon_next: cfg80211_beacon_data,
    pub unsol_bcast_probe_resp: cfg80211_unsol_bcast_probe_resp,
    pub count: u8,
    pub color: u8,
    pub link_id: u8,
}

//
// struct iface_combination_params - input parameters for interface combinations
//
// Used to pass interface combination parameters
//
// @radio_idx: wiphy radio index or -1 for global
// @num_different_channels: the number of different channels we want
// to use for verification
// @radar_detect: a bitmap where each bit corresponds to a channel
// width where radar detection is needed, as in the definition of
// &struct ieee80211_iface_combination.@radar_detect_widths
// @iftype_num: array with the number of interfaces of each interface
// type.  The index is the interface type as specified in &enum
// nl80211_iftype.
// @new_beacon_int: set this to the beacon interval of a new interface
// that's not operating yet, if such is to be checked as part of
// the verification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iface_combination_params {
    pub radio_idx: c_int,
    pub num_different_channels: c_int,
    pub radar_detect: u8,
    pub iftype_num: [c_int; NUM_NL80211_IFTYPES],
    pub new_beacon_int: u32,
}

//
// enum station_parameters_apply_mask - station parameter values to apply
// @STATION_PARAM_APPLY_UAPSD: apply new uAPSD parameters (uapsd_queues, max_sp)
// @STATION_PARAM_APPLY_CAPABILITY: apply new capability
// @STATION_PARAM_APPLY_PLINK_STATE: apply new plink state
//
// Not all station parameters have in-band "no change" signalling,
// for those that don't these flags will are used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum station_parameters_apply_mask {
    STATION_PARAM_APPLY_UAPSD = BIT(0),
    STATION_PARAM_APPLY_CAPABILITY = BIT(1),
    STATION_PARAM_APPLY_PLINK_STATE = BIT(2),
}

//
// struct sta_txpwr - station txpower configuration
//
// Used to configure txpower for station.
//
// @power: tx power (in dBm) to be used for sending data traffic. If tx power
// is not provided, the default per-interface tx power setting will be
// overriding. Driver should be picking up the lowest tx power, either tx
// power per-interface or per-station.
// @type: In particular if TPC %type is NL80211_TX_POWER_LIMITED then tx power
// will be less than or equal to specified from userspace, whereas if TPC
// %type is NL80211_TX_POWER_AUTOMATIC then it indicates default tx power.
// NL80211_TX_POWER_FIXED is not a valid configuration option for
// per peer TPC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_txpwr {
    pub power: i16,
    pub type: nl80211_tx_power_setting,
}

//
// struct link_station_parameters - link station parameters
//
// Used to change and create a new link station.
//
// @mld_mac: MAC address of the station
// @link_id: the link id (-1 for non-MLD station)
// @link_mac: MAC address of the link
// @supported_rates: supported rates in IEEE 802.11 format
// (or NULL for no change)
// @supported_rates_len: number of supported rates
// @ht_capa: HT capabilities of station
// @vht_capa: VHT capabilities of station
// @opmode_notif: operating mode field from Operating Mode Notification
// @opmode_notif_used: information if operating mode field is used
// @he_capa: HE capabilities of station
// @he_capa_len: the length of the HE capabilities
// @txpwr: transmit power for an associated station
// @txpwr_set: txpwr field is set
// @he_6ghz_capa: HE 6 GHz Band capabilities of station
// @eht_capa: EHT capabilities of station
// @eht_capa_len: the length of the EHT capabilities
// @s1g_capa: S1G capabilities of station
// @uhr_capa: UHR capabilities of the station
// @uhr_capa_len: the length of the UHR capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_station_parameters {
    pub mld_mac: *const u8,
    pub link_id: c_int,
    pub link_mac: *const u8,
    pub supported_rates: *const u8,
    pub supported_rates_len: u8,
    pub ht_capa: *const ieee80211_ht_cap,
    pub vht_capa: *const ieee80211_vht_cap,
    pub opmode_notif: u8,
    pub opmode_notif_used: bool,
    pub he_capa: *const ieee80211_he_cap_elem,
    pub he_capa_len: u8,
    pub txpwr: sta_txpwr,
    pub txpwr_set: bool,
    pub he_6ghz_capa: *const ieee80211_he_6ghz_capa,
    pub eht_capa: *const ieee80211_eht_cap_elem,
    pub eht_capa_len: u8,
    pub s1g_capa: *const ieee80211_s1g_cap,
    pub uhr_capa: *const ieee80211_uhr_cap,
    pub uhr_capa_len: u8,
}

//
// struct link_station_del_parameters - link station deletion parameters
//
// Used to delete a link station entry (or all stations).
//
// @mld_mac: MAC address of the station
// @link_id: the link id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_station_del_parameters {
    pub mld_mac: *const u8,
    pub link_id: u32,
}

//
// struct cfg80211_ttlm_params: TID to link mapping parameters
//
// Used for setting a TID to link mapping.
//
// @dlink: Downlink TID to link mapping, as defined in section 9.4.2.314
// (TID-To-Link Mapping element) in Draft P802.11be_D4.0.
// @ulink: Uplink TID to link mapping, as defined in section 9.4.2.314
// (TID-To-Link Mapping element) in Draft P802.11be_D4.0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ttlm_params {
    pub dlink: [u16; 8],
    pub ulink: [u16; 8],
}

//
// struct station_parameters - station parameters
//
// Used to change and create a new station.
//
// @vlan: vlan interface station should belong to
// @sta_flags_mask: station flags that changed
// (bitmask of BIT(%NL80211_STA_FLAG_...))
// @sta_flags_set: station flags values
// (bitmask of BIT(%NL80211_STA_FLAG_...))
// @listen_interval: listen interval or -1 for no change
// @aid: AID or zero for no change
// @vlan_id: VLAN ID for station (if nonzero)
// @peer_aid: mesh peer AID or zero for no change
// @plink_action: plink action to take
// @plink_state: set the peer link state for a station
// @uapsd_queues: bitmap of queues configured for uapsd. same format
// as the AC bitmap in the QoS info field
// @max_sp: max Service Period. same format as the MAX_SP in the
// QoS info field (but already shifted down)
// @sta_modify_mask: bitmap indicating which parameters changed
// (for those that don't have a natural "no change" value),
// see &enum station_parameters_apply_mask
// @local_pm: local link-specific mesh power save mode (no change when set
// to unknown)
// @capability: station capability
// @ext_capab: extended capabilities of the station
// @ext_capab_len: number of extended capabilities
// @supported_channels: supported channels in IEEE 802.11 format
// @supported_channels_len: number of supported channels
// @supported_oper_classes: supported oper classes in IEEE 802.11 format
// @supported_oper_classes_len: number of supported operating classes
// @support_p2p_ps: information if station supports P2P PS mechanism
// @airtime_weight: airtime scheduler weight for this station
// @eml_cap_present: Specifies if EML capabilities field (@eml_cap) is
// present/updated
// @eml_cap: EML capabilities of this station
// @link_sta_params: link related params.
// @epp_peer: EPP peer indication
// @nmi_mac: MAC address of the NMI station of the NAN peer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct station_parameters {
    pub vlan: *mut net_device,
    pub sta_flags_set: u32 sta_flags_mask,,
    pub sta_modify_mask: u32,
    pub listen_interval: c_int,
    pub aid: u16,
    pub vlan_id: u16,
    pub peer_aid: u16,
    pub plink_action: u8,
    pub plink_state: u8,
    pub uapsd_queues: u8,
    pub max_sp: u8,
    pub local_pm: nl80211_mesh_power_mode,
    pub capability: u16,
    pub ext_capab: *const u8,
    pub ext_capab_len: u8,
    pub supported_channels: *const u8,
    pub supported_channels_len: u8,
    pub supported_oper_classes: *const u8,
    pub supported_oper_classes_len: u8,
    pub support_p2p_ps: c_int,
    pub airtime_weight: u16,
    pub eml_cap_present: bool,
    pub eml_cap: u16,
    pub link_sta_params: link_station_parameters,
    pub epp_peer: bool,
    pub nmi_mac: *const u8,
}

//
// struct station_del_parameters - station deletion parameters
//
// Used to delete a station entry (or all stations).
//
// @mac: MAC address of the station to remove or NULL to remove all stations
// @subtype: Management frame subtype to use for indicating removal
// (10 = Disassociation, 12 = Deauthentication)
// @reason_code: Reason code for the Disassociation/Deauthentication frame
// @link_id: Link ID indicating a link that stations to be flushed must be
// using; valid only for MLO, but can also be -1 for MLO to really
// remove all stations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct station_del_parameters {
    pub mac: *const u8,
    pub subtype: u8,
    pub reason_code: u16,
    pub link_id: c_int,
}

//
// enum cfg80211_station_type - the type of station being modified
// @CFG80211_STA_AP_CLIENT: client of an AP interface
// @CFG80211_STA_AP_CLIENT_UNASSOC: client of an AP interface that is still
// unassociated (update properties for this type of client is permitted)
// @CFG80211_STA_AP_MLME_CLIENT: client of an AP interface that has
// the AP MLME in the device
// @CFG80211_STA_AP_STA: AP station on managed interface
// @CFG80211_STA_IBSS: IBSS station
// @CFG80211_STA_TDLS_PEER_SETUP: TDLS peer on managed interface (dummy entry
// while TDLS setup is in progress, it moves out of this state when
// being marked authorized; use this only if TDLS with external setup is
// supported/used)
// @CFG80211_STA_TDLS_PEER_ACTIVE: TDLS peer on managed interface (active
// entry that is operating, has been marked authorized by userspace)
// @CFG80211_STA_MESH_PEER_KERNEL: peer on mesh interface (kernel managed)
// @CFG80211_STA_MESH_PEER_USER: peer on mesh interface (user managed)
// @CFG80211_STA_NAN_MGMT: NAN management interface station
// @CFG80211_STA_NAN_DATA: NAN data path station
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_station_type {
    CFG80211_STA_AP_CLIENT,
    CFG80211_STA_AP_CLIENT_UNASSOC,
    CFG80211_STA_AP_MLME_CLIENT,
    CFG80211_STA_AP_STA,
    CFG80211_STA_IBSS,
    CFG80211_STA_TDLS_PEER_SETUP,
    CFG80211_STA_TDLS_PEER_ACTIVE,
    CFG80211_STA_MESH_PEER_KERNEL,
    CFG80211_STA_MESH_PEER_USER,
    CFG80211_STA_NAN_MGMT,
    CFG80211_STA_NAN_DATA,
}

//
// cfg80211_check_station_change - validate parameter changes
// @wiphy: the wiphy this operates on
// @params: the new parameters for a station
// @statype: the type of station being modified
//
// Utility function for the @change_station driver method. Call this function
// with the appropriate station type looking up the station (and checking that
// it exists). It will verify whether the station change is acceptable.
//
// Return: 0 if the change is acceptable, otherwise an error code. Note that
// it may modify the parameters for backward compatibility reasons, so don't
// use them before calling this.
//
// enum rate_info_flags - bitrate info flags
//
// Used by the driver to indicate the specific rate transmission
// type for 802.11n transmissions.
//
// @RATE_INFO_FLAGS_MCS: mcs field filled with HT MCS
// @RATE_INFO_FLAGS_VHT_MCS: mcs field filled with VHT MCS
// @RATE_INFO_FLAGS_SHORT_GI: 400ns guard interval
// @RATE_INFO_FLAGS_DMG: 60GHz MCS
// @RATE_INFO_FLAGS_HE_MCS: HE MCS information
// @RATE_INFO_FLAGS_EDMG: 60GHz MCS in EDMG mode
// @RATE_INFO_FLAGS_EXTENDED_SC_DMG: 60GHz extended SC MCS
// @RATE_INFO_FLAGS_EHT_MCS: EHT MCS information
// @RATE_INFO_FLAGS_S1G_MCS: MCS field filled with S1G MCS
// @RATE_INFO_FLAGS_UHR_MCS: UHR MCS information
// @RATE_INFO_FLAGS_UHR_ELR_MCS: UHR ELR MCS was used
// (set together with @RATE_INFO_FLAGS_UHR_MCS)
// @RATE_INFO_FLAGS_UHR_IM: UHR Interference Mitigation
// was used
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rate_info_flags {
    RATE_INFO_FLAGS_MCS			= BIT(0),
    RATE_INFO_FLAGS_VHT_MCS			= BIT(1),
    RATE_INFO_FLAGS_SHORT_GI		= BIT(2),
    RATE_INFO_FLAGS_DMG			= BIT(3),
    RATE_INFO_FLAGS_HE_MCS			= BIT(4),
    RATE_INFO_FLAGS_EDMG			= BIT(5),
    RATE_INFO_FLAGS_EXTENDED_SC_DMG		= BIT(6),
    RATE_INFO_FLAGS_EHT_MCS			= BIT(7),
    RATE_INFO_FLAGS_S1G_MCS			= BIT(8),
    RATE_INFO_FLAGS_UHR_MCS			= BIT(9),
    RATE_INFO_FLAGS_UHR_ELR_MCS		= BIT(10),
    RATE_INFO_FLAGS_UHR_IM			= BIT(11),
}

//
// enum rate_info_bw - rate bandwidth information
//
// Used by the driver to indicate the rate bandwidth.
//
// @RATE_INFO_BW_5: 5 MHz bandwidth
// @RATE_INFO_BW_10: 10 MHz bandwidth
// @RATE_INFO_BW_20: 20 MHz bandwidth
// @RATE_INFO_BW_40: 40 MHz bandwidth
// @RATE_INFO_BW_80: 80 MHz bandwidth
// @RATE_INFO_BW_160: 160 MHz bandwidth
// @RATE_INFO_BW_HE_RU: bandwidth determined by HE RU allocation
// @RATE_INFO_BW_320: 320 MHz bandwidth
// @RATE_INFO_BW_EHT_RU: bandwidth determined by EHT/UHR RU allocation
// @RATE_INFO_BW_1: 1 MHz bandwidth
// @RATE_INFO_BW_2: 2 MHz bandwidth
// @RATE_INFO_BW_4: 4 MHz bandwidth
// @RATE_INFO_BW_8: 8 MHz bandwidth
// @RATE_INFO_BW_16: 16 MHz bandwidth
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rate_info_bw {
    RATE_INFO_BW_20 = 0,
    RATE_INFO_BW_5,
    RATE_INFO_BW_10,
    RATE_INFO_BW_40,
    RATE_INFO_BW_80,
    RATE_INFO_BW_160,
    RATE_INFO_BW_HE_RU,
    RATE_INFO_BW_320,
    RATE_INFO_BW_EHT_RU,
    RATE_INFO_BW_1,
    RATE_INFO_BW_2,
    RATE_INFO_BW_4,
    RATE_INFO_BW_8,
    RATE_INFO_BW_16,
}

//
// struct rate_info - bitrate information
//
// Information about a receiving or transmitting bitrate
//
// @flags: bitflag of flags from &enum rate_info_flags
// @legacy: bitrate in 100kbit/s for 802.11abg
// @mcs: mcs index if struct describes an HT/VHT/HE/EHT/S1G/UHR rate
// @nss: number of streams (VHT & HE only)
// @bw: bandwidth (from &enum rate_info_bw)
// @he_gi: HE guard interval (from &enum nl80211_he_gi)
// @he_dcm: HE DCM value
// @he_ru_alloc: HE RU allocation (from &enum nl80211_he_ru_alloc,
// only valid if bw is %RATE_INFO_BW_HE_RU)
// @n_bonded_ch: In case of EDMG the number of bonded channels (1-4)
// @eht_gi: EHT guard interval (from &enum nl80211_eht_gi)
// @eht_ru_alloc: EHT RU allocation (from &enum nl80211_eht_ru_alloc,
// only valid if bw is %RATE_INFO_BW_EHT_RU)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_info {
    pub flags: u16,
    pub legacy: u16,
    pub mcs: u8,
    pub nss: u8,
    pub bw: u8,
    pub he_gi: u8,
    pub he_dcm: u8,
    pub he_ru_alloc: u8,
    pub n_bonded_ch: u8,
    pub eht_gi: u8,
    pub eht_ru_alloc: u8,
}

//
// enum bss_param_flags - bitrate info flags
//
// Used by the driver to indicate the specific rate transmission
// type for 802.11n transmissions.
//
// @BSS_PARAM_FLAGS_CTS_PROT: whether CTS protection is enabled
// @BSS_PARAM_FLAGS_SHORT_PREAMBLE: whether short preamble is enabled
// @BSS_PARAM_FLAGS_SHORT_SLOT_TIME: whether short slot time is enabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bss_param_flags {
    BSS_PARAM_FLAGS_CTS_PROT	= BIT(0),
    BSS_PARAM_FLAGS_SHORT_PREAMBLE	= BIT(1),
    BSS_PARAM_FLAGS_SHORT_SLOT_TIME	= BIT(2),
}

//
// struct sta_bss_parameters - BSS parameters for the attached station
//
// Information about the currently associated BSS
//
// @flags: bitflag of flags from &enum bss_param_flags
// @dtim_period: DTIM period for the BSS
// @beacon_interval: beacon interval
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_bss_parameters {
    pub flags: u8,
    pub dtim_period: u8,
    pub beacon_interval: u16,
}

//
// struct cfg80211_txq_stats - TXQ statistics for this TID
// @filled: bitmap of flags using the bits of &enum nl80211_txq_stats to
// indicate the relevant values in this struct are filled
// @backlog_bytes: total number of bytes currently backlogged
// @backlog_packets: total number of packets currently backlogged
// @flows: number of new flows seen
// @drops: total number of packets dropped
// @ecn_marks: total number of packets marked with ECN CE
// @overlimit: number of drops due to queue space overflow
// @overmemory: number of drops due to memory limit overflow
// @collisions: number of hash collisions
// @tx_bytes: total number of bytes dequeued
// @tx_packets: total number of packets dequeued
// @max_flows: maximum number of flows supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_txq_stats {
    pub filled: u32,
    pub backlog_bytes: u32,
    pub backlog_packets: u32,
    pub flows: u32,
    pub drops: u32,
    pub ecn_marks: u32,
    pub overlimit: u32,
    pub overmemory: u32,
    pub collisions: u32,
    pub tx_bytes: u32,
    pub tx_packets: u32,
    pub max_flows: u32,
}

//
// struct cfg80211_tid_stats - per-TID statistics
// @filled: bitmap of flags using the bits of &enum nl80211_tid_stats to
// indicate the relevant values in this struct are filled
// @rx_msdu: number of received MSDUs
// @tx_msdu: number of (attempted) transmitted MSDUs
// @tx_msdu_retries: number of retries (not counting the first) for
// transmitted MSDUs
// @tx_msdu_failed: number of failed transmitted MSDUs
// @txq_stats: TXQ statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_tid_stats {
    pub filled: u32,
    pub rx_msdu: u64,
    pub tx_msdu: u64,
    pub tx_msdu_retries: u64,
    pub tx_msdu_failed: u64,
    pub txq_stats: cfg80211_txq_stats,
}

pub const IEEE80211_MAX_CHAINS: c_int = 4;
//
// struct link_station_info - link station information
//
// Link station information filled by driver for get_station() and
// dump_station().
// @filled: bit flag of flags using the bits of &enum nl80211_sta_info to
// indicate the relevant values in this struct for them
// @connected_time: time(in secs) since a link of station is last connected
// @inactive_time: time since last activity for link station(tx/rx)
// in milliseconds
// @assoc_at: bootime (ns) of the last association of link of station
// @rx_bytes: bytes (size of MPDUs) received from this link of station
// @tx_bytes: bytes (size of MPDUs) transmitted to this link of station
// @signal: The signal strength, type depends on the wiphy's signal_type.
// For CFG80211_SIGNAL_TYPE_MBM, value is expressed in _dBm_.
// @signal_avg: Average signal strength, type depends on the wiphy's
// signal_type. For CFG80211_SIGNAL_TYPE_MBM, value is expressed in _dBm_
// @chains: bitmask for filled values in @chain_signal, @chain_signal_avg
// @chain_signal: per-chain signal strength of last received packet in dBm
// @chain_signal_avg: per-chain signal strength average in dBm
// @txrate: current unicast bitrate from this link of station
// @rxrate: current unicast bitrate to this link of station
// @rx_packets: packets (MSDUs & MMPDUs) received from this link of station
// @tx_packets: packets (MSDUs & MMPDUs) transmitted to this link of station
// @tx_retries: cumulative retry counts (MPDUs) for this link of station
// @tx_failed: number of failed transmissions (MPDUs) (retries exceeded, no ACK)
// @rx_dropped_misc:  Dropped for un-specified reason.
// @bss_param: current BSS parameters
// @beacon_loss_count: Number of times beacon loss event has triggered.
// @expected_throughput: expected throughput in kbps (including 802.11 headers)
// towards this station.
// @rx_beacon: number of beacons received from this peer
// @rx_beacon_signal_avg: signal strength average (in dBm) for beacons received
// from this peer
// @rx_duration: aggregate PPDU duration(usecs) for all the frames from a peer
// @tx_duration: aggregate PPDU duration(usecs) for all the frames to a peer
// @airtime_weight: current airtime scheduling weight
// @pertid: per-TID statistics, see &struct cfg80211_tid_stats, using the last
// (IEEE80211_NUM_TIDS) index for MSDUs not encapsulated in QoS-MPDUs.
// Note that this doesn't use the @filled bit, but is used if non-NULL.
// @ack_signal: signal strength (in dBm) of the last ACK frame.
// @avg_ack_signal: average rssi value of ack packet for the no of msdu's has
// been sent.
// @rx_mpdu_count: number of MPDUs received from this station
// @fcs_err_count: number of packets (MPDUs) received from this station with
// an FCS error. This counter should be incremented only when TA of the
// received packet with an FCS error matches the peer MAC address.
// @addr: For MLO STA connection, filled with address of the link of station.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_station_info {
    pub filled: u64,
    pub connected_time: u32,
    pub inactive_time: u32,
    pub assoc_at: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub signal: i8,
    pub signal_avg: i8,
    pub chains: u8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
    pub chain_signal_avg: [i8; IEEE80211_MAX_CHAINS],
    pub txrate: rate_info,
    pub rxrate: rate_info,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub tx_retries: u32,
    pub tx_failed: u32,
    pub rx_dropped_misc: u32,
    pub bss_param: sta_bss_parameters,
    pub beacon_loss_count: u32,
    pub expected_throughput: u32,
    pub tx_duration: u64,
    pub rx_duration: u64,
    pub rx_beacon: u64,
    pub rx_beacon_signal_avg: u8,
    pub airtime_weight: u16,
    pub ack_signal: i8,
    pub avg_ack_signal: i8,
    pub pertid: *mut cfg80211_tid_stats,
    pub rx_mpdu_count: u32,
    pub fcs_err_count: u32,
    pub __aligned(2): u8 addr[ETH_ALEN],
}

//
// struct station_info - station information
//
// Station information filled by driver for get_station() and dump_station.
//
// @filled: bitflag of flags using the bits of &enum nl80211_sta_info to
// indicate the relevant values in this struct for them
// @connected_time: time(in secs) since a station is last connected
// @inactive_time: time since last station activity (tx/rx) in milliseconds
// @assoc_at: bootime (ns) of the last association
// @rx_bytes: bytes (size of MPDUs) received from this station
// @tx_bytes: bytes (size of MPDUs) transmitted to this station
// @signal: The signal strength, type depends on the wiphy's signal_type.
// For CFG80211_SIGNAL_TYPE_MBM, value is expressed in _dBm_.
// @signal_avg: Average signal strength, type depends on the wiphy's signal_type.
// For CFG80211_SIGNAL_TYPE_MBM, value is expressed in _dBm_.
// @chains: bitmask for filled values in @chain_signal, @chain_signal_avg
// @chain_signal: per-chain signal strength of last received packet in dBm
// @chain_signal_avg: per-chain signal strength average in dBm
// @txrate: current unicast bitrate from this station
// @rxrate: current unicast bitrate to this station
// @rx_packets: packets (MSDUs & MMPDUs) received from this station
// @tx_packets: packets (MSDUs & MMPDUs) transmitted to this station
// @tx_retries: cumulative retry counts (MPDUs)
// @tx_failed: number of failed transmissions (MPDUs) (retries exceeded, no ACK)
// @rx_dropped_misc:  Dropped for un-specified reason.
// @bss_param: current BSS parameters
// @generation: generation number for nl80211 dumps.
// This number should increase every time the list of stations
// changes, i.e. when a station is added or removed, so that
// userspace can tell whether it got a consistent snapshot.
// @beacon_loss_count: Number of times beacon loss event has triggered.
// @assoc_req_ies: IEs from (Re)Association Request.
// This is used only when in AP mode with drivers that do not use
// user space MLME/SME implementation. The information is provided for
// the cfg80211_new_sta() calls to notify user space of the IEs.
// @assoc_req_ies_len: Length of assoc_req_ies buffer in octets.
// @sta_flags: station flags mask & values
// @t_offset: Time offset of the station relative to this host.
// @llid: mesh local link id
// @plid: mesh peer link id
// @plink_state: mesh peer link state
// @connected_to_gate: true if mesh STA has a path to mesh gate
// @connected_to_as: true if mesh STA has a path to authentication server
// @airtime_link_metric: mesh airtime link metric.
// @local_pm: local mesh STA power save mode
// @peer_pm: peer mesh STA power save mode
// @nonpeer_pm: non-peer mesh STA power save mode
// @expected_throughput: expected throughput in kbps (including 802.11 headers)
// towards this station.
// @rx_beacon: number of beacons received from this peer
// @rx_beacon_signal_avg: signal strength average (in dBm) for beacons received
// from this peer
// @rx_duration: aggregate PPDU duration(usecs) for all the frames from a peer
// @tx_duration: aggregate PPDU duration(usecs) for all the frames to a peer
// @airtime_weight: current airtime scheduling weight
// @pertid: per-TID statistics, see &struct cfg80211_tid_stats, using the last
// (IEEE80211_NUM_TIDS) index for MSDUs not encapsulated in QoS-MPDUs.
// Note that this doesn't use the @filled bit, but is used if non-NULL.
// @ack_signal: signal strength (in dBm) of the last ACK frame.
// @avg_ack_signal: average rssi value of ack packet for the no of msdu's has
// been sent.
// @rx_mpdu_count: number of MPDUs received from this station
// @fcs_err_count: number of packets (MPDUs) received from this station with
// an FCS error. This counter should be incremented only when TA of the
// received packet with an FCS error matches the peer MAC address.
// @mlo_params_valid: Indicates @assoc_link_id and @mld_addr fields are filled
// by driver. Drivers use this only in cfg80211_new_sta() calls when AP
// MLD's MLME/SME is offload to driver. Drivers won't fill this
// information in cfg80211_del_sta_sinfo(), get_station() and
// dump_station() callbacks.
// @assoc_link_id: Indicates MLO link ID of the AP, with which the station
// completed (re)association. This information filled for both MLO
// and non-MLO STA connections when the AP affiliated with an MLD.
// @mld_addr: For MLO STA connection, filled with MLD address of the station.
// For non-MLO STA connection, filled with all zeros.
// @assoc_resp_ies: IEs from (Re)Association Response.
// This is used only when in AP mode with drivers that do not use user
// space MLME/SME implementation. The information is provided only for the
// cfg80211_new_sta() calls to notify user space of the IEs. Drivers won't
// fill this information in cfg80211_del_sta_sinfo(), get_station() and
// dump_station() callbacks. User space needs this information to determine
// the accepted and rejected affiliated links of the connected station.
// @assoc_resp_ies_len: Length of @assoc_resp_ies buffer in octets.
// @valid_links: bitmap of valid links, or 0 for non-MLO. Drivers fill this
// information in cfg80211_new_sta(), cfg80211_del_sta_sinfo(),
// get_station() and dump_station() callbacks.
// @links: reference to Link sta entries for MLO STA, all link specific
// information is accessed through links[link_id].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct station_info {
    pub filled: u64,
    pub connected_time: u32,
    pub inactive_time: u32,
    pub assoc_at: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub signal: i8,
    pub signal_avg: i8,
    pub chains: u8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
    pub chain_signal_avg: [i8; IEEE80211_MAX_CHAINS],
    pub txrate: rate_info,
    pub rxrate: rate_info,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub tx_retries: u32,
    pub tx_failed: u32,
    pub rx_dropped_misc: u32,
    pub bss_param: sta_bss_parameters,
    pub sta_flags: nl80211_sta_flag_update,
    pub generation: c_int,
    pub beacon_loss_count: u32,
    pub assoc_req_ies: *const u8,
    pub assoc_req_ies_len: usize,
    pub t_offset: i64,
    pub llid: u16,
    pub plid: u16,
    pub plink_state: u8,
    pub connected_to_gate: u8,
    pub connected_to_as: u8,
    pub airtime_link_metric: u32,
    pub local_pm: nl80211_mesh_power_mode,
    pub peer_pm: nl80211_mesh_power_mode,
    pub nonpeer_pm: nl80211_mesh_power_mode,
    pub expected_throughput: u32,
    pub airtime_weight: u16,
    pub ack_signal: i8,
    pub avg_ack_signal: i8,
    pub pertid: *mut cfg80211_tid_stats,
    pub tx_duration: u64,
    pub rx_duration: u64,
    pub rx_beacon: u64,
    pub rx_beacon_signal_avg: u8,
    pub rx_mpdu_count: u32,
    pub fcs_err_count: u32,
    pub mlo_params_valid: bool,
    pub assoc_link_id: u8,
    pub __aligned(2): u8 mld_addr[ETH_ALEN],
    pub assoc_resp_ies: *const u8,
    pub assoc_resp_ies_len: usize,
    pub valid_links: u16,
    pub links: [*mut link_station_info; IEEE80211_MLD_MAX_NUM_LINKS],
}

//
// struct cfg80211_sar_sub_specs - sub specs limit
// @power: power limitation in 0.25dbm
// @freq_range_index: index the power limitation applies to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_sar_sub_specs {
    pub power: i32,
    pub freq_range_index: u32,
}

//
// struct cfg80211_sar_specs - sar limit specs
// @type: it's set with power in 0.25dbm or other types
// @num_sub_specs: number of sar sub specs
// @sub_specs: memory to hold the sar sub specs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_sar_specs {
    pub type: nl80211_sar_type,
    pub num_sub_specs: u32,
    pub __counted_by(num_sub_specs): cfg80211_sar_sub_specs sub_specs[],
}

//
// struct cfg80211_sar_freq_ranges - sar frequency ranges
// @start_freq:  start range edge frequency
// @end_freq:    end range edge frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_sar_freq_ranges {
    pub start_freq: u32,
    pub end_freq: u32,
}

//
// struct cfg80211_sar_capa - sar limit capability
// @type: it's set via power in 0.25dbm or other types
// @num_freq_ranges: number of frequency ranges
// @freq_ranges: memory to hold the freq ranges.
//
// Note: WLAN driver may append new ranges or split an existing
// range to small ones and then append them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_sar_capa {
    pub type: nl80211_sar_type,
    pub num_freq_ranges: u32,
    pub freq_ranges: *const cfg80211_sar_freq_ranges,
}

//
// cfg80211_get_station - retrieve information about a given station
// @dev: the device where the station is supposed to be connected to
// @mac_addr: the mac address of the station of interest
// @sinfo: pointer to the structure to fill with the information
//
// Return: 0 on success and sinfo is filled with the available information
// otherwise returns a negative error code and the content of sinfo has to be
// considered undefined.
//

//
// enum monitor_flags - monitor flags
//
// Monitor interface configuration flags. Note that these must be the bits
// according to the nl80211 flags.
//
// @MONITOR_FLAG_CHANGED: set if the flags were changed
// @MONITOR_FLAG_FCSFAIL: pass frames with bad FCS
// @MONITOR_FLAG_PLCPFAIL: pass frames with bad PLCP
// @MONITOR_FLAG_CONTROL: pass control frames
// @MONITOR_FLAG_OTHER_BSS: disable BSSID filtering
// @MONITOR_FLAG_COOK_FRAMES: deprecated, will unconditionally be refused
// @MONITOR_FLAG_ACTIVE: active monitor, ACKs frames on its MAC address
// @MONITOR_FLAG_SKIP_TX: do not pass locally transmitted frames
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum monitor_flags {
    MONITOR_FLAG_CHANGED		= BIT(__NL80211_MNTR_FLAG_INVALID),
    MONITOR_FLAG_FCSFAIL		= BIT(NL80211_MNTR_FLAG_FCSFAIL),
    MONITOR_FLAG_PLCPFAIL		= BIT(NL80211_MNTR_FLAG_PLCPFAIL),
    MONITOR_FLAG_CONTROL		= BIT(NL80211_MNTR_FLAG_CONTROL),
    MONITOR_FLAG_OTHER_BSS		= BIT(NL80211_MNTR_FLAG_OTHER_BSS),
    MONITOR_FLAG_COOK_FRAMES	= BIT(NL80211_MNTR_FLAG_COOK_FRAMES),
    MONITOR_FLAG_ACTIVE		= BIT(NL80211_MNTR_FLAG_ACTIVE),
    MONITOR_FLAG_SKIP_TX		= BIT(NL80211_MNTR_FLAG_SKIP_TX),
}

//
// enum mpath_info_flags -  mesh path information flags
//
// Used by the driver to indicate which info in &struct mpath_info it has filled
// in during get_station() or dump_station().
//
// @MPATH_INFO_FRAME_QLEN: @frame_qlen filled
// @MPATH_INFO_SN: @sn filled
// @MPATH_INFO_METRIC: @metric filled
// @MPATH_INFO_EXPTIME: @exptime filled
// @MPATH_INFO_DISCOVERY_TIMEOUT: @discovery_timeout filled
// @MPATH_INFO_DISCOVERY_RETRIES: @discovery_retries filled
// @MPATH_INFO_FLAGS: @flags filled
// @MPATH_INFO_HOP_COUNT: @hop_count filled
// @MPATH_INFO_PATH_CHANGE: @path_change_count filled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpath_info_flags {
    MPATH_INFO_FRAME_QLEN		= BIT(0),
    MPATH_INFO_SN			= BIT(1),
    MPATH_INFO_METRIC		= BIT(2),
    MPATH_INFO_EXPTIME		= BIT(3),
    MPATH_INFO_DISCOVERY_TIMEOUT	= BIT(4),
    MPATH_INFO_DISCOVERY_RETRIES	= BIT(5),
    MPATH_INFO_FLAGS		= BIT(6),
    MPATH_INFO_HOP_COUNT		= BIT(7),
    MPATH_INFO_PATH_CHANGE		= BIT(8),
}

//
// struct mpath_info - mesh path information
//
// Mesh path information filled by driver for get_mpath() and dump_mpath().
//
// @filled: bitfield of flags from &enum mpath_info_flags
// @frame_qlen: number of queued frames for this destination
// @sn: target sequence number
// @metric: metric (cost) of this mesh path
// @exptime: expiration time for the mesh path from now, in msecs
// @flags: mesh path flags from &enum mesh_path_flags
// @discovery_timeout: total mesh path discovery timeout, in msecs
// @discovery_retries: mesh path discovery retries
// @generation: generation number for nl80211 dumps.
// This number should increase every time the list of mesh paths
// changes, i.e. when a station is added or removed, so that
// userspace can tell whether it got a consistent snapshot.
// @hop_count: hops to destination
// @path_change_count: total number of path changes to destination
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpath_info {
    pub filled: u32,
    pub frame_qlen: u32,
    pub sn: u32,
    pub metric: u32,
    pub exptime: u32,
    pub discovery_timeout: u32,
    pub discovery_retries: u8,
    pub flags: u8,
    pub hop_count: u8,
    pub path_change_count: u32,
    pub generation: c_int,
}

//
// enum wiphy_bss_param_flags - bit positions for supported bss parameters.
//
// @WIPHY_BSS_PARAM_CTS_PROT: support changing CTS protection.
// @WIPHY_BSS_PARAM_SHORT_PREAMBLE: support changing short preamble usage.
// @WIPHY_BSS_PARAM_SHORT_SLOT_TIME: support changing short slot time usage.
// @WIPHY_BSS_PARAM_BASIC_RATES: support reconfiguring basic rates.
// @WIPHY_BSS_PARAM_AP_ISOLATE: support changing AP isolation.
// @WIPHY_BSS_PARAM_HT_OPMODE: support changing HT operating mode.
// @WIPHY_BSS_PARAM_P2P_CTWINDOW: support reconfiguring ctwindow.
// @WIPHY_BSS_PARAM_P2P_OPPPS: support changing P2P opportunistic power-save.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiphy_bss_param_flags {
    WIPHY_BSS_PARAM_CTS_PROT = BIT(0),
    WIPHY_BSS_PARAM_SHORT_PREAMBLE = BIT(1),
    WIPHY_BSS_PARAM_SHORT_SLOT_TIME = BIT(2),
    WIPHY_BSS_PARAM_BASIC_RATES = BIT(3),
    WIPHY_BSS_PARAM_AP_ISOLATE = BIT(4),
    WIPHY_BSS_PARAM_HT_OPMODE = BIT(5),
    WIPHY_BSS_PARAM_P2P_CTWINDOW = BIT(6),
    WIPHY_BSS_PARAM_P2P_OPPPS = BIT(7),
}

//
// struct bss_parameters - BSS parameters
//
// Used to change BSS parameters (mainly for AP mode).
//
// @link_id: link_id or -1 for non-MLD
// @use_cts_prot: Whether to use CTS protection
// (0 = no, 1 = yes, -1 = do not change)
// @use_short_preamble: Whether the use of short preambles is allowed
// (0 = no, 1 = yes, -1 = do not change)
// @use_short_slot_time: Whether the use of short slot time is allowed
// (0 = no, 1 = yes, -1 = do not change)
// @basic_rates: basic rates in IEEE 802.11 format
// (or NULL for no change)
// @basic_rates_len: number of basic rates
// @ap_isolate: do not forward packets between connected stations
// (0 = no, 1 = yes, -1 = do not change)
// @ht_opmode: HT Operation mode
// (u16 = opmode, -1 = do not change)
// @p2p_ctwindow: P2P CT Window (-1 = no change)
// @p2p_opp_ps: P2P opportunistic PS (-1 = no change)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_parameters {
    pub link_id: c_int,
    pub use_cts_prot: c_int,
    pub use_short_preamble: c_int,
    pub use_short_slot_time: c_int,
    pub basic_rates: *const u8,
    pub basic_rates_len: u8,
    pub ap_isolate: c_int,
    pub ht_opmode: c_int,
    pub p2p_opp_ps: s8 p2p_ctwindow,,
}

//
// struct mesh_config - 802.11s mesh configuration
//
// These parameters can be changed while the mesh is active.
//
// @dot11MeshRetryTimeout: the initial retry timeout in millisecond units used
// by the Mesh Peering Open message
// @dot11MeshConfirmTimeout: the initial retry timeout in millisecond units
// used by the Mesh Peering Open message
// @dot11MeshHoldingTimeout: the confirm timeout in millisecond units used by
// the mesh peering management to close a mesh peering
// @dot11MeshMaxPeerLinks: the maximum number of peer links allowed on this
// mesh interface
// @dot11MeshMaxRetries: the maximum number of peer link open retries that can
// be sent to establish a new peer link instance in a mesh
// @dot11MeshTTL: the value of TTL field set at a source mesh STA
// @element_ttl: the value of TTL field set at a mesh STA for path selection
// elements
// @auto_open_plinks: whether we should automatically open peer links when we
// detect compatible mesh peers
// @dot11MeshNbrOffsetMaxNeighbor: the maximum number of neighbors to
// synchronize to for 11s default synchronization method
// @dot11MeshHWMPmaxPREQretries: the number of action frames containing a PREQ
// that an originator mesh STA can send to a particular path target
// @path_refresh_time: how frequently to refresh mesh paths in milliseconds
// @min_discovery_timeout: the minimum length of time to wait until giving up on
// a path discovery in milliseconds
// @dot11MeshHWMPactivePathTimeout: the time (in TUs) for which mesh STAs
// receiving a PREQ shall consider the forwarding information from the
// root to be valid. (TU = time unit)
// @dot11MeshHWMPpreqMinInterval: the minimum interval of time (in TUs) during
// which a mesh STA can send only one action frame containing a PREQ
// element
// @dot11MeshHWMPperrMinInterval: the minimum interval of time (in TUs) during
// which a mesh STA can send only one Action frame containing a PERR
// element
// @dot11MeshHWMPnetDiameterTraversalTime: the interval of time (in TUs) that
// it takes for an HWMP information element to propagate across the mesh
// @dot11MeshHWMPRootMode: the configuration of a mesh STA as root mesh STA
// @dot11MeshHWMPRannInterval: the interval of time (in TUs) between root
// announcements are transmitted
// @dot11MeshGateAnnouncementProtocol: whether to advertise that this mesh
// station has access to a broader network beyond the MBSS. (This is
// missnamed in draft 12.0: dot11MeshGateAnnouncementProtocol set to true
// only means that the station will announce others it's a mesh gate, but
// not necessarily using the gate announcement protocol. Still keeping the
// same nomenclature to be in sync with the spec)
// @dot11MeshForwarding: whether the Mesh STA is forwarding or non-forwarding
// entity (default is TRUE - forwarding entity)
// @rssi_threshold: the threshold for average signal strength of candidate
// station to establish a peer link
// @ht_opmode: mesh HT protection mode
//
// @dot11MeshHWMPactivePathToRootTimeout: The time (in TUs) for which mesh STAs
// receiving a proactive PREQ shall consider the forwarding information to
// the root mesh STA to be valid.
//
// @dot11MeshHWMProotInterval: The interval of time (in TUs) between proactive
// PREQs are transmitted.
// @dot11MeshHWMPconfirmationInterval: The minimum interval of time (in TUs)
// during which a mesh STA can send only one Action frame containing
// a PREQ element for root path confirmation.
// @power_mode: The default mesh power save mode which will be the initial
// setting for new peer links.
// @dot11MeshAwakeWindowDuration: The duration in TUs the STA will remain awake
// after transmitting its beacon.
// @plink_timeout: If no tx activity is seen from a STA we've established
// peering with for longer than this time (in seconds), then remove it
// from the STA's list of peers.  Default is 30 minutes.
// @dot11MeshConnectedToAuthServer: if set to true then this mesh STA
// will advertise that it is connected to a authentication server
// in the mesh formation field.
// @dot11MeshConnectedToMeshGate: if set to true, advertise that this STA is
// connected to a mesh gate in mesh formation info.  If false, the
// value in mesh formation is determined by the presence of root paths
// in the mesh path table
// @dot11MeshNolearn: Try to avoid multi-hop path discovery (e.g. PREQ/PREP
// for HWMP) if the destination is a direct neighbor. Note that this might
// not be the optimal decision as a multi-hop route might be better. So
// if using this setting you will likely also want to disable
// dot11MeshForwarding and use another mesh routing protocol on top.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_config {
    pub dot11MeshRetryTimeout: u16,
    pub dot11MeshConfirmTimeout: u16,
    pub dot11MeshHoldingTimeout: u16,
    pub dot11MeshMaxPeerLinks: u16,
    pub dot11MeshMaxRetries: u8,
    pub dot11MeshTTL: u8,
    pub element_ttl: u8,
    pub auto_open_plinks: bool,
    pub dot11MeshNbrOffsetMaxNeighbor: u32,
    pub dot11MeshHWMPmaxPREQretries: u8,
    pub path_refresh_time: u32,
    pub min_discovery_timeout: u16,
    pub dot11MeshHWMPactivePathTimeout: u32,
    pub dot11MeshHWMPpreqMinInterval: u16,
    pub dot11MeshHWMPperrMinInterval: u16,
    pub dot11MeshHWMPnetDiameterTraversalTime: u16,
    pub dot11MeshHWMPRootMode: u8,
    pub dot11MeshConnectedToMeshGate: bool,
    pub dot11MeshConnectedToAuthServer: bool,
    pub dot11MeshHWMPRannInterval: u16,
    pub dot11MeshGateAnnouncementProtocol: bool,
    pub dot11MeshForwarding: bool,
    pub rssi_threshold: i32,
    pub ht_opmode: u16,
    pub dot11MeshHWMPactivePathToRootTimeout: u32,
    pub dot11MeshHWMProotInterval: u16,
    pub dot11MeshHWMPconfirmationInterval: u16,
    pub power_mode: nl80211_mesh_power_mode,
    pub dot11MeshAwakeWindowDuration: u16,
    pub plink_timeout: u32,
    pub dot11MeshNolearn: bool,
}

//
// struct mesh_setup - 802.11s mesh setup configuration
// @chandef: defines the channel to use
// @mesh_id: the mesh ID
// @mesh_id_len: length of the mesh ID, at least 1 and at most 32 bytes
// @sync_method: which synchronization method to use
// @path_sel_proto: which path selection protocol to use
// @path_metric: which metric to use
// @auth_id: which authentication method this mesh is using
// @ie: vendor information elements (optional)
// @ie_len: length of vendor information elements
// @is_authenticated: this mesh requires authentication
// @is_secure: this mesh uses security
// @user_mpm: userspace handles all MPM functions
// @dtim_period: DTIM period to use
// @beacon_interval: beacon interval to use
// @mcast_rate: multicast rate for Mesh Node [6Mbps is the default for 802.11a]
// @basic_rates: basic rates to use when creating the mesh
// @beacon_rate: bitrate to be used for beacons
// @userspace_handles_dfs: whether user space controls DFS operation, i.e.
// changes the channel when a radar is detected. This is required
// to operate on DFS channels.
// @control_port_over_nl80211: TRUE if userspace expects to exchange control
// port frames over NL80211 instead of the network interface.
//
// These parameters are fixed when the mesh is created.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_setup {
    pub chandef: cfg80211_chan_def,
    pub mesh_id: *const u8,
    pub mesh_id_len: u8,
    pub sync_method: u8,
    pub path_sel_proto: u8,
    pub path_metric: u8,
    pub auth_id: u8,
    pub ie: *const u8,
    pub ie_len: usize,
    pub is_authenticated: bool,
    pub is_secure: bool,
    pub user_mpm: bool,
    pub dtim_period: u8,
    pub beacon_interval: u16,
    pub mcast_rate: [c_int; NUM_NL80211_BANDS],
    pub basic_rates: u32,
    pub beacon_rate: cfg80211_bitrate_mask,
    pub userspace_handles_dfs: bool,
    pub control_port_over_nl80211: bool,
}

//
// struct ocb_setup - 802.11p OCB mode setup configuration
// @chandef: defines the channel to use
//
// These parameters are fixed when connecting to the network
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocb_setup {
    pub chandef: cfg80211_chan_def,
}

//
// struct ieee80211_txq_params - TX queue parameters
// @ac: AC identifier
// @txop: Maximum burst time in units of 32 usecs, 0 meaning disabled
// @cwmin: Minimum contention window [a value of the form 2^n-1 in the range
// 1..32767]
// @cwmax: Maximum contention window [a value of the form 2^n-1 in the range
// 1..32767]
// @aifs: Arbitration interframe space [0..255]
// @link_id: link_id or -1 for non-MLD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_txq_params {
    pub ac: nl80211_ac,
    pub txop: u16,
    pub cwmin: u16,
    pub cwmax: u16,
    pub aifs: u8,
    pub link_id: c_int,
}

//
// DOC: Scanning and BSS list handling
//
// The scanning process itself is fairly simple, but cfg80211 offers quite
// a bit of helper functionality. To start a scan, the scan operation will
// be invoked with a scan definition. This scan definition contains the
// channels to scan, and the SSIDs to send probe requests for (including the
// wildcard, if desired). A passive scan is indicated by having no SSIDs to
// probe. Additionally, a scan request may contain extra information elements
// that should be added to the probe request. The IEs are guaranteed to be
// well-formed, and will not exceed the maximum length the driver advertised
// in the wiphy structure.
//
// When scanning finds a BSS, cfg80211 needs to be notified of that, because
// it is responsible for maintaining the BSS list; the driver should not
// maintain a list itself. For this notification, various functions exist.
//
// Since drivers do not maintain a BSS list, there are also a number of
// functions to search for a BSS and obtain information about it from the
// BSS structure cfg80211 maintains. The BSS list is also made available
// to userspace.
//
// struct cfg80211_ssid - SSID description
// @ssid: the SSID
// @ssid_len: length of the ssid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ssid {
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
}

//
// struct cfg80211_scan_info - information about completed scan
// @scan_start_tsf: scan start time in terms of the TSF of the BSS that the
// wireless device that requested the scan is connected to. If this
// information is not available, this field is left zero.
// @tsf_bssid: the BSSID according to which %scan_start_tsf is set.
// @aborted: set to true if the scan was aborted for any reason,
// userspace will be notified of that
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_scan_info {
    pub scan_start_tsf: u64,
    pub __aligned(2): u8 tsf_bssid[ETH_ALEN],
    pub aborted: bool,
}

//
// struct cfg80211_scan_6ghz_params - relevant for 6 GHz only
//
// @short_ssid: short ssid to scan for
// @bssid: bssid to scan for
// @channel_idx: idx of the channel in the channel array in the scan request
// which the above info is relevant to
// @unsolicited_probe: the AP transmits unsolicited probe response every 20 TU
// @short_ssid_valid: @short_ssid is valid and can be used
// @psc_no_listen: when set, and the channel is a PSC channel, no need to wait
// 20 TUs before starting to send probe requests.
// @psd_20: The AP's 20 MHz PSD value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_scan_6ghz_params {
    pub short_ssid: u32,
    pub channel_idx: u32,
    pub bssid: [u8; ETH_ALEN],
    pub unsolicited_probe: bool,
    pub short_ssid_valid: bool,
    pub psc_no_listen: bool,
    pub psd_20: i8,
}

//
// struct cfg80211_scan_request - scan request description
//
// @ssids: SSIDs to scan for (active scan only)
// @n_ssids: number of SSIDs
// @channels: channels to scan on.
// @n_channels: total number of channels to scan
// @ie: optional information element(s) to add into Probe Request or %NULL
// @ie_len: length of ie in octets
// @duration: how long to listen on each channel, in TUs. If
// %duration_mandatory is not set, this is the maximum dwell time and
// the actual dwell time may be shorter.
// @duration_mandatory: if set, the scan duration must be as specified by the
// %duration field.
// @flags: control flags from &enum nl80211_scan_flags
// @rates: bitmap of rates to advertise for each band
// @wiphy: the wiphy this was for
// @scan_start: time (in jiffies) when the scan started
// @wdev: the wireless device to scan for
// @no_cck: used to send probe requests at non CCK rate in 2GHz band
// @mac_addr: MAC address used with randomisation
// @mac_addr_mask: MAC address mask used with randomisation, bits that
// are 0 in the mask should be randomised, bits that are 1 should
// be taken from the @mac_addr
// @scan_6ghz: relevant for split scan request only,
// true if this is a 6 GHz scan request
// @first_part: %true if this is the first part of a split scan request or a
// scan that was not split. May be %true for a @scan_6ghz scan if no other
// channels were requested
// @n_6ghz_params: number of 6 GHz params
// @scan_6ghz_params: 6 GHz params
// @bssid: BSSID to scan for (most commonly, the wildcard BSSID)
// @tsf_report_link_id: for MLO, indicates the link ID of the BSS that should be
// used for TSF reporting. Can be set to -1 to indicate no preference.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_scan_request {
    pub ssids: *mut cfg80211_ssid,
    pub n_ssids: c_int,
    pub n_channels: u32,
    pub ie: *const u8,
    pub ie_len: usize,
    pub duration: u16,
    pub duration_mandatory: bool,
    pub flags: u32,
    pub rates: [u32; NUM_NL80211_BANDS],
    pub wdev: *mut wireless_dev,
    pub __aligned(2): u8 mac_addr[ETH_ALEN],
    pub __aligned(2): u8 mac_addr_mask[ETH_ALEN],
    pub __aligned(2): u8 bssid[ETH_ALEN],
    pub wiphy: *mut wiphy,
    pub scan_start: c_ulong,
    pub no_cck: bool,
    pub scan_6ghz: bool,
    pub first_part: bool,
    pub n_6ghz_params: u32,
    pub scan_6ghz_params: *mut cfg80211_scan_6ghz_params,
    pub tsf_report_link_id: i8,
// keep last
    pub channels: [*mut ieee80211_channel; ],
}

//
// struct cfg80211_match_set - sets of attributes to match
//
// @ssid: SSID to be matched; may be zero-length in case of BSSID match
// or no match (RSSI only)
// @bssid: BSSID to be matched; may be all-zero BSSID in case of SSID match
// or no match (RSSI only)
// @rssi_thold: don't report scan results below this threshold (in s32 dBm)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_match_set {
    pub ssid: cfg80211_ssid,
    pub bssid: [u8; ETH_ALEN],
    pub rssi_thold: i32,
}

//
// struct cfg80211_sched_scan_plan - scan plan for scheduled scan
//
// @interval: interval between scheduled scan iterations. In seconds.
// @iterations: number of scan iterations in this scan plan. Zero means
// infinite loop.
// The last scan plan will always have this parameter set to zero,
// all other scan plans will have a finite number of iterations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_sched_scan_plan {
    pub interval: u32,
    pub iterations: u32,
}

//
// struct cfg80211_bss_select_adjust - BSS selection with RSSI adjustment.
//
// @band: band of BSS which should match for RSSI level adjustment.
// @delta: value of RSSI level adjustment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_bss_select_adjust {
    pub band: nl80211_band,
    pub delta: i8,
}

//
// struct cfg80211_sched_scan_request - scheduled scan request description
//
// @reqid: identifies this request.
// @ssids: SSIDs to scan for (passed in the probe_reqs in active scans)
// @n_ssids: number of SSIDs
// @n_channels: total number of channels to scan
// @ie: optional information element(s) to add into Probe Request or %NULL
// @ie_len: length of ie in octets
// @flags: control flags from &enum nl80211_scan_flags
// @match_sets: sets of parameters to be matched for a scan result
// entry to be considered valid and to be passed to the host
// (others are filtered out).
// If omitted, all results are passed.
// @n_match_sets: number of match sets
// @report_results: indicates that results were reported for this request
// @wiphy: the wiphy this was for
// @dev: the interface
// @scan_start: start time of the scheduled scan
// @channels: channels to scan
// @min_rssi_thold: for drivers only supporting a single threshold, this
// contains the minimum over all matchsets
// @mac_addr: MAC address used with randomisation
// @mac_addr_mask: MAC address mask used with randomisation, bits that
// are 0 in the mask should be randomised, bits that are 1 should
// be taken from the @mac_addr
// @scan_plans: scan plans to be executed in this scheduled scan. Lowest
// index must be executed first.
// @n_scan_plans: number of scan plans, at least 1.
// @rcu_head: RCU callback used to free the struct
// @owner_nlportid: netlink portid of owner (if this should is a request
// owned by a particular socket)
// @nl_owner_dead: netlink owner socket was closed - this request be freed
// @list: for keeping list of requests.
// @delay: delay in seconds to use before starting the first scan
// cycle.  The driver may ignore this parameter and start
// immediately (or at any other time), if this feature is not
// supported.
// @relative_rssi_set: Indicates whether @relative_rssi is set or not.
// @relative_rssi: Relative RSSI threshold in dB to restrict scan result
// reporting in connected state to cases where a matching BSS is determined
// to have better or slightly worse RSSI than the current connected BSS.
// The relative RSSI threshold values are ignored in disconnected state.
// @rssi_adjust: delta dB of RSSI preference to be given to the BSSs that belong
// to the specified band while deciding whether a better BSS is reported
// using @relative_rssi. If delta is a negative number, the BSSs that
// belong to the specified band will be penalized by delta dB in relative
// comparisons.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_sched_scan_request {
    pub reqid: u64,
    pub ssids: *mut cfg80211_ssid,
    pub n_ssids: c_int,
    pub n_channels: u32,
    pub ie: *const u8,
    pub ie_len: usize,
    pub flags: u32,
    pub match_sets: *mut cfg80211_match_set,
    pub n_match_sets: c_int,
    pub min_rssi_thold: i32,
    pub delay: u32,
    pub scan_plans: *mut cfg80211_sched_scan_plan,
    pub n_scan_plans: c_int,
    pub __aligned(2): u8 mac_addr[ETH_ALEN],
    pub __aligned(2): u8 mac_addr_mask[ETH_ALEN],
    pub relative_rssi_set: bool,
    pub relative_rssi: i8,
    pub rssi_adjust: cfg80211_bss_select_adjust,
// internal
    pub wiphy: *mut wiphy,
    pub dev: *mut net_device,
    pub scan_start: c_ulong,
    pub report_results: bool,
    pub rcu_head: rcu_head,
    pub owner_nlportid: u32,
    pub nl_owner_dead: bool,
    pub list: list_head,
// keep last
    pub __counted_by(n_channels): *mut *mut ieee80211_channel channels[],
}

//
// enum cfg80211_signal_type - signal type
//
// @CFG80211_SIGNAL_TYPE_NONE: no signal strength information available
// @CFG80211_SIGNAL_TYPE_MBM: signal strength in mBm (100*dBm)
// @CFG80211_SIGNAL_TYPE_UNSPEC: signal strength, increasing from 0 through 100
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_signal_type {
    CFG80211_SIGNAL_TYPE_NONE,
    CFG80211_SIGNAL_TYPE_MBM,
    CFG80211_SIGNAL_TYPE_UNSPEC,
}

//
// struct cfg80211_inform_bss - BSS inform data
// @chan: channel the frame was received on
// @signal: signal strength value, according to the wiphy's
// signal type
// @boottime_ns: timestamp (CLOCK_BOOTTIME) when the information was
// received; should match the time when the frame was actually
// received by the device (not just by the host, in case it was
// buffered on the device) and be accurate to about 10ms.
// If the frame isn't buffered, just passing the return value of
// ktime_get_boottime_ns() is likely appropriate.
// @parent_tsf: the time at the start of reception of the first octet of the
// timestamp field of the frame. The time is the TSF of the BSS specified
// by %parent_bssid.
// @parent_bssid: the BSS according to which %parent_tsf is set. This is set to
// the BSS that requested the scan in which the beacon/probe was received.
// @chains: bitmask for filled values in @chain_signal.
// @chain_signal: per-chain signal strength of last received BSS in dBm.
// @restrict_use: restrict usage, if not set, assume @use_for is
// %NL80211_BSS_USE_FOR_NORMAL.
// @use_for: bitmap of possible usage for this BSS, see
// &enum nl80211_bss_use_for
// @cannot_use_reasons: the reasons (bitmap) for not being able to connect,
// if @restrict_use is set and @use_for is zero (empty); may be 0 for
// unspecified reasons; see &enum nl80211_bss_cannot_use_reasons
// @drv_data: Data to be passed through to @inform_bss
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_inform_bss {
    pub chan: *mut ieee80211_channel,
    pub signal: i32,
    pub boottime_ns: u64,
    pub parent_tsf: u64,
    pub __aligned(2): u8 parent_bssid[ETH_ALEN],
    pub chains: u8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
    pub use_for:7: u8 restrict_use:1,,
    pub cannot_use_reasons: u8,
    pub drv_data: *mut c_void,
}

//
// struct cfg80211_bss_ies - BSS entry IE data
// @tsf: TSF contained in the frame that carried these IEs
// @rcu_head: internal use, for freeing
// @len: length of the IEs
// @from_beacon: these IEs are known to come from a beacon
// @data: IE data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_bss_ies {
    pub tsf: u64,
    pub rcu_head: rcu_head,
    pub len: c_int,
    pub from_beacon: bool,
    pub data: [u8; ],
}

//
// struct cfg80211_bss - BSS description
//
// This structure describes a BSS (which may also be a mesh network)
// for use in scan results and similar.
//
// @channel: channel this BSS is on
// @bssid: BSSID of the BSS
// @beacon_interval: the beacon interval as from the frame
// @capability: the capability field in host byte order
// @ies: the information elements (Note that there is no guarantee that these
// are well-formed!); this is a pointer to either the beacon_ies or
// proberesp_ies depending on whether Probe Response frame has been
// received. It is always non-%NULL.
// @beacon_ies: the information elements from the last Beacon frame
// (implementation note: if @hidden_beacon_bss is set this struct doesn't
// own the beacon_ies, but they're just pointers to the ones from the
// @hidden_beacon_bss struct)
// @proberesp_ies: the information elements from the last Probe Response frame
// @proberesp_ecsa_stuck: ECSA element is stuck in the Probe Response frame,
// cannot rely on it having valid data
// @hidden_beacon_bss: in case this BSS struct represents a probe response from
// a BSS that hides the SSID in its beacon, this points to the BSS struct
// that holds the beacon data. @beacon_ies is still valid, of course, and
// points to the same data as hidden_beacon_bss->beacon_ies in that case.
// @transmitted_bss: pointer to the transmitted BSS, if this is a
// non-transmitted one (multi-BSSID support)
// @nontrans_list: list of non-transmitted BSS, if this is a transmitted one
// (multi-BSSID support)
// @signal: signal strength value (type depends on the wiphy's signal_type)
// @ts_boottime: timestamp of the last BSS update in nanoseconds since boot
// @chains: bitmask for filled values in @chain_signal.
// @chain_signal: per-chain signal strength of last received BSS in dBm.
// @bssid_index: index in the multiple BSS set
// @max_bssid_indicator: max number of members in the BSS set
// @use_for: bitmap of possible usage for this BSS, see
// &enum nl80211_bss_use_for
// @cannot_use_reasons: the reasons (bitmap) for not being able to connect,
// if @restrict_use is set and @use_for is zero (empty); may be 0 for
// unspecified reasons; see &enum nl80211_bss_cannot_use_reasons
// @priv: private area for driver use, has at least wiphy->bss_priv_size bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_bss {
    pub channel: *mut ieee80211_channel,
    pub ies: *const cfg80211_bss_ies __rcu,
    pub beacon_ies: *const cfg80211_bss_ies __rcu,
    pub proberesp_ies: *const cfg80211_bss_ies __rcu,
    pub hidden_beacon_bss: *mut cfg80211_bss,
    pub transmitted_bss: *mut cfg80211_bss,
    pub nontrans_list: list_head,
    pub signal: i32,
    pub ts_boottime: u64,
    pub beacon_interval: u16,
    pub capability: u16,
    pub bssid: [u8; ETH_ALEN],
    pub chains: u8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
    pub proberesp_ecsa_stuck:1: u8,
    pub bssid_index: u8,
    pub max_bssid_indicator: u8,
    pub use_for: u8,
    pub cannot_use_reasons: u8,
    pub )): *mut u8 priv[] __aligned(sizeof(void,
}

//
// ieee80211_bss_get_elem - find element with given ID
// @bss: the bss to search
// @id: the element ID
//
// Note that the return value is an RCU-protected pointer, so
// rcu_read_lock() must be held when calling this function.
// Return: %NULL if not found.
//
// ieee80211_bss_get_ie - find IE with given ID
// @bss: the bss to search
// @id: the element ID
//
// Note that the return value is an RCU-protected pointer, so
// rcu_read_lock() must be held when calling this function.
// Return: %NULL if not found.
//
// struct cfg80211_auth_request - Authentication request data
//
// This structure provides information needed to complete IEEE 802.11
// authentication.
//
// @bss: The BSS to authenticate with, the callee must obtain a reference
// to it if it needs to keep it.
// @supported_selectors: List of selectors that should be assumed to be
// supported by the station.
// SAE_H2E must be assumed supported if set to %NULL.
// @supported_selectors_len: Length of supported_selectors in octets.
// @auth_type: Authentication type (algorithm)
// @ie: Extra IEs to add to Authentication frame or %NULL
// @ie_len: Length of ie buffer in octets
// @key_len: length of WEP key for shared key authentication
// @key_idx: index of WEP key for shared key authentication
// @key: WEP key for shared key authentication
// @auth_data: Fields and elements in Authentication frames. This contains
// the authentication frame body (non-IE and IE data), excluding the
// Authentication algorithm number, i.e., starting at the Authentication
// transaction sequence number field.
// @auth_data_len: Length of auth_data buffer in octets
// @link_id: if >= 0, indicates authentication should be done as an MLD,
// the interface address is included as the MLD address and the
// necessary link (with the given link_id) will be created (and
// given an MLD address) by the driver
// @ap_mld_addr: AP MLD address in case of authentication request with
// an AP MLD, valid iff @link_id >= 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_auth_request {
    pub bss: *mut cfg80211_bss,
    pub ie: *const u8,
    pub ie_len: usize,
    pub supported_selectors: *const u8,
    pub supported_selectors_len: u8,
    pub auth_type: nl80211_auth_type,
    pub key: *const u8,
    pub key_len: u8,
    pub key_idx: i8,
    pub auth_data: *const u8,
    pub auth_data_len: usize,
    pub link_id: i8,
    pub ap_mld_addr: *const u8,
}

//
// struct cfg80211_assoc_link - per-link information for MLO association
// @bss: the BSS pointer, see also &struct cfg80211_assoc_request::bss;
// if this is %NULL for a link, that link is not requested
// @elems: extra elements for the per-STA profile for this link
// @elems_len: length of the elements
// @error: per-link error code, must be <= 0. If there is an error, then the
// operation as a whole must fail.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_assoc_link {
    pub bss: *mut cfg80211_bss,
    pub elems: *const u8,
    pub elems_len: usize,
    pub error: c_int,
}

//
// struct cfg80211_ml_reconf_req - MLO link reconfiguration request
// @add_links: data for links to add, see &struct cfg80211_assoc_link
// @rem_links: bitmap of links to remove
// @ext_mld_capa_ops: extended MLD capabilities and operations set by
// userspace for the ML reconfiguration action frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ml_reconf_req {
    pub add_links: [cfg80211_assoc_link; IEEE80211_MLD_MAX_NUM_LINKS],
    pub rem_links: u16,
    pub ext_mld_capa_ops: u16,
}

//
// enum cfg80211_assoc_req_flags - Over-ride default behaviour in association.
//
// @ASSOC_REQ_DISABLE_HT:  Disable HT (802.11n)
// @ASSOC_REQ_DISABLE_VHT:  Disable VHT
// @ASSOC_REQ_USE_RRM: Declare RRM capability in this association
// @CONNECT_REQ_EXTERNAL_AUTH_SUPPORT: User space indicates external
// authentication capability. Drivers can offload authentication to
// userspace if this flag is set. Only applicable for cfg80211_connect()
// request (connect callback).
// @ASSOC_REQ_DISABLE_HE:  Disable HE
// @ASSOC_REQ_DISABLE_EHT:  Disable EHT
// @CONNECT_REQ_MLO_SUPPORT: Userspace indicates support for handling MLD links.
// Drivers shall disable MLO features for the current association if this
// flag is not set.
// @ASSOC_REQ_SPP_AMSDU: SPP A-MSDUs will be used on this connection (if any)
// @ASSOC_REQ_DISABLE_UHR: Disable UHR
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_assoc_req_flags {
    ASSOC_REQ_DISABLE_HT			= BIT(0),
    ASSOC_REQ_DISABLE_VHT			= BIT(1),
    ASSOC_REQ_USE_RRM			= BIT(2),
    CONNECT_REQ_EXTERNAL_AUTH_SUPPORT	= BIT(3),
    ASSOC_REQ_DISABLE_HE			= BIT(4),
    ASSOC_REQ_DISABLE_EHT			= BIT(5),
    CONNECT_REQ_MLO_SUPPORT			= BIT(6),
    ASSOC_REQ_SPP_AMSDU			= BIT(7),
    ASSOC_REQ_DISABLE_UHR			= BIT(8),
}

//
// struct cfg80211_assoc_request - (Re)Association request data
//
// This structure provides information needed to complete IEEE 802.11
// (re)association.
// @bss: The BSS to associate with. If the call is successful the driver is
// given a reference that it must give back to cfg80211_send_rx_assoc()
// or to cfg80211_assoc_timeout(). To ensure proper refcounting, new
// association requests while already associating must be rejected.
// This also applies to the @links.bss parameter, which is used instead
// of this one (it is %NULL) for MLO associations.
// @ie: Extra IEs to add to (Re)Association Request frame or %NULL
// @ie_len: Length of ie buffer in octets
// @use_mfp: Use management frame protection (IEEE 802.11w) in this association
// @crypto: crypto settings
// @prev_bssid: previous BSSID, if not %NULL use reassociate frame. This is used
// to indicate a request to reassociate within the ESS instead of a request
// do the initial association with the ESS. When included, this is set to
// the BSSID of the current association, i.e., to the value that is
// included in the Current AP address field of the Reassociation Request
// frame.
// @flags:  See &enum cfg80211_assoc_req_flags
// @supported_selectors: supported BSS selectors in IEEE 802.11 format
// (or %NULL for no change).
// If %NULL, then support for SAE_H2E should be assumed.
// @supported_selectors_len: number of supported BSS selectors
// @ht_capa:  HT Capabilities over-rides.  Values set in ht_capa_mask
// will be used in ht_capa.  Un-supported values will be ignored.
// @ht_capa_mask:  The bits of ht_capa which are to be used.
// @vht_capa: VHT capability override
// @vht_capa_mask: VHT capability mask indicating which fields to use
// @fils_kek: FILS KEK for protecting (Re)Association Request/Response frame or
// %NULL if FILS is not used.
// @fils_kek_len: Length of fils_kek in octets
// @fils_nonces: FILS nonces (part of AAD) for protecting (Re)Association
// Request/Response frame or %NULL if FILS is not used. This field starts
// with 16 octets of STA Nonce followed by 16 octets of AP Nonce.
// @s1g_capa: S1G capability override
// @s1g_capa_mask: S1G capability override mask
// @links: per-link information for MLO connections
// @link_id: >= 0 for MLO connections, where links are given, and indicates
// the link on which the association request should be sent
// @ap_mld_addr: AP MLD address in case of MLO association request,
// valid iff @link_id >= 0
// @ext_mld_capa_ops: extended MLD capabilities and operations set by
// userspace for the association
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_assoc_request {
    pub bss: *mut cfg80211_bss,
    pub prev_bssid: *const *const u8 ie,,
    pub ie_len: usize,
    pub crypto: cfg80211_crypto_settings,
    pub use_mfp: bool,
    pub flags: u32,
    pub supported_selectors: *const u8,
    pub supported_selectors_len: u8,
    pub ht_capa: ieee80211_ht_cap,
    pub ht_capa_mask: ieee80211_ht_cap,
    pub vht_capa_mask: ieee80211_vht_cap vht_capa,,
    pub fils_kek: *const u8,
    pub fils_kek_len: usize,
    pub fils_nonces: *const u8,
    pub s1g_capa_mask: ieee80211_s1g_cap s1g_capa,,
    pub links: [cfg80211_assoc_link; IEEE80211_MLD_MAX_NUM_LINKS],
    pub ap_mld_addr: *const u8,
    pub link_id: i8,
    pub ext_mld_capa_ops: u16,
}

//
// struct cfg80211_deauth_request - Deauthentication request data
//
// This structure provides information needed to complete IEEE 802.11
// deauthentication.
//
// @bssid: the BSSID or AP MLD address to deauthenticate from
// @ie: Extra IEs to add to Deauthentication frame or %NULL
// @ie_len: Length of ie buffer in octets
// @reason_code: The reason code for the deauthentication
// @local_state_change: if set, change local state only and
// do not set a deauth frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_deauth_request {
    pub bssid: *const u8,
    pub ie: *const u8,
    pub ie_len: usize,
    pub reason_code: u16,
    pub local_state_change: bool,
}

//
// struct cfg80211_disassoc_request - Disassociation request data
//
// This structure provides information needed to complete IEEE 802.11
// disassociation.
//
// @ap_addr: the BSSID or AP MLD address to disassociate from
// @ie: Extra IEs to add to Disassociation frame or %NULL
// @ie_len: Length of ie buffer in octets
// @reason_code: The reason code for the disassociation
// @local_state_change: This is a request for a local state only, i.e., no
// Disassociation frame is to be transmitted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_disassoc_request {
    pub ap_addr: *const u8,
    pub ie: *const u8,
    pub ie_len: usize,
    pub reason_code: u16,
    pub local_state_change: bool,
}

//
// struct cfg80211_ibss_params - IBSS parameters
//
// This structure defines the IBSS parameters for the join_ibss()
// method.
//
// @ssid: The SSID, will always be non-null.
// @ssid_len: The length of the SSID, will always be non-zero.
// @bssid: Fixed BSSID requested, maybe be %NULL, if set do not
// search for IBSSs with a different BSSID.
// @chandef: defines the channel to use if no other IBSS to join can be found
// @channel_fixed: The channel should be fixed -- do not search for
// IBSSs to join on other channels.
// @ie: information element(s) to include in the beacon
// @ie_len: length of that
// @beacon_interval: beacon interval to use
// @privacy: this is a protected network, keys will be configured
// after joining
// @control_port: whether user space controls IEEE 802.1X port, i.e.,
// sets/clears %NL80211_STA_FLAG_AUTHORIZED. If true, the driver is
// required to assume that the port is unauthorized until authorized by
// user space. Otherwise, port is marked authorized by default.
// @control_port_over_nl80211: TRUE if userspace expects to exchange control
// port frames over NL80211 instead of the network interface.
// @userspace_handles_dfs: whether user space controls DFS operation, i.e.
// changes the channel when a radar is detected. This is required
// to operate on DFS channels.
// @basic_rates: bitmap of basic rates to use when creating the IBSS
// @mcast_rate: per-band multicast rate index + 1 (0: disabled)
// @ht_capa:  HT Capabilities over-rides.  Values set in ht_capa_mask
// will be used in ht_capa.  Un-supported values will be ignored.
// @ht_capa_mask:  The bits of ht_capa which are to be used.
// @wep_keys: static WEP keys, if not NULL points to an array of
// CFG80211_MAX_WEP_KEYS WEP keys
// @wep_tx_key: key index (0..3) of the default TX static WEP key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ibss_params {
    pub ssid: *const u8,
    pub bssid: *const u8,
    pub chandef: cfg80211_chan_def,
    pub ie: *const u8,
    pub ie_len: u8 ssid_len,,
    pub beacon_interval: u16,
    pub basic_rates: u32,
    pub channel_fixed: bool,
    pub privacy: bool,
    pub control_port: bool,
    pub control_port_over_nl80211: bool,
    pub userspace_handles_dfs: bool,
    pub mcast_rate: [c_int; NUM_NL80211_BANDS],
    pub ht_capa: ieee80211_ht_cap,
    pub ht_capa_mask: ieee80211_ht_cap,
    pub wep_keys: *mut key_params,
    pub wep_tx_key: c_int,
}

//
// struct cfg80211_bss_selection - connection parameters for BSS selection.
//
// @behaviour: requested BSS selection behaviour.
// @param: parameters for requestion behaviour.
// @param.band_pref: preferred band for %NL80211_BSS_SELECT_ATTR_BAND_PREF.
// @param.adjust: parameters for %NL80211_BSS_SELECT_ATTR_RSSI_ADJUST.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_bss_selection {
    pub behaviour: nl80211_bss_select_attr,
    pub band_pref: nl80211_band,
    pub adjust: cfg80211_bss_select_adjust,
    pub param: },
}

//
// struct cfg80211_connect_params - Connection parameters
//
// This structure provides information needed to complete IEEE 802.11
// authentication and association.
//
// @channel: The channel to use or %NULL if not specified (auto-select based
// on scan results)
// @channel_hint: The channel of the recommended BSS for initial connection or
// %NULL if not specified
// @bssid: The AP BSSID or %NULL if not specified (auto-select based on scan
// results)
// @bssid_hint: The recommended AP BSSID for initial connection to the BSS or
// %NULL if not specified. Unlike the @bssid parameter, the driver is
// allowed to ignore this @bssid_hint if it has knowledge of a better BSS
// to use.
// @ssid: SSID
// @ssid_len: Length of ssid in octets
// @auth_type: Authentication type (algorithm)
// @ie: IEs for association request
// @ie_len: Length of assoc_ie in octets
// @privacy: indicates whether privacy-enabled APs should be used
// @mfp: indicate whether management frame protection is used
// @crypto: crypto settings
// @key_len: length of WEP key for shared key authentication
// @key_idx: index of WEP key for shared key authentication
// @key: WEP key for shared key authentication
// @flags:  See &enum cfg80211_assoc_req_flags
// @bg_scan_period:  Background scan period in seconds
// or -1 to indicate that default value is to be used.
// @ht_capa:  HT Capabilities over-rides.  Values set in ht_capa_mask
// will be used in ht_capa.  Un-supported values will be ignored.
// @ht_capa_mask:  The bits of ht_capa which are to be used.
// @vht_capa:  VHT Capability overrides
// @vht_capa_mask: The bits of vht_capa which are to be used.
// @pbss: if set, connect to a PCP instead of AP. Valid for DMG
// networks.
// @bss_select: criteria to be used for BSS selection.
// @prev_bssid: previous BSSID, if not %NULL use reassociate frame. This is used
// to indicate a request to reassociate within the ESS instead of a request
// do the initial association with the ESS. When included, this is set to
// the BSSID of the current association, i.e., to the value that is
// included in the Current AP address field of the Reassociation Request
// frame.
// @fils_erp_username: EAP re-authentication protocol (ERP) username part of the
// NAI or %NULL if not specified. This is used to construct FILS wrapped
// data IE.
// @fils_erp_username_len: Length of @fils_erp_username in octets.
// @fils_erp_realm: EAP re-authentication protocol (ERP) realm part of NAI or
// %NULL if not specified. This specifies the domain name of ER server and
// is used to construct FILS wrapped data IE.
// @fils_erp_realm_len: Length of @fils_erp_realm in octets.
// @fils_erp_next_seq_num: The next sequence number to use in the FILS ERP
// messages. This is also used to construct FILS wrapped data IE.
// @fils_erp_rrk: ERP re-authentication Root Key (rRK) used to derive additional
// keys in FILS or %NULL if not specified.
// @fils_erp_rrk_len: Length of @fils_erp_rrk in octets.
// @want_1x: indicates user-space supports and wants to use 802.1X driver
// offload of 4-way handshake.
// @edmg: define the EDMG channels.
// This may specify multiple channels and bonding options for the driver
// to choose from, based on BSS configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_connect_params {
    pub channel: *mut ieee80211_channel,
    pub channel_hint: *mut ieee80211_channel,
    pub bssid: *const u8,
    pub bssid_hint: *const u8,
    pub ssid: *const u8,
    pub ssid_len: usize,
    pub auth_type: nl80211_auth_type,
    pub ie: *const u8,
    pub ie_len: usize,
    pub privacy: bool,
    pub mfp: nl80211_mfp,
    pub crypto: cfg80211_crypto_settings,
    pub key: *const u8,
    pub key_idx: u8 key_len,,
    pub flags: u32,
    pub bg_scan_period: c_int,
    pub ht_capa: ieee80211_ht_cap,
    pub ht_capa_mask: ieee80211_ht_cap,
    pub vht_capa: ieee80211_vht_cap,
    pub vht_capa_mask: ieee80211_vht_cap,
    pub pbss: bool,
    pub bss_select: cfg80211_bss_selection,
    pub prev_bssid: *const u8,
    pub fils_erp_username: *const u8,
    pub fils_erp_username_len: usize,
    pub fils_erp_realm: *const u8,
    pub fils_erp_realm_len: usize,
    pub fils_erp_next_seq_num: u16,
    pub fils_erp_rrk: *const u8,
    pub fils_erp_rrk_len: usize,
    pub want_1x: bool,
    pub edmg: ieee80211_edmg,
}

//
// enum cfg80211_connect_params_changed - Connection parameters being updated
//
// This enum provides information of all connect parameters that
// have to be updated as part of update_connect_params() call.
//
// @UPDATE_ASSOC_IES: Indicates whether association request IEs are updated
// @UPDATE_FILS_ERP_INFO: Indicates that FILS connection parameters (realm,
// username, erp sequence number and rrk) are updated
// @UPDATE_AUTH_TYPE: Indicates that authentication type is updated
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_connect_params_changed {
    UPDATE_ASSOC_IES		= BIT(0),
    UPDATE_FILS_ERP_INFO		= BIT(1),
    UPDATE_AUTH_TYPE		= BIT(2),
}

//
// enum wiphy_params_flags - set_wiphy_params bitfield values
// @WIPHY_PARAM_RETRY_SHORT: wiphy->retry_short has changed
// @WIPHY_PARAM_RETRY_LONG: wiphy->retry_long has changed
// @WIPHY_PARAM_FRAG_THRESHOLD: wiphy->frag_threshold has changed
// @WIPHY_PARAM_RTS_THRESHOLD: wiphy->rts_threshold has changed
// @WIPHY_PARAM_COVERAGE_CLASS: coverage class changed
// @WIPHY_PARAM_DYN_ACK: dynack has been enabled
// @WIPHY_PARAM_TXQ_LIMIT: TXQ packet limit has been changed
// @WIPHY_PARAM_TXQ_MEMORY_LIMIT: TXQ memory limit has been changed
// @WIPHY_PARAM_TXQ_QUANTUM: TXQ scheduler quantum
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiphy_params_flags {
    WIPHY_PARAM_RETRY_SHORT		= BIT(0),
    WIPHY_PARAM_RETRY_LONG		= BIT(1),
    WIPHY_PARAM_FRAG_THRESHOLD	= BIT(2),
    WIPHY_PARAM_RTS_THRESHOLD	= BIT(3),
    WIPHY_PARAM_COVERAGE_CLASS	= BIT(4),
    WIPHY_PARAM_DYN_ACK		= BIT(5),
    WIPHY_PARAM_TXQ_LIMIT		= BIT(6),
    WIPHY_PARAM_TXQ_MEMORY_LIMIT	= BIT(7),
    WIPHY_PARAM_TXQ_QUANTUM		= BIT(8),
}

pub const IEEE80211_DEFAULT_AIRTIME_WEIGHT: c_int = 256;
// The per TXQ device queue limit in airtime
pub const IEEE80211_DEFAULT_AQL_TXQ_LIMIT_L: c_int = 5000;
pub const IEEE80211_DEFAULT_AQL_TXQ_LIMIT_H: c_int = 12000;
pub const IEEE80211_DEFAULT_AQL_TXQ_LIMIT_MC: c_int = 50000;
// The per interface airtime threshold to switch to lower queue limit
pub const IEEE80211_AQL_THRESHOLD: c_int = 24000;
//
// struct cfg80211_pmksa - PMK Security Association
//
// This structure is passed to the set/del_pmksa() method for PMKSA
// caching.
//
// @bssid: The AP's BSSID (may be %NULL).
// @pmkid: The identifier to refer a PMKSA.
// @pmk: The PMK for the PMKSA identified by @pmkid. This is used for key
// derivation by a FILS STA. Otherwise, %NULL.
// @pmk_len: Length of the @pmk. The length of @pmk can differ depending on
// the hash algorithm used to generate this.
// @ssid: SSID to specify the ESS within which a PMKSA is valid when using FILS
// cache identifier (may be %NULL).
// @ssid_len: Length of the @ssid in octets.
// @cache_id: 2-octet cache identifier advertized by a FILS AP identifying the
// scope of PMKSA. This is valid only if @ssid_len is non-zero (may be
// %NULL).
// @pmk_lifetime: Maximum lifetime for PMKSA in seconds
// (dot11RSNAConfigPMKLifetime) or 0 if not specified.
// The configured PMKSA must not be used for PMKSA caching after
// expiration and any keys derived from this PMK become invalid on
// expiration, i.e., the current association must be dropped if the PMK
// used for it expires.
// @pmk_reauth_threshold: Threshold time for reauthentication (percentage of
// PMK lifetime, dot11RSNAConfigPMKReauthThreshold) or 0 if not specified.
// Drivers are expected to trigger a full authentication instead of using
// this PMKSA for caching when reassociating to a new BSS after this
// threshold to generate a new PMK before the current one expires.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmksa {
    pub bssid: *const u8,
    pub pmkid: *const u8,
    pub pmk: *const u8,
    pub pmk_len: usize,
    pub ssid: *const u8,
    pub ssid_len: usize,
    pub cache_id: *const u8,
    pub pmk_lifetime: u32,
    pub pmk_reauth_threshold: u8,
}

//
// struct cfg80211_pkt_pattern - packet pattern
// @mask: bitmask where to match pattern and where to ignore bytes,
// one bit per byte, in same format as nl80211
// @pattern: bytes to match where bitmask is 1
// @pattern_len: length of pattern (in bytes)
// @pkt_offset: packet offset (in bytes)
//
// Internal note: @mask and @pattern are allocated in one chunk of
// memory, free @mask only!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pkt_pattern {
    pub pattern: *const *const u8 mask,,
    pub pattern_len: c_int,
    pub pkt_offset: c_int,
}

//
// struct cfg80211_wowlan_tcp - TCP connection parameters
//
// @sock: (internal) socket for source port allocation
// @src: source IP address
// @dst: destination IP address
// @dst_mac: destination MAC address
// @src_port: source port
// @dst_port: destination port
// @payload_len: data payload length
// @payload: data payload buffer
// @payload_seq: payload sequence stamping configuration
// @data_interval: interval at which to send data packets
// @wake_len: wakeup payload match length
// @wake_data: wakeup payload match data
// @wake_mask: wakeup payload match mask
// @tokens_size: length of the tokens buffer
// @payload_tok: payload token usage configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_wowlan_tcp {
    pub sock: *mut socket,
    pub dst: __be32 src,,
    pub dst_port: u16 src_port,,
    pub dst_mac: [u8; ETH_ALEN],
    pub payload_len: c_int,
    pub payload: *const u8,
    pub payload_seq: nl80211_wowlan_tcp_data_seq,
    pub data_interval: u32,
    pub wake_len: u32,
    pub wake_mask: *const *const u8 wake_data,,
    pub tokens_size: u32,
// must be last, variable member
    pub payload_tok: nl80211_wowlan_tcp_data_token,
}

//
// struct cfg80211_wowlan - Wake on Wireless-LAN support info
//
// This structure defines the enabled WoWLAN triggers for the device.
// @any: wake up on any activity -- special trigger if device continues
// operating as normal during suspend
// @disconnect: wake up if getting disconnected
// @magic_pkt: wake up on receiving magic packet
// @patterns: wake up on receiving packet matching a pattern
// @n_patterns: number of patterns
// @gtk_rekey_failure: wake up on GTK rekey failure
// @eap_identity_req: wake up on EAP identity request packet
// @four_way_handshake: wake up on 4-way handshake
// @rfkill_release: wake up when rfkill is released
// @tcp: TCP connection establishment/wakeup parameters, see nl80211.h.
// NULL if not configured.
// @nd_config: configuration for the scan to be used for net detect wake.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_wowlan {
    pub patterns: *mut cfg80211_pkt_pattern,
    pub tcp: *mut cfg80211_wowlan_tcp,
    pub n_patterns: c_int,
    pub nd_config: *mut cfg80211_sched_scan_request,
}

//
// struct cfg80211_coalesce_rules - Coalesce rule parameters
//
// This structure defines coalesce rule for the device.
// @delay: maximum coalescing delay in msecs.
// @condition: condition for packet coalescence.
// see &enum nl80211_coalesce_condition.
// @patterns: array of packet patterns
// @n_patterns: number of patterns
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_coalesce_rules {
    pub delay: c_int,
    pub condition: nl80211_coalesce_condition,
    pub patterns: *mut cfg80211_pkt_pattern,
    pub n_patterns: c_int,
}

//
// struct cfg80211_coalesce - Packet coalescing settings
//
// This structure defines coalescing settings.
// @rules: array of coalesce rules
// @n_rules: number of rules
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_coalesce {
    pub n_rules: c_int,
    pub __counted_by(n_rules): cfg80211_coalesce_rules rules[],
}

//
// struct cfg80211_wowlan_nd_match - information about the match
//
// @ssid: SSID of the match that triggered the wake up
// @n_channels: Number of channels where the match occurred.  This
// value may be zero if the driver can't report the channels.
// @channels: center frequencies of the channels where a match
// occurred (in MHz)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_wowlan_nd_match {
    pub ssid: cfg80211_ssid,
    pub n_channels: c_int,
    pub __counted_by(n_channels): u32 channels[],
}

//
// struct cfg80211_wowlan_nd_info - net detect wake up information
//
// @n_matches: Number of match information instances provided in
// @matches.  This value may be zero if the driver can't provide
// match information.
// @matches: Array of pointers to matches containing information about
// the matches that triggered the wake up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_wowlan_nd_info {
    pub n_matches: c_int,
    pub __counted_by(n_matches): *mut *mut cfg80211_wowlan_nd_match matches[],
}

//
// struct cfg80211_wowlan_wakeup - wakeup report
// @disconnect: woke up by getting disconnected
// @magic_pkt: woke up by receiving magic packet
// @gtk_rekey_failure: woke up by GTK rekey failure
// @eap_identity_req: woke up by EAP identity request packet
// @four_way_handshake: woke up by 4-way handshake
// @rfkill_release: woke up by rfkill being released
// @pattern_idx: pattern that caused wakeup, -1 if not due to pattern
// @packet_present_len: copied wakeup packet data
// @packet_len: original wakeup packet length
// @packet: The packet causing the wakeup, if any.
// @packet_80211:  For pattern match, magic packet and other data
// frame triggers an 802.3 frame should be reported, for
// disconnect due to deauth 802.11 frame. This indicates which
// it is.
// @tcp_match: TCP wakeup packet received
// @tcp_connlost: TCP connection lost or failed to establish
// @tcp_nomoretokens: TCP data ran out of tokens
// @net_detect: if not %NULL, woke up because of net detect
// @unprot_deauth_disassoc: woke up due to unprotected deauth or
// disassoc frame (in MFP).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_wowlan_wakeup {
    pub pattern_idx: i32,
    pub packet_len: u32 packet_present_len,,
    pub packet: *const c_void,
    pub net_detect: *mut cfg80211_wowlan_nd_info,
}

//
// struct cfg80211_gtk_rekey_data - rekey data
// @kek: key encryption key (@kek_len bytes)
// @kck: key confirmation key (@kck_len bytes)
// @replay_ctr: replay counter (NL80211_REPLAY_CTR_LEN bytes)
// @kek_len: length of kek
// @kck_len: length of kck
// @akm: akm (oui, id)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_gtk_rekey_data {
    pub replay_ctr: *const *const *const u8 kek, kck,,
    pub akm: u32,
    pub kck_len: u8 kek_len,,
}

//
// struct cfg80211_update_ft_ies_params - FT IE Information
//
// This structure provides information needed to update the fast transition IE
//
// @md: The Mobility Domain ID, 2 Octet value
// @ie: Fast Transition IEs
// @ie_len: Length of ft_ie in octets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_update_ft_ies_params {
    pub md: u16,
    pub ie: *const u8,
    pub ie_len: usize,
}

//
// struct cfg80211_mgmt_tx_params - mgmt tx parameters
//
// This structure provides information needed to transmit a mgmt frame
//
// @chan: channel to use
// @offchan: indicates whether off channel operation is required
// @wait: duration for ROC
// @buf: buffer to transmit
// @len: buffer length
// @no_cck: don't use cck rates for this frame
// @dont_wait_for_ack: tells the low level not to wait for an ack
// @n_csa_offsets: length of csa_offsets array
// @csa_offsets: array of all the csa offsets in the frame
// @link_id: for MLO, the link ID to transmit on, -1 if not given; note
// that the link ID isn't validated (much), it's in range but the
// link might not exist (or be used by the receiver STA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_mgmt_tx_params {
    pub chan: *mut ieee80211_channel,
    pub offchan: bool,
    pub wait: c_uint,
    pub buf: *const u8,
    pub len: usize,
    pub no_cck: bool,
    pub dont_wait_for_ack: bool,
    pub n_csa_offsets: c_int,
    pub csa_offsets: *const u16,
    pub link_id: c_int,
}

//
// struct cfg80211_dscp_exception - DSCP exception
//
// @dscp: DSCP value that does not adhere to the user priority range definition
// @up: user priority value to which the corresponding DSCP value belongs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_dscp_exception {
    pub dscp: u8,
    pub up: u8,
}

//
// struct cfg80211_dscp_range - DSCP range definition for user priority
//
// @low: lowest DSCP value of this user priority range, inclusive
// @high: highest DSCP value of this user priority range, inclusive
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_dscp_range {
    pub low: u8,
    pub high: u8,
}

// QoS Map Set element length defined in IEEE Std 802.11-2012, 8.4.2.97
pub const IEEE80211_QOS_MAP_MAX_EX: c_int = 21;
pub const IEEE80211_QOS_MAP_LEN_MIN: c_int = 16;

//
// struct cfg80211_qos_map - QoS Map Information
//
// This struct defines the Interworking QoS map setting for DSCP values
//
// @num_des: number of DSCP exceptions (0..21)
// @dscp_exception: optionally up to maximum of 21 DSCP exceptions from
// the user priority DSCP range definition
// @up: DSCP range definition for a particular user priority
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_qos_map {
    pub num_des: u8,
    pub dscp_exception: [cfg80211_dscp_exception; IEEE80211_QOS_MAP_MAX_EX],
    pub up: [cfg80211_dscp_range; 8],
}

//
// DOC: Neighbor Awareness Networking (NAN)
//
// NAN uses two interface types:
//
// - %NL80211_IFTYPE_NAN: a non-netdev interface. This has two roles: (1) holds
// the configuration of all NAN activities (DE parameters, synchronisation
// parameters, local schedule, etc.), and (2) uses as the NAN Management
// Interface (NMI), which is used for NAN management communication.
//
// - %NL80211_IFTYPE_NAN_DATA: The NAN Data Interface (NDI), used for data
// communication with NAN peers.
//
// An NDI interface can only be started (IFF_UP) if the NMI one is running and
// NAN is started. Before NAN is stopped, all associated NDI interfaces
// must be stopped first.
//
// The local schedule specifies which channels the device is available on and
// when. Must be cancelled before NAN is stopped.
//
// NAN Stations
// ~~~~~~~~~~~~
//
// There are two types of stations corresponding to the two interface types:
//
// - NMI station: Represents the NAN peer. Peer-specific data such as the peer's
// schedule and the HT, VHT and HE capabilities belongs to the NMI station.
// Also used for Tx/Rx of NAN management frames to/from the peer.
// Added on the %NL80211_IFTYPE_NAN interface.
//
// - NDI station: Used for Tx/Rx of data frames (and non-NAN management frames)
// for a specific NDP established with the NAN peer. Added on the
// %NL80211_IFTYPE_NAN_DATA interface.
//
// A peer may reuse its NMI address as the NDI address. In that case, two
// separate stations should be added even though they share the same MAC
// address.
//
// HT, VHT and HE capabilities should not changes after it was set. It is the
// driver's responsibility to check that.
//
// An NDI station can only be added if the corresponding NMI station has already
// been configured with HT (and possibly VHT and HE) capabilities. It is the
// driver's responsibility to check that.
//
// All NDI stations must be removed before corresponding NMI station is removed.
// Therefore, removing a NMI station implies that the associated NDI station(s)
// (if any) will be removed first.
//
// NAN Dependencies
// ~~~~~~~~~~~~~~~~
//
// The following diagram shows the dependencies between NAN components.
// An arrow from A to B means A must be started/added before B, and B must be
// stopped/removed before A:
//
// +-------------+
// |  NMI iface  |---(local schedule)
// +------+------+
// /       \
// v         v
// +-----------+  +-------------+
// | NDI iface |  |   NMI sta   |---(peer schedule)
// +-----+-----+  +------+------+
// \
// v         v
// +----------+
// | NDI sta  |
// +----------+
//
// struct cfg80211_nan_band_config - NAN band specific configuration
//
// @chan: Pointer to the IEEE 802.11 channel structure. The channel to be used
// for NAN operations on this band. For 2.4 GHz band, this is always
// channel 6. For 5 GHz band, the channel is either 44 or 149, according
// to the regulatory constraints. If chan pointer is NULL the entire band
// configuration entry is considered invalid and should not be used.
// @rssi_close: RSSI close threshold used for NAN state transition algorithm
// as described in chapters 3.3.6 and 3.3.7 "NAN Device Role and State
// Transition" of Wi-Fi Aware Specification v4.0. If not
// specified (set to 0), default device value is used. The value should
// be greater than -60 dBm.
// @rssi_middle: RSSI middle threshold used for NAN state transition algorithm.
// as described in chapters 3.3.6 and 3.3.7 "NAN Device Role and State
// Transition" of Wi-Fi Aware Specification v4.0. If not
// specified (set to 0), default device value is used. The value should be
// greater than -75 dBm and less than rssi_close.
// @awake_dw_interval: Committed DW interval. Valid values range: 0-5. 0
// indicates no wakeup for DW and can't be used on 2.4GHz band, otherwise
// 2^(n-1).
// @disable_scan: If true, the device will not scan this band for cluster
// merge. Disabling scan on 2.4 GHz band is not allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_band_config {
    pub chan: *mut ieee80211_channel,
    pub rssi_close: i8,
    pub rssi_middle: i8,
    pub awake_dw_interval: u8,
    pub disable_scan: bool,
}

//
// struct cfg80211_nan_conf - NAN configuration
//
// This struct defines NAN configuration parameters
//
// @master_pref: master preference (1 - 255)
// @bands: operating bands, a bitmap of &enum nl80211_band values.
// For instance, for NL80211_BAND_2GHZ, bit 0 would be set
// (i.e. BIT(NL80211_BAND_2GHZ)).
// @cluster_id: cluster ID used for NAN synchronization. This is a MAC address
// that can take a value from 50-6F-9A-01-00-00 to 50-6F-9A-01-FF-FF.
// @scan_period: period (in seconds) between NAN scans.
// @scan_dwell_time: dwell time (in milliseconds) for NAN scans.
// @discovery_beacon_interval: interval (in TUs) for discovery beacons.
// @enable_dw_notification: flag to enable/disable discovery window
// notifications.
// @band_cfgs: array of band specific configurations, indexed by
// &enum nl80211_band values.
// @extra_nan_attrs: pointer to additional NAN attributes.
// @extra_nan_attrs_len: length of the additional NAN attributes.
// @vendor_elems: pointer to vendor-specific elements.
// @vendor_elems_len: length of the vendor-specific elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_conf {
    pub master_pref: u8,
    pub bands: u8,
    pub __aligned(2): u8 cluster_id[ETH_ALEN],
    pub scan_period: u16,
    pub scan_dwell_time: u16,
    pub discovery_beacon_interval: u8,
    pub enable_dw_notification: bool,
    pub band_cfgs: [cfg80211_nan_band_config; NUM_NL80211_BANDS],
    pub extra_nan_attrs: *const u8,
    pub extra_nan_attrs_len: u16,
    pub vendor_elems: *const u8,
    pub vendor_elems_len: u16,
}

pub const CFG80211_NAN_SCHED_NUM_TIME_SLOTS: c_int = 32;
//
// struct cfg80211_nan_channel - NAN channel configuration
//
// This struct defines a NAN channel configuration
//
// @chandef: the channel definition
// @channel_entry: pointer to the Channel Entry blob as defined in Wi-Fi Aware
// (TM) 4.0 specification Table 100 (Channel Entry format for the NAN
// Availability attribute).
// @rx_nss: number of spatial streams supported on this channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_channel {
    pub chandef: cfg80211_chan_def,
    pub channel_entry: *const u8,
    pub rx_nss: u8,
}

//
// struct cfg80211_nan_local_sched - NAN local schedule
//
// This struct defines NAN local schedule parameters
//
// @schedule: a mapping of time slots to chandef indexes in %nan_channels.
// An unscheduled slot will be set to %NL80211_NAN_SCHED_NOT_AVAIL_SLOT.
// @n_channels: number of channel definitions in %nan_channels.
// @nan_avail_blob: pointer to NAN Availability attribute blob.
// See %NL80211_ATTR_NAN_AVAIL_BLOB for more details.
// @nan_avail_blob_len: length of the @nan_avail_blob in bytes.
// @deferred: if true, the command containing this schedule configuration is a
// request from the device to perform an announced schedule update. This
// means that it needs to send the updated NAN availability to the peers,
// and do the actual switch on the right time (i.e. at the end of the slot
// after the slot in which the updated NAN Availability was sent).
// See %NL80211_ATTR_NAN_SCHED_DEFERRED for more details.
// If false, the schedule is applied immediately.
// @nan_channels: array of NAN channel definitions that can be scheduled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_local_sched {
    pub schedule: [u8; CFG80211_NAN_SCHED_NUM_TIME_SLOTS],
    pub n_channels: u8,
    pub nan_avail_blob: *const u8,
    pub nan_avail_blob_len: u16,
    pub deferred: bool,
    pub __counted_by(n_channels): cfg80211_nan_channel nan_channels[],
}

//
// struct cfg80211_nan_peer_map - NAN peer schedule map
//
// This struct defines a single NAN peer schedule map
//
// @map_id: map ID of this schedule map
// @schedule: a mapping of time slots to chandef indexes in the schedule's
// @nan_channels. Each slot lasts 16TUs. An unscheduled slot will be
// set to %NL80211_NAN_SCHED_NOT_AVAIL_SLOT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_peer_map {
    pub map_id: u8,
    pub schedule: [u8; CFG80211_NAN_SCHED_NUM_TIME_SLOTS],
}

pub const CFG80211_NAN_MAX_PEER_MAPS: c_int = 2;
pub const CFG80211_NAN_INVALID_MAP_ID: c_uint = 0xff;
//
// struct cfg80211_nan_peer_sched - NAN peer schedule
//
// This struct defines NAN peer schedule parameters for a peer.
//
// @peer_addr: MAC address of the peer (NMI address)
// @seq_id: sequence ID of the peer schedule.
// @committed_dw: committed DW as published by the peer.
// See %NL80211_ATTR_NAN_COMMITTED_DW
// @max_chan_switch: maximum channel switch time in microseconds as published
// by the peer. See %NL80211_ATTR_NAN_MAX_CHAN_SWITCH_TIME.
// @init_ulw: initial ULWs as published by the peer.
// @ulw_size: number of bytes in @init_ulw.
// @n_channels: number of channel definitions in @nan_channels.
// @nan_channels: array of NAN channel definitions for this schedule.
// @maps: array of peer schedule maps. Unused entries have
// map_id = %CFG80211_NAN_INVALID_MAP_ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_peer_sched {
    pub peer_addr: *const u8,
    pub seq_id: u8,
    pub committed_dw: u16,
    pub max_chan_switch: u16,
    pub init_ulw: *const u8,
    pub ulw_size: u16,
    pub n_channels: u8,
    pub nan_channels: *mut cfg80211_nan_channel,
    pub maps: [cfg80211_nan_peer_map; CFG80211_NAN_MAX_PEER_MAPS],
}

//
// enum cfg80211_nan_conf_changes - indicates changed fields in NAN
// configuration
//
// @CFG80211_NAN_CONF_CHANGED_PREF: master preference
// @CFG80211_NAN_CONF_CHANGED_BANDS: operating bands
// @CFG80211_NAN_CONF_CHANGED_CONFIG: changed additional configuration.
// When this flag is set, it indicates that some additional attribute(s)
// (other then master_pref and bands) have been changed. In this case,
// all the unchanged attributes will be properly configured to their
// previous values. The driver doesn't need to store any
// previous configuration besides master_pref and bands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_nan_conf_changes {
    CFG80211_NAN_CONF_CHANGED_PREF = BIT(0),
    CFG80211_NAN_CONF_CHANGED_BANDS = BIT(1),
    CFG80211_NAN_CONF_CHANGED_CONFIG = BIT(2),
}

//
// struct cfg80211_nan_func_filter - a NAN function Rx / Tx filter
//
// @filter: the content of the filter
// @len: the length of the filter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_func_filter {
    pub filter: *const u8,
    pub len: u8,
}

//
// struct cfg80211_nan_func - a NAN function
//
// @type: &enum nl80211_nan_function_type
// @service_id: the service ID of the function
// @publish_type: &nl80211_nan_publish_type
// @close_range: if true, the range should be limited. Threshold is
// implementation specific.
// @publish_bcast: if true, the solicited publish should be broadcasted
// @subscribe_active: if true, the subscribe is active
// @followup_id: the instance ID for follow up
// @followup_reqid: the requester instance ID for follow up
// @followup_dest: MAC address of the recipient of the follow up
// @ttl: time to live counter in DW.
// @serv_spec_info: Service Specific Info
// @serv_spec_info_len: Service Specific Info length
// @srf_include: if true, SRF is inclusive
// @srf_bf: Bloom Filter
// @srf_bf_len: Bloom Filter length
// @srf_bf_idx: Bloom Filter index
// @srf_macs: SRF MAC addresses
// @srf_num_macs: number of MAC addresses in SRF
// @rx_filters: rx filters that are matched with corresponding peer's tx_filter
// @tx_filters: filters that should be transmitted in the SDF.
// @num_rx_filters: length of &rx_filters.
// @num_tx_filters: length of &tx_filters.
// @instance_id: driver allocated id of the function.
// @cookie: unique NAN function identifier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_func {
    pub type: nl80211_nan_function_type,
    pub service_id: [u8; NL80211_NAN_FUNC_SERVICE_ID_LEN],
    pub publish_type: u8,
    pub close_range: bool,
    pub publish_bcast: bool,
    pub subscribe_active: bool,
    pub followup_id: u8,
    pub followup_reqid: u8,
    pub followup_dest: mac_address,
    pub ttl: u32,
    pub serv_spec_info: *const u8,
    pub serv_spec_info_len: u8,
    pub srf_include: bool,
    pub srf_bf: *const u8,
    pub srf_bf_len: u8,
    pub srf_bf_idx: u8,
    pub srf_macs: *mut mac_address,
    pub srf_num_macs: c_int,
    pub rx_filters: *mut cfg80211_nan_func_filter,
    pub tx_filters: *mut cfg80211_nan_func_filter,
    pub num_tx_filters: u8,
    pub num_rx_filters: u8,
    pub instance_id: u8,
    pub cookie: u64,
}

//
// struct cfg80211_pmk_conf - PMK configuration
//
// @aa: authenticator address
// @pmk_len: PMK length in bytes.
// @pmk: the PMK material
// @pmk_r0_name: PMK-R0 Name. NULL if not applicable (i.e., the PMK
// is not PMK-R0). When pmk_r0_name is not NULL, the pmk field
// holds PMK-R0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmk_conf {
    pub aa: *const u8,
    pub pmk_len: u8,
    pub pmk: *const u8,
    pub pmk_r0_name: *const u8,
}

//
// struct cfg80211_external_auth_params - Trigger External authentication.
//
// Commonly used across the external auth request and event interfaces.
//
// @action: action type / trigger for external authentication. Only significant
// for the authentication request event interface (driver to user space).
// @bssid: BSSID of the peer with which the authentication has
// to happen. Used by both the authentication request event and
// authentication response command interface.
// @ssid: SSID of the AP.  Used by both the authentication request event and
// authentication response command interface.
// @key_mgmt_suite: AKM suite of the respective authentication. Used by the
// authentication request event interface.
// @status: status code, %WLAN_STATUS_SUCCESS for successful authentication,
// use %WLAN_STATUS_UNSPECIFIED_FAILURE if user space cannot give you
// the real status code for failures. Used only for the authentication
// response command interface (user space to driver).
// @pmkid: The identifier to refer a PMKSA.
// @mld_addr: MLD address of the peer. Used by the authentication request event
// interface. Driver indicates this to enable MLO during the authentication
// offload to user space. Driver shall look at %NL80211_ATTR_MLO_SUPPORT
// flag capability in NL80211_CMD_CONNECT to know whether the user space
// supports enabling MLO during the authentication offload.
// User space should use the address of the interface (on which the
// authentication request event reported) as self MLD address. User space
// and driver should use MLD addresses in RA, TA and BSSID fields of
// authentication frames sent or received via cfg80211. The driver
// translates the MLD addresses to/from link addresses based on the link
// chosen for the authentication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_external_auth_params {
    pub action: nl80211_external_auth_action,
    pub __aligned(2): u8 bssid[ETH_ALEN],
    pub ssid: cfg80211_ssid,
    pub key_mgmt_suite: c_uint,
    pub status: u16,
    pub pmkid: *const u8,
    pub __aligned(2): u8 mld_addr[ETH_ALEN],
}

//
// struct cfg80211_ftm_responder_stats - FTM responder statistics
//
// @filled: bitflag of flags using the bits of &enum nl80211_ftm_stats to
// indicate the relevant values in this struct for them
// @success_num: number of FTM sessions in which all frames were successfully
// answered
// @partial_num: number of FTM sessions in which part of frames were
// successfully answered
// @failed_num: number of failed FTM sessions
// @asap_num: number of ASAP FTM sessions
// @non_asap_num: number of  non-ASAP FTM sessions
// @total_duration_ms: total sessions durations - gives an indication
// of how much time the responder was busy
// @unknown_triggers_num: number of unknown FTM triggers - triggers from
// initiators that didn't finish successfully the negotiation phase with
// the responder
// @reschedule_requests_num: number of FTM reschedule requests - initiator asks
// for a new scheduling although it already has scheduled FTM slot
// @out_of_window_triggers_num: total FTM triggers out of scheduled window
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ftm_responder_stats {
    pub filled: u32,
    pub success_num: u32,
    pub partial_num: u32,
    pub failed_num: u32,
    pub asap_num: u32,
    pub non_asap_num: u32,
    pub total_duration_ms: u64,
    pub unknown_triggers_num: u32,
    pub reschedule_requests_num: u32,
    pub out_of_window_triggers_num: u32,
}

//
// struct cfg80211_pmsr_ftm_result - FTM result
// @failure_reason: if this measurement failed (PMSR status is
// %NL80211_PMSR_STATUS_FAILURE), this gives a more precise
// reason than just "failure"
// @burst_index: if reporting partial results, this is the index
// in [0 .. num_bursts-1] of the burst that's being reported
// @num_ftmr_attempts: number of FTM request frames transmitted
// @num_ftmr_successes: number of FTM request frames acked
// @busy_retry_time: if failure_reason is %NL80211_PMSR_FTM_FAILURE_PEER_BUSY,
// fill this to indicate in how many seconds a retry is deemed possible
// by the responder
// @num_bursts_exp: actual number of bursts exponent negotiated
// @burst_duration: actual burst duration negotiated
// @ftms_per_burst: actual FTMs per burst negotiated
// @burst_period: actual burst period negotiated in units of 100ms
// @lci_len: length of LCI information (if present)
// @civicloc_len: length of civic location information (if present)
// @lci: LCI data (may be %NULL)
// @civicloc: civic location data (may be %NULL)
// @rssi_avg: average RSSI over FTM action frames reported
// @rssi_spread: spread of the RSSI over FTM action frames reported
// @tx_rate: bitrate for transmitted FTM action frame response
// @rx_rate: bitrate of received FTM action frame
// @rtt_avg: average of RTTs measured (must have either this or @dist_avg)
// @rtt_variance: variance of RTTs measured (note that standard deviation is
// the square root of the variance)
// @rtt_spread: spread of the RTTs measured
// @dist_avg: average of distances (mm) measured
// (must have either this or @rtt_avg)
// @dist_variance: variance of distances measured (see also @rtt_variance)
// @dist_spread: spread of distances measured (see also @rtt_spread)
// @tx_ltf_repetition_count: negotiated value of number of tx ltf repetitions
// in NDP frames
// @rx_ltf_repetition_count: negotiated value of number of rx ltf repetitions
// in NDP frames
// @max_time_between_measurements: the negotiated maximum interval (in units of
// 10 ms) by which the ISTA must complete the next measurement cycle.
// @min_time_between_measurements: the negotiated minimum interval (in units of
// 100 us) between two consecutive range measurements initiated by the
// ISTA.
// @num_tx_spatial_streams: number of Tx space-time streams used in the NDP
// frame during the measurement sounding phase.
// @num_rx_spatial_streams: number of Rx space-time streams used in the NDP
// frame during the measurement sounding phase.
// @nominal_time: negotiated nominal duration between adjacent availability
// windows in units of milliseconds (u32).
// @availability_window: negotiated availability window time used in this
// session in units of milliseconds (u8).
// @chan_width: band width used for measurement.
// @preamble: preamble used for measurement.
// @num_ftmr_attempts_valid: @num_ftmr_attempts is valid
// @num_ftmr_successes_valid: @num_ftmr_successes is valid
// @rssi_avg_valid: @rssi_avg is valid
// @rssi_spread_valid: @rssi_spread is valid
// @tx_rate_valid: @tx_rate is valid
// @rx_rate_valid: @rx_rate is valid
// @rtt_avg_valid: @rtt_avg is valid
// @rtt_variance_valid: @rtt_variance is valid
// @rtt_spread_valid: @rtt_spread is valid
// @dist_avg_valid: @dist_avg is valid
// @dist_variance_valid: @dist_variance is valid
// @dist_spread_valid: @dist_spread is valid
// @tx_ltf_repetition_count_valid: @tx_ltf_repetition_count is valid
// @rx_ltf_repetition_count_valid: @rx_ltf_repetition_count is valid
// @max_time_between_measurements_valid: @max_time_between_measurements is valid
// @min_time_between_measurements_valid: @min_time_between_measurements is valid
// @num_tx_spatial_streams_valid: @num_tx_spatial_streams is valid
// @num_rx_spatial_streams_valid: @num_rx_spatial_streams is valid
// @nominal_time_valid: @nominal_time is valid
// @availability_window_valid: @availability_window is valid
// @chan_width_valid: @chan_width is valid.
// @preamble_valid: @preamble is valid.
// @is_delayed_lmr: indicates if the reported LMR is of the current burst or the
// previous burst, flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmsr_ftm_result {
    pub lci: *const u8,
    pub civicloc: *const u8,
    pub lci_len: c_uint,
    pub civicloc_len: c_uint,
    pub failure_reason: nl80211_peer_measurement_ftm_failure_reasons,
    pub num_ftmr_successes: u32 num_ftmr_attempts,,
    pub burst_index: i16,
    pub busy_retry_time: u8,
    pub num_bursts_exp: u8,
    pub burst_duration: u8,
    pub ftms_per_burst: u8,
    pub burst_period: u16,
    pub rssi_avg: i32,
    pub rssi_spread: i32,
    pub rx_rate: rate_info tx_rate,,
    pub rtt_avg: i64,
    pub rtt_variance: i64,
    pub rtt_spread: i64,
    pub dist_avg: i64,
    pub dist_variance: i64,
    pub dist_spread: i64,
    pub tx_ltf_repetition_count: u32,
    pub rx_ltf_repetition_count: u32,
    pub max_time_between_measurements: u32,
    pub min_time_between_measurements: u32,
    pub num_tx_spatial_streams: u8,
    pub num_rx_spatial_streams: u8,
    pub nominal_time: u32,
    pub availability_window: u8,
    pub chan_width: nl80211_chan_width,
    pub preamble: nl80211_preamble,
}

//
// struct cfg80211_pmsr_result - peer measurement result
// @addr: address of the peer
// @host_time: host time (use ktime_get_boottime() adjust to the time when the
// measurement was made)
// @ap_tsf: AP's TSF at measurement time
// @status: status of the measurement
// @final: if reporting partial results, mark this as the last one; if not
// reporting partial results always set this flag
// @ap_tsf_valid: indicates the @ap_tsf value is valid
// @type: type of the measurement reported, note that we only support reporting
// one type at a time, but you can report multiple results separately and
// they're all aggregated for userspace.
// @ftm: FTM result
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmsr_result {
    pub ap_tsf: u64 host_time,,
    pub status: nl80211_peer_measurement_status,
    pub addr: [u8; ETH_ALEN],
    pub type: nl80211_peer_measurement_type,
    pub ftm: cfg80211_pmsr_ftm_result,
}

//
// struct cfg80211_pmsr_ftm_request_peer - FTM request data
// @requested: indicates FTM is requested
// @preamble: frame preamble to use
// @burst_period: burst period to use
// @asap: indicates to use ASAP mode
// @num_bursts_exp: number of bursts exponent
// @burst_duration: burst duration. If @trigger_based or @non_trigger_based is
// set, this is the burst duration in milliseconds, and zero means the
// device should pick an appropriate value based on @ftms_per_burst.
// @ftms_per_burst: number of FTMs per burst. If set to 0, the firmware or
// driver can automatically select an appropriate value.
// @ftmr_retries: number of retries for FTM request
// @request_lci: request LCI information
// @request_civicloc: request civic location information
// @trigger_based: use trigger based ranging for the measurement
// If neither @trigger_based nor @non_trigger_based is set,
// EDCA based ranging will be used.
// @non_trigger_based: use non trigger based ranging for the measurement
// If neither @trigger_based nor @non_trigger_based is set,
// EDCA based ranging will be used.
// @lmr_feedback: negotiate for I2R LMR feedback. Only valid if either
// @trigger_based or @non_trigger_based is set.
// @rsta: Operate as the RSTA in the measurement. Only valid if @lmr_feedback
// and either @trigger_based or @non_trigger_based is set.
// @bss_color: the bss color of the responder. Optional. Set to zero to
// indicate the driver should set the BSS color. Only valid if
// @non_trigger_based or @trigger_based is set.
// @request_type: ranging request type, one of
// &enum nl80211_peer_measurement_ftm_req_type. Defaults to
// %NL80211_PMSR_FTM_REQ_TYPE_INFRA if not specified.
// @min_time_between_measurements: minimum time between two consecutive range
// measurements in units of 100 microseconds, for non-trigger based
// ranging. Should be set as short as possible to minimize turnaround
// time, since two-way ranging with delayed LMR requires two measurements.
// Only valid if @non_trigger_based is set.
// @max_time_between_measurements: maximum time between two consecutive range
// measurements in units of 10 milliseconds, for non-trigger based
// ranging. Acts as a session timeout; if exceeded, the ranging session
// should be terminated. Only valid if @non_trigger_based is set.
// @availability_window: duration of the availability window (AW) in units of
// 1 millisecond (0-255 ms). Only valid if @non_trigger_based is set.
// If set to 0, the firmware or driver can automatically select an
// appropriate value.
// @nominal_time: Nominal duration between adjacent availability windows
// in units of milli seconds. Only valid if @non_trigger_based is set.
// If set to 0, the firmware or driver can automatically select an
// appropriate value.
// @num_measurements: number of Availability Windows (AWs) to schedule
// for non-trigger-based ranging. Each AW may contain multiple FTM
// exchanges as configured by @ftms_per_burst. Only valid if
// @non_trigger_based is set. If set to 0, the firmware or driver
// can automatically select an appropriate value.
// @ingress_distance: optional ingress threshold in units of mm. When set,
// the measurement result of the peer needs to be indicated if the device
// moves into this range. Measurement results need to be sent on a burst
// index basis in this case.
// @egress_distance: optional egress threshold in units of mm. When set,
// the measurement result of the peer needs to be indicated if the device
// moves out of this range. Measurement results need to be sent on a burst
// index basis in this case.
// If neither or only one of @ingress_distance and @egress_distance
// is set, only the specified threshold is used. If both are set, both
// thresholds are applied. If neither is set, results are reported without
// threshold filtering.
// @pd_suppress_range_results: flag to suppress ranging results for PD
// requests. When set, the device performs ranging measurements to
// provide ranging services to a peer (e.g. in RSTA role) but does
// not report the measurement results to userspace. Only valid when
// @request_type is %NL80211_PMSR_FTM_REQ_TYPE_PD.
//
// See also nl80211 for the respective attribute documentation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmsr_ftm_request_peer {
    pub preamble: nl80211_preamble,
    pub burst_period: u16,
    pub num_bursts_exp: u8,
    pub burst_duration: u8,
    pub ftms_per_burst: u8,
    pub ftmr_retries: u8,
    pub bss_color: u8,
    pub request_type: u32,
    pub min_time_between_measurements: u32,
    pub max_time_between_measurements: u32,
    pub availability_window: u8,
    pub nominal_time: u32,
    pub num_measurements: u32,
    pub ingress_distance: u64,
    pub egress_distance: u64,
    pub pd_suppress_range_results:1: u8,
}

//
// struct cfg80211_pmsr_request_peer - peer data for a peer measurement request
// @addr: MAC address
// @chandef: channel to use
// @report_ap_tsf: report the associated AP's TSF
// @ftm: FTM data, see &struct cfg80211_pmsr_ftm_request_peer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmsr_request_peer {
    pub addr: [u8; ETH_ALEN],
    pub chandef: cfg80211_chan_def,
    pub report_ap_tsf:1: u8,
    pub ftm: cfg80211_pmsr_ftm_request_peer,
}

//
// struct cfg80211_pmsr_request - peer measurement request
// @cookie: cookie, set by cfg80211
// @nl_portid: netlink portid - used by cfg80211
// @drv_data: driver data for this request, if required for aborting,
// not otherwise freed or anything by cfg80211
// @mac_addr: MAC address used for (randomised) request
// @mac_addr_mask: MAC address mask used for randomisation, bits that
// are 0 in the mask should be randomised, bits that are 1 should
// be taken from the @mac_addr
// @list: used by cfg80211 to hold on to the request
// @timeout: timeout (in milliseconds) for the whole operation, if
// zero it means there's no timeout
// @n_peers: number of peers to do measurements with
// @peers: per-peer measurement request data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmsr_request {
    pub cookie: u64,
    pub drv_data: *mut c_void,
    pub n_peers: u32,
    pub nl_portid: u32,
    pub timeout: u32,
    pub __aligned(2): u8 mac_addr[ETH_ALEN],
    pub __aligned(2): u8 mac_addr_mask[ETH_ALEN],
    pub list: list_head,
    pub __counted_by(n_peers): cfg80211_pmsr_request_peer peers[],
}

//
// struct cfg80211_update_owe_info - OWE Information
//
// This structure provides information needed for the drivers to offload OWE
// (Opportunistic Wireless Encryption) processing to the user space.
//
// Commonly used across update_owe_info request and event interfaces.
//
// @peer: MAC address of the peer device for which the OWE processing
// has to be done.
// @status: status code, %WLAN_STATUS_SUCCESS for successful OWE info
// processing, use %WLAN_STATUS_UNSPECIFIED_FAILURE if user space
// cannot give you the real status code for failures. Used only for
// OWE update request command interface (user space to driver).
// @ie: IEs obtained from the peer or constructed by the user space. These are
// the IEs of the remote peer in the event from the host driver and
// the constructed IEs by the user space in the request interface.
// @ie_len: Length of IEs in octets.
// @assoc_link_id: MLO link ID of the AP, with which (re)association requested
// by peer. This will be filled by driver for both MLO and non-MLO station
// connections when the AP affiliated with an MLD. For non-MLD AP mode, it
// will be -1. Used only with OWE update event (driver to user space).
// @peer_mld_addr: For MLO connection, MLD address of the peer. For non-MLO
// connection, it will be all zeros. This is applicable only when
// @assoc_link_id is not -1, i.e., the AP affiliated with an MLD. Used only
// with OWE update event (driver to user space).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_update_owe_info {
    pub __aligned(2): u8 peer[ETH_ALEN],
    pub status: u16,
    pub ie: *const u8,
    pub ie_len: usize,
    pub assoc_link_id: c_int,
    pub __aligned(2): u8 peer_mld_addr[ETH_ALEN],
}

//
// struct mgmt_frame_regs - management frame registrations data
// @global_stypes: bitmap of management frame subtypes registered
// for the entire device
// @interface_stypes: bitmap of management frame subtypes registered
// for the given interface
// @global_mcast_stypes: mcast RX is needed globally for these subtypes
// @interface_mcast_stypes: mcast RX is needed on this interface
// for these subtypes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_frame_regs {
    pub interface_stypes: u32 global_stypes,,
    pub interface_mcast_stypes: u32 global_mcast_stypes,,
}

//
// struct cfg80211_ops - backend description for wireless configuration
//
// This struct is registered by fullmac card drivers and/or wireless stacks
// in order to handle configuration requests on their interfaces.
//
// All callbacks except where otherwise noted should return 0
// on success or a negative error code.
//
// All operations are invoked with the wiphy mutex held. The RTNL may be
// held in addition (due to wireless extensions) but this cannot be relied
// upon except in cases where documented below. Note that due to ordering,
// the RTNL also cannot be acquired in any handlers.
//
// @suspend: wiphy device needs to be suspended. The variable @wow will
// be %NULL or contain the enabled Wake-on-Wireless triggers that are
// configured for the device.
// @resume: wiphy device needs to be resumed
// @set_wakeup: Called when WoWLAN is enabled/disabled, use this callback
// to call device_set_wakeup_enable() to enable/disable wakeup from
// the device.
//
// @add_virtual_intf: create a new virtual interface with the given name,
// must set the struct wireless_dev's iftype. Beware: You must create
// the new netdev in the wiphy's network namespace! Returns the struct
// wireless_dev, or an ERR_PTR. For P2P device wdevs, the driver must
// also set the address member in the wdev.
// This additionally holds the RTNL to be able to do netdev changes.
//
// @del_virtual_intf: remove the virtual interface
// This additionally holds the RTNL to be able to do netdev changes.
//
// @change_virtual_intf: change type/configuration of virtual interface,
// keep the struct wireless_dev's iftype updated.
// This additionally holds the RTNL to be able to do netdev changes.
//
// @add_intf_link: Add a new MLO link to the given interface. Note that
// the wdev->link[] data structure has been updated, so the new link
// address is available.
// @del_intf_link: Remove an MLO link from the given interface.
//
// @add_key: add a key with the given parameters. @mac_addr will be %NULL
// when adding a group key. @link_id will be -1 for non-MLO connection.
// For MLO connection, @link_id will be >= 0 for group key and -1 for
// pairwise key, @mac_addr will be peer's MLD address for MLO pairwise key.
//
// @get_key: get information about the key with the given parameters.
// @mac_addr will be %NULL when requesting information for a group
// key. All pointers given to the @callback function need not be valid
// after it returns. This function should return an error if it is
// not possible to retrieve the key, -ENOENT if it doesn't exist.
// @link_id will be -1 for non-MLO connection. For MLO connection,
// @link_id will be >= 0 for group key and -1 for pairwise key, @mac_addr
// will be peer's MLD address for MLO pairwise key.
//
// @del_key: remove a key given the @mac_addr (%NULL for a group key)
// and @key_index, return -ENOENT if the key doesn't exist. @link_id will
// be -1 for non-MLO connection. For MLO connection, @link_id will be >= 0
// for group key and -1 for pairwise key, @mac_addr will be peer's MLD
// address for MLO pairwise key.
//
// @set_default_key: set the default key on an interface. @link_id will be >= 0
// for MLO connection and -1 for non-MLO connection.
//
// @set_default_mgmt_key: set the default management frame key on an interface.
// @link_id will be >= 0 for MLO connection and -1 for non-MLO connection.
//
// @set_default_beacon_key: set the default Beacon frame key on an interface.
// @link_id will be >= 0 for MLO connection and -1 for non-MLO connection.
//
// @set_rekey_data: give the data necessary for GTK rekeying to the driver
//
// @start_ap: Start acting in AP mode defined by the parameters.
// @change_beacon: Change the beacon parameters for an access point mode
// interface. This should reject the call when AP mode wasn't started.
// @stop_ap: Stop being an AP, including stopping beaconing.
//
// @add_station: Add a new station.
// @del_station: Remove a station
// @change_station: Modify a given station. Note that flags changes are not much
// validated in cfg80211, in particular the auth/assoc/authorized flags
// might come to the driver in invalid combinations -- make sure to check
// them, also against the existing state! Drivers must call
// cfg80211_check_station_change() to validate the information.
// @get_station: get station information for the station identified by @mac
// @dump_station: dump station callback -- resume dump at index @idx
//
// @add_mpath: add a fixed mesh path
// @del_mpath: delete a given mesh path
// @change_mpath: change a given mesh path
// @get_mpath: get a mesh path for the given parameters
// @dump_mpath: dump mesh path callback -- resume dump at index @idx
// @get_mpp: get a mesh proxy path for the given parameters
// @dump_mpp: dump mesh proxy path callback -- resume dump at index @idx
// @join_mesh: join the mesh network with the specified parameters
// (invoked with the wireless_dev mutex held)
// @leave_mesh: leave the current mesh network
// (invoked with the wireless_dev mutex held)
//
// @get_mesh_config: Get the current mesh configuration
//
// @update_mesh_config: Update mesh parameters on a running mesh.
// The mask is a bitfield which tells us which parameters to
// set, and which to leave alone.
//
// @change_bss: Modify parameters for a given BSS.
//
// @inform_bss: Called by cfg80211 while being informed about new BSS data
// for every BSS found within the reported data or frame. This is called
// from within the cfg8011 inform_bss handlers while holding the bss_lock.
// The data parameter is passed through from drv_data inside
// struct cfg80211_inform_bss.
// The new IE data for the BSS is explicitly passed.
//
// @set_txq_params: Set TX queue parameters
//
// @libertas_set_mesh_channel: Only for backward compatibility for libertas,
// as it doesn't implement join_mesh and needs to set the channel to
// join the mesh instead.
//
// @set_monitor_channel: Set the monitor mode channel for the device. If other
// interfaces are active this callback should reject the configuration.
// If no interfaces are active or the device is down, the channel should
// be stored for when a monitor interface becomes active.
//
// @scan: Request to do a scan. If returning zero, the scan request is given
// the driver, and will be valid until passed to cfg80211_scan_done().
// For scan results, call cfg80211_inform_bss(); you can call this outside
// the scan/scan_done bracket too.
// @abort_scan: Tell the driver to abort an ongoing scan. The driver shall
// indicate the status of the scan through cfg80211_scan_done().
//
// @auth: Request to authenticate with the specified peer
// (invoked with the wireless_dev mutex held)
// @assoc: Request to (re)associate with the specified peer
// (invoked with the wireless_dev mutex held)
// @deauth: Request to deauthenticate from the specified peer
// (invoked with the wireless_dev mutex held)
// @disassoc: Request to disassociate from the specified peer
// (invoked with the wireless_dev mutex held)
//
// @connect: Connect to the ESS with the specified parameters. When connected,
// call cfg80211_connect_result()/cfg80211_connect_bss() with status code
// %WLAN_STATUS_SUCCESS. If the connection fails for some reason, call
// cfg80211_connect_result()/cfg80211_connect_bss() with the status code
// from the AP or cfg80211_connect_timeout() if no frame with status code
// was received.
// The driver is allowed to roam to other BSSes within the ESS when the
// other BSS matches the connect parameters. When such roaming is initiated
// by the driver, the driver is expected to verify that the target matches
// the configured security parameters and to use Reassociation Request
// frame instead of Association Request frame.
// The connect function can also be used to request the driver to perform a
// specific roam when connected to an ESS. In that case, the prev_bssid
// parameter is set to the BSSID of the currently associated BSS as an
// indication of requesting reassociation.
// In both the driver-initiated and new connect() call initiated roaming
// cases, the result of roaming is indicated with a call to
// cfg80211_roamed(). (invoked with the wireless_dev mutex held)
// @update_connect_params: Update the connect parameters while connected to a
// BSS. The updated parameters can be used by driver/firmware for
// subsequent BSS selection (roaming) decisions and to form the
// Authentication/(Re)Association Request frames. This call does not
// request an immediate disassociation or reassociation with the current
// BSS, i.e., this impacts only subsequent (re)associations. The bits in
// changed are defined in &enum cfg80211_connect_params_changed.
// (invoked with the wireless_dev mutex held)
// @disconnect: Disconnect from the BSS/ESS or stop connection attempts if
// connection is in progress. Once done, call cfg80211_disconnected() in
// case connection was already established (invoked with the
// wireless_dev mutex held), otherwise call cfg80211_connect_timeout().
//
// @join_ibss: Join the specified IBSS (or create if necessary). Once done, call
// cfg80211_ibss_joined(), also call that function when changing BSSID due
// to a merge.
// (invoked with the wireless_dev mutex held)
// @leave_ibss: Leave the IBSS.
// (invoked with the wireless_dev mutex held)
//
// @set_mcast_rate: Set the specified multicast rate (only if vif is in ADHOC or
// MESH mode)
//
// @set_wiphy_params: Notify that wiphy parameters have changed;
// @changed bitfield (see &enum wiphy_params_flags) describes which values
// have changed. The actual parameter values are available in
// struct wiphy. If returning an error, no value should be changed.
//
// @set_tx_power: set the transmit power according to the parameters,
// the power passed is in mBm, to get dBm use MBM_TO_DBM(). The
// wdev may be %NULL if power was set for the wiphy, and will
// always be %NULL unless the driver supports per-vif TX power
// (as advertised by the nl80211 feature flag.)
// @get_tx_power: store the current TX power into the dbm variable;
// return 0 if successful
//
// @rfkill_poll: polls the hw rfkill line, use cfg80211 reporting
// functions to adjust rfkill hw state
//
// @dump_survey: get site survey information.
//
// @remain_on_channel: Request the driver to remain awake on the specified
// channel for the specified duration to complete an off-channel
// operation (e.g., public action frame exchange). When the driver is
// ready on the requested channel, it must indicate this with an event
// notification by calling cfg80211_ready_on_channel().
// @cancel_remain_on_channel: Cancel an on-going remain-on-channel operation.
// This allows the operation to be terminated prior to timeout based on
// the duration value.
// @mgmt_tx: Transmit a management frame.
// @mgmt_tx_cancel_wait: Cancel the wait time from transmitting a management
// frame on another channel
//
// @testmode_cmd: run a test mode command; @wdev may be %NULL
// @testmode_dump: Implement a test mode dump. The cb->args[2] and up may be
// used by the function, but 0 and 1 must not be touched. Additionally,
// return error codes other than -ENOBUFS and -ENOENT will terminate the
// dump and return to userspace with an error, so be careful. If any data
// was passed in from userspace then the data/len arguments will be present
// and point to the data contained in %NL80211_ATTR_TESTDATA.
//
// @set_bitrate_mask: set the bitrate mask configuration
//
// @set_pmksa: Cache a PMKID for a BSSID. This is mostly useful for fullmac
// devices running firmwares capable of generating the (re) association
// RSN IE. It allows for faster roaming between WPA2 BSSIDs.
// @del_pmksa: Delete a cached PMKID.
// @flush_pmksa: Flush all cached PMKIDs.
// @set_power_mgmt: Configure WLAN power management. A timeout value of -1
// allows the driver to adjust the dynamic ps timeout value.
// @set_cqm_rssi_config: Configure connection quality monitor RSSI threshold.
// After configuration, the driver should (soon) send an event indicating
// the current level is above/below the configured threshold; this may
// need some care when the configuration is changed (without first being
// disabled.)
// @set_cqm_rssi_range_config: Configure two RSSI thresholds in the
// connection quality monitor.  An event is to be sent only when the
// signal level is found to be outside the two values.  The driver should
// set %NL80211_EXT_FEATURE_CQM_RSSI_LIST if this method is implemented.
// If it is provided then there's no point providing @set_cqm_rssi_config.
// @set_cqm_txe_config: Configure connection quality monitor TX error
// thresholds.
// @sched_scan_start: Tell the driver to start a scheduled scan.
// @sched_scan_stop: Tell the driver to stop an ongoing scheduled scan with
// given request id. This call must stop the scheduled scan and be ready
// for starting a new one before it returns, i.e. @sched_scan_start may be
// called immediately after that again and should not fail in that case.
// The driver should not call cfg80211_sched_scan_stopped() for a requested
// stop (when this method returns 0).
//
// @update_mgmt_frame_registrations: Notify the driver that management frame
// registrations were updated. The callback is allowed to sleep.
//
// @set_antenna: Set antenna configuration (tx_ant, rx_ant) on the device.
// Parameters are bitmaps of allowed antennas to use for TX/RX. Drivers may
// reject TX/RX mask combinations they cannot support by returning -EINVAL
// (also see nl80211.h @NL80211_ATTR_WIPHY_ANTENNA_TX).
//
// @get_antenna: Get current antenna configuration from device (tx_ant, rx_ant).
//
// @tdls_mgmt: Transmit a TDLS management frame.
// @tdls_oper: Perform a high-level TDLS operation (e.g. TDLS link setup).
//
// @probe_peer: probe a connected peer (AP: STA MAC required; STA: no MAC),
// must use the @cookie as provided which is later passed to
// cfg80211_probe_status().
//
// @set_noack_map: Set the NoAck Map for the TIDs.
//
// @get_channel: Get the current operating channel for the virtual interface.
// For monitor interfaces, it should return %NULL unless there's a single
// current monitoring channel.
//
// @start_p2p_device: Start the given P2P device.
// @stop_p2p_device: Stop the given P2P device.
//
// @set_mac_acl: Sets MAC address control list in AP and P2P GO mode.
// Parameters include ACL policy, an array of MAC address of stations
// and the number of MAC addresses. If there is already a list in driver
// this new list replaces the existing one. Driver has to clear its ACL
// when number of MAC addresses entries is passed as 0. Drivers which
// advertise the support for MAC based ACL have to implement this callback.
//
// @start_radar_detection: Start radar detection in the driver.
//
// @end_cac: End running CAC, probably because a related CAC
// was finished on another phy.
//
// @update_ft_ies: Provide updated Fast BSS Transition information to the
// driver. If the SME is in the driver/firmware, this information can be
// used in building Authentication and Reassociation Request frames.
//
// @crit_proto_start: Indicates a critical protocol needs more link reliability
// for a given duration (milliseconds). The protocol is provided so the
// driver can take the most appropriate actions.
// @crit_proto_stop: Indicates critical protocol no longer needs increased link
// reliability. This operation can not fail.
// @set_coalesce: Set coalesce parameters.
//
// @channel_switch: initiate channel-switch procedure (with CSA). Driver is
// responsible for veryfing if the switch is possible. Since this is
// inherently tricky driver may decide to disconnect an interface later
// with cfg80211_stop_iface(). This doesn't mean driver can accept
// everything. It should do it's best to verify requests and reject them
// as soon as possible.
//
// @set_qos_map: Set QoS mapping information to the driver
//
// @set_ap_chanwidth: Set the AP (including P2P GO) mode channel width for the
// given interface This is used e.g. for dynamic HT 20/40 MHz channel width
// changes during the lifetime of the BSS.
//
// @add_tx_ts: validate (if admitted_time is 0) or add a TX TS to the device
// with the given parameters; action frame exchange has been handled by
// userspace so this just has to modify the TX path to take the TS into
// account.
// If the admitted time is 0 just validate the parameters to make sure
// the session can be created at all; it is valid to just always return
// success for that but that may result in inefficient behaviour (handshake
// with the peer followed by immediate teardown when the addition is later
// rejected)
// @del_tx_ts: remove an existing TX TS
//
// @join_ocb: join the OCB network with the specified parameters
// (invoked with the wireless_dev mutex held)
// @leave_ocb: leave the current OCB network
// (invoked with the wireless_dev mutex held)
//
// @tdls_channel_switch: Start channel-switching with a TDLS peer. The driver
// is responsible for continually initiating channel-switching operations
// and returning to the base channel for communication with the AP.
// @tdls_cancel_channel_switch: Stop channel-switching with a TDLS peer. Both
// peers must be on the base channel when the call completes.
// @start_nan: Start the NAN interface.
// @stop_nan: Stop the NAN interface.
// @add_nan_func: Add a NAN function. Returns negative value on failure.
// On success @nan_func ownership is transferred to the driver and
// it may access it outside of the scope of this function. The driver
// should free the @nan_func when no longer needed by calling
// cfg80211_free_nan_func().
// On success the driver should assign an instance_id in the
// provided @nan_func.
// @del_nan_func: Delete a NAN function.
// @nan_change_conf: changes NAN configuration. The changed parameters must
// be specified in @changes (using &enum cfg80211_nan_conf_changes);
// All other parameters must be ignored.
// @nan_set_local_sched: configure the local schedule for NAN. The schedule
// consists of an array of %cfg80211_nan_channel and the schedule itself,
// in which each entry maps each time slot to the channel on which the
// radio should operate on. If the chandef of a NAN channel is not
// changed, the channel entry must also remain unchanged. It is the
// driver's responsibility to verify this.
// @nan_set_peer_sched: configure the peer schedule for NAN. The schedule
// consists of an array of %cfg80211_nan_channel and the schedule itself,
// in which each entry maps each time slot to a channel on which the
// radio should operate on. In addition, it contains more peer's schedule
// information such as committed DW, etc. When updating an existing peer
// schedule, the full new schedule is provided - partial updates are not
// supported, and the new schedule completely replaces the previous one.
//
// @set_multicast_to_unicast: configure multicast to unicast conversion for BSS
//
// @get_txq_stats: Get TXQ stats for interface or phy. If wdev is %NULL, this
// function should return phy stats, and interface stats otherwise.
//
// @set_pmk: configure the PMK to be used for offloaded 802.1X 4-Way handshake.
// If not deleted through @del_pmk the PMK remains valid until disconnect
// upon which the driver should clear it.
// (invoked with the wireless_dev mutex held)
// @del_pmk: delete the previously configured PMK for the given authenticator.
// (invoked with the wireless_dev mutex held)
//
// @external_auth: indicates result of offloaded authentication processing from
// user space
//
// @tx_control_port: TX a control port frame (EAPoL).  The noencrypt parameter
// tells the driver that the frame should not be encrypted. A @cookie
// value of 0 means the caller does not want TX status reporting.
//
// @get_ftm_responder_stats: Retrieve FTM responder statistics, if available.
// Statistics should be cumulative, currently no way to reset is provided.
// @start_pmsr: start peer measurement (e.g. FTM)
// @abort_pmsr: abort peer measurement
//
// @update_owe_info: Provide updated OWE info to driver. Driver implementing SME
// but offloading OWE processing to the user space will get the updated
// DH IE through this interface.
//
// @probe_mesh_link: Probe direct Mesh peer's link quality by sending data frame
// and overrule HWMP path selection algorithm.
// @set_tid_config: TID specific configuration, this can be peer or BSS specific
// This callback may sleep.
// @reset_tid_config: Reset TID specific configuration for the peer, for the
// given TIDs. This callback may sleep.
//
// @set_sar_specs: Update the SAR (TX power) settings.
//
// @color_change: Initiate a color change.
//
// @set_fils_aad: Set FILS AAD data to the AP driver so that the driver can use
// those to decrypt (Re)Association Request and encrypt (Re)Association
// Response frame.
//
// @set_radar_background: Configure dedicated offchannel chain available for
// radar/CAC detection on some hw. This chain can't be used to transmit
// or receive frames and it is bounded to a running wdev.
// Background radar/CAC detection allows to avoid the CAC downtime
// switching to a different channel during CAC detection on the selected
// radar channel.
// The caller is expected to set chandef pointer to NULL in order to
// disable background CAC/radar detection.
// @add_link_station: Add a link to a station.
// @mod_link_station: Modify a link of a station.
// @del_link_station: Remove a link of a station.
//
// @set_hw_timestamp: Enable/disable HW timestamping of TM/FTM frames.
// @set_ttlm: set the TID to link mapping.
// @set_epcs: Enable/Disable EPCS for station mode.
// @get_radio_mask: get bitmask of radios in use.
// (invoked with the wiphy mutex held)
// @assoc_ml_reconf: Request a non-AP MLO connection to perform ML
// reconfiguration, i.e., add and/or remove links to/from the
// association using ML reconfiguration action frames. Successfully added
// links will be added to the set of valid links. Successfully removed
// links will be removed from the set of valid links. The driver must
// indicate removed links by calling cfg80211_links_removed() and added
// links by calling cfg80211_mlo_reconf_add_done(). When calling
// cfg80211_mlo_reconf_add_done() the bss pointer must be given for each
// link for which MLO reconfiguration 'add' operation was requested.
//
// @start_pd: Start the PD interface.
// @stop_pd: Stop the PD interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ops {
    pub wow): *mut *mut *mut int (suspend)(struct wiphy wiphy, struct cfg80211_wowlan,
    pub wiphy): *mut *mut int (resume)(struct wiphy,
    pub enabled): *mut *mut *mut void (set_wakeup)(struct wiphy wiphy, bool,
    pub params): *mut vif_params,
    pub wdev): *mut wireless_dev,
    pub params): *mut vif_params,
    pub link_id): c_uint,
    pub link_id): c_uint,
    pub params): *const *const u8 mac_addr, struct key_params,
    pub key_params*)): *mut *mut *mut void (callback)(void cookie, struct,
    pub mac_addr): *const u8,
    pub multicast): u8 key_index, bool unicast, bool,
    pub key_index): u8,
    pub key_index): u8,
    pub settings): *mut cfg80211_ap_settings,
    pub info): *mut cfg80211_ap_update,
    pub link_id): c_uint,
    pub params): *mut station_parameters,
    pub params): *mut station_del_parameters,
    pub params): *mut station_parameters,
    pub sinfo): *const *const u8 mac, struct station_info,
    pub sinfo): *mut *mut int idx, u8 mac, struct station_info,
    pub next_hop): *const *const u8 dst, u8,
    pub dst): *const u8,
    pub next_hop): *const *const u8 dst, u8,
    pub pinfo): *mut *mut *mut u8 dst, u8 next_hop, struct mpath_info,
    pub pinfo): *mut mpath_info,
    pub pinfo): *mut *mut *mut u8 dst, u8 mpp, struct mpath_info,
    pub pinfo): *mut mpath_info,
    pub conf): *mut mesh_config,
    pub nconf): *const mesh_config,
    pub setup): *const mesh_setup,
    pub dev): *mut *mut *mut int (leave_mesh)(struct wiphy wiphy, struct net_device,
    pub setup): *mut ocb_setup,
    pub dev): *mut *mut *mut int (leave_ocb)(struct wiphy wiphy, struct net_device,
    pub params): *mut bss_parameters,
    pub data): *const *const cfg80211_bss_ies ies, void,
    pub params): *mut ieee80211_txq_params,
    pub chan): *mut ieee80211_channel,
    pub chandef): *mut cfg80211_chan_def,
    pub request): *mut cfg80211_scan_request,
    pub wdev): *mut *mut *mut void (abort_scan)(struct wiphy wiphy, struct wireless_dev,
    pub req): *mut cfg80211_auth_request,
    pub req): *mut cfg80211_assoc_request,
    pub req): *mut cfg80211_deauth_request,
    pub req): *mut cfg80211_disassoc_request,
    pub sme): *mut cfg80211_connect_params,
    pub changed): u32,
    pub reason_code): u16,
    pub params): *mut cfg80211_ibss_params,
    pub dev): *mut *mut *mut int (leave_ibss)(struct wiphy wiphy, struct net_device,
    pub rate[NUM_NL80211_BANDS]): c_int,
    pub changed): u32,
    pub mbm): nl80211_tx_power_setting type, int,
    pub dbm): *mut int radio_idx, unsigned int link_id, int,
    pub wiphy): *mut *mut void (rfkill_poll)(struct wiphy,

    pub len): *mut *mut void data, int,
    pub len): *mut *mut void data, int,

    pub mask): *const cfg80211_bitrate_mask,
    pub info): *mut int idx, struct survey_info,
    pub pmksa): *mut cfg80211_pmksa,
    pub pmksa): *mut cfg80211_pmksa,
    pub netdev): *mut *mut *mut int (flush_pmksa)(struct wiphy wiphy, struct net_device,
    pub rx_addr): *const u64 cookie, u8,
    pub cookie): u64,
    pub cookie): u64,
    pub cookie): u64,
    pub timeout): bool enabled, int,
    pub rssi_hyst): s32 rssi_thold, u32,
    pub rssi_high): s32 rssi_low, s32,
    pub intvl): u32 rate, u32 pkts, u32,
    pub upd): *mut mgmt_frame_regs,
    pub rx_ant): u32 tx_ant, u32,
    pub rx_ant): *mut *mut u32 tx_ant, u32,
    pub request): *mut cfg80211_sched_scan_request,
    pub reqid): u64,
    pub data): *mut cfg80211_gtk_rekey_data,
    pub len): *const *const u8 buf, size_t,
    pub oper): *const *const u8 peer, enum nl80211_tdls_operation,
    pub cookie): *const *const u8 peer, u64,
    pub noack_map): u16,
    pub chandef): *mut cfg80211_chan_def,
    pub wdev): *mut wireless_dev,
    pub wdev): *mut wireless_dev,
    pub params): *const cfg80211_acl_data,
    pub link_id): u32 cac_time_ms, int,
    pub link_id): *mut *mut net_device dev, unsigned int,
    pub ftie): *mut cfg80211_update_ft_ies_params,
    pub duration): u16,
    pub wdev): *mut wireless_dev,
    pub coalesce): *mut cfg80211_coalesce,
    pub params): *mut cfg80211_csa_settings,
    pub qos_map): *mut cfg80211_qos_map,
    pub chandef): *mut cfg80211_chan_def,
    pub admitted_time): u16,
    pub peer): *const u8 tsid, u8,
    pub chandef): *mut cfg80211_chan_def,
    pub addr): *const u8,
    pub conf): *mut cfg80211_nan_conf,
    pub wdev): *mut *mut *mut void (stop_nan)(struct wiphy wiphy, struct wireless_dev,
    pub nan_func): *mut cfg80211_nan_func,
    pub cookie): u64,
    pub changes): u32,
    pub sched): *mut cfg80211_nan_local_sched,
    pub sched): *mut cfg80211_nan_peer_sched,
    pub enabled): bool,
    pub txqstats): *mut cfg80211_txq_stats,
    pub conf): *const cfg80211_pmk_conf,
    pub aa): *const u8,
    pub params): *mut cfg80211_external_auth_params,
    pub cookie): u64,
    pub ftm_stats): *mut cfg80211_ftm_responder_stats,
    pub request): *mut cfg80211_pmsr_request,
    pub request): *mut cfg80211_pmsr_request,
    pub owe_info): *mut cfg80211_update_owe_info,
    pub len): *const *const u8 buf, size_t,
    pub tid_conf): *mut cfg80211_tid_config,
    pub tids): *const *const u8 peer, u8,
    pub sar): *mut cfg80211_sar_specs,
    pub params): *mut cfg80211_color_change_settings,
    pub fils_aad): *mut cfg80211_fils_aad,
    pub chandef): *mut cfg80211_chan_def,
    pub params): *mut link_station_parameters,
    pub params): *mut link_station_parameters,
    pub params): *mut link_station_del_parameters,
    pub hwts): *mut cfg80211_set_hw_timestamp,
    pub params): *mut cfg80211_ttlm_params,
    pub dev): *mut *mut *mut u32 (get_radio_mask)(struct wiphy wiphy, struct net_device,
    pub req): *mut cfg80211_ml_reconf_req,
    pub val): bool,
    pub wdev): *mut *mut *mut int (start_pd)(struct wiphy wiphy, struct wireless_dev,
    pub wdev): *mut *mut *mut void (stop_pd)(struct wiphy wiphy, struct wireless_dev,
}

//
// wireless hardware and networking interfaces structures
// and registration/helper functions
//
// enum wiphy_flags - wiphy capability flags
//
// @WIPHY_FLAG_SPLIT_SCAN_6GHZ: if set to true, the scan request will be split
// into two, first for legacy bands and second for 6 GHz.
// @WIPHY_FLAG_NETNS_OK: if not set, do not allow changing the netns of this
// wiphy at all
// @WIPHY_FLAG_PS_ON_BY_DEFAULT: if set to true, powersave will be enabled
// by default -- this flag will be set depending on the kernel's default
// on wiphy_new(), but can be changed by the driver if it has a good
// reason to override the default
// @WIPHY_FLAG_4ADDR_AP: supports 4addr mode even on AP (with a single station
// on a VLAN interface). This flag also serves an extra purpose of
// supporting 4ADDR AP mode on devices which do not support AP/VLAN iftype.
// @WIPHY_FLAG_4ADDR_STATION: supports 4addr mode even as a station
// @WIPHY_FLAG_CONTROL_PORT_PROTOCOL: This device supports setting the
// control port protocol ethertype. The device also honours the
// control_port_no_encrypt flag.
// @WIPHY_FLAG_IBSS_RSN: The device supports IBSS RSN.
// @WIPHY_FLAG_MESH_AUTH: The device supports mesh authentication by routing
// auth frames to userspace. See @NL80211_MESH_SETUP_USERSPACE_AUTH.
// @WIPHY_FLAG_SUPPORTS_FW_ROAM: The device supports roaming feature in the
// firmware.
// @WIPHY_FLAG_AP_UAPSD: The device supports uapsd on AP.
// @WIPHY_FLAG_SUPPORTS_TDLS: The device supports TDLS (802.11z) operation.
// @WIPHY_FLAG_TDLS_EXTERNAL_SETUP: The device does not handle TDLS (802.11z)
// link setup/discovery operations internally. Setup, discovery and
// teardown packets should be sent through the @NL80211_CMD_TDLS_MGMT
// command. When this flag is not set, @NL80211_CMD_TDLS_OPER should be
// used for asking the driver/firmware to perform a TDLS operation.
// @WIPHY_FLAG_HAVE_AP_SME: device integrates AP SME
// @WIPHY_FLAG_REPORTS_OBSS: the device will report beacons from other BSSes
// when there are virtual interfaces in AP mode by calling
// cfg80211_report_obss_beacon().
// @WIPHY_FLAG_AP_PROBE_RESP_OFFLOAD: When operating as an AP, the device
// responds to probe-requests in hardware.
// @WIPHY_FLAG_OFFCHAN_TX: Device supports direct off-channel TX.
// @WIPHY_FLAG_HAS_REMAIN_ON_CHANNEL: Device supports remain-on-channel call.
// @WIPHY_FLAG_HAS_CHANNEL_SWITCH: Device supports channel switch in
// beaconing mode (AP, IBSS, Mesh, ...).
// @WIPHY_FLAG_SUPPORTS_EXT_KEK_KCK: The device supports bigger kek and kck keys
// @WIPHY_FLAG_SUPPORTS_MLO: This is a temporary flag gating the MLO APIs,
// in order to not have them reachable in normal drivers, until we have
// complete feature/interface combinations/etc. advertisement. No driver
// should set this flag for now.
// @WIPHY_FLAG_SUPPORTS_EXT_KCK_32: The device supports 32-byte KCK keys.
// @WIPHY_FLAG_NOTIFY_REGDOM_BY_DRIVER: The device could handle reg notify for
// NL80211_REGDOM_SET_BY_DRIVER.
// @WIPHY_FLAG_CHANNEL_CHANGE_ON_BEACON: reg_call_notifier() is called if driver
// set this flag to update channels on beacon hints.
// @WIPHY_FLAG_SUPPORTS_NSTR_NONPRIMARY: support connection to non-primary link
// of an NSTR mobile AP MLD.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiphy_flags {
    WIPHY_FLAG_SUPPORTS_EXT_KEK_KCK		= BIT(0),
    WIPHY_FLAG_SUPPORTS_MLO			= BIT(1),
    WIPHY_FLAG_SPLIT_SCAN_6GHZ		= BIT(2),
    WIPHY_FLAG_NETNS_OK			= BIT(3),
    WIPHY_FLAG_PS_ON_BY_DEFAULT		= BIT(4),
    WIPHY_FLAG_4ADDR_AP			= BIT(5),
    WIPHY_FLAG_4ADDR_STATION		= BIT(6),
    WIPHY_FLAG_CONTROL_PORT_PROTOCOL	= BIT(7),
    WIPHY_FLAG_IBSS_RSN			= BIT(8),
// reuse bit 9
    WIPHY_FLAG_MESH_AUTH			= BIT(10),
    WIPHY_FLAG_SUPPORTS_EXT_KCK_32          = BIT(11),
    WIPHY_FLAG_SUPPORTS_NSTR_NONPRIMARY	= BIT(12),
    WIPHY_FLAG_SUPPORTS_FW_ROAM		= BIT(13),
    WIPHY_FLAG_AP_UAPSD			= BIT(14),
    WIPHY_FLAG_SUPPORTS_TDLS		= BIT(15),
    WIPHY_FLAG_TDLS_EXTERNAL_SETUP		= BIT(16),
    WIPHY_FLAG_HAVE_AP_SME			= BIT(17),
    WIPHY_FLAG_REPORTS_OBSS			= BIT(18),
    WIPHY_FLAG_AP_PROBE_RESP_OFFLOAD	= BIT(19),
    WIPHY_FLAG_OFFCHAN_TX			= BIT(20),
    WIPHY_FLAG_HAS_REMAIN_ON_CHANNEL	= BIT(21),
    WIPHY_FLAG_HAS_CHANNEL_SWITCH		= BIT(23),
    WIPHY_FLAG_NOTIFY_REGDOM_BY_DRIVER	= BIT(24),
    WIPHY_FLAG_CHANNEL_CHANGE_ON_BEACON     = BIT(25),
}

//
// struct ieee80211_iface_limit - limit on certain interface types
// @max: maximum number of interfaces of these types
// @types: interface types (bits)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_iface_limit {
    pub max: u16,
    pub types: u16,
}

//
// struct ieee80211_iface_combination - possible interface combination
//
// With this structure the driver can describe which interface
// combinations it supports concurrently. When set in a struct wiphy_radio,
// the combinations refer to combinations of interfaces currently active on
// that radio.
//
// Examples:
//
// 1. Allow #STA <= 1, #AP <= 1, matching BI, channels = 1, 2 total:
//
// .. code-block:: c
//
// struct ieee80211_iface_limit limits1[] = {
// { .max = 1, .types = BIT(NL80211_IFTYPE_STATION), },
// { .max = 1, .types = BIT(NL80211_IFTYPE_AP), },
// };
// struct ieee80211_iface_combination combination1 = {
// .limits = limits1,
// .n_limits = ARRAY_SIZE(limits1),
// .max_interfaces = 2,
// .beacon_int_infra_match = true,
// };
//
// 2. Allow #{AP, P2P-GO} <= 8, channels = 1, 8 total:
//
// .. code-block:: c
//
// struct ieee80211_iface_limit limits2[] = {
// { .max = 8, .types = BIT(NL80211_IFTYPE_AP) |
// BIT(NL80211_IFTYPE_P2P_GO), },
// };
// struct ieee80211_iface_combination combination2 = {
// .limits = limits2,
// .n_limits = ARRAY_SIZE(limits2),
// .max_interfaces = 8,
// .num_different_channels = 1,
// };
//
// 3. Allow #STA <= 1, #{P2P-client,P2P-GO} <= 3 on two channels, 4 total.
//
// This allows for an infrastructure connection and three P2P connections.
//
// .. code-block:: c
//
// struct ieee80211_iface_limit limits3[] = {
// { .max = 1, .types = BIT(NL80211_IFTYPE_STATION), },
// { .max = 3, .types = BIT(NL80211_IFTYPE_P2P_GO) |
// BIT(NL80211_IFTYPE_P2P_CLIENT), },
// };
// struct ieee80211_iface_combination combination3 = {
// .limits = limits3,
// .n_limits = ARRAY_SIZE(limits3),
// .max_interfaces = 4,
// .num_different_channels = 2,
// };
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_iface_combination {
//
// @limits:
// limits for the given interface types
//
    pub limits: *const ieee80211_iface_limit,
//
// @num_different_channels:
// can use up to this many different channels
//
    pub num_different_channels: u32,
//
// @max_interfaces:
// maximum number of interfaces in total allowed in this group
//
    pub max_interfaces: u16,
//
// @n_limits:
// number of limitations
//
    pub n_limits: u8,
//
// @beacon_int_infra_match:
// In this combination, the beacon intervals between infrastructure
// and AP types must match. This is required only in special cases.
//
    pub beacon_int_infra_match: bool,
//
// @radar_detect_widths:
// bitmap of channel widths supported for radar detection
//
    pub radar_detect_widths: u8,
//
// @radar_detect_regions:
// bitmap of regions supported for radar detection
//
    pub radar_detect_regions: u8,
//
// @beacon_int_min_gcd:
// This interface combination supports different beacon intervals.
//
// = 0
// all beacon intervals for different interface must be same.
// > 0
// any beacon interval for the interface part of this combination AND
// GCD of all beacon intervals from beaconing interfaces of this
// combination must be greater or equal to this value.
//
    pub beacon_int_min_gcd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_txrx_stypes {
    pub rx: u16 tx,,
}

//
// enum wiphy_wowlan_support_flags - WoWLAN support flags
// @WIPHY_WOWLAN_ANY: supports wakeup for the special "any"
// trigger that keeps the device operating as-is and
// wakes up the host on any activity, for example a
// received packet that passed filtering; note that the
// packet should be preserved in that case
// @WIPHY_WOWLAN_MAGIC_PKT: supports wakeup on magic packet
// (see nl80211.h)
// @WIPHY_WOWLAN_DISCONNECT: supports wakeup on disconnect
// @WIPHY_WOWLAN_SUPPORTS_GTK_REKEY: supports GTK rekeying while asleep
// @WIPHY_WOWLAN_GTK_REKEY_FAILURE: supports wakeup on GTK rekey failure
// @WIPHY_WOWLAN_EAP_IDENTITY_REQ: supports wakeup on EAP identity request
// @WIPHY_WOWLAN_4WAY_HANDSHAKE: supports wakeup on 4-way handshake failure
// @WIPHY_WOWLAN_RFKILL_RELEASE: supports wakeup on RF-kill release
// @WIPHY_WOWLAN_NET_DETECT: supports wakeup on network detection
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiphy_wowlan_support_flags {
    WIPHY_WOWLAN_ANY		= BIT(0),
    WIPHY_WOWLAN_MAGIC_PKT		= BIT(1),
    WIPHY_WOWLAN_DISCONNECT		= BIT(2),
    WIPHY_WOWLAN_SUPPORTS_GTK_REKEY	= BIT(3),
    WIPHY_WOWLAN_GTK_REKEY_FAILURE	= BIT(4),
    WIPHY_WOWLAN_EAP_IDENTITY_REQ	= BIT(5),
    WIPHY_WOWLAN_4WAY_HANDSHAKE	= BIT(6),
    WIPHY_WOWLAN_RFKILL_RELEASE	= BIT(7),
    WIPHY_WOWLAN_NET_DETECT		= BIT(8),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_wowlan_tcp_support {
    pub tok: *const nl80211_wowlan_tcp_data_token_feature,
    pub data_payload_max: u32,
    pub data_interval_max: u32,
    pub wake_payload_max: u32,
    pub seq: bool,
}

//
// struct wiphy_wowlan_support - WoWLAN support data
// @flags: see &enum wiphy_wowlan_support_flags
// @n_patterns: number of supported wakeup patterns
// (see nl80211.h for the pattern definition)
// @pattern_max_len: maximum length of each pattern
// @pattern_min_len: minimum length of each pattern
// @max_pkt_offset: maximum Rx packet offset
// @max_nd_match_sets: maximum number of matchsets for net-detect,
// similar, but not necessarily identical, to max_match_sets for
// scheduled scans.
// See &struct cfg80211_sched_scan_request.@match_sets for more
// details.
// @tcp: TCP wakeup support information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_wowlan_support {
    pub flags: u32,
    pub n_patterns: c_int,
    pub pattern_max_len: c_int,
    pub pattern_min_len: c_int,
    pub max_pkt_offset: c_int,
    pub max_nd_match_sets: c_int,
    pub tcp: *const wiphy_wowlan_tcp_support,
}

//
// struct wiphy_coalesce_support - coalesce support data
// @n_rules: maximum number of coalesce rules
// @max_delay: maximum supported coalescing delay in msecs
// @n_patterns: number of supported patterns in a rule
// (see nl80211.h for the pattern definition)
// @pattern_max_len: maximum length of each pattern
// @pattern_min_len: minimum length of each pattern
// @max_pkt_offset: maximum Rx packet offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_coalesce_support {
    pub n_rules: c_int,
    pub max_delay: c_int,
    pub n_patterns: c_int,
    pub pattern_max_len: c_int,
    pub pattern_min_len: c_int,
    pub max_pkt_offset: c_int,
}

//
// enum wiphy_vendor_command_flags - validation flags for vendor commands
// @WIPHY_VENDOR_CMD_NEED_WDEV: vendor command requires wdev
// @WIPHY_VENDOR_CMD_NEED_NETDEV: vendor command requires netdev
// @WIPHY_VENDOR_CMD_NEED_RUNNING: interface/wdev must be up & running
// (must be combined with %_WDEV or %_NETDEV)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiphy_vendor_command_flags {
    WIPHY_VENDOR_CMD_NEED_WDEV = BIT(0),
    WIPHY_VENDOR_CMD_NEED_NETDEV = BIT(1),
    WIPHY_VENDOR_CMD_NEED_RUNNING = BIT(2),
}

//
// enum wiphy_opmode_flag - Station's ht/vht operation mode information flags
//
// @STA_OPMODE_MAX_BW_CHANGED: Max Bandwidth changed
// @STA_OPMODE_SMPS_MODE_CHANGED: SMPS mode changed
// @STA_OPMODE_N_SS_CHANGED: max N_SS (number of spatial streams) changed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiphy_opmode_flag {
    STA_OPMODE_MAX_BW_CHANGED	= BIT(0),
    STA_OPMODE_SMPS_MODE_CHANGED	= BIT(1),
    STA_OPMODE_N_SS_CHANGED		= BIT(2),
}

//
// struct sta_opmode_info - Station's ht/vht operation mode information
// @changed: contains value from &enum wiphy_opmode_flag
// @smps_mode: New SMPS mode value from &enum nl80211_smps_mode of a station
// @bw: new max bandwidth value from &enum nl80211_chan_width of a station
// @rx_nss: new rx_nss value of a station
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_opmode_info {
    pub changed: u32,
    pub smps_mode: nl80211_smps_mode,
    pub bw: nl80211_chan_width,
    pub rx_nss: u8,
}

//
// struct wiphy_vendor_command - vendor command definition
// @info: vendor command identifying information, as used in nl80211
// @flags: flags, see &enum wiphy_vendor_command_flags
// @doit: callback for the operation, note that wdev is %NULL if the
// flags didn't ask for a wdev and non-%NULL otherwise; the data
// pointer may be %NULL if userspace provided no data at all
// @dumpit: dump callback, for transferring bigger/multiple items. The
// @storage points to cb->args[5], ie. is preserved over the multiple
// dumpit calls.
// @policy: policy pointer for attributes within %NL80211_ATTR_VENDOR_DATA.
// Set this to %VENDOR_CMD_RAW_DATA if no policy can be given and the
// attribute is just raw data (e.g. a firmware command).
// @maxattr: highest attribute number in policy
// It's recommended to not have the same sub command with both @doit and
// @dumpit, so that userspace can assume certain ones are get and others
// are used with dump requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_vendor_command {
    pub info: nl80211_vendor_cmd_info,
    pub flags: u32,
    pub data_len): *const *const void data, int,
    pub storage): *mut c_ulong,
    pub policy: *const nla_policy,
    pub maxattr: c_uint,
}

//
// struct wiphy_iftype_ext_capab - extended capabilities per interface type
// @iftype: interface type
// @extended_capabilities: extended capabilities supported by the driver,
// additional capabilities might be supported by userspace; these are the
// 802.11 extended capabilities ("Extended Capabilities element") and are
// in the same format as in the information element. See IEEE Std
// 802.11-2012 8.4.2.29 for the defined fields.
// @extended_capabilities_mask: mask of the valid values
// @extended_capabilities_len: length of the extended capabilities
// @eml_capabilities: EML capabilities (for MLO)
// @mld_capa_and_ops: MLD capabilities and operations (for MLO)
// @ext_mld_capa_and_ops: Extended MLD capabilities and operations (for MLO)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_iftype_ext_capab {
    pub iftype: nl80211_iftype,
    pub extended_capabilities: *const u8,
    pub extended_capabilities_mask: *const u8,
    pub extended_capabilities_len: u8,
    pub eml_capabilities: u16,
    pub mld_capa_and_ops: u16,
    pub ext_mld_capa_and_ops: u16,
}

//
// cfg80211_get_iftype_ext_capa - lookup interface type extended capability
// @wiphy: the wiphy to look up from
// @type: the interface type to look up
//
// Return: The extended capability for the given interface @type, may be %NULL
//
// struct cfg80211_pmsr_capabilities - cfg80211 peer measurement capabilities
// @max_peers: maximum number of peers in a single measurement
// @report_ap_tsf: can report assoc AP's TSF for radio resource measurement
// @randomize_mac_addr: can randomize MAC address for measurement
// @ftm: FTM measurement data
// @ftm.supported: FTM measurement is supported
// @ftm.asap: ASAP-mode is supported
// @ftm.non_asap: non-ASAP-mode is supported
// @ftm.request_lci: can request LCI data
// @ftm.request_civicloc: can request civic location data
// @ftm.preambles: bitmap of preambles supported (&enum nl80211_preamble)
// @ftm.bandwidths: bitmap of bandwidths supported (&enum nl80211_chan_width)
// @ftm.max_bursts_exponent: maximum burst exponent supported
// (set to -1 if not limited; note that setting this will necessarily
// forbid using the value 15 to let the responder pick)
// @ftm.max_ftms_per_burst: maximum FTMs per burst supported (set to 0 if
// not limited)
// @ftm.trigger_based: trigger based ranging measurement is supported
// @ftm.non_trigger_based: non trigger based ranging measurement is supported
// @ftm.support_6ghz: supports ranging in 6 GHz band
// @ftm.max_tx_ltf_rep: maximum number of TX LTF repetitions supported (0 means
// only one LTF, no repetitions)
// @ftm.max_rx_ltf_rep: maximum number of RX LTF repetitions supported (0 means
// only one LTF, no repetitions)
// @ftm.max_tx_sts: maximum number of TX STS supported (zero based)
// @ftm.max_rx_sts: maximum number of RX STS supported (zero based)
// @ftm.max_total_ltf_tx: maximum total number of LTFs that can be transmitted
// (0 means unknown)
// @ftm.max_total_ltf_rx: maximum total number of LTFs that can be received
// (0 means unknown)
// @ftm.ista: initiator role capabilities
// @ftm.ista.support_ntb: supports operating as ISTA in PMSR FTM request for
// NTB ranging.
// @ftm.ista.support_tb: supports operating as ISTA in PMSR FTM request for
// TB ranging.
// @ftm.ista.support_edca: supports operating as ISTA in PMSR FTM request for
// EDCA based ranging.
// @ftm.ista.max_peers: maximum number of peers supported in the ISTA role.
// If zero, no role-specific peer limit applies.
// @ftm.rsta: responder role capabilities
// @ftm.rsta.support_ntb: supports operating as RSTA in PMSR FTM request for
// NTB ranging.
// @ftm.rsta.support_tb: supports operating as RSTA in PMSR FTM request for
// TB ranging.
// @ftm.rsta.support_edca: supports operating as RSTA in PMSR FTM request for
// EDCA based ranging.
// @ftm.rsta.max_peers: maximum number of peers supported in the RSTA role.
// If zero, no role-specific peer limit applies.
// @ftm.max_no_of_tx_antennas: maximum number of transmit antennas supported for
// EDCA based ranging (0 means unknown)
// @ftm.max_no_of_rx_antennas: maximum number of receive antennas supported for
// EDCA based ranging (0 means unknown)
// @ftm.min_allowed_ranging_interval_edca: Minimum EDCA ranging
// interval supported by the device in milli seconds. (0 means unknown).
// Applications can use this value to estimate the burst period to be
// given in the FTM request for the EDCA based ranging case. If
// non-zero, this value will be used to validate the burst period in
// the FTM request.
// @ftm.min_allowed_ranging_interval_ntb: Minimum NTB ranging
// interval supported by the device in milli seconds. (0 means unknown).
// Applications can use this value to estimate the burst period to be
// given in the FTM request for the NTB ranging case. If non-zero,
// this value will be used to validate the nominal time in the FTM
// request.
// @ftm.type: ranging type capabilities
// @ftm.type.infra_support: supports infrastructure ranging (STA-to-AP or
// AP-to-STA) as part of Proximity Detection
// @ftm.type.pd_support: supports peer-to-peer ranging as mentioned in the
// specification "PR Implementation Consideration Draft 1.9 rev 1" where
// PD stands for proximity detection
// @ftm.concurrent_ista_rsta_support: indicates if the device can
// simultaneously act as initiator and responder in a multi-peer
// measurement request. Only valid if @ftm.rsta_support is set.
// @ftm.pd_preambles: bitmap of preambles supported (&enum nl80211_preamble)
// for PD ranging requests. Ignored if @ftm.type.pd_support is not set.
// @ftm.pd_bandwidths: bitmap of bandwidths supported (&enum nl80211_chan_width)
// for PD ranging requests. Ignored if @ftm.type.pd_support is not set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_pmsr_capabilities {
    pub max_peers: c_uint,
    pub preambles: u32,
    pub bandwidths: u32,
    pub max_bursts_exponent: i8,
    pub max_ftms_per_burst: u8,
    pub max_tx_ltf_rep: u8,
    pub max_rx_ltf_rep: u8,
    pub max_tx_sts: u8,
    pub max_rx_sts: u8,
    pub max_total_ltf_tx: u8,
    pub max_total_ltf_rx: u8,
    pub max_peers: u32,
    pub ista: },
    pub max_peers: u32,
    pub rsta: },
    pub max_no_of_tx_antennas: u8,
    pub max_no_of_rx_antennas: u8,
    pub min_allowed_ranging_interval_edca: u32,
    pub min_allowed_ranging_interval_ntb: u32,
    pub type: },
    pub concurrent_ista_rsta_support:1: u8,
    pub pd_preambles: u32,
    pub pd_bandwidths: u32,
    pub ftm: },
}

//
// struct wiphy_iftype_akm_suites - This structure encapsulates supported akm
// suites for interface types defined in @iftypes_mask. Each type in the
// @iftypes_mask must be unique across all instances of iftype_akm_suites.
//
// @iftypes_mask: bitmask of interfaces types
// @akm_suites: points to an array of supported akm suites
// @n_akm_suites: number of supported AKM suites
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_iftype_akm_suites {
    pub iftypes_mask: u16,
    pub akm_suites: *const u32,
    pub n_akm_suites: c_int,
}

//
// struct wiphy_radio_cfg - physical radio config of a wiphy
// This structure describes the configurations of a physical radio in a
// wiphy. It is used to denote per-radio attributes belonging to a wiphy.
//
// @rts_threshold: RTS threshold (dot11RTSThreshold);
// -1 (default) = RTS/CTS disabled
// @radio_debugfsdir: Pointer to debugfs directory containing the radio-
// specific parameters.
// NULL (default) = Debugfs directory not created
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_radio_cfg {
    pub rts_threshold: u32,
    pub radio_debugfsdir: *mut dentry,
}

//
// struct wiphy_radio_freq_range - wiphy frequency range
// @start_freq:  start range edge frequency (kHz)
// @end_freq:    end range edge frequency (kHz)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_radio_freq_range {
    pub start_freq: u32,
    pub end_freq: u32,
}

//
// struct wiphy_radio - physical radio of a wiphy
// This structure describes a physical radio belonging to a wiphy.
// It is used to describe concurrent-channel capabilities. Only one channel
// can be active on the radio described by struct wiphy_radio.
//
// @freq_range: frequency range that the radio can operate on.
// @n_freq_range: number of elements in @freq_range
//
// @iface_combinations: Valid interface combinations array, should not
// list single interface types.
// @n_iface_combinations: number of entries in @iface_combinations array.
//
// @antenna_mask: bitmask of antennas connected to this radio.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_radio {
    pub freq_range: *const wiphy_radio_freq_range,
    pub n_freq_range: c_int,
    pub iface_combinations: *const ieee80211_iface_combination,
    pub n_iface_combinations: c_int,
    pub antenna_mask: u32,
}

//
// enum wiphy_nan_flags - NAN capabilities
//
// @WIPHY_NAN_FLAGS_CONFIGURABLE_SYNC: Device supports NAN configurable
// synchronization.
// @WIPHY_NAN_FLAGS_USERSPACE_DE: Device doesn't support DE offload.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiphy_nan_flags {
    WIPHY_NAN_FLAGS_CONFIGURABLE_SYNC = BIT(0),
    WIPHY_NAN_FLAGS_USERSPACE_DE   = BIT(1),
}

//
// struct wiphy_nan_capa - NAN capabilities
//
// This structure describes the NAN capabilities of a wiphy.
//
// @flags: NAN capabilities flags, see &enum wiphy_nan_flags
// @op_mode: NAN operation mode, as defined in Wi-Fi Aware (TM) specification
// Table 81.
// @n_antennas: number of antennas supported by the device for Tx/Rx. Lower
// nibble indicates the number of TX antennas and upper nibble indicates the
// number of RX antennas. Value 0 indicates the information is not
// available.
// @max_channel_switch_time: maximum channel switch time in microseconds.
// @dev_capabilities: NAN device capabilities as defined in Wi-Fi Aware (TM)
// specification Table 79 (Capabilities field).
// @phy: Band-agnostic capabilities for NAN data interfaces. Since NAN
// operates on multiple channels simultaneously, these capabilities apply
// across all bands. Valid only if NL80211_IFTYPE_NAN_DATA is supported.
// @phy.ht: HT capabilities (mandatory for NAN data)
// @phy.vht: VHT capabilities (optional)
// @phy.he: HE capabilities (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_nan_capa {
    pub flags: u32,
    pub op_mode: u8,
    pub n_antennas: u8,
    pub max_channel_switch_time: u16,
    pub dev_capabilities: u8,
    pub ht: ieee80211_sta_ht_cap,
    pub vht: ieee80211_sta_vht_cap,
    pub he: ieee80211_sta_he_cap,
    pub phy: },
}

pub const CFG80211_HW_TIMESTAMP_ALL_PEERS: c_uint = 0xffff;
//
// struct wiphy - wireless hardware description
// @mtx: mutex for the data (structures) of this device
// @reg_notifier: the driver's regulatory notification callback,
// note that if your driver uses wiphy_apply_custom_regulatory()
// the reg_notifier's request can be passed as NULL
// @regd: the driver's regulatory domain, if one was requested via
// the regulatory_hint() API. This can be used by the driver
// on the reg_notifier() if it chooses to ignore future
// regulatory domain changes caused by other drivers.
// @signal_type: signal type reported in &struct cfg80211_bss.
// @cipher_suites: supported cipher suites
// @n_cipher_suites: number of supported cipher suites
// @akm_suites: supported AKM suites. These are the default AKMs supported if
// the supported AKMs not advertized for a specific interface type in
// iftype_akm_suites.
// @n_akm_suites: number of supported AKM suites
// @iftype_akm_suites: array of supported akm suites info per interface type.
// Note that the bits in @iftypes_mask inside this structure cannot
// overlap (i.e. only one occurrence of each type is allowed across all
// instances of iftype_akm_suites).
// @num_iftype_akm_suites: number of interface types for which supported akm
// suites are specified separately.
// @retry_short: Retry limit for short frames (dot11ShortRetryLimit)
// @retry_long: Retry limit for long frames (dot11LongRetryLimit)
// @frag_threshold: Fragmentation threshold (dot11FragmentationThreshold);
// -1 = fragmentation disabled, only odd values >= 256 used
// @rts_threshold: RTS threshold (dot11RTSThreshold); -1 = RTS/CTS disabled
// @_net: the network namespace this wiphy currently lives in
// @perm_addr: permanent MAC address of this device
// @addr_mask: If the device supports multiple MAC addresses by masking,
// set this to a mask with variable bits set to 1, e.g. if the last
// four bits are variable then set it to 00-00-00-00-00-0f. The actual
// variable bits shall be determined by the interfaces added, with
// interfaces not matching the mask being rejected to be brought up.
// @n_addresses: number of addresses in @addresses.
// @addresses: If the device has more than one address, set this pointer
// to a list of addresses (6 bytes each). The first one will be used
// by default for perm_addr. In this case, the mask should be set to
// all-zeroes. In this case it is assumed that the device can handle
// the same number of arbitrary MAC addresses.
// @registered: protects ->resume and ->suspend sysfs callbacks against
// unregister hardware
// @debugfsdir: debugfs directory used for this wiphy (ieee80211/<wiphyname>).
// It will be renamed automatically on wiphy renames
// @dev: (virtual) struct device for this wiphy. The item in
// /sys/class/ieee80211/ points to this. You need use set_wiphy_dev()
// (see below).
// @wext: wireless extension handlers
// @priv: driver private data (sized according to wiphy_new() parameter)
// @interface_modes: bitmask of interfaces types valid for this wiphy,
// must be set by driver
// @iface_combinations: Valid interface combinations array, should not
// list single interface types.
// @n_iface_combinations: number of entries in @iface_combinations array.
// @software_iftypes: bitmask of software interface types, these are not
// subject to any restrictions since they are purely managed in SW.
// @flags: wiphy flags, see &enum wiphy_flags
// @regulatory_flags: wiphy regulatory flags, see
// &enum ieee80211_regulatory_flags
// @features: features advertised to nl80211, see &enum nl80211_feature_flags.
// @ext_features: extended features advertised to nl80211, see
// &enum nl80211_ext_feature_index.
// @bss_priv_size: each BSS struct has private data allocated with it,
// this variable determines its size
// @max_scan_ssids: maximum number of SSIDs the device can scan for in
// any given scan
// @max_sched_scan_reqs: maximum number of scheduled scan requests that
// the device can run concurrently.
// @max_sched_scan_ssids: maximum number of SSIDs the device can scan
// for in any given scheduled scan
// @max_match_sets: maximum number of match sets the device can handle
// when performing a scheduled scan, 0 if filtering is not
// supported.
// @max_scan_ie_len: maximum length of user-controlled IEs device can
// add to probe request frames transmitted during a scan, must not
// include fixed IEs like supported rates
// @max_sched_scan_ie_len: same as max_scan_ie_len, but for scheduled
// scans
// @max_sched_scan_plans: maximum number of scan plans (scan interval and number
// of iterations) for scheduled scan supported by the device.
// @max_sched_scan_plan_interval: maximum interval (in seconds) for a
// single scan plan supported by the device.
// @max_sched_scan_plan_iterations: maximum number of iterations for a single
// scan plan supported by the device.
// @coverage_class: current coverage class
// @fw_version: firmware version for ethtool reporting
// @hw_version: hardware version for ethtool reporting
// @max_num_pmkids: maximum number of PMKIDs supported by device
// @privid: a pointer that drivers can use to identify if an arbitrary
// wiphy is theirs, e.g. in global notifiers
// @bands: information about bands/channels supported by this device
//
// @mgmt_stypes: bitmasks of frame subtypes that can be subscribed to or
// transmitted through nl80211, points to an array indexed by interface
// type
//
// @available_antennas_tx: bitmap of antennas which are available to be
// configured as TX antennas. Antenna configuration commands will be
// rejected unless this or @available_antennas_rx is set.
//
// @available_antennas_rx: bitmap of antennas which are available to be
// configured as RX antennas. Antenna configuration commands will be
// rejected unless this or @available_antennas_tx is set.
//
// @probe_resp_offload:
// Bitmap of supported protocols for probe response offloading.
// See &enum nl80211_probe_resp_offload_support_attr. Only valid
// when the wiphy flag @WIPHY_FLAG_AP_PROBE_RESP_OFFLOAD is set.
//
// @max_remain_on_channel_duration: Maximum time a remain-on-channel operation
// may request, if implemented.
//
// @wowlan: WoWLAN support information
// @wowlan_config: current WoWLAN configuration; this should usually not be
// used since access to it is necessarily racy, use the parameter passed
// to the suspend() operation instead.
//
// @ap_sme_capa: AP SME capabilities, flags from &enum nl80211_ap_sme_features.
// @ht_capa_mod_mask:  Specify what ht_cap values can be over-ridden.
// If null, then none can be over-ridden.
// @vht_capa_mod_mask:  Specify what VHT capabilities can be over-ridden.
// If null, then none can be over-ridden.
//
// @wdev_list: the list of associated (virtual) interfaces; this list must
// not be modified by the driver, but can be read with RTNL/RCU protection.
//
// @max_acl_mac_addrs: Maximum number of MAC addresses that the device
// supports for ACL.
//
// @extended_capabilities: extended capabilities supported by the driver,
// additional capabilities might be supported by userspace; these are
// the 802.11 extended capabilities ("Extended Capabilities element")
// and are in the same format as in the information element. See
// 802.11-2012 8.4.2.29 for the defined fields. These are the default
// extended capabilities to be used if the capabilities are not specified
// for a specific interface type in iftype_ext_capab.
// @extended_capabilities_mask: mask of the valid values
// @extended_capabilities_len: length of the extended capabilities
// @iftype_ext_capab: array of extended capabilities per interface type
// @num_iftype_ext_capab: number of interface types for which extended
// capabilities are specified separately.
// @coalesce: packet coalescing support information
//
// @vendor_commands: array of vendor commands supported by the hardware
// @n_vendor_commands: number of vendor commands
// @vendor_events: array of vendor events supported by the hardware
// @n_vendor_events: number of vendor events
//
// @max_ap_assoc_sta: maximum number of associated stations supported in AP mode
// (including P2P GO) or 0 to indicate no such limit is advertised. The
// driver is allowed to advertise a theoretical limit that it can reach in
// some cases, but may not always reach.
//
// @max_num_csa_counters: Number of supported csa_counters in beacons
// and probe responses.  This value should be set if the driver
// wishes to limit the number of csa counters. Default (0) means
// infinite.
// @bss_param_support: bitmask indicating which bss_parameters as defined in
// &struct bss_parameters the driver can actually handle in the
// .change_bss() callback. The bit positions are defined in &enum
// wiphy_bss_param_flags.
//
// @bss_select_support: bitmask indicating the BSS selection criteria supported
// by the driver in the .connect() callback. The bit position maps to the
// attribute indices defined in &enum nl80211_bss_select_attr.
//
// @nan_supported_bands: bands supported by the device in NAN mode, a
// bitmap of &enum nl80211_band values.  For instance, for
// NL80211_BAND_2GHZ, bit 0 would be set
// (i.e. BIT(NL80211_BAND_2GHZ)).
// @nan_capa: NAN capabilities
//
// @txq_limit: configuration of internal TX queue frame limit
// @txq_memory_limit: configuration internal TX queue memory limit
// @txq_quantum: configuration of internal TX queue scheduler quantum
//
// @tx_queue_len: allow setting transmit queue len for drivers not using
// wake_tx_queue
//
// @support_mbssid: can HW support association with nontransmitted AP
// @support_only_he_mbssid: don't parse MBSSID elements if it is not
// HE AP, in order to avoid compatibility issues.
// @support_mbssid must be set for this to have any effect.
//
// @pmsr_capa: peer measurement capabilities
//
// @tid_config_support: describes the per-TID config support that the
// device has
// @tid_config_support.vif: bitmap of attributes (configurations)
// supported by the driver for each vif
// @tid_config_support.peer: bitmap of attributes (configurations)
// supported by the driver for each peer
// @tid_config_support.max_retry: maximum supported retry count for
// long/short retry configuration
//
// @max_data_retry_count: maximum supported per TID retry count for
// configuration through the %NL80211_TID_CONFIG_ATTR_RETRY_SHORT and
// %NL80211_TID_CONFIG_ATTR_RETRY_LONG attributes
// @sar_capa: SAR control capabilities
// @rfkill: a pointer to the rfkill structure
//
// @mbssid_max_interfaces: maximum number of interfaces supported by the driver
// in a multiple BSSID set. This field must be set to a non-zero value
// by the driver to advertise MBSSID support.
// @ema_max_profile_periodicity: maximum profile periodicity supported by
// the driver. Setting this field to a non-zero value indicates that the
// driver supports enhanced multi-BSSID advertisements (EMA AP).
// @max_num_akm_suites: maximum number of AKM suites allowed for
// configuration through %NL80211_CMD_CONNECT, %NL80211_CMD_ASSOCIATE and
// %NL80211_CMD_START_AP. Set to NL80211_MAX_NR_AKM_SUITES if not set by
// driver. If set by driver minimum allowed value is
// NL80211_MAX_NR_AKM_SUITES in order to avoid compatibility issues with
// legacy userspace and maximum allowed value is
// CFG80211_MAX_NUM_AKM_SUITES.
//
// @hw_timestamp_max_peers: maximum number of peers that the driver supports
// enabling HW timestamping for concurrently. Setting this field to a
// non-zero value indicates that the driver supports HW timestamping.
// A value of %CFG80211_HW_TIMESTAMP_ALL_PEERS indicates the driver
// supports enabling HW timestamping for all peers (i.e. no need to
// specify a mac address).
//
// @radio_cfg: configuration of radios belonging to a muli-radio wiphy. This
// struct contains a list of all radio specific attributes and should be
// used only for multi-radio wiphy.
//
// @radio: radios belonging to this wiphy
// @n_radio: number of radios
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy {
    pub mtx: mutex,
// assign these fields before you register the wiphy
    pub perm_addr: [u8; ETH_ALEN],
    pub addr_mask: [u8; ETH_ALEN],
    pub addresses: *mut mac_address,
    pub mgmt_stypes: *const ieee80211_txrx_stypes,
    pub iface_combinations: *const ieee80211_iface_combination,
    pub n_iface_combinations: c_int,
    pub software_iftypes: u16,
    pub n_addresses: u16,
// Supported interface modes, OR together BIT(NL80211_IFTYPE_...)
    pub interface_modes: u16,
    pub max_acl_mac_addrs: u16,
    pub features: u32 flags, regulatory_flags,,
    pub 8)]: u8 ext_features[DIV_ROUND_UP(NUM_NL80211_EXT_FEATURES,,
    pub ap_sme_capa: u32,
    pub signal_type: cfg80211_signal_type,
    pub bss_priv_size: c_int,
    pub max_scan_ssids: u8,
    pub max_sched_scan_reqs: u8,
    pub max_sched_scan_ssids: u8,
    pub max_match_sets: u8,
    pub max_scan_ie_len: u16,
    pub max_sched_scan_ie_len: u16,
    pub max_sched_scan_plans: u32,
    pub max_sched_scan_plan_interval: u32,
    pub max_sched_scan_plan_iterations: u32,
    pub n_cipher_suites: c_int,
    pub cipher_suites: *const u32,
    pub n_akm_suites: c_int,
    pub akm_suites: *const u32,
    pub iftype_akm_suites: *const wiphy_iftype_akm_suites,
    pub num_iftype_akm_suites: c_uint,
    pub retry_short: u8,
    pub retry_long: u8,
    pub frag_threshold: u32,
    pub rts_threshold: u32,
    pub coverage_class: u8,
    pub fw_version: [c_char; ETHTOOL_FWVERS_LEN],
    pub hw_version: u32,

    pub wowlan: *const wiphy_wowlan_support,
    pub wowlan_config: *mut cfg80211_wowlan,

    pub max_remain_on_channel_duration: u16,
    pub max_num_pmkids: u8,
    pub available_antennas_tx: u32,
    pub available_antennas_rx: u32,
    pub probe_resp_offload: u32,
    pub extended_capabilities_mask: *const *const u8 extended_capabilities,,
    pub extended_capabilities_len: u8,
    pub iftype_ext_capab: *const wiphy_iftype_ext_capab,
    pub num_iftype_ext_capab: c_uint,
    pub privid: *const c_void,
    pub bands: [*mut ieee80211_supported_band; NUM_NL80211_BANDS],
    pub request): *mut regulatory_request,
    pub radio_cfg: *mut wiphy_radio_cfg,
// fields below are read-only, assigned by cfg80211
    pub regd: *const ieee80211_regdomain __rcu,
    pub dev: device,
    pub registered: bool,
    pub debugfsdir: *mut dentry,
    pub ht_capa_mod_mask: *const ieee80211_ht_cap,
    pub vht_capa_mod_mask: *const ieee80211_vht_cap,
    pub wdev_list: list_head,
    pub _net: possible_net_t,

    pub wext: *const iw_handler_def,

    pub coalesce: *const wiphy_coalesce_support,
    pub vendor_commands: *const wiphy_vendor_command,
    pub vendor_events: *const nl80211_vendor_cmd_info,
    pub n_vendor_events: int n_vendor_commands,,
    pub max_ap_assoc_sta: u16,
    pub max_num_csa_counters: u8,
    pub bss_param_support: u32,
    pub bss_select_support: u32,
    pub nan_supported_bands: u8,
    pub nan_capa: wiphy_nan_capa,
    pub txq_limit: u32,
    pub txq_memory_limit: u32,
    pub txq_quantum: u32,
    pub tx_queue_len: c_ulong,
    pub pmsr_capa: *const cfg80211_pmsr_capabilities,
    pub vif: u64 peer,,
    pub max_retry: u8,
    pub tid_config_support: },
    pub max_data_retry_count: u8,
    pub sar_capa: *const cfg80211_sar_capa,
    pub rfkill: *mut rfkill,
    pub mbssid_max_interfaces: u8,
    pub ema_max_profile_periodicity: u8,
    pub max_num_akm_suites: u16,
    pub hw_timestamp_max_peers: u16,
    pub n_radio: c_int,
    pub radio: *const wiphy_radio,
    pub __aligned(NETDEV_ALIGN): char priv[],
}

extern "C" {
    pub fn read_pnet(_arg: &wiphy->_net) -> return;
}
//
// wiphy_priv - return priv from wiphy
//
// @wiphy: the wiphy whose priv pointer to return
// Return: The priv of @wiphy.
//
// priv_to_wiphy - return the wiphy containing the priv
//
// @priv: a pointer previously returned by wiphy_priv
// Return: The wiphy of @priv.
//
extern "C" {
    pub fn container_of(_arg: priv, wiphy: struct, _arg: priv) -> return;
}
//
// set_wiphy_dev - set device pointer for wiphy
//
// @wiphy: The wiphy whose device to bind
// @dev: The device to parent it to
//
// wiphy_dev - get wiphy dev pointer
//
// @wiphy: The wiphy whose device struct to look up
// Return: The dev of @wiphy.
//
// wiphy_name - get wiphy name
//
// @wiphy: The wiphy whose name to return
// Return: The name of @wiphy.
//
extern "C" {
    pub fn dev_name(_arg: &wiphy->dev) -> return;
}
//
// wiphy_new_nm - create a new wiphy for use with cfg80211
//
// @ops: The configuration operations for this device
// @sizeof_priv: The size of the private area to allocate
// @requested_name: Request a particular name.
// NULL is valid value, and means use the default phy%d naming.
//
// Create a new wiphy and associate the given operations with it.
// @sizeof_priv bytes are allocated for private use.
//
// Return: A pointer to the new wiphy. This pointer must be
// assigned to each netdev's ieee80211_ptr for proper operation.
//
// wiphy_new - create a new wiphy for use with cfg80211
//
// @ops: The configuration operations for this device
// @sizeof_priv: The size of the private area to allocate
//
// Create a new wiphy and associate the given operations with it.
// @sizeof_priv bytes are allocated for private use.
//
// Return: A pointer to the new wiphy. This pointer must be
// assigned to each netdev's ieee80211_ptr for proper operation.
//
extern "C" {
    pub fn wiphy_new_nm(_arg: ops, _arg: sizeof_priv, _arg: NULL) -> return;
}
//
// wiphy_register - register a wiphy with cfg80211
//
// @wiphy: The wiphy to register.
//
// Return: A non-negative wiphy index or a negative error code.
//
extern "C" {
    pub fn wiphy_register(wiphy: *mut wiphy) -> c_int;
}
// this is a define for better error reporting (file/line)

//
// rcu_dereference_wiphy - rcu_dereference with debug checking
// @wiphy: the wiphy to check the locking on
// @p: The pointer to read, prior to dereferencing
//
// Do an rcu_dereference(p), but check caller either holds rcu_read_lock()
// or RTNL. Note: Please prefer wiphy_dereference() or rcu_dereference().
//

//
// wiphy_dereference - fetch RCU pointer when updates are prevented by wiphy mtx
// @wiphy: the wiphy to check the locking on
// @p: The pointer to read, prior to dereferencing
//
// Return: the value of the specified RCU-protected pointer, but omit the
// READ_ONCE(), because caller holds the wiphy mutex used for updates.
//

//
// get_wiphy_regdom - get custom regdomain for the given wiphy
// @wiphy: the wiphy to get the regdomain from
//
// Context: Requires any of RTNL, wiphy mutex or RCU protection.
//
// Return: pointer to the regulatory domain associated with the wiphy
//
// wiphy_unregister - deregister a wiphy from cfg80211
//
// @wiphy: The wiphy to unregister.
//
// After this call, no more requests can be made with this priv
// pointer, but the call may sleep to wait for an outstanding
// request that is being handled.
//
extern "C" {
    pub fn wiphy_unregister(wiphy: *mut wiphy);
}
//
// wiphy_free - free wiphy
//
// @wiphy: The wiphy to free
//
extern "C" {
    pub fn wiphy_free(wiphy: *mut wiphy);
}
// internal structs
//
// wiphy_lock - lock the wiphy
// @wiphy: the wiphy to lock
//
// This is needed around registering and unregistering netdevs that
// aren't created through cfg80211 calls, since that requires locking
// in cfg80211 when the notifiers is called, but that cannot
// differentiate which way it's called.
//
// It can also be used by drivers for their own purposes.
//
// When cfg80211 ops are called, the wiphy is already locked.
//
// Note that this makes sure that no workers that have been queued
// with wiphy_queue_work() are running.
//
// wiphy_unlock - unlock the wiphy again
// @wiphy: the wiphy to unlock
//
extern "C" {
    pub fn void(: *mut *mut wiphy_work_func_t)(struct wiphy, : *mut wiphy_work) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_work {
    pub entry: list_head,
    pub func: wiphy_work_func_t,
}

//
// wiphy_work_queue - queue work for the wiphy
// @wiphy: the wiphy to queue for
// @work: the work item
//
// This is useful for work that must be done asynchronously, and work
// queued here has the special property that the wiphy mutex will be
// held as if wiphy_lock() was called, and that it cannot be running
// after wiphy_lock() was called. Therefore, wiphy_cancel_work() can
// use just cancel_work() instead of cancel_work_sync(), it requires
// being in a section protected by wiphy_lock().
//
extern "C" {
    pub fn wiphy_work_queue(wiphy: *mut wiphy, work: *mut wiphy_work);
}
//
// wiphy_work_cancel - cancel previously queued work
// @wiphy: the wiphy, for debug purposes
// @work: the work to cancel
//
// Cancel the work *without* waiting for it, this assumes being
// called under the wiphy mutex acquired by wiphy_lock().
//
extern "C" {
    pub fn wiphy_work_cancel(wiphy: *mut wiphy, work: *mut wiphy_work);
}
//
// wiphy_work_flush - flush previously queued work
// @wiphy: the wiphy, for debug purposes
// @work: the work to flush, this can be %NULL to flush all work
//
// Flush the work (i.e. run it if pending). This must be called
// under the wiphy mutex acquired by wiphy_lock().
//
extern "C" {
    pub fn wiphy_work_flush(wiphy: *mut wiphy, work: *mut wiphy_work);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_delayed_work {
    pub work: wiphy_work,
    pub wiphy: *mut wiphy,
    pub timer: timer_list,
}

extern "C" {
    pub fn wiphy_delayed_work_timer(t: *mut timer_list);
}
//
// wiphy_delayed_work_queue - queue delayed work for the wiphy
// @wiphy: the wiphy to queue for
// @dwork: the delayable worker
// @delay: number of jiffies to wait before queueing
//
// This is useful for work that must be done asynchronously, and work
// queued here has the special property that the wiphy mutex will be
// held as if wiphy_lock() was called, and that it cannot be running
// after wiphy_lock() was called. Therefore, wiphy_cancel_work() can
// use just cancel_work() instead of cancel_work_sync(), it requires
// being in a section protected by wiphy_lock().
//
// Note that these are scheduled with a timer where the accuracy
// becomes less the longer in the future the scheduled timer is. Use
// wiphy_hrtimer_work_queue() if the timer must be not be late by more
// than approximately 10 percent.
//
// wiphy_delayed_work_cancel - cancel previously queued delayed work
// @wiphy: the wiphy, for debug purposes
// @dwork: the delayed work to cancel
//
// Cancel the work *without* waiting for it, this assumes being
// called under the wiphy mutex acquired by wiphy_lock().
//
// wiphy_delayed_work_flush - flush previously queued delayed work
// @wiphy: the wiphy, for debug purposes
// @dwork: the delayed work to flush
//
// Flush the work (i.e. run it if pending). This must be called
// under the wiphy mutex acquired by wiphy_lock().
//
// wiphy_delayed_work_pending - Find out whether a wiphy delayable
// work item is currently pending.
//
// @wiphy: the wiphy, for debug purposes
// @dwork: the delayed work in question
//
// Return: true if timer is pending, false otherwise
//
// How wiphy_delayed_work_queue() works is by setting a timer which
// when it expires calls wiphy_work_queue() to queue the wiphy work.
// Because wiphy_delayed_work_queue() uses mod_timer(), if it is
// called twice and the second call happens before the first call
// deadline, the work will rescheduled for the second deadline and
// won't run before that.
//
// wiphy_delayed_work_pending() can be used to detect if calling
// wiphy_work_delayed_work_queue() would start a new work schedule
// or delayed a previous one. As seen below it cannot be used to
// detect precisely if the work has finished to execute nor if it
// is currently executing.
//
// CPU0                                CPU1
// wiphy_delayed_work_queue(wk)
// mod_timer(wk->timer)
// wiphy_delayed_work_pending(wk) -> true
//
// [...]
// expire_timers(wk->timer)
// detach_timer(wk->timer)
// wiphy_delayed_work_pending(wk) -> false
// wk->timer->function()                          |
// wiphy_work_queue(wk)                          | delayed work pending
// list_add_tail()                              | returns false but
// queue_work(cfg80211_wiphy_work)              | wk->func() has not
// | been run yet
// [...]                                           |
// cfg80211_wiphy_work()                          |
// wk->func()                                    V
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiphy_hrtimer_work {
    pub work: wiphy_work,
    pub wiphy: *mut wiphy,
    pub timer: hrtimer,
}

extern "C" {
    pub fn wiphy_hrtimer_work_timer(t: *mut hrtimer) -> hrtimer_restart;
}
//
// wiphy_hrtimer_work_queue - queue hrtimer work for the wiphy
// @wiphy: the wiphy to queue for
// @hrwork: the high resolution timer worker
// @delay: the delay given as a ktime_t
//
// Please refer to wiphy_delayed_work_queue(). The difference is that
// the hrtimer work uses a high resolution timer for scheduling. This
// may be needed if timeouts might be scheduled further in the future
// and the accuracy of the normal timer is not sufficient.
//
// Expect a delay of a few milliseconds as the timer is scheduled
// with some slack and some more time may pass between queueing the
// work and its start.
//
// wiphy_hrtimer_work_cancel - cancel previously queued hrtimer work
// @wiphy: the wiphy, for debug purposes
// @hrtimer: the hrtimer work to cancel
//
// Cancel the work *without* waiting for it, this assumes being
// called under the wiphy mutex acquired by wiphy_lock().
//
// wiphy_hrtimer_work_flush - flush previously queued hrtimer work
// @wiphy: the wiphy, for debug purposes
// @hrwork: the hrtimer work to flush
//
// Flush the work (i.e. run it if pending). This must be called
// under the wiphy mutex acquired by wiphy_lock().
//
// wiphy_hrtimer_work_pending - Find out whether a wiphy hrtimer
// work item is currently pending.
//
// @wiphy: the wiphy, for debug purposes
// @hrwork: the hrtimer work in question
//
// Return: true if timer is pending, false otherwise
//
// Please refer to the wiphy_delayed_work_pending() documentation as
// this is the equivalent function for hrtimer based delayed work
// items.
//
// enum ieee80211_ap_reg_power - regulatory power for an Access Point
//
// @IEEE80211_REG_UNSET_AP: Access Point has no regulatory power mode
// @IEEE80211_REG_LPI_AP: Indoor Access Point
// @IEEE80211_REG_SP_AP: Standard power Access Point
// @IEEE80211_REG_VLP_AP: Very low power Access Point
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_ap_reg_power {
    IEEE80211_REG_UNSET_AP,
    IEEE80211_REG_LPI_AP,
    IEEE80211_REG_SP_AP,
    IEEE80211_REG_VLP_AP,
}

//
// struct wireless_dev - wireless device state
//
// For netdevs, this structure must be allocated by the driver
// that uses the ieee80211_ptr field in struct net_device (this
// is intentional so it can be allocated along with the netdev.)
// It need not be registered then as netdev registration will
// be intercepted by cfg80211 to see the new wireless device,
// however, drivers must lock the wiphy before registering or
// unregistering netdevs if they pre-create any netdevs (in ops
// called from cfg80211, the wiphy is already locked.)
//
// For non-netdev uses, it must also be allocated by the driver
// in response to the cfg80211 callbacks that require it, as
// there's no netdev registration in that case it may not be
// allocated outside of callback operations that return it.
//
// @wiphy: pointer to hardware description
// @iftype: interface type
// @registered: is this wdev already registered with cfg80211
// @registering: indicates we're doing registration under wiphy lock
// for the notifier
// @list: (private) Used to collect the interfaces
// @netdev: (private) Used to reference back to the netdev, may be %NULL
// @identifier: (private) Identifier used in nl80211 to identify this
// wireless device if it has no netdev
// @u: union containing data specific to @iftype
// @connected: indicates if connected or not (STA mode)
// @wext: (private) Used by the internal wireless extensions compat code
// @wext.ibss: (private) IBSS data part of wext handling
// @wext.connect: (private) connection handling data
// @wext.keys: (private) (WEP) key data
// @wext.ie: (private) extra elements for association
// @wext.ie_len: (private) length of extra elements
// @wext.bssid: (private) selected network BSSID
// @wext.ssid: (private) selected network SSID
// @wext.default_key: (private) selected default key index
// @wext.default_mgmt_key: (private) selected default management key index
// @wext.prev_bssid: (private) previous BSSID for reassociation
// @wext.prev_bssid_valid: (private) previous BSSID validity
// @use_4addr: indicates 4addr mode is used on this interface, must be
// set by driver (if supported) on add_interface BEFORE registering the
// netdev and may otherwise be used by driver read-only, will be update
// by cfg80211 on change_interface
// @mgmt_registrations: list of registrations for management frames
// @mgmt_registrations_need_update: mgmt registrations were updated,
// need to propagate the update to the driver
// @address: The address for this device, valid only if @netdev is %NULL
// @is_running: true if this is a non-netdev device that has been started, e.g.
// the P2P Device.
// @ps: powersave mode is enabled
// @ps_timeout: dynamic powersave timeout
// @unexpected_nlportid: (private) netlink port ID of application
// registered for unexpected frames (AP mode or NAN_DATA mode)
// @conn: (private) cfg80211 software SME connection state machine data
// @connect_keys: (private) keys to set after connection is established
// @conn_bss_type: connecting/connected BSS type
// @conn_owner_nlportid: (private) connection owner socket port ID
// @disconnect_wk: (private) auto-disconnect work
// @disconnect_bssid: (private) the BSSID to use for auto-disconnect
// @event_list: (private) list for internal event processing
// @event_lock: (private) lock for event list
// @owner_nlportid: (private) owner socket port ID
// @nl_owner_dead: (private) owner socket went away
// @cqm_rssi_work: (private) CQM RSSI reporting work
// @cqm_config: (private) nl80211 RSSI monitor state
// @pmsr_list: (private) peer measurement requests
// @pmsr_lock: (private) peer measurements requests/results lock
// @pmsr_free_wk: (private) peer measurements cleanup work
// @unprot_beacon_reported: (private) timestamp of last
// unprotected beacon report
// @links: array of %IEEE80211_MLD_MAX_NUM_LINKS elements containing @addr
// @ap and @client for each link
// @links.cac_started: true if DFS channel availability check has been
// started
// @links.cac_start_time: timestamp (jiffies) when the dfs state was
// entered.
// @links.cac_time_ms: CAC time in ms
// @valid_links: bitmap describing what elements of @links are valid
// @radio_mask: Bitmask of radios that this interface is allowed to operate on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wireless_dev {
    pub wiphy: *mut wiphy,
    pub iftype: nl80211_iftype,
// the remainder of this struct should be private to cfg80211
    pub list: list_head,
    pub netdev: *mut net_device,
    pub identifier: u32,
    pub mgmt_registrations: list_head,
    pub mgmt_registrations_need_update:1: u8,
    pub registering: bool use_4addr, is_running, registered,,
    pub __aligned(sizeof(u16)): u8 address[ETH_ALEN],
// currently used for IBSS and SME - might be rearranged later
    pub conn: *mut cfg80211_conn,
    pub connect_keys: *mut cfg80211_cached_keys,
    pub conn_bss_type: ieee80211_bss_type,
    pub conn_owner_nlportid: u32,
    pub disconnect_wk: wiphy_work,
    pub disconnect_bssid: [u8; ETH_ALEN],
    pub event_list: list_head,
    pub event_lock: spinlock_t,
    pub connected:1: u8,
    pub ps: bool,
    pub ps_timeout: c_int,
    pub unexpected_nlportid: u32,
    pub owner_nlportid: u32,
    pub nl_owner_dead: bool,

// wext data
    pub ibss: cfg80211_ibss_params,
    pub connect: cfg80211_connect_params,
    pub keys: *mut cfg80211_cached_keys,
    pub ie: *const u8,
    pub ie_len: usize,
    pub bssid: [u8; ETH_ALEN],
    pub prev_bssid: [u8; ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub default_mgmt_key: s8 default_key,,
    pub prev_bssid_valid: bool,
    pub wext: },

    pub cqm_rssi_work: wiphy_work,
    pub cqm_config: *mut cfg80211_cqm_config __rcu,
    pub pmsr_list: list_head,
    pub pmsr_lock: spinlock_t,
    pub pmsr_free_wk: wiphy_work,
    pub unprot_beacon_reported: c_ulong,
    pub __aligned(2): u8 connected_addr[ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
    pub client: },
    pub beacon_interval: c_int,
    pub preset_chandef: cfg80211_chan_def,
    pub chandef: cfg80211_chan_def,
    pub id: [u8; IEEE80211_MAX_MESH_ID_LEN],
    pub id_up_len: u8 id_len,,
    pub mesh: },
    pub preset_chandef: cfg80211_chan_def,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
    pub ap: },
    pub current_bss: *mut cfg80211_internal_bss,
    pub chandef: cfg80211_chan_def,
    pub beacon_interval: c_int,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
    pub ibss: },
    pub chandef: cfg80211_chan_def,
    pub ocb: },
    pub __aligned(2): u8 cluster_id[ETH_ALEN],
    pub n_channels: u8,
    pub chandefs: *mut cfg80211_chan_def,
    pub sched_update_pending: bool,
    pub nan: },
    pub u: },
    pub __aligned(2): u8 addr[ETH_ALEN],
    pub beacon_interval: c_uint,
    pub chandef: cfg80211_chan_def,
    pub ap: },
    pub current_bss: *mut cfg80211_internal_bss,
    pub client: },
}

extern "C" {
    pub fn netif_running(_arg: wdev->netdev) -> return;
}
//
// wdev_priv - return wiphy priv from wireless_dev
//
// @wdev: The wireless device whose wiphy's priv pointer to return
// Return: The wiphy priv of @wdev.
//
extern "C" {
    pub fn wiphy_priv(_arg: wdev->wiphy) -> return;
}
//
// wdev_chandef - return chandef pointer from wireless_dev
// @wdev: the wdev
// @link_id: the link ID for MLO
//
// Return: The chandef depending on the mode, or %NULL.
//

//
// DOC: Utility functions
//
// cfg80211 offers a number of utility functions that can be useful.
//
// ieee80211_channel_equal - compare two struct ieee80211_channel
//
// @a: 1st struct ieee80211_channel
// @b: 2nd struct ieee80211_channel
// Return: true if center frequency of @a == @b
//
// ieee80211_channel_to_khz - convert ieee80211_channel to frequency in KHz
// @chan: struct ieee80211_channel to convert
// Return: The corresponding frequency (in KHz)
//
// ieee80211_channel_to_freq_khz - convert channel number to frequency
// @chan: channel number
// @band: band, necessary due to channel number overlap
// Return: The corresponding frequency (in KHz), or 0 if the conversion failed.
//
extern "C" {
    pub fn ieee80211_channel_to_freq_khz(chan: c_int, band: nl80211_band) -> u32;
}
//
// ieee80211_channel_to_frequency - convert channel number to frequency
// @chan: channel number
// @band: band, necessary due to channel number overlap
// Return: The corresponding frequency (in MHz), or 0 if the conversion failed.
//
extern "C" {
    pub fn KHZ_TO_MHZ(_arg: ieee80211_channel_to_freq_khz(chan, _arg: band)) -> return;
}
//
// ieee80211_freq_khz_to_channel - convert frequency to channel number
// @freq: center frequency in KHz
// Return: The corresponding channel, or 0 if the conversion failed.
//
extern "C" {
    pub fn ieee80211_freq_khz_to_channel(freq: u32) -> c_int;
}
//
// ieee80211_frequency_to_channel - convert frequency to channel number
// @freq: center frequency in MHz
// Return: The corresponding channel, or 0 if the conversion failed.
//
extern "C" {
    pub fn ieee80211_freq_khz_to_channel(_arg: MHZ_TO_KHZ(freq)) -> return;
}
//
// ieee80211_get_channel_khz - get channel struct from wiphy for specified
// frequency
// @wiphy: the struct wiphy to get the channel for
// @freq: the center frequency (in KHz) of the channel
// Return: The channel struct from @wiphy at @freq.
//
// ieee80211_get_channel - get channel struct from wiphy for specified frequency
//
// @wiphy: the struct wiphy to get the channel for
// @freq: the center frequency (in MHz) of the channel
// Return: The channel struct from @wiphy at @freq.
//
extern "C" {
    pub fn ieee80211_get_channel_khz(_arg: wiphy, _arg: MHZ_TO_KHZ(freq)) -> return;
}
//
// cfg80211_channel_is_psc - Check if the channel is a 6 GHz PSC
// @chan: control channel to check
//
// The Preferred Scanning Channels (PSC) are defined in
// Draft IEEE P802.11ax/D5.0, 26.17.2.3.3
//
// Return: %true if channel is a PSC, %false otherwise
//
// ieee80211_radio_freq_range_valid - Check if the radio supports the
// specified frequency range
//
// @radio: wiphy radio
// @freq: the frequency (in KHz) to be queried
// @width: the bandwidth (in KHz) to be queried
//
// Return: whether or not the given frequency range is valid for the given radio
//
// cfg80211_radio_chandef_valid - Check if the radio supports the chandef
//
// @radio: wiphy radio
// @chandef: chandef for current channel
//
// Return: whether or not the given chandef is valid for the given radio
//
// cfg80211_wdev_channel_allowed - Check if the wdev may use the channel
//
// @wdev: the wireless device
// @chan: channel to check
//
// Return: whether or not the wdev may use the channel
//
// ieee80211_get_response_rate - get basic rate for a given rate
//
// @sband: the band to look for rates in
// @basic_rates: bitmap of basic rates
// @bitrate: the bitrate for which to find the basic rate
//
// Return: The basic rate corresponding to a given bitrate, that
// is the next lower bitrate contained in the basic rate map,
// which is, for this function, given as a bitmap of indices of
// rates in the band's bitrate table.
//
// ieee80211_mandatory_rates - get mandatory rates for a given band
// @sband: the band to look for rates in
//
// Return: a bitmap of the mandatory rates for the given band, bits
// are set according to the rate position in the bitrates array.
//
extern "C" {
    pub fn ieee80211_mandatory_rates(sband: *mut ieee80211_supported_band) -> u32;
}
//
// Radiotap parsing functions -- for controlled injection support
//
// Implemented in net/wireless/radiotap.c
// Documentation in Documentation/networking/radiotap-headers.rst
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radiotap_align_size {
    pub size:4: uint8_t align:4,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_radiotap_namespace {
    pub align_size: *const radiotap_align_size,
    pub n_bits: c_int,
    pub oui: u32,
    pub subns: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_radiotap_vendor_namespaces {
    pub ns: *const ieee80211_radiotap_namespace,
    pub n_ns: c_int,
}

//
// struct ieee80211_radiotap_iterator - tracks walk thru present radiotap args
// @this_arg_index: index of current arg, valid after each successful call
// to ieee80211_radiotap_iterator_next()
// @this_arg: pointer to current radiotap arg; it is valid after each
// call to ieee80211_radiotap_iterator_next() but also after
// ieee80211_radiotap_iterator_init() where it will point to
// the beginning of the actual data portion
// @this_arg_size: length of the current arg, for convenience
// @current_namespace: pointer to the current namespace definition
// (or internally %NULL if the current namespace is unknown)
// @is_radiotap_ns: indicates whether the current namespace is the default
// radiotap namespace or not
//
// @_rtheader: pointer to the radiotap header we are walking through
// @_max_length: length of radiotap header in cpu byte ordering
// @_arg_index: next argument index
// @_arg: next argument pointer
// @_next_bitmap: internal pointer to next present u32
// @_bitmap_shifter: internal shifter for curr u32 bitmap, b0 set == arg present
// @_vns: vendor namespace definitions
// @_next_ns_data: beginning of the next namespace's data
// @_reset_on_ext: internal; reset the arg index to 0 when going to the
// next bitmap word
//
// Describes the radiotap parser state. Fields prefixed with an underscore
// must not be used by users of the parser, only by the parser internally.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_radiotap_iterator {
    pub _rtheader: *mut ieee80211_radiotap_header,
    pub _vns: *const ieee80211_radiotap_vendor_namespaces,
    pub current_namespace: *const ieee80211_radiotap_namespace,
    pub _next_ns_data: *mut *mut unsigned char _arg,,
    pub _next_bitmap: *mut __le32,
    pub this_arg: *mut c_uchar,
    pub this_arg_index: c_int,
    pub this_arg_size: c_int,
    pub is_radiotap_ns: c_int,
    pub _max_length: c_int,
    pub _arg_index: c_int,
    pub _bitmap_shifter: u32,
    pub _reset_on_ext: c_int,
}

//
// ieee80211_get_hdrlen_from_skb - get header length from data
//
// @skb: the frame
//
// Given an skb with a raw 802.11 header at the data pointer this function
// returns the 802.11 header length.
//
// Return: The 802.11 header length in bytes (not including encryption
// headers). Or 0 if the data in the sk_buff is too short to contain a valid
// 802.11 header.
//
extern "C" {
    pub fn ieee80211_get_hdrlen_from_skb(skb: *const sk_buff) -> c_uint;
}
//
// ieee80211_hdrlen - get header length in bytes from frame control
// @fc: frame control field in little-endian format
// Return: The header length in bytes.
//
extern "C" {
    pub fn ieee80211_hdrlen(fc: __le16) -> unsigned int __attribute_const__;
}
//
// ieee80211_get_mesh_hdrlen - get mesh extension header length
// @meshhdr: the mesh extension header, only the flags field
// (first byte) will be accessed
// Return: The length of the extension header, which is always at
// least 6 bytes and at most 18 if address 5 and 6 are present.
//
extern "C" {
    pub fn ieee80211_get_mesh_hdrlen(meshhdr: *mut ieee80211s_hdr) -> c_uint;
}
//
// DOC: Data path helpers
//
// In addition to generic utilities, cfg80211 also offers
// functions that help implement the data path for devices
// that do not do the 802.11/802.3 conversion on the device.
//
// ieee80211_data_to_8023_exthdr - convert an 802.11 data frame to 802.3
// @skb: the 802.11 data frame
// @ehdr: pointer to a &struct ethhdr that will get the header, instead
// of it being pushed into the SKB
// @addr: the device MAC address
// @iftype: the virtual interface type
// @data_offset: offset of payload after the 802.11 header
// @is_amsdu: true if the 802.11 header is A-MSDU
// Return: 0 on success. Non-zero on error.
//
// ieee80211_data_to_8023 - convert an 802.11 data frame to 802.3
// @skb: the 802.11 data frame
// @addr: the device MAC address
// @iftype: the virtual interface type
// Return: 0 on success. Non-zero on error.
//
extern "C" {
    pub fn ieee80211_data_to_8023_exthdr(_arg: skb, _arg: NULL, _arg: addr, _arg: iftype, _arg: 0, _arg: false) -> return;
}
//
// ieee80211_is_valid_amsdu - check if subframe lengths of an A-MSDU are valid
//
// This is used to detect non-standard A-MSDU frames, e.g. the ones generated
// by ath10k and ath11k, where the subframe length includes the length of the
// mesh control field.
//
// @skb: The input A-MSDU frame without any headers.
// @mesh_hdr: the type of mesh header to test
// 0: non-mesh A-MSDU length field
// 1: big-endian mesh A-MSDU length field
// 2: little-endian mesh A-MSDU length field
// Returns: true if subframe header lengths are valid for the @mesh_hdr mode
//
extern "C" {
    pub fn ieee80211_is_valid_amsdu(skb: *mut sk_buff, mesh_hdr: u8) -> bool;
}
//
// ieee80211_amsdu_to_8023s - decode an IEEE 802.11n A-MSDU frame
//
// Decode an IEEE 802.11 A-MSDU and convert it to a list of 802.3 frames.
// The @list will be empty if the decode fails. The @skb must be fully
// header-less before being passed in here; it is freed in this function.
//
// @skb: The input A-MSDU frame without any headers.
// @list: The output list of 802.3 frames. It must be allocated and
// initialized by the caller.
// @addr: The device MAC address.
// @iftype: The device interface type.
// @extra_headroom: The hardware extra headroom for SKBs in the @list.
// @check_da: DA to check in the inner ethernet header, or NULL
// @check_sa: SA to check in the inner ethernet header, or NULL
// @mesh_control: see mesh_hdr in ieee80211_is_valid_amsdu
//
// ieee80211_get_8023_tunnel_proto - get RFC1042 or bridge tunnel encap protocol
//
// Check for RFC1042 or bridge tunnel header and fetch the encapsulated
// protocol.
//
// @hdr: pointer to the MSDU payload
// @proto: destination pointer to store the protocol
// Return: true if encapsulation was found
//
extern "C" {
    pub fn ieee80211_get_8023_tunnel_proto(hdr: *const c_void, proto: *mut __be16) -> bool;
}
//
// ieee80211_strip_8023_mesh_hdr - strip mesh header from converted 802.3 frames
//
// Strip the mesh header, which was left in by ieee80211_data_to_8023 as part
// of the MSDU data. Also move any source/destination addresses from the mesh
// header to the ethernet header (if present).
//
// @skb: The 802.3 frame with embedded mesh header
//
// Return: 0 on success. Non-zero on error.
//
extern "C" {
    pub fn ieee80211_strip_8023_mesh_hdr(skb: *mut sk_buff) -> c_int;
}
//
// cfg80211_classify8021d - determine the 802.1p/1d tag for a data frame
// @skb: the data frame
// @qos_map: Interworking QoS mapping or %NULL if not in use
// Return: The 802.1p/1d tag.
//
// cfg80211_find_elem_match - match information element and byte array in data
//
// @eid: element ID
// @ies: data consisting of IEs
// @len: length of data
// @match: byte array to match
// @match_len: number of bytes in the match array
// @match_offset: offset in the IE data where the byte array should match.
// Note the difference to cfg80211_find_ie_match() which considers
// the offset to start from the element ID byte, but here we take
// the data portion instead.
//
// Return: %NULL if the element ID could not be found or if
// the element is invalid (claims to be longer than the given
// data) or if the byte array doesn't match; otherwise return the
// requested element struct.
//
// Note: There are no checks on the element length other than
// having to fit into the given data and being large enough for the
// byte array to match.
//
// cfg80211_find_ie_match - match information element and byte array in data
//
// @eid: element ID
// @ies: data consisting of IEs
// @len: length of data
// @match: byte array to match
// @match_len: number of bytes in the match array
// @match_offset: offset in the IE where the byte array should match.
// If match_len is zero, this must also be set to zero.
// Otherwise this must be set to 2 or more, because the first
// byte is the element id, which is already compared to eid, and
// the second byte is the IE length.
//
// Return: %NULL if the element ID could not be found or if
// the element is invalid (claims to be longer than the given
// data) or if the byte array doesn't match, or a pointer to the first
// byte of the requested element, that is the byte containing the
// element ID.
//
// Note: There are no checks on the element length other than
// having to fit into the given data and being large enough for the
// byte array to match.
//
// match_offset can't be smaller than 2, unless match_len is
// zero, in which case match_offset must be zero as well.
//
// cfg80211_find_elem - find information element in data
//
// @eid: element ID
// @ies: data consisting of IEs
// @len: length of data
//
// Return: %NULL if the element ID could not be found or if
// the element is invalid (claims to be longer than the given
// data) or if the byte array doesn't match; otherwise return the
// requested element struct.
//
// Note: There are no checks on the element length other than
// having to fit into the given data.
//
extern "C" {
    pub fn cfg80211_find_elem_match(_arg: eid, _arg: ies, _arg: len, _arg: NULL, _arg: 0, _arg: 0) -> return;
}
//
// cfg80211_find_ie - find information element in data
//
// @eid: element ID
// @ies: data consisting of IEs
// @len: length of data
//
// Return: %NULL if the element ID could not be found or if
// the element is invalid (claims to be longer than the given
// data), or a pointer to the first byte of the requested
// element, that is the byte containing the element ID.
//
// Note: There are no checks on the element length other than
// having to fit into the given data.
//
extern "C" {
    pub fn cfg80211_find_ie_match(_arg: eid, _arg: ies, _arg: len, _arg: NULL, _arg: 0, _arg: 0) -> return;
}
//
// cfg80211_find_ext_elem - find information element with EID Extension in data
//
// @ext_eid: element ID Extension
// @ies: data consisting of IEs
// @len: length of data
//
// Return: %NULL if the extended element could not be found or if
// the element is invalid (claims to be longer than the given
// data) or if the byte array doesn't match; otherwise return the
// requested element struct.
//
// Note: There are no checks on the element length other than
// having to fit into the given data.
//
// cfg80211_find_ext_ie - find information element with EID Extension in data
//
// @ext_eid: element ID Extension
// @ies: data consisting of IEs
// @len: length of data
//
// Return: %NULL if the extended element ID could not be found or if
// the element is invalid (claims to be longer than the given
// data), or a pointer to the first byte of the requested
// element, that is the byte containing the element ID.
//
// Note: There are no checks on the element length other than
// having to fit into the given data.
//
// cfg80211_find_vendor_elem - find vendor specific information element in data
//
// @oui: vendor OUI
// @oui_type: vendor-specific OUI type (must be < 0xff), negative means any
// @ies: data consisting of IEs
// @len: length of data
//
// Return: %NULL if the vendor specific element ID could not be found or if the
// element is invalid (claims to be longer than the given data); otherwise
// return the element structure for the requested element.
//
// Note: There are no checks on the element length other than having to fit into
// the given data.
//
// cfg80211_find_vendor_ie - find vendor specific information element in data
//
// @oui: vendor OUI
// @oui_type: vendor-specific OUI type (must be < 0xff), negative means any
// @ies: data consisting of IEs
// @len: length of data
//
// Return: %NULL if the vendor specific element ID could not be found or if the
// element is invalid (claims to be longer than the given data), or a pointer to
// the first byte of the requested element, that is the byte containing the
// element ID.
//
// Note: There are no checks on the element length other than having to fit into
// the given data.
//
// enum cfg80211_rnr_iter_ret - reduced neighbor report iteration state
// @RNR_ITER_CONTINUE: continue iterating with the next entry
// @RNR_ITER_BREAK: break iteration and return success
// @RNR_ITER_ERROR: break iteration and return error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_rnr_iter_ret {
    RNR_ITER_CONTINUE,
    RNR_ITER_BREAK,
    RNR_ITER_ERROR,
}

//
// cfg80211_iter_rnr - iterate reduced neighbor report entries
// @elems: the frame elements to iterate RNR elements and then
// their entries in
// @elems_len: length of the elements
// @iter: iteration function, see also &enum cfg80211_rnr_iter_ret
// for the return value
// @iter_data: additional data passed to the iteration function
// Return: %true on success (after successfully iterating all entries
// or if the iteration function returned %RNR_ITER_BREAK),
// %false on error (iteration function returned %RNR_ITER_ERROR
// or elements were malformed.)
//
// cfg80211_defragment_element - Defrag the given element data into a buffer
//
// @elem: the element to defragment
// @ies: elements where @elem is contained
// @ieslen: length of @ies
// @data: buffer to store element data, or %NULL to just determine size
// @data_len: length of @data, or 0
// @frag_id: the element ID of fragments
//
// Return: length of @data, or -EINVAL on error
//
// Copy out all data from an element that may be fragmented into @data, while
// skipping all headers.
//
// The function uses memmove() internally. It is acceptable to defragment an
// element in-place.
//
// cfg80211_send_layer2_update - send layer 2 update frame
//
// @dev: network device
// @addr: STA MAC address
//
// Wireless drivers can use this function to update forwarding tables in bridge
// devices upon STA association.
//
extern "C" {
    pub fn cfg80211_send_layer2_update(dev: *mut net_device, addr: *const u8);
}
//
// DOC: Regulatory enforcement infrastructure
//
// TODO
//
// regulatory_hint - driver hint to the wireless core a regulatory domain
// @wiphy: the wireless device giving the hint (used only for reporting
// conflicts)
// @alpha2: the ISO/IEC 3166 alpha2 the driver claims its regulatory domain
// should be in. If @rd is set this should be NULL. Note that if you
// set this to NULL you should still set rd->alpha2 to some accepted
// alpha2.
//
// Wireless drivers can use this function to hint to the wireless core
// what it believes should be the current regulatory domain by
// giving it an ISO/IEC 3166 alpha2 country code it knows its regulatory
// domain should be in or by providing a completely build regulatory domain.
// If the driver provides an ISO/IEC 3166 alpha2 userspace will be queried
// for a regulatory domain structure for the respective country.
//
// The wiphy must have been registered to cfg80211 prior to this call.
// For cfg80211 drivers this means you must first use wiphy_register(),
// for mac80211 drivers you must first use ieee80211_register_hw().
//
// Drivers should check the return value, its possible you can get
// an -ENOMEM.
//
// Return: 0 on success. -ENOMEM.
//
extern "C" {
    pub fn regulatory_hint(wiphy: *mut wiphy, alpha2: *const c_char) -> c_int;
}
//
// regulatory_set_wiphy_regd - set regdom info for self managed drivers
// @wiphy: the wireless device we want to process the regulatory domain on
// @rd: the regulatory domain information to use for this wiphy
//
// Set the regulatory domain information for self-managed wiphys, only they
// may use this function. See %REGULATORY_WIPHY_SELF_MANAGED for more
// information.
//
// Return: 0 on success. -EINVAL, -EPERM
//
// regulatory_set_wiphy_regd_sync - set regdom for self-managed drivers
// @wiphy: the wireless device we want to process the regulatory domain on
// @rd: the regulatory domain information to use for this wiphy
//
// This functions requires the RTNL and the wiphy mutex to be held and
// applies the new regdomain synchronously to this wiphy. For more details
// see regulatory_set_wiphy_regd().
//
// Return: 0 on success. -EINVAL, -EPERM
//
// wiphy_apply_custom_regulatory - apply a custom driver regulatory domain
// @wiphy: the wireless device we want to process the regulatory domain on
// @regd: the custom regulatory domain to use for this wiphy
//
// Drivers can sometimes have custom regulatory domains which do not apply
// to a specific country. Drivers can use this to apply such custom regulatory
// domains. This routine must be called prior to wiphy registration. The
// custom regulatory domain will be trusted completely and as such previous
// default channel settings will be disregarded. If no rule is found for a
// channel on the regulatory domain the channel will be disabled.
// Drivers using this for a wiphy should also set the wiphy flag
// REGULATORY_CUSTOM_REG or cfg80211 will set it for the wiphy
// that called this helper.
//
// freq_reg_info - get regulatory information for the given frequency
// @wiphy: the wiphy for which we want to process this rule for
// @center_freq: Frequency in KHz for which we want regulatory information for
//
// Use this function to get the regulatory rule for a specific frequency on
// a given wireless device. If the device has a specific regulatory domain
// it wants to follow we respect that unless a country IE has been received
// and processed already.
//
// Return: A valid pointer, or, when an error occurs, for example if no rule
// can be found, the return value is encoded using ERR_PTR(). Use IS_ERR() to
// check and PTR_ERR() to obtain the numeric return value. The numeric return
// value will be -ERANGE if we determine the given center_freq does not even
// have a regulatory rule for a frequency range in the center_freq's band.
// See freq_in_rule_band() for our current definition of a band -- this is
// purely subjective and right now it's 802.11 specific.
//
// reg_initiator_name - map regulatory request initiator enum to name
// @initiator: the regulatory request initiator
//
// You can use this to map the regulatory request initiator enum to a
// proper string representation.
//
// Return: pointer to string representation of the initiator
//
// regulatory_pre_cac_allowed - check if pre-CAC allowed in the current regdom
// @wiphy: wiphy for which pre-CAC capability is checked.
//
// Pre-CAC is allowed only in some regdomains (notable ETSI).
//
// Return: %true if allowed, %false otherwise
//
extern "C" {
    pub fn regulatory_pre_cac_allowed(wiphy: *mut wiphy) -> bool;
}
//
// DOC: Internal regulatory db functions
//
// reg_query_regdb_wmm -  Query internal regulatory db for wmm rule
// Regulatory self-managed driver can use it to proactively
//
// @alpha2: the ISO/IEC 3166 alpha2 wmm rule to be queried.
// @freq: the frequency (in MHz) to be queried.
// @rule: pointer to store the wmm rule from the regulatory db.
//
// Self-managed wireless drivers can use this function to  query
// the internal regulatory database to check whether the given
// ISO/IEC 3166 alpha2 country and freq have wmm rule limitations.
//
// Drivers should check the return value, its possible you can get
// an -ENODATA.
//
// Return: 0 on success. -ENODATA.
//
// callbacks for asynchronous cfg80211 methods, notification
// functions and BSS handling helpers
//
// cfg80211_scan_done - notify that scan finished
//
// @request: the corresponding scan request
// @info: information about the completed scan
//
// cfg80211_sched_scan_results - notify that new scan results are available
//
// @wiphy: the wiphy which got scheduled scan results
// @reqid: identifier for the related scheduled scan request
//
extern "C" {
    pub fn cfg80211_sched_scan_results(wiphy: *mut wiphy, reqid: u64);
}
//
// cfg80211_sched_scan_stopped - notify that the scheduled scan has stopped
//
// @wiphy: the wiphy on which the scheduled scan stopped
// @reqid: identifier for the related scheduled scan request
//
// The driver can call this function to inform cfg80211 that the
// scheduled scan had to be stopped, for whatever reason.  The driver
// is then called back via the sched_scan_stop operation when done.
//
extern "C" {
    pub fn cfg80211_sched_scan_stopped(wiphy: *mut wiphy, reqid: u64);
}
//
// cfg80211_sched_scan_stopped_locked - notify that the scheduled scan has stopped
//
// @wiphy: the wiphy on which the scheduled scan stopped
// @reqid: identifier for the related scheduled scan request
//
// The driver can call this function to inform cfg80211 that the
// scheduled scan had to be stopped, for whatever reason.  The driver
// is then called back via the sched_scan_stop operation when done.
// This function should be called with the wiphy mutex held.
//
extern "C" {
    pub fn cfg80211_sched_scan_stopped_locked(wiphy: *mut wiphy, reqid: u64);
}
//
// cfg80211_inform_bss_frame_data - inform cfg80211 of a received BSS frame
// @wiphy: the wiphy reporting the BSS
// @data: the BSS metadata
// @mgmt: the management frame (probe response or beacon)
// @len: length of the management frame
// @gfp: context flags
//
// This informs cfg80211 that BSS information was found and
// the BSS should be updated/added.
//
// Return: A referenced struct, must be released with cfg80211_put_bss()!
// Or %NULL on error.
//
extern "C" {
    pub fn cfg80211_inform_bss_frame_data(_arg: wiphy, _arg: &data, _arg: mgmt, _arg: len, _arg: gfp) -> return;
}
//
// cfg80211_gen_new_bssid - generate a nontransmitted BSSID for multi-BSSID
// @bssid: transmitter BSSID
// @max_bssid: max BSSID indicator, taken from Multiple BSSID element
// @mbssid_index: BSSID index, taken from Multiple BSSID index element
// @new_bssid: calculated nontransmitted BSSID
//
// cfg80211_is_element_inherited - returns if element ID should be inherited
// @element: element to check
// @non_inherit_element: non inheritance element
//
// Return: %true if should be inherited, %false otherwise
//
// cfg80211_merge_profile - merges a MBSSID profile if it is split between IEs
// @ie: ies
// @ielen: length of IEs
// @mbssid_elem: current MBSSID element
// @sub_elem: current MBSSID subelement (profile)
// @merged_ie: location of the merged profile
// @max_copy_len: max merged profile length
//
// Return: the number of bytes merged
//
// enum cfg80211_bss_frame_type - frame type that the BSS data came from
// @CFG80211_BSS_FTYPE_UNKNOWN: driver doesn't know whether the data is
// from a beacon or probe response
// @CFG80211_BSS_FTYPE_BEACON: data comes from a beacon
// @CFG80211_BSS_FTYPE_PRESP: data comes from a probe response
// @CFG80211_BSS_FTYPE_S1G_BEACON: data comes from an S1G beacon
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_bss_frame_type {
    CFG80211_BSS_FTYPE_UNKNOWN,
    CFG80211_BSS_FTYPE_BEACON,
    CFG80211_BSS_FTYPE_PRESP,
    CFG80211_BSS_FTYPE_S1G_BEACON,
}

//
// cfg80211_get_ies_channel_number - returns the channel number from ies
// @ie: IEs
// @ielen: length of IEs
// @band: enum nl80211_band of the channel
//
// Return: the channel number, or -1 if none could be determined.
//
// cfg80211_ssid_eq - compare two SSIDs
// @a: first SSID
// @b: second SSID
//
// Return: %true if SSIDs are equal, %false otherwise.
//
// cfg80211_inform_bss_data - inform cfg80211 of a new BSS
//
// @wiphy: the wiphy reporting the BSS
// @data: the BSS metadata
// @ftype: frame type (if known)
// @bssid: the BSSID of the BSS
// @tsf: the TSF sent by the peer in the beacon/probe response (or 0)
// @capability: the capability field sent by the peer
// @beacon_interval: the beacon interval announced by the peer
// @ie: additional IEs sent by the peer
// @ielen: length of the additional IEs
// @gfp: context flags
//
// This informs cfg80211 that BSS information was found and
// the BSS should be updated/added.
//
// Return: A referenced struct, must be released with cfg80211_put_bss()!
// Or %NULL on error.
//
// __cfg80211_get_bss - get a BSS reference
// @wiphy: the wiphy this BSS struct belongs to
// @channel: the channel to search on (or %NULL)
// @bssid: the desired BSSID (or %NULL)
// @ssid: the desired SSID (or %NULL)
// @ssid_len: length of the SSID (or 0)
// @bss_type: type of BSS, see &enum ieee80211_bss_type
// @privacy: privacy filter, see &enum ieee80211_privacy
// @use_for: indicates which use is intended
// @extack: (optional) extack that is filled with the reason when no
// usable entry was found; may be %NULL
//
// Return: Reference-counted BSS on success. %NULL on error.
//
// cfg80211_get_bss - get a BSS reference
// @wiphy: the wiphy this BSS struct belongs to
// @channel: the channel to search on (or %NULL)
// @bssid: the desired BSSID (or %NULL)
// @ssid: the desired SSID (or %NULL)
// @ssid_len: length of the SSID (or 0)
// @bss_type: type of BSS, see &enum ieee80211_bss_type
// @privacy: privacy filter, see &enum ieee80211_privacy
//
// This version implies regular usage, %NL80211_BSS_USE_FOR_NORMAL.
//
// Return: Reference-counted BSS on success. %NULL on error.
//
// cfg80211_ref_bss - reference BSS struct
// @wiphy: the wiphy this BSS struct belongs to
// @bss: the BSS struct to reference
//
// Increments the refcount of the given BSS struct.
//
extern "C" {
    pub fn cfg80211_ref_bss(wiphy: *mut wiphy, bss: *mut cfg80211_bss);
}
//
// cfg80211_put_bss - unref BSS struct
// @wiphy: the wiphy this BSS struct belongs to
// @bss: the BSS struct
//
// Decrements the refcount of the given BSS struct.
//
extern "C" {
    pub fn cfg80211_put_bss(wiphy: *mut wiphy, bss: *mut cfg80211_bss);
}
//
// cfg80211_unlink_bss - unlink BSS from internal data structures
// @wiphy: the wiphy
// @bss: the bss to remove
//
// This function removes the given BSS from the internal data structures
// thereby making it no longer show up in scan results etc. Use this
// function when you detect a BSS is gone. Normally BSSes will also time
// out, so it is not necessary to use this function at all.
//
extern "C" {
    pub fn cfg80211_unlink_bss(wiphy: *mut wiphy, bss: *mut cfg80211_bss);
}
//
// cfg80211_bss_iter - iterate all BSS entries
//
// This function iterates over the BSS entries associated with the given wiphy
// and calls the callback for the iterated BSS. The iterator function is not
// allowed to call functions that might modify the internal state of the BSS DB.
//
// @wiphy: the wiphy
// @chandef: if given, the iterator function will be called only if the channel
// of the currently iterated BSS is a subset of the given channel.
// @iter: the iterator function to call
// @iter_data: an argument to the iterator function
//
// cfg80211_rx_mlme_mgmt - notification of processed MLME management frame
// @dev: network device
// @buf: authentication frame (header + body)
// @len: length of the frame data
//
// This function is called whenever an authentication, disassociation or
// deauthentication frame has been received and processed in station mode.
// After being asked to authenticate via cfg80211_ops::auth() the driver must
// call either this function or cfg80211_auth_timeout().
// After being asked to associate via cfg80211_ops::assoc() the driver must
// call either this function or cfg80211_auth_timeout().
// While connected, the driver must calls this for received and processed
// disassociation and deauthentication frames. If the frame couldn't be used
// because it was unprotected, the driver must call the function
// cfg80211_rx_unprot_mlme_mgmt() instead.
//
// This function may sleep. The caller must hold the corresponding wdev's mutex.
//
extern "C" {
    pub fn cfg80211_rx_mlme_mgmt(dev: *mut net_device, buf: *const u8, len: usize);
}
//
// cfg80211_auth_timeout - notification of timed out authentication
// @dev: network device
// @addr: The MAC address of the device with which the authentication timed out
//
// This function may sleep. The caller must hold the corresponding wdev's
// mutex.
//
extern "C" {
    pub fn cfg80211_auth_timeout(dev: *mut net_device, addr: *const u8);
}
//
// struct cfg80211_rx_assoc_resp_data - association response data
// @buf: (Re)Association Response frame (header + body)
// @len: length of the frame data
// @uapsd_queues: bitmap of queues configured for uapsd. Same format
// as the AC bitmap in the QoS info field
// @req_ies: information elements from the (Re)Association Request frame
// @req_ies_len: length of req_ies data
// @assoc_encrypted: indicate if the (re)association exchange is encrypted.
// @ap_mld_addr: AP MLD address (in case of MLO)
// @links: per-link information indexed by link ID, use links[0] for
// non-MLO connections
// @links.bss: the BSS that association was requested with, ownership of the
// pointer moves to cfg80211 in the call to cfg80211_rx_assoc_resp()
// @links.status: Set this (along with a BSS pointer) for links that
// were rejected by the AP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_rx_assoc_resp_data {
    pub buf: *const u8,
    pub len: usize,
    pub req_ies: *const u8,
    pub req_ies_len: usize,
    pub uapsd_queues: c_int,
    pub assoc_encrypted: bool,
    pub ap_mld_addr: *const u8,
    pub __aligned(2): u8 addr[ETH_ALEN],
    pub bss: *mut cfg80211_bss,
    pub status: u16,
    pub links: [}; IEEE80211_MLD_MAX_NUM_LINKS],
}

//
// cfg80211_rx_assoc_resp - notification of processed association response
// @dev: network device
// @data: association response data, &struct cfg80211_rx_assoc_resp_data
//
// After being asked to associate via cfg80211_ops::assoc() the driver must
// call either this function or cfg80211_auth_timeout().
//
// This function may sleep. The caller must hold the corresponding wdev's mutex.
//
// struct cfg80211_assoc_failure - association failure data
// @ap_mld_addr: AP MLD address, or %NULL
// @bss: list of BSSes, must use entry 0 for non-MLO connections
// (@ap_mld_addr is %NULL)
// @timeout: indicates the association failed due to timeout, otherwise
// the association was abandoned for a reason reported through some
// other API (e.g. deauth RX)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_assoc_failure {
    pub ap_mld_addr: *const u8,
    pub bss: [*mut cfg80211_bss; IEEE80211_MLD_MAX_NUM_LINKS],
    pub timeout: bool,
}

//
// cfg80211_assoc_failure - notification of association failure
// @dev: network device
// @data: data describing the association failure
//
// This function may sleep. The caller must hold the corresponding wdev's mutex.
//
// cfg80211_tx_mlme_mgmt - notification of transmitted deauth/disassoc frame
// @dev: network device
// @buf: 802.11 frame (header + body)
// @len: length of the frame data
// @reconnect: immediate reconnect is desired (include the nl80211 attribute)
//
// This function is called whenever deauthentication has been processed in
// station mode. This includes both received deauthentication frames and
// locally generated ones. This function may sleep. The caller must hold the
// corresponding wdev's mutex.
//
// cfg80211_rx_unprot_mlme_mgmt - notification of unprotected mlme mgmt frame
// @dev: network device
// @buf: received management frame (header + body)
// @len: length of the frame data
//
// This function is called whenever a received deauthentication or dissassoc
// frame has been dropped in station mode because of MFP being used but the
// frame was not protected. This is also used to notify reception of a Beacon
// frame that was dropped because it did not include a valid MME MIC while
// beacon protection was enabled (BIGTK configured in station mode).
//
// This function may sleep.
//
// cfg80211_michael_mic_failure - notification of Michael MIC failure (TKIP)
// @dev: network device
// @addr: The source MAC address of the frame
// @key_type: The key type that the received frame used
// @key_id: Key identifier (0..3). Can be -1 if missing.
// @tsc: The TSC value of the frame that generated the MIC failure (6 octets)
// @gfp: allocation flags
//
// This function is called whenever the local MAC detects a MIC failure in a
// received frame. This matches with MLME-MICHAELMICFAILURE.indication()
// primitive.
//
// cfg80211_ibss_joined - notify cfg80211 that device joined an IBSS
//
// @dev: network device
// @bssid: the BSSID of the IBSS joined
// @channel: the channel of the IBSS joined
// @gfp: allocation flags
//
// This function notifies cfg80211 that the device joined an IBSS or
// switched to a different BSSID. Before this function can be called,
// either a beacon has to have been received from the IBSS, or one of
// the cfg80211_inform_bss{,_frame} functions must have been called
// with the locally generated beacon -- this guarantees that there is
// always a scan result for this IBSS. cfg80211 will handle the rest.
//
// cfg80211_notify_new_peer_candidate - notify cfg80211 of a new mesh peer
// candidate
//
// @dev: network device
// @macaddr: the MAC address of the new candidate
// @ie: information elements advertised by the peer candidate
// @ie_len: length of the information elements buffer
// @sig_dbm: signal level in dBm
// @gfp: allocation flags
//
// This function notifies cfg80211 that the mesh peer candidate has been
// detected, most likely via a beacon or, less likely, via a probe response.
// cfg80211 then sends a notification to userspace.
//
// DOC: RFkill integration
//
// RFkill integration in cfg80211 is almost invisible to drivers,
// as cfg80211 automatically registers an rfkill instance for each
// wireless device it knows about. Soft kill is also translated
// into disconnecting and turning all interfaces off. Drivers are
// expected to turn off the device when all interfaces are down.
//
// However, devices may have a hard RFkill line, in which case they
// also need to interact with the rfkill subsystem, via cfg80211.
// They can do this with a few helper functions documented here.
//
// wiphy_rfkill_set_hw_state_reason - notify cfg80211 about hw block state
// @wiphy: the wiphy
// @blocked: block status
// @reason: one of reasons in &enum rfkill_hard_block_reasons
//
// wiphy_rfkill_start_polling - start polling rfkill
// @wiphy: the wiphy
//
extern "C" {
    pub fn wiphy_rfkill_start_polling(wiphy: *mut wiphy);
}
//
// wiphy_rfkill_stop_polling - stop polling rfkill
// @wiphy: the wiphy
//
// DOC: Vendor commands
//
// Occasionally, there are special protocol or firmware features that
// can't be implemented very openly. For this and similar cases, the
// vendor command functionality allows implementing the features with
// (typically closed-source) userspace and firmware, using nl80211 as
// the configuration mechanism.
//
// A driver supporting vendor commands must register them as an array
// in struct wiphy, with handlers for each one. Each command has an
// OUI and sub command ID to identify it.
//
// Note that this feature should not be (ab)used to implement protocol
// features that could openly be shared across drivers. In particular,
// it must never be required to use vendor commands to implement any
// "normal" functionality that higher-level userspace like connection
// managers etc. need.
//
extern "C" {
    pub fn __cfg80211_send_event_skb(skb: *mut sk_buff, gfp: gfp_t);
}
//
// cfg80211_vendor_cmd_alloc_reply_skb - allocate vendor command reply
// @wiphy: the wiphy
// @approxlen: an upper bound of the length of the data that will
// be put into the skb
//
// This function allocates and pre-fills an skb for a reply to
// a vendor command. Since it is intended for a reply, calling
// it outside of a vendor command's doit() operation is invalid.
//
// The returned skb is pre-filled with some identifying data in
// a way that any data that is put into the skb (with skb_put(),
// nla_put() or similar) will end up being within the
// %NL80211_ATTR_VENDOR_DATA attribute, so all that needs to be done
// with the skb is adding data for the corresponding userspace tool
// which can then read that data out of the vendor data attribute.
// You must not modify the skb in any other way.
//
// When done, call cfg80211_vendor_cmd_reply() with the skb and return
// its error code as the result of the doit() operation.
//
// Return: An allocated and pre-filled skb. %NULL if any errors happen.
//
// cfg80211_vendor_cmd_reply - send the reply skb
// @skb: The skb, must have been allocated with
// cfg80211_vendor_cmd_alloc_reply_skb()
//
// Since calling this function will usually be the last thing
// before returning from the vendor command doit() you should
// return the error code.  Note that this function consumes the
// skb regardless of the return value.
//
// Return: An error code or 0 on success.
//
extern "C" {
    pub fn cfg80211_vendor_cmd_reply(skb: *mut sk_buff) -> c_int;
}
//
// cfg80211_vendor_cmd_get_sender - get the current sender netlink ID
// @wiphy: the wiphy
//
// Return: the current netlink port ID in a vendor command handler.
//
// Context: May only be called from a vendor command handler
//
extern "C" {
    pub fn cfg80211_vendor_cmd_get_sender(wiphy: *mut wiphy) -> c_uint;
}
//
// cfg80211_vendor_event_alloc - allocate vendor-specific event skb
// @wiphy: the wiphy
// @wdev: the wireless device
// @event_idx: index of the vendor event in the wiphy's vendor_events
// @approxlen: an upper bound of the length of the data that will
// be put into the skb
// @gfp: allocation flags
//
// This function allocates and pre-fills an skb for an event on the
// vendor-specific multicast group.
//
// If wdev != NULL, both the ifindex and identifier of the specified
// wireless device are added to the event message before the vendor data
// attribute.
//
// When done filling the skb, call cfg80211_vendor_event() with the
// skb to send the event.
//
// Return: An allocated and pre-filled skb. %NULL if any errors happen.
//
// cfg80211_vendor_event_alloc_ucast - alloc unicast vendor-specific event skb
// @wiphy: the wiphy
// @wdev: the wireless device
// @event_idx: index of the vendor event in the wiphy's vendor_events
// @portid: port ID of the receiver
// @approxlen: an upper bound of the length of the data that will
// be put into the skb
// @gfp: allocation flags
//
// This function allocates and pre-fills an skb for an event to send to
// a specific (userland) socket. This socket would previously have been
// obtained by cfg80211_vendor_cmd_get_sender(), and the caller MUST take
// care to register a netlink notifier to see when the socket closes.
//
// If wdev != NULL, both the ifindex and identifier of the specified
// wireless device are added to the event message before the vendor data
// attribute.
//
// When done filling the skb, call cfg80211_vendor_event() with the
// skb to send the event.
//
// Return: An allocated and pre-filled skb. %NULL if any errors happen.
//
// cfg80211_vendor_event - send the event
// @skb: The skb, must have been allocated with cfg80211_vendor_event_alloc()
// @gfp: allocation flags
//
// This function sends the given @skb, which must have been allocated
// by cfg80211_vendor_event_alloc(), as an event. It always consumes it.
//

//
// DOC: Test mode
//
// Test mode is a set of utility functions to allow drivers to
// interact with driver-specific tools to aid, for instance,
// factory programming.
//
// This chapter describes how drivers interact with it. For more
// information see the nl80211 book's chapter on it.
//
// cfg80211_testmode_alloc_reply_skb - allocate testmode reply
// @wiphy: the wiphy
// @approxlen: an upper bound of the length of the data that will
// be put into the skb
//
// This function allocates and pre-fills an skb for a reply to
// the testmode command. Since it is intended for a reply, calling
// it outside of the @testmode_cmd operation is invalid.
//
// The returned skb is pre-filled with the wiphy index and set up in
// a way that any data that is put into the skb (with skb_put(),
// nla_put() or similar) will end up being within the
// %NL80211_ATTR_TESTDATA attribute, so all that needs to be done
// with the skb is adding data for the corresponding userspace tool
// which can then read that data out of the testdata attribute. You
// must not modify the skb in any other way.
//
// When done, call cfg80211_testmode_reply() with the skb and return
// its error code as the result of the @testmode_cmd operation.
//
// Return: An allocated and pre-filled skb. %NULL if any errors happen.
//
// cfg80211_testmode_reply - send the reply skb
// @skb: The skb, must have been allocated with
// cfg80211_testmode_alloc_reply_skb()
//
// Since calling this function will usually be the last thing
// before returning from the @testmode_cmd you should return
// the error code.  Note that this function consumes the skb
// regardless of the return value.
//
// Return: An error code or 0 on success.
//
extern "C" {
    pub fn cfg80211_vendor_cmd_reply(_arg: skb) -> return;
}
//
// cfg80211_testmode_alloc_event_skb - allocate testmode event
// @wiphy: the wiphy
// @approxlen: an upper bound of the length of the data that will
// be put into the skb
// @gfp: allocation flags
//
// This function allocates and pre-fills an skb for an event on the
// testmode multicast group.
//
// The returned skb is set up in the same way as with
// cfg80211_testmode_alloc_reply_skb() but prepared for an event. As
// there, you should simply add data to it that will then end up in the
// %NL80211_ATTR_TESTDATA attribute. Again, you must not modify the skb
// in any other way.
//
// When done filling the skb, call cfg80211_testmode_event() with the
// skb to send the event.
//
// Return: An allocated and pre-filled skb. %NULL if any errors happen.
//
// cfg80211_testmode_event - send the event
// @skb: The skb, must have been allocated with
// cfg80211_testmode_alloc_event_skb()
// @gfp: allocation flags
//
// This function sends the given @skb, which must have been allocated
// by cfg80211_testmode_alloc_event_skb(), as an event. It always
// consumes it.
//

// Macro flag: #define CFG80211_TESTMODE_CMD(cmd)
// Macro flag: #define CFG80211_TESTMODE_DUMP(cmd)

//
// struct cfg80211_fils_resp_params - FILS connection response params
// @kek: KEK derived from a successful FILS connection (may be %NULL)
// @kek_len: Length of @fils_kek in octets
// @update_erp_next_seq_num: Boolean value to specify whether the value in
// @erp_next_seq_num is valid.
// @erp_next_seq_num: The next sequence number to use in ERP message in
// FILS Authentication. This value should be specified irrespective of the
// status for a FILS connection.
// @pmk: A new PMK if derived from a successful FILS connection (may be %NULL).
// @pmk_len: Length of @pmk in octets
// @pmkid: A new PMKID if derived from a successful FILS connection or the PMKID
// used for this FILS connection (may be %NULL).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_fils_resp_params {
    pub kek: *const u8,
    pub kek_len: usize,
    pub update_erp_next_seq_num: bool,
    pub erp_next_seq_num: u16,
    pub pmk: *const u8,
    pub pmk_len: usize,
    pub pmkid: *const u8,
}

//
// struct cfg80211_connect_resp_params - Connection response params
// @status: Status code, %WLAN_STATUS_SUCCESS for successful connection, use
// %WLAN_STATUS_UNSPECIFIED_FAILURE if your device cannot give you
// the real status code for failures. If this call is used to report a
// failure due to a timeout (e.g., not receiving an Authentication frame
// from the AP) instead of an explicit rejection by the AP, -1 is used to
// indicate that this is a failure, but without a status code.
// @timeout_reason is used to report the reason for the timeout in that
// case.
// @req_ie: Association request IEs (may be %NULL)
// @req_ie_len: Association request IEs length
// @resp_ie: Association response IEs (may be %NULL)
// @resp_ie_len: Association response IEs length
// @fils: FILS connection response parameters.
// @timeout_reason: Reason for connection timeout. This is used when the
// connection fails due to a timeout instead of an explicit rejection from
// the AP. %NL80211_TIMEOUT_UNSPECIFIED is used when the timeout reason is
// not known. This value is used only if @status < 0 to indicate that the
// failure is due to a timeout and not due to explicit rejection by the AP.
// This value is ignored in other cases (@status >= 0).
// @valid_links: For MLO connection, BIT mask of the valid link ids. Otherwise
// zero.
// @ap_mld_addr: For MLO connection, MLD address of the AP. Otherwise %NULL.
// @links : For MLO connection, contains link info for the valid links indicated
// using @valid_links. For non-MLO connection, links[0] contains the
// connected AP info.
// @links.addr: For MLO connection, MAC address of the STA link. Otherwise
// %NULL.
// @links.bssid: For MLO connection, MAC address of the AP link. For non-MLO
// connection, links[0].bssid points to the BSSID of the AP (may be %NULL).
// @links.bss: For MLO connection, entry of bss to which STA link is connected.
// For non-MLO connection, links[0].bss points to entry of bss to which STA
// is connected. It can be obtained through cfg80211_get_bss() (may be
// %NULL). It is recommended to store the bss from the connect_request and
// hold a reference to it and return through this param to avoid a warning
// if the bss is expired during the connection, esp. for those drivers
// implementing connect op. Only one parameter among @bssid and @bss needs
// to be specified.
// @links.status: per-link status code, to report a status code that's not
// %WLAN_STATUS_SUCCESS for a given link, it must also be in the
// @valid_links bitmap and may have a BSS pointer (which is then released)
// @assoc_encrypted: The driver should set this flag to indicate that the
// (Re)Association Request/Response frames are transmitted encrypted over
// the air.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_connect_resp_params {
    pub status: c_int,
    pub req_ie: *const u8,
    pub req_ie_len: usize,
    pub resp_ie: *const u8,
    pub resp_ie_len: usize,
    pub fils: cfg80211_fils_resp_params,
    pub timeout_reason: nl80211_timeout_reason,
    pub assoc_encrypted: bool,
    pub ap_mld_addr: *const u8,
    pub valid_links: u16,
    pub addr: *const u8,
    pub bssid: *const u8,
    pub bss: *mut cfg80211_bss,
    pub status: u16,
    pub links: [}; IEEE80211_MLD_MAX_NUM_LINKS],
}

//
// cfg80211_connect_done - notify cfg80211 of connection result
//
// @dev: network device
// @params: connection response parameters
// @gfp: allocation flags
//
// It should be called by the underlying driver once execution of the connection
// request from connect() has been completed. This is similar to
// cfg80211_connect_bss(), but takes a structure pointer for connection response
// parameters. Only one of the functions among cfg80211_connect_bss(),
// cfg80211_connect_result(), cfg80211_connect_timeout(),
// and cfg80211_connect_done() should be called.
//
// cfg80211_connect_bss - notify cfg80211 of connection result
//
// @dev: network device
// @bssid: the BSSID of the AP
// @bss: Entry of bss to which STA got connected to, can be obtained through
// cfg80211_get_bss() (may be %NULL). But it is recommended to store the
// bss from the connect_request and hold a reference to it and return
// through this param to avoid a warning if the bss is expired during the
// connection, esp. for those drivers implementing connect op.
// Only one parameter among @bssid and @bss needs to be specified.
// @req_ie: association request IEs (maybe be %NULL)
// @req_ie_len: association request IEs length
// @resp_ie: association response IEs (may be %NULL)
// @resp_ie_len: assoc response IEs length
// @status: status code, %WLAN_STATUS_SUCCESS for successful connection, use
// %WLAN_STATUS_UNSPECIFIED_FAILURE if your device cannot give you
// the real status code for failures. If this call is used to report a
// failure due to a timeout (e.g., not receiving an Authentication frame
// from the AP) instead of an explicit rejection by the AP, -1 is used to
// indicate that this is a failure, but without a status code.
// @timeout_reason is used to report the reason for the timeout in that
// case.
// @gfp: allocation flags
// @timeout_reason: reason for connection timeout. This is used when the
// connection fails due to a timeout instead of an explicit rejection from
// the AP. %NL80211_TIMEOUT_UNSPECIFIED is used when the timeout reason is
// not known. This value is used only if @status < 0 to indicate that the
// failure is due to a timeout and not due to explicit rejection by the AP.
// This value is ignored in other cases (@status >= 0).
//
// It should be called by the underlying driver once execution of the connection
// request from connect() has been completed. This is similar to
// cfg80211_connect_result(), but with the option of identifying the exact bss
// entry for the connection. Only one of the functions among
// cfg80211_connect_bss(), cfg80211_connect_result(),
// cfg80211_connect_timeout(), and cfg80211_connect_done() should be called.
//
// cfg80211_connect_result - notify cfg80211 of connection result
//
// @dev: network device
// @bssid: the BSSID of the AP
// @req_ie: association request IEs (maybe be %NULL)
// @req_ie_len: association request IEs length
// @resp_ie: association response IEs (may be %NULL)
// @resp_ie_len: assoc response IEs length
// @status: status code, %WLAN_STATUS_SUCCESS for successful connection, use
// %WLAN_STATUS_UNSPECIFIED_FAILURE if your device cannot give you
// the real status code for failures.
// @gfp: allocation flags
//
// It should be called by the underlying driver once execution of the connection
// request from connect() has been completed. This is similar to
// cfg80211_connect_bss() which allows the exact bss entry to be specified. Only
// one of the functions among cfg80211_connect_bss(), cfg80211_connect_result(),
// cfg80211_connect_timeout(), and cfg80211_connect_done() should be called.
//
// cfg80211_connect_timeout - notify cfg80211 of connection timeout
//
// @dev: network device
// @bssid: the BSSID of the AP
// @req_ie: association request IEs (maybe be %NULL)
// @req_ie_len: association request IEs length
// @gfp: allocation flags
// @timeout_reason: reason for connection timeout.
//
// It should be called by the underlying driver whenever connect() has failed
// in a sequence where no explicit authentication/association rejection was
// received from the AP. This could happen, e.g., due to not being able to send
// out the Authentication or Association Request frame or timing out while
// waiting for the response. Only one of the functions among
// cfg80211_connect_bss(), cfg80211_connect_result(),
// cfg80211_connect_timeout(), and cfg80211_connect_done() should be called.
//
// struct cfg80211_roam_info - driver initiated roaming information
//
// @req_ie: association request IEs (maybe be %NULL)
// @req_ie_len: association request IEs length
// @resp_ie: association response IEs (may be %NULL)
// @resp_ie_len: assoc response IEs length
// @fils: FILS related roaming information.
// @valid_links: For MLO roaming, BIT mask of the new valid links is set.
// Otherwise zero.
// @ap_mld_addr: For MLO roaming, MLD address of the new AP. Otherwise %NULL.
// @links : For MLO roaming, contains new link info for the valid links set in
// @valid_links. For non-MLO roaming, links[0] contains the new AP info.
// @links.addr: For MLO roaming, MAC address of the STA link. Otherwise %NULL.
// @links.bssid: For MLO roaming, MAC address of the new AP link. For non-MLO
// roaming, links[0].bssid points to the BSSID of the new AP. May be
// %NULL if %links.bss is set.
// @links.channel: the channel of the new AP.
// @links.bss: For MLO roaming, entry of new bss to which STA link got
// roamed. For non-MLO roaming, links[0].bss points to entry of bss to
// which STA got roamed (may be %NULL if %links.bssid is set)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_roam_info {
    pub req_ie: *const u8,
    pub req_ie_len: usize,
    pub resp_ie: *const u8,
    pub resp_ie_len: usize,
    pub fils: cfg80211_fils_resp_params,
    pub ap_mld_addr: *const u8,
    pub valid_links: u16,
    pub addr: *const u8,
    pub bssid: *const u8,
    pub channel: *mut ieee80211_channel,
    pub bss: *mut cfg80211_bss,
    pub links: [}; IEEE80211_MLD_MAX_NUM_LINKS],
}

//
// cfg80211_roamed - notify cfg80211 of roaming
//
// @dev: network device
// @info: information about the new BSS. struct &cfg80211_roam_info.
// @gfp: allocation flags
//
// This function may be called with the driver passing either the BSSID of the
// new AP or passing the bss entry to avoid a race in timeout of the bss entry.
// It should be called by the underlying driver whenever it roamed from one AP
// to another while connected. Drivers which have roaming implemented in
// firmware should pass the bss entry to avoid a race in bss entry timeout where
// the bss entry of the new AP is seen in the driver, but gets timed out by the
// time it is accessed in __cfg80211_roamed() due to delay in scheduling
// rdev->event_work. In case of any failures, the reference is released
// either in cfg80211_roamed() or in __cfg80211_romed(), Otherwise, it will be
// released while disconnecting from the current bss.
//
// cfg80211_port_authorized - notify cfg80211 of successful security association
//
// @dev: network device
// @peer_addr: BSSID of the AP/P2P GO in case of STA/GC or STA/GC MAC address
// in case of AP/P2P GO
// @td_bitmap: transition disable policy
// @td_bitmap_len: Length of transition disable policy
// @gfp: allocation flags
//
// This function should be called by a driver that supports 4 way handshake
// offload after a security association was successfully established (i.e.,
// the 4 way handshake was completed successfully). The call to this function
// should be preceded with a call to cfg80211_connect_result(),
// cfg80211_connect_done(), cfg80211_connect_bss() or cfg80211_roamed() to
// indicate the 802.11 association.
// This function can also be called by AP/P2P GO driver that supports
// authentication offload. In this case the peer_mac passed is that of
// associated STA/GC.
//
// cfg80211_disconnected - notify cfg80211 that connection was dropped
//
// @dev: network device
// @ie: information elements of the deauth/disassoc frame (may be %NULL)
// @ie_len: length of IEs
// @reason: reason code for the disconnection, set it to 0 if unknown
// @locally_generated: disconnection was requested locally
// @gfp: allocation flags
//
// After it calls this function, the driver should enter an idle state
// and not try to connect to any AP any more.
//
// cfg80211_ready_on_channel - notification of remain_on_channel start
// @wdev: wireless device
// @cookie: the request cookie
// @chan: The current channel (from remain_on_channel request)
// @duration: Duration in milliseconds that the driver intents to remain on the
// channel
// @gfp: allocation flags
//
// cfg80211_remain_on_channel_expired - remain_on_channel duration expired
// @wdev: wireless device
// @cookie: the request cookie
// @chan: The current channel (from remain_on_channel request)
// @gfp: allocation flags
//
// cfg80211_tx_mgmt_expired - tx_mgmt duration expired
// @wdev: wireless device
// @cookie: the requested cookie
// @chan: The current channel (from tx_mgmt request)
// @gfp: allocation flags
//
// cfg80211_sinfo_alloc_tid_stats - allocate per-tid statistics.
//
// @sinfo: the station information
// @gfp: allocation flags
//
// Return: 0 on success. Non-zero on error.
//
extern "C" {
    pub fn cfg80211_sinfo_alloc_tid_stats(sinfo: *mut station_info, gfp: gfp_t) -> c_int;
}
//
// cfg80211_link_sinfo_alloc_tid_stats - allocate per-tid statistics.
//
// @link_sinfo: the link station information
// @gfp: allocation flags
//
// Return: 0 on success. Non-zero on error.
//
// cfg80211_sinfo_release_content - release contents of station info
// @sinfo: the station information
//
// Releases any potentially allocated sub-information of the station
// information, but not the struct itself (since it's typically on
// the stack.)
//
// cfg80211_new_sta - notify userspace about station
//
// @wdev: the wireless device
// @mac_addr: the station's address
// @sinfo: the station information
// @gfp: allocation flags
//
// cfg80211_del_sta_sinfo - notify userspace about deletion of a station
// @wdev: the wireless device
// @mac_addr: the station's address. For MLD station, MLD address is used.
// @sinfo: the station information/statistics
// @gfp: allocation flags
//
// cfg80211_del_sta - notify userspace about deletion of a station
//
// @wdev: the wireless device
// @mac_addr: the station's address. For MLD station, MLD address is used.
// @gfp: allocation flags
//
// cfg80211_conn_failed - connection request failed notification
//
// @dev: the netdev
// @mac_addr: the station's address
// @reason: the reason for connection failure
// @gfp: allocation flags
//
// Whenever a station tries to connect to an AP and if the station
// could not connect to the AP as the AP has rejected the connection
// for some reasons, this function is called.
//
// The reason for connection failure can be any of the value from
// nl80211_connect_failed_reason enum
//
// struct cfg80211_rx_info - received management frame info
//
// @freq: Frequency on which the frame was received in kHz
// @sig_dbm: signal strength in dBm, or 0 if unknown
// @have_link_id: indicates the frame was received on a link of
// an MLD, i.e. the @link_id field is valid
// @link_id: the ID of the link the frame was received	on
// @buf: Management frame (header + body)
// @len: length of the frame data
// @flags: flags, as defined in &enum nl80211_rxmgmt_flags
// @rx_tstamp: Hardware timestamp of frame RX in nanoseconds
// @ack_tstamp: Hardware timestamp of ack TX in nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_rx_info {
    pub freq: c_int,
    pub sig_dbm: c_int,
    pub have_link_id: bool,
    pub link_id: u8,
    pub buf: *const u8,
    pub len: usize,
    pub flags: u32,
    pub rx_tstamp: u64,
    pub ack_tstamp: u64,
}

//
// cfg80211_rx_mgmt_ext - management frame notification with extended info
// @wdev: wireless device receiving the frame
// @info: RX info as defined in struct cfg80211_rx_info
//
// This function is called whenever an Action frame is received for a station
// mode interface, but is not processed in kernel.
//
// Return: %true if a user space application has registered for this frame.
// For action frames, that makes it responsible for rejecting unrecognized
// action frames; %false otherwise, in which case for action frames the
// driver is responsible for rejecting the frame.
//
// cfg80211_rx_mgmt_khz - notification of received, unprocessed management frame
// @wdev: wireless device receiving the frame
// @freq: Frequency on which the frame was received in KHz
// @sig_dbm: signal strength in dBm, or 0 if unknown
// @buf: Management frame (header + body)
// @len: length of the frame data
// @flags: flags, as defined in enum nl80211_rxmgmt_flags
//
// This function is called whenever an Action frame is received for a station
// mode interface, but is not processed in kernel.
//
// Return: %true if a user space application has registered for this frame.
// For action frames, that makes it responsible for rejecting unrecognized
// action frames; %false otherwise, in which case for action frames the
// driver is responsible for rejecting the frame.
//
extern "C" {
    pub fn cfg80211_rx_mgmt_ext(_arg: wdev, _arg: &info) -> return;
}
//
// cfg80211_rx_mgmt - notification of received, unprocessed management frame
// @wdev: wireless device receiving the frame
// @freq: Frequency on which the frame was received in MHz
// @sig_dbm: signal strength in dBm, or 0 if unknown
// @buf: Management frame (header + body)
// @len: length of the frame data
// @flags: flags, as defined in enum nl80211_rxmgmt_flags
//
// This function is called whenever an Action frame is received for a station
// mode interface, but is not processed in kernel.
//
// Return: %true if a user space application has registered for this frame.
// For action frames, that makes it responsible for rejecting unrecognized
// action frames; %false otherwise, in which case for action frames the
// driver is responsible for rejecting the frame.
//
extern "C" {
    pub fn cfg80211_rx_mgmt_ext(_arg: wdev, _arg: &info) -> return;
}
//
// struct cfg80211_tx_status - TX status for management frame information
//
// @cookie: Cookie returned by cfg80211_ops::mgmt_tx()
// @tx_tstamp: hardware TX timestamp in nanoseconds
// @ack_tstamp: hardware ack RX timestamp in nanoseconds
// @buf: Management frame (header + body)
// @len: length of the frame data
// @ack: Whether frame was acknowledged
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_tx_status {
    pub cookie: u64,
    pub tx_tstamp: u64,
    pub ack_tstamp: u64,
    pub buf: *const u8,
    pub len: usize,
    pub ack: bool,
}

//
// cfg80211_mgmt_tx_status_ext - TX status notification with extended info
// @wdev: wireless device receiving the frame
// @status: TX status data
// @gfp: context flags
//
// This function is called whenever a management frame was requested to be
// transmitted with cfg80211_ops::mgmt_tx() to report the TX status of the
// transmission attempt with extended info.
//
// cfg80211_mgmt_tx_status - notification of TX status for management frame
// @wdev: wireless device receiving the frame
// @cookie: Cookie returned by cfg80211_ops::mgmt_tx()
// @buf: Management frame (header + body)
// @len: length of the frame data
// @ack: Whether frame was acknowledged
// @gfp: context flags
//
// This function is called whenever a management frame was requested to be
// transmitted with cfg80211_ops::mgmt_tx() to report the TX status of the
// transmission attempt.
//
// cfg80211_control_port_tx_status - notification of TX status for control
// port frames
// @wdev: wireless device receiving the frame
// @cookie: Cookie returned by cfg80211_ops::tx_control_port()
// @buf: Data frame (header + body)
// @len: length of the frame data
// @ack: Whether frame was acknowledged
// @gfp: context flags
//
// This function is called whenever a control port frame was requested to be
// transmitted with cfg80211_ops::tx_control_port() to report the TX status of
// the transmission attempt.
//
// cfg80211_rx_control_port - notification about a received control port frame
// @dev: The device the frame matched to
// @skb: The skbuf with the control port frame.  It is assumed that the skbuf
// is 802.3 formatted (with 802.3 header).  The skb can be non-linear.
// This function does not take ownership of the skb, so the caller is
// responsible for any cleanup.  The caller must also ensure that
// skb->protocol is set appropriately.
// @unencrypted: Whether the frame was received unencrypted
// @link_id: the link the frame was received on, -1 if not applicable or unknown
//
// This function is used to inform userspace about a received control port
// frame.  It should only be used if userspace indicated it wants to receive
// control port frames over nl80211.
//
// The frame is the data portion of the 802.3 or 802.11 data frame with all
// network layer headers removed (e.g. the raw EAPoL frame).
//
// Return: %true if the frame was passed to userspace
//
// cfg80211_cqm_rssi_notify - connection quality monitoring rssi event
// @dev: network device
// @rssi_event: the triggered RSSI event
// @rssi_level: new RSSI level value or 0 if not available
// @gfp: context flags
//
// This function is called when a configured connection quality monitoring
// rssi threshold reached event occurs.
//
// cfg80211_cqm_pktloss_notify - notify userspace about packetloss to peer
// @dev: network device
// @peer: peer's MAC address
// @num_packets: how many packets were lost -- should be a fixed threshold
// but probably no less than maybe 50, or maybe a throughput dependent
// threshold (to account for temporary interference)
// @gfp: context flags
//
// cfg80211_cqm_txe_notify - TX error rate event
// @dev: network device
// @peer: peer's MAC address
// @num_packets: how many packets were lost
// @rate: % of packets which failed transmission
// @intvl: interval (in s) over which the TX failure threshold was breached.
// @gfp: context flags
//
// Notify userspace when configured % TX failures over number of packets in a
// given interval is exceeded.
//
// cfg80211_cqm_beacon_loss_notify - beacon loss event
// @dev: network device
// @gfp: context flags
//
// Notify userspace about beacon loss from the connected AP.
//
extern "C" {
    pub fn cfg80211_cqm_beacon_loss_notify(dev: *mut net_device, gfp: gfp_t);
}
//
// __cfg80211_radar_event - radar detection event
// @wiphy: the wiphy
// @chandef: chandef for the current channel
// @offchan: the radar has been detected on the offchannel chain
// @gfp: context flags
//
// This function is called when a radar is detected on the current chanenl.
//
// cfg80211_sta_opmode_change_notify - STA's ht/vht operation mode change event
// @dev: network device
// @mac: MAC address of a station which opmode got modified
// @sta_opmode: station's current opmode value
// @gfp: context flags
//
// Driver should call this function when station's opmode modified via action
// frame.
//
// cfg80211_cac_event - Channel availability check (CAC) event
// @netdev: network device
// @chandef: chandef for the current channel
// @event: type of event
// @gfp: context flags
// @link_id: valid link_id for MLO operation or 0 otherwise.
//
// This function is called when a Channel availability check (CAC) is finished
// or aborted. This must be called to notify the completion of a CAC process,
// also by full-MAC drivers.
//
// cfg80211_background_cac_abort - Channel Availability Check offchan abort event
// @wiphy: the wiphy
//
// This function is called by the driver when a Channel Availability Check
// (CAC) is aborted by a offchannel dedicated chain.
//
extern "C" {
    pub fn cfg80211_background_cac_abort(wiphy: *mut wiphy);
}
//
// cfg80211_gtk_rekey_notify - notify userspace about driver rekeying
// @dev: network device
// @bssid: BSSID of AP (to avoid races)
// @replay_ctr: new replay counter
// @gfp: allocation flags
//
// cfg80211_pmksa_candidate_notify - notify about PMKSA caching candidate
// @dev: network device
// @index: candidate index (the smaller the index, the higher the priority)
// @bssid: BSSID of AP
// @preauth: Whether AP advertises support for RSN pre-authentication
// @gfp: allocation flags
//
// cfg80211_rx_spurious_frame - inform userspace about a spurious frame
// @dev: The device the frame matched to
// @link_id: the link the frame was received on, -1 if not applicable or unknown
// @addr: the transmitter address
// @gfp: context flags
//
// This function is used in AP mode to inform userspace that a spurious
// class 3 frame was received, to be able to deauth the sender.
// It is also used in NAN_DATA mode to report frames from unknown peers
// (A2 not assigned to any active NDP), per Wi-Fi Aware (TM) 4.0 specification 6.2.5.
// Return: %true if the frame was passed to userspace (or this failed
// for a reason other than not having a subscription.)
//
// cfg80211_rx_unexpected_4addr_frame - inform about unexpected WDS frame
// @dev: The device the frame matched to
// @addr: the transmitter address
// @link_id: the link the frame was received on, -1 if not applicable or unknown
// @gfp: context flags
//
// This function is used in AP mode (only!) to inform userspace that
// an associated station sent a 4addr frame but that wasn't expected.
// It is allowed and desirable to send this event only once for each
// station to avoid event flooding.
// Return: %true if the frame was passed to userspace (or this failed
// for a reason other than not having a subscription.)
//
// cfg80211_probe_status - notify userspace about probe status
// @dev: the device the probe was sent on
// @peer: The peer MAC address (or MLD address for MLO) or %NULL if not
// applicable (e.g. for STA/P2P-client)
// @cookie: the cookie filled in @probe_peer previously
// @link_id: The link ID on which the probe was sent (or -1 for non-MLO)
// @acked: indicates whether probe was acked or not
// @ack_signal: signal strength (in dBm) of the ACK frame.
// @is_valid_ack_signal: indicates the ack_signal is valid or not.
// @gfp: allocation flags
//
// cfg80211_report_obss_beacon_khz - report beacon from other APs
// @wiphy: The wiphy that received the beacon
// @frame: the frame
// @len: length of the frame
// @freq: frequency the frame was received on in KHz
// @sig_dbm: signal strength in dBm, or 0 if unknown
//
// Use this function to report to userspace when a beacon was
// received. It is not useful to call this when there is no
// netdev that is in AP/GO mode.
//
// cfg80211_report_obss_beacon - report beacon from other APs
// @wiphy: The wiphy that received the beacon
// @frame: the frame
// @len: length of the frame
// @freq: frequency the frame was received on
// @sig_dbm: signal strength in dBm, or 0 if unknown
//
// Use this function to report to userspace when a beacon was
// received. It is not useful to call this when there is no
// netdev that is in AP/GO mode.
//
// struct cfg80211_beaconing_check_config - beacon check configuration
// @iftype: the interface type to check for
// @relax: allow IR-relaxation conditions to apply (e.g. another
// interface connected already on the same channel)
// NOTE: If this is set, wiphy mutex must be held.
// @reg_power: &enum ieee80211_ap_reg_power value indicating the
// advertised/used 6 GHz regulatory power setting
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_beaconing_check_config {
    pub iftype: nl80211_iftype,
    pub reg_power: ieee80211_ap_reg_power,
    pub relax: bool,
}

//
// cfg80211_reg_check_beaconing - check if beaconing is allowed
// @wiphy: the wiphy
// @chandef: the channel definition
// @cfg: additional parameters for the checking
//
// Return: %true if there is no secondary channel or the secondary channel(s)
// can be used for beaconing (i.e. is not a radar channel etc.)
//
// cfg80211_reg_can_beacon - check if beaconing is allowed
// @wiphy: the wiphy
// @chandef: the channel definition
// @iftype: interface type
//
// Return: %true if there is no secondary channel or the secondary channel(s)
// can be used for beaconing (i.e. is not a radar channel etc.)
//
extern "C" {
    pub fn cfg80211_reg_check_beaconing(_arg: wiphy, _arg: chandef, _arg: &config) -> return;
}
//
// cfg80211_reg_can_beacon_relax - check if beaconing is allowed with relaxation
// @wiphy: the wiphy
// @chandef: the channel definition
// @iftype: interface type
//
// Return: %true if there is no secondary channel or the secondary channel(s)
// can be used for beaconing (i.e. is not a radar channel etc.). This version
// also checks if IR-relaxation conditions apply, to allow beaconing under
// more permissive conditions.
//
// Context: Requires the wiphy mutex to be held.
//
extern "C" {
    pub fn cfg80211_reg_check_beaconing(_arg: wiphy, _arg: chandef, _arg: &config) -> return;
}
//
// cfg80211_ch_switch_notify - update wdev channel and notify userspace
// @dev: the device which switched channels
// @chandef: the new channel definition
// @link_id: the link ID for MLO, must be 0 for non-MLO
//
// Caller must hold wiphy mutex, therefore must only be called from sleepable
// driver context!
//
// cfg80211_ch_switch_started_notify - notify channel switch start
// @dev: the device on which the channel switch started
// @chandef: the future channel definition
// @link_id: the link ID for MLO, must be 0 for non-MLO
// @count: the number of TBTTs until the channel switch happens
// @quiet: whether or not immediate quiet was requested by the AP
//
// Inform the userspace about the channel switch that has just
// started, so that it can take appropriate actions (eg. starting
// channel switch on other vifs), if necessary.
//
// ieee80211_operating_class_to_band - convert operating class to band
//
// @operating_class: the operating class to convert
// @band: band pointer to fill
//
// Return: %true if the conversion was successful, %false otherwise.
//
// ieee80211_operating_class_to_chandef - convert operating class to chandef
//
// @operating_class: the operating class to convert
// @chan: the ieee80211_channel to convert
// @chandef: a pointer to the resulting chandef
//
// Return: %true if the conversion was successful, %false otherwise.
//
// ieee80211_chandef_to_operating_class - convert chandef to operation class
//
// @chandef: the chandef to convert
// @op_class: a pointer to the resulting operating class
//
// Return: %true if the conversion was successful, %false otherwise.
//
// ieee80211_chandef_to_khz - convert chandef to frequency in KHz
//
// @chandef: the chandef to convert
//
// Return: the center frequency of chandef (1st segment) in KHz.
//
// cfg80211_tdls_oper_request - request userspace to perform TDLS operation
// @dev: the device on which the operation is requested
// @peer: the MAC address of the peer device
// @oper: the requested TDLS operation (NL80211_TDLS_SETUP or
// NL80211_TDLS_TEARDOWN)
// @reason_code: the reason code for teardown request
// @gfp: allocation flags
//
// This function is used to request userspace to perform TDLS operation that
// requires knowledge of keys, i.e., link setup or teardown when the AP
// connection uses encryption. This is optional mechanism for the driver to use
// if it can automatically determine when a TDLS link could be useful (e.g.,
// based on traffic and signal strength for a peer).
//
// cfg80211_calculate_bitrate - calculate actual bitrate (in 100Kbps units)
// @rate: given rate_info to calculate bitrate from
//
// Return: calculated bitrate
//
extern "C" {
    pub fn cfg80211_calculate_bitrate(rate: *mut rate_info) -> u32;
}
//
// cfg80211_unregister_wdev - remove the given wdev
// @wdev: struct wireless_dev to remove
//
// This function removes the device so it can no longer be used. It is necessary
// to call this function even when cfg80211 requests the removal of the device
// by calling the del_virtual_intf() callback. The function must also be called
// when the driver wishes to unregister the wdev, e.g. when the hardware device
// is unbound from the driver.
//
// Context: Requires the RTNL and wiphy mutex to be held.
//
extern "C" {
    pub fn cfg80211_unregister_wdev(wdev: *mut wireless_dev);
}
//
// cfg80211_register_netdevice - register the given netdev
// @dev: the netdev to register
//
// Note: In contexts coming from cfg80211 callbacks, you must call this rather
// than register_netdevice(), unregister_netdev() is impossible as the RTNL is
// held. Otherwise, both register_netdevice() and register_netdev() are usable
// instead as well.
//
// Context: Requires the RTNL and wiphy mutex to be held.
//
// Return: 0 on success. Non-zero on error.
//
extern "C" {
    pub fn cfg80211_register_netdevice(dev: *mut net_device) -> c_int;
}
//
// cfg80211_unregister_netdevice - unregister the given netdev
// @dev: the netdev to register
//
// Note: In contexts coming from cfg80211 callbacks, you must call this rather
// than unregister_netdevice(), unregister_netdev() is impossible as the RTNL
// is held. Otherwise, both unregister_netdevice() and unregister_netdev() are
// usable instead as well.
//
// Context: Requires the RTNL and wiphy mutex to be held.
//

//
// struct cfg80211_ft_event_params - FT Information Elements
// @ies: FT IEs
// @ies_len: length of the FT IE in bytes
// @target_ap: target AP's MAC address
// @ric_ies: RIC IE
// @ric_ies_len: length of the RIC IE in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_ft_event_params {
    pub ies: *const u8,
    pub ies_len: usize,
    pub target_ap: *const u8,
    pub ric_ies: *const u8,
    pub ric_ies_len: usize,
}

//
// cfg80211_ft_event - notify userspace about FT IE and RIC IE
// @netdev: network device
// @ft_event: IE information
//
// cfg80211_get_p2p_attr - find and copy a P2P attribute from IE buffer
// @ies: the input IE buffer
// @len: the input length
// @attr: the attribute ID to find
// @buf: output buffer, can be %NULL if the data isn't needed, e.g.
// if the function is only called to get the needed buffer size
// @bufsize: size of the output buffer
//
// The function finds a given P2P attribute in the (vendor) IEs and
// copies its contents to the given buffer.
//
// Return: A negative error code (-%EILSEQ or -%ENOENT) if the data is
// malformed or the attribute can't be found (respectively), or the
// length of the found attribute (which can be zero).
//
// ieee80211_ie_split_ric - split an IE buffer according to ordering (with RIC)
// @ies: the IE buffer
// @ielen: the length of the IE buffer
// @ids: an array with element IDs that are allowed before
// the split. A WLAN_EID_EXTENSION value means that the next
// EID in the list is a sub-element of the EXTENSION IE.
// @n_ids: the size of the element ID array
// @after_ric: array IE types that come after the RIC element
// @n_after_ric: size of the @after_ric array
// @offset: offset where to start splitting in the buffer
//
// This function splits an IE buffer by updating the @offset
// variable to point to the location where the buffer should be
// split.
//
// It assumes that the given IE buffer is well-formed, this
// has to be guaranteed by the caller!
//
// It also assumes that the IEs in the buffer are ordered
// correctly, if not the result of using this function will not
// be ordered correctly either, i.e. it does no reordering.
//
// Return: The offset where the next part of the buffer starts, which
// may be @ielen if the entire (remainder) of the buffer should be
// used.
//
// ieee80211_ie_split - split an IE buffer according to ordering
// @ies: the IE buffer
// @ielen: the length of the IE buffer
// @ids: an array with element IDs that are allowed before
// the split. A WLAN_EID_EXTENSION value means that the next
// EID in the list is a sub-element of the EXTENSION IE.
// @n_ids: the size of the element ID array
// @offset: offset where to start splitting in the buffer
//
// This function splits an IE buffer by updating the @offset
// variable to point to the location where the buffer should be
// split.
//
// It assumes that the given IE buffer is well-formed, this
// has to be guaranteed by the caller!
//
// It also assumes that the IEs in the buffer are ordered
// correctly, if not the result of using this function will not
// be ordered correctly either, i.e. it does no reordering.
//
// Return: The offset where the next part of the buffer starts, which
// may be @ielen if the entire (remainder) of the buffer should be
// used.
//
extern "C" {
    pub fn ieee80211_ie_split_ric(_arg: ies, _arg: ielen, _arg: ids, _arg: n_ids, _arg: NULL, _arg: 0, _arg: offset) -> return;
}
//
// ieee80211_fragment_element - fragment the last element in skb
// @skb: The skbuf that the element was added to
// @len_pos: Pointer to length of the element to fragment
// @frag_id: The element ID to use for fragments
//
// This function fragments all data after @len_pos, adding fragmentation
// elements with the given ID as appropriate. The SKB will grow in size
// accordingly.
//
extern "C" {
    pub fn ieee80211_fragment_element(skb: *mut sk_buff, len_pos: *mut u8, frag_id: u8);
}
//
// cfg80211_report_wowlan_wakeup - report wakeup from WoWLAN
// @wdev: the wireless device reporting the wakeup
// @wakeup: the wakeup report
// @gfp: allocation flags
//
// This function reports that the given device woke up. If it
// caused the wakeup, report the reason(s), otherwise you may
// pass %NULL as the @wakeup parameter to advertise that something
// else caused the wakeup.
//
// cfg80211_crit_proto_stopped() - indicate critical protocol stopped by driver.
//
// @wdev: the wireless device for which critical protocol is stopped.
// @gfp: allocation flags
//
// This function can be called by the driver to indicate it has reverted
// operation back to normal. One reason could be that the duration given
// by .crit_proto_start() has expired.
//
extern "C" {
    pub fn cfg80211_crit_proto_stopped(wdev: *mut wireless_dev, gfp: gfp_t);
}
//
// ieee80211_get_num_supported_channels - get number of channels device has
// @wiphy: the wiphy
//
// Return: the number of channels supported by the device.
//
extern "C" {
    pub fn ieee80211_get_num_supported_channels(wiphy: *mut wiphy) -> c_uint;
}
//
// cfg80211_check_combinations - check interface combinations
//
// @wiphy: the wiphy
// @params: the interface combinations parameter
//
// This function can be called by the driver to check whether a
// combination of interfaces and their types are allowed according to
// the interface combinations.
//
// Return: 0 if combinations are allowed. Non-zero on error.
//
// cfg80211_iter_combinations - iterate over matching combinations
//
// @wiphy: the wiphy
// @params: the interface combinations parameter
// @iter: function to call for each matching combination
// @data: pointer to pass to iter function
//
// This function can be called by the driver to check what possible
// combinations it fits in at a given moment, e.g. for channel switching
// purposes.
//
// Return: 0 on success. Non-zero on error.
//
// cfg80211_get_radio_idx_by_chan - get the radio index by the channel
//
// @wiphy: the wiphy
// @chan: channel for which the supported radio index is required
//
// Return: radio index on success or -EINVAL otherwise
//
// cfg80211_stop_link - stop AP/P2P_GO link if link_id is non-negative or stops
// all links on the interface.
//
// @wiphy: the wiphy
// @wdev: wireless device
// @link_id: valid link ID in case of MLO AP/P2P_GO Operation or else -1
// @gfp: context flags
//
// If link_id is set during MLO operation, stops only the specified AP/P2P_GO
// link and if link_id is set to -1 or last link is stopped, the entire
// interface is stopped as if AP was stopped, IBSS/mesh left, STA disconnected.
//
// cfg80211_stop_iface - trigger interface disconnection
//
// @wiphy: the wiphy
// @wdev: wireless device
// @gfp: context flags
//
// Trigger interface to be stopped as if AP was stopped, IBSS/mesh left, STA
// disconnected.
//
// Note: This doesn't need any locks and is asynchronous.
//
// cfg80211_shutdown_all_interfaces - shut down all interfaces for a wiphy
// @wiphy: the wiphy to shut down
//
// This function shuts down all interfaces belonging to this wiphy by
// calling dev_close() (and treating non-netdev interfaces as needed).
// It shouldn't really be used unless there are some fatal device errors
// that really can't be recovered in any other way.
//
// Callers must hold the RTNL and be able to deal with callbacks into
// the driver while the function is running.
//
extern "C" {
    pub fn cfg80211_shutdown_all_interfaces(wiphy: *mut wiphy);
}
//
// wiphy_ext_feature_set - set the extended feature flag
//
// @wiphy: the wiphy to modify.
// @ftidx: extended feature bit index.
//
// The extended features are flagged in multiple bytes (see
// &struct wiphy.@ext_features)
//
// ft_byte |= BIT(ftidx % 8);
//
// wiphy_ext_feature_isset - check the extended feature flag
//
// @wiphy: the wiphy to modify.
// @ftidx: extended feature bit index.
//
// The extended features are flagged in multiple bytes (see
// &struct wiphy.@ext_features)
//
// Return: %true if extended feature flag is set, %false otherwise
//
// cfg80211_free_nan_func - free NAN function
// @f: NAN function that should be freed
//
// Frees all the NAN function and all it's allocated members.
//
extern "C" {
    pub fn cfg80211_free_nan_func(f: *mut cfg80211_nan_func);
}
//
// struct cfg80211_nan_match_params - NAN match parameters
// @type: the type of the function that triggered a match. If it is
// %NL80211_NAN_FUNC_SUBSCRIBE it means that we replied to a subscriber.
// If it is %NL80211_NAN_FUNC_PUBLISH, it means that we got a discovery
// result.
// If it is %NL80211_NAN_FUNC_FOLLOW_UP, we received a follow up.
// @inst_id: the local instance id
// @peer_inst_id: the instance id of the peer's function
// @addr: the MAC address of the peer
// @info_len: the length of the &info
// @info: the Service Specific Info from the peer (if any)
// @cookie: unique identifier of the corresponding function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_nan_match_params {
    pub type: nl80211_nan_function_type,
    pub inst_id: u8,
    pub peer_inst_id: u8,
    pub addr: *const u8,
    pub info_len: u8,
    pub info: *const u8,
    pub cookie: u64,
}

//
// cfg80211_nan_match - report a match for a NAN function.
// @wdev: the wireless device reporting the match
// @match: match notification parameters
// @gfp: allocation flags
//
// This function reports that the a NAN function had a match. This
// can be a subscribe that had a match or a solicited publish that
// was sent. It can also be a follow up that was received.
//
// cfg80211_nan_func_terminated - notify about NAN function termination.
//
// @wdev: the wireless device reporting the match
// @inst_id: the local instance id
// @reason: termination reason (one of the NL80211_NAN_FUNC_TERM_REASON_*)
// @cookie: unique NAN function identifier
// @gfp: allocation flags
//
// This function reports that the a NAN function is terminated.
//
// cfg80211_nan_sched_update_done - notify deferred schedule update completion
// @wdev: the wireless device reporting the event
// @success: whether or not the schedule update was successful
// @gfp: allocation flags
//
// This function notifies user space that a deferred local NAN schedule update
// (requested with %NL80211_ATTR_NAN_SCHED_DEFERRED) has been completed.
//
// ethtool helper
extern "C" {
    pub fn cfg80211_get_drvinfo(dev: *mut net_device, info: *mut ethtool_drvinfo);
}
//
// cfg80211_external_auth_request - userspace request for authentication
// @netdev: network device
// @params: External authentication parameters
// @gfp: allocation flags
// Returns: 0 on success, < 0 on error
//
// cfg80211_pmsr_report - report peer measurement result data
// @wdev: the wireless device reporting the measurement
// @req: the original measurement request
// @result: the result data
// @gfp: allocation flags
//
// cfg80211_pmsr_complete - report peer measurement completed
// @wdev: the wireless device reporting the measurement
// @req: the original measurement request
// @gfp: allocation flags
//
// Report that the entire measurement completed, after this
// the request pointer will no longer be valid.
//
// cfg80211_iftype_allowed - check whether the interface can be allowed
// @wiphy: the wiphy
// @iftype: interface type
// @is_4addr: use_4addr flag, must be '0' when check_swif is '1'
// @check_swif: check iftype against software interfaces
//
// Check whether the interface is allowed to operate; additionally, this API
// can be used to check iftype against the software interfaces when
// check_swif is '1'.
//
// Return: %true if allowed, %false otherwise
//
// cfg80211_assoc_comeback - notification of association that was
// temporarily rejected with a comeback
// @netdev: network device
// @ap_addr: AP (MLD) address that rejected the association
// @timeout: timeout interval value TUs.
//
// this function may sleep. the caller must hold the corresponding wdev's mutex.
//
// Logging, debugging and troubleshooting/diagnostic helpers.
// wiphy_printk helpers, similar to dev_printk

//
// wiphy_WARN() acts like wiphy_printk(), but with the key difference
// of using a WARN/WARN_ON to get the message out, including the
// file/line information and a backtrace.
//

//
// cfg80211_update_owe_info_event - Notify the peer's OWE info to user space
// @netdev: network device
// @owe_info: peer's owe info
// @gfp: allocation flags
//
// cfg80211_bss_flush - resets all the scan entries
// @wiphy: the wiphy
//
extern "C" {
    pub fn cfg80211_bss_flush(wiphy: *mut wiphy);
}
//
// cfg80211_bss_color_notify - notify about bss color event
// @dev: network device
// @cmd: the actual event we want to notify
// @count: the number of TBTTs until the color change happens
// @color_bitmap: representations of the colors that the local BSS is aware of
// @link_id: valid link_id in case of MLO or 0 for non-MLO.
//
// Return: 0 on success. Non-zero on error.
//
// cfg80211_obss_color_collision_notify - notify about bss color collision
// @dev: network device
// @color_bitmap: representations of the colors that the local BSS is aware of
// @link_id: valid link_id in case of MLO or 0 for non-MLO.
//
// Return: 0 on success. Non-zero on error.
//
// cfg80211_color_change_started_notify - notify color change start
// @dev: the device on which the color is switched
// @count: the number of TBTTs until the color change happens
// @link_id: valid link_id in case of MLO or 0 for non-MLO.
//
// Inform the userspace about the color change that has started.
//
// Return: 0 on success. Non-zero on error.
//
// cfg80211_color_change_aborted_notify - notify color change abort
// @dev: the device on which the color is switched
// @link_id: valid link_id in case of MLO or 0 for non-MLO.
//
// Inform the userspace about the color change that has aborted.
//
// Return: 0 on success. Non-zero on error.
//
// cfg80211_color_change_notify - notify color change completion
// @dev: the device on which the color was switched
// @link_id: valid link_id in case of MLO or 0 for non-MLO.
//
// Inform the userspace about the color change that has completed.
//
// Return: 0 on success. Non-zero on error.
//
// cfg80211_6ghz_power_type - determine AP regulatory power type
// @control: control flags
// @client_flags: &enum ieee80211_channel_flags for station mode to enable
// SP to LPI fallback, zero otherwise.
//
// Return: regulatory power type from &enum ieee80211_ap_reg_power
//
// cfg80211_links_removed - Notify about removed STA MLD setup links.
// @dev: network device.
// @link_mask: BIT mask of removed STA MLD setup link IDs.
//
// Inform cfg80211 and the userspace about removed STA MLD setup links due to
// AP MLD removing the corresponding affiliated APs with Multi-Link
// reconfiguration. Note that it's not valid to remove all links, in this
// case disconnect instead.
// Also note that the wdev mutex must be held.
//
extern "C" {
    pub fn cfg80211_links_removed(dev: *mut net_device, link_mask: u16);
}
//
// struct cfg80211_mlo_reconf_done_data - MLO reconfiguration data
// @buf: MLO Reconfiguration Response frame (header + body)
// @len: length of the frame data
// @driver_initiated: Indicates whether the add links request is initiated by
// driver. This is set to true when the link reconfiguration request
// initiated by driver due to AP link recommendation requests
// (Ex: BTM (BSS Transition Management) request) handling offloaded to
// driver.
// @added_links: BIT mask of links successfully added to the association
// @links: per-link information indexed by link ID
// @links.bss: the BSS that MLO reconfiguration was requested for, ownership of
// the pointer moves to cfg80211 in the call to
// cfg80211_mlo_reconf_add_done().
//
// The BSS pointer must be set for each link for which 'add' operation was
// requested in the assoc_ml_reconf callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_mlo_reconf_done_data {
    pub buf: *const u8,
    pub len: usize,
    pub driver_initiated: bool,
    pub added_links: u16,
    pub bss: *mut cfg80211_bss,
    pub addr: *mut u8,
    pub links: [}; IEEE80211_MLD_MAX_NUM_LINKS],
}

//
// cfg80211_mlo_reconf_add_done - Notify about MLO reconfiguration result
// @dev: network device.
// @data: MLO reconfiguration done data, &struct cfg80211_mlo_reconf_done_data
//
// Inform cfg80211 and the userspace that processing of ML reconfiguration
// request to add links to the association is done.
//
// cfg80211_schedule_channels_check - schedule regulatory check if needed
// @wdev: the wireless device to check
//
// In case the device supports NO_IR or DFS relaxations, schedule regulatory
// channels check, as previous concurrent operation conditions may not
// hold anymore.
//
extern "C" {
    pub fn cfg80211_schedule_channels_check(wdev: *mut wireless_dev);
}
//
// cfg80211_epcs_changed - Notify about a change in EPCS state
// @netdev: the wireless device whose EPCS state changed
// @enabled: set to true if EPCS was enabled, otherwise set to false.
//
extern "C" {
    pub fn cfg80211_epcs_changed(netdev: *mut net_device, enabled: bool);
}
//
// cfg80211_next_nan_dw_notif - Notify about the next NAN Discovery Window (DW)
// @wdev: Pointer to the wireless device structure
// @chan: DW channel (6, 44 or 149)
// @gfp: Memory allocation flags
//
// cfg80211_nan_cluster_joined - Notify about NAN cluster join
// @wdev: Pointer to the wireless device structure
// @cluster_id: Cluster ID of the NAN cluster that was joined or started
// @new_cluster: Indicates if this is a new cluster or an existing one
// @gfp: Memory allocation flags
//
// This function is used to notify user space when a NAN cluster has been
// joined, providing the cluster ID and a flag whether it is a new cluster.
//
// cfg80211_nan_ulw_update - Notify user space about ULW update
// @wdev: Pointer to the wireless device structure
// @ulw: Pointer to the ULW blob data
// @ulw_len: Length of the ULW blob in bytes
// @gfp: Memory allocation flags
//
// This function is used by drivers to notify user space when the device's
// ULW (Unaligned Schedule) blob has been updated. User space can use this
// blob to attach to frames sent to peers.
//
// cfg80211_nan_channel_evac - Notify user space about NAN channel evacuation
// @wdev: Pointer to the wireless device structure
// @chandef: Pointer to the channel definition of the NAN channel that was
// evacuated
// @gfp: Memory allocation flags
//
// This function is used by drivers to notify user space when a NAN
// channel has been evacuated (i.e. ULWed) due to channel resource conflicts
// with other interfaces.
// This can happen when another interface sharing the channel resource with NAN
// needs to move to a different channel (e.g. due to channel switch or link
// switch). User space may reconfigure the local schedule to exclude the
// evacuated channel.
//

//
// wiphy_locked_debugfs_read - do a locked read in debugfs
// @wiphy: the wiphy to use
// @file: the file being read
// @buf: the buffer to fill and then read from
// @bufsize: size of the buffer
// @userbuf: the user buffer to copy to
// @count: read count
// @ppos: read position
// @handler: the read handler to call (under wiphy lock)
// @data: additional data to pass to the read handler
//
// Return: the number of characters read, or a negative errno
//
// wiphy_locked_debugfs_write - do a locked write in debugfs
// @wiphy: the wiphy to use
// @file: the file being written to
// @buf: the buffer to copy the user data to
// @bufsize: size of the buffer
// @userbuf: the user buffer to copy from
// @count: read count
// @handler: the write handler to call (under wiphy lock)
// @data: additional data to pass to the write handler
//
// Return: the number of characters written, or a negative errno
//

//
// cfg80211_s1g_get_start_freq_khz - get S1G chandef start frequency
// @chandef: the chandef to use
//
// Return: the chandefs starting frequency in KHz
//
// cfg80211_s1g_get_end_freq_khz - get S1G chandef end frequency
// @chandef: the chandef to use
//
// Return: the chandefs ending frequency in KHz
//
// cfg80211_s1g_get_primary_sibling - retrieve the sibling 1MHz subchannel
// for an S1G chandef using a 2MHz primary channel.
// @wiphy: wiphy the channel belongs to
// @chandef: the chandef to use
//
// When chandef::s1g_primary_2mhz is set to true, we are operating on a 2MHz
// primary channel. The 1MHz subchannel designated by the primary channel
// location exists within chandef::chan, whilst the 'sibling' is denoted as
// being the other 1MHz subchannel that make up the 2MHz primary channel.
//
// Returns: the sibling 1MHz &struct ieee80211_channel, or %NULL on failure.
//
// Compute the index of the primary 1 MHz subchannel within the
// operating channel, relative to the lowest 1 MHz center frequency.
// Flip the least significant bit to select the even/odd sibling,
// then translate that index back into a channel frequency.
//
extern "C" {
    pub fn ieee80211_get_channel_khz(_arg: wiphy, _arg: sibling_1mhz_khz) -> return;
}
//
// cfg80211_incumbent_signal_notify - Notify userspace of incumbent signal detection
// @wiphy: the wiphy to use
// @chandef: channel definition in which the interference was detected
// @signal_interference_bitmap: bitmap indicating interference across 20 MHz segments
// @gfp: allocation context for message creation and multicast; pass GFP_ATOMIC
// if called from atomic context (e.g. firmware event handler), otherwise
// GFP_KERNEL
//
// Use this function to notify userspace when an incumbent signal is detected on
// the operating channel in the 6 GHz band. The notification includes the
// current channel definition and a bitmap representing interference across
// the operating bandwidth. Each bit in the bitmap corresponds to a 20 MHz
// segment, with the lowest bit representing the lowest frequency segment.
// Punctured sub-channels are included in the bitmap structure but are always
// set to zero since interference detection is not performed on them.
//
