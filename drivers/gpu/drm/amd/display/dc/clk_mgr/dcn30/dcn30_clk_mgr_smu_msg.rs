//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn30/dcn30_clk_mgr_smu_msg.h
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

extern "C" {
    pub fn dcn30_smu_test_message(clk_mgr: *mut clk_mgr_internal, input: u32) -> bool;
}
extern "C" {
    pub fn dcn30_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal, version: *mut c_int) -> bool;
}
extern "C" {
    pub fn dcn30_smu_check_driver_if_version(clk_mgr: *mut clk_mgr_internal) -> bool;
}
extern "C" {
    pub fn dcn30_smu_check_msg_header_version(clk_mgr: *mut clk_mgr_internal) -> bool;
}
extern "C" {
    pub fn dcn30_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
}
extern "C" {
    pub fn dcn30_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
}
extern "C" {
    pub fn dcn30_smu_transfer_wm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn30_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn30_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> c_uint;
}
extern "C" {
    pub fn dcn30_smu_set_hard_max_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> c_uint;
}
extern "C" {
    pub fn dcn30_smu_get_dpm_freq_by_index(clk_mgr: *mut clk_mgr_internal, clk: u32, dpm_level: u8) -> c_uint;
}
extern "C" {
    pub fn dcn30_smu_get_dc_mode_max_dpm_freq(clk_mgr: *mut clk_mgr_internal, clk: u32) -> c_uint;
}
extern "C" {
    pub fn dcn30_smu_set_min_deep_sleep_dcef_clk(clk_mgr: *mut clk_mgr_internal, freq_mhz: u32);
}
extern "C" {
    pub fn dcn30_smu_set_num_of_displays(clk_mgr: *mut clk_mgr_internal, num_displays: u32);
}
extern "C" {
    pub fn dcn30_smu_set_display_refresh_from_mall(clk_mgr: *mut clk_mgr_internal, enable: bool, cache_timer_delay: u8, cache_timer_scale: u8);
}
extern "C" {
    pub fn dcn30_smu_set_external_client_df_cstate_allow(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
extern "C" {
    pub fn dcn30_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
}
