//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/hwmgr_ppt.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_clock_voltage_dependency_record {
    pub clk: u32,
    pub vddInd: u8,
    pub vddciInd: u8,
    pub mvddInd: u8,
    pub vdd_offset: u16,
    pub vddc: u16,
    pub vddgfx: u16,
    pub vddci: u16,
    pub mvdd: u16,
    pub phases: u8,
    pub cks_enable: u8,
    pub cks_voffset: u8,
    pub sclk_offset: u32,
}

pub type phm_ppt_v1_clock_voltage_dependency_record = phm_ppt_v1_clock_voltage_dependency_record;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_clock_voltage_dependency_table {
    pub /: *mut *mut uint32_t count; / Number of entries.,
    pub /: *mut *mut phm_ppt_v1_clock_voltage_dependency_record entries[]; / Dynamically allocate count entries.,
}

pub type phm_ppt_v1_clock_voltage_dependency_table = phm_ppt_v1_clock_voltage_dependency_table;
// Multimedia Clock Voltage Dependency records and table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_mm_clock_voltage_dependency_record {
    pub /: *mut *mut uint32_t dclk; / UVD D-clock,
    pub /: *mut *mut uint32_t vclk; / UVD V-clock,
    pub /: *mut *mut uint32_t eclk; / VCE clock,
    pub /: *mut *mut uint32_t aclk; / ACP clock,
    pub /: *mut *mut uint32_t samclock; / SAMU clock,
    pub vddcInd: u8,
    pub vddgfx_offset: u16,
    pub vddc: u16,
    pub vddgfx: u16,
    pub phases: u8,
}

pub type phm_ppt_v1_mm_clock_voltage_dependency_record = phm_ppt_v1_mm_clock_voltage_dependency_record;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_mm_clock_voltage_dependency_table {
    pub /: *mut *mut uint32_t count; / Number of entries.,
    pub /: *mut *mut phm_ppt_v1_mm_clock_voltage_dependency_record entries[]; / Dynamically allocate count entries.,
}

pub type phm_ppt_v1_mm_clock_voltage_dependency_table = phm_ppt_v1_mm_clock_voltage_dependency_table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_voltage_lookup_record {
    pub us_calculated: u16,
    pub /: *mut *mut uint16_t us_vdd; / Base voltage,
    pub us_cac_low: u16,
    pub us_cac_mid: u16,
    pub us_cac_high: u16,
}

pub type phm_ppt_v1_voltage_lookup_record = phm_ppt_v1_voltage_lookup_record;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_voltage_lookup_table {
    pub count: u32,
    pub /: *mut *mut phm_ppt_v1_voltage_lookup_record entries[]; / Dynamically allocate count entries.,
}

pub type phm_ppt_v1_voltage_lookup_table = phm_ppt_v1_voltage_lookup_table;
// PCIE records and Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_pcie_record {
    pub gen_speed: u8,
    pub lane_width: u8,
    pub usreserved: u16,
    pub reserved: u16,
    pub pcie_sclk: u32,
}

pub type phm_ppt_v1_pcie_record = phm_ppt_v1_pcie_record;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_pcie_table {
    pub /: *mut *mut uint32_t count; / Number of entries.,
    pub /: *mut *mut phm_ppt_v1_pcie_record entries[]; / Dynamically allocate count entries.,
}

pub type phm_ppt_v1_pcie_table = phm_ppt_v1_pcie_table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_ppt_v1_gpio_table {
    pub /: *mut *mut uint8_t vrhot_triggered_sclk_dpm_index; / SCLK DPM level index to switch to when VRHot is triggered,
}

pub type phm_ppt_v1_gpio_table = phm_ppt_v1_gpio_table;
