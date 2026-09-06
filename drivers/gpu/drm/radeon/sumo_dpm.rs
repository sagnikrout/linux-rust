//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/sumo_dpm.h
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

pub const SUMO_MAX_HARDWARE_POWERLEVELS: c_int = 5;
pub const SUMO_PM_NUMBER_OF_TC: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_pl {
    pub sclk: u32,
    pub vddc_index: u32,
    pub ds_divider_index: u32,
    pub ss_divider_index: u32,
    pub allow_gnb_slow: u32,
    pub sclk_dpm_tdp_limit: u32,
}

// used for the flags field

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_ps {
    pub levels: [sumo_pl; SUMO_MAX_HARDWARE_POWERLEVELS],
    pub num_levels: u32,
// flags
    pub flags: u32,
}

pub const NUMBER_OF_M3ARB_PARAM_SETS: c_int = 10;
pub const SUMO_MAX_NUMBER_VOLTAGES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_disp_clock_voltage_mapping_table {
    pub num_max_voltage_levels: u32,
    pub display_clock_frequency: [u32; SUMO_MAX_NUMBER_VOLTAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_vid_mapping_entry {
    pub vid_2bit: u16,
    pub vid_7bit: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_vid_mapping_table {
    pub num_entries: u32,
    pub entries: [sumo_vid_mapping_entry; SUMO_MAX_NUMBER_VOLTAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_sclk_voltage_mapping_entry {
    pub sclk_frequency: u32,
    pub vid_2bit: u16,
    pub rsv: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_sclk_voltage_mapping_table {
    pub num_max_dpm_entries: u32,
    pub entries: [sumo_sclk_voltage_mapping_entry; SUMO_MAX_HARDWARE_POWERLEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_sys_info {
    pub bootup_sclk: u32,
    pub min_sclk: u32,
    pub bootup_uma_clk: u32,
    pub bootup_nb_voltage_index: u16,
    pub htc_tmp_lmt: u8,
    pub htc_hyst_lmt: u8,
    pub sclk_voltage_mapping_table: sumo_sclk_voltage_mapping_table,
    pub disp_clk_voltage_mapping_table: sumo_disp_clock_voltage_mapping_table,
    pub vid_mapping_table: sumo_vid_mapping_table,
    pub csr_m3_arb_cntl_default: [u32; NUMBER_OF_M3ARB_PARAM_SETS],
    pub csr_m3_arb_cntl_uvd: [u32; NUMBER_OF_M3ARB_PARAM_SETS],
    pub csr_m3_arb_cntl_fs3d: [u32; NUMBER_OF_M3ARB_PARAM_SETS],
    pub sclk_dpm_boost_margin: u32,
    pub sclk_dpm_throttle_margin: u32,
    pub sclk_dpm_tdp_limit_pg: u32,
    pub gnb_tdp_limit: u32,
    pub sclk_dpm_tdp_limit_boost: u32,
    pub boost_sclk: u32,
    pub boost_vid_2bit: u32,
    pub enable_boost: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sumo_power_info {
    pub asi: u32,
    pub pasi: u32,
    pub bsp: u32,
    pub bsu: u32,
    pub pbsp: u32,
    pub pbsu: u32,
    pub dsp: u32,
    pub psp: u32,
    pub thermal_auto_throttling: u32,
    pub uvd_m3_arbiter: u32,
    pub fw_version: u32,
    pub sys_info: sumo_sys_info,
    pub acpi_pl: sumo_pl,
    pub boot_pl: sumo_pl,
    pub boost_pl: sumo_pl,
    pub disable_gfx_power_gating_in_uvd: bool,
    pub driver_nbps_policy_disable: bool,
    pub enable_alt_vddnb: bool,
    pub enable_dynamic_m3_arbiter: bool,
    pub enable_gfx_clock_gating: bool,
    pub enable_gfx_power_gating: bool,
    pub enable_mg_clock_gating: bool,
    pub enable_sclk_ds: bool,
    pub enable_auto_thermal_throttling: bool,
    pub enable_dynamic_patch_ps: bool,
    pub enable_dpm: bool,
    pub enable_boost: bool,
    pub current_rps: radeon_ps,
    pub current_ps: sumo_ps,
    pub requested_rps: radeon_ps,
    pub requested_ps: sumo_ps,
}

pub const SUMO_UTC_DFLT_00: c_uint = 0x48;
pub const SUMO_UTC_DFLT_01: c_uint = 0x44;
pub const SUMO_UTC_DFLT_02: c_uint = 0x44;
pub const SUMO_UTC_DFLT_03: c_uint = 0x44;
pub const SUMO_UTC_DFLT_04: c_uint = 0x44;
pub const SUMO_UTC_DFLT_05: c_uint = 0x44;
pub const SUMO_UTC_DFLT_06: c_uint = 0x44;
pub const SUMO_UTC_DFLT_07: c_uint = 0x44;
pub const SUMO_UTC_DFLT_08: c_uint = 0x44;
pub const SUMO_UTC_DFLT_09: c_uint = 0x44;
pub const SUMO_UTC_DFLT_10: c_uint = 0x44;
pub const SUMO_UTC_DFLT_11: c_uint = 0x44;
pub const SUMO_UTC_DFLT_12: c_uint = 0x44;
pub const SUMO_UTC_DFLT_13: c_uint = 0x44;
pub const SUMO_UTC_DFLT_14: c_uint = 0x44;
pub const SUMO_DTC_DFLT_00: c_uint = 0x48;
pub const SUMO_DTC_DFLT_01: c_uint = 0x44;
pub const SUMO_DTC_DFLT_02: c_uint = 0x44;
pub const SUMO_DTC_DFLT_03: c_uint = 0x44;
pub const SUMO_DTC_DFLT_04: c_uint = 0x44;
pub const SUMO_DTC_DFLT_05: c_uint = 0x44;
pub const SUMO_DTC_DFLT_06: c_uint = 0x44;
pub const SUMO_DTC_DFLT_07: c_uint = 0x44;
pub const SUMO_DTC_DFLT_08: c_uint = 0x44;
pub const SUMO_DTC_DFLT_09: c_uint = 0x44;
pub const SUMO_DTC_DFLT_10: c_uint = 0x44;
pub const SUMO_DTC_DFLT_11: c_uint = 0x44;
pub const SUMO_DTC_DFLT_12: c_uint = 0x44;
pub const SUMO_DTC_DFLT_13: c_uint = 0x44;
pub const SUMO_DTC_DFLT_14: c_uint = 0x44;
pub const SUMO_AH_DFLT: c_int = 5;
pub const SUMO_R_DFLT0: c_int = 70;
pub const SUMO_R_DFLT1: c_int = 70;
pub const SUMO_R_DFLT2: c_int = 70;
pub const SUMO_R_DFLT3: c_int = 70;
pub const SUMO_R_DFLT4: c_int = 100;
pub const SUMO_L_DFLT0: c_int = 0;
pub const SUMO_L_DFLT1: c_int = 20;
pub const SUMO_L_DFLT2: c_int = 20;
pub const SUMO_L_DFLT3: c_int = 20;
pub const SUMO_L_DFLT4: c_int = 20;
pub const SUMO_VRC_DFLT: c_uint = 0x30033;
pub const SUMO_MGCGTTLOCAL0_DFLT: c_int = 0;
pub const SUMO_MGCGTTLOCAL1_DFLT: c_int = 0;
pub const SUMO_GICST_DFLT: c_int = 19;
pub const SUMO_SST_DFLT: c_int = 8;
pub const SUMO_VOLTAGEDROPT_DFLT: c_int = 1;
pub const SUMO_GFXPOWERGATINGT_DFLT: c_int = 100;
// sumo_dpm.c
extern "C" {
    pub fn sumo_gfx_clockgating_initialize(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_program_vc(rdev: *mut radeon_device, vrc: u32);
}
extern "C" {
    pub fn sumo_clear_vc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_program_sstp(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_take_smu_control(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn sumo_get_sleep_divider_from_id(id: u32) -> u32;
}
// sumo_smc.c
extern "C" {
    pub fn sumo_initialize_m3_arb(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_smu_pg_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_set_tdp_limit(rdev: *mut radeon_device, index: u32, tdp_limit: u32);
}
extern "C" {
    pub fn sumo_boost_state_enable(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn sumo_enable_boost_timer(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_get_running_fw_version(rdev: *mut radeon_device) -> u32;
}
