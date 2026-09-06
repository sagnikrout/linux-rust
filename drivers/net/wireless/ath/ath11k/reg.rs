//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/reg.h
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
// Copyright (c) 2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// DFS regdomains supported by Firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_dfs_region {
    ATH11K_DFS_REG_UNSET,
    ATH11K_DFS_REG_FCC,
    ATH11K_DFS_REG_ETSI,
    ATH11K_DFS_REG_MKK,
    ATH11K_DFS_REG_CN,
    ATH11K_DFS_REG_KR,
    ATH11K_DFS_REG_MKK_N,
    ATH11K_DFS_REG_UNDEF,
}

// Phy bitmaps

// ATH11K Regulatory API's
extern "C" {
    pub fn ath11k_reg_init(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_reg_reset_info(reg_info: *mut cur_regulatory_info);
}
extern "C" {
    pub fn ath11k_reg_free(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_regd_update_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath11k_regd_update_chan_list_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath11k_regd_update(ar: *mut ath11k) -> c_int;
}
extern "C" {
    pub fn ath11k_reg_update_chan_list(ar: *mut ath11k, wait: bool) -> c_int;
}
extern "C" {
    pub fn ath11k_reg_set_cc(ar: *mut ath11k) -> c_int;
}
