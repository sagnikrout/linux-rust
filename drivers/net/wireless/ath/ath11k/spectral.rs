//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/spectral.h
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
// Copyright (c) 2019-2020 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// enum ath11k_spectral_mode:
//
// @SPECTRAL_DISABLED: spectral mode is disabled
// @SPECTRAL_BACKGROUND: hardware sends samples when it is not busy with
// something else.
// @SPECTRAL_MANUAL: spectral scan is enabled, triggering for samples
// is performed manually.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_spectral_mode {
    ATH11K_SPECTRAL_DISABLED = 0,
    ATH11K_SPECTRAL_BACKGROUND,
    ATH11K_SPECTRAL_MANUAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_spectral {
    pub rx_ring: ath11k_dbring,
// Protects enabled
    pub lock: spinlock_t,
    pub /: *mut *mut *mut rchan rfs_scan; / relay(fs) channel for spectral scan,
    pub scan_ctl: *mut dentry,
    pub scan_count: *mut dentry,
    pub scan_bins: *mut dentry,
    pub mode: ath11k_spectral_mode,
    pub count: u16,
    pub fft_size: u8,
    pub enabled: bool,
    pub is_primary: bool,
}

extern "C" {
    pub fn ath11k_spectral_init(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_spectral_deinit(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_spectral_vif_stop(arvif: *mut ath11k_vif) -> c_int;
}
extern "C" {
    pub fn ath11k_spectral_reset_buffer(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_spectral_get_mode(ar: *mut ath11k) -> ath11k_spectral_mode;
}

