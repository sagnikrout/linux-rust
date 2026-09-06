//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/power_state.h
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
pub struct pp_hw_power_state {
    pub magic: c_uint,
}

//
// An item of a list containing Power States.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_StateLinkedList {
    pub next: *mut pp_power_state,
    pub prev: *mut pp_power_state,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_StateUILabel {
    PP_StateUILabel_None,
    PP_StateUILabel_Battery,
    PP_StateUILabel_MiddleLow,
    PP_StateUILabel_Balanced,
    PP_StateUILabel_MiddleHigh,
    PP_StateUILabel_Performance,
    PP_StateUILabel_BACO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_StateClassificationFlag {
    PP_StateClassificationFlag_Boot                = 0x0001,
    PP_StateClassificationFlag_Thermal             = 0x0002,
    PP_StateClassificationFlag_LimitedPowerSource  = 0x0004,
    PP_StateClassificationFlag_Rest                = 0x0008,
    PP_StateClassificationFlag_Forced              = 0x0010,
    PP_StateClassificationFlag_User3DPerformance   = 0x0020,
    PP_StateClassificationFlag_User2DPerformance   = 0x0040,
    PP_StateClassificationFlag_3DPerformance       = 0x0080,
    PP_StateClassificationFlag_ACOverdriveTemplate   = 0x0100,
    PP_StateClassificationFlag_Uvd                 = 0x0200,
    PP_StateClassificationFlag_3DPerformanceLow    = 0x0400,
    PP_StateClassificationFlag_ACPI                = 0x0800,
    PP_StateClassificationFlag_HD2                 = 0x1000,
    PP_StateClassificationFlag_UvdHD               = 0x2000,
    PP_StateClassificationFlag_UvdSD               = 0x4000,
    PP_StateClassificationFlag_UserDCPerformance    = 0x8000,
    PP_StateClassificationFlag_DCOverdriveTemplate   = 0x10000,
    PP_StateClassificationFlag_BACO                  = 0x20000,
    PP_StateClassificationFlag_LimitedPowerSource_2  = 0x40000,
    PP_StateClassificationFlag_ULV                   = 0x80000,
    PP_StateClassificationFlag_UvdMVC               = 0x100000,
}

pub type PP_StateClassificationFlags = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_StateClassificationBlock {
    pub ui_label: PP_StateUILabel,
    pub flags: PP_StateClassificationFlag,
    pub bios_index: c_int,
    pub temporary_state: bool,
    pub to_be_deleted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_StatePcieBlock {
    pub lanes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_RefreshrateSource {
    PP_RefreshrateSource_EDID,
    PP_RefreshrateSource_Explicit
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_StateDisplayBlock {
    pub disableFrameModulation: bool,
    pub limitRefreshrate: bool,
    pub refreshrateSource: PP_RefreshrateSource,
    pub explicitRefreshrate: c_int,
    pub edidRefreshrateIndex: c_int,
    pub enableVariBright: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_StateMemroyBlock {
    pub dllOff: bool,
    pub m3arb: u8,
    pub unused: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_StateSoftwareAlgorithmBlock {
    pub disableLoadBalancing: bool,
    pub enableSleepForTimestamps: bool,
}

pub const PP_TEMPERATURE_UNITS_PER_CENTIGRADES: c_int = 1000;
//
// Type to hold a temperature range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_TemperatureRange {
    pub min: c_int,
    pub max: c_int,
    pub edge_emergency_max: c_int,
    pub hotspot_min: c_int,
    pub hotspot_crit_max: c_int,
    pub hotspot_emergency_max: c_int,
    pub mem_min: c_int,
    pub mem_crit_max: c_int,
    pub mem_emergency_max: c_int,
    pub sw_ctf_threshold: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_StateValidationBlock {
    pub singleDisplayOnly: bool,
    pub disallowOnDC: bool,
    pub supportedPowerLevels: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_UVD_CLOCKS {
    pub VCLK: u32,
    pub DCLK: u32,
}

//
// Structure to hold a PowerPlay Power State.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_power_state {
    pub id: u32,
    pub orderedList: PP_StateLinkedList,
    pub allStatesList: PP_StateLinkedList,
    pub classification: PP_StateClassificationBlock,
    pub validation: PP_StateValidationBlock,
    pub pcie: PP_StatePcieBlock,
    pub display: PP_StateDisplayBlock,
    pub memory: PP_StateMemroyBlock,
    pub temperatures: PP_TemperatureRange,
    pub software: PP_StateSoftwareAlgorithmBlock,
    pub uvd_clocks: PP_UVD_CLOCKS,
    pub hardware: pp_hw_power_state,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_MMProfilingState {
    PP_MMProfilingState_NA = 0,
    PP_MMProfilingState_Started,
    PP_MMProfilingState_Stopped
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_clock_engine_request {
    pub client_type: c_ulong,
    pub ctx_id: c_ulong,
    pub context_handle: u64,
    pub sclk: c_ulong,
    pub sclk_hard_min: c_ulong,
    pub mclk: c_ulong,
    pub iclk: c_ulong,
    pub evclk: c_ulong,
    pub ecclk: c_ulong,
    pub ecclk_hard_min: c_ulong,
    pub vclk: c_ulong,
    pub dclk: c_ulong,
    pub sclk_over_drive: c_ulong,
    pub mclk_over_drive: c_ulong,
    pub sclk_threshold: c_ulong,
    pub flag: c_ulong,
    pub vclk_ceiling: c_ulong,
    pub dclk_ceiling: c_ulong,
    pub num_cus: c_ulong,
    pub pm_flag: c_ulong,
    pub mm_profiling_state: PP_MMProfilingState,
}
