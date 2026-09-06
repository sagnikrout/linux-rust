//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/iommu-priv.h
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
// Copyright (c) 2023, NVIDIA CORPORATION & AFFILIATES.
//

//
// Assume that valid ops must be installed if iommu_probe_device()
// has succeeded. The device ops are essentially for internal use
// within the IOMMU subsystem itself, so we should be able to trust
// ourselves not to misuse the helper.
//
extern "C" {
    pub fn dev_iommu_free(dev: *mut device);
}
extern "C" {
    pub fn iommu_ops_from_fwnode(NULL: fwspec ? fwspec->iommu_fwnode :) -> return;
}
extern "C" {
    pub fn iommu_fwspec_free(dev: *mut device);
}
extern "C" {
    pub fn iommu_mock_device_add(dev: *mut device, iommu: *mut iommu_device) -> c_int;
}

extern "C" {
    pub fn iommu_debug_init();
}

