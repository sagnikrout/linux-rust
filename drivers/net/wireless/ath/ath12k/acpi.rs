//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/acpi.h
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

pub const ATH12K_ACPI_DSM_FUNC_SUPPORT_FUNCS: c_int = 0;
pub const ATH12K_ACPI_DSM_FUNC_DISABLE_FLAG: c_int = 2;
pub const ATH12K_ACPI_DSM_FUNC_BDF_EXT: c_int = 3;
pub const ATH12K_ACPI_DSM_FUNC_BIOS_SAR: c_int = 4;
pub const ATH12K_ACPI_DSM_FUNC_GEO_OFFSET: c_int = 5;
pub const ATH12K_ACPI_DSM_FUNC_INDEX_CCA: c_int = 6;
pub const ATH12K_ACPI_DSM_FUNC_TAS_CFG: c_int = 8;
pub const ATH12K_ACPI_DSM_FUNC_TAS_DATA: c_int = 9;
pub const ATH12K_ACPI_DSM_FUNC_INDEX_BAND_EDGE: c_int = 10;

pub const ATH12K_ACPI_NOTIFY_EVENT: c_uint = 0x86;

pub const ATH12K_ACPI_TAS_DATA_VERSION: c_uint = 0x1;
pub const ATH12K_ACPI_TAS_DATA_ENABLE: c_uint = 0x1;
pub const ATH12K_ACPI_POWER_LIMIT_VERSION: c_uint = 0x1;
pub const ATH12K_ACPI_POWER_LIMIT_ENABLE_FLAG: c_uint = 0x1;
pub const ATH12K_ACPI_CCA_THR_VERSION: c_uint = 0x1;
pub const ATH12K_ACPI_CCA_THR_ENABLE_FLAG: c_uint = 0x1;
pub const ATH12K_ACPI_BAND_EDGE_VERSION: c_uint = 0x1;
pub const ATH12K_ACPI_BAND_EDGE_ENABLE_FLAG: c_uint = 0x1;
pub const ATH12K_ACPI_GEO_OFFSET_DATA_OFFSET: c_int = 1;
pub const ATH12K_ACPI_DBS_BACKOFF_DATA_OFFSET: c_int = 2;
pub const ATH12K_ACPI_CCA_THR_OFFSET_DATA_OFFSET: c_int = 5;
pub const ATH12K_ACPI_BIOS_SAR_DBS_BACKOFF_LEN: c_int = 10;
pub const ATH12K_ACPI_POWER_LIMIT_DATA_OFFSET: c_int = 12;
pub const ATH12K_ACPI_BIOS_SAR_GEO_OFFSET_LEN: c_int = 18;
pub const ATH12K_ACPI_BIOS_SAR_TABLE_LEN: c_int = 22;
pub const ATH12K_ACPI_CCA_THR_OFFSET_LEN: c_int = 36;
pub const ATH12K_ACPI_DSM_TAS_DATA_SIZE: c_int = 69;
pub const ATH12K_ACPI_DSM_BAND_EDGE_DATA_SIZE: c_int = 100;
pub const ATH12K_ACPI_DSM_TAS_CFG_SIZE: c_int = 108;
pub const ATH12K_ACPI_DSM_FUNC_MIN_BITMAP_SIZE: c_int = 1;
pub const ATH12K_ACPI_DSM_FUNC_MAX_BITMAP_SIZE: c_int = 4;

pub const ATH12K_ACPI_BDF_ANCHOR_STRING_LEN: c_int = 3;

pub const ATH12K_ACPI_BDF_MAX_LEN: c_int = 100;

extern "C" {
    pub fn ath12k_acpi_start(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_acpi_stop(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_acpi_get_disable_rfkill(ab: *mut ath12k_base) -> bool;
}
extern "C" {
    pub fn ath12k_acpi_get_disable_11be(ab: *mut ath12k_base) -> bool;
}
extern "C" {
    pub fn ath12k_acpi_set_dsm_func(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_acpi_check_bdf_variant_name(ab: *mut ath12k_base) -> c_int;
}

