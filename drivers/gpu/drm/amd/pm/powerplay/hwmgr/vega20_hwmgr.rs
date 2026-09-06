//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/vega20_hwmgr.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

pub const VEGA20_MAX_HARDWARE_POWERLEVELS: c_int = 2;
pub const WaterMarksExist: c_int = 1;
pub const WaterMarksLoaded: c_int = 2;
pub const VG20_PSUEDO_NUM_GFXCLK_DPM_LEVELS: c_int = 8;
pub const VG20_PSUEDO_NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const VG20_PSUEDO_NUM_DCEFCLK_DPM_LEVELS: c_int = 8;
pub const VG20_PSUEDO_NUM_UCLK_DPM_LEVELS: c_int = 4;
// OverDriver8 macro defs
pub const AVFS_CURVE: c_int = 0;
pub const OD8_HOTCURVE_TEMPERATURE: c_int = 85;
pub const VG20_CLOCK_MAX_DEFAULT: c_uint = 0xFFFF;
pub type PP_Clock = u32;

pub const SMC_DPM_FEATURES: c_uint = 0x30F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_features {
    pub supported: bool,
    pub enabled: bool,
    pub allowed: bool,
    pub smu_feature_id: u32,
    pub smu_feature_bitmap: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_performance_level {
    pub soc_clock: u32,
    pub gfx_clock: u32,
    pub mem_clock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_bacos {
    pub baco_flags: u32,
// struct vega20_performance_level  performance_level;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_uvd_clocks {
    pub vclk: u32,
    pub dclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_vce_clocks {
    pub evclk: u32,
    pub ecclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_power_state {
    pub magic: u32,
    pub uvd_clks: vega20_uvd_clocks,
    pub vce_clks: vega20_vce_clocks,
    pub performance_level_count: u16,
    pub dc_compatible: bool,
    pub sclk_threshold: u32,
    pub performance_levels: [vega20_performance_level; VEGA20_MAX_HARDWARE_POWERLEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_dpm_level {
    pub enabled: bool,
    pub value: u32,
    pub param1: u32,
}

pub const VEGA20_MAX_DEEPSLEEP_DIVIDER_ID: c_int = 5;
pub const MAX_REGULAR_DPM_NUMBER: c_int = 16;
pub const MAX_PCIE_CONF: c_int = 2;
pub const VEGA20_MINIMUM_ENGINE_CLOCK: c_int = 2500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_max_sustainable_clocks {
    pub display_clock: PP_Clock,
    pub phy_clock: PP_Clock,
    pub pixel_clock: PP_Clock,
    pub uclock: PP_Clock,
    pub dcef_clock: PP_Clock,
    pub soc_clock: PP_Clock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_dpm_state {
    pub soft_min_level: u32,
    pub soft_max_level: u32,
    pub hard_min_level: u32,
    pub hard_max_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_single_dpm_table {
    pub count: u32,
    pub dpm_state: vega20_dpm_state,
    pub dpm_levels: [vega20_dpm_level; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_odn_dpm_control {
    pub count: u32,
    pub entries: [u32; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_pcie_table {
    pub count: u16,
    pub pcie_gen: [u8; MAX_PCIE_CONF],
    pub pcie_lane: [u8; MAX_PCIE_CONF],
    pub lclk: [u32; MAX_PCIE_CONF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_dpm_table {
    pub soc_table: vega20_single_dpm_table,
    pub gfx_table: vega20_single_dpm_table,
    pub mem_table: vega20_single_dpm_table,
    pub eclk_table: vega20_single_dpm_table,
    pub vclk_table: vega20_single_dpm_table,
    pub dclk_table: vega20_single_dpm_table,
    pub dcef_table: vega20_single_dpm_table,
    pub pixel_table: vega20_single_dpm_table,
    pub display_table: vega20_single_dpm_table,
    pub phy_table: vega20_single_dpm_table,
    pub fclk_table: vega20_single_dpm_table,
    pub pcie_table: vega20_pcie_table,
}

pub const VEGA20_MAX_LEAKAGE_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_leakage_voltage {
    pub count: u16,
    pub leakage_id: [u16; VEGA20_MAX_LEAKAGE_COUNT],
    pub actual_voltage: [u16; VEGA20_MAX_LEAKAGE_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_display_timing {
    pub min_clock_in_sr: u32,
    pub num_existing_displays: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_dpmlevel_enable_mask {
    pub uvd_dpm_enable_mask: u32,
    pub vce_dpm_enable_mask: u32,
    pub samu_dpm_enable_mask: u32,
    pub sclk_dpm_enable_mask: u32,
    pub mclk_dpm_enable_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_vbios_boot_state {
    pub uc_cooling_id: u8,
    pub vddc: u16,
    pub vddci: u16,
    pub mvddc: u16,
    pub vdd_gfx: u16,
    pub gfx_clock: u32,
    pub mem_clock: u32,
    pub soc_clock: u32,
    pub dcef_clock: u32,
    pub eclock: u32,
    pub dclock: u32,
    pub vclock: u32,
    pub fclock: u32,
}

pub const DPMTABLE_OD_UPDATE_SCLK: c_uint = 0x00000001;
pub const DPMTABLE_OD_UPDATE_MCLK: c_uint = 0x00000002;
pub const DPMTABLE_UPDATE_SCLK: c_uint = 0x00000004;
pub const DPMTABLE_UPDATE_MCLK: c_uint = 0x00000008;
pub const DPMTABLE_OD_UPDATE_VDDC: c_uint = 0x00000010;
pub const DPMTABLE_OD_UPDATE_SCLK_MASK: c_uint = 0x00000020;
pub const DPMTABLE_OD_UPDATE_MCLK_MASK: c_uint = 0x00000040;
// To determine if sclk and mclk are in overdrive state
pub const SCLK_MASK_OVERDRIVE_ENABLED: c_uint = 0x00000008;
pub const MCLK_MASK_OVERDRIVE_ENABLED: c_uint = 0x00000010;
pub const SOCCLK_OVERDRIVE_ENABLED: c_uint = 0x00000020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_smc_state_table {
    pub soc_boot_level: u32,
    pub gfx_boot_level: u32,
    pub dcef_boot_level: u32,
    pub mem_boot_level: u32,
    pub uvd_boot_level: u32,
    pub vce_boot_level: u32,
    pub gfx_max_level: u32,
    pub mem_max_level: u32,
    pub vr_hot_gpio: u8,
    pub ac_dc_gpio: u8,
    pub therm_out_gpio: u8,
    pub therm_out_polarity: u8,
    pub therm_out_mode: u8,
    pub pp_table: PPTable_t,
    pub water_marks_table: Watermarks_t,
    pub avfs_debug_table: AvfsDebugTable_t,
    pub avfs_fuse_override_table: AvfsFuseOverride_t,
    pub smu_metrics: SmuMetrics_t,
    pub driver_smu_config: DriverSmuConfig_t,
    pub dpm_activity_monitor_coeffint: DpmActivityMonitorCoeffInt_t,
    pub overdrive_table: OverDriveTable_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_mclk_latency_entries {
    pub frequency: u32,
    pub latency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_mclk_latency_table {
    pub count: u32,
    pub entries: [vega20_mclk_latency_entries; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_registry_data {
    pub disallowed_features: u64,
    pub ac_dc_switch_gpio_support: u8,
    pub acg_loop_support: u8,
    pub clock_stretcher_support: u8,
    pub db_ramping_support: u8,
    pub didt_mode: u8,
    pub didt_support: u8,
    pub edc_didt_support: u8,
    pub force_dpm_high: u8,
    pub fuzzy_fan_control_support: u8,
    pub mclk_dpm_key_disabled: u8,
    pub od_state_in_dc_support: u8,
    pub pcie_lane_override: u8,
    pub pcie_speed_override: u8,
    pub pcie_clock_override: u32,
    pub pcie_dpm_key_disabled: u8,
    pub dcefclk_dpm_key_disabled: u8,
    pub prefetcher_dpm_key_disabled: u8,
    pub quick_transition_support: u8,
    pub regulator_hot_gpio_support: u8,
    pub master_deep_sleep_support: u8,
    pub gfx_clk_deep_sleep_support: u8,
    pub sclk_deep_sleep_support: u8,
    pub lclk_deep_sleep_support: u8,
    pub dce_fclk_deep_sleep_support: u8,
    pub sclk_dpm_key_disabled: u8,
    pub sclk_throttle_low_notification: u8,
    pub skip_baco_hardware: u8,
    pub socclk_dpm_key_disabled: u8,
    pub sq_ramping_support: u8,
    pub tcp_ramping_support: u8,
    pub td_ramping_support: u8,
    pub dbr_ramping_support: u8,
    pub gc_didt_support: u8,
    pub psm_didt_support: u8,
    pub thermal_support: u8,
    pub fw_ctf_enabled: u8,
    pub led_dpm_enabled: u8,
    pub fan_control_support: u8,
    pub ulv_support: u8,
    pub od8_feature_enable: u8,
    pub disable_water_mark: u8,
    pub disable_workload_policy: u8,
    pub force_workload_policy_mask: u32,
    pub disable_3d_fs_detection: u8,
    pub disable_pp_tuning: u8,
    pub disable_xlpp_tuning: u8,
    pub perf_ui_tuning_profile_turbo: u32,
    pub perf_ui_tuning_profile_powerSave: u32,
    pub perf_ui_tuning_profile_xl: u32,
    pub zrpm_stop_temp: u16,
    pub zrpm_start_temp: u16,
    pub stable_pstate_sclk_dpm_percentage: u32,
    pub fps_support: u8,
    pub vr0hot: u8,
    pub vr1hot: u8,
    pub disable_auto_wattman: u8,
    pub auto_wattman_debug: u32,
    pub auto_wattman_sample_period: u32,
    pub fclk_gfxclk_ratio: u32,
    pub auto_wattman_threshold: u8,
    pub log_avfs_param: u8,
    pub enable_enginess: u8,
    pub custom_fan_support: u8,
    pub disable_pcc_limit_control: u8,
    pub gfxoff_controlled_by_driver: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_odn_clock_voltage_dependency_table {
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_odn_dpm_table {
    pub control_gfxclk_state: vega20_odn_dpm_control,
    pub control_memclk_state: vega20_odn_dpm_control,
    pub odn_core_clock_dpm_levels: phm_odn_clock_levels,
    pub odn_memory_clock_dpm_levels: phm_odn_clock_levels,
    pub vdd_dependency_on_sclk: vega20_odn_clock_voltage_dependency_table,
    pub vdd_dependency_on_mclk: vega20_odn_clock_voltage_dependency_table,
    pub vdd_dependency_on_socclk: vega20_odn_clock_voltage_dependency_table,
    pub odn_mclk_min_limit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_odn_fan_table {
    pub target_fan_speed: u32,
    pub target_temperature: u32,
    pub min_performance_clock: u32,
    pub min_fan_limit: u32,
    pub force_fan_pwm: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_odn_temp_table {
    pub target_operating_temp: u16,
    pub default_target_operating_temp: u16,
    pub operating_temp_min_limit: u16,
    pub operating_temp_max_limit: u16,
    pub operating_temp_step: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_odn_data {
    pub apply_overdrive_next_settings_mask: u32,
    pub overdrive_next_state: u32,
    pub overdrive_next_capabilities: u32,
    pub odn_sclk_dpm_enable_mask: u32,
    pub odn_mclk_dpm_enable_mask: u32,
    pub odn_dpm_table: vega20_odn_dpm_table,
    pub odn_fan_table: vega20_odn_fan_table,
    pub odn_temp_table: vega20_odn_temp_table,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OD8_FEATURE_ID {
    OD8_GFXCLK_LIMITS               = 1 << 0,
    OD8_GFXCLK_CURVE                = 1 << 1,
    OD8_UCLK_MAX                    = 1 << 2,
    OD8_POWER_LIMIT                 = 1 << 3,
    OD8_ACOUSTIC_LIMIT_SCLK         = 1 << 4,   //FanMaximumRpm
    OD8_FAN_SPEED_MIN               = 1 << 5,   //FanMinimumPwm
    OD8_TEMPERATURE_FAN             = 1 << 6,   //FanTargetTemperature
    OD8_TEMPERATURE_SYSTEM          = 1 << 7,   //MaxOpTemp
    OD8_MEMORY_TIMING_TUNE          = 1 << 8,
    OD8_FAN_ZERO_RPM_CONTROL        = 1 << 9
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OD8_SETTING_ID {
    OD8_SETTING_GFXCLK_FMIN = 0,
    OD8_SETTING_GFXCLK_FMAX,
    OD8_SETTING_GFXCLK_FREQ1,
    OD8_SETTING_GFXCLK_VOLTAGE1,
    OD8_SETTING_GFXCLK_FREQ2,
    OD8_SETTING_GFXCLK_VOLTAGE2,
    OD8_SETTING_GFXCLK_FREQ3,
    OD8_SETTING_GFXCLK_VOLTAGE3,
    OD8_SETTING_UCLK_FMAX,
    OD8_SETTING_POWER_PERCENTAGE,
    OD8_SETTING_FAN_ACOUSTIC_LIMIT,
    OD8_SETTING_FAN_MIN_SPEED,
    OD8_SETTING_FAN_TARGET_TEMP,
    OD8_SETTING_OPERATING_TEMP_MAX,
    OD8_SETTING_AC_TIMING,
    OD8_SETTING_FAN_ZERO_RPM_CONTROL,
    OD8_SETTING_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_od8_single_setting {
    pub feature_id: u32,
    pub min_value: i32,
    pub max_value: i32,
    pub current_value: i32,
    pub default_value: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_od8_settings {
    pub overdrive8_capabilities: u32,
    pub od8_settings_array: [vega20_od8_single_setting; OD8_SETTING_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega20_hwmgr {
    pub dpm_table: vega20_dpm_table,
    pub golden_dpm_table: vega20_dpm_table,
    pub registry_data: vega20_registry_data,
    pub vbios_boot_state: vega20_vbios_boot_state,
    pub mclk_latency_table: vega20_mclk_latency_table,
    pub max_sustainable_clocks: vega20_max_sustainable_clocks,
    pub vddc_leakage: vega20_leakage_voltage,
    pub vddc_control: u32,
    pub vddc_voltage_table: pp_atomfwctrl_voltage_table,
    pub mvdd_control: u32,
    pub mvdd_voltage_table: pp_atomfwctrl_voltage_table,
    pub vddci_control: u32,
    pub vddci_voltage_table: pp_atomfwctrl_voltage_table,
    pub active_auto_throttle_sources: u32,
    pub bacos: vega20_bacos,
// ---- General data ----
    pub need_update_dpm_table: u8,
    pub cac_enabled: bool,
    pub battery_state: bool,
    pub is_tlu_enabled: bool,
    pub avfs_exist: bool,
    pub low_sclk_interrupt_threshold: u32,
    pub total_active_cus: u32,
    pub water_marks_bitmap: u32,
    pub display_timing: vega20_display_timing,
// ---- Vega20 Dyn Register Settings ----
    pub debug_settings: u32,
    pub lowest_uclk_reserved_for_ulv: u32,
    pub gfxclk_average_alpha: u32,
    pub socclk_average_alpha: u32,
    pub uclk_average_alpha: u32,
    pub gfx_activity_average_alpha: u32,
    pub display_voltage_mode: u32,
    pub dcef_clk_quad_eqn_a: u32,
    pub dcef_clk_quad_eqn_b: u32,
    pub dcef_clk_quad_eqn_c: u32,
    pub disp_clk_quad_eqn_a: u32,
    pub disp_clk_quad_eqn_b: u32,
    pub disp_clk_quad_eqn_c: u32,
    pub pixel_clk_quad_eqn_a: u32,
    pub pixel_clk_quad_eqn_b: u32,
    pub pixel_clk_quad_eqn_c: u32,
    pub phy_clk_quad_eqn_a: u32,
    pub phy_clk_quad_eqn_b: u32,
    pub phy_clk_quad_eqn_c: u32,
// ---- Thermal Temperature Setting ----
    pub dpm_level_enable_mask: vega20_dpmlevel_enable_mask,
// ---- Power Gating States ----
    pub uvd_power_gated: bool,
    pub vce_power_gated: bool,
    pub samu_power_gated: bool,
    pub need_long_memory_training: bool,
// Internal settings to apply the application power optimization parameters
    pub apply_optimized_settings: bool,
    pub disable_dpm_mask: u32,
// ---- Overdrive next setting ----
    pub odn_data: vega20_odn_data,
    pub gfxclk_overdrive: bool,
    pub memclk_overdrive: bool,
// ---- Overdrive8 Setting ----
    pub od8_settings: vega20_od8_settings,
// ---- Workload Mask ----
    pub workload_mask: u32,
// ---- SMU9 ----
    pub smu_version: u32,
    pub smu_features: [smu_features; GNLD_FEATURES_MAX],
    pub smc_state_table: vega20_smc_state_table,
// ---- Gfxoff ----
    pub gfxoff_allowed: bool,
    pub counter_gfxoff: u32,
    pub metrics_time: c_ulong,
    pub metrics_table: SmuMetrics_t,
    pub gpu_metrics_table: gpu_metrics_v1_0,
    pub pcie_parameters_override: bool,
    pub pcie_gen_level1: u32,
    pub pcie_width_level1: u32,
    pub is_custom_profile_set: bool,
}

pub const VEGA20_DPM2_NEAR_TDP_DEC: c_int = 10;
pub const VEGA20_DPM2_ABOVE_SAFE_INC: c_int = 5;
pub const VEGA20_DPM2_BELOW_SAFE_INC: c_int = 20;
pub const VEGA20_DPM2_LTA_WINDOW_SIZE: c_int = 7;
pub const VEGA20_DPM2_LTS_TRUNCATE: c_int = 0;
pub const VEGA20_DPM2_TDP_SAFE_LIMIT_PERCENT: c_int = 80;
pub const VEGA20_DPM2_MAXPS_PERCENT_M: c_int = 90;
pub const VEGA20_DPM2_MAXPS_PERCENT_H: c_int = 90;
pub const VEGA20_DPM2_PWREFFICIENCYRATIO_MARGIN: c_int = 50;
pub const VEGA20_DPM2_SQ_RAMP_MAX_POWER: c_uint = 0x3FFF;
pub const VEGA20_DPM2_SQ_RAMP_MIN_POWER: c_uint = 0x12;
pub const VEGA20_DPM2_SQ_RAMP_MAX_POWER_DELTA: c_uint = 0x15;
pub const VEGA20_DPM2_SQ_RAMP_SHORT_TERM_INTERVAL_SIZE: c_uint = 0x1E;
pub const VEGA20_DPM2_SQ_RAMP_LONG_TERM_INTERVAL_RATIO: c_uint = 0xF;
pub const VEGA20_VOLTAGE_CONTROL_NONE: c_uint = 0x0;
pub const VEGA20_VOLTAGE_CONTROL_BY_GPIO: c_uint = 0x1;
pub const VEGA20_VOLTAGE_CONTROL_BY_SVID2: c_uint = 0x2;
pub const VEGA20_VOLTAGE_CONTROL_MERGED: c_uint = 0x3;
// To convert to Q8.8 format for firmware
pub const VEGA20_Q88_FORMAT_CONVERSION_UNIT: c_int = 256;
pub const VEGA20_UNUSED_GPIO_PIN: c_uint = 0x7F;
pub const VEGA20_THERM_OUT_MODE_DISABLE: c_uint = 0x0;
pub const VEGA20_THERM_OUT_MODE_THERM_ONLY: c_uint = 0x1;
pub const VEGA20_THERM_OUT_MODE_THERM_VRHOT: c_uint = 0x2;
pub const PPVEGA20_VEGA20DISPLAYVOLTAGEMODE_DFLT: c_uint = 0xffffffff;
pub const PPREGKEY_VEGA20QUADRATICEQUATION_DFLT: c_uint = 0xffffffff;

pub const PPVEGA20_VEGA20LOWESTUCLKRESERVEDFORULV_DFLT: c_uint = 0xffffffff;
pub const PPVEGA20_VEGA20DISPLAYVOLTAGEMODE_DFLT: c_uint = 0xffffffff;
pub const PPREGKEY_VEGA20QUADRATICEQUATION_DFLT: c_uint = 0xffffffff;
pub const VEGA20_UMD_PSTATE_GFXCLK_LEVEL: c_uint = 0x3;
pub const VEGA20_UMD_PSTATE_SOCCLK_LEVEL: c_uint = 0x3;
pub const VEGA20_UMD_PSTATE_MCLK_LEVEL: c_uint = 0x2;
pub const VEGA20_UMD_PSTATE_UVDCLK_LEVEL: c_uint = 0x3;
pub const VEGA20_UMD_PSTATE_VCEMCLK_LEVEL: c_uint = 0x3;
