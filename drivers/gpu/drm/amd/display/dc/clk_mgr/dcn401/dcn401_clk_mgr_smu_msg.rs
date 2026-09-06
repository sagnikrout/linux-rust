//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn401/dcn401_clk_mgr_smu_msg.h
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
// Copyright 2024 Advanced Micro Devices, Inc.

extern "C" {
    pub fn dcn401_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal, version: *mut c_int) -> bool;
}
extern "C" {
    pub fn dcn401_smu_check_driver_if_version(clk_mgr: *mut clk_mgr_internal) -> bool;
}
extern "C" {
    pub fn dcn401_smu_check_msg_header_version(clk_mgr: *mut clk_mgr_internal) -> bool;
}
extern "C" {
    pub fn dcn401_smu_send_fclk_pstate_message(clk_mgr: *mut clk_mgr_internal, support: bool);
}
extern "C" {
    pub fn dcn401_smu_send_uclk_pstate_message(clk_mgr: *mut clk_mgr_internal, support: bool);
}
extern "C" {
    pub fn dcn401_smu_send_cab_for_uclk_message(clk_mgr: *mut clk_mgr_internal, num_ways: c_uint);
}
extern "C" {
    pub fn dcn401_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
}
extern "C" {
    pub fn dcn401_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
}
extern "C" {
    pub fn dcn401_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn401_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn401_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> c_uint;
}
extern "C" {
    pub fn dcn401_smu_wait_for_dmub_ack_mclk(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
extern "C" {
    pub fn dcn401_smu_indicate_drr_status(clk_mgr: *mut clk_mgr_internal, mod_drr_for_pstate: bool);
}
extern "C" {
    pub fn dcn401_smu_set_min_deep_sleep_dcef_clk(clk_mgr: *mut clk_mgr_internal, freq_mhz: u32);
}
extern "C" {
    pub fn dcn401_smu_set_num_of_displays(clk_mgr: *mut clk_mgr_internal, num_displays: u32);
}
extern "C" {
    pub fn dcn401_smu_get_num_of_umc_channels(clk_mgr: *mut clk_mgr_internal) -> c_uint;
}
extern "C" {
    pub fn dcn401_smu_get_dc_mode_max_dpm_freq(clk_mgr: *mut clk_mgr_internal, clk: u32) -> c_uint;
}
extern "C" {
    pub fn dcn401_smu_get_dpm_freq_by_index(clk_mgr: *mut clk_mgr_internal, clk: u32, dpm_level: u8) -> c_uint;
}
