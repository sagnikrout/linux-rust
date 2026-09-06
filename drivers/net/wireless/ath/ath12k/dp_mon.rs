//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/dp_mon.h
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
// Copyright (c) 2019-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH12K_MON_RX_DOT11_OFFSET: c_int = 5;
pub const ATH12K_MON_RX_PKT_OFFSET: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_monitor_mode {
    ATH12K_DP_TX_MONITOR_MODE,
    ATH12K_DP_RX_MONITOR_MODE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_mon_tx_ppdu_info_type {
    DP_MON_TX_PROT_PPDU_INFO,
    DP_MON_TX_DATA_PPDU_INFO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_mon_tx_tlv_status {
    DP_MON_TX_FES_SETUP,
    DP_MON_TX_FES_STATUS_END,
    DP_MON_RX_RESPONSE_REQUIRED_INFO,
    DP_MON_RESPONSE_END_STATUS_INFO,
    DP_MON_TX_MPDU_START,
    DP_MON_TX_MSDU_START,
    DP_MON_TX_BUFFER_ADDR,
    DP_MON_TX_DATA,
    DP_MON_TX_STATUS_PPDU_NOT_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_mon_tx_medium_protection_type {
    DP_MON_TX_MEDIUM_NO_PROTECTION,
    DP_MON_TX_MEDIUM_RTS_LEGACY,
    DP_MON_TX_MEDIUM_RTS_11AC_STATIC_BW,
    DP_MON_TX_MEDIUM_RTS_11AC_DYNAMIC_BW,
    DP_MON_TX_MEDIUM_CTS2SELF,
    DP_MON_TX_MEDIUM_QOS_NULL_NO_ACK_3ADDR,
    DP_MON_TX_MEDIUM_QOS_NULL_NO_ACK_4ADDR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_mon_qosframe_addr4 {
    pub frame_control: __le16,
    pub duration: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctrl: __le16,
    pub addr4: [u8; ETH_ALEN],
    pub qos_ctrl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_mon_frame_min_one {
    pub frame_control: __le16,
    pub duration: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_mon_packet_info {
    pub cookie: u64,
    pub dma_length: u16,
    pub msdu_continuation: bool,
    pub truncated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_mon_tx_ppdu_info {
    pub ppdu_id: u32,
    pub num_users: u8,
    pub is_used: bool,
    pub rx_status: hal_rx_mon_ppdu_info,
    pub dp_tx_mon_mpdu_list: list_head,
    pub tx_mon_mpdu: *mut dp_mon_mpdu,
}

extern "C" {
    pub fn ath12k_dp_mon_rx_process_ulofdma(ppdu_info: *mut hal_rx_mon_ppdu_info);
}
extern "C" {
    pub fn ath12k_dp_pkt_set_pktlen(skb: *mut sk_buff, len: u32) -> c_int;
}
// ath12k_dp_rx_alloc_mon_status_buf(struct ath12k_base *ab,
extern "C" {
    pub fn ath12k_dp_mon_comp_ppduid(msdu_ppdu_id: u32, ppdu_id: *mut u32) -> u32;
}
