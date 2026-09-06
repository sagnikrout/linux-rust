//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_device.h
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

//
// Display platforms and subplatforms. Keep platforms in display version based
// order, chronological order within a version, and subplatforms next to the
// platform.
//

// Platform group aliases */ \
// Display ver 2 */ \
// Display ver 3 */ \
// Display ver 4 */ \
// Display ver 5 */ \
// Display ver 6 */ \
// Display ver 7 */ \
// Display ver 8 */ \
// Display ver 9 */ \
// Display ver 11 */ \
// Display ver 12 */ \
// Display ver 13 */ \
// Display ver 14 (based on GMD ID) */ \
// Display ver 20 (based on GMD ID) */ \
// Display ver 14.1 (based on GMD ID) */ \
// Display ver 30 (based on GMD ID) */ \
// Display ver 35 (based on GMD ID) */ \

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_platforms {
}

// Keep in alphabetical order */ \

// Check that device has a display IP version within the specific range.

//
// Check if a device has a specific IP version as well as a stepping within the
// specified range [from, until).  The lower bound is inclusive, the upper
// bound is exclusive.  The most common use-case of this macro is for checking
// bounds for workarounds, which usually have a stepping ("from") at which the
// hardware issue is first present and another stepping ("until") at which a
// hardware fix is present and the software workaround is no longer necessary.
// E.g.,
//
// IS_DISPLAY_VERx100_STEP(display, 1400, STEP_A0, STEP_B2)
// IS_DISPLAY_VERx100_STEP(display, 1400, STEP_C0, STEP_FOREVER)
//
// "STEP_FOREVER" can be passed as "until" for workarounds that have no upper
// stepping bound for the specified IP version.
//

pub const ARLS_HOST_BRIDGE_PCI_ID1: c_uint = 0x7D1C;
pub const ARLS_HOST_BRIDGE_PCI_ID2: c_uint = 0x7D2D;
pub const ARLS_HOST_BRIDGE_PCI_ID3: c_uint = 0x7D2E;
pub const ARLS_HOST_BRIDGE_PCI_ID4: c_uint = 0x7D2F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_runtime_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_ip_ver {
    pub ver: u16,
    pub rel: u16,
    pub /: *mut *mut u16 step; / hardware,
    pub ip: },
    pub /: *mut *mut int step; / symbolic,
    pub /: *mut *mut char step_name[3]; / empty string if not applicable,
    pub rawclk_freq: u32,
    pub pipe_mask: u8,
    pub cpu_transcoder_mask: u16,
    pub port_mask: u16,
    pub num_sprites: [u8; I915_MAX_PIPES],
    pub num_scalers: [u8; I915_MAX_PIPES],
    pub fbc_mask: u8,
    pub has_hdcp: bool,
    pub has_dmc: bool,
    pub has_dsc: bool,
    pub edp_typec_support: bool,
    pub has_dbuf_overlap_detection: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_device_info {
// Initial runtime info.
    pub __runtime_defaults: intel_display_runtime_info,
    pub abox_mask: u8,
    pub /: *mut *mut u16 size; / in blocks,
    pub slice_mask: u8,
    pub dbuf: },

// Global register offset for the display engine
    pub mmio_offset: u32,
// Register offsets for the various display pipes and transcoders
    pub pipe_offsets: [u32; I915_MAX_TRANSCODERS],
    pub trans_offsets: [u32; I915_MAX_TRANSCODERS],
    pub cursor_offsets: [u32; I915_MAX_PIPES],
    pub degamma_lut_size: u32,
    pub gamma_lut_size: u32,
    pub degamma_lut_tests: u32,
    pub gamma_lut_tests: u32,
    pub color: },
}

extern "C" {
    pub fn intel_display_device_present(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_display_device_enabled(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_display_device_remove(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_device_info_runtime_init(display: *mut intel_display);
}
