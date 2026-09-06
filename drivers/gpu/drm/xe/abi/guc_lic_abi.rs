//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/guc_lic_abi.h
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

//
// enum guc_lic_type - Log Init Config KLV IDs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_lic_type {
//
// @GUC_LIC_TYPE_GUC_SW_VERSION: GuC firmware version. Value
// is a 32 bit number represented by guc_sw_version.
//
    GUC_LIC_TYPE_GUC_SW_VERSION = 0x1,
//
// @GUC_LIC_TYPE_GUC_DEVICE_ID: GuC device id. Value is a 32
// bit.
//
    GUC_LIC_TYPE_GUC_DEVICE_ID = 0x2,
//
// @GUC_LIC_TYPE_TSC_FREQUENCY: GuC timestamp counter
// frequency. Value is a 32 bit number representing frequency in
// kHz. This timestamp is utilized in log entries, timer and
// for engine utilization tracking.
//
    GUC_LIC_TYPE_TSC_FREQUENCY = 0x3,
//
// @GUC_LIC_TYPE_GMD_ID: HW GMD ID. Value is a 32 bit number
// representing graphics, media and display HW architecture IDs.
//
    GUC_LIC_TYPE_GMD_ID = 0x4,
//
// @GUC_LIC_TYPE_BUILD_PLATFORM_ID: GuC build platform ID.
// Value is 32 bits.
//
    GUC_LIC_TYPE_BUILD_PLATFORM_ID = 0x5,
}

//
// struct guc_lic - GuC LIC (Log-Init-Config) structure.
//
// This is populated by the GUC at log init time and is located in the log
// buffer memory allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_lic {
//
// @magic: A magic number set by GuC to identify that this
// structure contains valid information: magic = GUC_LIC_MAGIC.
//
    pub magic: u32,
pub const GUC_LIC_MAGIC: c_uint = 0x8086900D;
//
// @version: The version of the this structure.
// Major and minor version number are represented as bit fields.
//
    pub version: u32,

// @data_count: Number of dwords the `data` array contains.
    pub data_count: u32,
//
// @data: Array of dwords representing a list of LIC KLVs of
// type guc_klv_generic with keys represented by guc_lic_type
//
    pub __counted_by(data_count): u32 data[],
    pub __packed: },
