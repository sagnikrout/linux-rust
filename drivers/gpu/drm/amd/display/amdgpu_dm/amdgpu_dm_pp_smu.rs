//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_pp_smu.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

extern "C" {
    pub fn amdgpu_dm_smu_write_watermarks_table(adev: *mut amdgpu_device) -> c_int;
}

extern "C" {
    pub fn get_default_clock_levels(clk_type: dm_pp_clock_type, clks: *mut dm_pp_clock_levels);
}
extern "C" {
    pub fn dc_to_pp_clock_type(dm_pp_clk_type: dm_pp_clock_type) -> amd_pp_clock_type;
}
extern "C" {
    pub fn pp_rv_set_wm_ranges(pp: *mut pp_smu, ranges: *mut pp_smu_wm_range_sets);
}
extern "C" {
    pub fn pp_rv_set_pme_wa_enable(pp: *mut pp_smu);
}
extern "C" {
    pub fn pp_rv_set_active_display_count(pp: *mut pp_smu, count: c_int);
}
extern "C" {
    pub fn pp_rv_set_min_deep_sleep_dcfclk(pp: *mut pp_smu, clock: c_int);
}
extern "C" {
    pub fn pp_rv_set_hard_min_dcefclk_by_freq(pp: *mut pp_smu, clock: c_int);
}
extern "C" {
    pub fn pp_rv_set_hard_min_fclk_by_freq(pp: *mut pp_smu, mhz: c_int);
}
extern "C" {
    pub fn pp_nv_set_display_count(pp: *mut pp_smu, count: c_int) -> pp_smu_status;
}
extern "C" {
    pub fn pp_nv_set_min_deep_sleep_dcfclk(pp: *mut pp_smu, mhz: c_int) -> pp_smu_status;
}
extern "C" {
    pub fn pp_nv_set_hard_min_dcefclk_by_freq(pp: *mut pp_smu, mhz: c_int) -> pp_smu_status;
}
extern "C" {
    pub fn pp_nv_set_hard_min_uclk_by_freq(pp: *mut pp_smu, mhz: c_int) -> pp_smu_status;
}

