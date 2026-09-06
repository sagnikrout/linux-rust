//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn301/dcn301_smu.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
pub const SMU13_DRIVER_IF_VERSION: c_int = 2;
pub const NUM_WM_RANGES: c_int = 4;
// Watermarks
pub const TABLE_WATERMARKS: c_int = 1;

pub const VG_NUM_DCFCLK_DPM_LEVELS: c_int = 7;
pub const VG_NUM_DISPCLK_DPM_LEVELS: c_int = 7;
pub const VG_NUM_DPPCLK_DPM_LEVELS: c_int = 7;
pub const VG_NUM_SOCCLK_DPM_LEVELS: c_int = 7;
pub const VG_NUM_ISPICLK_DPM_LEVELS: c_int = 7;
pub const VG_NUM_ISPXCLK_DPM_LEVELS: c_int = 7;
pub const VG_NUM_VCN_DPM_LEVELS: c_int = 5;
pub const VG_NUM_FCLK_DPM_LEVELS: c_int = 4;
pub const VG_NUM_SOC_VOLTAGE_LEVELS: c_int = 8;
// copy from vgh/vangogh/pmfw_driver_if.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vg_dpm_clocks {
    pub DcfClocks: [u32; VG_NUM_DCFCLK_DPM_LEVELS],
    pub DispClocks: [u32; VG_NUM_DISPCLK_DPM_LEVELS],
    pub DppClocks: [u32; VG_NUM_DPPCLK_DPM_LEVELS],
    pub SocClocks: [u32; VG_NUM_SOCCLK_DPM_LEVELS],
    pub IspiClocks: [u32; VG_NUM_ISPICLK_DPM_LEVELS],
    pub IspxClocks: [u32; VG_NUM_ISPXCLK_DPM_LEVELS],
    pub VcnClocks: [vcn_clk_t; VG_NUM_VCN_DPM_LEVELS],
    pub SocVoltage: [u32; VG_NUM_SOC_VOLTAGE_LEVELS],
    pub DfPstateTable: [df_pstate_t; VG_NUM_FCLK_DPM_LEVELS],
    pub MinGfxClk: u32,
    pub MaxGfxClk: u32,
    pub NumDfPstatesEnabled: u8,
    pub NumDcfclkLevelsEnabled: u8,
    pub dppclk: uint8_t NumDispClkLevelsEnabled; //applies to both dispclk and,
    pub NumSocClkLevelsEnabled: u8,
    pub ispxclk: uint8_t IspClkLevelsEnabled; //applies to both ispiclk and,
    pub vclk/dclk: uint8_t VcnClkLevelsEnabled; //applies to both,
    pub spare: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_dpm_clks {
    pub dpm_clks: *mut vg_dpm_clocks,
    pub mc_address: large_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watermarks {
// Watermarks
    pub WatermarkRow: [WatermarkRowGeneric_t; WM_COUNT][NUM_WM_RANGES],
    pub use: uint32_t MmHubPadding[7]; // SMU internal,
}

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
    pub fn dcn301_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> c_int;
}
extern "C" {
    pub fn dcn301_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn301_smu_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> c_int;
}
extern "C" {
    pub fn dcn301_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn301_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn301_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: c_int) -> c_int;
}
extern "C" {
    pub fn dcn301_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32);
}
extern "C" {
    pub fn dcn301_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
extern "C" {
    pub fn dcn301_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn301_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
}
extern "C" {
    pub fn dcn301_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
}
extern "C" {
    pub fn dcn301_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn301_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
}
