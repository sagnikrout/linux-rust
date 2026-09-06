//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/dp_peer.h
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
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH12K_DP_PEER_ID_INVALID: c_uint = 0x3FFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppdu_user_delayba {
    pub sw_peer_id: u16,
    pub info0: u32,
    pub ru_end: u16,
    pub ru_start: u16,
    pub info1: u32,
    pub rate_flags: u32,
    pub resp_rate_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_rx_peer_rate_stats {
    pub 1]: u64 ht_mcs_count[HAL_RX_MAX_MCS_HT +,
    pub 1]: u64 vht_mcs_count[HAL_RX_MAX_MCS_VHT +,
    pub 1]: u64 he_mcs_count[HAL_RX_MAX_MCS_HE +,
    pub 1]: u64 be_mcs_count[HAL_RX_MAX_MCS_BE +,
    pub nss_count: [u64; HAL_RX_MAX_NSS],
    pub bw_count: [u64; HAL_RX_BW_MAX],
    pub gi_count: [u64; HAL_RX_GI_MAX],
    pub legacy_count: [u64; HAL_RX_MAX_NUM_LEGACY_RATES],
    pub 1]: u64 rx_rate[HAL_RX_BW_MAX][HAL_RX_GI_MAX][HAL_RX_MAX_NSS][HAL_RX_MAX_MCS_HT +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_rx_peer_stats {
    pub num_msdu: u64,
    pub num_mpdu_fcs_ok: u64,
    pub num_mpdu_fcs_err: u64,
    pub tcp_msdu_count: u64,
    pub udp_msdu_count: u64,
    pub other_msdu_count: u64,
    pub ampdu_msdu_count: u64,
    pub non_ampdu_msdu_count: u64,
    pub stbc_count: u64,
    pub beamformed_count: u64,
    pub coding_count: [u64; HAL_RX_SU_MU_CODING_MAX],
    pub 1]: u64 tid_count[IEEE80211_NUM_TIDS +,
    pub pream_cnt: [u64; HAL_RX_PREAMBLE_MAX],
    pub reception_type: [u64; HAL_RX_RECEPTION_TYPE_MAX],
    pub rx_duration: u64,
    pub dcm_count: u64,
    pub ru_alloc_cnt: [u64; HAL_RX_RU_ALLOC_TYPE_MAX],
    pub pkt_stats: ath12k_rx_peer_rate_stats,
    pub byte_stats: ath12k_rx_peer_rate_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_wbm_tx_stats {
    pub wbm_tx_comp_stats: [u64; HAL_WBM_REL_HTT_TX_COMP_STATUS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_peer_stats {
    pub rx_stats: *mut ath12k_rx_peer_stats,
    pub wbm_tx_stats: *mut ath12k_wbm_tx_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_link_peer {
    pub list: list_head,
    pub sta: *mut ieee80211_sta,
    pub dp_peer: *mut ath12k_dp_peer,
    pub vdev_id: c_int,
    pub addr: [u8; ETH_ALEN],
    pub peer_id: c_int,
    pub ast_hash: u16,
    pub pdev_idx: u8,
    pub hw_peer_id: u16,
    pub ppdu_stats_delayba: ppdu_user_delayba,
    pub delayba_flag: bool,
    pub is_authorized: bool,
    pub mlo: bool,
// protected by ab->data_lock
    pub ml_id: u16,
// any other ML info common for all partners can be added
// here and would be same for all partner peers.
//
    pub ml_addr: [u8; ETH_ALEN],
// To ensure only certain work related to dp is done once
    pub primary_link: bool,
// for reference to ath12k_link_sta
    pub link_id: u8,
// peer addr based rhashtable list pointer
    pub rhash_addr: rhash_head,
    pub hw_link_id: u8,
    pub rx_tid_active_bitmask: u32,
// link stats
    pub txrate: rate_info,
    pub last_txrate: rate_info,
    pub rx_duration: u64,
    pub tx_duration: u64,
    pub rssi_comb: u8,
    pub avg_rssi: ewma_avg_rssi,
    pub peer_stats: ath12k_dp_peer_stats,
    pub tx_retry_failed: u32,
    pub tx_retry_count: u32,
}

extern "C" {
    pub fn ath12k_dp_link_peer_unmap_event(ab: *mut ath12k_base, peer_id: u16);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_peer {
    pub list: list_head,
    pub is_mlo: bool,
    pub dp_setup_done: bool,
    pub ucast_keyidx: u8,
    pub addr: [u8; ETH_ALEN],
    pub mcast_keyidx: u8,
    pub ucast_ra_only: bool,
    pub peer_id: c_int,
    pub sta: *mut ieee80211_sta,
    pub hw_links: [u8; ATH12K_GROUP_MAX_RADIO],
    pub sec_type_grp: u16,
    pub sec_type: u16,
// Info used in MMIC verification of * RX fragments
    pub 1]: *mut *mut ieee80211_key_conf keys[WMI_MAX_KEY_INDEX +,
    pub link_peers_map: c_ulong,
    pub link_peers: [*mut ath12k_dp_link_peer __rcu; ATH12K_NUM_MAX_LINKS],
    pub 1]: ath12k_reoq_buf reoq_bufs[IEEE80211_NUM_TIDS +,
    pub 1]: ath12k_dp_rx_tid rx_tid[IEEE80211_NUM_TIDS +,
    pub use_4addr: bool,
}

extern "C" {
    pub fn ath12k_dp_link_peer_exist_by_vdev_id(dp: *mut ath12k_dp, vdev_id: c_int) -> bool;
}
extern "C" {
    pub fn ath12k_dp_link_peer_rhash_tbl_init(dp: *mut ath12k_dp) -> c_int;
}
extern "C" {
    pub fn ath12k_dp_link_peer_rhash_tbl_destroy(dp: *mut ath12k_dp);
}
extern "C" {
    pub fn ath12k_dp_peer_get_peerid_index(dp: *mut ath12k_dp, peer_id: u16) -> u16;
}
extern "C" {
    pub fn ath12k_dp_link_peer_free(peer: *mut ath12k_dp_link_peer);
}
