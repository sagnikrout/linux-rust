//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_drm_ras_types.h
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
// Copyright © 2026 Intel Corporation
//

// Error categories reported by hardware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hardware_error {
    HARDWARE_ERROR_CORRECTABLE = 0,
    HARDWARE_ERROR_NONFATAL,
    HARDWARE_ERROR_FATAL,
    HARDWARE_ERROR_MAX
}

//
// struct xe_drm_ras_counter - XE RAS counter
//
// This structure contains error component and counter information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_drm_ras_counter {
// @name: error component name
    pub name: *const c_char,
// @counter: count of error
    pub counter: core::sync::atomic::AtomicI32,
}

//
// struct xe_drm_ras - XE DRM RAS structure
//
// This structure has details of error counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_drm_ras {
// @node: DRM RAS node
    pub node: *mut drm_ras_node,
// @info: info array for all types of errors
    pub info: [*mut xe_drm_ras_counter; DRM_XE_RAS_ERR_SEV_MAX],
}
