//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/trinity_dpm.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trinity_pl {
    pub sclk: u32,
    pub vddc_index: u8,
    pub ds_divider_index: u8,
    pub ss_divider_index: u8,
    pub allow_gnb_slow: u8,
    pub force_nbp_state: u8,
    pub display_wm: u8,
    pub vce_wm: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trinity_ps {
    pub num_levels: u32,
    pub levels: [trinity_pl; SUMO_MAX_HARDWARE_POWERLEVELS],
    pub nbps_flags: u32,
    pub bapm_flags: u32,
    pub Dpm0PgNbPsLo: u8,
    pub Dpm0PgNbPsHi: u8,
    pub DpmXNbPsLo: u8,
    pub DpmXNbPsHi: u8,
    pub vclk_low_divider: u32,
    pub vclk_high_divider: u32,
    pub dclk_low_divider: u32,
    pub dclk_high_divider: u32,
}

pub const TRINITY_NUM_NBPSTATES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trinity_uvd_clock_table_entry {
    pub vclk: u32,
    pub dclk: u32,
    pub vclk_did: u8,
    pub dclk_did: u8,
    pub rsv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trinity_sys_info {
    pub bootup_uma_clk: u32,
    pub bootup_sclk: u32,
    pub min_sclk: u32,
    pub dentist_vco_freq: u32,
    pub nb_dpm_enable: u32,
    pub nbp_mclk: [u32; TRINITY_NUM_NBPSTATES],
    pub nbp_nclk: [u32; TRINITY_NUM_NBPSTATES],
    pub nbp_voltage_index: [u16; TRINITY_NUM_NBPSTATES],
    pub bootup_nb_voltage_index: u16,
    pub htc_tmp_lmt: u8,
    pub htc_hyst_lmt: u8,
    pub sclk_voltage_mapping_table: sumo_sclk_voltage_mapping_table,
    pub vid_mapping_table: sumo_vid_mapping_table,
    pub uma_channel_number: u32,
    pub uvd_clock_table_entries: [trinity_uvd_clock_table_entry; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trinity_power_info {
    pub at: [u32; SUMO_MAX_HARDWARE_POWERLEVELS],
    pub dpm_interval: u32,
    pub thermal_auto_throttling: u32,
    pub sys_info: trinity_sys_info,
    pub boot_pl: trinity_pl,
    pub min_sclk_did: u32,
    pub enable_nbps_policy: bool,
    pub voltage_drop_in_dce: bool,
    pub override_dynamic_mgpg: bool,
    pub enable_gfx_clock_gating: bool,
    pub enable_gfx_power_gating: bool,
    pub enable_mg_clock_gating: bool,
    pub enable_gfx_dynamic_mgpg: bool,
    pub enable_auto_thermal_throttling: bool,
    pub enable_dpm: bool,
    pub enable_sclk_ds: bool,
    pub enable_bapm: bool,
    pub uvd_dpm: bool,
    pub current_rps: radeon_ps,
    pub current_ps: trinity_ps,
    pub requested_rps: radeon_ps,
    pub requested_ps: trinity_ps,
}

pub const TRINITY_AT_DFLT: c_int = 30;
// trinity_smc.c
extern "C" {
    pub fn trinity_dpm_bapm_enable(rdev: *mut radeon_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_config(rdev: *mut radeon_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn trinity_uvd_dpm_config(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_force_state(rdev: *mut radeon_device, n: u32) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_n_levels_disabled(rdev: *mut radeon_device, n: u32) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_no_forced_level(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_gfx_dynamic_mgpg_config(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_acquire_mutex(rdev: *mut radeon_device);
}
extern "C" {
    pub fn trinity_release_mutex(rdev: *mut radeon_device);
}
