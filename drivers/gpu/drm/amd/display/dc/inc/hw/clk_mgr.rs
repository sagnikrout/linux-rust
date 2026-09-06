//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/clk_mgr.h
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
// Copyright 2012-2026 Advanced Micro Devices, Inc.
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

// Constants
pub const DDR4_DRAM_WIDTH: c_int = 64;
pub const WM_A: c_int = 0;
pub const WM_B: c_int = 1;
pub const WM_C: c_int = 2;
pub const WM_D: c_int = 3;
pub const WM_SET_COUNT: c_int = 4;
pub const WM_1A: c_int = 2;
pub const WM_1B: c_int = 3;
pub const DCN_MINIMUM_DISPCLK_Khz: c_int = 100000;
pub const DCN_MINIMUM_DPPCLK_Khz: c_int = 100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn3_clk_internal {
    pub dummy: c_int,
// TODO:
    pub //dispclk: uint32_t CLK1_CLK0_CURRENT_CNT;,
    pub //dppclk: uint32_t CLK1_CLK1_CURRENT_CNT;,
    pub //dprefclk: uint32_t CLK1_CLK2_CURRENT_CNT;,
    pub //dcfclk: uint32_t CLK1_CLK3_CURRENT_CNT;,
    pub CLK1_CLK4_CURRENT_CNT: u32,
    pub //dcf_deep_sleep_divider: uint32_t CLK1_CLK3_DS_CNTL;,
    pub //dcf_deep_sleep_allow: uint32_t CLK1_CLK3_ALLOW_DS;,
    pub bypass: uint32_t CLK1_CLK0_BYPASS_CNTL; //dispclk,
    pub bypass: uint32_t CLK1_CLK1_BYPASS_CNTL; //dppclk,
    pub bypass: uint32_t CLK1_CLK2_BYPASS_CNTL; //dprefclk,
    pub bypass: uint32_t CLK1_CLK3_BYPASS_CNTL; //dcfclk,
    pub //fclk: uint32_t CLK4_CLK0_CURRENT_CNT;,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn35_clk_internal {
    pub dummy: c_int,
    pub //dispclk: uint32_t CLK1_CLK0_CURRENT_CNT;,
    pub //dppclk: uint32_t CLK1_CLK1_CURRENT_CNT;,
    pub //dprefclk: uint32_t CLK1_CLK2_CURRENT_CNT;,
    pub //dcfclk: uint32_t CLK1_CLK3_CURRENT_CNT;,
    pub //dtbclk: uint32_t CLK1_CLK4_CURRENT_CNT;,
// uint32_t CLK1_CLK5_CURRENT_CNT; //dpiaclk
// uint32_t CLK1_CLK6_CURRENT_CNT; //srdbgclk
    pub //dcf_deep_sleep_divider: uint32_t CLK1_CLK3_DS_CNTL;,
    pub //dcf_deep_sleep_allow: uint32_t CLK1_CLK3_ALLOW_DS;,
    pub bypass: uint32_t CLK1_CLK0_BYPASS_CNTL; //dispclk,
    pub bypass: uint32_t CLK1_CLK1_BYPASS_CNTL; //dppclk,
    pub bypass: uint32_t CLK1_CLK2_BYPASS_CNTL; //dprefclk,
    pub bypass: uint32_t CLK1_CLK3_BYPASS_CNTL; //dcfclk,
    pub bypass: uint32_t CLK1_CLK4_BYPASS_CNTL; //dtbclk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn301_clk_internal {
    pub dummy: c_int,
    pub //dispclk: uint32_t CLK1_CLK0_CURRENT_CNT;,
    pub //dppclk: uint32_t CLK1_CLK1_CURRENT_CNT;,
    pub //dprefclk: uint32_t CLK1_CLK2_CURRENT_CNT;,
    pub //dcfclk: uint32_t CLK1_CLK3_CURRENT_CNT;,
    pub //dcf_deep_sleep_divider: uint32_t CLK1_CLK3_DS_CNTL;,
    pub //dcf_deep_sleep_allow: uint32_t CLK1_CLK3_ALLOW_DS;,
    pub bypass: uint32_t CLK1_CLK0_BYPASS_CNTL; //dispclk,
    pub bypass: uint32_t CLK1_CLK1_BYPASS_CNTL; //dppclk,
    pub bypass: uint32_t CLK1_CLK2_BYPASS_CNTL; //dprefclk,
    pub bypass: uint32_t CLK1_CLK3_BYPASS_CNTL; //dcfclk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_clk_internal {
    pub dummy: c_int,
    pub //dispclk: uint32_t CLK8_CLK0_CURRENT_CNT;,
    pub //dppclk: uint32_t CLK8_CLK1_CURRENT_CNT;,
    pub //dprefclk: uint32_t CLK8_CLK2_CURRENT_CNT;,
    pub //dcfclk: uint32_t CLK8_CLK3_CURRENT_CNT;,
    pub //dtbclk: uint32_t CLK8_CLK4_CURRENT_CNT;,
    pub deep_sleep_divider: uint32_t CLK8_CLK0_DS_CNTL; //dispclk,
    pub deep_sleep_divider: uint32_t CLK8_CLK1_DS_CNTL; //dppclk,
    pub deep_sleep_divider: uint32_t CLK8_CLK2_DS_CNTL; //dprefclk,
    pub deep_sleep_divider: uint32_t CLK8_CLK3_DS_CNTL; //dcfclk,
    pub deep_sleep_divider: uint32_t CLK8_CLK4_DS_CNTL; //dtbclk,
    pub bypass: uint32_t CLK8_CLK0_BYPASS_CNTL; //dispclk,
    pub bypass: uint32_t CLK8_CLK1_BYPASS_CNTL; //dppclk,
    pub bypass: uint32_t CLK8_CLK2_BYPASS_CNTL; //dprefclk,
    pub bypass: uint32_t CLK8_CLK3_BYPASS_CNTL; //dcfclk,
    pub bypass: uint32_t CLK8_CLK4_BYPASS_CNTL; //dtbclk,
    pub CLK8_CLK_TICK_CNT__TIMER_THRESHOLD: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42b_clk_internal {
    pub dummy: c_int,
    pub //dispclk: uint32_t CLK5_CLK0_CURRENT_CNT;,
    pub //dppclk: uint32_t CLK5_CLK1_CURRENT_CNT;,
    pub //dprefclk: uint32_t CLK5_CLK2_CURRENT_CNT;,
    pub //dcfclk: uint32_t CLK5_CLK3_CURRENT_CNT;,
// uint32_t CLK5_CLK4_CURRENT_CNT; //dtbclk
    pub deep_sleep_divider: uint32_t CLK5_CLK0_DS_CNTL; //dispclk,
    pub deep_sleep_divider: uint32_t CLK5_CLK1_DS_CNTL; //dppclk,
    pub deep_sleep_divider: uint32_t CLK5_CLK2_DS_CNTL; //dprefclk,
    pub deep_sleep_divider: uint32_t CLK5_CLK3_DS_CNTL; //dcfclk,
    pub //dcf_deep_sleep_allow: uint32_t CLK5_CLK3_ALLOW_DS;,
// uint32_t CLK8_CLK4_DS_CNTL;	    //dtbclk deep_sleep_divider
    pub bypass: uint32_t CLK5_CLK0_BYPASS_CNTL; //dispclk,
    pub bypass: uint32_t CLK5_CLK1_BYPASS_CNTL; //dppclk,
    pub bypass: uint32_t CLK5_CLK2_BYPASS_CNTL; //dprefclk,
    pub bypass: uint32_t CLK5_CLK3_BYPASS_CNTL; //dcfclk,
// uint32_t CLK5_CLK4_BYPASS_CNTL; //dtbclk bypass
    pub CLK5_CLK_TICK_CNT__TIMER_THRESHOLD: u32,
}

// Will these bw structures be ASIC specific?
pub const MAX_NUM_DPM_LVL: c_int = 8;
pub const WM_SET_COUNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_type {
    CLK_TYPE_DCFCLK,
    CLK_TYPE_FCLK,
    CLK_TYPE_MCLK,
    CLK_TYPE_SOCCLK,
    CLK_TYPE_DTBCLK,
    CLK_TYPE_DISPCLK,
    CLK_TYPE_DPPCLK,
    CLK_TYPE_DSCCLK,
    CLK_TYPE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_limit_table_entry {
    pub /: *mut *mut unsigned int voltage; / milivolts withh 2 fractional bits,
    pub dcfclk_mhz: c_uint,
    pub fclk_mhz: c_uint,
    pub memclk_mhz: c_uint,
    pub socclk_mhz: c_uint,
    pub dtbclk_mhz: c_uint,
    pub dispclk_mhz: c_uint,
    pub dppclk_mhz: c_uint,
    pub phyclk_mhz: c_uint,
    pub phyclk_d18_mhz: c_uint,
    pub wck_ratio: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_limit_num_entries {
    pub num_dcfclk_levels: c_uint,
    pub num_fclk_levels: c_uint,
    pub num_memclk_levels: c_uint,
    pub num_socclk_levels: c_uint,
    pub num_dtbclk_levels: c_uint,
    pub num_dispclk_levels: c_uint,
    pub num_dppclk_levels: c_uint,
    pub num_phyclk_levels: c_uint,
    pub num_phyclk_d18_levels: c_uint,
}

// This table is contiguous
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_limit_table {
    pub entries: [clk_limit_table_entry; MAX_NUM_DPM_LVL],
    pub num_entries_per_clk: clk_limit_num_entries,
    pub /: *mut *mut unsigned int num_entries; / highest populated dpm level for back compatibility,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm_range_table_entry {
    pub wm_inst: c_uint,
    pub wm_type: c_uint,
    pub pstate_latency_us: double,
    pub sr_exit_time_us: double,
    pub sr_enter_plus_exit_time_us: double,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_wm_range_entry {
    pub valid: bool,
    pub wm_type: u8,
    pub min_dcfclk: u16,
    pub max_dcfclk: u16,
    pub min_uclk: u16,
    pub max_uclk: u16,
    pub pmfw_breakdown: },
    pub pstate_latency_us: double,
    pub sr_exit_time_us: double,
    pub sr_enter_plus_exit_time_us: double,
    pub fclk_change_latency_us: double,
    pub dml_input: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_log_info {
    pub enabled: bool,
    pub pBuf: *mut c_char,
    pub bufSize: c_uint,
    pub sum_chars_printed: *mut c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_state_registers_and_bypass {
    pub dcfclk: u32,
    pub dcf_deep_sleep_divider: u32,
    pub dcf_deep_sleep_allow: u32,
    pub dprefclk: u32,
    pub dispclk: u32,
    pub dppclk: u32,
    pub dtbclk: u32,
    pub fclk: u32,
    pub dppclk_bypass: u32,
    pub dcfclk_bypass: u32,
    pub dprefclk_bypass: u32,
    pub dispclk_bypass: u32,
    pub timer_threshold: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv1_clk_internal {
    pub //dcfclk: uint32_t CLK0_CLK8_CURRENT_CNT;,
    pub //dcf_deep_sleep_divider: uint32_t CLK0_CLK8_DS_CNTL;,
    pub //dcf_deep_sleep_allow: uint32_t CLK0_CLK8_ALLOW_DS;,
    pub //dprefclk: uint32_t CLK0_CLK10_CURRENT_CNT;,
    pub //dispclk: uint32_t CLK0_CLK11_CURRENT_CNT;,
    pub bypass: uint32_t CLK0_CLK8_BYPASS_CNTL; //dcfclk,
    pub bypass: uint32_t CLK0_CLK10_BYPASS_CNTL; //dprefclk,
    pub bypass: uint32_t CLK0_CLK11_BYPASS_CNTL; //dispclk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rn_clk_internal {
    pub //dispclk: uint32_t CLK1_CLK0_CURRENT_CNT;,
    pub //dppclk: uint32_t CLK1_CLK1_CURRENT_CNT;,
    pub //dprefclk: uint32_t CLK1_CLK2_CURRENT_CNT;,
    pub //dcfclk: uint32_t CLK1_CLK3_CURRENT_CNT;,
    pub //dcf_deep_sleep_divider: uint32_t CLK1_CLK3_DS_CNTL;,
    pub //dcf_deep_sleep_allow: uint32_t CLK1_CLK3_ALLOW_DS;,
    pub bypass: uint32_t CLK1_CLK0_BYPASS_CNTL; //dispclk,
    pub bypass: uint32_t CLK1_CLK1_BYPASS_CNTL; //dppclk,
    pub bypass: uint32_t CLK1_CLK2_BYPASS_CNTL; //dprefclk,
    pub bypass: uint32_t CLK1_CLK3_BYPASS_CNTL; //dcfclk,
}

// For dtn logging and debugging
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_state_registers {
    pub //dcfclk: uint32_t CLK0_CLK8_CURRENT_CNT;,
    pub //dcf_deep_sleep_divider: uint32_t CLK0_CLK8_DS_CNTL;,
    pub //dcf_deep_sleep_allow: uint32_t CLK0_CLK8_ALLOW_DS;,
    pub //dprefclk: uint32_t CLK0_CLK10_CURRENT_CNT;,
    pub //dispclk: uint32_t CLK0_CLK11_CURRENT_CNT;,
}

// TODO: combine this with the above
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_bypass {
    pub dcfclk_bypass: u32,
    pub dispclk_pypass: u32,
    pub dprefclk_bypass: u32,
}

//
// This table is not contiguous, can have holes, each
// entry correspond to one set of WM. For example if
// we have 2 DPM and LPDDR, we will WM set A, B and
// D occupied, C will be emptry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm_table {
    pub nv_entries: [nv_wm_range_entry; WM_SET_COUNT],
    pub entries: [wm_range_table_entry; WM_SET_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy_pstate_entry {
    pub dram_speed_mts: c_uint,
    pub dummy_pstate_latency_us: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_bw_params {
    pub vram_type: c_uint,
    pub num_channels: c_uint,
    pub dram_channel_width_bytes: c_uint,
    pub dispclk_vco_khz: c_uint,
    pub dc_mode_softmax_memclk: c_uint,
    pub max_memclk_mhz: c_uint,
    pub clk_table: clk_limit_table,
    pub wm_table: wm_table,
    pub dummy_pstate_table: [dummy_pstate_entry; 4],
    pub dc_mode_limit: clk_limit_table_entry,
    pub utm_qos_model: *const utm_qos_model,
}

// Public interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_states {
    pub dprefclk_khz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_funcs {
//
// This function should set new clocks based on the input "safe_to_lower".
// If safe_to_lower == false, then only clocks which are to be increased
// should changed.
// If safe_to_lower == true, then only clocks which are to be decreased
// should be changed.
//
    pub safe_to_lower): bool,
    pub clk_mgr): *mut *mut int (get_dp_ref_clk_frequency)(struct clk_mgr,
    pub clk_mgr): *mut *mut int (get_dtb_ref_clk_frequency)(struct clk_mgr,
    pub clk_mgr): *mut *mut void (set_low_power_state)(struct clk_mgr,
    pub clk_mgr): *mut *mut void (exit_low_power_state)(struct clk_mgr,
    pub clk_mgr): *mut *mut bool (is_ips_supported)(struct clk_mgr,
    pub enable): *mut *mut *mut void (set_idle_power_optimizations)(struct clk_mgr clk_mgr, bool,
    pub clk_mgr): *mut *mut void (init_clocks)(struct clk_mgr,
    pub log_info): *mut *mut clk_mgr clk_mgr_base, clk_log_info,
    pub clk_mgr): *mut *mut void (enable_pme_wa) (struct clk_mgr,
    pub clock_cfg): *mut dc_clock_config,
    pub b): *mut dc_clocks,
    pub clk_mgr): *mut *mut void (notify_wm_ranges)(struct clk_mgr,
// Notify clk_mgr of a change in link rate, update phyclk frequency if necessary
    pub link): *mut *mut *mut void (notify_link_rate_change)(struct clk_mgr clk_mgr, struct dc_link,
//
// Send message to PMFW to set hard min memclk frequency
// When current_mode = false, set DPM0
// When current_mode = true, set required clock for current mode
//
    pub current_mode): *mut *mut *mut void (set_hard_min_memclk)(struct clk_mgr clk_mgr, bool,
    pub clk_mgr): *mut *mut int (get_hard_min_memclk)(struct clk_mgr,
    pub clk_mgr): *mut *mut int (get_hard_min_fclk)(struct clk_mgr,
// Send message to PMFW to set hard max memclk frequency to highest DPM
    pub clk_mgr): *mut *mut void (set_hard_max_memclk)(struct clk_mgr,
// Custom set a memclk freq range
    pub memclk_mhz): *mut *mut *mut void (set_max_memclk)(struct clk_mgr clk_mgr, unsigned int,
    pub memclk_mhz): *mut *mut *mut void (set_min_memclk)(struct clk_mgr clk_mgr, unsigned int,
// Get current memclk states from PMFW, update relevant structures
    pub clk_mgr): *mut *mut void (get_memclk_states_from_smu)(struct clk_mgr,
// Get SMU present
    pub clk_mgr): *mut *mut bool (is_smu_present)(struct clk_mgr,
    pub clk_mgr_base): *mut *mut int (get_dispclk_from_dentist)(struct clk_mgr,
    pub clk_mgr): *mut *mut bool (is_dc_mode_present)(struct clk_mgr,
    pub pins_to_set): *mut *mut *mut uint32_t (set_smartmux_switch)(struct clk_mgr clk_mgr, uint32_t,
    pub clk_type): *mut *mut *mut unsigned int (get_max_clock_khz)(struct clk_mgr clk_mgr_base, enum clk_type,
//
// override_memory_bandwidth_request - Override the DCN nominal memory
// bandwidth request sent to PMFW, independent of the current display
// mode. For debug use only.
// @clk_mgr: clock manager instance
// @bw_kbps: requested bandwidth in kbps; 0 clears the override
//
// Return: capped bandwidth value actually applied (kbps)
//
    pub bw_kbps): c_uint,
//
// get_requested_memory_qos - Retrieve current QoS request from the clock manager's
// current clock state, reflecting any active bandwidth overrides.
// @clk_mgr: clock manager instance
// @qos: pointer to dc_requested_memory_qos structure to populate
//
    pub qos): *mut dc_requested_memory_qos,
//
// notify_cstate_disable - Vote DCN's DF C-state policy to PMFW.
// @disable: true  -> vote "disable"
// false -> vote "allow"
// Sends the message only when the cached dc_clocks.cstate_allow would
// change, then updates the cache on an OK response (idempotent no-op
// otherwise).
//
    pub disable): *mut *mut *mut void (notify_cstate_disable)(struct clk_mgr clk_mgr, bool,
    pub seq_state): *mut block_sequence_state,
    pub clk_mgr): *mut *mut void (execute_clk_mgr_block_sequence)(struct clk_mgr,
    pub enable): *mut *mut *mut void (request_dtbclk)(struct clk_mgr clk_mgr_base, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr {
    pub ctx: *mut dc_context,
    pub funcs: *mut clk_mgr_funcs,
    pub clks: dc_clocks,
    pub psr_allow_active_cache: bool,
    pub force_smu_not_present: bool,
    pub dc_mode_softmax_enabled: bool,
    pub goes: int dprefclk_khz; // Used by program pixel clock in clock source funcs, need to figureout where this,
    pub DCN314: int dp_dto_source_clock_in_khz; // Used to program DP DTO with ss adjustment on,
    pub dentist_vco_freq_khz: c_int,
    pub boot_snapshot: clk_state_registers_and_bypass,
    pub bw_params: *mut clk_bw_params,
    pub ranges: pp_smu_wm_range_sets,
}

// forward declarations
extern "C" {
    pub fn dc_destroy_clk_mgr(clk_mgr: *mut clk_mgr);
}
extern "C" {
    pub fn clk_mgr_exit_optimized_pwr_state(dc: *const dc, clk_mgr: *mut clk_mgr);
}
extern "C" {
    pub fn clk_mgr_optimize_pwr_state(dc: *const dc, clk_mgr: *mut clk_mgr);
}
