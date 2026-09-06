//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_klv_thresholds_set_types.h
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
// Copyright © 2024 Intel Corporation
//

//
// MAKE_XE_GUC_KLV_THRESHOLDS_SET - Generate various GuC thresholds definitions.
// @define: name of the inner macro to expand.
//
// The GuC firmware is able to monitor VF's adverse activity and will notify the
// PF driver once any threshold is exceeded.
//
// This super macro allows various conversions between the GuC adverse event
// threshold KLV definitions and the driver code without repeating similar code
// or risking missing some cases.
//
// For each GuC threshold definition, the inner macro &define will be provided
// with the &TAG, that corresponds to the GuC threshold KLV key name defined by
// ABI and the associated &NAME, that may be used in code or debugfs/sysfs::
//
// define(TAG, NAME)
//
// If required, KLVs can be labeled with GuC firmware version that added them::
//
// define(TAG, NAME, MAJOR, MINOR)
// define(TAG, NAME, MAJOR, MINOR, PATCH)
//

// end
//
// XE_GUC_KLV_NUM_THRESHOLDS - Number of GuC thresholds KLVs.
//
// Calculated automatically using &MAKE_XE_GUC_KLV_THRESHOLDS_SET.
//

//
// MAKE_XE_GUC_KLV_THRESHOLD_INDEX - Create enumerator name.
// @TAG: unique TAG of the enum xe_guc_klv_threshold_index.
//

//
// enum xe_guc_klv_threshold_index - Index of the tracked GuC threshold.
//
// This enum is automatically generated using &MAKE_XE_GUC_KLV_THRESHOLDS_SET.
// All these generated enumerators will only be used by the also generated code.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_guc_klv_threshold_index {

    \
    MAKE_XE_GUC_KLV_THRESHOLD_INDEX(TAG),

// private: auto-generated enum definitions
    MAKE_XE_GUC_KLV_THRESHOLDS_SET(define_xe_guc_klv_threshold_index_enum)

}
