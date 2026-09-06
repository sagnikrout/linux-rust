//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn31/dcn31_smu.h
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


//
// Copyright 2018 Advanced Micro Devices, Inc.
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
// Authors: AMD
//
pub const PMFW_DRIVER_IF_VERSION: c_int = 4;
pub const NUM_WM_RANGES: c_int = 4;
pub const WM_PSTATE_CHG: c_int = 0;
pub const WM_RETRAINING: c_int = 1;
// Watermarks
pub const NUM_DCFCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DISPCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DPPCLK_DPM_LEVELS: c_int = 8;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const NUM_VCN_DPM_LEVELS: c_int = 8;
pub const NUM_SOC_VOLTAGE_LEVELS: c_int = 8;
pub const NUM_DF_PSTATE_LEVELS: c_int = 4;
// Freq in MHz
// Voltage in milli volts with 2 fractional bits
// Throttler Status Bitmask
pub const THROTTLER_STATUS_BIT_SPL: c_int = 0;
pub const THROTTLER_STATUS_BIT_FPPT: c_int = 1;
pub const THROTTLER_STATUS_BIT_SPPT: c_int = 2;
pub const THROTTLER_STATUS_BIT_SPPT_APU: c_int = 3;
pub const THROTTLER_STATUS_BIT_THM_CORE: c_int = 4;
pub const THROTTLER_STATUS_BIT_THM_GFX: c_int = 5;
pub const THROTTLER_STATUS_BIT_THM_SOC: c_int = 6;
pub const THROTTLER_STATUS_BIT_TDC_VDD: c_int = 7;
pub const THROTTLER_STATUS_BIT_TDC_SOC: c_int = 8;
pub const THROTTLER_STATUS_BIT_PROCHOT_CPU: c_int = 9;
pub const THROTTLER_STATUS_BIT_PROCHOT_GFX: c_int = 10;
pub const THROTTLER_STATUS_BIT_EDC_CPU: c_int = 11;
pub const THROTTLER_STATUS_BIT_EDC_GFX: c_int = 12;
// 3rd party tools in Windows need this info in the case of APUs
// Workload bits
pub const WORKLOAD_PPLIB_FULL_SCREEN_3D_BIT: c_int = 0;
pub const WORKLOAD_PPLIB_VIDEO_BIT: c_int = 2;
pub const WORKLOAD_PPLIB_VR_BIT: c_int = 3;
pub const WORKLOAD_PPLIB_COMPUTE_BIT: c_int = 4;
pub const WORKLOAD_PPLIB_CUSTOM_BIT: c_int = 5;
pub const WORKLOAD_PPLIB_COUNT: c_int = 6;

pub const TABLE_SPARE1: c_int = 3;

pub const TABLE_COUNT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_watermarks {
// Watermarks
    pub WatermarkRow: [WatermarkRowGeneric_t; WM_COUNT][NUM_WM_RANGES],
    pub use: uint32_t MmHubPadding[7]; // SMU internal,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_smu_dpm_clks {
    pub dpm_clks: *mut DpmClocks_t,
    pub mc_address: large_integer,
}

// TODO: taken from vgh, may not be correct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_idle_optimization {
    pub 1: unsigned int df_request_disabled :,
    pub 1: unsigned int phy_ref_clk_off :,
    pub 1: unsigned int s0i2_rdy :,
    pub 29: unsigned int reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union display_idle_optimization_u {
    pub idle_info: display_idle_optimization,
    pub data: u32,
}

extern "C" {
    pub fn dcn31_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> c_int;
}
extern "C" {
    pub fn dcn31_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn31_smu_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> c_int;
}
extern "C" {
    pub fn dcn31_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn31_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn31_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn31_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32);
}
extern "C" {
    pub fn dcn31_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
extern "C" {
    pub fn dcn31_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn31_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
}
extern "C" {
    pub fn dcn31_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
}
extern "C" {
    pub fn dcn31_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn31_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn31_smu_set_zstate_support(clk_mgr: *mut clk_mgr_internal, support: dcn_zstate_support_state);
}
extern "C" {
    pub fn dcn31_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
