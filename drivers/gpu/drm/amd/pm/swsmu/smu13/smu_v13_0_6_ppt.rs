//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/smu13/smu_v13_0_6_ppt.h
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
// Copyright 2021 Advanced Micro Devices, Inc.
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
pub const SMU_13_0_6_UMD_PSTATE_GFXCLK_LEVEL: c_uint = 0x2;
pub const SMU_13_0_6_UMD_PSTATE_SOCCLK_LEVEL: c_uint = 0x4;
pub const SMU_13_0_6_UMD_PSTATE_MCLK_LEVEL: c_uint = 0x2;

// 0*/   METRICS_VERSION_V0                  = 0,
// 1*/   METRICS_VERSION_V1                  = 1,
// 2*/   METRICS_VERSION_V2                  = 2,
// 3*/   NUM_METRICS                         = 3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PPTable_t {
    pub MaxSocketPowerLimit: u32,
    pub MaxGfxclkFrequency: u32,
    pub MinGfxclkFrequency: u32,
    pub FclkFrequencyTable: [u32; 4],
    pub UclkFrequencyTable: [u32; 4],
    pub SocclkFrequencyTable: [u32; 4],
    pub VclkFrequencyTable: [u32; 4],
    pub DclkFrequencyTable: [u32; 4],
    pub LclkFrequencyTable: [u32; 4],
    pub MaxLclkDpmRange: u32,
    pub MinLclkDpmRange: u32,
    pub PublicSerialNumber_AID: u64,
    pub MaxNodePowerLimit: u32,
    pub PPT1Max: u32,
    pub PPT1Min: u32,
    pub PPT1Default: u32,
    pub Init: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_v13_0_6_caps {
    SMU_CAP(DPM),
    SMU_CAP(DPM_POLICY),
    SMU_CAP(OTHER_END_METRICS),
    SMU_CAP(SET_UCLK_MAX),
    SMU_CAP(PCIE_METRICS),
    SMU_CAP(MCA_DEBUG_MODE),
    SMU_CAP(PER_INST_METRICS),
    SMU_CAP(CTF_LIMIT),
    SMU_CAP(RMA_MSG),
    SMU_CAP(ACA_SYND),
    SMU_CAP(SDMA_RESET),
    SMU_CAP(VCN_RESET),
    SMU_CAP(STATIC_METRICS),
    SMU_CAP(HST_LIMIT_METRICS),
    SMU_CAP(BOARD_VOLTAGE),
    SMU_CAP(PLDM_VERSION),
    SMU_CAP(TEMP_METRICS),
    SMU_CAP(NPM_METRICS),
    SMU_CAP(RAS_EEPROM),
    SMU_CAP(FAST_PPT),
    SMU_CAP(SYSTEM_POWER_METRICS),
    SMU_CAP(TEMP_AID_XCD_HBM),
    SMU_CAP(ALL),
}

pub const SMU_13_0_6_NUM_XGMI_LINKS: c_int = 8;
pub const SMU_13_0_6_MAX_GFX_CLKS: c_int = 8;
pub const SMU_13_0_6_MAX_CLKS: c_int = 4;
pub const SMU_13_0_6_MAX_XCC: c_int = 8;
pub const SMU_13_0_6_MAX_VCN: c_int = 4;
pub const SMU_13_0_6_MAX_JPEG: c_int = 40;
pub const SMU_13_0_6_MAX_AID: c_int = 4;
pub const SMU_13_0_6_MAX_HBM_STACKS: c_int = 8;
extern "C" {
    pub fn smu_v13_0_6_set_ppt_funcs(smu: *mut smu_context);
}
extern "C" {
    pub fn smu_v13_0_6_cap_supported(smu: *mut smu_context, cap: smu_v13_0_6_caps) -> bool;
}
extern "C" {
    pub fn smu_v13_0_6_get_static_metrics_table(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_12_is_dpm_running(smu: *mut smu_context) -> bool;
}
extern "C" {
    pub fn smu_v13_0_12_get_max_metrics_size() -> c_int;
}
extern "C" {
    pub fn smu_v13_0_12_get_system_metrics_size() -> usize;
}
extern "C" {
    pub fn smu_v13_0_12_setup_driver_pptable(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_12_tables_init(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_v13_0_12_tables_fini(smu: *mut smu_context);
}

// SMUv 13.0.6 GPU metrics

