//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/virtual/mac80211_hwsim_nan.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// mac80211_hwsim_nan - NAN software simulation for mac80211_hwsim
// Copyright (C) 2025-2026 Intel Corporation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac80211_hwsim_nan_phase {
    MAC80211_HWSIM_NAN_PHASE_SCAN,
    MAC80211_HWSIM_NAN_PHASE_WARMUP,
    MAC80211_HWSIM_NAN_PHASE_UP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac80211_hwsim_nan_role {
    MAC80211_HWSIM_NAN_ROLE_MASTER,
    MAC80211_HWSIM_NAN_ROLE_SYNC,
    MAC80211_HWSIM_NAN_ROLE_NON_SYNC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac80211_hwsim_nan_data {
    pub device_vif: *mut ieee80211_vif,
    pub bands: u8,
    pub slot_timer: hrtimer,
    pub resume_txqs_timer: hrtimer,
    pub notify_dw: bool,
    pub discovery_beacon_timer: hrtimer,
// Later members are protected by this lock
    pub state_lock: spinlock_t,
    pub master_pref: u8,
    pub random_factor: u8,
    pub random_factor_valid_dwst: u8,
    pub phase: mac80211_hwsim_nan_phase,
    pub role: mac80211_hwsim_nan_role,
    pub cluster_id: [u8; ETH_ALEN],
    pub current_ami: ieee80211_nan_anchor_master_info,
    pub last_ami: ieee80211_nan_anchor_master_info,
// Wi-Fi Aware version 4.0, section 3.3.6.1 and 3.3.6.2
    pub master_transition_score: c_int,
// Wi-Fi Aware version 4.0, section 3.3.6.3 and 3.3.6.4
    pub sync_transition_score: c_int,
    pub tsf_adjusted: bool,
    pub tsf_discontinuity: bool,
//
// Local schedule - stores channel definition for each 16TU slot.
// Derived from NMI vif->cfg.nan_schedule. chan == NULL means not
// available in that slot (except DW which is implicit).
//
    pub local_sched: [cfg80211_chan_def; CFG80211_NAN_SCHED_NUM_TIME_SLOTS],
}
