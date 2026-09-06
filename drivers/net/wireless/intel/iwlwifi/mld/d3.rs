//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/d3.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024, 2026 Intel Corporation
//

// Macro flag: #define __iwl_mld_d3_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_rekey_data {
    pub valid: bool,
    pub kck: [u8; NL80211_KCK_EXT_LEN],
    pub kek: [u8; NL80211_KEK_EXT_LEN],
    pub kck_len: usize,
    pub kek_len: usize,
    pub replay_ctr: __le64,
    pub akm: u32,
}

//
// struct iwl_mld_wowlan_data - data used by the wowlan suspend flow
//
// @target_ipv6_addrs: IPv6 addresses on this interface for offload
// @tentative_addrs: bitmap of tentative IPv6 addresses in @target_ipv6_addrs
// @num_target_ipv6_addrs: number of @target_ipv6_addrs
// @rekey_data: security key data used for rekeying during D3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_wowlan_data {
    pub target_ipv6_addrs: [in6_addr; IWL_PROTO_OFFLOAD_NUM_IPV6_ADDRS_MAX],
    pub tentative_addrs: [c_ulong; BITS_TO_LONGS(IWL_PROTO_OFFLOAD_NUM_IPV6_ADDRS_MAX)],
    pub num_target_ipv6_addrs: c_int,

    pub rekey_data: iwl_mld_rekey_data,
}

extern "C" {
    pub fn iwl_mld_no_wowlan_resume(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_no_wowlan_suspend(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_wowlan_resume(mld: *mut iwl_mld) -> c_int;
}

