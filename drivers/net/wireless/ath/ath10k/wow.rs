//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/wow.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2015,2017 Qualcomm Atheros, Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_wow {
    pub max_num_patterns: u32,
    pub wakeup_completed: completion,
    pub wowlan_support: wiphy_wowlan_support,
}

extern "C" {
    pub fn ath10k_wow_init(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_wow_op_resume(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn ath10k_wow_op_set_wakeup(hw: *mut ieee80211_hw, enabled: bool);
}

