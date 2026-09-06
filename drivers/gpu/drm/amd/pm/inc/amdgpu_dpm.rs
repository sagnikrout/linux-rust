//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/inc/amdgpu_dpm.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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
// Argument for PPSMC_MSG_GpuChangeState
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gfx_change_state {
    sGpuChangeState_D0Entry = 1,
    sGpuChangeState_D3Entry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_int_thermal_type {
    THERMAL_TYPE_NONE,
    THERMAL_TYPE_EXTERNAL,
    THERMAL_TYPE_EXTERNAL_GPIO,
    THERMAL_TYPE_RV6XX,
    THERMAL_TYPE_RV770,
    THERMAL_TYPE_ADT7473_WITH_INTERNAL,
    THERMAL_TYPE_EVERGREEN,
    THERMAL_TYPE_SUMO,
    THERMAL_TYPE_NI,
    THERMAL_TYPE_SI,
    THERMAL_TYPE_EMC2103_WITH_INTERNAL,
    THERMAL_TYPE_CI,
    THERMAL_TYPE_KV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_runpm_mode {
    AMDGPU_RUNPM_NONE,
    AMDGPU_RUNPM_PX,
    AMDGPU_RUNPM_BOCO,
    AMDGPU_RUNPM_BACO,
    AMDGPU_RUNPM_BAMACO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ps {
    pub /: *mut *mut u32 caps; / vbios flags,
    pub /: *mut *mut u32 class; / vbios flags,
    pub /: *mut *mut u32 class2; / vbios flags,
// UVD clocks
    pub vclk: u32,
    pub dclk: u32,
// VCE clocks
    pub evclk: u32,
    pub ecclk: u32,
    pub vce_active: bool,
    pub vce_level: amd_vce_level,
// asic priv
    pub ps_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dpm_thermal {
// thermal interrupt work
    pub work: work_struct,
// low temperature threshold
    pub min_temp: c_int,
// high temperature threshold
    pub max_temp: c_int,
// edge max emergency(shutdown) temp
    pub max_edge_emergency_temp: c_int,
// hotspot low temperature threshold
    pub min_hotspot_temp: c_int,
// hotspot high temperature critical threshold
    pub max_hotspot_crit_temp: c_int,
// hotspot max emergency(shutdown) temp
    pub max_hotspot_emergency_temp: c_int,
// memory low temperature threshold
    pub min_mem_temp: c_int,
// memory high temperature critical threshold
    pub max_mem_crit_temp: c_int,
// memory max emergency(shutdown) temp
    pub max_mem_emergency_temp: c_int,
// SWCTF threshold
    pub sw_ctf_threshold: c_int,
// was last interrupt low to high or high to low
    pub high_to_low: bool,
// interrupt source
    pub irq: amdgpu_irq_src,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_clock_and_voltage_limits {
    pub sclk: u32,
    pub mclk: u32,
    pub vddc: u16,
    pub vddci: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_clock_array {
    pub count: u32,
    pub values: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_clock_voltage_dependency_entry {
    pub clk: u32,
    pub v: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_clock_voltage_dependency_table {
    pub count: u32,
    pub entries: *mut amdgpu_clock_voltage_dependency_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amdgpu_cac_leakage_entry {
    pub vddc: u16,
    pub leakage: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_cac_leakage_table {
    pub count: u32,
    pub entries: *mut amdgpu_cac_leakage_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_phase_shedding_limits_entry {
    pub voltage: u16,
    pub sclk: u32,
    pub mclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_phase_shedding_limits_table {
    pub count: u32,
    pub entries: *mut amdgpu_phase_shedding_limits_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_uvd_clock_voltage_dependency_entry {
    pub vclk: u32,
    pub dclk: u32,
    pub v: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_uvd_clock_voltage_dependency_table {
    pub count: u8,
    pub entries: *mut amdgpu_uvd_clock_voltage_dependency_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vce_clock_voltage_dependency_entry {
    pub ecclk: u32,
    pub evclk: u32,
    pub v: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vce_clock_voltage_dependency_table {
    pub count: u8,
    pub entries: *mut amdgpu_vce_clock_voltage_dependency_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ppm_table {
    pub ppm_design: u8,
    pub cpu_core_number: u16,
    pub platform_tdp: u32,
    pub small_ac_platform_tdp: u32,
    pub platform_tdc: u32,
    pub small_ac_platform_tdc: u32,
    pub apu_tdp: u32,
    pub dgpu_tdp: u32,
    pub dgpu_ulv_power: u32,
    pub tj_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_cac_tdp_table {
    pub tdp: u16,
    pub configurable_tdp: u16,
    pub tdc: u16,
    pub battery_power_limit: u16,
    pub small_power_limit: u16,
    pub low_cac_leakage: u16,
    pub high_cac_leakage: u16,
    pub maximum_power_delivery_limit: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dpm_dynamic_state {
    pub vddc_dependency_on_sclk: amdgpu_clock_voltage_dependency_table,
    pub vddci_dependency_on_mclk: amdgpu_clock_voltage_dependency_table,
    pub vddc_dependency_on_mclk: amdgpu_clock_voltage_dependency_table,
    pub mvdd_dependency_on_mclk: amdgpu_clock_voltage_dependency_table,
    pub vddc_dependency_on_dispclk: amdgpu_clock_voltage_dependency_table,
    pub uvd_clock_voltage_dependency_table: amdgpu_uvd_clock_voltage_dependency_table,
    pub vce_clock_voltage_dependency_table: amdgpu_vce_clock_voltage_dependency_table,
    pub samu_clock_voltage_dependency_table: amdgpu_clock_voltage_dependency_table,
    pub acp_clock_voltage_dependency_table: amdgpu_clock_voltage_dependency_table,
    pub vddgfx_dependency_on_sclk: amdgpu_clock_voltage_dependency_table,
    pub valid_sclk_values: amdgpu_clock_array,
    pub valid_mclk_values: amdgpu_clock_array,
    pub max_clock_voltage_on_dc: amdgpu_clock_and_voltage_limits,
    pub max_clock_voltage_on_ac: amdgpu_clock_and_voltage_limits,
    pub mclk_sclk_ratio: u32,
    pub sclk_mclk_delta: u32,
    pub vddc_vddci_delta: u16,
    pub min_vddc_for_pcie_gen2: u16,
    pub cac_leakage_table: amdgpu_cac_leakage_table,
    pub phase_shedding_limits_table: amdgpu_phase_shedding_limits_table,
    pub ppm_table: *mut amdgpu_ppm_table,
    pub cac_tdp_table: *mut amdgpu_cac_tdp_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dpm_fan {
    pub t_min: u16,
    pub t_med: u16,
    pub t_high: u16,
    pub pwm_min: u16,
    pub pwm_med: u16,
    pub pwm_high: u16,
    pub t_hyst: u8,
    pub cycle_delay: u32,
    pub t_max: u16,
    pub control_mode: u8,
    pub default_max_fan_pwm: u16,
    pub default_fan_output_sensitivity: u16,
    pub fan_output_sensitivity: u16,
    pub ucode_fan_control: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dpm {
    pub ps: *mut amdgpu_ps,
// number of valid power states
    pub num_ps: c_int,
// current power state that is active
    pub current_ps: *mut amdgpu_ps,
// requested power state
    pub requested_ps: *mut amdgpu_ps,
// boot up power state
    pub boot_ps: *mut amdgpu_ps,
// default uvd power state
    pub uvd_ps: *mut amdgpu_ps,
// vce requirements
    pub num_of_vce_states: u32,
    pub vce_states: [amd_vce_state; AMD_MAX_VCE_LEVELS],
    pub vce_level: amd_vce_level,
    pub state: amd_pm_state_type,
    pub user_state: amd_pm_state_type,
    pub last_state: amd_pm_state_type,
    pub last_user_state: amd_pm_state_type,
    pub platform_caps: u32,
    pub voltage_response_time: u32,
    pub backbias_response_time: u32,
    pub priv: *mut c_void,
    pub dyn_state: amdgpu_dpm_dynamic_state,
    pub fan: amdgpu_dpm_fan,
    pub tdp_limit: u32,
    pub near_tdp_limit: u32,
    pub near_tdp_limit_adjusted: u32,
    pub sq_ramping_threshold: u32,
    pub cac_leakage: u32,
    pub tdp_od_limit: u16,
    pub tdp_adjustment: u32,
    pub load_line_slope: u16,
    pub power_control: bool,
// special states active
    pub thermal_active: bool,
    pub uvd_active: bool,
    pub vce_active: bool,
// thermal handling
    pub thermal: amdgpu_dpm_thermal,
// forced levels
    pub forced_level: amd_dpm_forced_level,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_power_state {
    POWER_STATE_UNKNOWN,
    POWER_STATE_ON,
    POWER_STATE_OFF,
}

// Used to mask smu debug modes

pub const MAX_SMU_I2C_BUSES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_smu_i2c_bus {
    pub adapter: i2c_adapter,
    pub adev: *mut amdgpu_device,
    pub port: c_int,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_pm {
    pub mutex: mutex,
    pub current_sclk: u32,
    pub current_mclk: u32,
    pub default_sclk: u32,
    pub default_mclk: u32,
    pub i2c_bus: *mut amdgpu_i2c_chan,
    pub bus_locked: bool,
// internal thermal controller on rv6xx+
    pub int_thermal_type: amdgpu_int_thermal_type,
    pub int_hwmon_dev: *mut device,
// fan control parameters
    pub no_fan: bool,
    pub fan_pulses_per_revolution: u8,
    pub fan_min_rpm: u8,
    pub fan_max_rpm: u8,
// dpm
    pub dpm_enabled: bool,
    pub sysfs_initialized: bool,
    pub dpm: amdgpu_dpm,
    pub /: *const *const *const firmware fw; / SMC firmware,
    pub fw_version: u32,
    pub pcie_gen_mask: u32,
    pub pcie_mlw_mask: u32,
    pub /: *mut *mut amd_pp_display_configuration pm_display_cfg;/ set by dc,
    pub smu_prv_buffer_size: u32,
    pub smu_prv_buffer: *mut amdgpu_bo,
    pub ac_power: bool,
// powerplay feature
    pub pp_feature: u32,
// Used for I2C access to various EEPROMs on relevant ASICs
    pub smu_i2c: [amdgpu_smu_i2c_bus; MAX_SMU_I2C_BUSES],
    pub ras_eeprom_i2c_bus: *mut i2c_adapter,
    pub fru_eeprom_i2c_bus: *mut i2c_adapter,
    pub pm_attr_list: list_head,
    pub pwr_state: [core::sync::atomic::AtomicI32; AMD_IP_BLOCK_TYPE_NUM],
//
// 0 = disabled (default), otherwise enable corresponding debug mode
//
    pub smu_debug_mask: u32,
    pub pp_force_state_enabled: bool,
    pub stable_pstate_ctx_lock: mutex,
    pub stable_pstate_ctx: *mut amdgpu_ctx,
    pub config_table: config_table_setting,
// runtime mode
    pub rpm_mode: amdgpu_runpm_mode,
    pub od_kobj_list: list_head,
    pub od_feature_mask: u32,
}

extern "C" {
    pub fn amdgpu_dpm_get_apu_thermal_limit(adev: *mut amdgpu_device, limit: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_set_apu_thermal_limit(adev: *mut amdgpu_device, limit: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_sclk(adev: *mut amdgpu_device, low: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_mclk(adev: *mut amdgpu_device, low: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_baco_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_mode2_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_link_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_enable_gfx_features(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_is_baco_supported(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_is_mode1_reset_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_dpm_is_link_reset_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_dpm_mode1_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_set_gfx_power_up_by_imu(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_baco_exit(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_baco_enter(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_enable_mgpu_fan_boost(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_pm_acpi_event_handler(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dpm_compute_clocks(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dpm_enable_uvd(adev: *mut amdgpu_device, enable: bool);
}
extern "C" {
    pub fn amdgpu_dpm_enable_vcn(adev: *mut amdgpu_device, enable: bool, inst: c_int);
}
extern "C" {
    pub fn amdgpu_dpm_enable_vce(adev: *mut amdgpu_device, enable: bool);
}
extern "C" {
    pub fn amdgpu_dpm_enable_jpeg(adev: *mut amdgpu_device, enable: bool);
}
extern "C" {
    pub fn amdgpu_dpm_enable_vpe(adev: *mut amdgpu_device, enable: bool);
}
extern "C" {
    pub fn amdgpu_pm_load_smu_firmware(adev: *mut amdgpu_device, smu_version: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_handle_passthrough_sbr(adev: *mut amdgpu_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_send_hbm_bad_pages_num(adev: *mut amdgpu_device, size: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_send_hbm_bad_channel_flag(adev: *mut amdgpu_device, size: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_send_rma_reason(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_write_watermarks_table(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_residency_gfxoff(adev: *mut amdgpu_device, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_set_residency_gfxoff(adev: *mut amdgpu_device, value: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_entrycount_gfxoff(adev: *mut amdgpu_device, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_status_gfxoff(adev: *mut amdgpu_device, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_thermal_throttling_counter(adev: *mut amdgpu_device) -> u64;
}
extern "C" {
    pub fn amdgpu_dpm_get_current_power_state(adev: *mut amdgpu_device, state: *mut amd_pm_state_type);
}
extern "C" {
    pub fn amdgpu_dpm_get_performance_level(adev: *mut amdgpu_device) -> amd_dpm_forced_level;
}
extern "C" {
    pub fn amdgpu_dpm_get_ppfeature_status(adev: *mut amdgpu_device, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_sclk_od(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_set_sclk_od(adev: *mut amdgpu_device, value: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_mclk_od(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_set_mclk_od(adev: *mut amdgpu_device, value: u32) -> c_int;
}
//
// @get_pm_metrics: Get one snapshot of power management metrics from PMFW. The
// sample is copied to pm_metrics buffer. It's expected to be allocated by the
// caller and size of the allocated buffer is passed. Max size expected for a
// metrics sample is 4096 bytes.
//
// Return: Actual size of the metrics sample
//
extern "C" {
    pub fn amdgpu_dpm_is_cclk_dpm_supported(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_is_overdrive_supported(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_is_overdrive_enabled(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_get_num_cpu_cores(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_stb_debug_fs_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dpm_notify_smu_enable_pwe(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dpm_reset_sdma(adev: *mut amdgpu_device, inst_mask: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_reset_sdma_is_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_dpm_reset_vcn(adev: *mut amdgpu_device, inst_mask: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_dpm_reset_vcn_is_supported(adev: *mut amdgpu_device) -> bool;
}
