//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu7_discrete.h
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
// Copyright 2013 Advanced Micro Devices, Inc.
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

pub const SMU7_DTE_ITERATIONS: c_int = 5;
pub const SMU7_DTE_SOURCES: c_int = 3;
pub const SMU7_DTE_SINKS: c_int = 1;
pub const SMU7_NUM_CPU_TES: c_int = 0;
pub const SMU7_NUM_GPU_TES: c_int = 1;
pub const SMU7_NUM_NON_TES: c_int = 2;
pub type SMU7_SoftRegisters = SMU7_SoftRegisters;
pub type SMU7_Discrete_VoltageLevel = SMU7_Discrete_VoltageLevel;
pub type SMU7_Discrete_GraphicsLevel = SMU7_Discrete_GraphicsLevel;
pub type SMU7_Discrete_ACPILevel = SMU7_Discrete_ACPILevel;
pub type SMU7_Discrete_Ulv = SMU7_Discrete_Ulv;
pub type SMU7_Discrete_MemoryLevel = SMU7_Discrete_MemoryLevel;
pub type SMU7_Discrete_LinkLevel = SMU7_Discrete_LinkLevel;
pub type SMU7_Discrete_MCArbDramTimingTableEntry = SMU7_Discrete_MCArbDramTimingTableEntry;
pub type SMU7_Discrete_MCArbDramTimingTable = SMU7_Discrete_MCArbDramTimingTable;
pub type SMU7_Discrete_UvdLevel = SMU7_Discrete_UvdLevel;
pub type SMU7_Discrete_ExtClkLevel = SMU7_Discrete_ExtClkLevel;
pub type SMU7_Discrete_StateInfo = SMU7_Discrete_StateInfo;
// SMU7_Discrete_VoltageLevel          VddcStandardReference   [SMU7_MAX_LEVELS_VDDC];
// uint32_t                            SamuDefaultLevel;
pub type SMU7_Discrete_DpmTable = SMU7_Discrete_DpmTable;
pub const SMU7_DISCRETE_MC_REGISTER_ARRAY_SIZE: c_int = 16;

pub type SMU7_Discrete_MCRegisterAddress = SMU7_Discrete_MCRegisterAddress;
pub type SMU7_Discrete_MCRegisterSet = SMU7_Discrete_MCRegisterSet;
pub type SMU7_Discrete_MCRegisters = SMU7_Discrete_MCRegisters;
pub type SMU7_Discrete_FanTable = SMU7_Discrete_FanTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_PmFuses {
// dw0-dw1
    pub BapmVddCVidHiSidd: [u8; 8],
// dw2-dw3
    pub BapmVddCVidLoSidd: [u8; 8],
// dw4-dw5
    pub VddCVid: [u8; 8],
// dw6
    pub SviLoadLineEn: u8,
    pub SviLoadLineVddC: u8,
    pub SviLoadLineTrimVddC: u8,
    pub SviLoadLineOffsetVddC: u8,
// dw7
    pub TDC_VDDC_PkgLimit: u16,
    pub TDC_VDDC_ThrottleReleaseLimitPerc: u8,
    pub TDC_MAWt: u8,
// dw8
    pub TdcWaterfallCtl: u8,
    pub LPMLTemperatureMin: u8,
    pub LPMLTemperatureMax: u8,
    pub Reserved: u8,
// dw9-dw10
    pub BapmVddCVidHiSidd2: [u8; 8],
// dw11-dw12
    pub FuzzyFan_ErrorSetDelta: i16,
    pub FuzzyFan_ErrorRateSetDelta: i16,
    pub FuzzyFan_PwmSetDelta: i16,
    pub CalcMeasPowerBlend: u16,
// dw13-dw16
    pub GnbLPML: [u8; 16],
// dw17
    pub GnbLPMLMaxVid: u8,
    pub GnbLPMLMinVid: u8,
    pub Reserved1: [u8; 2],
// dw18
    pub BapmVddCBaseLeakageHiSidd: u16,
    pub BapmVddCBaseLeakageLoSidd: u16,
}

pub type SMU7_Discrete_PmFuses = SMU7_Discrete_PmFuses;

