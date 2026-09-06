//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_vf_types.h
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
// struct xe_sriov_vf_relay_version - PF ABI version details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_vf_relay_version {
// @major: major version.
    pub major: u16,
// @minor: minor version.
    pub minor: u16,
}

//
// struct xe_device_vf - Xe Virtual Function related data
//
// The data in this structure is valid only if driver is running in the
// @XE_SRIOV_MODE_VF mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_device_vf {
// @pf_version: negotiated VF/PF ABI version.
    pub pf_version: xe_sriov_vf_relay_version,
// @migration: VF Migration state data
//
// @migration.disabled: flag indicating if migration support
// was turned off due to missing prerequisites
//
    pub disabled: bool,
    pub migration: },
// @ccs: VF CCS state data
    pub ccs: xe_sriov_vf_ccs,
}
