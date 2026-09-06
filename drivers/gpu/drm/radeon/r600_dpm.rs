//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r600_dpm.h
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

pub const R600_ASI_DFLT: c_int = 10000;
pub const R600_BSP_DFLT: c_uint = 0x41EB;
pub const R600_BSU_DFLT: c_uint = 0x2;
pub const R600_AH_DFLT: c_int = 5;
pub const R600_RLP_DFLT: c_int = 25;
pub const R600_RMP_DFLT: c_int = 65;
pub const R600_LHP_DFLT: c_int = 40;
pub const R600_LMP_DFLT: c_int = 15;
pub const R600_TD_DFLT: c_int = 0;
pub const R600_UTC_DFLT_00: c_uint = 0x24;
pub const R600_UTC_DFLT_01: c_uint = 0x22;
pub const R600_UTC_DFLT_02: c_uint = 0x22;
pub const R600_UTC_DFLT_03: c_uint = 0x22;
pub const R600_UTC_DFLT_04: c_uint = 0x22;
pub const R600_UTC_DFLT_05: c_uint = 0x22;
pub const R600_UTC_DFLT_06: c_uint = 0x22;
pub const R600_UTC_DFLT_07: c_uint = 0x22;
pub const R600_UTC_DFLT_08: c_uint = 0x22;
pub const R600_UTC_DFLT_09: c_uint = 0x22;
pub const R600_UTC_DFLT_10: c_uint = 0x22;
pub const R600_UTC_DFLT_11: c_uint = 0x22;
pub const R600_UTC_DFLT_12: c_uint = 0x22;
pub const R600_UTC_DFLT_13: c_uint = 0x22;
pub const R600_UTC_DFLT_14: c_uint = 0x22;
pub const R600_DTC_DFLT_00: c_uint = 0x24;
pub const R600_DTC_DFLT_01: c_uint = 0x22;
pub const R600_DTC_DFLT_02: c_uint = 0x22;
pub const R600_DTC_DFLT_03: c_uint = 0x22;
pub const R600_DTC_DFLT_04: c_uint = 0x22;
pub const R600_DTC_DFLT_05: c_uint = 0x22;
pub const R600_DTC_DFLT_06: c_uint = 0x22;
pub const R600_DTC_DFLT_07: c_uint = 0x22;
pub const R600_DTC_DFLT_08: c_uint = 0x22;
pub const R600_DTC_DFLT_09: c_uint = 0x22;
pub const R600_DTC_DFLT_10: c_uint = 0x22;
pub const R600_DTC_DFLT_11: c_uint = 0x22;
pub const R600_DTC_DFLT_12: c_uint = 0x22;
pub const R600_DTC_DFLT_13: c_uint = 0x22;
pub const R600_DTC_DFLT_14: c_uint = 0x22;
pub const R600_VRC_DFLT: c_uint = 0x0000C003;
pub const R600_VOLTAGERESPONSETIME_DFLT: c_int = 1000;
pub const R600_BACKBIASRESPONSETIME_DFLT: c_int = 1000;
pub const R600_VRU_DFLT: c_uint = 0x3;
pub const R600_SPLLSTEPTIME_DFLT: c_uint = 0x1000;
pub const R600_SPLLSTEPUNIT_DFLT: c_uint = 0x3;
pub const R600_TPU_DFLT: c_int = 0;
pub const R600_TPC_DFLT: c_uint = 0x200;
pub const R600_SSTU_DFLT: c_int = 0;
pub const R600_SST_DFLT: c_uint = 0x00C8;
pub const R600_GICST_DFLT: c_uint = 0x200;
pub const R600_FCT_DFLT: c_uint = 0x0400;
pub const R600_FCTU_DFLT: c_int = 0;
pub const R600_CTXCGTT3DRPHC_DFLT: c_uint = 0x20;
pub const R600_CTXCGTT3DRSDC_DFLT: c_uint = 0x40;
pub const R600_VDDC3DOORPHC_DFLT: c_uint = 0x100;
pub const R600_VDDC3DOORSDC_DFLT: c_uint = 0x7;
pub const R600_VDDC3DOORSU_DFLT: c_int = 0;
pub const R600_MPLLLOCKTIME_DFLT: c_int = 100;
pub const R600_MPLLRESETTIME_DFLT: c_int = 150;
pub const R600_VCOSTEPPCT_DFLT: c_int = 20;
pub const R600_ENDINGVCOSTEPPCT_DFLT: c_int = 5;
pub const R600_REFERENCEDIVIDER_DFLT: c_int = 4;
pub const R600_PM_NUMBER_OF_TC: c_int = 15;
pub const R600_PM_NUMBER_OF_SCLKS: c_int = 20;
pub const R600_PM_NUMBER_OF_MCLKS: c_int = 4;
pub const R600_PM_NUMBER_OF_VOLTAGE_LEVELS: c_int = 4;
pub const R600_PM_NUMBER_OF_ACTIVITY_LEVELS: c_int = 3;
// XXX are these ok?

pub const FDO_PWM_MODE_STATIC: c_int = 1;
pub const FDO_PWM_MODE_STATIC_RPM: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_power_level {
    R600_POWER_LEVEL_LOW = 0,
    R600_POWER_LEVEL_MEDIUM = 1,
    R600_POWER_LEVEL_HIGH = 2,
    R600_POWER_LEVEL_CTXSW = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_td {
    R600_TD_AUTO,
    R600_TD_UP,
    R600_TD_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_display_watermark {
    R600_DISPLAY_WATERMARK_LOW = 0,
    R600_DISPLAY_WATERMARK_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_display_gap {
    R600_PM_DISPLAY_GAP_VBLANK_OR_WM = 0,
    R600_PM_DISPLAY_GAP_VBLANK       = 1,
    R600_PM_DISPLAY_GAP_WATERMARK    = 2,
    R600_PM_DISPLAY_GAP_IGNORE       = 3,
}

extern "C" {
    pub fn r600_dpm_print_class_info(class: u32, class2: u32);
}
extern "C" {
    pub fn r600_dpm_print_cap_info(caps: u32);
}
extern "C" {
    pub fn r600_dpm_get_vblank_time(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn r600_dpm_get_vrefresh(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn r600_is_uvd_state(class: u32, class2: u32) -> bool;
}
extern "C" {
    pub fn r600_calculate_at(t: u32, h: u32, fh: u32, fl: u32, tl: *mut u32, th: *mut u32) -> c_int;
}
extern "C" {
    pub fn r600_gfx_clockgating_enable(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn r600_dynamicpm_enable(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn r600_enable_thermal_protection(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn r600_enable_acpi_pm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_enable_dynamic_pcie_gen2(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn r600_dynamicpm_enabled(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn r600_enable_sclk_control(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn r600_enable_mclk_control(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn r600_enable_spll_bypass(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn r600_wait_for_spll_change(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_set_bsp(rdev: *mut radeon_device, u: u32, p: u32);
}
extern "C" {
    pub fn r600_set_tc(rdev: *mut radeon_device, index: u32, u_t: u32, d_t: u32);
}
extern "C" {
    pub fn r600_select_td(rdev: *mut radeon_device, td: r600_td);
}
extern "C" {
    pub fn r600_set_vrc(rdev: *mut radeon_device, vrv: u32);
}
extern "C" {
    pub fn r600_set_tpu(rdev: *mut radeon_device, u: u32);
}
extern "C" {
    pub fn r600_set_tpc(rdev: *mut radeon_device, c: u32);
}
extern "C" {
    pub fn r600_set_sstu(rdev: *mut radeon_device, u: u32);
}
extern "C" {
    pub fn r600_set_sst(rdev: *mut radeon_device, t: u32);
}
extern "C" {
    pub fn r600_set_git(rdev: *mut radeon_device, t: u32);
}
extern "C" {
    pub fn r600_set_fctu(rdev: *mut radeon_device, u: u32);
}
extern "C" {
    pub fn r600_set_fct(rdev: *mut radeon_device, t: u32);
}
extern "C" {
    pub fn r600_set_ctxcgtt3d_rphc(rdev: *mut radeon_device, p: u32);
}
extern "C" {
    pub fn r600_set_ctxcgtt3d_rsdc(rdev: *mut radeon_device, s: u32);
}
extern "C" {
    pub fn r600_set_vddc3d_oorsu(rdev: *mut radeon_device, u: u32);
}
extern "C" {
    pub fn r600_set_vddc3d_oorphc(rdev: *mut radeon_device, p: u32);
}
extern "C" {
    pub fn r600_set_vddc3d_oorsdc(rdev: *mut radeon_device, s: u32);
}
extern "C" {
    pub fn r600_set_mpll_lock_time(rdev: *mut radeon_device, lock_time: u32);
}
extern "C" {
    pub fn r600_set_mpll_reset_time(rdev: *mut radeon_device, reset_time: u32);
}
extern "C" {
    pub fn r600_vid_rt_set_ssu(rdev: *mut radeon_device, u: u32);
}
extern "C" {
    pub fn r600_vid_rt_set_vru(rdev: *mut radeon_device, u: u32);
}
extern "C" {
    pub fn r600_vid_rt_set_vrt(rdev: *mut radeon_device, rt: u32);
}
extern "C" {
    pub fn r600_power_level_get_current_index(rdev: *mut radeon_device) -> r600_power_level;
}
extern "C" {
    pub fn r600_power_level_get_target_index(rdev: *mut radeon_device) -> r600_power_level;
}
extern "C" {
    pub fn r600_start_dpm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_stop_dpm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_is_internal_thermal_sensor(sensor: radeon_int_thermal_type) -> bool;
}
extern "C" {
    pub fn r600_get_platform_caps(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_parse_extended_power_table(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_free_extended_power_table(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_encode_pci_lane_width(lanes: u32) -> u8;
}
