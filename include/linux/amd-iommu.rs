//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amd-iommu.h
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
// Copyright (C) 2007-2010 Advanced Micro Devices, Inc.
// Author: Joerg Roedel <joerg.roedel@amd.com>
// Leo Duran <leo.duran@amd.com>
//

extern "C" {
    pub fn amd_iommu_detect();
}

// IOMMU AVIC Function
extern "C" {
    pub fn amd_iommu_register_ga_log_notifier((*notifier)(u32): *mut c_int) -> c_int;
}
extern "C" {
    pub fn amd_iommu_update_ga(data: *mut c_void, cpu: c_int, ga_log_intr: bool) -> c_int;
}
extern "C" {
    pub fn amd_iommu_activate_guest_mode(data: *mut c_void, cpu: c_int, ga_log_intr: bool) -> c_int;
}
extern "C" {
    pub fn amd_iommu_deactivate_guest_mode(data: *mut c_void) -> c_int;
}

extern "C" {
    pub fn amd_iommu_get_num_iommus() -> c_int;
}
extern "C" {
    pub fn amd_iommu_pc_supported() -> bool;
}
extern "C" {
    pub fn amd_iommu_pc_get_max_banks(idx: c_uint) -> u8;
}
extern "C" {
    pub fn amd_iommu_pc_get_max_counters(idx: c_uint) -> u8;
}

extern "C" {
    pub fn amd_iommu_snp_disable() -> c_int;
}
extern "C" {
    pub fn amd_iommu_sev_tio_supported() -> bool;
}

