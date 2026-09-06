//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/reg.h
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

pub const ATH12K_2GHZ_MAX_FREQUENCY: c_int = 2495;
pub const ATH12K_5GHZ_MAX_FREQUENCY: c_int = 5920;
// DFS regdomains supported by Firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_dfs_region {
    ATH12K_DFS_REG_UNSET,
    ATH12K_DFS_REG_FCC,
    ATH12K_DFS_REG_ETSI,
    ATH12K_DFS_REG_MKK,
    ATH12K_DFS_REG_CN,
    ATH12K_DFS_REG_KR,
    ATH12K_DFS_REG_MKK_N,
    ATH12K_DFS_REG_UNDEF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_reg_cc_code {
    REG_SET_CC_STATUS_PASS = 0,
    REG_CURRENT_ALPHA2_NOT_FOUND = 1,
    REG_INIT_ALPHA2_NOT_FOUND = 2,
    REG_SET_CC_CHANGE_NOT_ALLOWED = 3,
    REG_SET_CC_STATUS_NO_MEMORY = 4,
    REG_SET_CC_STATUS_FAIL = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_reg_rule {
    pub start_freq: u16,
    pub end_freq: u16,
    pub max_bw: u16,
    pub reg_power: u8,
    pub ant_gain: u8,
    pub flags: u16,
    pub psd_flag: bool,
    pub psd_eirp: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_reg_info {
    pub status_code: ath12k_reg_cc_code,
    pub num_phy: u8,
    pub phy_id: u8,
    pub reg_dmn_pair: u16,
    pub ctry_code: u16,
    pub 1]: u8 alpha2[REG_ALPHA2_LEN +,
    pub dfs_region: u32,
    pub phybitmap: u32,
    pub is_ext_reg_event: bool,
    pub min_bw_2g: u32,
    pub max_bw_2g: u32,
    pub min_bw_5g: u32,
    pub max_bw_5g: u32,
    pub num_2g_reg_rules: u32,
    pub num_5g_reg_rules: u32,
    pub reg_rules_2g_ptr: *mut ath12k_reg_rule,
    pub reg_rules_5g_ptr: *mut ath12k_reg_rule,
    pub client_type: wmi_reg_6g_client_type,
    pub rnr_tpe_usable: bool,
    pub unspecified_ap_usable: bool,
// TODO: All 6G related info can be stored only for required
// combination instead of all types, to optimize memory usage.
//
    pub domain_code_6g_ap: [u8; WMI_REG_CURRENT_MAX_AP_TYPE],
    pub domain_code_6g_client: [u8; WMI_REG_CURRENT_MAX_AP_TYPE][WMI_REG_MAX_CLIENT_TYPE],
    pub domain_code_6g_super_id: u32,
    pub min_bw_6g_ap: [u32; WMI_REG_CURRENT_MAX_AP_TYPE],
    pub max_bw_6g_ap: [u32; WMI_REG_CURRENT_MAX_AP_TYPE],
    pub min_bw_6g_client: [u32; WMI_REG_CURRENT_MAX_AP_TYPE][WMI_REG_MAX_CLIENT_TYPE],
    pub max_bw_6g_client: [u32; WMI_REG_CURRENT_MAX_AP_TYPE][WMI_REG_MAX_CLIENT_TYPE],
    pub num_6g_reg_rules_ap: [u32; WMI_REG_CURRENT_MAX_AP_TYPE],
    pub num_6g_reg_rules_cl: [u32; WMI_REG_CURRENT_MAX_AP_TYPE][WMI_REG_MAX_CLIENT_TYPE],
    pub reg_rules_6g_ap_ptr: [*mut ath12k_reg_rule; WMI_REG_CURRENT_MAX_AP_TYPE],
}

// Phy bitmaps
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_reg_phy_bitmap {
    ATH12K_REG_PHY_BITMAP_NO11AX	= BIT(5),
    ATH12K_REG_PHY_BITMAP_NO11BE	= BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_reg_status {
    ATH12K_REG_STATUS_VALID,
    ATH12K_REG_STATUS_DROP,
    ATH12K_REG_STATUS_FALLBACK,
}

extern "C" {
    pub fn ath12k_reg_init(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn ath12k_reg_free(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_regd_update_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath12k_regd_update(ar: *mut ath12k, init: bool) -> c_int;
}
extern "C" {
    pub fn ath12k_reg_update_chan_list(ar: *mut ath12k, wait: bool) -> c_int;
}
extern "C" {
    pub fn ath12k_reg_reset_reg_info(reg_info: *mut ath12k_reg_info);
}
extern "C" {
    pub fn ath12k_regd_update_chan_list_work(work: *mut work_struct);
}
