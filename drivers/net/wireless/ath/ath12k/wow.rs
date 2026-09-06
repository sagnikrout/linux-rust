//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/wow.h
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
// Copyright (c) 2020 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
pub const ATH12K_WOW_RETRY_NUM: c_int = 10;
pub const ATH12K_WOW_RETRY_WAIT_MS: c_int = 200;
pub const ATH12K_WOW_PATTERNS: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_wow {
    pub max_num_patterns: u32,
    pub wakeup_completed: completion,
    pub wowlan_support: wiphy_wowlan_support,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pkt_pattern {
    pub pattern: [u8; WOW_MAX_PATTERN_SIZE],
    pub bytemask: [u8; WOW_MAX_PATTERN_SIZE],
    pub pattern_len: c_int,
    pub pkt_offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfc1042_hdr {
    pub llc_dsap: u8,
    pub llc_ssap: u8,
    pub llc_ctrl: u8,
    pub snap_oui: [u8; 3],
    pub eth_type: __be16,
    pub __packed: },

    pub ar): *mut int ath12k_wow_init(struct ath12k,
    pub wowlan): *mut cfg80211_wowlan,
    pub hw): *mut int ath12k_wow_op_resume(struct ieee80211_hw,
    pub enabled): *mut *mut void ath12k_wow_op_set_wakeup(struct ieee80211_hw hw, bool,
    pub ar): *mut int ath12k_wow_enable(struct ath12k,
    pub ar): *mut int ath12k_wow_wakeup(struct ath12k,

    pub 0: return,
    pub 0: return,
    pub 0: return,

