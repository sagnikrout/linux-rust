//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_eld.h
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
// Copyright © 2023 Intel Corporation
//

// ELD Header Block
pub const DRM_ELD_HEADER_BLOCK_SIZE: c_int = 4;
pub const DRM_ELD_VER: c_int = 0;

// ELD Baseline Block for ELD_Ver == 2
pub const DRM_ELD_CEA_EDID_VER_MNL: c_int = 4;

pub const DRM_ELD_SAD_COUNT_CONN_TYPE: c_int = 5;

pub const DRM_ELD_SPEAKER: c_int = 7;

pub const DRM_ELD_MANUFACTURER_NAME0: c_int = 16;
pub const DRM_ELD_MANUFACTURER_NAME1: c_int = 17;
pub const DRM_ELD_PRODUCT_CODE0: c_int = 18;
pub const DRM_ELD_PRODUCT_CODE1: c_int = 19;

//
// drm_eld_mnl - Get ELD monitor name length in bytes.
// @eld: pointer to an eld memory structure with mnl set
//
extern "C" {
    pub fn drm_eld_sad_get(eld: *const u8, sad_index: c_int, cta_sad: *mut cea_sad) -> c_int;
}
extern "C" {
    pub fn drm_eld_sad_set(eld: *mut u8, sad_index: c_int, cta_sad: *const cea_sad) -> c_int;
}
//
// drm_eld_sad - Get ELD SAD structures.
// @eld: pointer to an eld memory structure with sad_count set
//
// drm_eld_sad_count - Get ELD SAD count.
// @eld: pointer to an eld memory structure with sad_count set
//
// drm_eld_calc_baseline_block_size - Calculate baseline block size in bytes
// @eld: pointer to an eld memory structure with mnl and sad_count set
//
// This is a helper for determining the payload size of the baseline block, in
// bytes, for e.g. setting the Baseline_ELD_Len field in the ELD header block.
//
// drm_eld_size - Get ELD size in bytes
// @eld: pointer to a complete eld memory structure
//
// The returned value does not include the vendor block. It's vendor specific,
// and comprises of the remaining bytes in the ELD memory buffer after
// drm_eld_size() bytes of header and baseline block.
//
// The returned value is guaranteed to be a multiple of 4.
//
// drm_eld_get_spk_alloc - Get speaker allocation
// @eld: pointer to an ELD memory structure
//
// The returned value is the speakers mask. User has to use %DRM_ELD_SPEAKER
// field definitions to identify speakers.
//
// drm_eld_get_conn_type - Get device type hdmi/dp connected
// @eld: pointer to an ELD memory structure
//
// The caller need to use %DRM_ELD_CONN_TYPE_HDMI or %DRM_ELD_CONN_TYPE_DP to
// identify the display type connected.
//
