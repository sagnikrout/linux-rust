//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/kgd_pp_interface.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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
pub enum smu_temp_metric_type {
    SMU_TEMP_METRIC_BASEBOARD,
    SMU_TEMP_METRIC_GPUBOARD,
    SMU_TEMP_METRIC_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_event_type {
    SMU_EVENT_RESET_COMPLETE = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_vce_state {
// vce clocks
    pub evclk: u32,
    pub ecclk: u32,
// gpu clocks
    pub sclk: u32,
    pub mclk: u32,
    pub clk_idx: u8,
    pub pstate: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_dpm_forced_level {
    AMD_DPM_FORCED_LEVEL_AUTO = 0x1,
    AMD_DPM_FORCED_LEVEL_MANUAL = 0x2,
    AMD_DPM_FORCED_LEVEL_LOW = 0x4,
    AMD_DPM_FORCED_LEVEL_HIGH = 0x8,
    AMD_DPM_FORCED_LEVEL_PROFILE_STANDARD = 0x10,
    AMD_DPM_FORCED_LEVEL_PROFILE_MIN_SCLK = 0x20,
    AMD_DPM_FORCED_LEVEL_PROFILE_MIN_MCLK = 0x40,
    AMD_DPM_FORCED_LEVEL_PROFILE_PEAK = 0x80,
    AMD_DPM_FORCED_LEVEL_PROFILE_EXIT = 0x100,
    AMD_DPM_FORCED_LEVEL_PERF_DETERMINISM = 0x200,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pm_state_type {
// not used for dpm
    POWER_STATE_TYPE_DEFAULT,
    POWER_STATE_TYPE_POWERSAVE,
// user selectable states
    POWER_STATE_TYPE_BATTERY,
    POWER_STATE_TYPE_BALANCED,
    POWER_STATE_TYPE_PERFORMANCE,
// internal states
    POWER_STATE_TYPE_INTERNAL_UVD,
    POWER_STATE_TYPE_INTERNAL_UVD_SD,
    POWER_STATE_TYPE_INTERNAL_UVD_HD,
    POWER_STATE_TYPE_INTERNAL_UVD_HD2,
    POWER_STATE_TYPE_INTERNAL_UVD_MVC,
    POWER_STATE_TYPE_INTERNAL_BOOT,
    POWER_STATE_TYPE_INTERNAL_THERMAL,
    POWER_STATE_TYPE_INTERNAL_ACPI,
    POWER_STATE_TYPE_INTERNAL_ULV,
    POWER_STATE_TYPE_INTERNAL_3DPERF,
}

pub const AMD_MAX_VCE_LEVELS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_vce_level {
    AMD_VCE_LEVEL_AC_ALL = 0,     /* AC, All cases */
    AMD_VCE_LEVEL_DC_EE = 1,      /* DC, entropy encoding */
    AMD_VCE_LEVEL_DC_LL_LOW = 2,  /* DC, low latency queue, res <= 720 */
    AMD_VCE_LEVEL_DC_LL_HIGH = 3, /* DC, low latency queue, 1080 >= res > 720 */
    AMD_VCE_LEVEL_DC_GP_LOW = 4,  /* DC, general purpose queue, res <= 720 */
    AMD_VCE_LEVEL_DC_GP_HIGH = 5, /* DC, general purpose queue, 1080 >= res > 720 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_fan_ctrl_mode {
    AMD_FAN_CTRL_NONE = 0,
    AMD_FAN_CTRL_MANUAL = 1,
    AMD_FAN_CTRL_AUTO = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_clock_type {
    PP_SCLK,
    PP_MCLK,
    PP_PCIE,
    PP_SOCCLK,
    PP_FCLK,
    PP_DCEFCLK,
    PP_VCLK,
    PP_VCLK1,
    PP_DCLK,
    PP_DCLK1,
    PP_ISPICLK,
    PP_ISPXCLK,
    OD_SCLK,
    OD_MCLK,
    OD_FCLK,
    OD_VDDC_CURVE,
    OD_RANGE,
    OD_VDDGFX_OFFSET,
    OD_CCLK,
    OD_FAN_CURVE,
    OD_ACOUSTIC_LIMIT,
    OD_ACOUSTIC_TARGET,
    OD_FAN_TARGET_TEMPERATURE,
    OD_FAN_MINIMUM_PWM,
    OD_FAN_ZERO_RPM_ENABLE,
    OD_FAN_ZERO_RPM_STOP_TEMP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pp_sensors {
    AMDGPU_PP_SENSOR_GFX_SCLK = 0,
    AMDGPU_PP_SENSOR_CPU_CLK,
    AMDGPU_PP_SENSOR_VDDNB,
    AMDGPU_PP_SENSOR_VDDGFX,
    AMDGPU_PP_SENSOR_UVD_VCLK,
    AMDGPU_PP_SENSOR_UVD_DCLK,
    AMDGPU_PP_SENSOR_VCE_ECCLK,
    AMDGPU_PP_SENSOR_GPU_LOAD,
    AMDGPU_PP_SENSOR_MEM_LOAD,
    AMDGPU_PP_SENSOR_GFX_MCLK,
    AMDGPU_PP_SENSOR_GPU_TEMP,
    AMDGPU_PP_SENSOR_EDGE_TEMP = AMDGPU_PP_SENSOR_GPU_TEMP,
    AMDGPU_PP_SENSOR_HOTSPOT_TEMP,
    AMDGPU_PP_SENSOR_MEM_TEMP,
    AMDGPU_PP_SENSOR_VCE_POWER,
    AMDGPU_PP_SENSOR_UVD_POWER,
    AMDGPU_PP_SENSOR_GPU_AVG_POWER, /* milliwatts */
    AMDGPU_PP_SENSOR_GPU_INPUT_POWER, /* milliwatts */
    AMDGPU_PP_SENSOR_SS_APU_SHARE,
    AMDGPU_PP_SENSOR_SS_DGPU_SHARE,
    AMDGPU_PP_SENSOR_STABLE_PSTATE_SCLK,
    AMDGPU_PP_SENSOR_STABLE_PSTATE_MCLK,
    AMDGPU_PP_SENSOR_ENABLED_SMC_FEATURES_MASK,
    AMDGPU_PP_SENSOR_MIN_FAN_RPM,
    AMDGPU_PP_SENSOR_MAX_FAN_RPM,
    AMDGPU_PP_SENSOR_VCN_POWER_STATE,
    AMDGPU_PP_SENSOR_PEAK_PSTATE_SCLK,
    AMDGPU_PP_SENSOR_PEAK_PSTATE_MCLK,
    AMDGPU_PP_SENSOR_VCN_LOAD,
    AMDGPU_PP_SENSOR_VDDBOARD,
    AMDGPU_PP_SENSOR_NODEPOWERLIMIT,
    AMDGPU_PP_SENSOR_NODEPOWER,
    AMDGPU_PP_SENSOR_GPPTRESIDENCY,
    AMDGPU_PP_SENSOR_MAXNODEPOWERLIMIT,
    AMDGPU_PP_SENSOR_UBB_POWER,
    AMDGPU_PP_SENSOR_UBB_POWER_LIMIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pp_task {
    AMD_PP_TASK_DISPLAY_CONFIG_CHANGE,
    AMD_PP_TASK_ENABLE_USER_STATE,
    AMD_PP_TASK_READJUST_POWER_STATE,
    AMD_PP_TASK_COMPLETE_INIT,
    AMD_PP_TASK_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_SMC_POWER_PROFILE {
    PP_SMC_POWER_PROFILE_UNKNOWN = -1,
    PP_SMC_POWER_PROFILE_BOOTUP_DEFAULT = 0x0,
    PP_SMC_POWER_PROFILE_FULLSCREEN3D = 0x1,
    PP_SMC_POWER_PROFILE_POWERSAVING  = 0x2,
    PP_SMC_POWER_PROFILE_VIDEO        = 0x3,
    PP_SMC_POWER_PROFILE_VR           = 0x4,
    PP_SMC_POWER_PROFILE_COMPUTE      = 0x5,
    PP_SMC_POWER_PROFILE_CUSTOM       = 0x6,
    PP_SMC_POWER_PROFILE_WINDOW3D     = 0x7,
    PP_SMC_POWER_PROFILE_CAPPED	  = 0x8,
    PP_SMC_POWER_PROFILE_UNCAPPED	  = 0x9,
    PP_SMC_POWER_PROFILE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_OD_DPM_TABLE_COMMAND {
    PP_OD_EDIT_SCLK_VDDC_TABLE,
    PP_OD_EDIT_MCLK_VDDC_TABLE,
    PP_OD_EDIT_FCLK_TABLE,
    PP_OD_EDIT_CCLK_VDDC_TABLE,
    PP_OD_EDIT_VDDC_CURVE,
    PP_OD_RESTORE_DEFAULT_TABLE,
    PP_OD_COMMIT_DPM_TABLE,
    PP_OD_EDIT_VDDGFX_OFFSET,
    PP_OD_EDIT_FAN_CURVE,
    PP_OD_EDIT_ACOUSTIC_LIMIT,
    PP_OD_EDIT_ACOUSTIC_TARGET,
    PP_OD_EDIT_FAN_TARGET_TEMPERATURE,
    PP_OD_EDIT_FAN_MINIMUM_PWM,
    PP_OD_EDIT_FAN_ZERO_RPM_ENABLE,
    PP_OD_EDIT_FAN_ZERO_RPM_STOP_TEMP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_states_info {
    pub nums: u32,
    pub states: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_HWMON_TEMP {
    PP_TEMP_EDGE = 0,
    PP_TEMP_JUNCTION,
    PP_TEMP_MEM,
    PP_TEMP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_mp1_state {
    PP_MP1_STATE_NONE,
    PP_MP1_STATE_SHUTDOWN,
    PP_MP1_STATE_UNLOAD,
    PP_MP1_STATE_RESET,
    PP_MP1_STATE_FLR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_df_cstate {
    DF_CSTATE_DISALLOW = 0,
    DF_CSTATE_ALLOW,
}

//
// DOC: amdgpu_pp_power
//
// APU power is managed to system-level requirements through the PPT
// (package power tracking) feature. PPT is intended to limit power to the
// requirements of the power source and could be dynamically updated to
// maximize APU performance within the system power budget.
//
// Two types of power measurement can be requested, where supported, with
// :c:type:`enum pp_power_type <pp_power_type>`.
//
// enum pp_power_limit_level - Used to query the power limits
//
// @PP_PWR_LIMIT_MIN: Minimum Power Limit
// @PP_PWR_LIMIT_CURRENT: Current Power Limit
// @PP_PWR_LIMIT_DEFAULT: Default Power Limit
// @PP_PWR_LIMIT_MAX: Maximum Power Limit
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_power_limit_level {
    PP_PWR_LIMIT_MIN = -1,
    PP_PWR_LIMIT_CURRENT,
    PP_PWR_LIMIT_DEFAULT,
    PP_PWR_LIMIT_MAX,
}

//
// enum pp_power_type - Used to specify the type of the requested power
//
// @PP_PWR_TYPE_SUSTAINED: manages the configurable, thermally significant
// moving average of APU power (default ~5000 ms).
// @PP_PWR_TYPE_FAST: manages the ~10 ms moving average of APU power,
// where supported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_power_type {
    PP_PWR_TYPE_SUSTAINED,
    PP_PWR_TYPE_FAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_xgmi_plpd_mode {
    XGMI_PLPD_NONE = -1,
    XGMI_PLPD_DISALLOW,
    XGMI_PLPD_DEFAULT,
    XGMI_PLPD_OPTIMIZED,
    XGMI_PLPD_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_pm_policy {
    PP_PM_POLICY_NONE = -1,
    PP_PM_POLICY_SOC_PSTATE = 0,
    PP_PM_POLICY_XGMI_PLPD,
    PP_PM_POLICY_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_policy_soc_pstate {
    SOC_PSTATE_DEFAULT = 0,
    SOC_PSTATE_0,
    SOC_PSTATE_1,
    SOC_PSTATE_2,
    SOC_PSTAT_COUNT,
}

pub const PP_POLICY_MAX_LEVELS: c_int = 5;
pub const PP_GROUP_MASK: c_uint = 0xF0000000;
pub const PP_GROUP_SHIFT: c_int = 28;
pub const PP_BLOCK_MASK: c_uint = 0x0FFFFF00;
pub const PP_BLOCK_SHIFT: c_int = 8;
pub const PP_BLOCK_GFX_CG: c_uint = 0x01;
pub const PP_BLOCK_GFX_MG: c_uint = 0x02;
pub const PP_BLOCK_GFX_3D: c_uint = 0x04;
pub const PP_BLOCK_GFX_RLC: c_uint = 0x08;
pub const PP_BLOCK_GFX_CP: c_uint = 0x10;
pub const PP_BLOCK_SYS_BIF: c_uint = 0x01;
pub const PP_BLOCK_SYS_MC: c_uint = 0x02;
pub const PP_BLOCK_SYS_ROM: c_uint = 0x04;
pub const PP_BLOCK_SYS_DRM: c_uint = 0x08;
pub const PP_BLOCK_SYS_HDP: c_uint = 0x10;
pub const PP_BLOCK_SYS_SDMA: c_uint = 0x20;
pub const PP_STATE_MASK: c_uint = 0x0000000F;
pub const PP_STATE_SHIFT: c_int = 0;
pub const PP_STATE_SUPPORT_MASK: c_uint = 0x000000F0;
pub const PP_STATE_SUPPORT_SHIFT: c_int = 0;
pub const PP_STATE_CG: c_uint = 0x01;
pub const PP_STATE_LS: c_uint = 0x02;
pub const PP_STATE_DS: c_uint = 0x04;
pub const PP_STATE_SD: c_uint = 0x08;
pub const PP_STATE_SUPPORT_CG: c_uint = 0x10;
pub const PP_STATE_SUPPORT_LS: c_uint = 0x20;
pub const PP_STATE_SUPPORT_DS: c_uint = 0x40;
pub const PP_STATE_SUPPORT_SD: c_uint = 0x80;

pub const XGMI_MODE_PSTATE_D3: c_int = 0;
pub const XGMI_MODE_PSTATE_D0: c_int = 1;
pub const NUM_HBM_INSTANCES: c_int = 4;
pub const NUM_XGMI_LINKS: c_int = 8;
pub const MAX_GFX_CLKS: c_int = 8;
pub const MAX_CLKS: c_int = 4;
pub const NUM_VCN: c_int = 4;
pub const NUM_JPEG_ENG: c_int = 32;
pub const NUM_JPEG_ENG_V1: c_int = 40;
pub const MAX_XCC: c_int = 8;
pub const NUM_XCP: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_metrics {
// Utilization Instantaneous (%)
    pub gfx_busy_inst: [u32; MAX_XCC],
    pub jpeg_busy: [u16; NUM_JPEG_ENG],
    pub vcn_busy: [u16; NUM_VCN],
// Utilization Accumulated (%)
    pub gfx_busy_acc: [u64; MAX_XCC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_metrics_v1_1 {
// Utilization Instantaneous (%)
    pub gfx_busy_inst: [u32; MAX_XCC],
    pub jpeg_busy: [u16; NUM_JPEG_ENG],
    pub vcn_busy: [u16; NUM_VCN],
// Utilization Accumulated (%)
    pub gfx_busy_acc: [u64; MAX_XCC],
// Total App Clock Counter Accumulated
    pub gfx_below_host_limit_acc: [u64; MAX_XCC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_metrics_v1_2 {
// Utilization Instantaneous (%)
    pub gfx_busy_inst: [u32; MAX_XCC],
    pub jpeg_busy: [u16; NUM_JPEG_ENG_V1],
    pub vcn_busy: [u16; NUM_VCN],
// Utilization Accumulated (%)
    pub gfx_busy_acc: [u64; MAX_XCC],
// Total App Clock Counter Accumulated
    pub gfx_below_host_limit_ppt_acc: [u64; MAX_XCC],
    pub gfx_below_host_limit_thm_acc: [u64; MAX_XCC],
    pub gfx_low_utilization_acc: [u64; MAX_XCC],
    pub gfx_below_host_limit_total_acc: [u64; MAX_XCC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pm_funcs {
// export for dpm on ci and si
    pub handle): *mut *mut int (pre_set_power_state)(void,
    pub handle): *mut *mut int (set_power_state)(void,
    pub handle): *mut *mut void (post_set_power_state)(void,
    pub handle): *mut *mut void (display_configuration_changed)(void,
    pub ps): *mut *mut *mut void (print_power_state)(void handle, void,
    pub handle): *mut *mut bool (vblank_too_short)(void,
    pub handle): *mut *mut void (notify_ac_dc)(void,
    pub equal): *mut bool,
// export for sysfs
    pub mode): *mut *mut *mut int (set_fan_control_mode)(void handle, u32,
    pub fan_mode): *mut *mut *mut int (get_fan_control_mode)(void handle, u32,
    pub speed): *mut *mut *mut int (set_fan_speed_pwm)(void handle, u32,
    pub speed): *mut *mut *mut int (get_fan_speed_pwm)(void handle, u32,
    pub mask): *mut *mut *mut int (force_clock_level)(void handle, enum pp_clock_type type, uint32_t,
    pub buf): *mut *mut *mut int (print_clock_levels)(void handle, enum pp_clock_type type, char,
    pub offset): *mut *mut *mut *mut int (emit_clock_levels)(void handle, enum pp_clock_type type, char buf, int,
    pub level): *mut *mut *mut int (force_performance_level)(void handle, enum amd_dpm_forced_level,
    pub handle): *mut *mut int (get_sclk_od)(void,
    pub value): *mut *mut *mut int (set_sclk_od)(void handle, uint32_t,
    pub handle): *mut *mut int (get_mclk_od)(void,
    pub value): *mut *mut *mut int (set_mclk_od)(void handle, uint32_t,
    pub size): *mut *mut *mut *mut int (read_sensor)(void handle, int idx, void value, int,
    pub limit): *mut *mut *mut int (get_apu_thermal_limit)(void handle, uint32_t,
    pub limit): *mut *mut *mut int (set_apu_thermal_limit)(void handle, uint32_t,
    pub handle): *mut *mut amd_dpm_forced_level (get_performance_level)(void,
    pub handle): *mut *mut amd_pm_state_type (get_current_power_state)(void,
    pub rpm): *mut *mut *mut int (get_fan_speed_rpm)(void handle, uint32_t,
    pub rpm): *mut *mut *mut int (set_fan_speed_rpm)(void handle, uint32_t,
    pub data): *mut *mut *mut int (get_pp_num_states)(void handle, struct pp_states_info,
    pub table): *mut *mut *mut int (get_pp_table)(void handle, char,
    pub size): *const *const *const *const int (set_pp_table)(void handle, char buf, size_t,
    pub m): *mut *mut *mut void (debugfs_print_current_performance_level)(void handle, struct seq_file,
    pub en): *mut *mut *mut int (switch_power_profile)(void handle, enum PP_SMC_POWER_PROFILE type, bool,
    pub pause): *mut *mut *mut int (pause_power_profile)(void handle, bool,
// export to amdgpu
    pub idx): *mut *mut *mut *mut amd_vce_state (get_vce_clock_state)(void handle, u32,
    pub user_state): *mut amd_pm_state_type,
    pub handle): *mut *mut int (load_firmware)(void,
    pub handle): *mut *mut int (wait_for_fw_loading_complete)(void,
    pub inst): c_int,
    pub msg_id): *mut *mut *mut int (set_clockgating_by_smu)(void handle, uint32_t,
    pub n): *mut *mut *mut int (set_power_limit)(void handle, uint32_t limit_type, uint32_t,
    pub power_type): pp_power_type,
    pub buf): *mut *mut *mut int (get_power_profile_mode)(void handle, char,
    pub size): *mut *mut *mut *mut int (set_power_profile_mode)(void handle, long input, uint32_t,
    pub size): *mut *mut *mut *mut int (set_fine_grain_clk_vol)(void handle, uint32_t type, long input, uint32_t,
    pub size): *mut *mut long input, uint32_t,
    pub mp1_state): *mut *mut *mut int (set_mp1_state)(void handle, enum pp_mp1_state,
    pub acquire): *mut *mut *mut int (smu_i2c_bus_access)(void handle, bool,
    pub state): *mut *mut *mut int (gfx_state_change_set)(void handle, uint32_t,
// export to DC
    pub low): *mut *mut *mut u32 (get_sclk)(void handle, bool,
    pub low): *mut *mut *mut u32 (get_mclk)(void handle, bool,
    pub input): *const amd_pp_display_configuration,
    pub clocks): *mut amd_pp_clock_info,
    pub clocks): *mut amd_pp_clocks,
    pub clocks): *mut pp_clock_levels_with_latency,
    pub clocks): *mut pp_clock_levels_with_voltage,
    pub clock_ranges): *mut c_void,
    pub clock): *mut pp_display_clock_request,
    pub clocks): *mut amd_pp_simple_clock_info,
    pub handle): *mut *mut int (notify_smu_enable_pwe)(void,
    pub handle): *mut *mut int (enable_mgpu_fan_boost)(void,
    pub count): *mut *mut *mut int (set_active_display_count)(void handle, uint32_t,
    pub clock): *mut *mut *mut int (set_hard_min_dcefclk_by_freq)(void handle, uint32_t,
    pub clock): *mut *mut *mut int (set_hard_min_fclk_by_freq)(void handle, uint32_t,
    pub clock): *mut *mut *mut int (set_min_deep_sleep_dcefclk)(void handle, uint32_t,
    pub handle): *mut *mut int (get_asic_baco_capability)(void,
    pub state): *mut *mut *mut int (get_asic_baco_state)(void handle, int,
    pub state): *mut *mut *mut int (set_asic_baco_state)(void handle, int,
    pub buf): *mut *mut *mut int (get_ppfeature_status)(void handle, char,
    pub ppfeature_masks): *mut *mut *mut int (set_ppfeature_status)(void handle, uint64_t,
    pub handle): *mut *mut int (asic_reset_mode_2)(void,
    pub handle): *mut *mut int (asic_reset_enable_gfx_features)(void,
    pub state): *mut *mut *mut int (set_df_cstate)(void handle, enum pp_df_cstate,
    pub pstate): *mut *mut *mut int (set_xgmi_pstate)(void handle, uint32_t,
    pub table): *mut *mut *mut ssize_t (get_gpu_metrics)(void handle, void,
    pub table): *mut *mut *mut ssize_t (get_temp_metrics)(void handle, enum smu_temp_metric_type type, void,
    pub type): *mut *mut *mut bool (temp_metrics_is_supported)(void handle, enum smu_temp_metric_type,
    pub table): *mut *mut *mut ssize_t (get_xcp_metrics)(void handle, int xcp_id, void,
    pub size): *mut *mut *mut *mut ssize_t (get_pm_metrics)(void handle, void pmmetrics, size_t,
    pub ranges): *mut pp_smu_wm_range_sets,
    pub disable_memory_clock_switch): bool,
    pub max_clocks): *mut pp_smu_nv_clock_table,
    pub num_states): *mut c_uint,
    pub clock_table): *mut dpm_clocks,
    pub size): *mut *mut *mut *mut *mut int (get_smu_prv_buf_details)(void handle, void addr, size_t,
    pub handle): *mut *mut void (pm_compute_clocks)(void,
    pub en): *mut *mut *mut int (notify_rlc_state)(void handle, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct metrics_table_header {
    pub structure_size: u16,
    pub format_revision: u8,
    pub content_revision: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_metrics_attr_id {
    AMDGPU_METRICS_ATTR_ID_TEMPERATURE_HOTSPOT,
    AMDGPU_METRICS_ATTR_ID_TEMPERATURE_MEM,
    AMDGPU_METRICS_ATTR_ID_TEMPERATURE_VRSOC,
    AMDGPU_METRICS_ATTR_ID_CURR_SOCKET_POWER,
    AMDGPU_METRICS_ATTR_ID_AVERAGE_GFX_ACTIVITY,
    AMDGPU_METRICS_ATTR_ID_AVERAGE_UMC_ACTIVITY,
    AMDGPU_METRICS_ATTR_ID_MEM_MAX_BANDWIDTH,
    AMDGPU_METRICS_ATTR_ID_ENERGY_ACCUMULATOR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_CLOCK_COUNTER,
    AMDGPU_METRICS_ATTR_ID_ACCUMULATION_COUNTER,
    AMDGPU_METRICS_ATTR_ID_PROCHOT_RESIDENCY_ACC,
    AMDGPU_METRICS_ATTR_ID_PPT_RESIDENCY_ACC,
    AMDGPU_METRICS_ATTR_ID_SOCKET_THM_RESIDENCY_ACC,
    AMDGPU_METRICS_ATTR_ID_VR_THM_RESIDENCY_ACC,
    AMDGPU_METRICS_ATTR_ID_HBM_THM_RESIDENCY_ACC,
    AMDGPU_METRICS_ATTR_ID_GFXCLK_LOCK_STATUS,
    AMDGPU_METRICS_ATTR_ID_PCIE_LINK_WIDTH,
    AMDGPU_METRICS_ATTR_ID_PCIE_LINK_SPEED,
    AMDGPU_METRICS_ATTR_ID_XGMI_LINK_WIDTH,
    AMDGPU_METRICS_ATTR_ID_XGMI_LINK_SPEED,
    AMDGPU_METRICS_ATTR_ID_GFX_ACTIVITY_ACC,
    AMDGPU_METRICS_ATTR_ID_MEM_ACTIVITY_ACC,
    AMDGPU_METRICS_ATTR_ID_PCIE_BANDWIDTH_ACC,
    AMDGPU_METRICS_ATTR_ID_PCIE_BANDWIDTH_INST,
    AMDGPU_METRICS_ATTR_ID_PCIE_L0_TO_RECOV_COUNT_ACC,
    AMDGPU_METRICS_ATTR_ID_PCIE_REPLAY_COUNT_ACC,
    AMDGPU_METRICS_ATTR_ID_PCIE_REPLAY_ROVER_COUNT_ACC,
    AMDGPU_METRICS_ATTR_ID_PCIE_NAK_SENT_COUNT_ACC,
    AMDGPU_METRICS_ATTR_ID_PCIE_NAK_RCVD_COUNT_ACC,
    AMDGPU_METRICS_ATTR_ID_XGMI_READ_DATA_ACC,
    AMDGPU_METRICS_ATTR_ID_XGMI_WRITE_DATA_ACC,
    AMDGPU_METRICS_ATTR_ID_XGMI_LINK_STATUS,
    AMDGPU_METRICS_ATTR_ID_FIRMWARE_TIMESTAMP,
    AMDGPU_METRICS_ATTR_ID_CURRENT_GFXCLK,
    AMDGPU_METRICS_ATTR_ID_CURRENT_SOCCLK,
    AMDGPU_METRICS_ATTR_ID_CURRENT_VCLK0,
    AMDGPU_METRICS_ATTR_ID_CURRENT_DCLK0,
    AMDGPU_METRICS_ATTR_ID_CURRENT_UCLK,
    AMDGPU_METRICS_ATTR_ID_NUM_PARTITION,
    AMDGPU_METRICS_ATTR_ID_PCIE_LC_PERF_OTHER_END_RECOVERY,
    AMDGPU_METRICS_ATTR_ID_GFX_BUSY_INST,
    AMDGPU_METRICS_ATTR_ID_JPEG_BUSY,
    AMDGPU_METRICS_ATTR_ID_VCN_BUSY,
    AMDGPU_METRICS_ATTR_ID_GFX_BUSY_ACC,
    AMDGPU_METRICS_ATTR_ID_GFX_BELOW_HOST_LIMIT_PPT_ACC,
    AMDGPU_METRICS_ATTR_ID_GFX_BELOW_HOST_LIMIT_THM_ACC,
    AMDGPU_METRICS_ATTR_ID_GFX_LOW_UTILIZATION_ACC,
    AMDGPU_METRICS_ATTR_ID_GFX_BELOW_HOST_LIMIT_TOTAL_ACC,
    AMDGPU_METRICS_ATTR_ID_TEMPERATURE_HBM,
    AMDGPU_METRICS_ATTR_ID_TEMPERATURE_MID,
    AMDGPU_METRICS_ATTR_ID_TEMPERATURE_AID,
    AMDGPU_METRICS_ATTR_ID_TEMPERATURE_XCD,
    AMDGPU_METRICS_ATTR_ID_LABEL_VERSION,
    AMDGPU_METRICS_ATTR_ID_NODE_ID,
    AMDGPU_METRICS_ATTR_ID_NODE_TEMP_RETIMER,
    AMDGPU_METRICS_ATTR_ID_NODE_TEMP_IBC,
    AMDGPU_METRICS_ATTR_ID_NODE_TEMP_IBC_2,
    AMDGPU_METRICS_ATTR_ID_NODE_TEMP_VDD18_VR,
    AMDGPU_METRICS_ATTR_ID_NODE_TEMP_04_HBM_B_VR,
    AMDGPU_METRICS_ATTR_ID_NODE_TEMP_04_HBM_D_VR,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_SOCIO_A,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_SOCIO_C,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_X0,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_X1,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_HBM_B,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_HBM_D,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_04_HBM_B,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_04_HBM_D,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_HBM_B,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_HBM_D,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_075_HBM_B,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_075_HBM_D,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_11_GTA_A,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_11_GTA_C,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDAN_075_GTA_A,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDAN_075_GTA_C,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_075_UCIE,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_065_UCIEAA,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_065_UCIEAM_A,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_065_UCIEAM_C,
    AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDAN_075,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FPGA,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FRONT,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_BACK,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_OAM7,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_IBC,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_UFPGA,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_OAM1,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_0_1_HSC,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_2_3_HSC,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_4_5_HSC,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_6_7_HSC,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FPGA_0V72_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FPGA_3V3_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_0_1_2_3_1V2_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_4_5_6_7_1V2_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_0_1_0V9_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_4_5_0V9_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_2_3_0V9_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_6_7_0V9_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_0_1_2_3_3V3_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_4_5_6_7_3V3_VR,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_IBC_HSC,
    AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_IBC,
    AMDGPU_METRICS_ATTR_ID_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_metrics_attr_type {
    AMDGPU_METRICS_TYPE_U8,
    AMDGPU_METRICS_TYPE_S8,
    AMDGPU_METRICS_TYPE_U16,
    AMDGPU_METRICS_TYPE_S16,
    AMDGPU_METRICS_TYPE_U32,
    AMDGPU_METRICS_TYPE_S32,
    AMDGPU_METRICS_TYPE_U64,
    AMDGPU_METRICS_TYPE_S64,
    AMDGPU_METRICS_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_metrics_attr_unit {
// None
    AMDGPU_METRICS_UNIT_NONE,
// MHz
    AMDGPU_METRICS_UNIT_CLOCK_1,
// Degree Celsius
    AMDGPU_METRICS_UNIT_TEMP_1,
// Watts
    AMDGPU_METRICS_UNIT_POWER_1,
// In nanoseconds
    AMDGPU_METRICS_UNIT_TIME_1,
// In 10 nanoseconds
    AMDGPU_METRICS_UNIT_TIME_2,
// Speed in GT/s
    AMDGPU_METRICS_UNIT_SPEED_1,
// Speed in 0.1 GT/s
    AMDGPU_METRICS_UNIT_SPEED_2,
// Bandwidth GB/s
    AMDGPU_METRICS_UNIT_BW_1,
// Data in KB
    AMDGPU_METRICS_UNIT_DATA_1,
// Percentage
    AMDGPU_METRICS_UNIT_PERCENT,
    AMDGPU_METRICS_UNIT_MAX,
}

pub const AMDGPU_METRICS_ATTR_UNIT_MASK: c_uint = 0xFF000000;
pub const AMDGPU_METRICS_ATTR_UNIT_SHIFT: c_int = 24;
pub const AMDGPU_METRICS_ATTR_TYPE_MASK: c_uint = 0x00F00000;
pub const AMDGPU_METRICS_ATTR_TYPE_SHIFT: c_int = 20;
pub const AMDGPU_METRICS_ATTR_ID_MASK: c_uint = 0x000FFC00;
pub const AMDGPU_METRICS_ATTR_ID_SHIFT: c_int = 10;
pub const AMDGPU_METRICS_ATTR_INST_MASK: c_uint = 0x000003FF;
pub const AMDGPU_METRICS_ATTR_INST_SHIFT: c_int = 0;

//
// gpu_metrics_v1_0 is not recommended as it's not naturally aligned.
// Use gpu_metrics_v1_1 or later instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_0 {
    pub common_header: metrics_table_header,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Temperature
    pub temperature_edge: u16,
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrgfx: u16,
    pub temperature_vrsoc: u16,
    pub temperature_vrmem: u16,
// Utilization
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Power/Energy
    pub average_socket_power: u16,
    pub energy_accumulator: u32,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_vclk0_frequency: u16,
    pub average_dclk0_frequency: u16,
    pub average_vclk1_frequency: u16,
    pub average_dclk1_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_vclk0: u16,
    pub current_dclk0: u16,
    pub current_vclk1: u16,
    pub current_dclk1: u16,
// Throttle status
    pub throttle_status: u32,
// Fans
    pub current_fan_speed: u16,
// Link width/speed
    pub pcie_link_width: u8,
    pub GT/s: uint8_t pcie_link_speed; // in 0.1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_1 {
    pub common_header: metrics_table_header,
// Temperature
    pub temperature_edge: u16,
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrgfx: u16,
    pub temperature_vrsoc: u16,
    pub temperature_vrmem: u16,
// Utilization
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Power/Energy
    pub average_socket_power: u16,
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_vclk0_frequency: u16,
    pub average_dclk0_frequency: u16,
    pub average_vclk1_frequency: u16,
    pub average_dclk1_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_vclk0: u16,
    pub current_dclk0: u16,
    pub current_vclk1: u16,
    pub current_dclk1: u16,
// Throttle status
    pub throttle_status: u32,
// Fans
    pub current_fan_speed: u16,
// Link width/speed
    pub pcie_link_width: u16,
    pub GT/s: uint16_t pcie_link_speed; // in 0.1,
    pub padding: u16,
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
    pub temperature_hbm: [u16; NUM_HBM_INSTANCES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_2 {
    pub common_header: metrics_table_header,
// Temperature
    pub temperature_edge: u16,
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrgfx: u16,
    pub temperature_vrsoc: u16,
    pub temperature_vrmem: u16,
// Utilization
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Power/Energy
    pub average_socket_power: u16,
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_vclk0_frequency: u16,
    pub average_dclk0_frequency: u16,
    pub average_vclk1_frequency: u16,
    pub average_dclk1_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_vclk0: u16,
    pub current_dclk0: u16,
    pub current_vclk1: u16,
    pub current_dclk1: u16,
// Throttle status (ASIC dependent)
    pub throttle_status: u32,
// Fans
    pub current_fan_speed: u16,
// Link width/speed
    pub pcie_link_width: u16,
    pub GT/s: uint16_t pcie_link_speed; // in 0.1,
    pub padding: u16,
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
    pub temperature_hbm: [u16; NUM_HBM_INSTANCES],
// PMFW attached timestamp (10ns resolution)
    pub firmware_timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_3 {
    pub common_header: metrics_table_header,
// Temperature
    pub temperature_edge: u16,
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrgfx: u16,
    pub temperature_vrsoc: u16,
    pub temperature_vrmem: u16,
// Utilization
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Power/Energy
    pub average_socket_power: u16,
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_vclk0_frequency: u16,
    pub average_dclk0_frequency: u16,
    pub average_vclk1_frequency: u16,
    pub average_dclk1_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_vclk0: u16,
    pub current_dclk0: u16,
    pub current_vclk1: u16,
    pub current_dclk1: u16,
// Throttle status
    pub throttle_status: u32,
// Fans
    pub current_fan_speed: u16,
// Link width/speed
    pub pcie_link_width: u16,
    pub GT/s: uint16_t pcie_link_speed; // in 0.1,
    pub padding: u16,
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
    pub temperature_hbm: [u16; NUM_HBM_INSTANCES],
// PMFW attached timestamp (10ns resolution)
    pub firmware_timestamp: u64,
// Voltage (mV)
    pub voltage_soc: u16,
    pub voltage_gfx: u16,
    pub voltage_mem: u16,
    pub padding1: u16,
// Throttle status (ASIC independent)
    pub indep_throttle_status: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_4 {
    pub common_header: metrics_table_header,
// Temperature (Celsius)
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrsoc: u16,
// Power (Watts)
    pub curr_socket_power: u16,
// Utilization (%)
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
    pub vcn_activity: [u16; NUM_VCN],
// Energy (15.259uJ (2^-16) units)
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Throttle status
    pub throttle_status: u32,
// Clock Lock Status. Each bit corresponds to clock instance
    pub gfxclk_lock_status: u32,
// Link width (number of lanes) and speed (in 0.1 GT/s)
    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,
// XGMI bus width and bitrate (in Gbps)
    pub xgmi_link_width: u16,
    pub xgmi_link_speed: u16,
// Utilization Accumulated (%)
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
// PCIE accumulated bandwidth (GB/sec)
    pub pcie_bandwidth_acc: u64,
// PCIE instantaneous bandwidth (GB/sec)
    pub pcie_bandwidth_inst: u64,
// PCIE L0 to recovery state transition accumulated count
    pub pcie_l0_to_recov_count_acc: u64,
// PCIE replay accumulated count
    pub pcie_replay_count_acc: u64,
// PCIE replay rollover accumulated count
    pub pcie_replay_rover_count_acc: u64,
// XGMI accumulated data transfer size(KiloBytes)
    pub xgmi_read_data_acc: [u64; NUM_XGMI_LINKS],
    pub xgmi_write_data_acc: [u64; NUM_XGMI_LINKS],
// PMFW attached timestamp (10ns resolution)
    pub firmware_timestamp: u64,
// Current clocks (Mhz)
    pub current_gfxclk: [u16; MAX_GFX_CLKS],
    pub current_socclk: [u16; MAX_CLKS],
    pub current_vclk0: [u16; MAX_CLKS],
    pub current_dclk0: [u16; MAX_CLKS],
    pub current_uclk: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_5 {
    pub common_header: metrics_table_header,
// Temperature (Celsius)
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrsoc: u16,
// Power (Watts)
    pub curr_socket_power: u16,
// Utilization (%)
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
    pub vcn_activity: [u16; NUM_VCN],
    pub jpeg_activity: [u16; NUM_JPEG_ENG],
// Energy (15.259uJ (2^-16) units)
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Throttle status
    pub throttle_status: u32,
// Clock Lock Status. Each bit corresponds to clock instance
    pub gfxclk_lock_status: u32,
// Link width (number of lanes) and speed (in 0.1 GT/s)
    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,
// XGMI bus width and bitrate (in Gbps)
    pub xgmi_link_width: u16,
    pub xgmi_link_speed: u16,
// Utilization Accumulated (%)
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
// PCIE accumulated bandwidth (GB/sec)
    pub pcie_bandwidth_acc: u64,
// PCIE instantaneous bandwidth (GB/sec)
    pub pcie_bandwidth_inst: u64,
// PCIE L0 to recovery state transition accumulated count
    pub pcie_l0_to_recov_count_acc: u64,
// PCIE replay accumulated count
    pub pcie_replay_count_acc: u64,
// PCIE replay rollover accumulated count
    pub pcie_replay_rover_count_acc: u64,
// PCIE NAK sent  accumulated count
    pub pcie_nak_sent_count_acc: u32,
// PCIE NAK received accumulated count
    pub pcie_nak_rcvd_count_acc: u32,
// XGMI accumulated data transfer size(KiloBytes)
    pub xgmi_read_data_acc: [u64; NUM_XGMI_LINKS],
    pub xgmi_write_data_acc: [u64; NUM_XGMI_LINKS],
// PMFW attached timestamp (10ns resolution)
    pub firmware_timestamp: u64,
// Current clocks (Mhz)
    pub current_gfxclk: [u16; MAX_GFX_CLKS],
    pub current_socclk: [u16; MAX_CLKS],
    pub current_vclk0: [u16; MAX_CLKS],
    pub current_dclk0: [u16; MAX_CLKS],
    pub current_uclk: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_6 {
    pub common_header: metrics_table_header,
// Temperature (Celsius)
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrsoc: u16,
// Power (Watts)
    pub curr_socket_power: u16,
// Utilization (%)
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
// Energy (15.259uJ (2^-16) units)
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Accumulation cycle counter
    pub accumulation_counter: u32,
// Accumulated throttler residencies
    pub prochot_residency_acc: u32,
    pub ppt_residency_acc: u32,
    pub socket_thm_residency_acc: u32,
    pub vr_thm_residency_acc: u32,
    pub hbm_thm_residency_acc: u32,
// Clock Lock Status. Each bit corresponds to clock instance
    pub gfxclk_lock_status: u32,
// Link width (number of lanes) and speed (in 0.1 GT/s)
    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,
// XGMI bus width and bitrate (in Gbps)
    pub xgmi_link_width: u16,
    pub xgmi_link_speed: u16,
// Utilization Accumulated (%)
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
// PCIE accumulated bandwidth (GB/sec)
    pub pcie_bandwidth_acc: u64,
// PCIE instantaneous bandwidth (GB/sec)
    pub pcie_bandwidth_inst: u64,
// PCIE L0 to recovery state transition accumulated count
    pub pcie_l0_to_recov_count_acc: u64,
// PCIE replay accumulated count
    pub pcie_replay_count_acc: u64,
// PCIE replay rollover accumulated count
    pub pcie_replay_rover_count_acc: u64,
// PCIE NAK sent  accumulated count
    pub pcie_nak_sent_count_acc: u32,
// PCIE NAK received accumulated count
    pub pcie_nak_rcvd_count_acc: u32,
// XGMI accumulated data transfer size(KiloBytes)
    pub xgmi_read_data_acc: [u64; NUM_XGMI_LINKS],
    pub xgmi_write_data_acc: [u64; NUM_XGMI_LINKS],
// PMFW attached timestamp (10ns resolution)
    pub firmware_timestamp: u64,
// Current clocks (Mhz)
    pub current_gfxclk: [u16; MAX_GFX_CLKS],
    pub current_socclk: [u16; MAX_CLKS],
    pub current_vclk0: [u16; MAX_CLKS],
    pub current_dclk0: [u16; MAX_CLKS],
    pub current_uclk: u16,
// Number of current partition
    pub num_partition: u16,
// XCP metrics stats
    pub xcp_stats: [amdgpu_xcp_metrics; NUM_XCP],
// PCIE other end recovery counter
    pub pcie_lc_perf_other_end_recovery: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_7 {
    pub common_header: metrics_table_header,
// Temperature (Celsius)
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrsoc: u16,
// Power (Watts)
    pub curr_socket_power: u16,
// Utilization (%)
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
// VRAM max bandwidthi (in GB/sec) at max memory clock
    pub mem_max_bandwidth: u64,
// Energy (15.259uJ (2^-16) units)
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Accumulation cycle counter
    pub accumulation_counter: u32,
// Accumulated throttler residencies
    pub prochot_residency_acc: u32,
    pub ppt_residency_acc: u32,
    pub socket_thm_residency_acc: u32,
    pub vr_thm_residency_acc: u32,
    pub hbm_thm_residency_acc: u32,
// Clock Lock Status. Each bit corresponds to clock instance
    pub gfxclk_lock_status: u32,
// Link width (number of lanes) and speed (in 0.1 GT/s)
    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,
// XGMI bus width and bitrate (in Gbps)
    pub xgmi_link_width: u16,
    pub xgmi_link_speed: u16,
// Utilization Accumulated (%)
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
// PCIE accumulated bandwidth (GB/sec)
    pub pcie_bandwidth_acc: u64,
// PCIE instantaneous bandwidth (GB/sec)
    pub pcie_bandwidth_inst: u64,
// PCIE L0 to recovery state transition accumulated count
    pub pcie_l0_to_recov_count_acc: u64,
// PCIE replay accumulated count
    pub pcie_replay_count_acc: u64,
// PCIE replay rollover accumulated count
    pub pcie_replay_rover_count_acc: u64,
// PCIE NAK sent  accumulated count
    pub pcie_nak_sent_count_acc: u32,
// PCIE NAK received accumulated count
    pub pcie_nak_rcvd_count_acc: u32,
// XGMI accumulated data transfer size(KiloBytes)
    pub xgmi_read_data_acc: [u64; NUM_XGMI_LINKS],
    pub xgmi_write_data_acc: [u64; NUM_XGMI_LINKS],
// XGMI link status(active/inactive)
    pub xgmi_link_status: [u16; NUM_XGMI_LINKS],
    pub padding: u16,
// PMFW attached timestamp (10ns resolution)
    pub firmware_timestamp: u64,
// Current clocks (Mhz)
    pub current_gfxclk: [u16; MAX_GFX_CLKS],
    pub current_socclk: [u16; MAX_CLKS],
    pub current_vclk0: [u16; MAX_CLKS],
    pub current_dclk0: [u16; MAX_CLKS],
    pub current_uclk: u16,
// Number of current partition
    pub num_partition: u16,
// XCP metrics stats
    pub xcp_stats: [amdgpu_xcp_metrics_v1_1; NUM_XCP],
// PCIE other end recovery counter
    pub pcie_lc_perf_other_end_recovery: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_8 {
    pub common_header: metrics_table_header,
// Temperature (Celsius)
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrsoc: u16,
// Power (Watts)
    pub curr_socket_power: u16,
// Utilization (%)
    pub average_gfx_activity: u16,
    pub controller: uint16_t average_umc_activity; // memory,
// VRAM max bandwidthi (in GB/sec) at max memory clock
    pub mem_max_bandwidth: u64,
// Energy (15.259uJ (2^-16) units)
    pub energy_accumulator: u64,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Accumulation cycle counter
    pub accumulation_counter: u32,
// Accumulated throttler residencies
    pub prochot_residency_acc: u32,
    pub ppt_residency_acc: u32,
    pub socket_thm_residency_acc: u32,
    pub vr_thm_residency_acc: u32,
    pub hbm_thm_residency_acc: u32,
// Clock Lock Status. Each bit corresponds to clock instance
    pub gfxclk_lock_status: u32,
// Link width (number of lanes) and speed (in 0.1 GT/s)
    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,
// XGMI bus width and bitrate (in Gbps)
    pub xgmi_link_width: u16,
    pub xgmi_link_speed: u16,
// Utilization Accumulated (%)
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
// PCIE accumulated bandwidth (GB/sec)
    pub pcie_bandwidth_acc: u64,
// PCIE instantaneous bandwidth (GB/sec)
    pub pcie_bandwidth_inst: u64,
// PCIE L0 to recovery state transition accumulated count
    pub pcie_l0_to_recov_count_acc: u64,
// PCIE replay accumulated count
    pub pcie_replay_count_acc: u64,
// PCIE replay rollover accumulated count
    pub pcie_replay_rover_count_acc: u64,
// PCIE NAK sent  accumulated count
    pub pcie_nak_sent_count_acc: u32,
// PCIE NAK received accumulated count
    pub pcie_nak_rcvd_count_acc: u32,
// XGMI accumulated data transfer size(KiloBytes)
    pub xgmi_read_data_acc: [u64; NUM_XGMI_LINKS],
    pub xgmi_write_data_acc: [u64; NUM_XGMI_LINKS],
// XGMI link status(active/inactive)
    pub xgmi_link_status: [u16; NUM_XGMI_LINKS],
    pub padding: u16,
// PMFW attached timestamp (10ns resolution)
    pub firmware_timestamp: u64,
// Current clocks (Mhz)
    pub current_gfxclk: [u16; MAX_GFX_CLKS],
    pub current_socclk: [u16; MAX_CLKS],
    pub current_vclk0: [u16; MAX_CLKS],
    pub current_dclk0: [u16; MAX_CLKS],
    pub current_uclk: u16,
// Number of current partition
    pub num_partition: u16,
// XCP metrics stats
    pub xcp_stats: [amdgpu_xcp_metrics_v1_2; NUM_XCP],
// PCIE other end recovery counter
    pub pcie_lc_perf_other_end_recovery: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_attr {
// Field type encoded with AMDGPU_METRICS_ENC_ATTR
    pub attr_encoding: u64,
// Attribute value, depends on attr_encoding
    pub attr_value: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v1_9 {
    pub common_header: metrics_table_header,
    pub attr_count: c_int,
    pub metrics_attrs: [gpu_metrics_attr; ],
}

//
// gpu_metrics_v2_0 is not recommended as it's not naturally aligned.
// Use gpu_metrics_v2_1 or later instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v2_0 {
    pub common_header: metrics_table_header,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Temperature
    pub APUs: uint16_t temperature_gfx; // gfx temperature on,
    pub APUs: uint16_t temperature_soc; // soc temperature on,
    pub APUs: uint16_t temperature_core[8]; // CPU core temperature on,
    pub temperature_l3: [u16; 2],
// Utilization
    pub average_gfx_activity: u16,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Power/Energy
    pub platform: uint16_t average_socket_power; // dGPU + APU power on A + A,
    pub average_cpu_power: u16,
    pub average_soc_power: u16,
    pub average_gfx_power: u16,
    pub APUs: uint16_t average_core_power[8]; // CPU core power on,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_fclk_frequency: u16,
    pub average_vclk_frequency: u16,
    pub average_dclk_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_fclk: u16,
    pub current_vclk: u16,
    pub current_dclk: u16,
    pub clocks: uint16_t current_coreclk[8]; // CPU core,
    pub current_l3clk: [u16; 2],
// Throttle status
    pub throttle_status: u32,
// Fans
    pub fan_pwm: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v2_1 {
    pub common_header: metrics_table_header,
// Temperature
    pub APUs: uint16_t temperature_gfx; // gfx temperature on,
    pub APUs: uint16_t temperature_soc; // soc temperature on,
    pub APUs: uint16_t temperature_core[8]; // CPU core temperature on,
    pub temperature_l3: [u16; 2],
// Utilization
    pub average_gfx_activity: u16,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Power/Energy
    pub platform: uint16_t average_socket_power; // dGPU + APU power on A + A,
    pub average_cpu_power: u16,
    pub average_soc_power: u16,
    pub average_gfx_power: u16,
    pub APUs: uint16_t average_core_power[8]; // CPU core power on,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_fclk_frequency: u16,
    pub average_vclk_frequency: u16,
    pub average_dclk_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_fclk: u16,
    pub current_vclk: u16,
    pub current_dclk: u16,
    pub clocks: uint16_t current_coreclk[8]; // CPU core,
    pub current_l3clk: [u16; 2],
// Throttle status
    pub throttle_status: u32,
// Fans
    pub fan_pwm: u16,
    pub padding: [u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v2_2 {
    pub common_header: metrics_table_header,
// Temperature
    pub APUs: uint16_t temperature_gfx; // gfx temperature on,
    pub APUs: uint16_t temperature_soc; // soc temperature on,
    pub APUs: uint16_t temperature_core[8]; // CPU core temperature on,
    pub temperature_l3: [u16; 2],
// Utilization
    pub average_gfx_activity: u16,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Power/Energy
    pub platform: uint16_t average_socket_power; // dGPU + APU power on A + A,
    pub average_cpu_power: u16,
    pub average_soc_power: u16,
    pub average_gfx_power: u16,
    pub APUs: uint16_t average_core_power[8]; // CPU core power on,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_fclk_frequency: u16,
    pub average_vclk_frequency: u16,
    pub average_dclk_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_fclk: u16,
    pub current_vclk: u16,
    pub current_dclk: u16,
    pub clocks: uint16_t current_coreclk[8]; // CPU core,
    pub current_l3clk: [u16; 2],
// Throttle status (ASIC dependent)
    pub throttle_status: u32,
// Fans
    pub fan_pwm: u16,
    pub padding: [u16; 3],
// Throttle status (ASIC independent)
    pub indep_throttle_status: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v2_3 {
    pub common_header: metrics_table_header,
// Temperature
    pub APUs: uint16_t temperature_gfx; // gfx temperature on,
    pub APUs: uint16_t temperature_soc; // soc temperature on,
    pub APUs: uint16_t temperature_core[8]; // CPU core temperature on,
    pub temperature_l3: [u16; 2],
// Utilization
    pub average_gfx_activity: u16,
    pub VCN: uint16_t average_mm_activity; // UVD or,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Power/Energy
    pub platform: uint16_t average_socket_power; // dGPU + APU power on A + A,
    pub average_cpu_power: u16,
    pub average_soc_power: u16,
    pub average_gfx_power: u16,
    pub APUs: uint16_t average_core_power[8]; // CPU core power on,
// Average clocks
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_fclk_frequency: u16,
    pub average_vclk_frequency: u16,
    pub average_dclk_frequency: u16,
// Current clocks
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_fclk: u16,
    pub current_vclk: u16,
    pub current_dclk: u16,
    pub clocks: uint16_t current_coreclk[8]; // CPU core,
    pub current_l3clk: [u16; 2],
// Throttle status (ASIC dependent)
    pub throttle_status: u32,
// Fans
    pub fan_pwm: u16,
    pub padding: [u16; 3],
// Throttle status (ASIC independent)
    pub indep_throttle_status: u64,
// Average Temperature
    pub APUs: uint16_t average_temperature_gfx; // average gfx temperature on,
    pub APUs: uint16_t average_temperature_soc; // average soc temperature on,
    pub APUs: uint16_t average_temperature_core[8]; // average CPU core temperature on,
    pub average_temperature_l3: [u16; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v2_4 {
    pub common_header: metrics_table_header,
// Temperature (unit: centi-Celsius)
    pub temperature_gfx: u16,
    pub temperature_soc: u16,
    pub temperature_core: [u16; 8],
    pub temperature_l3: [u16; 2],
// Utilization (unit: centi)
    pub average_gfx_activity: u16,
    pub average_mm_activity: u16,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Power/Energy (unit: mW)
    pub average_socket_power: u16,
    pub average_cpu_power: u16,
    pub average_soc_power: u16,
    pub average_gfx_power: u16,
    pub average_core_power: [u16; 8],
// Average clocks (unit: MHz)
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_fclk_frequency: u16,
    pub average_vclk_frequency: u16,
    pub average_dclk_frequency: u16,
// Current clocks (unit: MHz)
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_fclk: u16,
    pub current_vclk: u16,
    pub current_dclk: u16,
    pub current_coreclk: [u16; 8],
    pub current_l3clk: [u16; 2],
// Throttle status (ASIC dependent)
    pub throttle_status: u32,
// Fans
    pub fan_pwm: u16,
    pub padding: [u16; 3],
// Throttle status (ASIC independent)
    pub indep_throttle_status: u64,
// Average Temperature (unit: centi-Celsius)
    pub average_temperature_gfx: u16,
    pub average_temperature_soc: u16,
    pub average_temperature_core: [u16; 8],
    pub average_temperature_l3: [u16; 2],
// Power/Voltage (unit: mV)
    pub average_cpu_voltage: u16,
    pub average_soc_voltage: u16,
    pub average_gfx_voltage: u16,
// Power/Current (unit: mA)
    pub average_cpu_current: u16,
    pub average_soc_current: u16,
    pub average_gfx_current: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_metrics_v3_0 {
    pub common_header: metrics_table_header,
// Temperature
// gfx temperature on APUs
    pub temperature_gfx: u16,
// soc temperature on APUs
    pub temperature_soc: u16,
// CPU core temperature on APUs
    pub temperature_core: [u16; 16],
// skin temperature on APUs
    pub temperature_skin: u16,
// Utilization
// time filtered GFX busy % [0-100]
    pub average_gfx_activity: u16,
// time filtered VCN busy % [0-100]
    pub average_vcn_activity: u16,
// time filtered IPU per-column busy % [0-100]
    pub average_ipu_activity: [u16; 8],
// time filtered per-core C0 residency % [0-100]
    pub average_core_c0_activity: [u16; 16],
// time filtered DRAM read bandwidth [MB/sec]
    pub average_dram_reads: u16,
// time filtered DRAM write bandwidth [MB/sec]
    pub average_dram_writes: u16,
// time filtered IPU read bandwidth [MB/sec]
    pub average_ipu_reads: u16,
// time filtered IPU write bandwidth [MB/sec]
    pub average_ipu_writes: u16,
// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
// Power/Energy
// time filtered power used for PPT/STAPM [APU+dGPU] [mW]
    pub average_socket_power: u32,
// time filtered IPU power [mW]
    pub average_ipu_power: u16,
// time filtered APU power [mW]
    pub average_apu_power: u32,
// time filtered GFX power [mW]
    pub average_gfx_power: u32,
// time filtered dGPU power [mW]
    pub average_dgpu_power: u32,
// time filtered sum of core power across all cores in the socket [mW]
    pub average_all_core_power: u32,
// calculated core power [mW]
    pub average_core_power: [u16; 16],
// time filtered total system power [mW]
    pub average_sys_power: u16,
// maximum IRM defined STAPM power limit [mW]
    pub stapm_power_limit: u16,
// time filtered STAPM power limit [mW]
    pub current_stapm_power_limit: u16,
// time filtered clocks [MHz]
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_vpeclk_frequency: u16,
    pub average_ipuclk_frequency: u16,
    pub average_fclk_frequency: u16,
    pub average_vclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_mpipu_frequency: u16,
// Current clocks
// target core frequency [MHz]
    pub current_coreclk: [u16; 16],
// CCLK frequency limit enforced on classic cores [MHz]
    pub current_core_maxfreq: u16,
// GFXCLK frequency limit enforced on GFX [MHz]
    pub current_gfx_maxfreq: u16,
// Throttle Residency (ASIC dependent)
    pub throttle_residency_prochot: u32,
    pub throttle_residency_spl: u32,
    pub throttle_residency_fppt: u32,
    pub throttle_residency_sppt: u32,
    pub throttle_residency_thm_core: u32,
    pub throttle_residency_thm_gfx: u32,
    pub throttle_residency_thm_soc: u32,
// Metrics table alpha filter time constant [us]
    pub time_filter_alphavalue: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_pmmetrics_header {
    pub structure_size: u16,
    pub pad: u16,
    pub mp1_ip_discovery_version: u32,
    pub pmfw_version: u32,
    pub pmmetrics_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_pm_metrics {
    pub common_header: amdgpu_pmmetrics_header,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_vr_temp {
    AMDGPU_VDDCR_VDD0_TEMP,
    AMDGPU_VDDCR_VDD1_TEMP,
    AMDGPU_VDDCR_VDD2_TEMP,
    AMDGPU_VDDCR_VDD3_TEMP,
    AMDGPU_VDDCR_SOC_A_TEMP,
    AMDGPU_VDDCR_SOC_C_TEMP,
    AMDGPU_VDDCR_SOCIO_A_TEMP,
    AMDGPU_VDDCR_SOCIO_C_TEMP,
    AMDGPU_VDD_085_HBM_TEMP,
    AMDGPU_VDDCR_11_HBM_B_TEMP,
    AMDGPU_VDDCR_11_HBM_D_TEMP,
    AMDGPU_VDD_USR_TEMP,
    AMDGPU_VDDIO_11_E32_TEMP,
    AMDGPU_VR_MAX_TEMP_ENTRIES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_system_temp {
    AMDGPU_UBB_FPGA_TEMP,
    AMDGPU_UBB_FRONT_TEMP,
    AMDGPU_UBB_BACK_TEMP,
    AMDGPU_UBB_OAM7_TEMP,
    AMDGPU_UBB_IBC_TEMP,
    AMDGPU_UBB_UFPGA_TEMP,
    AMDGPU_UBB_OAM1_TEMP,
    AMDGPU_OAM_0_1_HSC_TEMP,
    AMDGPU_OAM_2_3_HSC_TEMP,
    AMDGPU_OAM_4_5_HSC_TEMP,
    AMDGPU_OAM_6_7_HSC_TEMP,
    AMDGPU_UBB_FPGA_0V72_VR_TEMP,
    AMDGPU_UBB_FPGA_3V3_VR_TEMP,
    AMDGPU_RETIMER_0_1_2_3_1V2_VR_TEMP,
    AMDGPU_RETIMER_4_5_6_7_1V2_VR_TEMP,
    AMDGPU_RETIMER_0_1_0V9_VR_TEMP,
    AMDGPU_RETIMER_4_5_0V9_VR_TEMP,
    AMDGPU_RETIMER_2_3_0V9_VR_TEMP,
    AMDGPU_RETIMER_6_7_0V9_VR_TEMP,
    AMDGPU_OAM_0_1_2_3_3V3_VR_TEMP,
    AMDGPU_OAM_4_5_6_7_3V3_VR_TEMP,
    AMDGPU_IBC_HSC_TEMP,
    AMDGPU_IBC_TEMP,
    AMDGPU_SYSTEM_MAX_TEMP_ENTRIES = 32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_node_temp {
    AMDGPU_RETIMER_X_TEMP,
    AMDGPU_OAM_X_IBC_TEMP,
    AMDGPU_OAM_X_IBC_2_TEMP,
    AMDGPU_OAM_X_VDD18_VR_TEMP,
    AMDGPU_OAM_X_04_HBM_B_VR_TEMP,
    AMDGPU_OAM_X_04_HBM_D_VR_TEMP,
    AMDGPU_NODE_MAX_TEMP_ENTRIES = 12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gpuboard_temp_metrics_v1_0 {
    pub common_header: metrics_table_header,
    pub label_version: u16,
    pub node_id: u16,
    pub accumulation_counter: u64,
// Encoded temperature in Celcius, 24:31 is sensor id 0:23 is temp value
    pub node_temp: [u32; AMDGPU_NODE_MAX_TEMP_ENTRIES],
    pub vr_temp: [u32; AMDGPU_VR_MAX_TEMP_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_baseboard_temp_metrics_v1_0 {
    pub common_header: metrics_table_header,
    pub label_version: u16,
    pub node_id: u16,
    pub accumulation_counter: u64,
// Encoded temperature in Celcius, 24:31 is sensor id 0:23 is temp value
    pub system_temp: [u32; AMDGPU_SYSTEM_MAX_TEMP_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_partition_metrics_v1_0 {
    pub common_header: metrics_table_header,
// Current clocks (Mhz)
    pub current_gfxclk: [u16; MAX_XCC],
    pub current_socclk: [u16; MAX_CLKS],
    pub current_vclk0: [u16; MAX_CLKS],
    pub current_dclk0: [u16; MAX_CLKS],
    pub current_uclk: u16,
    pub padding: u16,
// Utilization Instantaneous (%)
    pub gfx_busy_inst: [u32; MAX_XCC],
    pub jpeg_busy: [u16; NUM_JPEG_ENG_V1],
    pub vcn_busy: [u16; NUM_VCN],
// Utilization Accumulated (%)
    pub gfx_busy_acc: [u64; MAX_XCC],
// Total App Clock Counter Accumulated
    pub gfx_below_host_limit_ppt_acc: [u64; MAX_XCC],
    pub gfx_below_host_limit_thm_acc: [u64; MAX_XCC],
    pub gfx_low_utilization_acc: [u64; MAX_XCC],
    pub gfx_below_host_limit_total_acc: [u64; MAX_XCC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_partition_metrics_v1_1 {
    pub common_header: metrics_table_header,
    pub attr_count: c_int,
    pub metrics_attrs: [gpu_metrics_attr; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_xgmi_link_status {
    AMDGPU_XGMI_LINK_INACTIVE = 0,
    AMDGPU_XGMI_LINK_ACTIVE = 1,
// Status not available
    AMDGPU_XGMI_LINK_NA = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gpuboard_temp_metrics_v1_1 {
    pub common_header: metrics_table_header,
    pub attr_count: c_int,
    pub metrics_attrs: [gpu_metrics_attr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_baseboard_temp_metrics_v1_1 {
    pub common_header: metrics_table_header,
    pub attr_count: c_int,
    pub metrics_attrs: [gpu_metrics_attr; ],
}
