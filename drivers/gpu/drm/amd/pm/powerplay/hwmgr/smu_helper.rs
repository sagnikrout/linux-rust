//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/smu_helper.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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
extern "C" {
    pub fn convert_to_vid(vddc: u16) -> u8;
}
extern "C" {
    pub fn convert_to_vddc(vid: u8) -> u16;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watermark_row_generic_t {
    pub MinClock: u16,
    pub MaxClock: u16,
    pub MinUclk: u16,
    pub MaxUclk: u16,
    pub WmSetting: u8,
    pub Padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watermarks {
    pub WatermarkRow: [watermark_row_generic_t; 2][4],
    pub padding: [u32; 7],
}

extern "C" {
    pub fn phm_cf_want_uvd_power_gating(hwmgr: *mut pp_hwmgr) -> bool;
}
extern "C" {
    pub fn phm_cf_want_vce_power_gating(hwmgr: *mut pp_hwmgr) -> bool;
}
extern "C" {
    pub fn phm_cf_want_microcode_fan_ctrl(hwmgr: *mut pp_hwmgr) -> bool;
}
extern "C" {
    pub fn phm_trim_voltage_table(vol_table: *mut pp_atomctrl_voltage_table) -> c_int;
}
extern "C" {
    pub fn phm_get_svi2_mvdd_voltage_table(vol_table: *mut pp_atomctrl_voltage_table, dep_table: *mut phm_ppt_v1_clock_voltage_dependency_table) -> c_int;
}
extern "C" {
    pub fn phm_get_svi2_vddci_voltage_table(vol_table: *mut pp_atomctrl_voltage_table, dep_table: *mut phm_ppt_v1_clock_voltage_dependency_table) -> c_int;
}
extern "C" {
    pub fn phm_get_svi2_vdd_voltage_table(vol_table: *mut pp_atomctrl_voltage_table, lookup_table: *mut phm_ppt_v1_voltage_lookup_table) -> c_int;
}
extern "C" {
    pub fn phm_trim_voltage_table_to_fit_state_table(max_vol_steps: u32, vol_table: *mut pp_atomctrl_voltage_table);
}
extern "C" {
    pub fn phm_reset_single_dpm_table(table: *mut c_void, count: u32, max: c_int) -> c_int;
}
extern "C" {
    pub fn phm_setup_pcie_table_entry(table: *mut c_void, index: u32, pcie_gen: u32, pcie_lanes: u32);
}
extern "C" {
    pub fn phm_get_dpm_level_enable_mask_value(table: *mut c_void) -> i32;
}
extern "C" {
    pub fn phm_get_voltage_index(lookup_table: *mut phm_ppt_v1_voltage_lookup_table, voltage: u16) -> u8;
}
extern "C" {
    pub fn phm_find_closest_vddci(vddci_table: *mut pp_atomctrl_voltage_table, vddci: u16) -> u16;
}
extern "C" {
    pub fn phm_find_boot_level(table: *mut c_void, value: u32, boot_level: *mut u32) -> c_int;
}
extern "C" {
    pub fn phm_get_lowest_enabled_level(hwmgr: *mut pp_hwmgr, mask: u32) -> u32;
}
extern "C" {
    pub fn phm_set_field_to_u32(offset: u32, original_data: u32, field: u32, size: u32) -> u32;
}
//
// Helper function to make sysfs_emit_at() happy. Align buf to
// the current page boundary and record the offset.
//
// offset = offset_in_page(*buf);
// buf -= *offset;
extern "C" {
    pub fn smu9_register_irq_handlers(hwmgr: *mut pp_hwmgr) -> c_int;
}

// Operations on named fields.

