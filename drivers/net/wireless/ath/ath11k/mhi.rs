//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/mhi.h
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
// Copyright (c) 2020 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const PCIE_TXVECDB: c_uint = 0x360;
pub const PCIE_TXVECSTATUS: c_uint = 0x368;
pub const PCIE_RXVECDB: c_uint = 0x394;
pub const PCIE_RXVECSTATUS: c_uint = 0x39C;
pub const MHISTATUS: c_uint = 0x48;
pub const MHICTRL: c_uint = 0x38;
pub const MHICTRL_RESET_MASK: c_uint = 0x2;
extern "C" {
    pub fn ath11k_mhi_start(ar_pci: *mut ath11k_pci) -> c_int;
}
extern "C" {
    pub fn ath11k_mhi_stop(ar_pci: *mut ath11k_pci, is_suspend: bool);
}
extern "C" {
    pub fn ath11k_mhi_register(ar_pci: *mut ath11k_pci) -> c_int;
}
extern "C" {
    pub fn ath11k_mhi_unregister(ar_pci: *mut ath11k_pci);
}
extern "C" {
    pub fn ath11k_mhi_set_mhictrl_reset(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_mhi_clear_vector(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_mhi_suspend(ar_pci: *mut ath11k_pci) -> c_int;
}
extern "C" {
    pub fn ath11k_mhi_resume(ar_pci: *mut ath11k_pci) -> c_int;
}
extern "C" {
    pub fn ath11k_mhi_coredump(mhi_ctrl: *mut mhi_controller, in_panic: bool);
}
