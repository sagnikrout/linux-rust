//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/si_dpm.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si_cac_config_reg_type {
    SISLANDS_CACCONFIG_MMR = 0,
    SISLANDS_CACCONFIG_CGIND,
    SISLANDS_CACCONFIG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_cac_config_reg {
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub value: u32,
    pub type: si_cac_config_reg_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_powertune_data {
    pub cac_window: u32,
    pub l2_lta_window_size_default: u32,
    pub lts_truncate_default: u8,
    pub shift_n_default: u8,
    pub operating_temp: u8,
    pub leakage_coefficients: ni_leakage_coeffients,
    pub fixed_kt: u32,
    pub lkge_lut_v0_percent: u32,
    pub dc_cac: [u8; NISLANDS_DCCAC_MAX_LEVELS],
    pub enable_powertune_by_default: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_dyn_powertune_data {
    pub cac_leakage: u32,
    pub leakage_minimum_temperature: i32,
    pub wintime: u32,
    pub l2_lta_window_size: u32,
    pub lts_truncate: u8,
    pub shift_n: u8,
    pub dc_pwr_value: u8,
    pub disable_uvd_powertune: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_dte_data {
    pub tau: [u32; SMC_SISLANDS_DTE_MAX_FILTER_STAGES],
    pub r: [u32; SMC_SISLANDS_DTE_MAX_FILTER_STAGES],
    pub k: u32,
    pub t0: u32,
    pub max_t: u32,
    pub window_size: u8,
    pub temp_select: u8,
    pub dte_mode: u8,
    pub tdep_count: u8,
    pub t_limits: [u8; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],
    pub tdep_tau: [u32; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],
    pub tdep_r: [u32; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],
    pub t_threshold: u32,
    pub enable_dte_by_default: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_clock_registers {
    pub cg_spll_func_cntl: u32,
    pub cg_spll_func_cntl_2: u32,
    pub cg_spll_func_cntl_3: u32,
    pub cg_spll_func_cntl_4: u32,
    pub cg_spll_spread_spectrum: u32,
    pub cg_spll_spread_spectrum_2: u32,
    pub dll_cntl: u32,
    pub mclk_pwrmgt_cntl: u32,
    pub mpll_ad_func_cntl: u32,
    pub mpll_dq_func_cntl: u32,
    pub mpll_func_cntl: u32,
    pub mpll_func_cntl_1: u32,
    pub mpll_func_cntl_2: u32,
    pub mpll_ss1: u32,
    pub mpll_ss2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_mc_reg_table {
    pub last: u8,
    pub num_entries: u8,
    pub valid_flag: u16,
    pub mc_reg_table_entry: [si_mc_reg_entry; MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [SMC_NIslands_MCRegisterAddress; SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE],
}

pub const SISLANDS_MCREGISTERTABLE_INITIAL_SLOT: c_int = 0;
pub const SISLANDS_MCREGISTERTABLE_ACPI_SLOT: c_int = 1;
pub const SISLANDS_MCREGISTERTABLE_ULV_SLOT: c_int = 2;
pub const SISLANDS_MCREGISTERTABLE_FIRST_DRIVERSTATE_SLOT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_leakage_voltage_entry {
    pub voltage: u16,
    pub leakage_index: u16,
}

pub const SISLANDS_LEAKAGE_INDEX0: c_uint = 0xff01;
pub const SISLANDS_MAX_LEAKAGE_COUNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_leakage_voltage {
    pub count: u16,
    pub entries: [si_leakage_voltage_entry; SISLANDS_MAX_LEAKAGE_COUNT],
}

pub const SISLANDS_MAX_HARDWARE_POWERLEVELS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_ulv_param {
    pub supported: bool,
    pub cg_ulv_control: u32,
    pub cg_ulv_parameter: u32,
    pub volt_change_delay: u32,
    pub pl: rv7xx_pl,
    pub one_pcie_lane_in_ulv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_power_info {
// must be first!
    pub ni: ni_power_info,
    pub clock_registers: si_clock_registers,
    pub mc_reg_table: si_mc_reg_table,
    pub mvdd_voltage_table: atom_voltage_table,
    pub vddc_phase_shed_table: atom_voltage_table,
    pub leakage_voltage: si_leakage_voltage,
    pub mvdd_bootup_value: u16,
    pub ulv: si_ulv_param,
    pub max_cu: u32,
// pcie gen
    pub force_pcie_gen: radeon_pcie_gen,
    pub boot_pcie_gen: radeon_pcie_gen,
    pub acpi_pcie_gen: radeon_pcie_gen,
    pub sys_pcie_mask: u32,
// flags
    pub enable_dte: bool,
    pub enable_ppm: bool,
    pub vddc_phase_shed_control: bool,
    pub pspp_notify_required: bool,
    pub sclk_deep_sleep_above_low: bool,
    pub voltage_control_svi2: bool,
    pub vddci_control_svi2: bool,
// smc offsets
    pub sram_end: u32,
    pub state_table_start: u32,
    pub soft_regs_start: u32,
    pub mc_reg_table_start: u32,
    pub arb_table_start: u32,
    pub cac_table_start: u32,
    pub dte_table_start: u32,
    pub spll_table_start: u32,
    pub papm_cfg_table_start: u32,
    pub fan_table_start: u32,
// CAC stuff
    pub cac_weights: *const si_cac_config_reg,
    pub lcac_config: *const si_cac_config_reg,
    pub cac_override: *const si_cac_config_reg,
    pub powertune_data: *const si_powertune_data,
    pub dyn_powertune_data: si_dyn_powertune_data,
// DTE stuff
    pub dte_data: si_dte_data,
// scratch structs
    pub smc_mc_reg_table: SMC_SIslands_MCRegisters,
    pub smc_statetable: SISLANDS_SMC_STATETABLE,
    pub papm_parm: PP_SIslands_PAPMParameters,
// SVI2
    pub svd_gpio_id: u8,
    pub svc_gpio_id: u8,
// fan control
    pub fan_ctrl_is_in_default_mode: bool,
    pub t_min: u32,
    pub fan_ctrl_default_mode: u32,
    pub fan_is_controlled_by_smc: bool,
}

pub const SISLANDS_INITIAL_STATE_ARB_INDEX: c_int = 0;
pub const SISLANDS_ACPI_STATE_ARB_INDEX: c_int = 1;
pub const SISLANDS_ULV_STATE_ARB_INDEX: c_int = 2;
pub const SISLANDS_DRIVER_STATE_ARB_INDEX: c_int = 3;
pub const SISLANDS_DPM2_MAX_PULSE_SKIP: c_int = 256;
pub const SISLANDS_DPM2_NEAR_TDP_DEC: c_int = 10;
pub const SISLANDS_DPM2_ABOVE_SAFE_INC: c_int = 5;
pub const SISLANDS_DPM2_BELOW_SAFE_INC: c_int = 20;
pub const SISLANDS_DPM2_TDP_SAFE_LIMIT_PERCENT: c_int = 80;
pub const SISLANDS_DPM2_MAXPS_PERCENT_H: c_int = 99;
pub const SISLANDS_DPM2_MAXPS_PERCENT_M: c_int = 99;
pub const SISLANDS_DPM2_SQ_RAMP_MAX_POWER: c_uint = 0x3FFF;
pub const SISLANDS_DPM2_SQ_RAMP_MIN_POWER: c_uint = 0x12;
pub const SISLANDS_DPM2_SQ_RAMP_MAX_POWER_DELTA: c_uint = 0x15;
pub const SISLANDS_DPM2_SQ_RAMP_STI_SIZE: c_uint = 0x1E;
pub const SISLANDS_DPM2_SQ_RAMP_LTI_RATIO: c_uint = 0xF;
pub const SISLANDS_DPM2_PWREFFICIENCYRATIO_MARGIN: c_int = 10;
pub const SISLANDS_VRC_DFLT: c_uint = 0xC000B3;
pub const SISLANDS_ULVVOLTAGECHANGEDELAY_DFLT: c_int = 1687;
pub const SISLANDS_CGULVPARAMETER_DFLT: c_uint = 0x00040035;
pub const SISLANDS_CGULVCONTROL_DFLT: c_uint = 0x1f007550;
extern "C" {
    pub fn si_get_ddr3_mclk_frequency_ratio(memory_clock: u32) -> u8;
}
extern "C" {
    pub fn si_get_mclk_frequency_ratio(memory_clock: u32, strobe_mode: bool) -> u8;
}
