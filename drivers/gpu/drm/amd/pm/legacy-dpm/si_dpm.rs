//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/legacy-dpm/si_dpm.h
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

pub const MC_CG_CONFIG: c_uint = 0x96f;
pub const MC_ARB_CG: c_uint = 0x9fa;

pub const MC_ARB_DRAM_TIMING_1: c_uint = 0x9fc;
pub const MC_ARB_DRAM_TIMING_2: c_uint = 0x9fd;
pub const MC_ARB_DRAM_TIMING_3: c_uint = 0x9fe;
pub const MC_ARB_DRAM_TIMING2_1: c_uint = 0x9ff;
pub const MC_ARB_DRAM_TIMING2_2: c_uint = 0xa00;
pub const MC_ARB_DRAM_TIMING2_3: c_uint = 0xa01;
pub const NISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE: c_int = 16;
pub const RV770_ASI_DFLT: c_int = 1000;
pub const CYPRESS_HASI_DFLT: c_int = 400000;
pub const PCIE_PERF_REQ_PECI_GEN1: c_int = 2;
pub const PCIE_PERF_REQ_PECI_GEN2: c_int = 3;
pub const PCIE_PERF_REQ_PECI_GEN3: c_int = 4;

pub const SMC_STROBE_RATIO: c_uint = 0x0F;
pub const SMC_STROBE_ENABLE: c_uint = 0x10;
pub const SMC_MC_EDC_RD_FLAG: c_uint = 0x01;
pub const SMC_MC_EDC_WR_FLAG: c_uint = 0x02;
pub const SMC_MC_RTT_ENABLE: c_uint = 0x04;
pub const SMC_MC_STUTTER_EN: c_uint = 0x08;
pub const SISLANDS_MCREGISTERTABLE_INITIAL_SLOT: c_int = 0;
pub const SISLANDS_MCREGISTERTABLE_ACPI_SLOT: c_int = 1;
pub const SISLANDS_MCREGISTERTABLE_ULV_SLOT: c_int = 2;
pub const SISLANDS_MCREGISTERTABLE_FIRST_DRIVERSTATE_SLOT: c_int = 3;
pub const SISLANDS_LEAKAGE_INDEX0: c_uint = 0xff01;
pub const SISLANDS_MAX_LEAKAGE_COUNT: c_int = 4;
pub const SISLANDS_MAX_HARDWARE_POWERLEVELS: c_int = 5;
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
pub const SI_ASI_DFLT: c_int = 10000;
pub const SI_BSP_DFLT: c_uint = 0x41EB;
pub const SI_BSU_DFLT: c_uint = 0x2;
pub const SI_AH_DFLT: c_int = 5;
pub const SI_RLP_DFLT: c_int = 25;
pub const SI_RMP_DFLT: c_int = 65;
pub const SI_LHP_DFLT: c_int = 40;
pub const SI_LMP_DFLT: c_int = 15;
pub const SI_TD_DFLT: c_int = 0;
pub const SI_UTC_DFLT_00: c_uint = 0x24;
pub const SI_UTC_DFLT_01: c_uint = 0x22;
pub const SI_UTC_DFLT_02: c_uint = 0x22;
pub const SI_UTC_DFLT_03: c_uint = 0x22;
pub const SI_UTC_DFLT_04: c_uint = 0x22;
pub const SI_UTC_DFLT_05: c_uint = 0x22;
pub const SI_UTC_DFLT_06: c_uint = 0x22;
pub const SI_UTC_DFLT_07: c_uint = 0x22;
pub const SI_UTC_DFLT_08: c_uint = 0x22;
pub const SI_UTC_DFLT_09: c_uint = 0x22;
pub const SI_UTC_DFLT_10: c_uint = 0x22;
pub const SI_UTC_DFLT_11: c_uint = 0x22;
pub const SI_UTC_DFLT_12: c_uint = 0x22;
pub const SI_UTC_DFLT_13: c_uint = 0x22;
pub const SI_UTC_DFLT_14: c_uint = 0x22;
pub const SI_DTC_DFLT_00: c_uint = 0x24;
pub const SI_DTC_DFLT_01: c_uint = 0x22;
pub const SI_DTC_DFLT_02: c_uint = 0x22;
pub const SI_DTC_DFLT_03: c_uint = 0x22;
pub const SI_DTC_DFLT_04: c_uint = 0x22;
pub const SI_DTC_DFLT_05: c_uint = 0x22;
pub const SI_DTC_DFLT_06: c_uint = 0x22;
pub const SI_DTC_DFLT_07: c_uint = 0x22;
pub const SI_DTC_DFLT_08: c_uint = 0x22;
pub const SI_DTC_DFLT_09: c_uint = 0x22;
pub const SI_DTC_DFLT_10: c_uint = 0x22;
pub const SI_DTC_DFLT_11: c_uint = 0x22;
pub const SI_DTC_DFLT_12: c_uint = 0x22;
pub const SI_DTC_DFLT_13: c_uint = 0x22;
pub const SI_DTC_DFLT_14: c_uint = 0x22;
pub const SI_VRC_DFLT: c_uint = 0x0000C003;
pub const SI_VOLTAGERESPONSETIME_DFLT: c_int = 1000;
pub const SI_BACKBIASRESPONSETIME_DFLT: c_int = 1000;
pub const SI_VRU_DFLT: c_uint = 0x3;
pub const SI_SPLLSTEPTIME_DFLT: c_uint = 0x1000;
pub const SI_SPLLSTEPUNIT_DFLT: c_uint = 0x3;
pub const SI_TPU_DFLT: c_int = 0;
pub const SI_TPC_DFLT: c_uint = 0x200;
pub const SI_SSTU_DFLT: c_int = 0;
pub const SI_SST_DFLT: c_uint = 0x00C8;
pub const SI_GICST_DFLT: c_uint = 0x200;
pub const SI_FCT_DFLT: c_uint = 0x0400;
pub const SI_FCTU_DFLT: c_int = 0;
pub const SI_CTXCGTT3DRPHC_DFLT: c_uint = 0x20;
pub const SI_CTXCGTT3DRSDC_DFLT: c_uint = 0x40;
pub const SI_VDDC3DOORPHC_DFLT: c_uint = 0x100;
pub const SI_VDDC3DOORSDC_DFLT: c_uint = 0x7;
pub const SI_VDDC3DOORSU_DFLT: c_int = 0;
pub const SI_MPLLLOCKTIME_DFLT: c_int = 100;
pub const SI_MPLLRESETTIME_DFLT: c_int = 150;
pub const SI_VCOSTEPPCT_DFLT: c_int = 20;
pub const SI_ENDINGVCOSTEPPCT_DFLT: c_int = 5;
pub const SI_REFERENCEDIVIDER_DFLT: c_int = 4;
pub const SI_PM_NUMBER_OF_TC: c_int = 15;
pub const SI_PM_NUMBER_OF_SCLKS: c_int = 20;
pub const SI_PM_NUMBER_OF_MCLKS: c_int = 4;
pub const SI_PM_NUMBER_OF_VOLTAGE_LEVELS: c_int = 4;
pub const SI_PM_NUMBER_OF_ACTIVITY_LEVELS: c_int = 3;
// XXX are these ok?

pub const FDO_PWM_MODE_STATIC: c_int = 1;
pub const FDO_PWM_MODE_STATIC_RPM: c_int = 5;
pub type SMC_NIslands_MCRegisterAddress = SMC_NIslands_MCRegisterAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv7xx_power_info {
// flags
    pub /: *mut *mut bool voltage_control; / vddc,
    pub mvdd_control: bool,
    pub sclk_ss: bool,
    pub mclk_ss: bool,
    pub dynamic_ss: bool,
    pub thermal_protection: bool,
// voltage
    pub mvdd_split_frequency: u32,
    pub max_vddc: u16,
    pub max_vddc_in_table: u16,
    pub min_vddc_in_table: u16,
// stored values
    pub acpi_vddc: u16,
    pub ref_div: u32,
    pub active_auto_throttle_sources: u32,
    pub mclk_stutter_mode_threshold: u32,
    pub mclk_strobe_mode_threshold: u32,
    pub mclk_edc_enable_threshold: u32,
    pub bsp: u32,
    pub bsu: u32,
    pub pbsp: u32,
    pub pbsu: u32,
    pub dsp: u32,
    pub psp: u32,
    pub asi: u32,
    pub pasi: u32,
    pub vrc: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si_pcie_gen {
    SI_PCIE_GEN1 = 0,
    SI_PCIE_GEN2 = 1,
    SI_PCIE_GEN3 = 2,
    SI_PCIE_GEN_INVALID = 0xffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv7xx_pl {
    pub sclk: u32,
    pub mclk: u32,
    pub vddc: u16,
    pub /: *mut *mut u16 vddci; / eg+ only,
    pub flags: u32,
    pub /: *mut *mut si_pcie_gen pcie_gen; / si+ only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_ps {
    pub performance_level_count: u16,
    pub dc_compatible: bool,
    pub performance_levels: [rv7xx_pl; NISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE],
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
    pub pcie_performance_request: bool,
    pub sclk_deep_sleep: bool,
    pub smu_uvd_hs: bool,
    pub uvd_enabled: bool,
// stored values
    pub acpi_vddci: u16,
    pub mclk_edc_wr_enable_threshold: u32,
    pub vddc_voltage_table: atom_voltage_table,
    pub vddci_voltage_table: atom_voltage_table,
    pub current_rps: amdgpu_ps,
    pub requested_rps: amdgpu_ps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_power_info {
// must be first!
    pub eg: evergreen_power_info,
    pub mclk_rtt_mode_threshold: u32,
// flags
    pub support_cac_long_term_average: bool,
    pub cac_enabled: bool,
    pub cac_configuration_required: bool,
    pub driver_calculate_cac_leakage: bool,
    pub enable_power_containment: bool,
    pub enable_cac: bool,
    pub enable_sq_ramping: bool,
    pub current_ps: si_ps,
    pub requested_ps: si_ps,
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
    pub force_pcie_gen: si_pcie_gen,
    pub boot_pcie_gen: si_pcie_gen,
    pub acpi_pcie_gen: si_pcie_gen,
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
