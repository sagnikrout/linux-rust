//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/dp_cmn.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// ML Peer IDs start from 8192, assuming max SLO clients count 1536,
// then max peer id shall be 9728, therefore rounding the peer table size
// to the nearest next power of 2 i.e 16384.
//
pub const MAX_DP_PEER_LIST_SIZE: c_int = 16384;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_hw {
    pub dp_peers: [*mut ath12k_dp_peer __rcu; MAX_DP_PEER_LIST_SIZE],
// Lock for protection of dp_peer_list and peers
    pub peer_lock: spinlock_t,
    pub dp_peers_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_hw_group {
    pub dp: [*mut ath12k_dp; ATH12K_MAX_DEVICES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_link_vif {
    pub vdev_id: u32,
    pub search_type: u8,
    pub hal_addr_search_flags: u8,
    pub pdev_idx: u8,
    pub lmac_id: u8,
    pub ast_idx: u16,
    pub ast_hash: u16,
    pub tcl_metadata: u16,
    pub vdev_id_check_en: u8,
    pub bank_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_vif {
    pub tx_encap_type: u8,
    pub key_cipher: u32,
    pub mcbc_gsn: core::sync::atomic::AtomicI32,
    pub dp_link_vif: [ath12k_dp_link_vif; ATH12K_NUM_MAX_LINKS],
}

// TODO: Move this to a separate dp_stats file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_per_peer_tx_stats {
    pub succ_bytes: u32,
    pub retry_bytes: u32,
    pub failed_bytes: u32,
    pub duration: u32,
    pub succ_pkts: u16,
    pub retry_pkts: u16,
    pub failed_pkts: u16,
    pub ru_start: u16,
    pub ru_tones: u16,
    pub ba_fails: u8,
    pub ppdu_type: u8,
    pub mu_grpid: u32,
    pub mu_pos: u32,
    pub is_ampdu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_peer_create_params {
    pub sta: *mut ieee80211_sta,
    pub is_mlo: bool,
    pub peer_id: u16,
    pub ucast_ra_only: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_link_peer_rate_info {
    pub txrate: rate_info,
    pub rx_duration: u64,
    pub tx_duration: u64,
    pub rssi_comb: u8,
    pub signal_avg: i8,
}

extern "C" {
    pub fn ath12k_dp_cmn_device_deinit(dp: *mut ath12k_dp);
}
extern "C" {
    pub fn ath12k_dp_cmn_device_init(dp: *mut ath12k_dp) -> c_int;
}
extern "C" {
    pub fn ath12k_dp_link_peer_reset_rx_stats(dp: *mut ath12k_dp, addr: *const u8);
}
