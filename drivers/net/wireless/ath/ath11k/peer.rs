//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/peer.h
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
pub struct ath11k_peer {
    pub list: list_head,
    pub sta: *mut ieee80211_sta,
    pub vdev_id: c_int,
    pub addr: [u8; ETH_ALEN],
    pub peer_id: c_int,
    pub ast_hash: u16,
    pub pdev_idx: u8,
    pub hw_peer_id: u16,
// protected by ab->data_lock
    pub 1]: *mut *mut ieee80211_key_conf keys[WMI_MAX_KEY_INDEX +,
    pub 1]: dp_rx_tid rx_tid[IEEE80211_NUM_TIDS +,
// peer id based rhashtable list pointer
    pub rhash_id: rhash_head,
// peer addr based rhashtable list pointer
    pub rhash_addr: rhash_head,
// Info used in MMIC verification of
// RX fragments
//
    pub mcast_keyidx: u8,
    pub ucast_keyidx: u8,
    pub sec_type: u16,
    pub sec_type_grp: u16,
    pub is_authorized: bool,
    pub dp_setup_done: bool,
}

extern "C" {
    pub fn ath11k_peer_unmap_event(ab: *mut ath11k_base, peer_id: u16);
}
extern "C" {
    pub fn ath11k_peer_cleanup(ar: *mut ath11k, vdev_id: u32);
}
extern "C" {
    pub fn ath11k_peer_delete(ar: *mut ath11k, vdev_id: u32, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn ath11k_peer_rhash_tbl_init(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_peer_rhash_tbl_destroy(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_peer_rhash_delete(ab: *mut ath11k_base, peer: *mut ath11k_peer) -> c_int;
}
