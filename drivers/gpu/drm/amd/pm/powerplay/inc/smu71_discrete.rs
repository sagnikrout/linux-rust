//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu71_discrete.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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

pub const VDDC_ON_SVI2: c_uint = 0x1;
pub const VDDCI_ON_SVI2: c_uint = 0x2;
pub const MVDD_ON_SVI2: c_uint = 0x4;
pub type SMU71_Discrete_VoltageLevel = SMU71_Discrete_VoltageLevel;
pub type SMU71_Discrete_GraphicsLevel = SMU71_Discrete_GraphicsLevel;
pub type SMU71_Discrete_ACPILevel = SMU71_Discrete_ACPILevel;
pub type SMU71_Discrete_Ulv = SMU71_Discrete_Ulv;
pub type SMU71_Discrete_MemoryLevel = SMU71_Discrete_MemoryLevel;
pub type SMU71_Discrete_LinkLevel = SMU71_Discrete_LinkLevel;

// MC ARB DRAM Timing registers.
pub type SMU71_Discrete_MCArbDramTimingTableEntry = SMU71_Discrete_MCArbDramTimingTableEntry;
pub type SMU71_Discrete_MCArbDramTimingTable = SMU71_Discrete_MCArbDramTimingTable;

// UVD VCLK/DCLK state (level) definition.
pub type SMU71_Discrete_UvdLevel = SMU71_Discrete_UvdLevel;
// Clocks for other external blocks (VCE, ACP, SAMU).
pub type SMU71_Discrete_ExtClkLevel = SMU71_Discrete_ExtClkLevel;
// Everything that we need to keep track of about the current state.
// Use this instead of copies of the GraphicsLevel and MemoryLevel structures to keep track of state parameters
// that need to be checked later.
// We don't need to cache everything about a state, just a few parameters.
pub type SMU71_Discrete_StateInfo = SMU71_Discrete_StateInfo;
// Multi-DPM controller settings
// SMIO masks for voltage and phase controls
// State table entries for each DPM state
pub type SMU71_Discrete_DpmTable = SMU71_Discrete_DpmTable;
// --------------------------------------------------- AC Timing Parameters ------------------------------------------------
pub const SMU71_DISCRETE_MC_REGISTER_ARRAY_SIZE: c_int = 16;

pub type SMU71_Discrete_MCRegisterAddress = SMU71_Discrete_MCRegisterAddress;
pub type SMU71_Discrete_MCRegisterSet = SMU71_Discrete_MCRegisterSet;
pub type SMU71_Discrete_MCRegisters = SMU71_Discrete_MCRegisters;
// --------------------------------------------------- Fan Table -----------------------------------------------------------
pub type SMU71_Discrete_FanTable = SMU71_Discrete_FanTable;
pub const SMU7_DISCRETE_GPIO_SCLK_DEBUG: c_int = 4;

pub type SMU71_MclkDpmScoreboard = SMU71_MclkDpmScoreboard;
pub type SMU71_UlvScoreboard = SMU71_UlvScoreboard;
pub type SMU71_VddGfxScoreboard = SMU71_VddGfxScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_AcpiScoreboard {
    pub SavedInterruptMask: [u32; 2],
    pub LastACPIRequest: u8,
    pub CgBifResp: u8,
    pub RequestType: u8,
    pub Padding: u8,
    pub D0Level: SMU71_Discrete_ACPILevel,
}

pub type SMU71_AcpiScoreboard = SMU71_AcpiScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_Discrete_PmFuses {
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
// dw9-dw12
    pub LPMLTemperatureScaler: [u8; 16],
// dw13-dw14
    pub FuzzyFan_ErrorSetDelta: i16,
    pub FuzzyFan_ErrorRateSetDelta: i16,
    pub FuzzyFan_PwmSetDelta: i16,
    pub Reserved6: u16,
// dw15
    pub GnbLPML: [u8; 16],
// dw15
    pub GnbLPMLMaxVid: u8,
    pub GnbLPMLMinVid: u8,
    pub Reserved1: [u8; 2],
// dw16
    pub BapmVddCBaseLeakageHiSidd: u16,
    pub BapmVddCBaseLeakageLoSidd: u16,
}

pub type SMU71_Discrete_PmFuses = SMU71_Discrete_PmFuses;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_Discrete_Log_Header_Table {
    pub version: u32,
    pub asic_id: u32,
    pub flags: u16,
    pub entry_size: u16,
    pub total_size: u32,
    pub num_of_entries: u32,
    pub type: u8,
    pub mode: u8,
    pub filler_0: [u8; 2],
    pub filler_1: [u32; 2],
}

pub type SMU71_Discrete_Log_Header_Table = SMU71_Discrete_Log_Header_Table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_Discrete_Log_Cntl {
    pub Enabled: u8,
    pub Type: u8,
    pub padding: [u8; 2],
    pub BufferSize: u32,
    pub SamplesLogged: u32,
    pub SampleSize: u32,
    pub AddrL: u32,
    pub AddrH: u32,
}

pub type SMU71_Discrete_Log_Cntl = SMU71_Discrete_Log_Cntl;

pub const CAC_ACC_NW_NUM_OF_SIGNALS: c_int = 83;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_Discrete_Cac_Collection_Table {
    pub temperature: u32,
    pub cac_acc_nw: [u32; CAC_ACC_NW_NUM_OF_SIGNALS],
    pub filler: [u32; 4],
}

pub type SMU71_Discrete_Cac_Collection_Table = SMU71_Discrete_Cac_Collection_Table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_Discrete_Cac_Verification_Table {
    pub VddcTotalPower: u32,
    pub VddcLeakagePower: u32,
    pub VddcConstantPower: u32,
    pub VddcGfxDynamicPower: u32,
    pub VddcUvdDynamicPower: u32,
    pub VddcVceDynamicPower: u32,
    pub VddcAcpDynamicPower: u32,
    pub VddcPcieDynamicPower: u32,
    pub VddcDceDynamicPower: u32,
    pub VddcCurrent: u32,
    pub VddcVoltage: u32,
    pub VddciTotalPower: u32,
    pub VddciLeakagePower: u32,
    pub VddciConstantPower: u32,
    pub VddciDynamicPower: u32,
    pub Vddr1TotalPower: u32,
    pub Vddr1LeakagePower: u32,
    pub Vddr1ConstantPower: u32,
    pub Vddr1DynamicPower: u32,
    pub spare: [u32; 8],
    pub temperature: u32,
}

pub type SMU71_Discrete_Cac_Verification_Table = SMU71_Discrete_Cac_Verification_Table;

