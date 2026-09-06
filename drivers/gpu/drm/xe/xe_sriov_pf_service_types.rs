//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_pf_service_types.h
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
// struct xe_sriov_pf_service_version - VF/PF ABI Version.
// @major: the major version of the VF/PF ABI
// @minor: the minor version of the VF/PF ABI
//
// See `GuC Relay Communication`_.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_pf_service_version {
    pub major: u16,
    pub minor: u16,
}

//
// struct xe_sriov_pf_service - Data used by the PF service.
// @version: information about VF/PF ABI versions for current platform.
// @version.base: lowest VF/PF ABI version that could be negotiated with VF.
// @version.latest: latest VF/PF ABI version supported by the PF driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_pf_service {
    pub base: xe_sriov_pf_service_version,
    pub latest: xe_sriov_pf_service_version,
    pub version: },
}
