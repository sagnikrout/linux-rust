//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/fw.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_fw_ie_type {
    ATH12K_FW_IE_TIMESTAMP = 0,
    ATH12K_FW_IE_FEATURES = 1,
    ATH12K_FW_IE_AMSS_IMAGE = 2,
    ATH12K_FW_IE_M3_IMAGE = 3,
    ATH12K_FW_IE_AMSS_DUALMAC_IMAGE = 4,
    ATH12K_FW_IE_AUX_UC_IMAGE = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_fw_features {
// The firmware supports setting the QRTR id via register
// PCIE_LOCAL_REG_QRTR_NODE_ID
//
    ATH12K_FW_FEATURE_MULTI_QRTR_ID = 0,

// The firmware supports MLO capability
    ATH12K_FW_FEATURE_MLO,

// keep last
    ATH12K_FW_FEATURE_COUNT,
}

extern "C" {
    pub fn ath12k_fw_map(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_fw_unmap(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_fw_feature_supported(ab: *mut ath12k_base, feat: ath12k_fw_features) -> bool;
}
