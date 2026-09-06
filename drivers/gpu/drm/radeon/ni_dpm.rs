//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/ni_dpm.h
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
#[derive(Copy, Clone)]
pub struct ni_clock_registers {
    pub cg_spll_func_cntl: u32,
    pub cg_spll_func_cntl_2: u32,
    pub cg_spll_func_cntl_3: u32,
    pub cg_spll_func_cntl_4: u32,
    pub cg_spll_spread_spectrum: u32,
    pub cg_spll_spread_spectrum_2: u32,
    pub mclk_pwrmgt_cntl: u32,
    pub dll_cntl: u32,
    pub mpll_ad_func_cntl: u32,
    pub mpll_ad_func_cntl_2: u32,
    pub mpll_dq_func_cntl: u32,
    pub mpll_dq_func_cntl_2: u32,
    pub mpll_ss1: u32,
    pub mpll_ss2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; SMC_NISLANDS_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_mc_reg_table {
    pub last: u8,
    pub num_entries: u8,
    pub valid_flag: u16,
    pub mc_reg_table_entry: [ni_mc_reg_entry; MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [SMC_NIslands_MCRegisterAddress; SMC_NISLANDS_MC_REGISTER_ARRAY_SIZE],
}

pub const NISLANDS_MCREGISTERTABLE_FIRST_DRIVERSTATE_SLOT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_dc_cac_level {
    NISLANDS_DCCAC_LEVEL_0 = 0,
    NISLANDS_DCCAC_LEVEL_1,
    NISLANDS_DCCAC_LEVEL_2,
    NISLANDS_DCCAC_LEVEL_3,
    NISLANDS_DCCAC_LEVEL_4,
    NISLANDS_DCCAC_LEVEL_5,
    NISLANDS_DCCAC_LEVEL_6,
    NISLANDS_DCCAC_LEVEL_7,
    NISLANDS_DCCAC_MAX_LEVELS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_leakage_coeffients {
    pub at: u32,
    pub bt: u32,
    pub av: u32,
    pub bv: u32,
    pub t_slope: i32,
    pub t_intercept: i32,
    pub t_ref: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_cac_data {
    pub leakage_coefficients: ni_leakage_coeffients,
    pub i_leakage: u32,
    pub leakage_minimum_temperature: i32,
    pub pwr_const: u32,
    pub dc_cac_value: u32,
    pub bif_cac_value: u32,
    pub lkge_pwr: u32,
    pub mc_wr_weight: u8,
    pub mc_rd_weight: u8,
    pub allow_ovrflw: u8,
    pub num_win_tdp: u8,
    pub l2num_win_tdp: u8,
    pub lts_truncate_n: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_cac_weights {
    pub weight_tcp_sig0: u32,
    pub weight_tcp_sig1: u32,
    pub weight_ta_sig: u32,
    pub weight_tcc_en0: u32,
    pub weight_tcc_en1: u32,
    pub weight_tcc_en2: u32,
    pub weight_cb_en0: u32,
    pub weight_cb_en1: u32,
    pub weight_cb_en2: u32,
    pub weight_cb_en3: u32,
    pub weight_db_sig0: u32,
    pub weight_db_sig1: u32,
    pub weight_db_sig2: u32,
    pub weight_db_sig3: u32,
    pub weight_sxm_sig0: u32,
    pub weight_sxm_sig1: u32,
    pub weight_sxm_sig2: u32,
    pub weight_sxs_sig0: u32,
    pub weight_sxs_sig1: u32,
    pub weight_xbr_0: u32,
    pub weight_xbr_1: u32,
    pub weight_xbr_2: u32,
    pub weight_spi_sig0: u32,
    pub weight_spi_sig1: u32,
    pub weight_spi_sig2: u32,
    pub weight_spi_sig3: u32,
    pub weight_spi_sig4: u32,
    pub weight_spi_sig5: u32,
    pub weight_lds_sig0: u32,
    pub weight_lds_sig1: u32,
    pub weight_sc: u32,
    pub weight_bif: u32,
    pub weight_cp: u32,
    pub weight_pa_sig0: u32,
    pub weight_pa_sig1: u32,
    pub weight_vgt_sig0: u32,
    pub weight_vgt_sig1: u32,
    pub weight_vgt_sig2: u32,
    pub weight_dc_sig0: u32,
    pub weight_dc_sig1: u32,
    pub weight_dc_sig2: u32,
    pub weight_dc_sig3: u32,
    pub weight_uvd_sig0: u32,
    pub weight_uvd_sig1: u32,
    pub weight_spare0: u32,
    pub weight_spare1: u32,
    pub weight_sq_vsp: u32,
    pub weight_sq_vsp0: u32,
    pub weight_sq_gpr: u32,
    pub ovr_mode_spare_0: u32,
    pub ovr_val_spare_0: u32,
    pub ovr_mode_spare_1: u32,
    pub ovr_val_spare_1: u32,
    pub vsp: u32,
    pub vsp0: u32,
    pub gpr: u32,
    pub mc_read_weight: u8,
    pub mc_write_weight: u8,
    pub tid_cnt: u32,
    pub tid_unit: u32,
    pub l2_lta_window_size: u32,
    pub lts_truncate: u32,
    pub dc_cac: [u32; NISLANDS_DCCAC_MAX_LEVELS],
    pub pcie_cac: [u32; SMC_NISLANDS_BIF_LUT_NUM_OF_ENTRIES],
    pub enable_power_containment_by_default: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_ps {
    pub performance_level_count: u16,
    pub dc_compatible: bool,
    pub performance_levels: [rv7xx_pl; NISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_power_info {
// must be first!
    pub eg: evergreen_power_info,
    pub clock_registers: ni_clock_registers,
    pub mc_reg_table: ni_mc_reg_table,
    pub mclk_rtt_mode_threshold: u32,
// flags
    pub use_power_boost_limit: bool,
    pub support_cac_long_term_average: bool,
    pub cac_enabled: bool,
    pub cac_configuration_required: bool,
    pub driver_calculate_cac_leakage: bool,
    pub pc_enabled: bool,
    pub enable_power_containment: bool,
    pub enable_cac: bool,
    pub enable_sq_ramping: bool,
// smc offsets
    pub arb_table_start: u16,
    pub fan_table_start: u16,
    pub cac_table_start: u16,
    pub spll_table_start: u16,
// CAC stuff
    pub cac_data: ni_cac_data,
    pub dc_cac_table: [u32; NISLANDS_DCCAC_MAX_LEVELS],
    pub cac_weights: *const ni_cac_weights,
    pub lta_window_size: u8,
    pub lts_truncate: u8,
    pub current_ps: ni_ps,
    pub requested_ps: ni_ps,
// scratch structs
    pub smc_mc_reg_table: SMC_NIslands_MCRegisters,
    pub smc_statetable: NISLANDS_SMC_STATETABLE,
}

pub const NISLANDS_INITIAL_STATE_ARB_INDEX: c_int = 0;
pub const NISLANDS_ACPI_STATE_ARB_INDEX: c_int = 1;
pub const NISLANDS_ULV_STATE_ARB_INDEX: c_int = 2;
pub const NISLANDS_DRIVER_STATE_ARB_INDEX: c_int = 3;
pub const NISLANDS_DPM2_MAX_PULSE_SKIP: c_int = 256;
pub const NISLANDS_DPM2_NEAR_TDP_DEC: c_int = 10;
pub const NISLANDS_DPM2_ABOVE_SAFE_INC: c_int = 5;
pub const NISLANDS_DPM2_BELOW_SAFE_INC: c_int = 20;
pub const NISLANDS_DPM2_TDP_SAFE_LIMIT_PERCENT: c_int = 80;
pub const NISLANDS_DPM2_MAXPS_PERCENT_H: c_int = 90;
pub const NISLANDS_DPM2_MAXPS_PERCENT_M: c_int = 0;
pub const NISLANDS_DPM2_SQ_RAMP_MAX_POWER: c_uint = 0x3FFF;
pub const NISLANDS_DPM2_SQ_RAMP_MIN_POWER: c_uint = 0x12;
pub const NISLANDS_DPM2_SQ_RAMP_MAX_POWER_DELTA: c_uint = 0x15;
pub const NISLANDS_DPM2_SQ_RAMP_STI_SIZE: c_uint = 0x1E;
pub const NISLANDS_DPM2_SQ_RAMP_LTI_RATIO: c_uint = 0xF;
extern "C" {
    pub fn ni_dpm_vblank_too_short(rdev: *mut radeon_device) -> bool;
}
