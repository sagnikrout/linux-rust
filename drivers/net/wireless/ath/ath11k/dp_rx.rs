//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/dp_rx.h
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
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const DP_MAX_NWIFI_HDR_LEN: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_rx_decap_type {
    DP_RX_DECAP_TYPE_RAW,
    DP_RX_DECAP_TYPE_NATIVE_WIFI,
    DP_RX_DECAP_TYPE_ETHERNET2_DIX,
    DP_RX_DECAP_TYPE_8023,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dp_amsdu_subframe_hdr {
    pub dst: [u8; ETH_ALEN],
    pub src: [u8; ETH_ALEN],
    pub len: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dp_rfc1042_hdr {
    pub llc_dsap: u8,
    pub llc_ssap: u8,
    pub llc_ctrl: u8,
    pub snap_oui: [u8; 3],
    pub snap_type: __be16,
    pub __packed: },
    pub params): *mut ieee80211_ampdu_params,
    pub params): *mut ieee80211_ampdu_params,
    pub key): *mut ieee80211_key_conf,
    pub peer): *mut *mut void ath11k_peer_frags_flush(struct ath11k ar, struct ath11k_peer,
    pub peer): *mut *mut void ath11k_peer_rx_tid_cleanup(struct ath11k ar, struct ath11k_peer,
    pub tid): *mut *mut ath11k_peer peer, u8,
    pub pn_type): hal_pn_type,
    pub skb): *mut sk_buff,
    pub ab): *mut int ath11k_dp_pdev_reo_setup(struct ath11k_base,
    pub ab): *mut void ath11k_dp_pdev_reo_cleanup(struct ath11k_base,
    pub pdev_idx): *mut *mut int ath11k_dp_rx_pdev_alloc(struct ath11k_base ab, int,
    pub pdev_idx): *mut *mut void ath11k_dp_rx_pdev_free(struct ath11k_base ab, int,
    pub ab): *mut void ath11k_dp_reo_cmd_list_cleanup(struct ath11k_base,
    pub ab): *mut void ath11k_dp_process_reo_status(struct ath11k_base,
    pub budget): *mut *mut int ath11k_dp_process_rxdma_err(struct ath11k_base ab, int mac_id, int,
    pub budget): *mut *mut napi_napi, int,
    pub budget): c_int,
    pub budget): c_int,
    pub mgr): hal_rx_buf_return_buf_manager,
    pub data): *mut c_void,
    pub budget): *mut *mut napi_napi, int,
    pub budget): *mut *mut napi_napi, int,
    pub mgr): hal_rx_buf_return_buf_manager,
    pub ar): *mut int ath11k_dp_rx_pdev_mon_detach(struct ath11k,
    pub ar): *mut int ath11k_dp_rx_pdev_mon_attach(struct ath11k,
    pub vdev_id): *const *const *const int ath11k_peer_rx_frag_setup(struct ath11k ar, u8 peer_mac, int,
    pub ab): *mut int ath11k_dp_rx_pktlog_start(struct ath11k_base,
    pub stop_timer): *mut *mut int ath11k_dp_rx_pktlog_stop(struct ath11k_base ab, bool,
    pub enctype): *mut *mut int ath11k_dp_rx_crypto_mic_len(struct ath11k ar, enum hal_encrypt_type,
