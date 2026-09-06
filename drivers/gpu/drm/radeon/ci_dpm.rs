//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/ci_dpm.h
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
// Copyright 2013 Advanced Micro Devices, Inc.
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

pub const SMU__NUM_SCLK_DPM_STATE: c_int = 8;
pub const SMU__NUM_MCLK_DPM_LEVELS: c_int = 6;
pub const SMU__NUM_LCLK_DPM_LEVELS: c_int = 8;
pub const SMU__NUM_PCIE_DPM_LEVELS: c_int = 8;

pub const CISLANDS_MAX_HARDWARE_POWERLEVELS: c_int = 2;
pub const CISLANDS_UNUSED_GPIO_PIN: c_uint = 0x7F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_pl {
    pub mclk: u32,
    pub sclk: u32,
    pub pcie_gen: radeon_pcie_gen,
    pub pcie_lane: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_ps {
    pub performance_level_count: u16,
    pub dc_compatible: bool,
    pub sclk_t: u32,
    pub performance_levels: [ci_pl; CISLANDS_MAX_HARDWARE_POWERLEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_dpm_level {
    pub enabled: bool,
    pub value: u32,
    pub param1: u32,
}

pub const CISLAND_MAX_DEEPSLEEP_DIVIDER_ID: c_int = 5;
pub const MAX_REGULAR_DPM_NUMBER: c_int = 8;
pub const CISLAND_MINIMUM_ENGINE_CLOCK: c_int = 800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_single_dpm_table {
    pub count: u32,
    pub dpm_levels: [ci_dpm_level; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_dpm_table {
    pub sclk_table: ci_single_dpm_table,
    pub mclk_table: ci_single_dpm_table,
    pub pcie_speed_table: ci_single_dpm_table,
    pub vddc_table: ci_single_dpm_table,
    pub vddci_table: ci_single_dpm_table,
    pub mvdd_table: ci_single_dpm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; SMU7_DISCRETE_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_mc_reg_table {
    pub last: u8,
    pub num_entries: u8,
    pub valid_flag: u16,
    pub mc_reg_table_entry: [ci_mc_reg_entry; MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [SMU7_Discrete_MCRegisterAddress; SMU7_DISCRETE_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_ulv_parm {
    pub supported: bool,
    pub cg_ulv_parameter: u32,
    pub volt_change_delay: u32,
    pub pl: ci_pl,
}

pub const CISLANDS_MAX_LEAKAGE_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_leakage_voltage {
    pub count: u16,
    pub leakage_id: [u16; CISLANDS_MAX_LEAKAGE_COUNT],
    pub actual_voltage: [u16; CISLANDS_MAX_LEAKAGE_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_dpm_level_enable_mask {
    pub uvd_dpm_enable_mask: u32,
    pub vce_dpm_enable_mask: u32,
    pub acp_dpm_enable_mask: u32,
    pub samu_dpm_enable_mask: u32,
    pub sclk_dpm_enable_mask: u32,
    pub mclk_dpm_enable_mask: u32,
    pub pcie_dpm_enable_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_vbios_boot_state {
    pub mvdd_bootup_value: u16,
    pub vddc_bootup_value: u16,
    pub vddci_bootup_value: u16,
    pub sclk_bootup_value: u32,
    pub mclk_bootup_value: u32,
    pub pcie_gen_bootup_value: u16,
    pub pcie_lane_bootup_value: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_clock_registers {
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
pub struct ci_thermal_temperature_setting {
    pub temperature_low: i32,
    pub temperature_high: i32,
    pub temperature_shutdown: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_pcie_perf_range {
    pub max: u16,
    pub min: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ci_pt_config_reg_type {
    CISLANDS_CONFIGREG_MMR = 0,
    CISLANDS_CONFIGREG_SMC_IND,
    CISLANDS_CONFIGREG_DIDT_IND,
    CISLANDS_CONFIGREG_CACHE,
    CISLANDS_CONFIGREG_MAX
}

pub const POWERCONTAINMENT_FEATURE_BAPM: c_uint = 0x00000001;
pub const POWERCONTAINMENT_FEATURE_TDCLimit: c_uint = 0x00000002;
pub const POWERCONTAINMENT_FEATURE_PkgPwrLimit: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_pt_config_reg {
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub value: u32,
    pub type: ci_pt_config_reg_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_pt_defaults {
    pub svi_load_line_en: u8,
    pub svi_load_line_vddc: u8,
    pub tdc_vddc_throttle_release_limit_perc: u8,
    pub tdc_mawt: u8,
    pub tdc_waterfall_ctl: u8,
    pub dte_ambient_temp_base: u8,
    pub display_cac: u32,
    pub bapm_temp_gradient: u32,
    pub SMU7_DTE_SINKS]: *mut *mut *mut u16 bapmti_r[SMU7_DTE_ITERATIONS  SMU7_DTE_SOURCES,
    pub SMU7_DTE_SINKS]: *mut *mut *mut u16 bapmti_rc[SMU7_DTE_ITERATIONS  SMU7_DTE_SOURCES,
}

pub const DPMTABLE_OD_UPDATE_SCLK: c_uint = 0x00000001;
pub const DPMTABLE_OD_UPDATE_MCLK: c_uint = 0x00000002;
pub const DPMTABLE_UPDATE_SCLK: c_uint = 0x00000004;
pub const DPMTABLE_UPDATE_MCLK: c_uint = 0x00000008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_power_info {
    pub dpm_table: ci_dpm_table,
    pub voltage_control: u32,
    pub mvdd_control: u32,
    pub vddci_control: u32,
    pub active_auto_throttle_sources: u32,
    pub clock_registers: ci_clock_registers,
    pub acpi_vddc: u16,
    pub acpi_vddci: u16,
    pub force_pcie_gen: radeon_pcie_gen,
    pub acpi_pcie_gen: radeon_pcie_gen,
    pub vddc_leakage: ci_leakage_voltage,
    pub vddci_leakage: ci_leakage_voltage,
    pub max_vddc_in_pp_table: u16,
    pub min_vddc_in_pp_table: u16,
    pub max_vddci_in_pp_table: u16,
    pub min_vddci_in_pp_table: u16,
    pub mclk_strobe_mode_threshold: u32,
    pub mclk_stutter_mode_threshold: u32,
    pub mclk_edc_enable_threshold: u32,
    pub mclk_edc_wr_enable_threshold: u32,
    pub vbios_boot_state: ci_vbios_boot_state,
// smc offsets
    pub sram_end: u32,
    pub dpm_table_start: u32,
    pub soft_regs_start: u32,
    pub mc_reg_table_start: u32,
    pub fan_table_start: u32,
    pub arb_table_start: u32,
// smc tables
    pub smc_state_table: SMU7_Discrete_DpmTable,
    pub smc_mc_reg_table: SMU7_Discrete_MCRegisters,
    pub smc_powertune_table: SMU7_Discrete_PmFuses,
// other stuff
    pub mc_reg_table: ci_mc_reg_table,
    pub vddc_voltage_table: atom_voltage_table,
    pub vddci_voltage_table: atom_voltage_table,
    pub mvdd_voltage_table: atom_voltage_table,
    pub ulv: ci_ulv_parm,
    pub power_containment_features: u32,
    pub powertune_defaults: *const ci_pt_defaults,
    pub dte_tj_offset: u32,
    pub vddc_phase_shed_control: bool,
    pub thermal_temp_setting: ci_thermal_temperature_setting,
    pub dpm_level_enable_mask: ci_dpm_level_enable_mask,
    pub need_update_smu7_dpm_table: u32,
    pub sclk_dpm_key_disabled: u32,
    pub mclk_dpm_key_disabled: u32,
    pub pcie_dpm_key_disabled: u32,
    pub thermal_sclk_dpm_enabled: u32,
    pub pcie_gen_performance: ci_pcie_perf_range,
    pub pcie_lane_performance: ci_pcie_perf_range,
    pub pcie_gen_powersaving: ci_pcie_perf_range,
    pub pcie_lane_powersaving: ci_pcie_perf_range,
    pub activity_target: [u32; SMU7_MAX_LEVELS_GRAPHICS],
    pub mclk_activity_target: u32,
    pub low_sclk_interrupt_t: u32,
    pub last_mclk_dpm_enable_mask: u32,
    pub sys_pcie_mask: u32,
// caps
    pub caps_power_containment: bool,
    pub caps_cac: bool,
    pub caps_sq_ramping: bool,
    pub caps_db_ramping: bool,
    pub caps_td_ramping: bool,
    pub caps_tcp_ramping: bool,
    pub caps_fps: bool,
    pub caps_sclk_ds: bool,
    pub caps_sclk_ss_support: bool,
    pub caps_mclk_ss_support: bool,
    pub caps_uvd_dpm: bool,
    pub caps_vce_dpm: bool,
    pub caps_samu_dpm: bool,
    pub caps_acp_dpm: bool,
    pub caps_automatic_dc_transition: bool,
    pub caps_sclk_throttle_low_notification: bool,
    pub caps_dynamic_ac_timing: bool,
    pub caps_od_fuzzy_fan_control_support: bool,
// flags
    pub thermal_protection: bool,
    pub pcie_performance_request: bool,
    pub dynamic_ss: bool,
    pub dll_default_on: bool,
    pub cac_enabled: bool,
    pub uvd_enabled: bool,
    pub battery_state: bool,
    pub pspp_notify_required: bool,
    pub mem_gddr5: bool,
    pub enable_bapm_feature: bool,
    pub enable_tdc_limit_feature: bool,
    pub enable_pkg_pwr_tracking_feature: bool,
    pub use_pcie_performance_levels: bool,
    pub use_pcie_powersaving_levels: bool,
    pub uvd_power_gated: bool,
// driver states
    pub current_rps: radeon_ps,
    pub current_ps: ci_ps,
    pub requested_rps: radeon_ps,
    pub requested_ps: ci_ps,
// fan control
    pub fan_ctrl_is_in_default_mode: bool,
    pub fan_is_controlled_by_smc: bool,
    pub t_min: u32,
    pub fan_ctrl_default_mode: u32,
}

pub const CISLANDS_VOLTAGE_CONTROL_NONE: c_uint = 0x0;
pub const CISLANDS_VOLTAGE_CONTROL_BY_GPIO: c_uint = 0x1;
pub const CISLANDS_VOLTAGE_CONTROL_BY_SVID2: c_uint = 0x2;
pub const CISLANDS_Q88_FORMAT_CONVERSION_UNIT: c_int = 256;
pub const CISLANDS_VRC_DFLT0: c_uint = 0x3FFFC000;
pub const CISLANDS_VRC_DFLT1: c_uint = 0x000400;
pub const CISLANDS_VRC_DFLT2: c_uint = 0xC00080;
pub const CISLANDS_VRC_DFLT3: c_uint = 0xC00200;
pub const CISLANDS_VRC_DFLT4: c_uint = 0xC01680;
pub const CISLANDS_VRC_DFLT5: c_uint = 0xC00033;
pub const CISLANDS_VRC_DFLT6: c_uint = 0xC00033;
pub const CISLANDS_VRC_DFLT7: c_uint = 0x3FFFC000;
pub const CISLANDS_CGULVPARAMETER_DFLT: c_uint = 0x00040035;
pub const CISLAND_TARGETACTIVITY_DFLT: c_int = 30;
pub const CISLAND_MCLK_TARGETACTIVITY_DFLT: c_int = 10;
pub const PCIE_PERF_REQ_REMOVE_REGISTRY: c_int = 0;
pub const PCIE_PERF_REQ_FORCE_LOWPOWER: c_int = 1;
pub const PCIE_PERF_REQ_PECI_GEN1: c_int = 2;
pub const PCIE_PERF_REQ_PECI_GEN2: c_int = 3;
pub const PCIE_PERF_REQ_PECI_GEN3: c_int = 4;
extern "C" {
    pub fn ci_start_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_reset_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_program_jump_on_start(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ci_stop_smc_clock(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_start_smc_clock(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_is_smc_running(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn ci_wait_for_smc_inactive(rdev: *mut radeon_device) -> PPSMC_Result;
}
extern "C" {
    pub fn ci_load_smc_ucode(rdev: *mut radeon_device, limit: u32) -> c_int;
}
