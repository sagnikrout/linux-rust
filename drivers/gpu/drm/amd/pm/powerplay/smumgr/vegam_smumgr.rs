//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/smumgr/vegam_smumgr.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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

pub const SMC_RAM_END: c_uint = 0x40000;
pub const DPMTuning_Uphyst_Shift: c_int = 0;
pub const DPMTuning_Downhyst_Shift: c_int = 8;
pub const DPMTuning_Activity_Shift: c_int = 16;
pub const GraphicsDPMTuning_VEGAM: c_uint = 0x001e6400;
pub const MemoryDPMTuning_VEGAM: c_uint = 0x000f3c0a;
pub const SclkDPMTuning_VEGAM: c_uint = 0x002d000a;
pub const MclkDPMTuning_VEGAM: c_uint = 0x001f100a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vegam_pt_defaults {
    pub SviLoadLineEn: u8,
    pub SviLoadLineVddC: u8,
    pub TDC_VDDC_ThrottleReleaseLimitPerc: u8,
    pub TDC_MAWt: u8,
    pub TdcWaterfallCtl: u8,
    pub DTEAmbientTempBase: u8,
    pub DisplayCac: u32,
    pub BAPM_TEMP_GRADIENT: u32,
    pub SMU75_DTE_SINKS]: *mut *mut *mut uint16_t BAPMTI_R[SMU75_DTE_ITERATIONS  SMU75_DTE_SOURCES,
    pub SMU75_DTE_SINKS]: *mut *mut *mut uint16_t BAPMTI_RC[SMU75_DTE_ITERATIONS  SMU75_DTE_SOURCES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vegam_range_table {
    pub /: *mut *mut uint32_t trans_lower_frequency; / in 10khz,
    pub trans_upper_frequency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vegam_smumgr {
    pub smu7_data: smu7_smumgr,
    pub protected_mode: u8,
    pub smc_state_table: SMU75_Discrete_DpmTable,
    pub ulv_setting: SMU75_Discrete_Ulv,
    pub power_tune_table: SMU75_Discrete_PmFuses,
    pub range_table: [vegam_range_table; NUM_SCLK_RANGE],
    pub power_tune_defaults: *const vegam_pt_defaults,
    pub bif_sclk_table: [u32; SMU75_MAX_LEVELS_LINK],
}
