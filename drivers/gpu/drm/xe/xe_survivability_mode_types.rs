//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_survivability_mode_types.h
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
// Copyright © 2025 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_reg {
    CAPABILITY_INFO,
    POSTCODE_TRACE,
    POSTCODE_TRACE_OVERFLOW,
    AUX_INFO0,
    AUX_INFO1,
    AUX_INFO2,
    AUX_INFO3,
    AUX_INFO4,
    MAX_SCRATCH_REG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_survivability_type {
    XE_SURVIVABILITY_TYPE_BOOT,
    XE_SURVIVABILITY_TYPE_RUNTIME,
}

//
// struct xe_survivability: Contains survivability mode information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_survivability {
// @info: survivability debug info
    pub info: [u32; MAX_SCRATCH_REG],
// @size: number of scratch registers
    pub size: u32,
// @boot_status: indicates critical/non critical boot failure
    pub boot_status: u8,
// @mode: boolean to indicate survivability mode
    pub mode: bool,
// @type: survivability type
    pub type: xe_survivability_type,
// @fdo_mode: indicates if FDO mode is enabled
    pub fdo_mode: bool,
// @version: breadcrumb version of survivability mode
    pub version: u8,
}
