//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/spectral.h
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
// Copyright (c) 2013-2015 Qualcomm Atheros, Inc.
//

//
// struct ath10k_spec_scan - parameters for Atheros spectral scan
//
// @count: number of scan results requested for manual mode
// @fft_size: number of bins to be requested = 2^(fft_size - bin_scale)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_spec_scan {
    pub count: u8,
    pub fft_size: u8,
}

// enum ath10k_spectral_mode:
//
// @SPECTRAL_DISABLED: spectral mode is disabled
// @SPECTRAL_BACKGROUND: hardware sends samples when it is not busy with
// something else.
// @SPECTRAL_MANUAL: spectral scan is enabled, triggering for samples
// is performed manually.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_spectral_mode {
    SPECTRAL_DISABLED = 0,
    SPECTRAL_BACKGROUND,
    SPECTRAL_MANUAL,
}

extern "C" {
    pub fn ath10k_spectral_start(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_spectral_vif_stop(arvif: *mut ath10k_vif) -> c_int;
}
extern "C" {
    pub fn ath10k_spectral_create(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_spectral_destroy(ar: *mut ath10k);
}

