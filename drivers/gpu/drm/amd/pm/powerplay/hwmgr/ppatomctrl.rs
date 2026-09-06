//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/ppatomctrl.h
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

// As returned from PowerConnectorDetectionTable.
pub const PP_ATOM_POWER_BUDGET_DISABLE_OVERDRIVE: c_uint = 0x80;
pub const PP_ATOM_POWER_BUDGET_SHOW_WARNING: c_uint = 0x40;
pub const PP_ATOM_POWER_BUDGET_SHOW_WAIVER: c_uint = 0x20;
pub const PP_ATOM_POWER_POWER_BUDGET_BEHAVIOUR: c_uint = 0x0F;
// New functions for Evergreen and beyond.
pub const PP_ATOMCTRL_MAX_VOLTAGE_ENTRIES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_clock_dividers {
    pub pll_post_divider: u32,
    pub pll_feedback_divider: u32,
    pub pll_ref_divider: u32,
    pub enable_post_divider: bool,
}

pub type pp_atomctrl_clock_dividers = pp_atomctrl_clock_dividers;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pp_atomctrl_tcipll_fb_divider {
    pub 14: uint32_t ul_fb_div_frac :,
    pub 12: uint32_t ul_fb_div :,
    pub 6: uint32_t un_used :,
}

pub type pp_atomctrl_tcipll_fb_divider = pp_atomctrl_tcipll_fb_divider;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_clock_dividers_rv730 {
    pub pll_post_divider: u32,
    pub mpll_feedback_divider: pp_atomctrl_tcipll_fb_divider,
    pub pll_ref_divider: u32,
    pub enable_post_divider: bool,
    pub enable_dithen: bool,
    pub vco_mode: u32,
}

pub type pp_atomctrl_clock_dividers_rv730 = pp_atomctrl_clock_dividers_rv730;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_clock_dividers_kong {
    pub pll_post_divider: u32,
    pub real_clock: u32,
}

pub type pp_atomctrl_clock_dividers_kong = pp_atomctrl_clock_dividers_kong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_clock_dividers_ci {
    pub /: *mut *mut uint32_t pll_post_divider; / post divider value,
    pub real_clock: u32,
    pub /: *mut *mut pp_atomctrl_tcipll_fb_divider ul_fb_div; / Output Parameter: PLL FB divider,
    pub /: *mut *mut uint8_t uc_pll_ref_div; / Output Parameter: PLL ref divider,
    pub /: *mut *mut uint8_t uc_pll_post_div; / Output Parameter: PLL post divider,
    pub /: *mut *mut uint8_t uc_pll_cntl_flag; /Output Flags: control flag,
}

pub type pp_atomctrl_clock_dividers_ci = pp_atomctrl_clock_dividers_ci;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_clock_dividers_vi {
    pub /: *mut *mut uint32_t pll_post_divider; / post divider value,
    pub real_clock: u32,
    pub /: *mut *mut pp_atomctrl_tcipll_fb_divider ul_fb_div; /Output Parameter: PLL FB divider,
    pub /: *mut *mut uint8_t uc_pll_ref_div; /Output Parameter: PLL ref divider,
    pub /: *mut *mut uint8_t uc_pll_post_div; /Output Parameter: PLL post divider,
    pub /: *mut *mut uint8_t uc_pll_cntl_flag; /Output Flags: control flag,
}

pub type pp_atomctrl_clock_dividers_vi = pp_atomctrl_clock_dividers_vi;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_clock_dividers_ai {
    pub usSclk_fcw_frac: u16,
    pub usSclk_fcw_int: u16,
    pub ucSclkPostDiv: u8,
    pub ucSclkVcoMode: u8,
    pub ucSclkPllRange: u8,
    pub ucSscEnable: u8,
    pub usSsc_fcw1_frac: u16,
    pub usSsc_fcw1_int: u16,
    pub usReserved: u16,
    pub usPcc_fcw_int: u16,
    pub usSsc_fcw_slew_frac: u16,
    pub usPcc_fcw_slew_frac: u16,
}

pub type pp_atomctrl_clock_dividers_ai = pp_atomctrl_clock_dividers_ai;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pp_atomctrl_s_mpll_fb_divider {
    pub 12: uint32_t cl_kf :,
    pub 12: uint32_t clk_frac :,
    pub 8: uint32_t un_used :,
}

pub type pp_atomctrl_s_mpll_fb_divider = pp_atomctrl_s_mpll_fb_divider;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_atomctrl_spread_spectrum_mode {
    pp_atomctrl_spread_spectrum_mode_down = 0,
    pp_atomctrl_spread_spectrum_mode_center
}

pub type pp_atomctrl_spread_spectrum_mode = pp_atomctrl_spread_spectrum_mode;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_memory_clock_param {
    pub mpll_fb_divider: pp_atomctrl_s_mpll_fb_divider,
    pub mpll_post_divider: u32,
    pub bw_ctrl: u32,
    pub dll_speed: u32,
    pub vco_mode: u32,
    pub yclk_sel: u32,
    pub qdr: u32,
    pub half_rate: u32,
}

pub type pp_atomctrl_memory_clock_param = pp_atomctrl_memory_clock_param;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_memory_clock_param_ai {
    pub ulClock: u32,
    pub ulPostDiv: u32,
    pub ulMclk_fcw_frac: u16,
    pub ulMclk_fcw_int: u16,
}

pub type pp_atomctrl_memory_clock_param_ai = pp_atomctrl_memory_clock_param_ai;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_internal_ss_info {
    pub /: *mut *mut uint32_t speed_spectrum_percentage; / in 1/100 percentage,
    pub /: *mut *mut uint32_t speed_spectrum_rate; / in KHz,
    pub speed_spectrum_mode: pp_atomctrl_spread_spectrum_mode,
}

pub type pp_atomctrl_internal_ss_info = pp_atomctrl_internal_ss_info;

pub const NUMBER_OF_M3ARB_PARAMS: c_int = 3;

pub const NUMBER_OF_M3ARB_PARAM_SETS: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_kong_system_info {
    pub /: *mut *mut uint32_t ul_bootup_uma_clock; / in 10kHz unit,
    pub /: *mut *mut uint16_t us_max_nb_voltage; / high NB voltage, calculated using current VDDNB (D24F2xDC) and VDDNB offset fuse;,
    pub /: *mut *mut uint16_t us_min_nb_voltage; / low NB voltage, calculated using current VDDNB (D24F2xDC) and VDDNB offset fuse;,
    pub /: *mut *mut uint16_t us_bootup_nb_voltage; / boot up NB voltage,
    pub /: *mut *mut uint8_t uc_htc_tmp_lmt; / bit [22:16] of D24F3x64 Hardware Thermal Control (HTC) Register, may not be needed, TBD,
    pub /: *mut *mut uint8_t uc_tj_offset; / bit [28:22] of D24F3xE4 Thermtrip Status Register,may not be needed, TBD,
// 0: default 1: uvd 2: fs-3d
    pub /: *mut *mut uint32_t ul_csr_m3_srb_cntl[NUMBER_OF_M3ARB_PARAM_SETS][NUMBER_OF_M3ARB_PARAMS];/ arrays with values for CSR M3 arbiter for default,
}

pub type pp_atomctrl_kong_system_info = pp_atomctrl_kong_system_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_memory_info {
    pub memory_vendor: u8,
    pub memory_type: u8,
}

pub type pp_atomctrl_memory_info = pp_atomctrl_memory_info;
pub const MAX_AC_TIMING_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_memory_clock_range_table {
    pub num_entries: u8,
    pub rsv: [u8; 3],
    pub mclk: [u32; MAX_AC_TIMING_ENTRIES],
}

pub type pp_atomctrl_memory_clock_range_table = pp_atomctrl_memory_clock_range_table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_voltage_table_entry {
    pub value: u16,
    pub smio_low: u32,
}

pub type pp_atomctrl_voltage_table_entry = pp_atomctrl_voltage_table_entry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_voltage_table {
    pub count: u32,
    pub mask_low: u32,
    pub /: *mut *mut uint32_t phase_delay; / Used for ATOM_GPIO_VOLTAGE_OBJECT_V3 and later,
    pub entries: [pp_atomctrl_voltage_table_entry; PP_ATOMCTRL_MAX_VOLTAGE_ENTRIES],
}

pub type pp_atomctrl_voltage_table = pp_atomctrl_voltage_table;
pub const VBIOS_MC_REGISTER_ARRAY_SIZE: c_int = 32;
pub const VBIOS_MAX_AC_TIMING_ENTRIES: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

pub type pp_atomctrl_mc_reg_entry = pp_atomctrl_mc_reg_entry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_mc_register_address {
    pub s1: u16,
    pub uc_pre_reg_data: u8,
}

pub type pp_atomctrl_mc_register_address = pp_atomctrl_mc_register_address;
pub const MAX_SCLK_RANGE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atom_ctrl_sclk_range_table_entry {
    pub ucVco_setting: u8,
    pub ucPostdiv: u8,
    pub usFcw_pcc: u16,
    pub usFcw_trans_upper: u16,
    pub usRcw_trans_lower: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atom_ctrl_sclk_range_table {
    pub entry: [pp_atom_ctrl_sclk_range_table_entry; MAX_SCLK_RANGE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_mc_reg_table {
    pub /: *mut *mut uint8_t last; / number of registers,
    pub /: *mut *mut uint8_t num_entries; / number of AC timing entries,
    pub mc_reg_table_entry: [pp_atomctrl_mc_reg_entry; VBIOS_MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [pp_atomctrl_mc_register_address; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

pub type pp_atomctrl_mc_reg_table = pp_atomctrl_mc_reg_table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atomctrl_gpio_pin_assignment {
    pub us_gpio_pin_aindex: u16,
    pub uc_gpio_pin_bit_shift: u8,
}

pub type pp_atomctrl_gpio_pin_assignment = pp_atomctrl_gpio_pin_assignment;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_atom_ctrl__avfs_parameters {
    pub ulAVFS_meanNsigma_Acontant0: u32,
    pub ulAVFS_meanNsigma_Acontant1: u32,
    pub ulAVFS_meanNsigma_Acontant2: u32,
    pub usAVFS_meanNsigma_DC_tol_sigma: u16,
    pub usAVFS_meanNsigma_Platform_mean: u16,
    pub usAVFS_meanNsigma_Platform_sigma: u16,
    pub ulGB_VDROOP_TABLE_CKSOFF_a0: u32,
    pub ulGB_VDROOP_TABLE_CKSOFF_a1: u32,
    pub ulGB_VDROOP_TABLE_CKSOFF_a2: u32,
    pub ulGB_VDROOP_TABLE_CKSON_a0: u32,
    pub ulGB_VDROOP_TABLE_CKSON_a1: u32,
    pub ulGB_VDROOP_TABLE_CKSON_a2: u32,
    pub ulAVFSGB_FUSE_TABLE_CKSOFF_m1: u32,
    pub usAVFSGB_FUSE_TABLE_CKSOFF_m2: u16,
    pub ulAVFSGB_FUSE_TABLE_CKSOFF_b: u32,
    pub ulAVFSGB_FUSE_TABLE_CKSON_m1: u32,
    pub usAVFSGB_FUSE_TABLE_CKSON_m2: u16,
    pub ulAVFSGB_FUSE_TABLE_CKSON_b: u32,
    pub usMaxVoltage_0_25mv: u16,
    pub ucEnableGB_VDROOP_TABLE_CKSOFF: u8,
    pub ucEnableGB_VDROOP_TABLE_CKSON: u8,
    pub ucEnableGB_FUSE_TABLE_CKSOFF: u8,
    pub ucEnableGB_FUSE_TABLE_CKSON: u8,
    pub usPSM_Age_ComFactor: u16,
    pub ucEnableApplyAVFS_CKS_OFF_Voltage: u8,
    pub ucReserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _AtomCtrl_HiLoLeakageOffsetTable {
    pub usHiLoLeakageThreshold: USHORT,
    pub usEdcDidtLoDpm7TableOffset: USHORT,
    pub usEdcDidtHiDpm7TableOffset: USHORT,
}

pub type AtomCtrl_HiLoLeakageOffsetTable = _AtomCtrl_HiLoLeakageOffsetTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _AtomCtrl_EDCLeakgeTable {
    pub DIDT_REG: [ULONG; 24],
}

pub type AtomCtrl_EDCLeakgeTable = _AtomCtrl_EDCLeakgeTable;
extern "C" {
    pub fn atomctrl_get_pp_assign_pin(hwmgr: *mut pp_hwmgr, pinId: u32, gpio_pin_assignment: *mut pp_atomctrl_gpio_pin_assignment) -> bool;
}
extern "C" {
    pub fn atomctrl_get_voltage_evv_on_sclk(hwmgr: *mut pp_hwmgr, voltage_type: u8, sclk: u32, virtual_voltage_Id: u16, voltage: *mut u16) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_voltage_evv(hwmgr: *mut pp_hwmgr, virtual_voltage_id: u16, voltage: *mut u16) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_mpll_reference_clock(hwmgr: *mut pp_hwmgr) -> u32;
}
extern "C" {
    pub fn atomctrl_is_asic_internal_ss_supported(hwmgr: *mut pp_hwmgr) -> bool;
}
extern "C" {
    pub fn atomctrl_get_memory_clock_spread_spectrum(hwmgr: *mut pp_hwmgr, memory_clock: u32, ssInfo: *mut pp_atomctrl_internal_ss_info) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_engine_clock_spread_spectrum(hwmgr: *mut pp_hwmgr, engine_clock: u32, ssInfo: *mut pp_atomctrl_internal_ss_info) -> c_int;
}
extern "C" {
    pub fn atomctrl_initialize_mc_reg_table(hwmgr: *mut pp_hwmgr, module_index: u8, table: *mut pp_atomctrl_mc_reg_table) -> c_int;
}
extern "C" {
    pub fn atomctrl_initialize_mc_reg_table_v2_2(hwmgr: *mut pp_hwmgr, module_index: u8, table: *mut pp_atomctrl_mc_reg_table) -> c_int;
}
extern "C" {
    pub fn atomctrl_set_engine_dram_timings_rv770(hwmgr: *mut pp_hwmgr, engine_clock: u32, memory_clock: u32) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_reference_clock(hwmgr: *mut pp_hwmgr) -> u32;
}
extern "C" {
    pub fn atomctrl_get_memory_pll_dividers_si(hwmgr: *mut pp_hwmgr, clock_value: u32, mpll_param: *mut pp_atomctrl_memory_clock_param, strobe_mode: bool) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_engine_pll_dividers_vi(hwmgr: *mut pp_hwmgr, clock_value: u32, dividers: *mut pp_atomctrl_clock_dividers_vi) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_dfs_pll_dividers_vi(hwmgr: *mut pp_hwmgr, clock_value: u32, dividers: *mut pp_atomctrl_clock_dividers_vi) -> c_int;
}
extern "C" {
    pub fn atomctrl_is_voltage_controlled_by_gpio_v3(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8) -> bool;
}
extern "C" {
    pub fn atomctrl_get_voltage_table_v3(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8, voltage_table: *mut pp_atomctrl_voltage_table) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_engine_pll_dividers_ai(hwmgr: *mut pp_hwmgr, clock_value: u32, dividers: *mut pp_atomctrl_clock_dividers_ai) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_smc_sclk_range_table(hwmgr: *mut pp_hwmgr, table: *mut pp_atom_ctrl_sclk_range_table) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_avfs_information(hwmgr: *mut pp_hwmgr, param: *mut pp_atom_ctrl__avfs_parameters) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_leakage_id_from_efuse(hwmgr: *mut pp_hwmgr, virtual_voltage_id: *mut u16) -> c_int;
}
extern "C" {
    pub fn atomctrl_get_vddc_shared_railinfo(hwmgr: *mut pp_hwmgr, shared_rail: *mut u8) -> c_int;
}
