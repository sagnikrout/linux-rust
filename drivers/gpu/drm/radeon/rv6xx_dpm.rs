//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv6xx_dpm.h
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
// Authors: Alex Deucher
//

// Represents a single SCLK step.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv6xx_sclk_stepping {
    pub vco_frequency: u32,
    pub post_divider: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv6xx_pm_hw_state {
    pub sclks: [u32; R600_PM_NUMBER_OF_ACTIVITY_LEVELS],
    pub mclks: [u32; R600_PM_NUMBER_OF_MCLKS],
    pub vddc: [u16; R600_PM_NUMBER_OF_VOLTAGE_LEVELS],
    pub backbias: [bool; R600_PM_NUMBER_OF_VOLTAGE_LEVELS],
    pub pcie_gen2: [bool; R600_PM_NUMBER_OF_ACTIVITY_LEVELS],
    pub high_sclk_index: u8,
    pub medium_sclk_index: u8,
    pub low_sclk_index: u8,
    pub high_mclk_index: u8,
    pub medium_mclk_index: u8,
    pub low_mclk_index: u8,
    pub high_vddc_index: u8,
    pub medium_vddc_index: u8,
    pub low_vddc_index: u8,
    pub rp: [u8; R600_PM_NUMBER_OF_ACTIVITY_LEVELS],
    pub lp: [u8; R600_PM_NUMBER_OF_ACTIVITY_LEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv6xx_power_info {
// flags
    pub voltage_control: bool,
    pub sclk_ss: bool,
    pub mclk_ss: bool,
    pub dynamic_ss: bool,
    pub dynamic_pcie_gen2: bool,
    pub thermal_protection: bool,
    pub display_gap: bool,
    pub gfx_clock_gating: bool,
// clk values
    pub fb_div_scale: u32,
    pub spll_ref_div: u32,
    pub mpll_ref_div: u32,
    pub bsu: u32,
    pub bsp: u32,
//
    pub active_auto_throttle_sources: u32,
// current power state
    pub restricted_levels: u32,
    pub hw: rv6xx_pm_hw_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv6xx_pl {
    pub sclk: u32,
    pub mclk: u32,
    pub vddc: u16,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv6xx_ps {
    pub high: rv6xx_pl,
    pub medium: rv6xx_pl,
    pub low: rv6xx_pl,
}

