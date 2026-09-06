//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/hwmgr.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

// Decode legacy unsigned Q24.8 watts to internal milliwatts.

pub const VOLTAGE_SCALE: c_int = 4;
pub const VOLTAGE_VID_OFFSET_SCALE1: c_int = 625;
pub const VOLTAGE_VID_OFFSET_SCALE2: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DISPLAY_GAP {
    DISPLAY_GAP_VBLANK_OR_WM = 0,   /* Wait for vblank or MCHG watermark. */
    DISPLAY_GAP_VBLANK       = 1,   /* Wait for vblank. */
    DISPLAY_GAP_WATERMARK    = 2,   /* Wait for MCHG watermark. (Note that HW may deassert WM in VBI depending on DC_STUTTER_CNTL.) */
    DISPLAY_GAP_IGNORE       = 3    /* Do not wait. */
}

pub type DISPLAY_GAP = DISPLAY_GAP;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BACO_STATE {
    BACO_STATE_OUT = 0,
    BACO_STATE_IN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vi_dpm_level {
    pub enabled: bool,
    pub value: u32,
    pub param1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vi_dpm_table {
    pub count: u32,
    pub dpm_level: [vi_dpm_level; ],
}

pub const PCIE_PERF_REQ_REMOVE_REGISTRY: c_int = 0;
pub const PCIE_PERF_REQ_FORCE_LOWPOWER: c_int = 1;
pub const PCIE_PERF_REQ_GEN1: c_int = 2;
pub const PCIE_PERF_REQ_GEN2: c_int = 3;
pub const PCIE_PERF_REQ_GEN3: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PHM_BackEnd_Magic {
    PHM_Dummy_Magic       = 0xAA5555AA,
    PHM_RV770_Magic       = 0xDCBAABCD,
    PHM_Kong_Magic        = 0x239478DF,
    PHM_NIslands_Magic    = 0x736C494E,
    PHM_Sumo_Magic        = 0x8339FA11,
    PHM_SIslands_Magic    = 0x369431AC,
    PHM_Trinity_Magic     = 0x96751873,
    PHM_CIslands_Magic    = 0x38AC78B0,
    PHM_Kv_Magic          = 0xDCBBABC0,
    PHM_VIslands_Magic    = 0x20130307,
    PHM_Cz_Magic          = 0x67DCBA25,
    PHM_Rv_Magic          = 0x20161121
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_set_power_state_input {
    pub pcurrent_state: *const pp_hw_power_state,
    pub pnew_state: *const pp_hw_power_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_clock_array {
    pub count: u32,
    pub values: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_clock_voltage_dependency_record {
    pub clk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_vceclock_voltage_dependency_record {
    pub ecclk: u32,
    pub evclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_uvdclock_voltage_dependency_record {
    pub vclk: u32,
    pub dclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_samuclock_voltage_dependency_record {
    pub samclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_acpclock_voltage_dependency_record {
    pub acpclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_clock_voltage_dependency_table {
    pub count: u32,
    pub entries: [phm_clock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_phase_shedding_limits_record {
    pub Voltage: u32,
    pub Sclk: u32,
    pub Mclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_uvd_clock_voltage_dependency_record {
    pub vclk: u32,
    pub dclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_uvd_clock_voltage_dependency_table {
    pub count: u8,
    pub entries: [phm_uvd_clock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_acp_clock_voltage_dependency_record {
    pub acpclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_acp_clock_voltage_dependency_table {
    pub count: u32,
    pub entries: [phm_acp_clock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_vce_clock_voltage_dependency_record {
    pub ecclk: u32,
    pub evclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_phase_shedding_limits_table {
    pub count: u32,
    pub entries: [phm_phase_shedding_limits_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_vceclock_voltage_dependency_table {
    pub count: u8,
    pub entries: [phm_vceclock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_uvdclock_voltage_dependency_table {
    pub count: u8,
    pub entries: [phm_uvdclock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_samuclock_voltage_dependency_table {
    pub count: u8,
    pub entries: [phm_samuclock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_acpclock_voltage_dependency_table {
    pub count: u32,
    pub entries: [phm_acpclock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_vce_clock_voltage_dependency_table {
    pub count: u8,
    pub entries: [phm_vce_clock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_ASIC_RESET_MODE {
    SMU_ASIC_RESET_MODE_0,
    SMU_ASIC_RESET_MODE_1,
    SMU_ASIC_RESET_MODE_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smumgr_func {
    pub name: *mut c_char,
    pub hwmgr): *mut *mut int (smu_init)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (smu_fini)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (start_smu)(struct pp_hwmgr,
    pub firmware): u32,
    pub hwmgr): *mut *mut int (request_smu_load_fw)(struct pp_hwmgr,
    pub firmware): u32,
    pub hwmgr): *mut *mut uint32_t (get_argument)(struct pp_hwmgr,
    pub msg): *mut *mut *mut int (send_msg_to_smc)(struct pp_hwmgr hwmgr, uint16_t,
    pub parameter): uint16_t msg, uint32_t,
    pub table): *mut c_void,
    pub hwmgr): *mut *mut int (upload_pptable_settings)(struct pp_hwmgr,
    pub type): *mut *mut *mut int (update_smc_table)(struct pp_hwmgr hwmgr, uint32_t,
    pub hwmgr): *mut *mut int (process_firmware_header)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (update_sclk_threshold)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (thermal_setup_fan_table)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (thermal_avfs_enable)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (init_smc_table)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (populate_all_graphic_levels)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (populate_all_memory_levels)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (initialize_mc_reg_table)(struct pp_hwmgr,
    pub member): *mut *mut uint32_t (get_offsetof)(uint32_t type, uint32_t,
    pub value): *mut *mut uint32_t (get_mac_definition)(uint32_t,
    pub hwmgr): *mut *mut bool (is_dpm_running)(struct pp_hwmgr,
    pub hwmgr): *mut *mut bool (is_hw_avfs_present)(struct pp_hwmgr,
    pub profile_setting): *mut *mut *mut int (update_dpm_settings)(struct pp_hwmgr hwmgr, void,
    pub /: *mut *mut *mut *mut *mut int (smc_table_manager)(struct pp_hwmgr hwmgr, uint8_t table, uint16_t table_id, bool rw); /rw: true for read, false for write,
    pub hwmgr): *mut *mut int (stop_smc)(struct pp_hwmgr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_hwmgr_func {
    pub hw_mgr): *mut *mut int (backend_init)(struct pp_hwmgr,
    pub hw_mgr): *mut *mut int (backend_fini)(struct pp_hwmgr,
    pub hw_mgr): *mut *mut int (asic_setup)(struct pp_hwmgr,
    pub hw_mgr): *mut *mut int (get_power_state_size)(struct pp_hwmgr,
    pub pcurrent_ps): *const pp_power_state,
    pub hwmgr): *mut *mut int (apply_clocks_adjust_rules)(struct pp_hwmgr,
    pub level): amd_dpm_forced_level,
    pub hw_mgr): *mut pp_hwmgr,
    pub hw_mgr): *mut pp_hwmgr,
    pub hw_ps): *mut pp_hw_power_state,
    pub ): *mut unsigned long, struct pp_power_state,
    pub hwmgr): *mut *mut int (get_num_of_pp_table_entries)(struct pp_hwmgr,
    pub bgate): *mut *mut *mut void (powergate_vce)(struct pp_hwmgr hwmgr, bool,
    pub bgate): *mut *mut *mut void (powergate_uvd)(struct pp_hwmgr hwmgr, bool,
    pub bgate): *mut *mut *mut void (powergate_acp)(struct pp_hwmgr hwmgr, bool,
    pub low): *mut *mut *mut uint32_t (get_mclk)(struct pp_hwmgr hwmgr, bool,
    pub low): *mut *mut *mut uint32_t (get_sclk)(struct pp_hwmgr hwmgr, bool,
    pub state): *const c_void,
    pub hwmgr): *mut *mut int (notify_smc_display_config_after_ps_adjustment)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (pre_display_config_changed)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (display_config_changed)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (disable_clock_power_gating)(struct pp_hwmgr,
    pub msg_id): *const u32,
    pub us_max_fan_pwm): *mut *mut *mut int (set_max_fan_rpm_output)(struct pp_hwmgr hwmgr, uint16_t,
    pub us_max_fan_pwm): *mut *mut *mut int (set_max_fan_pwm_output)(struct pp_hwmgr hwmgr, uint16_t,
    pub hwmgr): *mut *mut int (stop_thermal_controller)(struct pp_hwmgr,
    pub fan_speed_info): *mut *mut *mut int (get_fan_speed_info)(struct pp_hwmgr hwmgr, struct phm_fan_speed_info,
    pub mode): *mut *mut *mut void (set_fan_control_mode)(struct pp_hwmgr hwmgr, uint32_t,
    pub hwmgr): *mut *mut uint32_t (get_fan_control_mode)(struct pp_hwmgr,
    pub speed): *mut *mut *mut int (set_fan_speed_pwm)(struct pp_hwmgr hwmgr, uint32_t,
    pub speed): *mut *mut *mut int (get_fan_speed_pwm)(struct pp_hwmgr hwmgr, uint32_t,
    pub speed): *mut *mut *mut int (set_fan_speed_rpm)(struct pp_hwmgr hwmgr, uint32_t,
    pub speed): *mut *mut *mut int (get_fan_speed_rpm)(struct pp_hwmgr hwmgr, uint32_t,
    pub hwmgr): *mut *mut int (reset_fan_speed_to_default)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (uninitialize_thermal_controller)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (register_irq_handlers)(struct pp_hwmgr,
    pub hwmgr): *mut *mut bool (check_smc_update_required_for_display_configuration)(struct pp_hwmgr,
    pub equal): *mut bool,
    pub hwmgr): *mut *mut int (set_cpu_power_state)(struct pp_hwmgr,
    pub pstate_switch_disable): bool,
    pub ): *mut PHM_PerformanceLevelDesignation, uint32_t, PHM_PerformanceLevel,
    pub clock_info): *const *const pp_hw_power_state state, pp_clock_info,
    pub clocks): *mut *mut *mut int (get_clock_by_type)(struct pp_hwmgr hwmgr, enum amd_pp_clock_type type, struct amd_pp_clocks,
    pub clocks): *mut pp_clock_levels_with_latency,
    pub clocks): *mut pp_clock_levels_with_voltage,
    pub clock_ranges): *mut *mut *mut int (set_watermarks_for_clocks_ranges)(struct pp_hwmgr hwmgr, void,
    pub clock): *mut pp_display_clock_request,
    pub clocks): *mut *mut *mut int (get_max_high_clocks)(struct pp_hwmgr hwmgr, struct amd_pp_simple_clock_info,
    pub hwmgr): *mut *mut int (power_off_asic)(struct pp_hwmgr,
    pub mask): *mut *mut *mut int (force_clock_level)(struct pp_hwmgr hwmgr, enum pp_clock_type type, uint32_t,
    pub offset): *mut *mut pp_clock_type type, char buf, int,
    pub buf): *mut *mut *mut int (print_clock_levels)(struct pp_hwmgr hwmgr, enum pp_clock_type type, char,
    pub enable): *mut *mut *mut int (powergate_gfx)(struct pp_hwmgr hwmgr, bool,
    pub hwmgr): *mut *mut int (get_sclk_od)(struct pp_hwmgr,
    pub value): *mut *mut *mut int (set_sclk_od)(struct pp_hwmgr hwmgr, uint32_t,
    pub hwmgr): *mut *mut int (get_mclk_od)(struct pp_hwmgr,
    pub value): *mut *mut *mut int (set_mclk_od)(struct pp_hwmgr hwmgr, uint32_t,
    pub size): *mut *mut *mut *mut int (read_sensor)(struct pp_hwmgr hwmgr, int idx, void value, int,
    pub enable): *mut *mut *mut int (avfs_control)(struct pp_hwmgr hwmgr, bool,
    pub hwmgr): *mut *mut int (disable_smc_firmware_ctf)(struct pp_hwmgr,
    pub count): *mut *mut *mut int (set_active_display_count)(struct pp_hwmgr hwmgr, uint32_t,
    pub clock): *mut *mut *mut int (set_min_deep_sleep_dcefclk)(struct pp_hwmgr hwmgr, uint32_t,
    pub range): *mut *mut *mut int (start_thermal_controller)(struct pp_hwmgr hwmgr, struct PP_TemperatureRange,
    pub size): u32,
    pub range): *mut PP_TemperatureRange,
    pub buf): *mut *mut *mut int (get_power_profile_mode)(struct pp_hwmgr hwmgr, char,
    pub size): *mut *mut *mut *mut int (set_power_profile_mode)(struct pp_hwmgr hwmgr, long input, uint32_t,
    pub size): *mut *mut long input, uint32_t,
    pub size): *mut *mut long input, uint32_t,
    pub n): *mut *mut *mut int (set_power_limit)(struct pp_hwmgr hwmgr, uint32_t,
    pub hwmgr): *mut *mut int (powergate_mmhub)(struct pp_hwmgr,
    pub hwmgr): *mut *mut int (smus_notify_pwe)(struct pp_hwmgr,
    pub bgate): *mut *mut *mut int (powergate_sdma)(struct pp_hwmgr hwmgr, bool,
    pub hwmgr): *mut *mut int (enable_mgpu_fan_boost)(struct pp_hwmgr,
    pub clock): *mut *mut *mut int (set_hard_min_dcefclk_by_freq)(struct pp_hwmgr hwmgr, uint32_t,
    pub clock): *mut *mut *mut int (set_hard_min_fclk_by_freq)(struct pp_hwmgr hwmgr, uint32_t,
    pub clock): *mut *mut *mut int (set_hard_min_gfxclk_by_freq)(struct pp_hwmgr hwmgr, uint32_t,
    pub clock): *mut *mut *mut int (set_soft_max_gfxclk_by_freq)(struct pp_hwmgr hwmgr, uint32_t,
    pub hwmgr): *mut *mut int (get_bamaco_support)(struct pp_hwmgr,
    pub state): *mut *mut *mut int (get_asic_baco_state)(struct pp_hwmgr hwmgr, enum BACO_STATE,
    pub state): *mut *mut *mut int (set_asic_baco_state)(struct pp_hwmgr hwmgr, enum BACO_STATE,
    pub buf): *mut *mut *mut int (get_ppfeature_status)(struct pp_hwmgr hwmgr, char,
    pub ppfeature_masks): *mut *mut *mut int (set_ppfeature_status)(struct pp_hwmgr hwmgr, uint64_t,
    pub mp1_state): *mut *mut *mut int (set_mp1_state)(struct pp_hwmgr hwmgr, enum pp_mp1_state,
    pub mode): *mut *mut *mut int (asic_reset)(struct pp_hwmgr hwmgr, enum SMU_ASIC_RESET_MODE,
    pub acquire): *mut *mut *mut int (smu_i2c_bus_access)(struct pp_hwmgr hwmgr, bool,
    pub state): *mut *mut *mut int (set_df_cstate)(struct pp_hwmgr hwmgr, enum pp_df_cstate,
    pub pstate): *mut *mut *mut int (set_xgmi_pstate)(struct pp_hwmgr hwmgr, uint32_t,
    pub disable): bool,
    pub table): *mut *mut *mut ssize_t (get_gpu_metrics)(struct pp_hwmgr hwmgr, void,
    pub state): *mut *mut *mut int (gfx_state_change)(struct pp_hwmgr hwmgr, uint32_t,
    pub hwmgr): *mut *mut void (notify_ac_dc)(struct pp_hwmgr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_table_func {
    pub hw_mgr): *mut *mut int (pptable_init)(struct pp_hwmgr,
    pub hw_mgr): *mut *mut int (pptable_fini)(struct pp_hwmgr,
    pub hw_mgr): *mut *mut int (pptable_get_number_of_vce_state_table_entries)(struct pp_hwmgr,
    pub flag): *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union phm_cac_leakage_record {
    pub /: *mut *mut uint16_t Vddc; / in CI, we use it for StdVoltageHiSidd,
    pub /: *mut *mut uint32_t Leakage; / in CI, we use it for StdVoltageLoSidd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_cac_leakage_table {
    pub count: u32,
    pub entries: [phm_cac_leakage_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_samu_clock_voltage_dependency_record {
    pub samclk: u32,
    pub v: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_samu_clock_voltage_dependency_table {
    pub count: u8,
    pub entries: [phm_samu_clock_voltage_dependency_record; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_cac_tdp_table {
    pub usTDP: u16,
    pub usConfigurableTDP: u16,
    pub usTDC: u16,
    pub usBatteryPowerLimit: u16,
    pub usSmallPowerLimit: u16,
    pub usLowCACLeakage: u16,
    pub usHighCACLeakage: u16,
    pub usMaximumPowerDeliveryLimit: u16,
    pub usEDCLimit: u16,
    pub usOperatingTempMinLimit: u16,
    pub usOperatingTempMaxLimit: u16,
    pub usOperatingTempStep: u16,
    pub usOperatingTempHyst: u16,
    pub usDefaultTargetOperatingTemp: u16,
    pub usTargetOperatingTemp: u16,
    pub usPowerTuneDataSetID: u16,
    pub usSoftwareShutdownTemp: u16,
    pub usClockStretchAmount: u16,
    pub usTemperatureLimitHotspot: u16,
    pub usTemperatureLimitLiquid1: u16,
    pub usTemperatureLimitLiquid2: u16,
    pub usTemperatureLimitVrVddc: u16,
    pub usTemperatureLimitVrMvdd: u16,
    pub usTemperatureLimitPlx: u16,
    pub ucLiquid1_I2C_address: u8,
    pub ucLiquid2_I2C_address: u8,
    pub ucLiquid_I2C_Line: u8,
    pub ucVr_I2C_address: u8,
    pub ucVr_I2C_Line: u8,
    pub ucPlx_I2C_address: u8,
    pub ucPlx_I2C_Line: u8,
    pub usBoostPowerLimit: u32,
    pub ucCKS_LDO_REFSEL: u8,
    pub ucHotSpotOnly: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_tdp_table {
    pub usTDP: u16,
    pub usConfigurableTDP: u16,
    pub usTDC: u16,
    pub usBatteryPowerLimit: u16,
    pub usSmallPowerLimit: u16,
    pub usLowCACLeakage: u16,
    pub usHighCACLeakage: u16,
    pub usMaximumPowerDeliveryLimit: u16,
    pub usEDCLimit: u16,
    pub usOperatingTempMinLimit: u16,
    pub usOperatingTempMaxLimit: u16,
    pub usOperatingTempStep: u16,
    pub usOperatingTempHyst: u16,
    pub usDefaultTargetOperatingTemp: u16,
    pub usTargetOperatingTemp: u16,
    pub usPowerTuneDataSetID: u16,
    pub usSoftwareShutdownTemp: u16,
    pub usClockStretchAmount: u16,
    pub usTemperatureLimitTedge: u16,
    pub usTemperatureLimitHotspot: u16,
    pub usTemperatureLimitLiquid1: u16,
    pub usTemperatureLimitLiquid2: u16,
    pub usTemperatureLimitHBM: u16,
    pub usTemperatureLimitVrVddc: u16,
    pub usTemperatureLimitVrMvdd: u16,
    pub usTemperatureLimitPlx: u16,
    pub ucLiquid1_I2C_address: u8,
    pub ucLiquid2_I2C_address: u8,
    pub ucLiquid_I2C_Line: u8,
    pub ucVr_I2C_address: u8,
    pub ucVr_I2C_Line: u8,
    pub ucPlx_I2C_address: u8,
    pub ucPlx_I2C_Line: u8,
    pub ucLiquid_I2C_LineSDA: u8,
    pub ucVr_I2C_LineSDA: u8,
    pub ucPlx_I2C_LineSDA: u8,
    pub usBoostPowerLimit: u32,
    pub usBoostStartTemperature: u16,
    pub usBoostStopTemperature: u16,
    pub ulBoostClock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppm_table {
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
pub struct phm_vq_budgeting_record {
    pub ulCUs: u32,
    pub ulSustainableSOCPowerLimitLow: u32,
    pub ulSustainableSOCPowerLimitHigh: u32,
    pub ulMinSclkLow: u32,
    pub ulMinSclkHigh: u32,
    pub ucDispConfig: u8,
    pub ulDClk: u32,
    pub ulEClk: u32,
    pub ulSustainableSclk: u32,
    pub ulSustainableCUs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_vq_budgeting_table {
    pub numEntries: u8,
    pub entries: [phm_vq_budgeting_record; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_clock_and_voltage_limits {
    pub sclk: u32,
    pub mclk: u32,
    pub gfxclk: u32,
    pub vddc: u16,
    pub vddci: u16,
    pub vddgfx: u16,
    pub vddmem: u16,
}

// Structure to hold PPTable information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_information {
    pub vdd_dep_on_sclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_mclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_socclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_dcefclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub valid_sclk_values: *mut phm_clock_array,
    pub valid_mclk_values: *mut phm_clock_array,
    pub valid_socclk_values: *mut phm_clock_array,
    pub valid_dcefclk_values: *mut phm_clock_array,
    pub max_clock_voltage_on_dc: phm_clock_and_voltage_limits,
    pub max_clock_voltage_on_ac: phm_clock_and_voltage_limits,
    pub ppm_parameter_table: *mut phm_ppm_table,
    pub cac_dtp_table: *mut phm_cac_tdp_table,
    pub tdp_table: *mut phm_tdp_table,
    pub mm_dep_table: *mut phm_ppt_v1_mm_clock_voltage_dependency_table,
    pub vddc_lookup_table: *mut phm_ppt_v1_voltage_lookup_table,
    pub vddgfx_lookup_table: *mut phm_ppt_v1_voltage_lookup_table,
    pub vddmem_lookup_table: *mut phm_ppt_v1_voltage_lookup_table,
    pub pcie_table: *mut phm_ppt_v1_pcie_table,
    pub gpio_table: *mut phm_ppt_v1_gpio_table,
    pub us_ulv_voltage_offset: u16,
    pub us_ulv_smnclk_did: u16,
    pub us_ulv_mp1clk_did: u16,
    pub us_ulv_gfxclk_bypass: u16,
    pub us_gfxclk_slew_rate: u16,
    pub us_min_gfxclk_freq_limit: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v2_information {
    pub vdd_dep_on_sclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_mclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_socclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_dcefclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_pixclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_dispclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub vdd_dep_on_phyclk: *mut phm_ppt_v1_clock_voltage_dependency_table,
    pub mm_dep_table: *mut phm_ppt_v1_mm_clock_voltage_dependency_table,
    pub vddc_dep_on_dalpwrl: *mut phm_clock_voltage_dependency_table,
    pub valid_sclk_values: *mut phm_clock_array,
    pub valid_mclk_values: *mut phm_clock_array,
    pub valid_socclk_values: *mut phm_clock_array,
    pub valid_dcefclk_values: *mut phm_clock_array,
    pub max_clock_voltage_on_dc: phm_clock_and_voltage_limits,
    pub max_clock_voltage_on_ac: phm_clock_and_voltage_limits,
    pub ppm_parameter_table: *mut phm_ppm_table,
    pub cac_dtp_table: *mut phm_cac_tdp_table,
    pub tdp_table: *mut phm_tdp_table,
    pub vddc_lookup_table: *mut phm_ppt_v1_voltage_lookup_table,
    pub vddgfx_lookup_table: *mut phm_ppt_v1_voltage_lookup_table,
    pub vddmem_lookup_table: *mut phm_ppt_v1_voltage_lookup_table,
    pub vddci_lookup_table: *mut phm_ppt_v1_voltage_lookup_table,
    pub pcie_table: *mut phm_ppt_v1_pcie_table,
    pub us_ulv_voltage_offset: u16,
    pub us_ulv_smnclk_did: u16,
    pub us_ulv_mp1clk_did: u16,
    pub us_ulv_gfxclk_bypass: u16,
    pub us_gfxclk_slew_rate: u16,
    pub us_min_gfxclk_freq_limit: u16,
    pub uc_gfx_dpm_voltage_mode: u8,
    pub uc_soc_dpm_voltage_mode: u8,
    pub uc_uclk_dpm_voltage_mode: u8,
    pub uc_uvd_dpm_voltage_mode: u8,
    pub uc_vce_dpm_voltage_mode: u8,
    pub uc_mp0_dpm_voltage_mode: u8,
    pub uc_dcef_dpm_voltage_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v3_information {
    pub uc_thermal_controller_type: u8,
    pub us_small_power_limit1: u16,
    pub us_small_power_limit2: u16,
    pub us_boost_power_limit: u16,
    pub us_od_turbo_power_limit: u16,
    pub us_od_powersave_power_limit: u16,
    pub us_software_shutdown_temp: u16,
    pub power_saving_clock_max: *mut u32,
    pub power_saving_clock_min: *mut u32,
    pub od_feature_capabilities: *mut u8,
    pub od_settings_max: *mut u32,
    pub od_settings_min: *mut u32,
    pub smc_pptable: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_dynamic_state_info {
    pub vddc_dependency_on_sclk: *mut phm_clock_voltage_dependency_table,
    pub vddci_dependency_on_mclk: *mut phm_clock_voltage_dependency_table,
    pub vddc_dependency_on_mclk: *mut phm_clock_voltage_dependency_table,
    pub mvdd_dependency_on_mclk: *mut phm_clock_voltage_dependency_table,
    pub vddc_dependency_on_display_clock: *mut phm_clock_voltage_dependency_table,
    pub valid_sclk_values: *mut phm_clock_array,
    pub valid_mclk_values: *mut phm_clock_array,
    pub max_clock_voltage_on_dc: phm_clock_and_voltage_limits,
    pub max_clock_voltage_on_ac: phm_clock_and_voltage_limits,
    pub mclk_sclk_ratio: u32,
    pub sclk_mclk_delta: u32,
    pub vddc_vddci_delta: u32,
    pub min_vddc_for_pcie_gen2: u32,
    pub cac_leakage_table: *mut phm_cac_leakage_table,
    pub vddc_phase_shed_limits_table: *mut phm_phase_shedding_limits_table,
// vce_clock_voltage_dependency_table;
// uvd_clock_voltage_dependency_table;
// acp_clock_voltage_dependency_table;
// samu_clock_voltage_dependency_table;
    pub ppm_parameter_table: *mut phm_ppm_table,
    pub cac_dtp_table: *mut phm_cac_tdp_table,
    pub vdd_gfx_dependency_on_sclk: *mut phm_clock_voltage_dependency_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_fan_info {
    pub bNoFan: bool,
    pub ucTachometerPulsesPerRevolution: u8,
    pub ulMinRPM: u32,
    pub ulMaxRPM: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_advance_fan_control_parameters {
    pub /: *mut *mut uint16_t usTMin; / The temperature, in 0.01 centigrades, below which we just run at a minimal PWM.,
    pub /: *mut *mut uint16_t usTMed; / The middle temperature where we change slopes.,
    pub /: *mut *mut uint16_t usTHigh; / The high temperature for setting the second slope.,
    pub /: *mut *mut uint16_t usPWMMin; / The minimum PWM value in percent (0.01% increments).,
    pub /: *mut *mut uint16_t usPWMMed; / The PWM value (in percent) at TMed.,
    pub /: *mut *mut uint16_t usPWMHigh; / The PWM value at THigh.,
    pub /: *mut *mut uint8_t ucTHyst; / Temperature hysteresis. Integer.,
    pub /: *mut *mut uint32_t ulCycleDelay; / The time between two invocations of the fan control routine in microseconds.,
    pub /: *mut *mut uint16_t usTMax; / The max temperature,
    pub ucFanControlMode: u8,
    pub usFanPWMMinLimit: u16,
    pub usFanPWMMaxLimit: u16,
    pub usFanPWMStep: u16,
    pub usDefaultMaxFanPWM: u16,
    pub usFanOutputSensitivity: u16,
    pub usDefaultFanOutputSensitivity: u16,
    pub /: *mut *mut uint16_t usMaxFanPWM; / The max Fan PWM value for Fuzzy Fan Control feature,
    pub /: *mut *mut uint16_t usFanRPMMinLimit; / Minimum limit range in percentage, need to calculate based on minRPM/MaxRpm,
    pub /: *mut *mut uint16_t usFanRPMMaxLimit; / Maximum limit range in percentage, usually set to 100% by default,
    pub /: *mut *mut uint16_t usFanRPMStep; / Step increments/decerements, in percent,
    pub /: *mut *mut uint16_t usDefaultMaxFanRPM; / The max Fan RPM value for Fuzzy Fan Control feature, default from PPTable,
    pub /: *mut *mut uint16_t usMaxFanRPM; / The max Fan RPM value for Fuzzy Fan Control feature, user defined,
    pub /: *mut *mut uint16_t usFanCurrentLow; / Low current,
    pub /: *mut *mut uint16_t usFanCurrentHigh; / High current,
    pub /: *mut *mut uint16_t usFanRPMLow; / Low RPM,
    pub /: *mut *mut uint16_t usFanRPMHigh; / High RPM,
    pub /: *mut *mut uint32_t ulMinFanSCLKAcousticLimit; / Minimum Fan Controller SCLK Frequency Acoustic Limit.,
    pub /: *mut *mut uint8_t ucTargetTemperature; / Advanced fan controller target temperature.,
    pub /: *mut *mut uint8_t ucMinimumPWMLimit; / The minimum PWM that the advanced fan controller can set. This should be set to the highest PWM that will run the fan at its lowest RPM.,
    pub /: *mut *mut uint16_t usFanGainEdge; / The following is added for Fiji,
    pub usFanGainHotspot: u16,
    pub usFanGainLiquid: u16,
    pub usFanGainVrVddc: u16,
    pub usFanGainVrMvdd: u16,
    pub usFanGainPlx: u16,
    pub usFanGainHbm: u16,
    pub ucEnableZeroRPM: u8,
    pub ucFanStopTemperature: u8,
    pub ucFanStartTemperature: u8,
    pub /: *mut *mut uint32_t ulMaxFanSCLKAcousticLimit; / Maximum Fan Controller SCLK Frequency Acoustic Limit.,
    pub ulTargetGfxClk: u32,
    pub usZeroRPMStartTemperature: u16,
    pub usZeroRPMStopTemperature: u16,
    pub usMGpuThrottlingRPMLimit: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_thermal_controller_info {
    pub ucType: u8,
    pub ucI2cLine: u8,
    pub ucI2cAddress: u8,
    pub use_hw_fan_control: u8,
    pub fanInfo: pp_fan_info,
    pub advanceFanControlParameters: pp_advance_fan_control_parameters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_microcode_version_info {
    pub SMC: u32,
    pub DMCU: u32,
    pub MC: u32,
    pub NB: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_TABLE_VERSION {
    PP_TABLE_V0 = 0,
    PP_TABLE_V1,
    PP_TABLE_V2,
    PP_TABLE_MAX
}

//
// The main hardware manager structure.
//
pub const Workload_Policy_Max: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_hwmgr {
    pub adev: *mut c_void,
    pub chip_family: u32,
    pub chip_id: u32,
    pub smu_version: u32,
    pub not_vf: bool,
    pub pm_en: bool,
    pub pp_one_vf: bool,
    pub msg_lock: mutex,
    pub pp_table_version: u32,
    pub device: *mut c_void,
    pub smumgr: *mut pp_smumgr,
    pub soft_pp_table: *const c_void,
    pub soft_pp_table_size: u32,
    pub hardcode_pp_table: *mut c_void,
    pub need_pp_table_upload: bool,
    pub vce_states: [amd_vce_state; AMD_MAX_VCE_LEVELS],
    pub num_vce_state_tables: u32,
    pub dpm_level: amd_dpm_forced_level,
    pub saved_dpm_level: amd_dpm_forced_level,
    pub request_dpm_level: amd_dpm_forced_level,
    pub usec_timeout: u32,
    pub pptable: *mut c_void,
    pub platform_descriptor: phm_platform_descriptor,
    pub backend: *mut c_void,
    pub smu_backend: *mut c_void,
    pub smumgr_funcs: *const pp_smumgr_func,
    pub is_kicker: bool,
    pub dyn_state: phm_dynamic_state_info,
    pub hwmgr_func: *const pp_hwmgr_func,
    pub pptable_func: *const pp_table_func,
    pub ps: *mut pp_power_state,
    pub num_ps: u32,
    pub thermal_controller: pp_thermal_controller_info,
    pub fan_ctrl_is_in_default_mode: bool,
    pub fan_ctrl_default_mode: u32,
    pub fan_ctrl_enabled: bool,
    pub tmin: u32,
    pub microcode_version_info: phm_microcode_version_info,
    pub ps_size: u32,
    pub current_ps: *mut pp_power_state,
    pub request_ps: *mut pp_power_state,
    pub boot_ps: *mut pp_power_state,
    pub uvd_ps: *mut pp_power_state,
    pub display_config: *const amd_pp_display_configuration,
    pub feature_mask: u32,
    pub avfs_supported: bool,
// UMD Pstate
    pub en_umd_pstate: bool,
    pub power_profile_mode: u32,
    pub default_power_profile_mode: u32,
    pub pstate_sclk: u32,
    pub pstate_mclk: u32,
    pub od_enabled: bool,
    pub power_limit: u32,
    pub default_power_limit: u32,
    pub workload_mask: u32,
    pub workload_prority: [u32; Workload_Policy_Max],
    pub workload_setting: [u32; Workload_Policy_Max],
    pub gfxoff_state_changed_by_workload: bool,
    pub pstate_sclk_peak: u32,
    pub pstate_mclk_peak: u32,
    pub swctf_delayed_work: delayed_work,
}

extern "C" {
    pub fn hwmgr_early_init(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn hwmgr_sw_init(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn hwmgr_sw_fini(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn hwmgr_hw_init(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn hwmgr_hw_fini(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn hwmgr_suspend(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn hwmgr_resume(hwmgr: *mut pp_hwmgr) -> c_int;
}
pub const PHM_ENTIRE_REGISTER_MASK: c_uint = 0xFFFFFFFFU;
extern "C" {
    pub fn smu7_init_function_pointers(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu8_init_function_pointers(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn vega12_hwmgr_init(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn vega20_hwmgr_init(hwmgr: *mut pp_hwmgr) -> c_int;
}
