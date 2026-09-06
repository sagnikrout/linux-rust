//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/clk_mgr_internal.h
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
// Copyright 2018-2026 Advanced Micro Devices, Inc.
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

//
// only thing needed from here is MEMORY_TYPE_MULTIPLIER_CZ, which is also
// used in resource, perhaps this should be defined somewhere more common.
//

// Starting DID for each range
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dentist_base_divider_id {
    DENTIST_BASE_DID_1 = 0x08,
    DENTIST_BASE_DID_2 = 0x40,
    DENTIST_BASE_DID_3 = 0x60,
    DENTIST_BASE_DID_4 = 0x7e,
    DENTIST_MAX_DID = 0x7f
}

// Starting point and step size for each divider range.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dentist_divider_range {
    DENTIST_DIVIDER_RANGE_1_START = 8,   /* 2.00  */
    DENTIST_DIVIDER_RANGE_1_STEP  = 1,   /* 0.25  */
    DENTIST_DIVIDER_RANGE_2_START = 64,  /* 16.00 */
    DENTIST_DIVIDER_RANGE_2_STEP  = 2,   /* 0.50  */
    DENTIST_DIVIDER_RANGE_3_START = 128, /* 32.00 */
    DENTIST_DIVIDER_RANGE_3_STEP  = 4,   /* 1.00  */
    DENTIST_DIVIDER_RANGE_4_START = 248, /* 62.00 */
    DENTIST_DIVIDER_RANGE_4_STEP  = 264, /* 66.00 */
    DENTIST_DIVIDER_RANGE_SCALE_FACTOR = 4
}

//
// Clock Manager Private Macros and Defines
//
// Macros
// Macro flag: #define TO_CLK_MGR_INTERNAL(clk_mgr)\

//
// Clock Manager Private Structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_registers {
    pub DPREFCLK_CNTL: u32,
    pub DENTIST_DISPCLK_CNTL: u32,
    pub CLK4_CLK2_CURRENT_CNT: u32,
    pub CLK4_CLK_PLL_REQ: u32,
    pub CLK4_CLK0_CURRENT_CNT: u32,
    pub CLK3_CLK2_DFS_CNTL: u32,
    pub CLK3_CLK_PLL_REQ: u32,
    pub CLK0_CLK2_DFS_CNTL: u32,
    pub CLK0_CLK_PLL_REQ: u32,
    pub CLK1_CLK_PLL_REQ: u32,
    pub CLK1_CLK0_DFS_CNTL: u32,
    pub CLK1_CLK1_DFS_CNTL: u32,
    pub CLK1_CLK2_DFS_CNTL: u32,
    pub CLK1_CLK3_DFS_CNTL: u32,
    pub CLK1_CLK4_DFS_CNTL: u32,
    pub CLK1_CLK5_DFS_CNTL: u32,
    pub CLK2_CLK2_DFS_CNTL: u32,
    pub CLK1_CLK0_CURRENT_CNT: u32,
    pub CLK1_CLK1_CURRENT_CNT: u32,
    pub CLK1_CLK2_CURRENT_CNT: u32,
    pub CLK1_CLK3_CURRENT_CNT: u32,
    pub CLK1_CLK4_CURRENT_CNT: u32,
    pub CLK1_CLK5_CURRENT_CNT: u32,
    pub CLK0_CLK0_DFS_CNTL: u32,
    pub CLK0_CLK1_DFS_CNTL: u32,
    pub CLK0_CLK3_DFS_CNTL: u32,
    pub CLK0_CLK4_DFS_CNTL: u32,
    pub CLK1_CLK0_BYPASS_CNTL: u32,
    pub CLK1_CLK1_BYPASS_CNTL: u32,
    pub CLK1_CLK2_BYPASS_CNTL: u32,
    pub CLK1_CLK3_BYPASS_CNTL: u32,
    pub CLK1_CLK4_BYPASS_CNTL: u32,
    pub CLK1_CLK5_BYPASS_CNTL: u32,
    pub CLK1_CLK0_DS_CNTL: u32,
    pub CLK1_CLK1_DS_CNTL: u32,
    pub CLK1_CLK2_DS_CNTL: u32,
    pub CLK1_CLK3_DS_CNTL: u32,
    pub CLK1_CLK4_DS_CNTL: u32,
    pub CLK1_CLK5_DS_CNTL: u32,
    pub CLK1_CLK0_ALLOW_DS: u32,
    pub CLK1_CLK1_ALLOW_DS: u32,
    pub CLK1_CLK2_ALLOW_DS: u32,
    pub CLK1_CLK3_ALLOW_DS: u32,
    pub CLK1_CLK4_ALLOW_DS: u32,
    pub CLK1_CLK5_ALLOW_DS: u32,
    pub CLK5_spll_field_8: u32,
    pub CLK6_spll_field_8: u32,
    pub CLK8_CLK0_ALLOW_DS: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_mask {
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clock_type {
    clock_type_dispclk = 1,
    clock_type_dcfclk,
    clock_type_socclk,
    clock_type_pixelclk,
    clock_type_phyclk,
    clock_type_dppclk,
    clock_type_fclk,
    clock_type_dcfdsclk,
    clock_type_dscclk,
    clock_type_uclk,
    clock_type_dramclk,
    clock_type_dprefclk,
    clock_type_dtbclk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_internal {
    pub base: clk_mgr,
    pub smu_ver: c_int,
    pub pp_smu: *mut pp_smu_funcs,
    pub funcs: *mut clk_mgr_internal_funcs,
    pub dccg: *mut dccg,
//
// For backwards compatbility with previous implementation
// TODO: remove these after everything transitions to new pattern
// Rationale is that clk registers change a lot across DCE versions
// and a shared data structure doesn't really make sense.
//
    pub regs: *const clk_mgr_registers,
    pub clk_mgr_shift: *const clk_mgr_shift,
    pub clk_mgr_mask: *const clk_mgr_mask,
// TODO: figure out which of the below fields should be here vs in asic specific portion
// Cache the status of DFS-bypass feature
    pub dfs_bypass_enabled: bool,
// True if the DFS-bypass feature is enabled and active.
    pub dfs_bypass_active: bool,
    pub dfs_ref_freq_khz: u32,
//
// Cache the display clock returned by VBIOS if DFS-bypass is enabled.
// This is basically "Crystal Frequency In KHz" (XTALIN) frequency
//
    pub dfs_bypass_disp_clk: c_int,
//
// @ss_on_dprefclk:
//
// True if spread spectrum is enabled on the DP ref clock.
//
    pub ss_on_dprefclk: bool,
//
// @xgmi_enabled:
//
// True if xGMI is enabled. On VG20, both audio and display clocks need
// to be adjusted with the WAFL link's SS info if xGMI is enabled.
//
    pub xgmi_enabled: bool,
//
// @dprefclk_ss_percentage:
//
// DPREFCLK SS percentage (if down-spread enabled).
//
// Note that if XGMI is enabled, the SS info (percentage and divider)
// from the WAFL link is used instead. This is decided during
// dce_clk_mgr initialization.
//
    pub dprefclk_ss_percentage: c_int,
//
// @dprefclk_ss_divider:
//
// DPREFCLK SS percentage Divider (100 or 1000).
//
    pub dprefclk_ss_divider: c_int,
    pub periodic_retraining_disabled: bool,
    pub cur_phyclk_req_table: [c_uint; MAX_LINKS],
    pub smu_present: bool,
    pub wm_range_table: *mut c_void,
    pub wm_range_table_addr: c_longlong,
//
// @dal_init_table:
//
// GPU-accessible DRAM buffer for the DAL init table transferred
// from PMFW via DALSMC_MSG_TransferTableSmu2Dram(TABLE_DAL_INIT).
// Contains all static PMFW data needed at init: DPM clock tables,
// UTM QoS parameters, and memory configuration.
//
    pub dal_init_table: *const c_void,
    pub dal_init_table_addr: c_longlong,
    pub dpm_present: bool,
    pub pme_trigger_pending: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_internal_funcs {
    pub requested_dispclk_khz): *mut *mut *mut int (set_dispclk)(struct clk_mgr_internal clk_mgr, int,
    pub clk_mgr): *mut *mut int (set_dprefclk)(struct clk_mgr_internal,
}

//
// Clock Manager Level Helper functions
//
