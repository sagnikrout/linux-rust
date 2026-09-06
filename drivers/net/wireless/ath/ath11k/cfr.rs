//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/cfr.h
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
// Copyright (c) 2020-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH11K_CFR_NUM_RESP_PER_EVENT: c_int = 1;
pub const ATH11K_CFR_EVENT_TIMEOUT_MS: c_int = 1;
pub const ATH11K_CFR_NUM_RING_ENTRIES: c_int = 1;
pub const ATH11K_MAX_CFR_ENABLED_CLIENTS: c_int = 10;
pub const CFR_MAX_LUT_ENTRIES: c_int = 136;
pub const HOST_MAX_CHAINS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_correlate_event_type {
    ATH11K_CORRELATE_DBR_EVENT,
    ATH11K_CORRELATE_TX_EVENT,
}

pub const ATH11K_CFR_START_MAGIC: c_uint = 0xDEADBEAF;
pub const ATH11K_CFR_END_MAGIC: c_uint = 0xBEAFDEAD;
pub const VENDOR_QCA: c_uint = 0x8cfdf0;
pub const PLATFORM_TYPE_ARM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_meta_version {
    ATH11K_CFR_META_VERSION_NONE,
    ATH11K_CFR_META_VERSION_1,
    ATH11K_CFR_META_VERSION_2,
    ATH11K_CFR_META_VERSION_3,
    ATH11K_CFR_META_VERSION_4,
    ATH11K_CFR_META_VERSION_MAX = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_data_version {
    ATH11K_CFR_DATA_VERSION_NONE,
    ATH11K_CFR_DATA_VERSION_1,
    ATH11K_CFR_DATA_VERSION_MAX = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_capture_ack_mode {
    ATH11K_CFR_CAPTURE_LEGACY_ACK,
    ATH11K_CFR_CAPTURE_DUP_LEGACY_ACK,
    ATH11K_CFR_CAPTURE_HT_ACK,
    ATH11K_CFR_CAPTURE_VHT_ACK,

// Always keep this at last
    ATH11K_CFR_CAPTURE_INVALID_ACK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_correlate_status {
    ATH11K_CORRELATE_STATUS_RELEASE,
    ATH11K_CORRELATE_STATUS_HOLD,
    ATH11K_CORRELATE_STATUS_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_preamble_type {
    ATH11K_CFR_PREAMBLE_TYPE_LEGACY,
    ATH11K_CFR_PREAMBLE_TYPE_HT,
    ATH11K_CFR_PREAMBLE_TYPE_VHT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_cfr_peer_tx_param {
    pub capture_method: u32,
    pub vdev_id: u32,
    pub peer_mac_addr: [u8; ETH_ALEN],
    pub primary_20mhz_chan: u32,
    pub bandwidth: u32,
    pub phy_mode: u32,
    pub band_center_freq1: u32,
    pub band_center_freq2: u32,
    pub spatial_streams: u32,
    pub correlation_info_1: u32,
    pub correlation_info_2: u32,
    pub status: u32,
    pub timestamp_us: u32,
    pub counter: u32,
    pub chain_rssi: [u32; WMI_MAX_CHAINS],
    pub chain_phase: [u16; WMI_MAX_CHAINS],
    pub cfo_measurement: u32,
    pub agc_gain: [u8; HOST_MAX_CHAINS],
    pub rx_start_ts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfr_metadata {
    pub peer_addr: [u8; ETH_ALEN],
    pub status: u8,
    pub capture_bw: u8,
    pub channel_bw: u8,
    pub phy_mode: u8,
    pub prim20_chan: u16,
    pub center_freq1: u16,
    pub center_freq2: u16,
    pub capture_mode: u8,
    pub capture_type: u8,
    pub sts_count: u8,
    pub num_rx_chain: u8,
    pub timestamp: u32,
    pub length: u32,
    pub chain_rssi: [u32; HOST_MAX_CHAINS],
    pub chain_phase: [u16; HOST_MAX_CHAINS],
    pub cfo_measurement: u32,
    pub agc_gain: [u8; HOST_MAX_CHAINS],
    pub rx_start_ts: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_csi_cfr_header {
    pub start_magic_num: u32,
    pub vendorid: u32,
    pub cfr_metadata_version: u8,
    pub cfr_data_version: u8,
    pub chip_type: u8,
    pub platform_type: u8,
    pub cfr_metadata_len: u32,
    pub meta_data: cfr_metadata,
    pub __packed: },
pub const TONES_IN_20MHZ: c_int = 256;
pub const TONES_IN_40MHZ: c_int = 512;
pub const TONES_IN_80MHZ: c_int = 1024;

pub const TONES_INVALID: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_cfr_dma_hdr {
    pub info0: u16,
    pub info1: u16,
    pub sw_peer_id: u16,
    pub phy_ppdu_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_look_up_table {
    pub dbr_recv: bool,
    pub tx_recv: bool,
    pub data: *mut u8,
    pub data_len: u32,
    pub dbr_ppdu_id: u16,
    pub tx_ppdu_id: u16,
    pub dbr_address: dma_addr_t,
    pub header: ath11k_csi_cfr_header,
    pub hdr: ath11k_cfr_dma_hdr,
    pub txrx_tstamp: u64,
    pub dbr_tstamp: u64,
    pub header_length: u32,
    pub payload_length: u32,
    pub buff: *mut ath11k_dbring_element,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfr_unassoc_pool_entry {
    pub peer_mac: [u8; ETH_ALEN],
    pub period: u32,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_cfr {
    pub rx_ring: ath11k_dbring,
// Protects cfr data
    pub lock: spinlock_t,
// Protect for lut entries
    pub lut_lock: spinlock_t,
    pub lut: *mut ath11k_look_up_table,
    pub enable_cfr: *mut dentry,
    pub cfr_unassoc: *mut dentry,
    pub rfs_cfr_capture: *mut rchan,
    pub cfr_enabled_peer_cnt: u8,
    pub lut_num: u32,
    pub tx_evt_cnt: u64,
    pub dbr_evt_cnt: u64,
    pub release_cnt: u64,
    pub tx_peer_status_cfr_fail: u64,
    pub tx_evt_status_cfr_fail: u64,
    pub tx_dbr_lookup_fail: u64,
    pub last_success_tstamp: u64,
    pub flush_dbr_cnt: u64,
    pub clear_txrx_event: u64,
    pub cfr_dma_aborts: u64,
    pub enabled: bool,
    pub phymode: wmi_phy_mode,
    pub unassoc_pool: [cfr_unassoc_pool_entry; ATH11K_MAX_CFR_ENABLED_CLIENTS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_capture_method {
    ATH11K_CFR_CAPTURE_METHOD_NULL_FRAME,
    ATH11K_CFR_CAPTURE_METHOD_NULL_FRAME_WITH_PHASE,
    ATH11K_CFR_CAPTURE_METHOD_PROBE_RESP,
    ATH11K_CFR_CAPTURE_METHOD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_cfr_capture_bw {
    ATH11K_CFR_CAPTURE_BW_20,
    ATH11K_CFR_CAPTURE_BW_40,
    ATH11K_CFR_CAPTURE_BW_80,
    ATH11K_CFR_CAPTURE_BW_MAX,
}

extern "C" {
    pub fn ath11k_cfr_init(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_cfr_deinit(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_cfr_release_lut_entry(lut: *mut ath11k_look_up_table);
}
extern "C" {
    pub fn ath11k_cfr_update_phymode(ar: *mut ath11k, phymode: wmi_phy_mode);
}

