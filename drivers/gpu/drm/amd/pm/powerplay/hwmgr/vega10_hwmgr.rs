//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/vega10_hwmgr.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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

pub const VEGA10_MAX_HARDWARE_POWERLEVELS: c_int = 2;
pub const WaterMarksExist: c_int = 1;
pub const WaterMarksLoaded: c_int = 2;

pub const SMC_DPM_FEATURES: c_uint = 0x30F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_features {
    pub supported: bool,
    pub enabled: bool,
    pub smu_feature_id: u32,
    pub smu_feature_bitmap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_performance_level {
    pub soc_clock: u32,
    pub gfx_clock: u32,
    pub mem_clock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_bacos {
    pub baco_flags: u32,
// struct vega10_performance_level  performance_level;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_uvd_clocks {
    pub vclk: u32,
    pub dclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_vce_clocks {
    pub evclk: u32,
    pub ecclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_power_state {
    pub magic: u32,
    pub uvd_clks: vega10_uvd_clocks,
    pub vce_clks: vega10_vce_clocks,
    pub performance_level_count: u16,
    pub dc_compatible: bool,
    pub sclk_threshold: u32,
    pub performance_levels: [vega10_performance_level; VEGA10_MAX_HARDWARE_POWERLEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_dpm_level {
    pub enabled: bool,
    pub value: u32,
    pub param1: u32,
}

pub const VEGA10_MAX_DEEPSLEEP_DIVIDER_ID: c_int = 5;
pub const MAX_REGULAR_DPM_NUMBER: c_int = 8;
pub const MAX_PCIE_CONF: c_int = 2;
pub const VEGA10_MINIMUM_ENGINE_CLOCK: c_int = 2500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_dpm_state {
    pub soft_min_level: u32,
    pub soft_max_level: u32,
    pub hard_min_level: u32,
    pub hard_max_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_single_dpm_table {
    pub count: u32,
    pub dpm_state: vega10_dpm_state,
    pub dpm_levels: [vega10_dpm_level; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_pcie_table {
    pub count: u16,
    pub pcie_gen: [u8; MAX_PCIE_CONF],
    pub pcie_lane: [u8; MAX_PCIE_CONF],
    pub lclk: [u32; MAX_PCIE_CONF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_dpm_table {
    pub soc_table: vega10_single_dpm_table,
    pub gfx_table: vega10_single_dpm_table,
    pub mem_table: vega10_single_dpm_table,
    pub eclk_table: vega10_single_dpm_table,
    pub vclk_table: vega10_single_dpm_table,
    pub dclk_table: vega10_single_dpm_table,
    pub dcef_table: vega10_single_dpm_table,
    pub pixel_table: vega10_single_dpm_table,
    pub display_table: vega10_single_dpm_table,
    pub phy_table: vega10_single_dpm_table,
    pub pcie_table: vega10_pcie_table,
}

pub const VEGA10_MAX_LEAKAGE_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_leakage_voltage {
    pub count: u16,
    pub leakage_id: [u16; VEGA10_MAX_LEAKAGE_COUNT],
    pub actual_voltage: [u16; VEGA10_MAX_LEAKAGE_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_display_timing {
    pub min_clock_in_sr: u32,
    pub num_existing_displays: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_dpmlevel_enable_mask {
    pub uvd_dpm_enable_mask: u32,
    pub vce_dpm_enable_mask: u32,
    pub acp_dpm_enable_mask: u32,
    pub samu_dpm_enable_mask: u32,
    pub sclk_dpm_enable_mask: u32,
    pub mclk_dpm_enable_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_vbios_boot_state {
    pub bsoc_vddc_lock: bool,
    pub vddc: u16,
    pub vddci: u16,
    pub mvddc: u16,
    pub vdd_gfx: u16,
    pub gfx_clock: u32,
    pub mem_clock: u32,
    pub soc_clock: u32,
    pub dcef_clock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_smc_state_table {
    pub soc_boot_level: u32,
    pub gfx_boot_level: u32,
    pub dcef_boot_level: u32,
    pub mem_boot_level: u32,
    pub uvd_boot_level: u32,
    pub vce_boot_level: u32,
    pub gfx_max_level: u32,
    pub mem_max_level: u32,
    pub soc_max_level: u32,
    pub vr_hot_gpio: u8,
    pub ac_dc_gpio: u8,
    pub therm_out_gpio: u8,
    pub therm_out_polarity: u8,
    pub therm_out_mode: u8,
    pub pp_table: PPTable_t,
    pub water_marks_table: Watermarks_t,
    pub avfs_table: AvfsTable_t,
    pub avfs_fuse_override_table: AvfsFuseOverride_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_mclk_latency_entries {
    pub frequency: u32,
    pub latency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_mclk_latency_table {
    pub count: u32,
    pub entries: [vega10_mclk_latency_entries; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_registry_data {
    pub ac_dc_switch_gpio_support: u8,
    pub avfs_support: u8,
    pub cac_support: u8,
    pub clock_stretcher_support: u8,
    pub db_ramping_support: u8,
    pub didt_mode: u8,
    pub didt_support: u8,
    pub edc_didt_support: u8,
    pub dynamic_state_patching_support: u8,
    pub enable_pkg_pwr_tracking_feature: u8,
    pub enable_tdc_limit_feature: u8,
    pub fast_watermark_threshold: u32,
    pub force_dpm_high: u8,
    pub fuzzy_fan_control_support: u8,
    pub long_idle_baco_support: u8,
    pub mclk_dpm_key_disabled: u8,
    pub od_state_in_dc_support: u8,
    pub pcieLaneOverride: u8,
    pub pcieSpeedOverride: u8,
    pub pcieClockOverride: u32,
    pub pcie_dpm_key_disabled: u8,
    pub dcefclk_dpm_key_disabled: u8,
    pub power_containment_support: u8,
    pub ppt_support: u8,
    pub prefetcher_dpm_key_disabled: u8,
    pub quick_transition_support: u8,
    pub regulator_hot_gpio_support: u8,
    pub sclk_deep_sleep_support: u8,
    pub sclk_dpm_key_disabled: u8,
    pub sclk_from_vbios: u8,
    pub sclk_throttle_low_notification: u8,
    pub show_baco_dbg_info: u8,
    pub skip_baco_hardware: u8,
    pub socclk_dpm_key_disabled: u8,
    pub spll_shutdown_support: u8,
    pub sq_ramping_support: u8,
    pub stable_pstate_sclk_dpm_percentage: u32,
    pub tcp_ramping_support: u8,
    pub tdc_support: u8,
    pub td_ramping_support: u8,
    pub dbr_ramping_support: u8,
    pub gc_didt_support: u8,
    pub psm_didt_support: u8,
    pub thermal_out_gpio_support: u8,
    pub thermal_support: u8,
    pub fw_ctf_enabled: u8,
    pub fan_control_support: u8,
    pub ulps_support: u8,
    pub ulv_support: u8,
    pub vddc_vddci_delta: u32,
    pub odn_feature_enable: u8,
    pub disable_water_mark: u8,
    pub zrpm_stop_temp: u8,
    pub zrpm_start_temp: u8,
    pub led_dpm_enabled: u8,
    pub vr0hot_enabled: u8,
    pub vr1hot_enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_odn_clock_voltage_dependency_table {
    pub count: u32,
    pub entries: [phm_ppt_v1_clock_voltage_dependency_record; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_odn_vddc_lookup_table {
    pub count: u32,
    pub entries: [phm_ppt_v1_voltage_lookup_record; MAX_REGULAR_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_odn_dpm_table {
    pub vdd_dep_on_sclk: vega10_odn_clock_voltage_dependency_table,
    pub vdd_dep_on_mclk: vega10_odn_clock_voltage_dependency_table,
    pub vdd_dep_on_socclk: vega10_odn_clock_voltage_dependency_table,
    pub vddc_lookup_table: vega10_odn_vddc_lookup_table,
    pub max_vddc: u32,
    pub min_vddc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_odn_fan_table {
    pub target_fan_speed: u32,
    pub target_temperature: u32,
    pub min_performance_clock: u32,
    pub min_fan_limit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vega10_hwmgr {
    pub dpm_table: vega10_dpm_table,
    pub golden_dpm_table: vega10_dpm_table,
    pub registry_data: vega10_registry_data,
    pub vbios_boot_state: vega10_vbios_boot_state,
    pub mclk_latency_table: vega10_mclk_latency_table,
    pub vddc_leakage: vega10_leakage_voltage,
    pub vddc_control: u32,
    pub vddc_voltage_table: pp_atomfwctrl_voltage_table,
    pub mvdd_control: u32,
    pub mvdd_voltage_table: pp_atomfwctrl_voltage_table,
    pub vddci_control: u32,
    pub vddci_voltage_table: pp_atomfwctrl_voltage_table,
    pub active_auto_throttle_sources: u32,
    pub water_marks_bitmap: u32,
    pub bacos: vega10_bacos,
    pub odn_dpm_table: vega10_odn_dpm_table,
    pub odn_fan_table: vega10_odn_fan_table,
// ---- General data ----
    pub need_update_dpm_table: u8,
    pub cac_enabled: bool,
    pub battery_state: bool,
    pub is_tlu_enabled: bool,
    pub low_sclk_interrupt_threshold: u32,
    pub total_active_cus: u32,
    pub display_timing: vega10_display_timing,
// ---- Vega10 Dyn Register Settings ----
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
    pub dpm_level_enable_mask: vega10_dpmlevel_enable_mask,
// ---- Power Gating States ----
    pub uvd_power_gated: bool,
    pub vce_power_gated: bool,
    pub need_long_memory_training: bool,
// Internal settings to apply the application power optimization parameters
    pub disable_dpm_mask: u32,
// ---- SMU9 ----
    pub smu_features: [smu_features; GNLD_FEATURES_MAX],
    pub smc_state_table: vega10_smc_state_table,
    pub config_telemetry: u32,
    pub acg_loop_state: u32,
    pub mem_channels: u32,
    pub custom_profile_mode: [u8; 4],
}

pub const VEGA10_DPM2_NEAR_TDP_DEC: c_int = 10;
pub const VEGA10_DPM2_ABOVE_SAFE_INC: c_int = 5;
pub const VEGA10_DPM2_BELOW_SAFE_INC: c_int = 20;
pub const VEGA10_DPM2_LTA_WINDOW_SIZE: c_int = 7;
pub const VEGA10_DPM2_LTS_TRUNCATE: c_int = 0;
pub const VEGA10_DPM2_TDP_SAFE_LIMIT_PERCENT: c_int = 80;
pub const VEGA10_DPM2_MAXPS_PERCENT_M: c_int = 90;
pub const VEGA10_DPM2_MAXPS_PERCENT_H: c_int = 90;
pub const VEGA10_DPM2_PWREFFICIENCYRATIO_MARGIN: c_int = 50;
pub const VEGA10_DPM2_SQ_RAMP_MAX_POWER: c_uint = 0x3FFF;
pub const VEGA10_DPM2_SQ_RAMP_MIN_POWER: c_uint = 0x12;
pub const VEGA10_DPM2_SQ_RAMP_MAX_POWER_DELTA: c_uint = 0x15;
pub const VEGA10_DPM2_SQ_RAMP_SHORT_TERM_INTERVAL_SIZE: c_uint = 0x1E;
pub const VEGA10_DPM2_SQ_RAMP_LONG_TERM_INTERVAL_RATIO: c_uint = 0xF;
pub const VEGA10_VOLTAGE_CONTROL_NONE: c_uint = 0x0;
pub const VEGA10_VOLTAGE_CONTROL_BY_GPIO: c_uint = 0x1;
pub const VEGA10_VOLTAGE_CONTROL_BY_SVID2: c_uint = 0x2;
pub const VEGA10_VOLTAGE_CONTROL_MERGED: c_uint = 0x3;
// To convert to Q8.8 format for firmware
pub const VEGA10_Q88_FORMAT_CONVERSION_UNIT: c_int = 256;
pub const VEGA10_UNUSED_GPIO_PIN: c_uint = 0x7F;
pub const VEGA10_THERM_OUT_MODE_DISABLE: c_uint = 0x0;
pub const VEGA10_THERM_OUT_MODE_THERM_ONLY: c_uint = 0x1;
pub const VEGA10_THERM_OUT_MODE_THERM_VRHOT: c_uint = 0x2;
pub const PPVEGA10_VEGA10DISPLAYVOLTAGEMODE_DFLT: c_uint = 0xffffffff;
pub const PPREGKEY_VEGA10QUADRATICEQUATION_DFLT: c_uint = 0xffffffff;

pub const VEGA10_UMD_PSTATE_GFXCLK_LEVEL: c_uint = 0x3;
pub const VEGA10_UMD_PSTATE_SOCCLK_LEVEL: c_uint = 0x3;
pub const VEGA10_UMD_PSTATE_MCLK_LEVEL: c_uint = 0x2;
extern "C" {
    pub fn tonga_initializa_dynamic_state_adjustment_rule_settings(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn tonga_hwmgr_backend_fini(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn tonga_get_mc_microcode_version(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn tonga_notify_smc_display_config_after_ps_adjustment(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn tonga_notify_smc_display_change(hwmgr: *mut pp_hwmgr, has_display: bool) -> c_int;
}
extern "C" {
    pub fn vega10_update_vce_dpm(hwmgr: *mut pp_hwmgr, input: *const c_void) -> c_int;
}
extern "C" {
    pub fn vega10_update_uvd_dpm(hwmgr: *mut pp_hwmgr, bgate: bool) -> c_int;
}
extern "C" {
    pub fn vega10_update_samu_dpm(hwmgr: *mut pp_hwmgr, bgate: bool) -> c_int;
}
extern "C" {
    pub fn vega10_update_acp_dpm(hwmgr: *mut pp_hwmgr, bgate: bool) -> c_int;
}
extern "C" {
    pub fn vega10_enable_disable_vce_dpm(hwmgr: *mut pp_hwmgr, enable: bool) -> c_int;
}
extern "C" {
    pub fn vega10_hwmgr_init(hwmgr: *mut pp_hwmgr) -> c_int;
}
