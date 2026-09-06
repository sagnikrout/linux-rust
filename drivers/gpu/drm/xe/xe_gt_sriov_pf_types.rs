//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_types.h
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
// Copyright © 2023-2024 Intel Corporation
//

//
// struct xe_gt_sriov_metadata - GT level per-VF metadata.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_metadata {
// @config: per-VF provisioning data.
    pub config: xe_gt_sriov_config,
// @monitor: per-VF monitoring data.
    pub monitor: xe_gt_sriov_monitor,
// @control: per-VF control data.
    pub control: xe_gt_sriov_control_state,
// @version: negotiated VF/PF ABI version
    pub version: xe_gt_sriov_pf_service_version,
// @migration: per-VF migration data.
    pub migration: xe_gt_sriov_migration_data,
}

//
// struct xe_gt_sriov_pf_workers - GT level workers used by the PF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_pf_workers {
// @restart: worker that executes actions post GT reset
    pub restart: work_struct,
}

//
// struct xe_gt_sriov_pf - GT level PF virtualization data.
// @workers: workers data.
// @service: service data.
// @control: control data.
// @policy: policy data.
// @spare: PF-only provisioning configuration.
// @vfs: metadata for all VFs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_pf {
    pub workers: xe_gt_sriov_pf_workers,
    pub service: xe_gt_sriov_pf_service,
    pub control: xe_gt_sriov_pf_control,
    pub policy: xe_gt_sriov_pf_policy,
    pub spare: xe_gt_sriov_spare_config,
    pub vfs: *mut xe_gt_sriov_metadata,
}
