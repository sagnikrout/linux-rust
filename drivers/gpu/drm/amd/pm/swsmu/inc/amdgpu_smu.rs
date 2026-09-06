//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/amdgpu_smu.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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

pub const SMU_THERMAL_MINIMUM_ALERT_TEMP: c_int = 0;
pub const SMU_THERMAL_MAXIMUM_ALERT_TEMP: c_int = 255;

pub const SMU_FW_NAME_LEN: c_uint = 0x24;

pub const SMU_GPU_METRICS_CACHE_INTERVAL: c_int = 5;
// Power Throttlers
pub const SMU_THROTTLER_PPT0_BIT: c_int = 0;
pub const SMU_THROTTLER_PPT1_BIT: c_int = 1;
pub const SMU_THROTTLER_PPT2_BIT: c_int = 2;
pub const SMU_THROTTLER_PPT3_BIT: c_int = 3;
pub const SMU_THROTTLER_SPL_BIT: c_int = 4;
pub const SMU_THROTTLER_FPPT_BIT: c_int = 5;
pub const SMU_THROTTLER_SPPT_BIT: c_int = 6;
pub const SMU_THROTTLER_SPPT_APU_BIT: c_int = 7;
// Current Throttlers
pub const SMU_THROTTLER_TDC_GFX_BIT: c_int = 16;
pub const SMU_THROTTLER_TDC_SOC_BIT: c_int = 17;
pub const SMU_THROTTLER_TDC_MEM_BIT: c_int = 18;
pub const SMU_THROTTLER_TDC_VDD_BIT: c_int = 19;
pub const SMU_THROTTLER_TDC_CVIP_BIT: c_int = 20;
pub const SMU_THROTTLER_EDC_CPU_BIT: c_int = 21;
pub const SMU_THROTTLER_EDC_GFX_BIT: c_int = 22;
pub const SMU_THROTTLER_APCC_BIT: c_int = 23;
// Temperature
pub const SMU_THROTTLER_TEMP_GPU_BIT: c_int = 32;
pub const SMU_THROTTLER_TEMP_CORE_BIT: c_int = 33;
pub const SMU_THROTTLER_TEMP_MEM_BIT: c_int = 34;
pub const SMU_THROTTLER_TEMP_EDGE_BIT: c_int = 35;
pub const SMU_THROTTLER_TEMP_HOTSPOT_BIT: c_int = 36;
pub const SMU_THROTTLER_TEMP_SOC_BIT: c_int = 37;
pub const SMU_THROTTLER_TEMP_VR_GFX_BIT: c_int = 38;
pub const SMU_THROTTLER_TEMP_VR_SOC_BIT: c_int = 39;
pub const SMU_THROTTLER_TEMP_VR_MEM0_BIT: c_int = 40;
pub const SMU_THROTTLER_TEMP_VR_MEM1_BIT: c_int = 41;
pub const SMU_THROTTLER_TEMP_LIQUID0_BIT: c_int = 42;
pub const SMU_THROTTLER_TEMP_LIQUID1_BIT: c_int = 43;
pub const SMU_THROTTLER_VRHOT0_BIT: c_int = 44;
pub const SMU_THROTTLER_VRHOT1_BIT: c_int = 45;
pub const SMU_THROTTLER_PROCHOT_CPU_BIT: c_int = 46;
pub const SMU_THROTTLER_PROCHOT_GFX_BIT: c_int = 47;
// Other
pub const SMU_THROTTLER_PPM_BIT: c_int = 56;
pub const SMU_THROTTLER_FIT_BIT: c_int = 57;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_hw_power_state {
    pub magic: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_state_ui_label {
    SMU_STATE_UI_LABEL_NONE,
    SMU_STATE_UI_LABEL_BATTERY,
    SMU_STATE_UI_TABEL_MIDDLE_LOW,
    SMU_STATE_UI_LABEL_BALLANCED,
    SMU_STATE_UI_LABEL_MIDDLE_HIGHT,
    SMU_STATE_UI_LABEL_PERFORMANCE,
    SMU_STATE_UI_LABEL_BACO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_state_classification_flag {
    SMU_STATE_CLASSIFICATION_FLAG_BOOT                     = 0x0001,
    SMU_STATE_CLASSIFICATION_FLAG_THERMAL                  = 0x0002,
    SMU_STATE_CLASSIFICATIN_FLAG_LIMITED_POWER_SOURCE      = 0x0004,
    SMU_STATE_CLASSIFICATION_FLAG_RESET                    = 0x0008,
    SMU_STATE_CLASSIFICATION_FLAG_FORCED                   = 0x0010,
    SMU_STATE_CLASSIFICATION_FLAG_USER_3D_PERFORMANCE      = 0x0020,
    SMU_STATE_CLASSIFICATION_FLAG_USER_2D_PERFORMANCE      = 0x0040,
    SMU_STATE_CLASSIFICATION_FLAG_3D_PERFORMANCE           = 0x0080,
    SMU_STATE_CLASSIFICATION_FLAG_AC_OVERDIRVER_TEMPLATE   = 0x0100,
    SMU_STATE_CLASSIFICATION_FLAG_UVD                      = 0x0200,
    SMU_STATE_CLASSIFICATION_FLAG_3D_PERFORMANCE_LOW       = 0x0400,
    SMU_STATE_CLASSIFICATION_FLAG_ACPI                     = 0x0800,
    SMU_STATE_CLASSIFICATION_FLAG_HD2                      = 0x1000,
    SMU_STATE_CLASSIFICATION_FLAG_UVD_HD                   = 0x2000,
    SMU_STATE_CLASSIFICATION_FLAG_UVD_SD                   = 0x4000,
    SMU_STATE_CLASSIFICATION_FLAG_USER_DC_PERFORMANCE      = 0x8000,
    SMU_STATE_CLASSIFICATION_FLAG_DC_OVERDIRVER_TEMPLATE   = 0x10000,
    SMU_STATE_CLASSIFICATION_FLAG_BACO                     = 0x20000,
    SMU_STATE_CLASSIFICATIN_FLAG_LIMITED_POWER_SOURCE2      = 0x40000,
    SMU_STATE_CLASSIFICATION_FLAG_ULV                      = 0x80000,
    SMU_STATE_CLASSIFICATION_FLAG_UVD_MVC                  = 0x100000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_state_classification_block {
    pub ui_label: smu_state_ui_label,
    pub flags: smu_state_classification_flag,
    pub bios_index: c_int,
    pub temporary_state: bool,
    pub to_be_deleted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_state_pcie_block {
    pub lanes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_refreshrate_source {
    SMU_REFRESHRATE_SOURCE_EDID,
    SMU_REFRESHRATE_SOURCE_EXPLICIT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_state_display_block {
    pub disable_frame_modulation: bool,
    pub limit_refreshrate: bool,
    pub refreshrate_source: smu_refreshrate_source,
    pub explicit_refreshrate: c_int,
    pub edid_refreshrate_index: c_int,
    pub enable_vari_bright: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_state_memory_block {
    pub dll_off: bool,
    pub m3arb: u8,
    pub unused: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_state_software_algorithm_block {
    pub disable_load_balancing: bool,
    pub enable_sleep_for_timestamps: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_temperature_range {
    pub min: c_int,
    pub max: c_int,
    pub edge_emergency_max: c_int,
    pub hotspot_min: c_int,
    pub hotspot_crit_max: c_int,
    pub hotspot_emergency_max: c_int,
    pub mem_min: c_int,
    pub mem_crit_max: c_int,
    pub mem_emergency_max: c_int,
    pub software_shutdown_temp: c_int,
    pub software_shutdown_temp_offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_state_validation_block {
    pub single_display_only: bool,
    pub disallow_on_dc: bool,
    pub supported_power_levels: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_uvd_clocks {
    pub vclk: u32,
    pub dclk: u32,
}

//
// Structure to hold a SMU Power State.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_power_state {
    pub id: u32,
    pub ordered_list: list_head,
    pub all_states_list: list_head,
    pub classification: smu_state_classification_block,
    pub validation: smu_state_validation_block,
    pub pcie: smu_state_pcie_block,
    pub display: smu_state_display_block,
    pub memory: smu_state_memory_block,
    pub software: smu_state_software_algorithm_block,
    pub uvd_clocks: smu_uvd_clocks,
    pub hardware: smu_hw_power_state,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_power_src_type {
    SMU_POWER_SOURCE_AC,
    SMU_POWER_SOURCE_DC,
    SMU_POWER_SOURCE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_ppt_limit_type {
    SMU_PPT_LIMIT_PPT0 = 0,
    SMU_PPT_LIMIT_PPT1,
    SMU_LIMIT_TYPE_COUNT,
    SMU_DEFAULT_PPT_LIMIT = SMU_PPT_LIMIT_PPT0,
    SMU_SLOW_PPT_LIMIT = SMU_PPT_LIMIT_PPT0,
    SMU_FAST_PPT_LIMIT = SMU_PPT_LIMIT_PPT1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_ppt_limit_level {
    SMU_PPT_LIMIT_MIN = -1,
    SMU_PPT_LIMIT_CURRENT,
    SMU_PPT_LIMIT_DEFAULT,
    SMU_PPT_LIMIT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_ppt_limit_range {
    pub default_value: u32,
    pub min: u32,
    pub max: u32,
    pub od_min: u32,
    pub od_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_ppt_limit_context {
    pub supported_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_memory_pool_size {
    SMU_MEMORY_POOL_SIZE_ZERO   = 0,
    SMU_MEMORY_POOL_SIZE_256_MB = 0x10000000,
    SMU_MEMORY_POOL_SIZE_512_MB = 0x20000000,
    SMU_MEMORY_POOL_SIZE_1_GB   = 0x40000000,
    SMU_MEMORY_POOL_SIZE_2_GB   = 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_user_dpm_profile {
    pub fan_mode: u32,
    pub ppt_limits: [u32; SMU_POWER_SOURCE_COUNT][SMU_LIMIT_TYPE_COUNT],
    pub ppt_limit_user_mask: [u32; SMU_POWER_SOURCE_COUNT],
    pub fan_speed_pwm: u32,
    pub fan_speed_rpm: u32,
    pub flags: u32,
    pub user_od: u32,
// user clock state information
    pub clk_mask: [u32; SMU_CLK_COUNT],
    pub clk_dependency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_table_cache {
    pub buffer: *mut c_void,
    pub size: usize,
// interval in ms
    pub interval: u32,
    pub last_cache_time: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_table {
    pub size: u64,
    pub align: u32,
    pub domain: u8,
    pub mc_address: u64,
    pub cpu_addr: *mut c_void,
    pub bo: *mut amdgpu_bo,
    pub version: u32,
    pub cache: smu_table_cache,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_driver_table_id {
    SMU_DRIVER_TABLE_GPU_METRICS = 0,
    SMU_DRIVER_TABLE_GPUBOARD_TEMP_METRICS,
    SMU_DRIVER_TABLE_BASEBOARD_TEMP_METRICS,
    SMU_DRIVER_TABLE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_driver_table {
    pub id: smu_driver_table_id,
    pub cache: smu_table_cache,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_perf_level_designation {
    PERF_LEVEL_ACTIVITY,
    PERF_LEVEL_POWER_CONTAINMENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_performance_level {
    pub core_clock: u32,
    pub memory_clock: u32,
    pub vddc: u32,
    pub vddci: u32,
    pub non_local_mem_freq: u32,
    pub non_local_mem_width: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_clock_info {
    pub min_mem_clk: u32,
    pub max_mem_clk: u32,
    pub min_eng_clk: u32,
    pub max_eng_clk: u32,
    pub min_bus_bandwidth: u32,
    pub max_bus_bandwidth: u32,
}

pub const SMU_MAX_DPM_LEVELS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_dpm_clk_level {
    pub enabled: bool,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_dpm_table {
    pub clk_type: smu_clk_type,
    pub count: u32,
    pub flags: u32,
    pub dpm_levels: [smu_dpm_clk_level; SMU_MAX_DPM_LEVELS],
}

pub const SMU_MAX_PCIE_LEVELS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_pcie_table {
    pub pcie_gen: [u8; SMU_MAX_PCIE_LEVELS],
    pub pcie_lane: [u8; SMU_MAX_PCIE_LEVELS],
    pub lclk_freq: [u16; SMU_MAX_PCIE_LEVELS],
    pub lclk_levels: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_bios_boot_up_values {
    pub revision: u32,
    pub gfxclk: u32,
    pub uclk: u32,
    pub socclk: u32,
    pub dcefclk: u32,
    pub eclk: u32,
    pub vclk: u32,
    pub dclk: u32,
    pub vddc: u16,
    pub vddci: u16,
    pub mvddc: u16,
    pub vdd_gfx: u16,
    pub cooling_id: u8,
    pub pp_table_id: u32,
    pub format_revision: u32,
    pub content_revision: u32,
    pub fclk: u32,
    pub lclk: u32,
    pub firmware_caps: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_table_id {
    SMU_TABLE_PPTABLE = 0,
    SMU_TABLE_WATERMARKS,
    SMU_TABLE_CUSTOM_DPM,
    SMU_TABLE_DPMCLOCKS,
    SMU_TABLE_AVFS,
    SMU_TABLE_AVFS_PSM_DEBUG,
    SMU_TABLE_AVFS_FUSE_OVERRIDE,
    SMU_TABLE_PMSTATUSLOG,
    SMU_TABLE_SMU_METRICS,
    SMU_TABLE_DRIVER_SMU_CONFIG,
    SMU_TABLE_ACTIVITY_MONITOR_COEFF,
    SMU_TABLE_OVERDRIVE,
    SMU_TABLE_I2C_COMMANDS,
    SMU_TABLE_PACE,
    SMU_TABLE_ECCINFO,
    SMU_TABLE_COMBO_PPTABLE,
    SMU_TABLE_WIFIBAND,
    SMU_TABLE_PMFW_SYSTEM_METRICS,
    SMU_TABLE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_table_context {
    pub power_play_table: *mut c_void,
    pub power_play_table_size: u32,
    pub hardcode_pptable: *mut c_void,
    pub metrics_time: c_ulong,
    pub metrics_table: *mut c_void,
    pub clocks_table: *mut c_void,
    pub watermarks_table: *mut c_void,
    pub metrics_lock: mutex,
    pub max_sustainable_clocks: *mut c_void,
    pub boot_values: smu_bios_boot_up_values,
    pub driver_pptable: *mut c_void,
    pub combo_pptable: *mut c_void,
    pub ecc_table: *mut c_void,
    pub driver_smu_config_table: *mut c_void,
    pub tables: [smu_table; SMU_TABLE_COUNT],
//
// The driver table is just a staging buffer for
// uploading/downloading content from the SMU.
//
// And the table_id for SMU_MSG_TransferTableSmu2Dram
// SMU_MSG_TransferTableDram2Smu instructs SMU
// which content driver is interested.
//
    pub driver_table: smu_table,
    pub memory_pool: smu_table,
    pub dummy_read_1_table: smu_table,
    pub thermal_controller_type: u8,
    pub overdrive_table: *mut c_void,
    pub boot_overdrive_table: *mut c_void,
    pub user_overdrive_table: *mut c_void,
    pub driver_tables: [smu_driver_table; SMU_DRIVER_TABLE_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_dpm_policy_desc {
    pub name: *const c_char,
    pub level): *mut *mut *mut *mut char (get_desc)(struct smu_dpm_policy dpm_policy, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_dpm_policy {
    pub desc: *mut smu_dpm_policy_desc,
    pub policy_type: pp_pm_policy,
    pub level_mask: c_ulong,
    pub current_level: c_int,
    pub level): *mut *mut *mut int (set_policy)(struct smu_context ctxt, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_dpm_policy_ctxt {
    pub policies: [smu_dpm_policy; PP_PM_POLICY_NUM],
    pub policy_mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_dpm_context {
    pub dpm_context_size: u32,
    pub dpm_context: *mut c_void,
    pub golden_dpm_context: *mut c_void,
    pub dpm_level: amd_dpm_forced_level,
    pub saved_dpm_level: amd_dpm_forced_level,
    pub requested_dpm_level: amd_dpm_forced_level,
    pub dpm_request_power_state: *mut smu_power_state,
    pub dpm_current_power_state: *mut smu_power_state,
    pub mclk_latency_table: *mut mclock_latency_table,
    pub dpm_policies: *mut smu_dpm_policy_ctxt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_temp_context {
    pub temp_funcs: *const smu_temp_funcs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_power_gate {
    pub uvd_gated: bool,
    pub vce_gated: bool,
    pub vcn_gated: [core::sync::atomic::AtomicI32; AMDGPU_MAX_VCN_INSTANCES],
    pub jpeg_gated: core::sync::atomic::AtomicI32,
    pub vpe_gated: core::sync::atomic::AtomicI32,
    pub isp_gated: core::sync::atomic::AtomicI32,
    pub umsch_mm_gated: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_power_context {
    pub power_context: *mut c_void,
    pub power_context_size: u32,
    pub power_gate: smu_power_gate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_feature_bits {
    pub SMU_FEATURE_MAX): DECLARE_BITMAP(bits,,
}

//
// Helpers for initializing smu_feature_bits statically.
// Use SMU_FEATURE_BIT_INIT() which automatically handles array indexing:
// static const struct smu_feature_bits example = {
// .bits = {
// SMU_FEATURE_BIT_INIT(5),
// SMU_FEATURE_BIT_INIT(10),
// SMU_FEATURE_BIT_INIT(65),
// SMU_FEATURE_BIT_INIT(100)
// }
// };
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_feature_list {
    SMU_FEATURE_LIST_SUPPORTED,
    SMU_FEATURE_LIST_ALLOWED,
    SMU_FEATURE_LIST_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_feature {
    pub feature_num: u32,
    pub bits: [smu_feature_bits; SMU_FEATURE_LIST_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_clocks {
    pub engine_clock: u32,
    pub memory_clock: u32,
    pub bus_bandwidth: u32,
    pub engine_clock_in_sr: u32,
    pub dcef_clock: u32,
    pub dcef_clock_in_sr: u32,
}

pub const MAX_REGULAR_DPM_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mclk_latency_entries {
    pub frequency: u32,
    pub latency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mclock_latency_table {
    pub count: u32,
    pub entries: [mclk_latency_entries; MAX_REGULAR_DPM_NUM],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_reset_mode {
    SMU_RESET_MODE_0,
    SMU_RESET_MODE_1,
    SMU_RESET_MODE_2,
    SMU_RESET_MODE_3,
    SMU_RESET_MODE_4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_baco_state {
    SMU_BACO_STATE_ENTER = 0,
    SMU_BACO_STATE_EXIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_baco_context {
    pub state: u32,
    pub platform_support: bool,
    pub maco_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_freq_info {
    pub min: u32,
    pub max: u32,
    pub freq_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstates_clk_freq {
    pub min: u32,
    pub standard: u32,
    pub peak: u32,
    pub custom: smu_freq_info,
    pub curr: smu_freq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_umd_pstate_table {
    pub gfxclk_pstate: pstates_clk_freq,
    pub socclk_pstate: pstates_clk_freq,
    pub uclk_pstate: pstates_clk_freq,
    pub vclk_pstate: pstates_clk_freq,
    pub dclk_pstate: pstates_clk_freq,
    pub fclk_pstate: pstates_clk_freq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmn2asic_msg_mapping {
    pub valid_mapping: c_int,
    pub map_to: c_int,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmn2asic_mapping {
    pub valid_mapping: c_int,
    pub map_to: c_int,
}

pub const SMU_MSG_MAX_ARGS: c_int = 4;
// Message flags for smu_msg_args

// smu_msg_ctl flags

//
// struct smu_msg_config - IP-level register configuration
// @msg_reg: Message register offset
// @resp_reg: Response register offset
// @arg_regs: Argument register offsets (up to SMU_MSG_MAX_ARGS)
// @num_arg_regs: Number of argument registers available
// @debug_msg_reg: Debug message register offset
// @debug_resp_reg: Debug response register offset
// @debug_param_reg: Debug parameter register offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_msg_config {
    pub msg_reg: u32,
    pub resp_reg: u32,
    pub arg_regs: [u32; SMU_MSG_MAX_ARGS],
    pub num_arg_regs: c_int,
    pub debug_msg_reg: u32,
    pub debug_resp_reg: u32,
    pub debug_param_reg: u32,
}

//
// struct smu_msg_args - Per-call message arguments
// @msg: Common message type (enum smu_message_type)
// @args: Input arguments
// @num_args: Number of input arguments
// @out_args: Output arguments (filled after successful send)
// @num_out_args: Number of output arguments to read
// @flags: Message flags (SMU_MSG_FLAG_*)
// @timeout: Per-message timeout in us (0 = use default)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_msg_args {
    pub msg: smu_message_type,
    pub args: [u32; SMU_MSG_MAX_ARGS],
    pub num_args: c_int,
    pub out_args: [u32; SMU_MSG_MAX_ARGS],
    pub num_out_args: c_int,
    pub flags: u32,
    pub timeout: u32,
}

//
// struct smu_msg_ops - IP-level protocol operations
// @send_msg: send message protocol
// @wait_response: wait for response (for split send/wait cases)
// @decode_response: Convert response register value to errno
// @send_debug_msg: send debug message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_msg_ops {
    pub args): *mut *mut *mut int (send_msg)(struct smu_msg_ctl ctl, struct smu_msg_args,
    pub timeout_us): *mut *mut *mut int (wait_response)(struct smu_msg_ctl ctl, u32,
    pub resp): *mut *mut int (decode_response)(u32,
    pub param): *mut *mut *mut int (send_debug_msg)(struct smu_msg_ctl ctl, u32 msg, u32,
}

//
// struct smu_msg_ctl - Per-device message control block
// This is a standalone control block that encapsulates everything
// needed for SMU messaging. The ops->send_msg implements the complete
// protocol including all filtering and error handling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_msg_ctl {
    pub smu: *mut smu_context,
    pub lock: mutex,
    pub config: smu_msg_config,
    pub ops: *const smu_msg_ops,
    pub message_map: *const cmn2asic_msg_mapping,
    pub default_timeout: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb_context {
    pub stb_buf_size: u32,
    pub enabled: bool,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_fw_status {
    SMU_FW_INIT = 0,
    SMU_FW_RUNTIME,
    SMU_FW_HANG,
}

pub const WORKLOAD_POLICY_MAX: c_int = 7;
//
// Configure wbrf event handling pace as there can be only one
// event processed every SMU_WBRF_EVENT_HANDLING_PACE ms.
//
pub const SMU_WBRF_EVENT_HANDLING_PACE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_feature_cap_id {
    SMU_FEATURE_CAP_ID__LINK_RESET = 0,
    SMU_FEATURE_CAP_ID__SDMA_RESET,
    SMU_FEATURE_CAP_ID__VCN_RESET,
    SMU_FEATURE_CAP_ID__COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_feature_cap {
    pub SMU_FEATURE_CAP_ID__COUNT): DECLARE_BITMAP(cap_map,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_context {
    pub adev: *mut amdgpu_device,
    pub irq_source: amdgpu_irq_src,
    pub ppt_funcs: *const pptable_funcs,
    pub clock_map: *const cmn2asic_mapping,
    pub feature_map: *const cmn2asic_mapping,
    pub table_map: *const cmn2asic_mapping,
    pub pwr_src_map: *const cmn2asic_mapping,
    pub workload_map: *const cmn2asic_mapping,
    pub pool_size: u64,
    pub smu_table: smu_table_context,
    pub smu_dpm: smu_dpm_context,
    pub smu_power: smu_power_context,
    pub smu_temp: smu_temp_context,
    pub smu_feature: smu_feature,
    pub display_config: *mut amd_pp_display_configuration,
    pub smu_baco: smu_baco_context,
    pub thermal_range: smu_temperature_range,
    pub fea_cap: smu_feature_cap,
    pub od_settings: *mut c_void,
    pub pstate_table: smu_umd_pstate_table,
    pub pstate_sclk: u32,
    pub pstate_mclk: u32,
    pub od_enabled: bool,
    pub ppt_limits: smu_ppt_limit_context,
// soft pptable
    pub ppt_offset_bytes: u32,
    pub ppt_size_bytes: u32,
    pub ppt_start_addr: *mut u8,
    pub support_power_containment: bool,
    pub disable_watermark: bool,

    pub watermarks_bitmap: u32,
    pub hard_min_uclk_req_from_dal: u32,
    pub disable_uclk_switch: bool,
// asic agnostic workload mask
    pub workload_mask: u32,
    pub pause_workload: bool,
// default/user workload preference
    pub power_profile_mode: u32,
    pub workload_refcount: [u32; PP_SMC_POWER_PROFILE_COUNT],
// backend specific custom workload settings
    pub custom_profile_params: *mut c_long,
    pub pm_enabled: bool,
    pub is_apu: bool,
// Power dependency link from an integrated xHCI controller to the GPU
    pub usb_power_link: *mut device_link,
    pub smc_driver_if_version: u32,
    pub smc_fw_if_version: u32,
    pub smc_fw_version: u32,
    pub smc_fw_caps: u32,
    pub smc_fw_state: u8,
    pub uploading_custom_pp_table: bool,
    pub dc_controlled_by_gpio: bool,
    pub throttling_logging_work: work_struct,
    pub throttle_int_counter: core::sync::atomic::AtomicI64,
    pub interrupt_work: work_struct,
    pub fan_max_rpm: unsigned,
    pub manual_fan_speed_pwm: unsigned,
    pub gfx_default_hard_min_freq: u32,
    pub gfx_default_soft_max_freq: u32,
    pub gfx_actual_hard_min_freq: u32,
    pub gfx_actual_soft_max_freq: u32,
// APU only
    pub cpu_default_soft_min_freq: u32,
    pub cpu_default_soft_max_freq: u32,
    pub cpu_actual_soft_min_freq: u32,
    pub cpu_actual_soft_max_freq: u32,
    pub cpu_core_id_select: u32,
    pub cpu_core_num: u16,
    pub user_dpm_profile: smu_user_dpm_profile,
    pub stb_context: stb_context,
    pub pptable_firmware: firmware,
    pub swctf_delayed_work: delayed_work,
// data structures for wbrf feature support
    pub wbrf_supported: bool,
    pub wbrf_notifier: notifier_block,
    pub wbrf_delayed_work: delayed_work,
// SMU message control block
    pub msg_ctl: smu_msg_ctl,
}

//
// struct smu_temp_funcs - Callbacks used to get temperature data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_temp_funcs {
//
// @get_temp_metrics: Calibrate voltage/frequency curve to fit the system's
// power delivery and voltage margins. Required for adaptive
// @type Temperature metrics type(baseboard/gpuboard)
// Return: Size of &table
//
    pub table): *mut smu_temp_metric_type type, void,
//
// @temp_metrics_is_support: Get if specific temperature metrics is supported
// @type Temperature metrics type(baseboard/gpuboard)
// Return: true if supported else false
//
    pub type): *mut *mut *mut bool (temp_metrics_is_supported)(struct smu_context smu, enum smu_temp_metric_type,
}

//
// struct pptable_funcs - Callbacks used to interact with the SMU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pptable_funcs {
//
// @run_btc: Calibrate voltage/frequency curve to fit the system's
// power delivery and voltage margins. Required for adaptive
// voltage frequency scaling (AVFS).
//
    pub smu): *mut *mut int (run_btc)(struct smu_context,
//
// @init_allowed_features: Initialize allowed features bitmap.
// Directly sets allowed features using smu_feature wrapper functions.
//
    pub smu): *mut *mut int (init_allowed_features)(struct smu_context,
//
// @get_current_power_state: Get the current power state.
//
// Return: Current power state on success, negative errno on failure.
//
    pub smu): *mut *mut amd_pm_state_type (get_current_power_state)(struct smu_context,
//
// @set_default_dpm_table: Retrieve the default overdrive settings from
// the SMU.
//
    pub smu): *mut *mut int (set_default_dpm_table)(struct smu_context,
//
// @populate_umd_state_clk: Populate the UMD power state table with
// defaults.
//
    pub smu): *mut *mut int (populate_umd_state_clk)(struct smu_context,
//
// @emit_clk_levels: Print DPM clock levels for a clock domain
// to buffer using sysfs_emit_at. Star current level.
//
// Used for sysfs interfaces.
// &buf: sysfs buffer
// &offset: offset within buffer to start printing, which is updated by the
// function.
//
// Return: 0 on Success or Negative to indicate an error occurred.
//
    pub offset): *mut *mut *mut *mut int (emit_clk_levels)(struct smu_context smu, enum smu_clk_type clk_type, char buf, int,
//
// @force_clk_levels: Set a range of allowed DPM levels for a clock
// domain.
// &clk_type: Clock domain.
// &mask: Range of allowed DPM levels.
//
    pub mask): *mut *mut *mut int (force_clk_levels)(struct smu_context smu, enum smu_clk_type clk_type, uint32_t,
//
// @od_edit_dpm_table: Edit the custom overdrive DPM table.
// &type: Type of edit.
// &input: Edit parameters.
// &size: Size of &input.
//
    pub size): *mut *mut long input, uint32_t,
//
// @restore_user_od_settings: Restore the user customized
// OD settings on S3/S4/Runpm resume.
//
    pub smu): *mut *mut int (restore_user_od_settings)(struct smu_context,
//
// @get_clock_by_type_with_latency: Get the speed and latency of a clock
// domain.
//
// clocks);
//
// @get_power_profile_mode: Print all power profile modes to
// buffer. Star current mode.
//
    pub buf): *mut *mut *mut int (get_power_profile_mode)(struct smu_context smu, char,
//
// @set_power_profile_mode: Set a power profile mode. Also used to
// create/set custom power profile modes.
// &input: Power profile mode parameters.
// &workload_mask: mask of workloads to enable
// &custom_params: custom profile parameters
// &custom_params_max_idx: max valid idx into custom_params
//
    pub custom_params_max_idx): *mut *mut long custom_params, u32,
//
// @dpm_set_vcn_enable: Enable/disable VCN engine dynamic power
// management.
//
    pub inst): *mut *mut *mut int (dpm_set_vcn_enable)(struct smu_context smu, bool enable, int,
//
// @dpm_set_jpeg_enable: Enable/disable JPEG engine dynamic power
// management.
//
    pub enable): *mut *mut *mut int (dpm_set_jpeg_enable)(struct smu_context smu, bool,
//
// @set_gfx_power_up_by_imu: Enable GFX engine with IMU
//
    pub smu): *mut *mut int (set_gfx_power_up_by_imu)(struct smu_context,
//
// @read_sensor: Read data from a sensor.
// &sensor: Sensor to read data from.
// &data: Sensor reading.
// &size: Size of &data.
//
    pub size): *mut *mut void data, uint32_t,
//
// @get_apu_thermal_limit: get apu core limit from smu
// &limit: current limit temperature in millidegrees Celsius
//
    pub limit): *mut *mut *mut int (get_apu_thermal_limit)(struct smu_context smu, uint32_t,
//
// @set_apu_thermal_limit: update all controllers with new limit
// &limit: limit temperature to be setted, in millidegrees Celsius
//
    pub limit): *mut *mut *mut int (set_apu_thermal_limit)(struct smu_context smu, uint32_t,
//
// @pre_display_config_changed: Prepare GPU for a display configuration
// change.
//
// Disable display tracking and pin memory clock speed to maximum. Used
// in display component synchronization.
//
    pub smu): *mut *mut int (pre_display_config_changed)(struct smu_context,
//
// @display_config_changed: Notify the SMU of the current display
// configuration.
//
// Allows SMU to properly track blanking periods for memory clock
// adjustment. Used in display component synchronization.
//
    pub smu): *mut *mut int (display_config_changed)(struct smu_context,
    pub smu): *mut *mut int (apply_clocks_adjust_rules)(struct smu_context,
//
// @notify_smc_display_config: Applies display requirements to the
// current power state.
//
// Optimize deep sleep DCEFclk and mclk for the current display
// configuration. Used in display component synchronization.
//
    pub smu): *mut *mut int (notify_smc_display_config)(struct smu_context,
//
// @is_dpm_running: Check if DPM is running.
//
// Return: True if DPM is running, false otherwise.
//
    pub smu): *mut *mut bool (is_dpm_running)(struct smu_context,
//
// @get_fan_speed_pwm: Get the current fan speed in PWM.
//
    pub speed): *mut *mut *mut int (get_fan_speed_pwm)(struct smu_context smu, uint32_t,
//
// @get_fan_speed_rpm: Get the current fan speed in rpm.
//
    pub speed): *mut *mut *mut int (get_fan_speed_rpm)(struct smu_context smu, uint32_t,
//
// @set_watermarks_table: Configure and upload the watermarks tables to
// the SMU.
//
    pub clock_ranges): *mut pp_smu_wm_range_sets,
//
// @get_thermal_temperature_range: Get safe thermal limits in Celcius.
//
    pub range): *mut *mut *mut int (get_thermal_temperature_range)(struct smu_context smu, struct smu_temperature_range,
//
// @get_uclk_dpm_states: Get memory clock DPM levels in kHz.
// &clocks_in_khz: Array of DPM levels.
// &num_states: Elements in &clocks_in_khz.
//
    pub num_states): *mut *mut *mut *mut int (get_uclk_dpm_states)(struct smu_context smu, uint32_t clocks_in_khz, uint32_t,
//
// @set_default_od_settings: Set the overdrive tables to defaults.
//
    pub smu): *mut *mut int (set_default_od_settings)(struct smu_context,
//
// @set_performance_level: Set a performance level.
//
    pub level): *mut *mut *mut int (set_performance_level)(struct smu_context smu, enum amd_dpm_forced_level,
//
// @display_disable_memory_clock_switch: Enable/disable dynamic memory
// clock switching.
//
// Disabling this feature forces memory clock speed to maximum.
// Enabling sets the minimum memory clock capable of driving the
// current display configuration.
//
    pub disable_memory_clock_switch): *mut *mut *mut int (display_disable_memory_clock_switch)(struct smu_context smu, bool,
//
// @get_ppt_limit: Get the effective runtime PPT0 or PPT1 limit.
//
    pub current_ppt_limit): *mut u32,
//
// @set_df_cstate: Set data fabric cstate.
//
    pub state): *mut *mut *mut int (set_df_cstate)(struct smu_context smu, enum pp_df_cstate,
//
// @update_pcie_parameters: Update and upload the system's PCIe
// capabilites to the SMU.
// &pcie_gen_cap: Maximum allowed PCIe generation.
// &pcie_width_cap: Maximum allowed PCIe width.
//
    pub pcie_width_cap): *mut *mut *mut int (update_pcie_parameters)(struct smu_context smu, uint8_t pcie_gen_cap, uint8_t,
//
// @i2c_init: Initialize i2c.
//
// The i2c bus is used internally by the SMU voltage regulators and
// other devices. The i2c's EEPROM also stores bad page tables on boards
// with ECC.
//
    pub smu): *mut *mut int (i2c_init)(struct smu_context,
//
// @i2c_fini: Tear down i2c.
//
    pub smu): *mut *mut void (i2c_fini)(struct smu_context,
//
// @get_unique_id: Get the GPU's unique id. Used for asset tracking.
//
    pub smu): *mut *mut void (get_unique_id)(struct smu_context,
//
// @get_dpm_clock_table: Get a copy of the DPM clock table.
//
// Used by display component in bandwidth and watermark calculations.
//
    pub clock_table): *mut *mut *mut int (get_dpm_clock_table)(struct smu_context smu, struct dpm_clocks,
//
// @init_microcode: Request the SMU's firmware from the kernel.
//
    pub smu): *mut *mut int (init_microcode)(struct smu_context,
//
// @load_microcode: Load firmware onto the SMU.
//
    pub smu): *mut *mut int (load_microcode)(struct smu_context,
//
// @fini_microcode: Release the SMU's firmware.
//
    pub smu): *mut *mut void (fini_microcode)(struct smu_context,
//
// @init_smc_tables: Initialize the SMU tables.
//
    pub smu): *mut *mut int (init_smc_tables)(struct smu_context,
//
// @fini_smc_tables: Release the SMU tables.
//
    pub smu): *mut *mut int (fini_smc_tables)(struct smu_context,
//
// @init_power: Initialize the power gate table context.
//
    pub smu): *mut *mut int (init_power)(struct smu_context,
//
// @fini_power: Release the power gate table context.
//
    pub smu): *mut *mut int (fini_power)(struct smu_context,
//
// @check_fw_status: Check the SMU's firmware status.
//
// Return: Zero if check passes, negative errno on failure.
//
    pub smu): *mut *mut int (check_fw_status)(struct smu_context,
//
// @set_mp1_state: put SMU into a correct state for comming
// resume from runpm or gpu reset.
//
    pub mp1_state): pp_mp1_state,
//
// @setup_pptable: Initialize the power play table and populate it with
// default values.
//
    pub smu): *mut *mut int (setup_pptable)(struct smu_context,
//
// @get_vbios_bootup_values: Get default boot values from the VBIOS.
//
    pub smu): *mut *mut int (get_vbios_bootup_values)(struct smu_context,
//
// @check_fw_version: Print driver and SMU interface versions to the
// system log.
//
// Interface mismatch is not a critical failure.
//
    pub smu): *mut *mut int (check_fw_version)(struct smu_context,
//
// @powergate_sdma: Power up/down system direct memory access.
//
    pub gate): *mut *mut *mut int (powergate_sdma)(struct smu_context smu, bool,
//
// @set_gfx_cgpg: Enable/disable graphics engine course grain power
// gating.
//
    pub enable): *mut *mut *mut int (set_gfx_cgpg)(struct smu_context smu, bool,
//
// @write_pptable: Write the power play table to the SMU.
//
    pub smu): *mut *mut int (write_pptable)(struct smu_context,
//
// @set_driver_table_location: Send the location of the driver table to
// the SMU.
//
    pub smu): *mut *mut int (set_driver_table_location)(struct smu_context,
//
// @set_tool_table_location: Send the location of the tool table to the
// SMU.
//
    pub smu): *mut *mut int (set_tool_table_location)(struct smu_context,
//
// @notify_memory_pool_location: Send the location of the memory pool to
// the SMU.
//
    pub smu): *mut *mut int (notify_memory_pool_location)(struct smu_context,
//
// @system_features_control: Enable/disable all SMU features.
//
    pub en): *mut *mut *mut int (system_features_control)(struct smu_context smu, bool,
//
// @init_display_count: Notify the SMU of the number of display
// components in current display configuration.
//
    pub count): *mut *mut *mut int (init_display_count)(struct smu_context smu, uint32_t,
//
// @set_allowed_mask: Notify the SMU of the features currently allowed
// by the driver.
//
    pub smu): *mut *mut int (set_allowed_mask)(struct smu_context,
//
// @get_enabled_mask: Get a mask of features that are currently enabled
// on the SMU.
// &feature_mask: Enabled feature mask.
//
    pub feature_mask): *mut smu_feature_bits,
//
// @feature_is_enabled: Test if a feature is enabled.
//
// Return: One if enabled, zero if disabled.
//
    pub mask): *mut *mut *mut int (feature_is_enabled)(struct smu_context smu, enum smu_feature_mask,
//
// @disable_all_features_with_exception: Disable all features with
// exception to those in &mask.
//
    pub mask): smu_feature_mask,
//
// @notify_display_change: General interface call to let SMU know about DC change
//
    pub smu): *mut *mut int (notify_display_change)(struct smu_context,
//
// @set_ppt_limit: Set the PPT0 or PPT1 limit in watts.
//
    pub limit): u32,
//
// @init_max_sustainable_clocks: Populate max sustainable clock speed
// table with values from the SMU.
//
    pub smu): *mut *mut int (init_max_sustainable_clocks)(struct smu_context,
//
// @enable_thermal_alert: Enable thermal alert interrupts.
//
    pub smu): *mut *mut int (enable_thermal_alert)(struct smu_context,
//
// @disable_thermal_alert: Disable thermal alert interrupts.
//
    pub smu): *mut *mut int (disable_thermal_alert)(struct smu_context,
//
// @set_min_dcef_deep_sleep: Set a minimum display fabric deep sleep
// clock speed in MHz.
//
    pub clk): *mut *mut *mut int (set_min_dcef_deep_sleep)(struct smu_context smu, uint32_t,
//
// @display_clock_voltage_request: Set a hard minimum frequency
// for a clock domain.
//
// clock_req);
//
// @get_fan_control_mode: Get the current fan control mode.
//
    pub smu): *mut *mut uint32_t (get_fan_control_mode)(struct smu_context,
//
// @set_fan_control_mode: Set the fan control mode.
//
    pub mode): *mut *mut *mut int (set_fan_control_mode)(struct smu_context smu, uint32_t,
//
// @set_fan_speed_pwm: Set a static fan speed in PWM.
//
    pub speed): *mut *mut *mut int (set_fan_speed_pwm)(struct smu_context smu, uint32_t,
//
// @set_fan_speed_rpm: Set a static fan speed in rpm.
//
    pub speed): *mut *mut *mut int (set_fan_speed_rpm)(struct smu_context smu, uint32_t,
//
// @set_xgmi_pstate: Set inter-chip global memory interconnect pstate.
// &pstate: Pstate to set. D0 if Nonzero, D3 otherwise.
//
    pub pstate): *mut *mut *mut int (set_xgmi_pstate)(struct smu_context smu, uint32_t,
//
// @gfx_off_control: Enable/disable graphics engine poweroff.
//
    pub enable): *mut *mut *mut int (gfx_off_control)(struct smu_context smu, bool,
//
// @get_gfx_off_status: Get graphics engine poweroff status.
//
// Return:
// 0 - GFXOFF(default).
// 1 - Transition out of GFX State.
// 2 - Not in GFXOFF.
// 3 - Transition into GFXOFF.
//
    pub smu): *mut *mut uint32_t (get_gfx_off_status)(struct smu_context,
//
// @gfx_off_entrycount: total GFXOFF entry count at the time of
// query since system power-up
//
    pub entrycount): *mut *mut *mut u32 (get_gfx_off_entrycount)(struct smu_context smu, uint64_t,
//
// @set_gfx_off_residency: set 1 to start logging, 0 to stop logging
//
    pub start): *mut *mut *mut u32 (set_gfx_off_residency)(struct smu_context smu, bool,
//
// @get_gfx_off_residency: Live GFXOFF residency percentage
//
    pub residency): *mut *mut *mut u32 (get_gfx_off_residency)(struct smu_context smu, uint32_t,
//
// @register_irq_handler: Register interupt request handlers.
//
    pub smu): *mut *mut int (register_irq_handler)(struct smu_context,
//
// @get_max_sustainable_clocks_by_dc: Get a copy of the max sustainable
// clock speeds table.
//
// Provides a way for the display component (DC) to get the max
// sustainable clocks from the SMU.
//
    pub max_clocks): *mut *mut *mut int (get_max_sustainable_clocks_by_dc)(struct smu_context smu, struct pp_smu_nv_clock_table,
//
// @get_bamaco_support: Check if GPU supports BACO/MACO
// BACO: Bus Active, Chip Off
// MACO: Memory Active, Chip Off
//
    pub smu): *mut *mut int (get_bamaco_support)(struct smu_context,
//
// @baco_enter: Enter BACO.
//
    pub smu): *mut *mut int (baco_enter)(struct smu_context,
//
// @baco_exit: Exit Baco.
//
    pub smu): *mut *mut int (baco_exit)(struct smu_context,
//
// @mode1_reset_is_support: Check if GPU supports mode1 reset.
//
    pub smu): *mut *mut bool (mode1_reset_is_support)(struct smu_context,
//
// @mode1_reset: Perform mode1 reset.
//
// Complete GPU reset.
//
    pub smu): *mut *mut int (mode1_reset)(struct smu_context,
//
// @mode2_reset: Perform mode2 reset.
//
// Mode2 reset generally does not reset as many IPs as mode1 reset. The
// IPs reset varies by asic.
//
    pub smu): *mut *mut int (mode2_reset)(struct smu_context,
// for gfx feature enablement after mode2 reset
    pub smu): *mut *mut int (enable_gfx_features)(struct smu_context,
//
// @link_reset: Perform link reset.
//
// The gfx device driver reset
//
    pub smu): *mut *mut int (link_reset)(struct smu_context,
//
// @get_dpm_ultimate_freq: Get the hard frequency range of a clock
// domain in MHz.
//
    pub max): *mut *mut *mut *mut int (get_dpm_ultimate_freq)(struct smu_context smu, enum smu_clk_type clk_type, uint32_t min, uint32_t,
//
// @set_soft_freq_limited_range: Set the soft frequency range of a clock
// domain in MHz.
//
    pub automatic): bool,
//
// @set_power_source: Notify the SMU of the current power source.
//
    pub power_src): *mut *mut *mut int (set_power_source)(struct smu_context smu, enum smu_power_src_type,
//
// @log_thermal_throttling_event: Print a thermal throttling warning to
// the system's log.
//
    pub smu): *mut *mut void (log_thermal_throttling_event)(struct smu_context,
//
// @get_pp_feature_mask: Print a human readable table of enabled
// features to buffer.
//
    pub buf): *mut *mut *mut size_t (get_pp_feature_mask)(struct smu_context smu, char,
//
// @set_pp_feature_mask: Request the SMU enable/disable features to
// match those enabled in &new_mask.
//
    pub new_mask): *mut *mut *mut int (set_pp_feature_mask)(struct smu_context smu, uint64_t,
//
// @get_gpu_metrics: Get a copy of the GPU metrics table from the SMU.
//
// Return: Size of &table
//
    pub table): *mut *mut *mut ssize_t (get_gpu_metrics)(struct smu_context smu, void,
//
// @get_pm_metrics: Get one snapshot of power management metrics from
// PMFW.
//
// Return: Size of the metrics sample
//
    pub size): usize,
//
// @enable_mgpu_fan_boost: Enable multi-GPU fan boost.
//
    pub smu): *mut *mut int (enable_mgpu_fan_boost)(struct smu_context,
//
// @gfx_ulv_control: Enable/disable ultra low voltage.
//
    pub enablement): *mut *mut *mut int (gfx_ulv_control)(struct smu_context smu, bool,
//
// @deep_sleep_control: Enable/disable deep sleep.
//
    pub enablement): *mut *mut *mut int (deep_sleep_control)(struct smu_context smu, bool,
//
// @get_fan_parameters: Get fan parameters.
//
// Get maximum fan speed from the power play table.
//
    pub smu): *mut *mut int (get_fan_parameters)(struct smu_context,
//
// @post_init: Helper function for asic specific workarounds.
//
    pub smu): *mut *mut int (post_init)(struct smu_context,
//
// @interrupt_work: Work task scheduled from SMU interrupt handler.
//
    pub smu): *mut *mut void (interrupt_work)(struct smu_context,
//
// @gpo_control: Enable/disable graphics power optimization if supported.
//
    pub enablement): *mut *mut *mut int (gpo_control)(struct smu_context smu, bool,
//
// @gfx_state_change_set: Send the current graphics state to the SMU.
//
    pub state): *mut *mut *mut int (gfx_state_change_set)(struct smu_context smu, uint32_t,
//
// @set_fine_grain_gfx_freq_parameters: Set fine grain graphics clock
// parameters to defaults.
//
    pub smu): *mut *mut int (set_fine_grain_gfx_freq_parameters)(struct smu_context,
//
// @smu_handle_passthrough_sbr:  Send message to SMU about special handling for SBR.
//
    pub enable): *mut *mut *mut int (smu_handle_passthrough_sbr)(struct smu_context smu, bool,
//
// @wait_for_event:  Wait for events from SMU.
//
    pub event_arg): smu_event_type event, uint64_t,
//
// @sned_hbm_bad_pages_num:  message SMU to update bad page number
// of SMUBUS table.
//
    pub size): *mut *mut *mut int (send_hbm_bad_pages_num)(struct smu_context smu, uint32_t,
//
// @send_rma_reason: message rma reason event to SMU.
//
    pub smu): *mut *mut int (send_rma_reason)(struct smu_context,
//
// @reset_sdma: message SMU to soft reset sdma instance.
//
    pub inst_mask): *mut *mut *mut int (reset_sdma)(struct smu_context smu, uint32_t,
//
// @reset_vcn: message SMU to soft reset vcn instance.
//
    pub inst_mask): *mut *mut *mut int (dpm_reset_vcn)(struct smu_context smu, uint32_t,
//
// @get_ecc_table:  message SMU to get ECC INFO table.
//
    pub table): *mut *mut *mut ssize_t (get_ecc_info)(struct smu_context smu, void,
//
// @stb_collect_info: Collects Smart Trace Buffers data.
//
    pub size): *mut *mut *mut *mut int (stb_collect_info)(struct smu_context smu, void buf, uint32_t,
//
// @get_default_config_table_settings: Get the ASIC default DriverSmuConfig table settings.
//
    pub table): *mut *mut *mut int (get_default_config_table_settings)(struct smu_context smu, struct config_table_setting,
//
// @set_config_table: Apply the input DriverSmuConfig table settings.
//
    pub table): *mut *mut *mut int (set_config_table)(struct smu_context smu, struct config_table_setting,
//
// @sned_hbm_bad_channel_flag:  message SMU to update bad channel info
// of SMUBUS table.
//
    pub size): *mut *mut *mut int (send_hbm_bad_channel_flag)(struct smu_context smu, uint32_t,
//
// @init_pptable_microcode: Prepare the pptable microcode to upload via PSP
//
    pub smu): *mut *mut int (init_pptable_microcode)(struct smu_context,
//
// @dpm_set_vpe_enable: Enable/disable VPE engine dynamic power
// management.
//
    pub enable): *mut *mut *mut int (dpm_set_vpe_enable)(struct smu_context smu, bool,
//
// @dpm_set_isp_enable: Enable/disable ISP engine dynamic power
// management.
//
    pub enable): *mut *mut *mut int (dpm_set_isp_enable)(struct smu_context smu, bool,
//
// @dpm_set_umsch_mm_enable: Enable/disable UMSCH engine dynamic power
// management.
//
    pub enable): *mut *mut *mut int (dpm_set_umsch_mm_enable)(struct smu_context smu, bool,
//
// @set_mall_enable: Init MALL power gating control.
//
    pub smu): *mut *mut int (set_mall_enable)(struct smu_context,
//
// @notify_rlc_state: Notify RLC power state to SMU.
//
    pub en): *mut *mut *mut int (notify_rlc_state)(struct smu_context smu, bool,
//
// @is_asic_wbrf_supported: check whether PMFW supports the wbrf feature
//
    pub smu): *mut *mut bool (is_asic_wbrf_supported)(struct smu_context,
//
// @enable_uclk_shadow: Enable the uclk shadow feature on wbrf supported
//
    pub enable): *mut *mut *mut int (enable_uclk_shadow)(struct smu_context smu, bool,
//
// @set_wbrf_exclusion_ranges: notify SMU the wifi bands occupied
//
    pub exclusion_ranges): *mut freq_band_range,
//
// @get_xcp_metrics: Get a copy of the partition metrics table from SMU.
// Return: Size of table
//
    pub table): *mut c_void,
//
// @ras_send_msg: Send a message with a parameter from Ras
// &msg: Type of message.
// &param: Message parameter.
// &read_arg: SMU response (optional).
//
    pub read_arg): *mut smu_message_type msg, uint32_t param, uint32_t,
//
// @set_power_dep: Create or destroy a power dependency link
// from an integrated xHCI controller to the GPU so that the GPU is
// resumed before the USB controller during PM resume. @enable is true
// to create the link and false to tear it down.
//
    pub enable): *mut *mut *mut int (set_power_dep)(struct smu_context smu, bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_cmn2asic_mapping_type {
    CMN2ASIC_MAPPING_MSG,
    CMN2ASIC_MAPPING_CLK,
    CMN2ASIC_MAPPING_FEATURE,
    CMN2ASIC_MAPPING_TABLE,
    CMN2ASIC_MAPPING_PWR,
    CMN2ASIC_MAPPING_WORKLOAD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_baco_seq {
    BACO_SEQ_BACO = 0,
    BACO_SEQ_MSR,
    BACO_SEQ_BAMACO,
    BACO_SEQ_ULPS,
    BACO_SEQ_COUNT,
}

//
// smu_memcpy_trailing - Copy the end of one structure into the middle of another
//
// @dst: Pointer to destination struct
// @first_dst_member: The member name in @dst where the overwrite begins
// @last_dst_member: The member name in @dst where the overwrite ends after
// @src: Pointer to the source struct
// @first_src_member: The member name in @src where the copy begins
//

extern "C" {
    pub fn smu_mode1_reset_is_support(smu: *mut smu_context) -> bool;
}
extern "C" {
    pub fn smu_link_reset_is_support(smu: *mut smu_context) -> bool;
}
extern "C" {
    pub fn smu_mode1_reset(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_link_reset(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn amdgpu_smu_early_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn is_support_cclk_dpm(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn smu_write_watermarks_table(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_set_gfx_power_up_by_imu(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_set_ac_dc(smu: *mut smu_context, restore_ppt_policy: bool) -> c_int;
}
extern "C" {
    pub fn smu_get_entrycount_gfxoff(smu: *mut smu_context, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn smu_get_residency_gfxoff(smu: *mut smu_context, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn smu_set_residency_gfxoff(smu: *mut smu_context, value: bool) -> c_int;
}
extern "C" {
    pub fn smu_get_status_gfxoff(smu: *mut smu_context, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn smu_handle_passthrough_sbr(smu: *mut smu_context, enable: bool) -> c_int;
}
extern "C" {
    pub fn smu_get_ecc_info(smu: *mut smu_context, umc_ecc: *mut c_void) -> c_int;
}
extern "C" {
    pub fn smu_stb_collect_info(smu: *mut smu_context, buff: *mut c_void, size: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_smu_stb_debug_fs_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn smu_send_hbm_bad_pages_num(smu: *mut smu_context, size: u32) -> c_int;
}
extern "C" {
    pub fn smu_send_hbm_bad_channel_flag(smu: *mut smu_context, size: u32) -> c_int;
}
extern "C" {
    pub fn smu_send_rma_reason(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_reset_sdma(smu: *mut smu_context, inst_mask: u32) -> c_int;
}
extern "C" {
    pub fn smu_reset_sdma_is_supported(smu: *mut smu_context) -> bool;
}
extern "C" {
    pub fn smu_reset_vcn(smu: *mut smu_context, inst_mask: u32) -> c_int;
}
extern "C" {
    pub fn smu_reset_vcn_is_supported(smu: *mut smu_context) -> bool;
}

extern "C" {
    pub fn smu_feature_cap_set(smu: *mut smu_context, fea_id: smu_feature_cap_id);
}
extern "C" {
    pub fn smu_feature_cap_test(smu: *mut smu_context, fea_id: smu_feature_cap_id) -> bool;
}
extern "C" {
    pub fn test_bit(_arg: bit, _arg: bits->bits) -> return;
}
extern "C" {
    pub fn bitmap_intersects(_arg: bits->bits, _arg: mask, _arg: SMU_FEATURE_MAX) -> return;
}
extern "C" {
    pub fn bitmap_empty(_arg: bits->bits, _arg: nbits) -> return;
}
extern "C" {
    pub fn bitmap_full(_arg: bits->bits, _arg: nbits) -> return;
}
extern "C" {
    pub fn smu_feature_bits_is_set(_arg: __smu_feature_get_list(smu, _arg: list), _arg: bit) -> return;
}
//
// smu_safe_u16_nn - Make u16 safe by filtering negative overflow errors
// @val: Input u16 value, may contain invalid negative overflows
//
// Convert u16 to non-negative value. Cast to s16 to detect negative values
// caused by calculation errors. Return 0 for negative errors, return
// original value if valid.
//
// Return: Valid u16 value or 0
//
