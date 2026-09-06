//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu7_fusion.h
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
pub const SMU7_DTE_SOURCES: c_int = 5;
pub const SMU7_DTE_SINKS: c_int = 3;
pub const SMU7_NUM_CPU_TES: c_int = 2;
pub const SMU7_NUM_GPU_TES: c_int = 1;
pub const SMU7_NUM_NON_TES: c_int = 2;
// All 'soft registers' should be uint32_t.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_SoftRegisters {
    pub RefClockFrequency: u32,
    pub PmTimerP: u32,
    pub FeatureEnables: u32,
    pub HandshakeDisables: u32,
    pub DisplayPhy1Config: u8,
    pub DisplayPhy2Config: u8,
    pub DisplayPhy3Config: u8,
    pub DisplayPhy4Config: u8,
    pub DisplayPhy5Config: u8,
    pub DisplayPhy6Config: u8,
    pub DisplayPhy7Config: u8,
    pub DisplayPhy8Config: u8,
    pub AverageGraphicsA: u32,
    pub AverageMemoryA: u32,
    pub AverageGioA: u32,
    pub SClkDpmEnabledLevels: u8,
    pub MClkDpmEnabledLevels: u8,
    pub LClkDpmEnabledLevels: u8,
    pub PCIeDpmEnabledLevels: u8,
    pub UVDDpmEnabledLevels: u8,
    pub SAMUDpmEnabledLevels: u8,
    pub ACPDpmEnabledLevels: u8,
    pub VCEDpmEnabledLevels: u8,
    pub DRAM_LOG_ADDR_H: u32,
    pub DRAM_LOG_ADDR_L: u32,
    pub DRAM_LOG_PHY_ADDR_H: u32,
    pub DRAM_LOG_PHY_ADDR_L: u32,
    pub DRAM_LOG_BUFF_SIZE: u32,
    pub UlvEnterC: u32,
    pub UlvTime: u32,
    pub Reserved: [u32; 3],
}

pub type SMU7_SoftRegisters = SMU7_SoftRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_GraphicsLevel {
    pub MinVddNb: u32,
    pub SclkFrequency: u32,
    pub Vid: u8,
    pub VidOffset: u8,
    pub AT: u16,
    pub PowerThrottle: u8,
    pub GnbSlow: u8,
    pub ForceNbPs1: u8,
    pub SclkDid: u8,
    pub DisplayWatermark: u8,
    pub EnabledForActivity: u8,
    pub EnabledForThrottle: u8,
    pub UpH: u8,
    pub DownH: u8,
    pub VoltageDownH: u8,
    pub DeepSleepDivId: u8,
    pub ClkBypassCntl: u8,
    pub reserved: u32,
}

pub type SMU7_Fusion_GraphicsLevel = SMU7_Fusion_GraphicsLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_GIOLevel {
    pub EnabledForActivity: u8,
    pub LclkDid: u8,
    pub Vid: u8,
    pub VoltageDownH: u8,
    pub MinVddNb: u32,
    pub ResidencyCounter: u16,
    pub UpH: u8,
    pub DownH: u8,
    pub LclkFrequency: u32,
    pub ActivityLevel: u8,
    pub EnabledForThrottle: u8,
    pub ClkBypassCntl: u8,
    pub padding: u8,
}

pub type SMU7_Fusion_GIOLevel = SMU7_Fusion_GIOLevel;
// UVD VCLK/DCLK state (level) definition.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_UvdLevel {
    pub VclkFrequency: u32,
    pub DclkFrequency: u32,
    pub MinVddNb: u16,
    pub VclkDivider: u8,
    pub DclkDivider: u8,
    pub VClkBypassCntl: u8,
    pub DClkBypassCntl: u8,
    pub padding: [u8; 2],
}

pub type SMU7_Fusion_UvdLevel = SMU7_Fusion_UvdLevel;
// Clocks for other external blocks (VCE, ACP, SAMU).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_ExtClkLevel {
    pub Frequency: u32,
    pub MinVoltage: u16,
    pub Divider: u8,
    pub ClkBypassCntl: u8,
    pub Reserved: u32,
}

pub type SMU7_Fusion_ExtClkLevel = SMU7_Fusion_ExtClkLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_ACPILevel {
    pub Flags: u32,
    pub MinVddNb: u32,
    pub SclkFrequency: u32,
    pub SclkDid: u8,
    pub GnbSlow: u8,
    pub ForceNbPs1: u8,
    pub DisplayWatermark: u8,
    pub DeepSleepDivId: u8,
    pub padding: [u8; 3],
}

pub type SMU7_Fusion_ACPILevel = SMU7_Fusion_ACPILevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_NbDpm {
    pub DpmXNbPsHi: u8,
    pub DpmXNbPsLo: u8,
    pub Dpm0PgNbPsHi: u8,
    pub Dpm0PgNbPsLo: u8,
    pub EnablePsi1: u8,
    pub SkipDPM0: u8,
    pub SkipPG: u8,
    pub Hysteresis: u8,
    pub EnableDpmPstatePoll: u8,
    pub padding: [u8; 3],
}

pub type SMU7_Fusion_NbDpm = SMU7_Fusion_NbDpm;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_StateInfo {
    pub SclkFrequency: u32,
    pub LclkFrequency: u32,
    pub VclkFrequency: u32,
    pub DclkFrequency: u32,
    pub SamclkFrequency: u32,
    pub AclkFrequency: u32,
    pub EclkFrequency: u32,
    pub DisplayWatermark: u8,
    pub McArbIndex: u8,
    pub SclkIndex: i8,
    pub MclkIndex: i8,
}

pub type SMU7_Fusion_StateInfo = SMU7_Fusion_StateInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_DpmTable {
    pub SystemFlags: u32,
    pub GraphicsPIDController: SMU7_PIDController,
    pub GioPIDController: SMU7_PIDController,
    pub GraphicsDpmLevelCount: u8,
    pub GIOLevelCount: u8,
    pub UvdLevelCount: u8,
    pub VceLevelCount: u8,
    pub AcpLevelCount: u8,
    pub SamuLevelCount: u8,
    pub FpsHighT: u16,
    pub GraphicsLevel: [SMU7_Fusion_GraphicsLevel; SMU__NUM_SCLK_DPM_STATE],
    pub ACPILevel: SMU7_Fusion_ACPILevel,
    pub UvdLevel: [SMU7_Fusion_UvdLevel; SMU7_MAX_LEVELS_UVD],
    pub VceLevel: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_VCE],
    pub AcpLevel: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_ACP],
    pub SamuLevel: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_SAMU],
    pub UvdBootLevel: u8,
    pub VceBootLevel: u8,
    pub AcpBootLevel: u8,
    pub SamuBootLevel: u8,
    pub UVDInterval: u8,
    pub VCEInterval: u8,
    pub ACPInterval: u8,
    pub SAMUInterval: u8,
    pub GraphicsBootLevel: u8,
    pub GraphicsInterval: u8,
    pub GraphicsThermThrottleEnable: u8,
    pub GraphicsVoltageChangeEnable: u8,
    pub GraphicsClkSlowEnable: u8,
    pub GraphicsClkSlowDivider: u8,
    pub FpsLowT: u16,
    pub DisplayCac: u32,
    pub LowSclkInterruptT: u32,
    pub DRAM_LOG_ADDR_H: u32,
    pub DRAM_LOG_ADDR_L: u32,
    pub DRAM_LOG_PHY_ADDR_H: u32,
    pub DRAM_LOG_PHY_ADDR_L: u32,
    pub DRAM_LOG_BUFF_SIZE: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Fusion_GIODpmTable {
    pub GIOLevel: [SMU7_Fusion_GIOLevel; SMU7_MAX_LEVELS_GIO],
    pub GioPIDController: SMU7_PIDController,
    pub GIOLevelCount: u32,
    pub Enable: u8,
    pub GIOVoltageChangeEnable: u8,
    pub GIOBootLevel: u8,
    pub padding: u8,
    pub padding1: [u8; 2],
    pub TargetState: u8,
    pub CurrenttState: u8,
    pub ThrottleOnHtc: u8,
    pub ThermThrottleStatus: u8,
    pub ThermThrottleTempSelect: u8,
    pub ThermThrottleEnable: u8,
    pub TemperatureLimitHigh: u16,
    pub TemperatureLimitLow: u16,
}

pub type SMU7_Fusion_DpmTable = SMU7_Fusion_DpmTable;
pub type SMU7_Fusion_GIODpmTable = SMU7_Fusion_GIODpmTable;

