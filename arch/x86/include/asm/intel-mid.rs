//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/intel-mid.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Intel MID specific setup code
//
// (C) Copyright 2009, 2021 Intel Corporation
//

extern "C" {
    pub fn intel_mid_pci_init() -> c_int;
}
extern "C" {
    pub fn intel_mid_pci_set_power_state(pdev: *mut pci_dev, state: pci_power_t) -> c_int;
}
extern "C" {
    pub fn intel_mid_pci_get_power_state(pdev: *mut pci_dev) -> pci_power_t;
}
extern "C" {
    pub fn intel_mid_pwr_power_off();
}
pub const INTEL_MID_PWR_LSS_OFFSET: c_int = 4;

extern "C" {
    pub fn intel_mid_pwr_get_lss_id(pdev: *mut pci_dev) -> c_int;
}
