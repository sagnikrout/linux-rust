//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/peer.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_peer_delete_wait {
    pub list: list_head,
    pub vdev_id: u32,
    pub addr: [u8; ETH_ALEN],
    pub done: completion,
}

extern "C" {
    pub fn ath12k_peer_delete_resp_signal(ar: *mut ath12k, vdev_id: u32, addr: *const u8);
}
extern "C" {
    pub fn ath12k_peer_delete_wait_flush(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_peer_cleanup(ar: *mut ath12k, vdev_id: u32);
}
extern "C" {
    pub fn ath12k_peer_delete(ar: *mut ath12k, vdev_id: u32, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn ath12k_peer_mlo_link_peers_delete(ahvif: *mut ath12k_vif, ahsta: *mut ath12k_sta) -> c_int;
}
extern "C" {
    pub fn ath12k_link_sta_rhash_tbl_init(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_link_sta_rhash_tbl_destroy(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_link_sta_rhash_delete(ab: *mut ath12k_base, arsta: *mut ath12k_link_sta);
}
extern "C" {
    pub fn ath12k_link_sta_rhash_add(ab: *mut ath12k_base, arsta: *mut ath12k_link_sta) -> c_int;
}
extern "C" {
    pub fn ath12k_peer_ml_alloc(ah: *mut ath12k_hw) -> u16;
}
extern "C" {
    pub fn ath12k_peer_ml_free(ah: *mut ath12k_hw, ahsta: *mut ath12k_sta);
}
