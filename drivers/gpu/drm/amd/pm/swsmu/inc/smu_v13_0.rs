//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/smu_v13_0.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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

// MP Apertures
pub const MP0_Public: c_uint = 0x03800000;
pub const MP0_SRAM: c_uint = 0x03900000;
pub const MP1_Public: c_uint = 0x03b00000;
pub const MP1_SRAM: c_uint = 0x03c00004;
// address block
pub const smnMP1_FIRMWARE_FLAGS: c_uint = 0x3010024;
pub const smnMP1_V13_0_4_FIRMWARE_FLAGS: c_uint = 0x3010028;
pub const smnMP0_FW_INTF: c_uint = 0x30101c0;
pub const smnMP1_PUB_CTRL: c_uint = 0x3010b14;

pub const SMU13_TOOL_SIZE: c_uint = 0x19000;
pub const MAX_PCIE_CONF: c_int = 3;
pub const CTF_OFFSET_EDGE: c_int = 5;
pub const CTF_OFFSET_HOTSPOT: c_int = 5;
pub const CTF_OFFSET_MEM: c_int = 5;
pub const SMU_13_VCLK_SHIFT: c_int = 16;

// Convert Q10 watts to milliwatts, preserving the fractional part

// Convert Q10 degrees Celsius to millidegrees, preserving the fractional part

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_13_0_max_sustainable_clocks {
    pub display_clock: u32,
    pub phy_clock: u32,
    pub pixel_clock: u32,
    pub uclock: u32,
    pub dcef_clock: u32,
    pub soc_clock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_13_0_dpm_tables {
    pub soc_table: smu_dpm_table,
    pub gfx_table: smu_dpm_table,
    pub uclk_table: smu_dpm_table,
    pub eclk_table: smu_dpm_table,
    pub vclk_table: smu_dpm_table,
    pub dclk_table: smu_dpm_table,
    pub dcef_table: smu_dpm_table,
    pub pixel_table: smu_dpm_table,
    pub display_table: smu_dpm_table,
    pub phy_table: smu_dpm_table,
    pub fclk_table: smu_dpm_table,
    pub pcie_table: smu_pcie_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_13_0_dpm_context {
    pub dpm_tables: smu_13_0_dpm_tables,
    pub workload_policy_mask: u32,
    pub dcef_min_ds_clk: u32,
    pub caps: u64,
    pub board_volt: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_13_0_power_state {
    SMU_13_0_POWER_STATE__D0 = 0,
    SMU_13_0_POWER_STATE__D1,
    SMU_13_0_POWER_STATE__D3, /* Sleep*/
    SMU_13_0_POWER_STATE__D4, /* Hibernate*/
    SMU_13_0_POWER_STATE__D5, /* Power off*/
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_13_0_power_context {
    pub power_source: u32,
    pub in_power_limit_boost_mode: u8,
    pub power_state: smu_13_0_power_state,
    pub throttle_status: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn smu_v13_0_init_microcode(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_fini_microcode(smu: *mut smu_context);
}
extern "C" {
    pub fn smu_v13_0_load_microcode(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_init_smc_tables(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_fini_smc_tables(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_init_power(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_fini_power(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_check_fw_status(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_setup_pptable(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_get_vbios_bootup_values(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_set_driver_table_location(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_set_tool_table_location(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_notify_memory_pool_location(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_set_allowed_mask(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_notify_display_change(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_init_max_sustainable_clocks(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_enable_thermal_alert(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_disable_thermal_alert(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_get_gfx_vdd(smu: *mut smu_context, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_gfx_off_control(smu: *mut smu_context, enable: bool) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_register_irq_handler(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_get_bamaco_support(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_baco_enter(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_baco_exit(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_get_current_pcie_link_width_level(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_get_current_pcie_link_width(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_get_current_pcie_link_speed_level(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_get_current_pcie_link_speed(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_init_pptable_microcode(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_run_btc(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_set_gfx_power_up_by_imu(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_set_default_dpm_tables(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_mode1_reset(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_disable_pmfw_state(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_enable_uclk_shadow(smu: *mut smu_context, enable: bool) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_interrupt_work(smu: *mut smu_context);
}
extern "C" {
    pub fn smu_v13_0_reset_custom_level(smu: *mut smu_context);
}

