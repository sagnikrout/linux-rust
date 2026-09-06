//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/dp_tx.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dp_htt_wbm_tx_status {
    pub msdu_id: u32,
    pub acked: bool,
    pub ack_rssi: i8,
    pub peer_id: u16,
}

extern "C" {
    pub fn ath11k_dp_tx_update_txcompl(ar: *mut ath11k, ts: *mut hal_tx_status);
}
extern "C" {
    pub fn ath11k_dp_tx_htt_h2t_ver_req_msg(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_dp_tx_completion_handler(ab: *mut ath11k_base, ring_id: c_int);
}
extern "C" {
    pub fn ath11k_dp_tx_htt_h2t_ppdu_stats_req(ar: *mut ath11k, mask: u32) -> c_int;
}
extern "C" {
    pub fn ath11k_dp_tx_htt_monitor_mode_ring_config(ar: *mut ath11k, reset: bool) -> c_int;
}
