//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn42b/dcn42b_clk_mgr.h
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

// DCN42B reuses the following from DCN42:
// - dcn42_update_clocks (dtbclk_en=false so all dtbclk branches are skipped)
// - dcn42_get_dispclk_from_dentist (DENTIST_DISPCLK_CNTL has same DCN offset)
// - dcn42_get_dpm_table_from_smu (identical, only SMU calls)
// - dcn42_are_clock_states_equal
// - dcn42_enable_pme_wa
// - dcn42_update_clocks_update_dpp_dto
// - dcn42_update_clocks_update_dtb_dto
// - dcn42_build_watermark_ranges
// - dcn42_is_spll_ssc_enabled
// - dcn42_has_active_display
// - dcn42_notify_wm_ranges
// - dcn42_set_low_power_state
// - dcn42_exit_low_power_state
// - dcn42_get_max_clock_khz
// - dcn42_is_smu_present
//
// CANNOT reuse from DCN42 (hardware register differences):
// - dcn42_read_ss_info_from_lut (CLK8 vs CLK5 registers)
// - dcn42_dump_clk_registers* (CLK8 vs CLK5 registers)
// - dcn42_get_clock_freq_from_clkip (CLK8 vs CLK5 registers)
// - dcn42_init_clocks (calls CLK8-specific functions, dtbclk logic)
// - init_clk_states (dtbclk_en difference: true for dcn42, false for dcn42b)
//
// See dcn42_clk_mgr.h for declarations
//
pub const NUM_CLOCK_SOURCES: c_int = 5;
extern "C" {
    pub fn dcn42b_init_clocks(clk_mgr: *mut clk_mgr);
}
