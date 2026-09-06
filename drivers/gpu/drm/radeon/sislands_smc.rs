//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/sislands_smc.h
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

pub const SISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE: c_int = 16;
pub type PP_SIslands_Dpm2PerfLevel = PP_SIslands_Dpm2PerfLevel;
pub type PP_SIslands_DPM2Status = PP_SIslands_DPM2Status;
pub type PP_SIslands_DPM2Parameters = PP_SIslands_DPM2Parameters;
pub type PP_SIslands_PAPMStatus = PP_SIslands_PAPMStatus;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_SIslands_PAPMParameters {
    pub NearTDPLimitTherm: u32,
    pub NearTDPLimitPAPM: u32,
    pub PlatformPowerLimit: u32,
    pub dGPU_T_Limit: u32,
    pub dGPU_T_Warning: u32,
    pub dGPU_T_Hysteresis: u32,
}

pub type PP_SIslands_PAPMParameters = PP_SIslands_PAPMParameters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_SCLK_VALUE {
    pub vCG_SPLL_FUNC_CNTL: u32,
    pub vCG_SPLL_FUNC_CNTL_2: u32,
    pub vCG_SPLL_FUNC_CNTL_3: u32,
    pub vCG_SPLL_FUNC_CNTL_4: u32,
    pub vCG_SPLL_SPREAD_SPECTRUM: u32,
    pub vCG_SPLL_SPREAD_SPECTRUM_2: u32,
    pub sclk_value: u32,
}

pub type SISLANDS_SMC_SCLK_VALUE = SISLANDS_SMC_SCLK_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_MCLK_VALUE {
    pub vMPLL_FUNC_CNTL: u32,
    pub vMPLL_FUNC_CNTL_1: u32,
    pub vMPLL_FUNC_CNTL_2: u32,
    pub vMPLL_AD_FUNC_CNTL: u32,
    pub vMPLL_DQ_FUNC_CNTL: u32,
    pub vMCLK_PWRMGT_CNTL: u32,
    pub vDLL_CNTL: u32,
    pub vMPLL_SS: u32,
    pub vMPLL_SS2: u32,
    pub mclk_value: u32,
}

pub type SISLANDS_SMC_MCLK_VALUE = SISLANDS_SMC_MCLK_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_VOLTAGE_VALUE {
    pub value: u16,
    pub index: u8,
    pub phase_settings: u8,
}

pub type SISLANDS_SMC_VOLTAGE_VALUE = SISLANDS_SMC_VOLTAGE_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_HW_PERFORMANCE_LEVEL {
    pub ACIndex: u8,
    pub displayWatermark: u8,
    pub gen2PCIE: u8,
    pub UVDWatermark: u8,
    pub VCEWatermark: u8,
    pub strobeMode: u8,
    pub mcFlags: u8,
    pub padding: u8,
    pub aT: u32,
    pub bSP: u32,
    pub sclk: SISLANDS_SMC_SCLK_VALUE,
    pub mclk: SISLANDS_SMC_MCLK_VALUE,
    pub vddc: SISLANDS_SMC_VOLTAGE_VALUE,
    pub mvdd: SISLANDS_SMC_VOLTAGE_VALUE,
    pub vddci: SISLANDS_SMC_VOLTAGE_VALUE,
    pub std_vddc: SISLANDS_SMC_VOLTAGE_VALUE,
    pub hysteresisUp: u8,
    pub hysteresisDown: u8,
    pub stateFlags: u8,
    pub arbRefreshState: u8,
    pub SQPowerThrottle: u32,
    pub SQPowerThrottle_2: u32,
    pub MaxPoweredUpCU: u32,
    pub high_temp_vddc: SISLANDS_SMC_VOLTAGE_VALUE,
    pub low_temp_vddc: SISLANDS_SMC_VOLTAGE_VALUE,
    pub reserved: [u32; 2],
    pub dpm2: PP_SIslands_Dpm2PerfLevel,
}

pub const SISLANDS_SMC_STROBE_RATIO: c_uint = 0x0F;
pub const SISLANDS_SMC_STROBE_ENABLE: c_uint = 0x10;
pub const SISLANDS_SMC_MC_EDC_RD_FLAG: c_uint = 0x01;
pub const SISLANDS_SMC_MC_EDC_WR_FLAG: c_uint = 0x02;
pub const SISLANDS_SMC_MC_RTT_ENABLE: c_uint = 0x04;
pub const SISLANDS_SMC_MC_STUTTER_EN: c_uint = 0x08;
pub const SISLANDS_SMC_MC_PG_EN: c_uint = 0x10;
pub type SISLANDS_SMC_HW_PERFORMANCE_LEVEL = SISLANDS_SMC_HW_PERFORMANCE_LEVEL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_SWSTATE {
    pub flags: u8,
    pub levelCount: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub levels: [SISLANDS_SMC_HW_PERFORMANCE_LEVEL; ],
}

pub type SISLANDS_SMC_SWSTATE = SISLANDS_SMC_SWSTATE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_SWSTATE_SINGLE {
    pub flags: u8,
    pub levelCount: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub level: SISLANDS_SMC_HW_PERFORMANCE_LEVEL,
}

pub const SISLANDS_SMC_VOLTAGEMASK_VDDC: c_int = 0;
pub const SISLANDS_SMC_VOLTAGEMASK_MVDD: c_int = 1;
pub const SISLANDS_SMC_VOLTAGEMASK_VDDCI: c_int = 2;
pub const SISLANDS_SMC_VOLTAGEMASK_VDDC_PHASE_SHEDDING: c_int = 3;
pub const SISLANDS_SMC_VOLTAGEMASK_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_VOLTAGEMASKTABLE {
    pub lowMask: [u32; SISLANDS_SMC_VOLTAGEMASK_MAX],
}

pub type SISLANDS_SMC_VOLTAGEMASKTABLE = SISLANDS_SMC_VOLTAGEMASKTABLE;
pub const SISLANDS_MAX_NO_VREG_STEPS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SISLANDS_SMC_STATETABLE {
    pub thermalProtectType: u8,
    pub systemFlags: u8,
    pub maxVDDCIndexInPPTable: u8,
    pub extraFlags: u8,
    pub lowSMIO: [u32; SISLANDS_MAX_NO_VREG_STEPS],
    pub voltageMaskTable: SISLANDS_SMC_VOLTAGEMASKTABLE,
    pub phaseMaskTable: SISLANDS_SMC_VOLTAGEMASKTABLE,
    pub dpm2Params: PP_SIslands_DPM2Parameters,
    pub initialState: SISLANDS_SMC_SWSTATE_SINGLE,
    pub ACPIState: SISLANDS_SMC_SWSTATE_SINGLE,
    pub ULVState: SISLANDS_SMC_SWSTATE_SINGLE,
    pub driverState: SISLANDS_SMC_SWSTATE,
    pub dpmLevels: [SISLANDS_SMC_HW_PERFORMANCE_LEVEL; SISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE],
}

pub type SISLANDS_SMC_STATETABLE = SISLANDS_SMC_STATETABLE;
pub const SI_SMC_SOFT_REGISTER_mclk_chg_timeout: c_uint = 0x0;
pub const SI_SMC_SOFT_REGISTER_delay_vreg: c_uint = 0xC;
pub const SI_SMC_SOFT_REGISTER_delay_acpi: c_uint = 0x28;
pub const SI_SMC_SOFT_REGISTER_seq_index: c_uint = 0x5C;
pub const SI_SMC_SOFT_REGISTER_mvdd_chg_time: c_uint = 0x60;
pub const SI_SMC_SOFT_REGISTER_mclk_switch_lim: c_uint = 0x70;
pub const SI_SMC_SOFT_REGISTER_watermark_threshold: c_uint = 0x78;
pub const SI_SMC_SOFT_REGISTER_phase_shedding_delay: c_uint = 0x88;
pub const SI_SMC_SOFT_REGISTER_ulv_volt_change_delay: c_uint = 0x8C;
pub const SI_SMC_SOFT_REGISTER_mc_block_delay: c_uint = 0x98;
pub const SI_SMC_SOFT_REGISTER_ticks_per_us: c_uint = 0xA8;
pub const SI_SMC_SOFT_REGISTER_crtc_index: c_uint = 0xC4;
pub const SI_SMC_SOFT_REGISTER_mclk_change_block_cp_min: c_uint = 0xC8;
pub const SI_SMC_SOFT_REGISTER_mclk_change_block_cp_max: c_uint = 0xCC;
pub const SI_SMC_SOFT_REGISTER_non_ulv_pcie_link_width: c_uint = 0xF4;
pub const SI_SMC_SOFT_REGISTER_tdr_is_about_to_happen: c_uint = 0xFC;
pub const SI_SMC_SOFT_REGISTER_vr_hot_gpio: c_uint = 0x100;
pub const SI_SMC_SOFT_REGISTER_svi_rework_plat_type: c_uint = 0x118;
pub const SI_SMC_SOFT_REGISTER_svi_rework_gpio_id_svd: c_uint = 0x11c;
pub const SI_SMC_SOFT_REGISTER_svi_rework_gpio_id_svc: c_uint = 0x120;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_SIslands_FanTable {
    pub fdo_mode: u8,
    pub padding: u8,
    pub temp_min: i16,
    pub temp_med: i16,
    pub temp_max: i16,
    pub slope1: i16,
    pub slope2: i16,
    pub fdo_min: i16,
    pub hys_up: i16,
    pub hys_down: i16,
    pub hys_slope: i16,
    pub temp_resp_lim: i16,
    pub temp_curr: i16,
    pub slope_curr: i16,
    pub pwm_curr: i16,
    pub refresh_period: u32,
    pub fdo_max: i16,
    pub temp_src: u8,
    pub padding2: i8,
}

pub type PP_SIslands_FanTable = PP_SIslands_FanTable;
pub const SMC_SISLANDS_LKGE_LUT_NUM_OF_TEMP_ENTRIES: c_int = 16;
pub const SMC_SISLANDS_LKGE_LUT_NUM_OF_VOLT_ENTRIES: c_int = 32;
pub const SMC_SISLANDS_SCALE_I: c_int = 7;
pub const SMC_SISLANDS_SCALE_R: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_SIslands_CacConfig {
    pub cac_lkge_lut: [u16; SMC_SISLANDS_LKGE_LUT_NUM_OF_TEMP_ENTRIES][SMC_SISLANDS_LKGE_LUT_NUM_OF_VOLT_ENTRIES],
    pub lkge_lut_V0: u32,
    pub lkge_lut_Vstep: u32,
    pub WinTime: u32,
    pub R_LL: u32,
    pub calculation_repeats: u32,
    pub l2numWin_TDP: u32,
    pub dc_cac: u32,
    pub lts_truncate_n: u8,
    pub SHIFT_N: u8,
    pub log2_PG_LKG_SCALE: u8,
    pub cac_temp: u8,
    pub lkge_lut_T0: u32,
    pub lkge_lut_Tstep: u32,
}

pub type PP_SIslands_CacConfig = PP_SIslands_CacConfig;
pub const SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE: c_int = 16;
pub const SMC_SISLANDS_MC_REGISTER_ARRAY_SET_COUNT: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_SIslands_MCRegisterAddress {
    pub s0: u16,
    pub s1: u16,
}

pub type SMC_SIslands_MCRegisterAddress = SMC_SIslands_MCRegisterAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_SIslands_MCRegisterSet {
    pub value: [u32; SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE],
}

pub type SMC_SIslands_MCRegisterSet = SMC_SIslands_MCRegisterSet;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_SIslands_MCRegisters {
    pub last: u8,
    pub reserved: [u8; 3],
    pub address: [SMC_SIslands_MCRegisterAddress; SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE],
    pub data: [SMC_SIslands_MCRegisterSet; SMC_SISLANDS_MC_REGISTER_ARRAY_SET_COUNT],
}

pub type SMC_SIslands_MCRegisters = SMC_SIslands_MCRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_SIslands_MCArbDramTimingRegisterSet {
    pub mc_arb_dram_timing: u32,
    pub mc_arb_dram_timing2: u32,
    pub mc_arb_rfsh_rate: u8,
    pub mc_arb_burst_time: u8,
    pub padding: [u8; 2],
}

pub type SMC_SIslands_MCArbDramTimingRegisterSet = SMC_SIslands_MCArbDramTimingRegisterSet;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_SIslands_MCArbDramTimingRegisters {
    pub arb_current: u8,
    pub reserved: [u8; 3],
    pub data: [SMC_SIslands_MCArbDramTimingRegisterSet; 16],
}

pub type SMC_SIslands_MCArbDramTimingRegisters = SMC_SIslands_MCArbDramTimingRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_SISLANDS_SPLL_DIV_TABLE {
    pub freq: [u32; 256],
    pub ss: [u32; 256],
}

pub const SMC_SISLANDS_SPLL_DIV_TABLE_FBDIV_MASK: c_uint = 0x01ffffff;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_FBDIV_SHIFT: c_int = 0;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_PDIV_MASK: c_uint = 0xfe000000;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_PDIV_SHIFT: c_int = 25;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKV_MASK: c_uint = 0x000fffff;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKV_SHIFT: c_int = 0;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKS_MASK: c_uint = 0xfff00000;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKS_SHIFT: c_int = 20;
pub type SMC_SISLANDS_SPLL_DIV_TABLE = SMC_SISLANDS_SPLL_DIV_TABLE;
pub const SMC_SISLANDS_DTE_MAX_FILTER_STAGES: c_int = 5;
pub const SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Smc_SIslands_DTE_Configuration {
    pub tau: [u32; SMC_SISLANDS_DTE_MAX_FILTER_STAGES],
    pub R: [u32; SMC_SISLANDS_DTE_MAX_FILTER_STAGES],
    pub K: u32,
    pub T0: u32,
    pub MaxT: u32,
    pub WindowSize: u8,
    pub Tdep_count: u8,
    pub temp_select: u8,
    pub DTE_mode: u8,
    pub T_limits: [u8; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],
    pub Tdep_tau: [u32; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],
    pub Tdep_R: [u32; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],
    pub Tthreshold: u32,
}

pub type Smc_SIslands_DTE_Configuration = Smc_SIslands_DTE_Configuration;
pub const SMC_SISLANDS_DTE_STATUS_FLAG_DTE_ON: c_int = 1;
pub const SISLANDS_SMC_FIRMWARE_HEADER_LOCATION: c_uint = 0x10000;
pub const SISLANDS_SMC_FIRMWARE_HEADER_version: c_uint = 0x0;
pub const SISLANDS_SMC_FIRMWARE_HEADER_flags: c_uint = 0x4;
pub const SISLANDS_SMC_FIRMWARE_HEADER_softRegisters: c_uint = 0xC;
pub const SISLANDS_SMC_FIRMWARE_HEADER_stateTable: c_uint = 0x10;
pub const SISLANDS_SMC_FIRMWARE_HEADER_fanTable: c_uint = 0x14;
pub const SISLANDS_SMC_FIRMWARE_HEADER_CacConfigTable: c_uint = 0x18;
pub const SISLANDS_SMC_FIRMWARE_HEADER_mcRegisterTable: c_uint = 0x24;
pub const SISLANDS_SMC_FIRMWARE_HEADER_mcArbDramAutoRefreshTable: c_uint = 0x30;
pub const SISLANDS_SMC_FIRMWARE_HEADER_spllTable: c_uint = 0x38;
pub const SISLANDS_SMC_FIRMWARE_HEADER_DteConfiguration: c_uint = 0x40;
pub const SISLANDS_SMC_FIRMWARE_HEADER_PAPMParameters: c_uint = 0x48;

extern "C" {
    pub fn si_start_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_reset_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_program_jump_on_start(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_stop_smc_clock(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_start_smc_clock(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_is_smc_running(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn si_send_msg_to_smc(rdev: *mut radeon_device, msg: PPSMC_Msg) -> PPSMC_Result;
}
extern "C" {
    pub fn si_wait_for_smc_inactive(rdev: *mut radeon_device) -> PPSMC_Result;
}
extern "C" {
    pub fn si_load_smc_ucode(rdev: *mut radeon_device, limit: u32) -> c_int;
}
