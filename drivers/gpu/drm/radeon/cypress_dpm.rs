//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/cypress_dpm.h
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
// Copyright 2011 Advanced Micro Devices, Inc.
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
pub struct evergreen_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; SMC_EVERGREEN_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evergreen_mc_reg_table {
    pub last: u8,
    pub num_entries: u8,
    pub valid_flag: u16,
    pub mc_reg_table_entry: [evergreen_mc_reg_entry; MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [SMC_Evergreen_MCRegisterAddress; SMC_EVERGREEN_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evergreen_ulv_param {
    pub supported: bool,
    pub pl: *mut rv7xx_pl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evergreen_arb_registers {
    pub mc_arb_dram_timing: u32,
    pub mc_arb_dram_timing2: u32,
    pub mc_arb_rfsh_rate: u32,
    pub mc_arb_burst_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at {
    pub rlp: u32,
    pub rmp: u32,
    pub lhp: u32,
    pub lmp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evergreen_power_info {
// must be first!
    pub rv7xx: rv7xx_power_info,
// flags
    pub vddci_control: bool,
    pub dynamic_ac_timing: bool,
    pub abm: bool,
    pub mcls: bool,
    pub light_sleep: bool,
    pub memory_transition: bool,
    pub pcie_performance_request: bool,
    pub pcie_performance_request_registered: bool,
    pub sclk_deep_sleep: bool,
    pub dll_default_on: bool,
    pub ls_clock_gating: bool,
    pub smu_uvd_hs: bool,
    pub uvd_enabled: bool,
// stored values
    pub acpi_vddci: u16,
    pub mvdd_high_index: u8,
    pub mvdd_low_index: u8,
    pub mclk_edc_wr_enable_threshold: u32,
    pub mc_reg_table: evergreen_mc_reg_table,
    pub vddc_voltage_table: atom_voltage_table,
    pub vddci_voltage_table: atom_voltage_table,
    pub bootup_arb_registers: evergreen_arb_registers,
    pub ulv: evergreen_ulv_param,
    pub ats: [at; 2],
// smc offsets
    pub mc_reg_table_start: u16,
    pub current_rps: radeon_ps,
    pub current_ps: rv7xx_ps,
    pub requested_rps: radeon_ps,
    pub requested_ps: rv7xx_ps,
}

pub const CYPRESS_HASI_DFLT: c_int = 400000;
pub const CYPRESS_MGCGTTLOCAL0_DFLT: c_uint = 0x00000000;
pub const CYPRESS_MGCGTTLOCAL1_DFLT: c_uint = 0x00000000;
pub const CYPRESS_MGCGTTLOCAL2_DFLT: c_uint = 0x00000000;
pub const CYPRESS_MGCGTTLOCAL3_DFLT: c_uint = 0x00000000;
pub const CYPRESS_MGCGCGTSSMCTRL_DFLT: c_uint = 0x81944bc0;
pub const REDWOOD_MGCGCGTSSMCTRL_DFLT: c_uint = 0x6e944040;
pub const CEDAR_MGCGCGTSSMCTRL_DFLT: c_uint = 0x46944040;
pub const CYPRESS_VRC_DFLT: c_uint = 0xC00033;
pub const PCIE_PERF_REQ_REMOVE_REGISTRY: c_int = 0;
pub const PCIE_PERF_REQ_FORCE_LOWPOWER: c_int = 1;
pub const PCIE_PERF_REQ_PECI_GEN1: c_int = 2;
pub const PCIE_PERF_REQ_PECI_GEN2: c_int = 3;
pub const PCIE_PERF_REQ_PECI_GEN3: c_int = 4;
extern "C" {
    pub fn cypress_construct_voltage_tables(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cypress_get_mvdd_configuration(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cypress_enable_display_gap(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_get_table_locations(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cypress_program_response_times(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_start_dpm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_advertise_gen2_capability(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_map_clkf_to_ibias(rdev: *mut radeon_device, clkf: u32) -> u32;
}
extern "C" {
    pub fn cypress_get_strobe_mode_settings(rdev: *mut radeon_device, mclk: u32) -> u8;
}
