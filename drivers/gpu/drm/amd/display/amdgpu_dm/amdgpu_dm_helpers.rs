//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_helpers.h
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
// Copyright 2026 Advanced Micro Devices, Inc.
//

// Exported for KUnit testing
extern "C" {
    pub fn edid_extract_panel_id(edid: *mut edid) -> u32;
}
extern "C" {
    pub fn get_max_frl_rate(max_lanes: u8, max_rate_per_lane: u8) -> u8;
}
extern "C" {
    pub fn get_dsc_max_slices(max_slices: u8, clk_per_slice: c_int) -> u8;
}
extern "C" {
    pub fn dm_is_freesync_pcon_whitelist(branch_dev_id: u32) -> bool;
}
extern "C" {
    pub fn dm_freesync_pcon_whitelist_count() -> u32;
}
extern "C" {
    pub fn dm_helpers_get_dc_debug_mask() -> c_uint;
}
extern "C" {
    pub fn dm_helpers_set_dc_debug_mask(debug_mask: c_uint);
}
extern "C" {
    pub fn dm_helpers_probe_acpi_edid(data: *mut c_void, buf: *mut u8, block: c_uint, len: usize) -> c_int;
}
extern "C" {
    pub fn apply_synaptics_fifo_reset_wa(aux: *mut drm_dp_aux);
}

