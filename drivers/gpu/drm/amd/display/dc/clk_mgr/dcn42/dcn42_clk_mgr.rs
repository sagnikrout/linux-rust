//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn42/dcn42_clk_mgr.h
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
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const NUM_CLOCK_SOURCES: c_int = 5;
pub const DCN42_CLKIP_REFCLK: c_int = 48000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_smu_watermark_set {
    pub wm_set: *mut dcn42_watermarks,
    pub mc_address: large_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_ss_info_table {
    pub ss_divider: u32,
    pub ss_percentage: [u32; NUM_CLOCK_SOURCES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_dcn42 {
    pub base: clk_mgr_internal,
    pub smu_wm_set: dcn42_smu_watermark_set,
}

extern "C" {
    pub fn dcn42_init_clocks(clk_mgr: *mut clk_mgr);
}
extern "C" {
    pub fn dcn42_clk_mgr_destroy(clk_mgr_int: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn42_convert_wck_ratio(wck_ratio: u8) -> c_uint;
}
extern "C" {
    pub fn dcn42_build_watermark_ranges(bw_params: *mut clk_bw_params, table: *mut dcn42_watermarks);
}
extern "C" {
    pub fn dcn42_enable_pme_wa(clk_mgr_base: *mut clk_mgr);
}
extern "C" {
    pub fn dcn42_notify_cstate_disable(clk_mgr_base: *mut clk_mgr, disable: bool);
}
extern "C" {
    pub fn dcn42_notify_wm_ranges(clk_mgr_base: *mut clk_mgr);
}
extern "C" {
    pub fn dcn42_set_low_power_state(clk_mgr_base: *mut clk_mgr);
}
extern "C" {
    pub fn dcn42_exit_low_power_state(clk_mgr_base: *mut clk_mgr);
}
extern "C" {
    pub fn dcn42_get_max_clock_khz(clk_mgr_base: *mut clk_mgr, clk_type: clk_type) -> c_uint;
}
extern "C" {
    pub fn dcn42_is_smu_present(clk_mgr_base: *mut clk_mgr) -> bool;
}
extern "C" {
    pub fn dcn42_has_active_display(dc: *mut dc, context: *const dc_state) -> bool;
}
extern "C" {
    pub fn dcn42_get_active_display_cnt_wa(dc: *mut dc, context: *mut dc_state, all_active_disps: *mut c_int) -> c_int;
}
extern "C" {
    pub fn dcn42_has_active_display(dc: *mut dc, context: *const dc_state) -> bool;
}
extern "C" {
    pub fn dcn42_update_clocks_update_dpp_dto(clk_mgr: *mut clk_mgr_internal, context: *mut dc_state, safe_to_lower: bool);
}
extern "C" {
    pub fn dcn42_update_clocks_update_dtb_dto(clk_mgr: *mut clk_mgr_internal, context: *mut dc_state, ref_dtbclk_khz: c_int);
}
extern "C" {
    pub fn dcn42_is_spll_ssc_enabled(clk_mgr_base: *mut clk_mgr) -> bool;
}
extern "C" {
    pub fn dcn42_get_dpm_table_from_smu(clk_mgr: *mut clk_mgr_internal, smu_dpm_clks: *mut dcn42_smu_dpm_clks);
}
extern "C" {
    pub fn dcn42_get_smu_clocks(clk_mgr_int: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn42_update_clocks_fpga(clk_mgr: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool);
}
extern "C" {
    pub fn dcn42_get_dispclk_from_dentist(clk_mgr_base: *mut clk_mgr) -> c_int;
}
extern "C" {
    pub fn dcn42_request_dtbclk(clk_mgr_base: *mut clk_mgr, enable: bool);
}
