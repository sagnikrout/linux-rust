//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_pf_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023-2025 Intel Corporation
//

//
// struct xe_sriov_metadata - per-VF device level metadata
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_metadata {
// @kobj: kobject representing VF in PF's SR-IOV sysfs tree.
    pub kobj: *mut kobject,
// @version: negotiated VF/PF ABI version
    pub version: xe_sriov_pf_service_version,
// @migration: migration state
    pub migration: xe_sriov_migration_state,
}

//
// struct xe_device_pf - Xe PF related data
//
// The data in this structure is valid only if driver is running in the
// @XE_SRIOV_MODE_PF mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_device_pf {
// @device_total_vfs: Maximum number of VFs supported by the device.
    pub device_total_vfs: u16,
// @driver_max_vfs: Maximum number of VFs supported by the driver.
    pub driver_max_vfs: u16,
// @guard_vfs_enabling: guards VFs enabling
    pub guard_vfs_enabling: xe_guard,
// @master_lock: protects all VFs configurations across GTs
    pub master_lock: mutex,
// @provision: device level provisioning data.
    pub provision: xe_sriov_pf_provision,
// @migration: device level migration data.
    pub migration: xe_sriov_pf_migration,
// @service: device level service data.
    pub service: xe_sriov_pf_service,
// @sysfs: device level sysfs data.
// @sysfs.root: the root kobject for all SR-IOV entries in sysfs.
    pub root: *mut kobject,
    pub sysfs: },
// @vfs: metadata for all VFs.
    pub vfs: *mut xe_sriov_metadata,
}
