//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/debugfs.h
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

extern "C" {
    pub fn ath12k_debugfs_soc_create(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_debugfs_soc_destroy(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_debugfs_register(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_debugfs_unregister(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_debugfs_pdev_create(ab: *mut ath12k_base);
}
pub const ATH12K_CCK_RATES: c_int = 4;
pub const ATH12K_OFDM_RATES: c_int = 8;
pub const ATH12K_HT_RATES: c_int = 8;
pub const ATH12K_VHT_RATES: c_int = 12;
pub const ATH12K_HE_RATES: c_int = 12;
pub const ATH12K_HE_RATES_WITH_EXTRA_MCS: c_int = 14;
pub const ATH12K_EHT_RATES: c_int = 16;

pub const ATH12K_NSS_1: c_int = 1;
pub const ATH12K_NSS_4: c_int = 4;
pub const ATH12K_NSS_8: c_int = 8;

pub const MAX_TPC_PREAM_STR_LEN: c_int = 7;

pub const TPC_MAX: c_int = 127;

pub const TPC_STATS_TOT_ROW: c_int = 700;
pub const TPC_STATS_TOT_COLUMN: c_int = 100;
pub const MODULATION_LIMIT: c_int = 126;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tpc_pream_bw {
    WMI_TPC_PREAM_CCK,
    WMI_TPC_PREAM_OFDM,
    WMI_TPC_PREAM_HT20,
    WMI_TPC_PREAM_HT40,
    WMI_TPC_PREAM_VHT20,
    WMI_TPC_PREAM_VHT40,
    WMI_TPC_PREAM_VHT80,
    WMI_TPC_PREAM_VHT160,
    WMI_TPC_PREAM_HE20,
    WMI_TPC_PREAM_HE40,
    WMI_TPC_PREAM_HE80,
    WMI_TPC_PREAM_HE160,
    WMI_TPC_PREAM_EHT20,
    WMI_TPC_PREAM_EHT40,
    WMI_TPC_PREAM_EHT60,
    WMI_TPC_PREAM_EHT80,
    WMI_TPC_PREAM_EHT120,
    WMI_TPC_PREAM_EHT140,
    WMI_TPC_PREAM_EHT160,
    WMI_TPC_PREAM_EHT200,
    WMI_TPC_PREAM_EHT240,
    WMI_TPC_PREAM_EHT280,
    WMI_TPC_PREAM_EHT320,
    WMI_TPC_PREAM_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_debug_tpc_stats_ctl_mode {
    ATH12K_TPC_STATS_CTL_MODE_LEGACY_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HT_VHT20_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HE_EHT20_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HT_VHT40_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HE_EHT40_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_VHT80_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HE_EHT80_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_VHT160_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HE_EHT160_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HE_EHT320_5GHZ_6GHZ,
    ATH12K_TPC_STATS_CTL_MODE_CCK_2GHZ,
    ATH12K_TPC_STATS_CTL_MODE_LEGACY_2GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HT20_2GHZ,
    ATH12K_TPC_STATS_CTL_MODE_HT40_2GHZ,

    ATH12K_TPC_STATS_CTL_MODE_EHT80_SU_PUNC20 = 23,
    ATH12K_TPC_STATS_CTL_MODE_EHT160_SU_PUNC20,
    ATH12K_TPC_STATS_CTL_MODE_EHT320_SU_PUNC40,
    ATH12K_TPC_STATS_CTL_MODE_EHT320_SU_PUNC80,
    ATH12K_TPC_STATS_CTL_MODE_EHT320_SU_PUNC120
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_debug_tpc_stats_support_modes {
    ATH12K_TPC_STATS_SUPPORT_160 = 0,
    ATH12K_TPC_STATS_SUPPORT_320,
    ATH12K_TPC_STATS_SUPPORT_AX,
    ATH12K_TPC_STATS_SUPPORT_AX_EXTRA_MCS,
    ATH12K_TPC_STATS_SUPPORT_BE,
    ATH12K_TPC_STATS_SUPPORT_BE_PUNC,
}

