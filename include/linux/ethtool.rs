//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ethtool.h
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


// SPDX-License-Identifier: GPL-2.0
//
// ethtool.h: Defines for Linux ethtool.
//
// Copyright (C) 1998 David S. Miller (davem@redhat.com)
// Copyright 2001 Jeff Garzik <jgarzik@pobox.com>
// Portions Copyright 2001 Sun Microsystems (thockin@sun.com)
// Portions Copyright 2002 Intel (eli.kupermann@intel.com,
// christopher.leech@intel.com,
// scott.feldman@intel.com)
// Portions Copyright (C) Sun Microsystems 2008
//

pub const ETHTOOL_MM_MAX_VERIFY_TIME_MS: c_int = 128;
pub const ETHTOOL_MM_MAX_VERIFY_RETRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ethtool_rx_flow_spec {
    pub flow_type: u32,
    pub h_u: ethtool_flow_union,
    pub h_ext: ethtool_flow_ext,
    pub m_u: ethtool_flow_union,
    pub m_ext: ethtool_flow_ext,
    pub ring_cookie: compat_u64,
    pub location: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ethtool_rxnfc {
    pub cmd: u32,
    pub flow_type: u32,
    pub data: compat_u64,
    pub fs: compat_ethtool_rx_flow_spec,
    pub rule_cnt: u32,
    pub rule_locs: [u32; ],
}

//
// enum ethtool_phys_id_state - indicator state for physical identification
// @ETHTOOL_ID_INACTIVE: Physical ID indicator should be deactivated
// @ETHTOOL_ID_ACTIVE: Physical ID indicator should be activated
// @ETHTOOL_ID_ON: LED should be turned on (used iff %ETHTOOL_ID_ACTIVE
// is not supported)
// @ETHTOOL_ID_OFF: LED should be turned off (used iff %ETHTOOL_ID_ACTIVE
// is not supported)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_phys_id_state {
    ETHTOOL_ID_INACTIVE,
    ETHTOOL_ID_ACTIVE,
    ETHTOOL_ID_ON,
    ETHTOOL_ID_OFF
}

//
// Add your fresh new hash function bits above and remember to update
// rss_hash_func_strings[] in ethtool.c
//
// struct kernel_ethtool_ringparam - RX/TX ring configuration
// @rx_buf_len: Current length of buffers on the rx ring.
// @tcp_data_split: Scatter packet headers and data to separate buffers
// @tx_push: The flag of tx push mode
// @rx_push: The flag of rx push mode
// @cqe_size: Size of TX/RX completion queue event
// @tx_push_buf_len: Size of TX push buffer
// @tx_push_buf_max_len: Maximum allowed size of TX push buffer
// @hds_thresh: Packet size threshold for header data split (HDS)
// @hds_thresh_max: Maximum supported setting for @hds_threshold
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_ethtool_ringparam {
    pub rx_buf_len: u32,
    pub tcp_data_split: u8,
    pub tx_push: u8,
    pub rx_push: u8,
    pub cqe_size: u32,
    pub tx_push_buf_len: u32,
    pub tx_push_buf_max_len: u32,
    pub hds_thresh: u32,
    pub hds_thresh_max: u32,
}

//
// enum ethtool_supported_ring_param - indicator caps for setting ring params
// @ETHTOOL_RING_USE_RX_BUF_LEN: capture for setting rx_buf_len
// @ETHTOOL_RING_USE_CQE_SIZE: capture for setting cqe_size
// @ETHTOOL_RING_USE_TX_PUSH: capture for setting tx_push
// @ETHTOOL_RING_USE_RX_PUSH: capture for setting rx_push
// @ETHTOOL_RING_USE_TX_PUSH_BUF_LEN: capture for setting tx_push_buf_len
// @ETHTOOL_RING_USE_TCP_DATA_SPLIT: capture for setting tcp_data_split
// @ETHTOOL_RING_USE_HDS_THRS: capture for setting header-data-split-thresh
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_supported_ring_param {
    ETHTOOL_RING_USE_RX_BUF_LEN		= BIT(0),
    ETHTOOL_RING_USE_CQE_SIZE		= BIT(1),
    ETHTOOL_RING_USE_TX_PUSH		= BIT(2),
    ETHTOOL_RING_USE_RX_PUSH		= BIT(3),
    ETHTOOL_RING_USE_TX_PUSH_BUF_LEN	= BIT(4),
    ETHTOOL_RING_USE_TCP_DATA_SPLIT		= BIT(5),
    ETHTOOL_RING_USE_HDS_THRS		= BIT(6),
}

pub const ETH_RSS_HASH_UNKNOWN: c_int = 0;
pub const ETH_RSS_HASH_NO_CHANGE: c_int = 0;
// Link extended state and substate.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_link_ext_state_info {
    pub link_ext_state: ethtool_link_ext_state,
    pub autoneg: ethtool_link_ext_substate_autoneg,
    pub link_training: ethtool_link_ext_substate_link_training,
    pub link_logical_mismatch: ethtool_link_ext_substate_link_logical_mismatch,
    pub bad_signal_integrity: ethtool_link_ext_substate_bad_signal_integrity,
    pub cable_issue: ethtool_link_ext_substate_cable_issue,
    pub module: ethtool_link_ext_substate_module,
    pub __link_ext_substate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_link_ext_stats {
// Custom Linux statistic for PHY level link down events.
// In a simpler world it should be equal to netdev->carrier_down_count
// unfortunately netdev also counts local reconfigurations which don't
// actually take the physical link down, not to mention NC-SI which,
// if present, keeps the link up regardless of host state.
// This statistic counts when PHY _actually_ went down, or lost link.
//
// Note that we need u64 for ethtool_stats_init() and comparisons
// to ETHTOOL_STAT_NOT_SET, but only u32 is exposed to the user.
//
    pub link_down_events: u64,
}

//
// ethtool_rxfh_indir_default - get default value for RX flow hash indirection
// @index: Index in RX flow hash indirection table
// @n_rx_rings: Number of RX rings to use
//
// This function provides the default policy for RX flow hash indirection.
//
// struct ethtool_rxfh_context - a custom RSS context configuration
// @indir_size: Number of u32 entries in indirection table
// @key_size: Size of hash key, in bytes
// @indir_user_size: number of user provided entries for the
// indirection table
// @priv_size: Size of driver private data, in bytes
// @hfunc: RSS hash function identifier.  One of the %ETH_RSS_HASH_
// @input_xfrm: Defines how the input data is transformed. Valid values are one
// of %RXH_XFRM_*.
// @indir_configured: indir has been specified (at create time or subsequently)
// @key_configured: hkey has been specified (at create time or subsequently)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rxfh_context {
    pub indir_size: u32,
    pub key_size: u32,
    pub indir_user_size: u32,
    pub priv_size: u16,
    pub hfunc: u8,
    pub input_xfrm: u8,
    pub indir_configured:1: u8,
    pub key_configured:1: u8,
// private: driver private data, indirection table, and hash key are
// stored sequentially in @data area.  Use below helpers to access.
//
    pub key_off: u32,
    pub )): *mut u8 data[] __aligned(sizeof(void,
}

extern "C" {
    pub fn ethtool_rxfh_context_lost(dev: *mut net_device, context_id: u32);
}
extern "C" {
    pub fn ethtool_rxfh_indir_lost(dev: *mut net_device);
}
extern "C" {
    pub fn ethtool_rxfh_ctxs_can_resize(dev: *mut net_device, new_indir_size: u32) -> c_int;
}
extern "C" {
    pub fn ethtool_rxfh_ctxs_resize(dev: *mut net_device, new_indir_size: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_mode_info {
    pub speed: c_int,
    pub lanes: u8,
    pub min_pairs: u8,
    pub pairs: u8,
    pub duplex: u8,
    pub mediums: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_link_medium {
    ETHTOOL_LINK_MEDIUM_BASET = 0,
    ETHTOOL_LINK_MEDIUM_BASEK,
    ETHTOOL_LINK_MEDIUM_BASES,
    ETHTOOL_LINK_MEDIUM_BASEC,
    ETHTOOL_LINK_MEDIUM_BASEL,
    ETHTOOL_LINK_MEDIUM_BASED,
    ETHTOOL_LINK_MEDIUM_BASEE,
    ETHTOOL_LINK_MEDIUM_BASEF,
    ETHTOOL_LINK_MEDIUM_BASEV,
    ETHTOOL_LINK_MEDIUM_BASEMLD,
    ETHTOOL_LINK_MEDIUM_NONE,

    __ETHTOOL_LINK_MEDIUM_LAST,
}

extern "C" {
    pub fn ethtool_str_to_medium(str: *const c_char) -> ethtool_link_medium;
}
// declare a link mode bitmap

// drivers must ignore base.cmd and base.link_mode_masks_nwords
// fields, but they are allowed to overwrite them (will be ignored).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_link_ksettings {
    pub base: ethtool_link_settings,
    pub link_modes: },
    pub lanes: u32,
}

//
// ethtool_link_ksettings_zero_link_mode - clear link_ksettings link mode mask
// @ptr : pointer to struct ethtool_link_ksettings
// @name : one of supported/advertising/lp_advertising
//

//
// ethtool_link_ksettings_add_link_mode - set bit in link_ksettings
// link mode mask
// @ptr : pointer to struct ethtool_link_ksettings
// @name : one of supported/advertising/lp_advertising
// @mode : one of the ETHTOOL_LINK_MODE_*_BIT
// (not atomic, no bound checking)
//

//
// ethtool_link_ksettings_del_link_mode - clear bit in link_ksettings
// link mode mask
// @ptr : pointer to struct ethtool_link_ksettings
// @name : one of supported/advertising/lp_advertising
// @mode : one of the ETHTOOL_LINK_MODE_*_BIT
// (not atomic, no bound checking)
//

//
// ethtool_link_ksettings_test_link_mode - test bit in ksettings link mode mask
// @ptr : pointer to struct ethtool_link_ksettings
// @name : one of supported/advertising/lp_advertising
// @mode : one of the ETHTOOL_LINK_MODE_*_BIT
// (not atomic, no bound checking)
//
// Returns: true/false.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_keee {
    pub tx_lpi_timer: u32,
    pub tx_lpi_enabled: bool,
    pub eee_active: bool,
    pub eee_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_ethtool_coalesce {
    pub use_cqe_mode_tx: u8,
    pub use_cqe_mode_rx: u8,
    pub tx_aggr_max_bytes: u32,
    pub tx_aggr_max_frames: u32,
    pub tx_aggr_time_usecs: u32,
    pub rx_cqe_frames: u32,
    pub rx_cqe_nsecs: u32,
}

//
// ethtool_intersect_link_masks - Given two link masks, AND them together
// @dst: first mask and where result is stored
// @src: second mask to intersect with
//
// Given two link mode masks, AND them together and save the result in dst.
//
// return false if src had higher bits set. lower bits always updated.

// Basic IEEE 802.3 MAC statistics (30.3.1.1.*), not otherwise exposed
// via a more targeted API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_eth_mac_stats {
    pub src: ethtool_mac_stats_src,
    pub FramesTransmittedOK: u64,
    pub SingleCollisionFrames: u64,
    pub MultipleCollisionFrames: u64,
    pub FramesReceivedOK: u64,
    pub FrameCheckSequenceErrors: u64,
    pub AlignmentErrors: u64,
    pub OctetsTransmittedOK: u64,
    pub FramesWithDeferredXmissions: u64,
    pub LateCollisions: u64,
    pub FramesAbortedDueToXSColls: u64,
    pub FramesLostDueToIntMACXmitError: u64,
    pub CarrierSenseErrors: u64,
    pub OctetsReceivedOK: u64,
    pub FramesLostDueToIntMACRcvError: u64,
    pub MulticastFramesXmittedOK: u64,
    pub BroadcastFramesXmittedOK: u64,
    pub FramesWithExcessiveDeferral: u64,
    pub MulticastFramesReceivedOK: u64,
    pub BroadcastFramesReceivedOK: u64,
    pub InRangeLengthErrors: u64,
    pub OutOfRangeLengthField: u64,
    pub FrameTooLongErrors: u64,
}

// Basic IEEE 802.3 PHY statistics (30.3.2.1.*), not otherwise exposed
// via a more targeted API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_eth_phy_stats {
    pub src: ethtool_mac_stats_src,
    pub SymbolErrorDuringCarrier: u64,
}

//
// struct ethtool_phy_stats - PHY-level statistics counters
// @rx_packets: Total successfully received frames
// @rx_bytes: Total successfully received bytes
// @rx_errors: Total received frames with errors (e.g., CRC errors)
// @tx_packets: Total successfully transmitted frames
// @tx_bytes: Total successfully transmitted bytes
// @tx_errors: Total transmitted frames with errors
//
// This structure provides a standardized interface for reporting
// PHY-level statistics counters. It is designed to expose statistics
// commonly provided by PHYs but not explicitly defined in the IEEE
// 802.3 standard.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_phy_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_errors: u64,
}

// Basic IEEE 802.3 MAC Ctrl statistics (30.3.3.*), not otherwise exposed
// via a more targeted API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_eth_ctrl_stats {
    pub src: ethtool_mac_stats_src,
    pub MACControlFramesTransmitted: u64,
    pub MACControlFramesReceived: u64,
    pub UnsupportedOpcodesReceived: u64,
}

//
// struct ethtool_pause_stats - statistics for IEEE 802.3x pause frames
// @src: input field denoting whether stats should be queried from the eMAC or
// pMAC (if the MM layer is supported). To be ignored otherwise.
// @tx_pause_frames: transmitted pause frame count. Reported to user space
// as %ETHTOOL_A_PAUSE_STAT_TX_FRAMES.
//
// Equivalent to `30.3.4.2 aPAUSEMACCtrlFramesTransmitted`
// from the standard.
//
// @rx_pause_frames: received pause frame count. Reported to user space
// as %ETHTOOL_A_PAUSE_STAT_RX_FRAMES. Equivalent to:
//
// Equivalent to `30.3.4.3 aPAUSEMACCtrlFramesReceived`
// from the standard.
// @tx_pause_storm_events: TX pause storm event count (see ethtool.yaml).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_pause_stats {
    pub src: ethtool_mac_stats_src,
    pub tx_pause_frames: u64,
    pub rx_pause_frames: u64,
    pub tx_pause_storm_events: u64,
}

pub const ETHTOOL_MAX_LANES: c_int = 8;
//
// IEEE 802.3ck/df defines 16 bins for FEC histogram plus one more for
// the end-of-list marker, total 17 items
//
pub const ETHTOOL_FEC_HIST_MAX: c_int = 17;
//
// struct ethtool_fec_hist_range - error bits range for FEC histogram
// statistics
// @low: low bound of the bin (inclusive)
// @high: high bound of the bin (inclusive)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_fec_hist_range {
    pub low: u16,
    pub high: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_fec_hist {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_fec_hist_value {
    pub sum: u64,
    pub per_lane: [u64; ETHTOOL_MAX_LANES],
    pub values: [}; ETHTOOL_FEC_HIST_MAX],
    pub ranges: *const ethtool_fec_hist_range,
    pub ranges_buf: [ethtool_fec_hist_range; ETHTOOL_FEC_HIST_MAX],
}

//
// struct ethtool_fec_stats - statistics for IEEE 802.3 FEC
// @corrected_blocks: number of received blocks corrected by FEC
// Reported to user space as %ETHTOOL_A_FEC_STAT_CORRECTED.
//
// Equivalent to `30.5.1.1.17 aFECCorrectedBlocks` from the standard.
//
// @uncorrectable_blocks: number of received blocks FEC was not able to correct
// Reported to user space as %ETHTOOL_A_FEC_STAT_UNCORR.
//
// Equivalent to `30.5.1.1.18 aFECUncorrectableBlocks` from the standard.
//
// @corrected_bits: number of bits corrected by FEC
// Similar to @corrected_blocks but counts individual bit changes,
// not entire FEC data blocks. This is a non-standard statistic.
// Reported to user space as %ETHTOOL_A_FEC_STAT_CORR_BITS.
//
// For each of the above fields, the two substructure members are:
//
// - @lanes: per-lane/PCS-instance counts as defined by the standard
// - @total: error counts for the entire port, for drivers incapable of reporting
// per-lane stats
//
// Drivers should fill in either only total or per-lane statistics, core
// will take care of adding lane values up to produce the total.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_fec_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_fec_stat {
    pub total: u64,
    pub lanes: [u64; ETHTOOL_MAX_LANES],
    pub corrected_bits: } corrected_blocks, uncorrectable_blocks,,
}

//
// struct ethtool_rmon_hist_range - byte range for histogram statistics
// @low: low bound of the bucket (inclusive)
// @high: high bound of the bucket (inclusive)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rmon_hist_range {
    pub low: u16,
    pub high: u16,
}

pub const ETHTOOL_RMON_HIST_MAX: c_int = 11;
//
// struct ethtool_rmon_stats - selected RMON (RFC 2819) statistics
// @src: input field denoting whether stats should be queried from the eMAC or
// pMAC (if the MM layer is supported). To be ignored otherwise.
// @undersize_pkts: Equivalent to `etherStatsUndersizePkts` from the RFC.
// @oversize_pkts: Equivalent to `etherStatsOversizePkts` from the RFC.
// @fragments: Equivalent to `etherStatsFragments` from the RFC.
// @jabbers: Equivalent to `etherStatsJabbers` from the RFC.
// @hist: Packet counter for packet length buckets (e.g.
// `etherStatsPkts128to255Octets` from the RFC).
// @hist_tx: Tx counters in similar form to @hist, not defined in the RFC.
//
// Selection of RMON (RFC 2819) statistics which are not exposed via different
// APIs, primarily the packet-length-based counters.
// Unfortunately different designs choose different buckets beyond
// the 1024B mark (jumbo frame teritory), so the definition of the bucket
// ranges is left to the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rmon_stats {
    pub src: ethtool_mac_stats_src,
    pub undersize_pkts: u64,
    pub oversize_pkts: u64,
    pub fragments: u64,
    pub jabbers: u64,
    pub hist: [u64; ETHTOOL_RMON_HIST_MAX],
    pub hist_tx: [u64; ETHTOOL_RMON_HIST_MAX],
}

//
// struct ethtool_ts_stats - HW timestamping statistics
// @pkts: Number of packets successfully timestamped by the hardware.
// @onestep_pkts_unconfirmed: Number of PTP packets with one-step TX
// timestamping that were sent, but for which the
// device offers no confirmation whether they made
// it onto the wire and the timestamp was inserted
// in the originTimestamp or correctionField, or
// not.
// @lost: Number of hardware timestamping requests where the timestamping
// information from the hardware never arrived for submission with
// the skb.
// @err: Number of arbitrary timestamp generation error events that the
// hardware encountered, exclusive of @lost statistics. Cases such
// as resource exhaustion, unavailability, firmware errors, and
// detected illogical timestamp values not submitted with the skb
// are inclusive to this counter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_ts_stats {
    pub pkts: u64,
    pub onestep_pkts_unconfirmed: u64,
    pub lost: u64,
    pub err: u64,
}

pub const ETH_MODULE_EEPROM_PAGE_LEN: c_int = 128;
pub const ETH_MODULE_MAX_I2C_ADDRESS: c_uint = 0x7f;
//
// struct ethtool_module_eeprom - plug-in module EEPROM read / write parameters
// @offset: When @offset is 0-127, it is used as an address to the Lower Memory
// (@page must be 0). Otherwise, it is used as an address to the
// Upper Memory.
// @length: Number of bytes to read / write.
// @page: Page number.
// @bank: Bank number, if supported by EEPROM spec.
// @i2c_address: I2C address of a page. Value less than 0x7f expected. Most
// EEPROMs use 0x50 or 0x51.
// @data: Pointer to buffer with EEPROM data of @length size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_module_eeprom {
    pub offset: u32,
    pub length: u32,
    pub page: u8,
    pub bank: u8,
    pub i2c_address: u8,
    pub data: *mut u8,
}

//
// struct ethtool_module_power_mode_params - module power mode parameters
// @policy: The power mode policy enforced by the host for the plug-in module.
// @mode: The operational power mode of the plug-in module. Should be filled by
// device drivers on get operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_module_power_mode_params {
    pub policy: ethtool_module_power_mode_policy,
    pub mode: ethtool_module_power_mode,
}

//
// struct ethtool_mm_state - 802.3 MAC merge layer state
// @verify_time:
// wait time between verification attempts in ms (according to clause
// 30.14.1.6 aMACMergeVerifyTime)
// @max_verify_time:
// maximum accepted value for the @verify_time variable in set requests
// @verify_status:
// state of the verification state machine of the MM layer (according to
// clause 30.14.1.2 aMACMergeStatusVerify)
// @tx_enabled:
// set if the MM layer is administratively enabled in the TX direction
// (according to clause 30.14.1.3 aMACMergeEnableTx)
// @tx_active:
// set if the MM layer is enabled in the TX direction, which makes FP
// possible (according to 30.14.1.5 aMACMergeStatusTx). This should be
// true if MM is enabled, and the verification status is either verified,
// or disabled.
// @pmac_enabled:
// set if the preemptible MAC is powered on and is able to receive
// preemptible packets and respond to verification frames.
// @verify_enabled:
// set if the Verify function of the MM layer (which sends SMD-V
// verification requests) is administratively enabled (regardless of
// whether it is currently in the ETHTOOL_MM_VERIFY_STATUS_DISABLED state
// or not), according to clause 30.14.1.4 aMACMergeVerifyDisableTx (but
// using positive rather than negative logic). The device should always
// respond to received SMD-V requests as long as @pmac_enabled is set.
// @tx_min_frag_size:
// the minimum size of non-final mPacket fragments that the link partner
// supports receiving, expressed in octets. Compared to the definition
// from clause 30.14.1.7 aMACMergeAddFragSize which is expressed in the
// range 0 to 3 (requiring a translation to the size in octets according
// to the formula 64 * (1 + addFragSize) - 4), a value in a continuous and
// unbounded range can be specified here.
// @rx_min_frag_size:
// the minimum size of non-final mPacket fragments that this device
// supports receiving, expressed in octets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_mm_state {
    pub verify_time: u32,
    pub max_verify_time: u32,
    pub verify_status: ethtool_mm_verify_status,
    pub tx_enabled: bool,
    pub tx_active: bool,
    pub pmac_enabled: bool,
    pub verify_enabled: bool,
    pub tx_min_frag_size: u32,
    pub rx_min_frag_size: u32,
}

//
// struct ethtool_mm_cfg - 802.3 MAC merge layer configuration
// @verify_time: see struct ethtool_mm_state
// @verify_enabled: see struct ethtool_mm_state
// @tx_enabled: see struct ethtool_mm_state
// @pmac_enabled: see struct ethtool_mm_state
// @tx_min_frag_size: see struct ethtool_mm_state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_mm_cfg {
    pub verify_time: u32,
    pub verify_enabled: bool,
    pub tx_enabled: bool,
    pub pmac_enabled: bool,
    pub tx_min_frag_size: u32,
}

//
// struct ethtool_mm_stats - 802.3 MAC merge layer statistics
// @MACMergeFrameAssErrorCount:
// received MAC frames with reassembly errors
// @MACMergeFrameSmdErrorCount:
// received MAC frames/fragments rejected due to unknown or incorrect SMD
// @MACMergeFrameAssOkCount:
// received MAC frames that were successfully reassembled and passed up
// @MACMergeFragCountRx:
// number of additional correct SMD-C mPackets received due to preemption
// @MACMergeFragCountTx:
// number of additional mPackets sent due to preemption
// @MACMergeHoldCount:
// number of times the MM layer entered the HOLD state, which blocks
// transmission of preemptible traffic
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_mm_stats {
    pub MACMergeFrameAssErrorCount: u64,
    pub MACMergeFrameSmdErrorCount: u64,
    pub MACMergeFrameAssOkCount: u64,
    pub MACMergeFragCountRx: u64,
    pub MACMergeFragCountTx: u64,
    pub MACMergeHoldCount: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_mmsv_event {
    ETHTOOL_MMSV_LP_SENT_VERIFY_MPACKET,
    ETHTOOL_MMSV_LD_SENT_VERIFY_MPACKET,
    ETHTOOL_MMSV_LP_SENT_RESPONSE_MPACKET,
}

// MAC Merge verification mPacket type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_mpacket {
    ETHTOOL_MPACKET_VERIFY,
    ETHTOOL_MPACKET_RESPONSE,
}

//
// struct ethtool_mmsv_ops - Operations for MAC Merge Software Verification
// @configure_tx: Driver callback for the event where the preemptible TX
// becomes active or inactive. Preemptible traffic
// classes must be committed to hardware only while
// preemptible TX is active.
// @configure_pmac: Driver callback for the event where the pMAC state
// changes as result of an administrative setting
// (ethtool) or a call to ethtool_mmsv_link_state_handle().
// @send_mpacket: Driver-provided method for sending a Verify or a Response
// mPacket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_mmsv_ops {
    pub tx_active): *mut *mut *mut void (configure_tx)(struct ethtool_mmsv mmsv, bool,
    pub pmac_enabled): *mut *mut *mut void (configure_pmac)(struct ethtool_mmsv mmsv, bool,
    pub mpacket): *mut *mut *mut void (send_mpacket)(struct ethtool_mmsv mmsv, enum ethtool_mpacket,
}

//
// struct ethtool_mmsv - MAC Merge Software Verification
// @ops: operations for MAC Merge Software Verification
// @dev: pointer to net_device structure
// @lock: serialize access to MAC Merge state between
// ethtool requests and link state updates.
// @status: current verification FSM state
// @verify_timer: timer for verification in local TX direction
// @verify_enabled: indicates if verification is enabled
// @verify_retries: number of retries for verification
// @pmac_enabled: indicates if the preemptible MAC is enabled
// @verify_time: time for verification in milliseconds
// @tx_enabled: indicates if transmission is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_mmsv {
    pub ops: *const ethtool_mmsv_ops,
    pub dev: *mut net_device,
    pub lock: spinlock_t,
    pub status: ethtool_mm_verify_status,
    pub verify_timer: timer_list,
    pub verify_enabled: bool,
    pub verify_retries: c_int,
    pub pmac_enabled: bool,
    pub verify_time: u32,
    pub tx_enabled: bool,
}

extern "C" {
    pub fn ethtool_mmsv_stop(mmsv: *mut ethtool_mmsv);
}
extern "C" {
    pub fn ethtool_mmsv_link_state_handle(mmsv: *mut ethtool_mmsv, up: bool);
}
extern "C" {
    pub fn ethtool_mmsv_set_mm(mmsv: *mut ethtool_mmsv, cfg: *mut ethtool_mm_cfg);
}
//
// struct ethtool_rxfh_param - RXFH (RSS) parameters
// @hfunc: Defines the current RSS hash function used by HW (or to be set to).
// Valid values are one of the %ETH_RSS_HASH_*.
// @indir_size: On SET, the array size of the user buffer for the
// indirection table, which may be zero, or
// %ETH_RXFH_INDIR_NO_CHANGE.  On GET (read from the driver),
// the array size of the hardware indirection table.
// @indir: The indirection table of size @indir_size entries.
// @key_size: On SET, the array size of the user buffer for the hash key,
// which may be zero.  On GET (read from the driver), the size of the
// hardware hash key.
// @key: The hash key of size @key_size bytes.
// @rss_context: RSS context identifier.  Context 0 is the default for normal
// traffic; other contexts can be referenced as the destination for RX flow
// classification rules.  On SET, %ETH_RXFH_CONTEXT_ALLOC is used
// to allocate a new RSS context; on return this field will
// contain the ID of the newly allocated context.
// @rss_delete: Set to non-ZERO to remove the @rss_context context.
// @input_xfrm: Defines how the input data is transformed. Valid values are one
// of %RXH_XFRM_*.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rxfh_param {
    pub hfunc: u8,
    pub indir_size: u32,
    pub indir: *mut u32,
    pub key_size: u32,
    pub key: *mut u8,
    pub rss_context: u32,
    pub rss_delete: u8,
    pub input_xfrm: u8,
}

//
// struct ethtool_rxfh_fields - Rx Flow Hashing (RXFH) header field config
// @data: which header fields are used for hashing, bitmask of RXH_* defines
// @flow_type: L2-L4 network traffic flow type
// @rss_context: RSS context, will only be used if rxfh_per_ctx_fields is
// set in struct ethtool_ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rxfh_fields {
    pub data: u32,
    pub flow_type: u32,
    pub rss_context: u32,
}

//
// struct kernel_ethtool_ts_info - kernel copy of struct ethtool_ts_info
// @cmd: command number = %ETHTOOL_GET_TS_INFO
// @so_timestamping: bit mask of the sum of the supported SO_TIMESTAMPING flags
// @phc_index: device index of the associated PHC, or -1 if there is none
// @phc_qualifier: qualifier of the associated PHC
// @phc_source: source device of the associated PHC
// @phc_phyindex: index of PHY device source of the associated PHC
// @tx_types: bit mask of the supported hwtstamp_tx_types enumeration values
// @rx_filters: bit mask of the supported hwtstamp_rx_filters enumeration values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_ethtool_ts_info {
    pub cmd: u32,
    pub so_timestamping: u32,
    pub phc_index: c_int,
    pub phc_qualifier: hwtstamp_provider_qualifier,
    pub phc_source: hwtstamp_source,
    pub phc_phyindex: c_int,
    pub tx_types: u32,
    pub rx_filters: u32,
}

// Bits for ethtool_ops::op_needs_rtnl
// LINKSETTINGS cover a number of commands, but in most cases we want to keep
// these bits separate, per GET and SET. GET is much easier to "unlock".
//

//
// struct ethtool_ops - optional netdev operations
// @supported_input_xfrm: supported types of input xfrm from %RXH_XFRM_*.
// @cap_link_lanes_supported: indicates if the driver supports lanes
// parameter.
// @rxfh_per_ctx_fields: device supports selecting different header fields
// for Rx hash calculation and RSS for each additional context.
// @rxfh_per_ctx_key: device supports setting different RSS key for each
// additional context. Netlink API should report hfunc, key, and input_xfrm
// for every context, not just context 0.
// @cap_rss_rxnfc_adds: device supports nonzero ring_cookie in filters with
// %FLOW_RSS flag; the queue ID from the filter is added to the value from
// the indirection table to determine the delivery queue.
// @rxfh_indir_space: max size of RSS indirection tables, if indirection table
// size as returned by @get_rxfh_indir_size may change during lifetime
// of the device. Leave as 0 if the table size is constant.
// @rxfh_key_space: same as @rxfh_indir_space, but for the key.
// @rxfh_priv_size: size of the driver private data area the core should
// allocate for an RSS context (in &struct ethtool_rxfh_context).
// @rxfh_max_num_contexts: maximum (exclusive) supported RSS context ID.
// If this is zero then the core may choose any (nonzero) ID, otherwise
// the core will only use IDs strictly less than this value, as the
// @rss_context argument to @create_rxfh_context and friends.
// @supported_coalesce_params: supported types of interrupt coalescing.
// @supported_ring_params: supported ring params.
// @supported_hwtstamp_qualifiers: bitfield of supported hwtstamp qualifier.
// @op_needs_rtnl: mask of %ETHTOOL_OP_NEEDS_RTNL_* bits.
// For use with ops-locked drivers (ignored otherwise). Selects which
// ethtool callbacks driver needs to still be executed under rtnl_lock
// (in addition to the netdev instance lock).
// The following commonly used core APIs currently require rtnl_lock
// (this list may not be exhaustive):
// - phylink helpers (note that phydev is currently unsupported!)
// - netdev_update_features()
// - netif_set_real_num_tx_queues()
// - ethtool_op_get_link() (syncs link watch under rtnl_lock)
//
// @get_drvinfo: Report driver/device information. Modern drivers no
// longer have to implement this callback. Most fields are
// correctly filled in by the core using system information, or
// populated using other driver operations.
// @get_regs_len: Get buffer length required for @get_regs
// @get_regs: Get device registers
// @get_wol: Report whether Wake-on-Lan is enabled
// @set_wol: Turn Wake-on-Lan on or off.  Returns a negative error code
// or zero.
// @get_msglevel: Report driver message level.  This should be the value
// of the @msg_enable field used by netif logging functions.
// @set_msglevel: Set driver message level
// @nway_reset: Restart autonegotiation.  Returns a negative error code
// or zero.
// @get_link: Report whether physical link is up.  Will only be called if
// the netdev is up.  Should usually be set to ethtool_op_get_link(),
// which uses netif_carrier_ok().
// @get_link_ext_state: Report link extended state. Should set link_ext_state and
// link_ext_substate (link_ext_substate of 0 means link_ext_substate is unknown,
// do not attach ext_substate attribute to netlink message). If link_ext_state
// and link_ext_substate are unknown, return -ENODATA. If not implemented,
// link_ext_state and link_ext_substate will not be sent to userspace.
// @get_link_ext_stats: Read extra link-related counters.
// @get_eeprom_len: Read range of EEPROM addresses for validation of
// @get_eeprom and @set_eeprom requests.
// Returns 0 if device does not support EEPROM access.
// @get_eeprom: Read data from the device EEPROM.
// Should fill in the magic field.  Don't need to check len for zero
// or wraparound.  Fill in the data argument with the eeprom values
// from offset to offset + len.  Update len to the amount read.
// Returns an error or zero.
// @set_eeprom: Write data to the device EEPROM.
// Should validate the magic field.  Don't need to check len for zero
// or wraparound.  Update len to the amount written.  Returns an error
// or zero.
// @get_coalesce: Get interrupt coalescing parameters.  Returns a negative
// error code or zero.
// @set_coalesce: Set interrupt coalescing parameters.  Supported coalescing
// types should be set in @supported_coalesce_params.
// Returns a negative error code or zero.
// @get_ringparam: Report ring sizes
// @set_ringparam: Set ring sizes.  Returns a negative error code or zero.
// @get_pause_stats: Report pause frame statistics. Drivers must not zero
// statistics which they don't report. The stats structure is initialized
// to ETHTOOL_STAT_NOT_SET indicating driver does not report statistics.
// @get_pauseparam: Report pause parameters
// @set_pauseparam: Set pause parameters.  Returns a negative error code
// or zero.
// @self_test: Run specified self-tests
// @get_strings: Return a set of strings that describe the requested objects
// @set_phys_id: Identify the physical devices, e.g. by flashing an LED
// attached to it.  The implementation may update the indicator
// asynchronously or synchronously, but in either case it must return
// quickly.  It is initially called with the argument %ETHTOOL_ID_ACTIVE,
// and must either activate asynchronous updates and return zero, return
// a negative error or return a positive frequency for synchronous
// indication (e.g. 1 for one on/off cycle per second).  If it returns
// a frequency then it will be called again at intervals with the
// argument %ETHTOOL_ID_ON or %ETHTOOL_ID_OFF and should set the state of
// the indicator accordingly.  Finally, it is called with the argument
// %ETHTOOL_ID_INACTIVE and must deactivate the indicator.  Returns a
// negative error code or zero.
// @get_ethtool_stats: Return extended statistics about the device.
// This is only useful if the device maintains statistics not
// included in &struct rtnl_link_stats64.
// @begin: Function to be called before any other operation.  Returns a
// negative error code or zero.
// @complete: Function to be called after any other operation except
// @begin.  Will be called even if the other operation failed.
// @get_priv_flags: Report driver-specific feature flags.
// @set_priv_flags: Set driver-specific feature flags.  Returns a negative
// error code or zero.
// @get_sset_count: Get number of strings that @get_strings will write.
// @get_rxnfc: Get RX flow classification rules.  Returns a negative
// error code or zero.
// @set_rxnfc: Set RX flow classification rules.  Returns a negative
// error code or zero.
// @flash_device: Write a firmware image to device's flash memory.
// Returns a negative error code or zero.
// @reset: Reset (part of) the device, as specified by a bitmask of
// flags from &enum ethtool_reset_flags.  Returns a negative
// error code or zero.
// @get_rx_ring_count: Return the number of RX rings
// @get_rxfh_key_size: Get the size of the RX flow hash key.
// Returns zero if not supported for this specific device.
// @get_rxfh_indir_size: Get the size of the RX flow hash indirection table.
// Returns zero if not supported for this specific device.
// @get_rxfh: Get the contents of the RX flow hash indirection table, hash key
// and/or hash function.
// Returns a negative error code or zero.
// @set_rxfh: Set the contents of the RX flow hash indirection table, hash
// key, and/or hash function.  Arguments which are set to %NULL or zero
// will remain unchanged.
// Returns a negative error code or zero. An error code must be returned
// if at least one unsupported change was requested.
// @get_rxfh_fields: Get header fields used for flow hashing.
// @set_rxfh_fields: Set header fields used for flow hashing.
// @create_rxfh_context: Create a new RSS context with the specified RX flow
// hash indirection table, hash key, and hash function.
// The &struct ethtool_rxfh_context for this context is passed in @ctx;
// note that the indir table, hkey and hfunc are not yet populated as
// of this call.  The driver does not need to update these; the core
// will do so if this op succeeds.
// However, if @rxfh.indir is set to %NULL, the driver must update the
// indir table in @ctx with the (default or inherited) table actually in
// use; similarly, if @rxfh.key is %NULL, @rxfh.hfunc is
// %ETH_RSS_HASH_NO_CHANGE, or @rxfh.input_xfrm is %RXH_XFRM_NO_CHANGE,
// the driver should update the corresponding information in @ctx.
// If the driver provides this method, it must also provide
// @modify_rxfh_context and @remove_rxfh_context.
// Returns a negative error code or zero.
// @modify_rxfh_context: Reconfigure the specified RSS context.  Allows setting
// the contents of the RX flow hash indirection table, hash key, and/or
// hash function associated with the given context.
// Parameters which are set to %NULL or zero will remain unchanged.
// The &struct ethtool_rxfh_context for this context is passed in @ctx;
// note that it will still contain the *old* settings.  The driver does
// not need to update these; the core will do so if this op succeeds.
// Returns a negative error code or zero. An error code must be returned
// if at least one unsupported change was requested.
// @remove_rxfh_context: Remove the specified RSS context.
// The &struct ethtool_rxfh_context for this context is passed in @ctx.
// Returns a negative error code or zero.
// @get_channels: Get number of channels.
// @set_channels: Set number of channels.  Returns a negative error code or
// zero.
// @get_dump_flag: Get dump flag indicating current dump length, version,
// and flag of the device.
// @get_dump_data: Get dump data.
// @set_dump: Set dump specific flags to the device.
// @get_ts_info: Get the time stamping and PTP hardware clock capabilities.
// It may be called with RCU, or rtnl or reference on the device.
// Drivers supporting transmit time stamps in software should set this to
// ethtool_op_get_ts_info().
// @get_ts_stats: Query the device hardware timestamping statistics. Drivers
// must not zero statistics which they don't report. The stats structure
// is initialized to ETHTOOL_STAT_NOT_SET indicating driver does not
// report statistics.
// @get_module_info: Get the size and type of the eeprom contained within
// a plug-in module.
// @get_module_eeprom: Get the eeprom information from the plug-in module
// @get_eee: Get Energy-Efficient (EEE) supported and status.
// @set_eee: Set EEE status (enable/disable) as well as LPI timers.
// @get_tunable: Read the value of a driver / device tunable.
// @set_tunable: Set the value of a driver / device tunable.
// @get_per_queue_coalesce: Get interrupt coalescing parameters per queue.
// It must check that the given queue number is valid. If neither a RX nor
// a TX queue has this number, return -EINVAL. If only a RX queue or a TX
// queue has this number, set the inapplicable fields to ~0 and return 0.
// Returns a negative error code or zero.
// @set_per_queue_coalesce: Set interrupt coalescing parameters per queue.
// It must check that the given queue number is valid. If neither a RX nor
// a TX queue has this number, return -EINVAL. If only a RX queue or a TX
// queue has this number, ignore the inapplicable fields. Supported
// coalescing types should be set in @supported_coalesce_params.
// Returns a negative error code or zero.
// @get_link_ksettings: Get various device settings including Ethernet link
// settings. The %cmd and %link_mode_masks_nwords fields should be
// ignored (use %__ETHTOOL_LINK_MODE_MASK_NBITS instead of the latter),
// any change to them will be overwritten by kernel. Returns a negative
// error code or zero.
// @set_link_ksettings: Set various device settings including Ethernet link
// settings. The %cmd and %link_mode_masks_nwords fields should be
// ignored (use %__ETHTOOL_LINK_MODE_MASK_NBITS instead of the latter),
// any change to them will be overwritten by kernel. Returns a negative
// error code or zero.
// @get_fec_stats: Report FEC statistics.
// Core will sum up per-lane stats to get the total.
// Drivers must not zero statistics which they don't report. The stats
// structure is initialized to ETHTOOL_STAT_NOT_SET indicating driver does
// not report statistics.
// @get_fecparam: Get the network device Forward Error Correction parameters.
// @set_fecparam: Set the network device Forward Error Correction parameters.
// @get_ethtool_phy_stats: Return extended statistics about the PHY device.
// This is only useful if the device maintains PHY statistics and
// cannot use the standard PHY library helpers.
// @get_phy_tunable: Read the value of a PHY tunable.
// @set_phy_tunable: Set the value of a PHY tunable.
// @get_module_eeprom_by_page: Get a region of plug-in module EEPROM data from
// specified page. Returns a negative error code or the amount of bytes
// read.
// @set_module_eeprom_by_page: Write to a region of plug-in module EEPROM,
// from kernel space only. Returns a negative error code or zero.
// @get_eth_phy_stats: Query some of the IEEE 802.3 PHY statistics.
// @get_eth_mac_stats: Query some of the IEEE 802.3 MAC statistics.
// @get_eth_ctrl_stats: Query some of the IEEE 802.3 MAC Ctrl statistics.
// @get_rmon_stats: Query some of the RMON (RFC 2819) statistics.
// Set %ranges to a pointer to zero-terminated array of byte ranges.
// @get_module_power_mode: Get the power mode policy for the plug-in module
// used by the network device and its operational power mode, if
// plugged-in.
// @set_module_power_mode: Set the power mode policy for the plug-in module
// used by the network device.
// @get_mm: Query the 802.3 MAC Merge layer state.
// @set_mm: Set the 802.3 MAC Merge layer parameters.
// @get_mm_stats: Query the 802.3 MAC Merge layer statistics.
//
// All operations are optional (i.e. the function pointer may be set
// to %NULL) and callers must take this into account.
//
// For traditional drivers callers hold ``rtnl_lock`` across the call.
// For "ops locked" drivers (see Documentation/networking/netdevices.rst)
// callers instead hold the netdev instance lock (``netdev_lock_ops``);
// ``rtnl_lock`` is additionally held only for callbacks for which
// the driver opts in via the matching ``ETHTOOL_OP_NEEDS_RTNL_*`` bit
// in @op_needs_rtnl.
//
// See the structures used by these operations for further documentation.
// Note that for all operations using a structure ending with a zero-
// length array, the array is allocated separately in the kernel and
// is passed to the driver as an additional parameter.
//
// See &struct net_device and &struct net_device_ops for documentation
// of the generic netdev features interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_ops {
    pub supported_input_xfrm:8: u32,
    pub cap_link_lanes_supported:1: u32,
    pub rxfh_per_ctx_fields:1: u32,
    pub rxfh_per_ctx_key:1: u32,
    pub cap_rss_rxnfc_adds:1: u32,
    pub rxfh_indir_space: u32,
    pub rxfh_key_space: u16,
    pub rxfh_priv_size: u16,
    pub rxfh_max_num_contexts: u32,
    pub supported_coalesce_params: u32,
    pub supported_ring_params: u32,
    pub supported_hwtstamp_qualifiers: u32,
    pub op_needs_rtnl: u32,
    pub ): *mut *mut *mut void (get_drvinfo)(struct net_device , struct ethtool_drvinfo,
    pub ): *mut *mut int (get_regs_len)(struct net_device,
    pub ): *mut *mut *mut *mut void (get_regs)(struct net_device , struct ethtool_regs , void,
    pub ): *mut *mut *mut void (get_wol)(struct net_device , struct ethtool_wolinfo,
    pub ): *mut *mut *mut int (set_wol)(struct net_device , struct ethtool_wolinfo,
    pub ): *mut *mut u32 (get_msglevel)(struct net_device,
    pub u32): *mut *mut *mut void (set_msglevel)(struct net_device ,,
    pub ): *mut *mut int (nway_reset)(struct net_device,
    pub ): *mut *mut u32 (get_link)(struct net_device,
    pub ): *mut ethtool_link_ext_state_info,
    pub stats): *mut ethtool_link_ext_stats,
    pub ): *mut *mut int (get_eeprom_len)(struct net_device,
    pub ): *mut *mut ethtool_eeprom , u8,
    pub ): *mut *mut ethtool_eeprom , u8,
    pub ): *mut netlink_ext_ack,
    pub ): *mut netlink_ext_ack,
    pub ): *mut netlink_ext_ack,
    pub ): *mut netlink_ext_ack,
    pub pause_stats): *mut ethtool_pause_stats,
    pub ethtool_pauseparam*): *mut struct,
    pub ethtool_pauseparam*): *mut struct,
    pub ): *mut *mut *mut *mut void (self_test)(struct net_device , struct ethtool_test , u64,
    pub ): *mut *mut *mut void (get_strings)(struct net_device , u32 stringset, u8,
    pub ethtool_phys_id_state): *mut *mut *mut int (set_phys_id)(struct net_device , enum,
    pub ): *mut *mut ethtool_stats , u64,
    pub ): *mut *mut int (begin)(struct net_device,
    pub ): *mut *mut void (complete)(struct net_device,
    pub ): *mut *mut u32 (get_priv_flags)(struct net_device,
    pub u32): *mut *mut *mut int (set_priv_flags)(struct net_device ,,
    pub int): *mut *mut *mut int (get_sset_count)(struct net_device ,,
    pub rule_locs): *mut *mut ethtool_rxnfc , u32,
    pub ): *mut *mut *mut int (set_rxnfc)(struct net_device , struct ethtool_rxnfc,
    pub ): *mut *mut *mut int (flash_device)(struct net_device , struct ethtool_flash,
    pub ): *mut *mut *mut int (reset)(struct net_device , u32,
    pub dev): *mut *mut u32 (get_rx_ring_count)(struct net_device,
    pub ): *mut *mut u32 (get_rxfh_key_size)(struct net_device,
    pub ): *mut *mut u32 (get_rxfh_indir_size)(struct net_device,
    pub ): *mut *mut *mut int (get_rxfh)(struct net_device , struct ethtool_rxfh_param,
    pub extack): *mut netlink_ext_ack,
    pub ): *mut ethtool_rxfh_fields,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub ): *mut *mut *mut void (get_channels)(struct net_device , struct ethtool_channels,
    pub ): *mut *mut *mut int (set_channels)(struct net_device , struct ethtool_channels,
    pub ): *mut *mut *mut int (get_dump_flag)(struct net_device , struct ethtool_dump,
    pub ): *mut *mut ethtool_dump , void,
    pub ): *mut *mut *mut int (set_dump)(struct net_device , struct ethtool_dump,
    pub ): *mut *mut *mut int (get_ts_info)(struct net_device , struct kernel_ethtool_ts_info,
    pub ts_stats): *mut ethtool_ts_stats,
    pub ): *mut ethtool_modinfo,
    pub ): *mut *mut ethtool_eeprom , u8,
    pub eee): *mut *mut *mut int (get_eee)(struct net_device dev, struct ethtool_keee,
    pub eee): *mut *mut *mut int (set_eee)(struct net_device dev, struct ethtool_keee,
    pub ): *const *const ethtool_tunable , void,
    pub ): *const *const ethtool_tunable , void,
    pub ): *mut ethtool_coalesce,
    pub ): *mut ethtool_coalesce,
    pub ): *mut ethtool_link_ksettings,
    pub ): *const ethtool_link_ksettings,
    pub hist): *mut ethtool_fec_hist,
    pub ): *mut ethtool_fecparam,
    pub ): *mut ethtool_fecparam,
    pub ): *mut *mut ethtool_stats , u64,
    pub ): *const *const ethtool_tunable , void,
    pub ): *const *const ethtool_tunable , void,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub phy_stats): *mut ethtool_eth_phy_stats,
    pub mac_stats): *mut ethtool_eth_mac_stats,
    pub ctrl_stats): *mut ethtool_eth_ctrl_stats,
    pub ranges): *const ethtool_rmon_hist_range,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub state): *mut *mut *mut int (get_mm)(struct net_device dev, struct ethtool_mm_state,
    pub extack): *mut netlink_ext_ack,
    pub stats): *mut *mut *mut void (get_mm_stats)(struct net_device dev, struct ethtool_mm_stats,
}

extern "C" {
    pub fn ethtool_check_ops(ops: *const ethtool_ops) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rx_flow_rule {
    pub rule: *mut flow_rule,
    pub priv: [c_ulong; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rx_flow_spec_input {
    pub fs: *const ethtool_rx_flow_spec,
    pub rss_ctx: u32,
}

extern "C" {
    pub fn ethtool_rx_flow_rule_destroy(rule: *mut ethtool_rx_flow_rule);
}
extern "C" {
    pub fn ethtool_virtdev_validate_cmd(cmd: *const ethtool_link_ksettings) -> bool;
}
//
// struct ethtool_netdev_state - per-netdevice state for ethtool features
// @rss_ctx:		XArray of custom RSS contexts
// @rss_lock:		Protects entries in @rss_ctx.  May be taken from
// within RTNL.
// @rss_indir_user_size: Number of user provided entries for the default
// (context 0) indirection table.
// @phys_id_busy:	Loop blinking the device LED is running.
// @wol_enabled:	Wake-on-LAN is enabled
// @module_fw_flash_in_progress: Module firmware flashing is in progress.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_netdev_state {
    pub rss_ctx: xarray,
    pub rss_lock: mutex,
    pub rss_indir_user_size: u32,
    pub phys_id_busy:1: unsigned,
    pub wol_enabled:1: unsigned,
    pub module_fw_flash_in_progress:1: unsigned,
}

//
// struct ethtool_phy_ops - Optional PHY device options
// @get_sset_count: Get number of strings that @get_strings will write.
// @get_strings: Return a set of strings that describe the requested objects
// @get_stats: Return extended statistics about the PHY device.
// @get_plca_cfg: Return PLCA configuration.
// @set_plca_cfg: Set PLCA configuration.
// @get_plca_status: Get PLCA configuration.
// @start_cable_test: Start a cable test
// @start_cable_test_tdr: Start a Time Domain Reflectometry cable test
//
// All operations are optional (i.e. the function pointer may be set to %NULL)
// and callers must take this into account. Callers must hold the RTNL lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_phy_ops {
    pub dev): *mut *mut int (get_sset_count)(struct phy_device,
    pub data): *mut *mut *mut int (get_strings)(struct phy_device dev, u8,
    pub data): *mut *mut ethtool_stats stats, u64,
    pub plca_cfg): *mut phy_plca_cfg,
    pub extack): *mut netlink_ext_ack,
    pub plca_st): *mut phy_plca_status,
    pub extack): *mut netlink_ext_ack,
    pub config): *const phy_tdr_config,
}

//
// ethtool_set_ethtool_phy_ops - Set the ethtool_phy_ops singleton
// @ops: Ethtool PHY operations to set
//
extern "C" {
    pub fn ethtool_set_ethtool_phy_ops(ops: *const ethtool_phy_ops);
}
//
// ethtool_params_from_link_mode - Derive link parameters from a given link mode
// @link_ksettings: Link parameters to be derived from the link mode
// @link_mode: Link mode
//
// ethtool_get_phc_vclocks - Derive phc vclocks information, and caller
// is responsible to free memory of vclock_index
// @dev: pointer to net_device structure
// @vclock_index: pointer to pointer of vclock index
//
// Return: number of phc vclocks
//
extern "C" {
    pub fn ethtool_get_phc_vclocks(dev: *mut net_device, vclock_index: *mut c_int) -> c_int;
}
// Some generic methods drivers may use in their ethtool_ops
extern "C" {
    pub fn ethtool_op_get_link(dev: *mut net_device) -> u32;
}
//
// ethtool_mm_frag_size_add_to_min - Translate (standard) additional fragment
// size expressed as multiplier into (absolute) minimum fragment size
// value expressed in octets
// @val_add: Value of addFragSize multiplier
//
// ethtool_mm_frag_size_min_to_add - Translate (absolute) minimum fragment size
// expressed in octets into (standard) additional fragment size expressed
// as multiplier
// @val_min: Value of addFragSize variable in octets
// @val_add: Pointer where the standard addFragSize value is to be returned
// @extack: Netlink extended ack
//
// Translate a value in octets to one of 0, 1, 2, 3 according to the reverse
// application of the 802.3 formula 64 * (1 + addFragSize) - 4. To be called
// by drivers which do not support programming the minimum fragment size to a
// continuous range. Returns error on other fragment length values.
//
// val_add = add_frag_size;
//
// ethtool_get_ts_info_by_layer - Obtains time stamping capabilities from the MAC or PHY layer.
// @dev: pointer to net_device structure
// @info: buffer to hold the result
// Returns: zero on success, non-zero otherwise.
//
// ethtool_sprintf - Write formatted string to ethtool string data
// @data: Pointer to a pointer to the start of string to update
// @fmt: Format of string to write
//
// Write formatted string to *data. Update *data to point at start of
// next string.
//
extern "C" {
    pub fn __printf(_arg: 2, data: *mut 3) void ethtool_sprintf(u8, fmt: *const c_char, ...) -> extern;
}
//
// ethtool_puts - Write string to ethtool string data
// @data: Pointer to a pointer to the start of string to update
// @str: String to write
//
// Write string to *data without a trailing newline. Update *data
// to point at start of next string.
//
// Prefer this function to ethtool_sprintf() when given only
// two arguments or if @fmt is just "%s".
//
extern "C" {
    pub fn ethtool_puts(data: *mut u8, str: *const c_char);
}
//
// ethtool_cpy - Write possibly-not-NUL-terminated string to ethtool string data
// @data: Pointer to a pointer to the start of string to write into
// @str: NUL-byte padded char array of size ETH_GSTRING_LEN to copy from
//

// (data) += ETH_GSTRING_LEN;				\
// Link mode to forced speed capabilities maps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_forced_speed_map {
    pub speed: u32,
    pub cap_arr: *const u32,
    pub arr_size: u32,
}

