//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv770_dpm.h
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
// Copyright 2011 Advanced Micro Devices, Inc.
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
pub struct rv770_clock_registers {
    pub cg_spll_func_cntl: u32,
    pub cg_spll_func_cntl_2: u32,
    pub cg_spll_func_cntl_3: u32,
    pub cg_spll_spread_spectrum: u32,
    pub cg_spll_spread_spectrum_2: u32,
    pub mpll_ad_func_cntl: u32,
    pub mpll_ad_func_cntl_2: u32,
    pub mpll_dq_func_cntl: u32,
    pub mpll_dq_func_cntl_2: u32,
    pub mclk_pwrmgt_cntl: u32,
    pub dll_cntl: u32,
    pub mpll_ss1: u32,
    pub mpll_ss2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv730_clock_registers {
    pub cg_spll_func_cntl: u32,
    pub cg_spll_func_cntl_2: u32,
    pub cg_spll_func_cntl_3: u32,
    pub cg_spll_spread_spectrum: u32,
    pub cg_spll_spread_spectrum_2: u32,
    pub mclk_pwrmgt_cntl: u32,
    pub dll_cntl: u32,
    pub mpll_func_cntl: u32,
    pub mpll_func_cntl2: u32,
    pub mpll_func_cntl3: u32,
    pub mpll_ss: u32,
    pub mpll_ss2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union r7xx_clock_registers {
    pub rv770: rv770_clock_registers,
    pub rv730: rv730_clock_registers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vddc_table_entry {
    pub vddc: u16,
    pub vddc_index: u8,
    pub high_smio: u8,
    pub low_smio: u32,
}

pub const MAX_NO_OF_MVDD_VALUES: c_int = 2;
pub const MAX_NO_VREG_STEPS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv7xx_power_info {
// flags
    pub mem_gddr5: bool,
    pub pcie_gen2: bool,
    pub dynamic_pcie_gen2: bool,
    pub acpi_pcie_gen2: bool,
    pub boot_in_gen2: bool,
    pub /: *mut *mut bool voltage_control; / vddc,
    pub mvdd_control: bool,
    pub sclk_ss: bool,
    pub mclk_ss: bool,
    pub dynamic_ss: bool,
    pub gfx_clock_gating: bool,
    pub mg_clock_gating: bool,
    pub mgcgtssm: bool,
    pub power_gating: bool,
    pub thermal_protection: bool,
    pub display_gap: bool,
    pub dcodt: bool,
    pub ulps: bool,
// registers
    pub clk_regs: r7xx_clock_registers,
    pub s0_vid_lower_smio_cntl: u32,
// voltage
    pub vddc_mask_low: u32,
    pub mvdd_mask_low: u32,
    pub mvdd_split_frequency: u32,
    pub mvdd_low_smio: [u32; MAX_NO_OF_MVDD_VALUES],
    pub max_vddc: u16,
    pub max_vddc_in_table: u16,
    pub min_vddc_in_table: u16,
    pub vddc_table: [vddc_table_entry; MAX_NO_VREG_STEPS],
    pub valid_vddc_entries: u8,
// dc odt
    pub mclk_odt_threshold: u32,
    pub odt_value_0: [u8; 2],
    pub odt_value_1: [u8; 2],
// stored values
    pub boot_sclk: u32,
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
    pub restricted_levels: u32,
    pub rlp: u32,
    pub rmp: u32,
    pub lhp: u32,
    pub lmp: u32,
// smc offsets
    pub state_table_start: u16,
    pub soft_regs_start: u16,
    pub sram_end: u16,
// scratch structs
    pub smc_statetable: RV770_SMC_STATETABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv7xx_pl {
    pub sclk: u32,
    pub mclk: u32,
    pub vddc: u16,
    pub /: *mut *mut u16 vddci; / eg+ only,
    pub flags: u32,
    pub /: *mut *mut radeon_pcie_gen pcie_gen; / si+ only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv7xx_ps {
    pub high: rv7xx_pl,
    pub medium: rv7xx_pl,
    pub low: rv7xx_pl,
    pub dc_compatible: bool,
}

pub const RV770_RLP_DFLT: c_int = 10;
pub const RV770_RMP_DFLT: c_int = 25;
pub const RV770_LHP_DFLT: c_int = 25;
pub const RV770_LMP_DFLT: c_int = 10;
pub const RV770_VRC_DFLT: c_uint = 0x003f;
pub const RV770_ASI_DFLT: c_int = 1000;
pub const RV770_HASI_DFLT: c_int = 200000;
pub const RV770_MGCGTTLOCAL0_DFLT: c_uint = 0x00100000;
pub const RV7XX_MGCGTTLOCAL0_DFLT: c_int = 0;
pub const RV770_MGCGTTLOCAL1_DFLT: c_uint = 0xFFFF0000;
pub const RV770_MGCGCGTSSMCTRL_DFLT: c_uint = 0x55940000;
pub const MVDD_LOW_INDEX: c_int = 0;
pub const MVDD_HIGH_INDEX: c_int = 1;
pub const MVDD_LOW_VALUE: c_int = 0;
pub const MVDD_HIGH_VALUE: c_uint = 0xffff;

// rv730/rv710
extern "C" {
    pub fn rv730_read_clock_registers(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv730_start_dpm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv730_stop_dpm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv730_program_dcodt(rdev: *mut radeon_device, use_dcodt: bool);
}
extern "C" {
    pub fn rv730_get_odt_values(rdev: *mut radeon_device);
}
// rv740
extern "C" {
    pub fn rv740_read_clock_registers(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv740_get_mclk_frequency_ratio(memory_clock: u32) -> u8;
}
extern "C" {
    pub fn rv740_get_dll_speed(is_gddr5: bool, memory_clock: u32) -> u32;
}
extern "C" {
    pub fn rv740_get_decoded_reference_divider(encoded_ref: u32) -> u32;
}
// rv770
extern "C" {
    pub fn rv770_map_clkf_to_ibias(rdev: *mut radeon_device, clkf: u32) -> u32;
}
extern "C" {
    pub fn rv770_program_response_times(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_read_voltage_smio_registers(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_get_memory_type(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r7xx_start_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_get_memory_module_index(rdev: *mut radeon_device) -> u8;
}
extern "C" {
    pub fn rv770_get_max_vddc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_get_pcie_gen2_status(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_enable_acpi_pm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_restore_cgcg(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_dpm_enabled(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn rv770_setup_bsp(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_program_git(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_program_tp(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_program_tpp(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_program_sstp(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_program_engine_speed_parameters(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_program_vc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_clear_vc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_upload_firmware(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_stop_dpm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r7xx_stop_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_reset_smio_status(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_restrict_performance_levels_before_switch(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_halt_smc(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_resume_smc(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_set_sw_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_set_boot_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv7xx_parse_power_table(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_get_engine_memory_ss(rdev: *mut radeon_device);
}
// smc
