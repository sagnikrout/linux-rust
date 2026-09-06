//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu71.h
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

pub const SMU__NUM_PCIE_DPM_LEVELS: c_int = 8;
pub const SMU__NUM_SCLK_DPM_STATE: c_int = 8;
pub const SMU__NUM_MCLK_DPM_LEVELS: c_int = 4;
pub const SMU__VARIANT__ICELAND: c_int = 1;
pub const SMU__DGPU_ONLY: c_int = 1;
pub const SMU__DYNAMIC_MCARB_SETTINGS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SID_OPTION {
    SID_OPTION_HI,
    SID_OPTION_LO,
    SID_OPTION_COUNT
}

pub const SMU7_CONTEXT_ID_SMC: c_int = 1;
pub const SMU7_CONTEXT_ID_VBIOS: c_int = 2;
pub const SMU71_MAX_LEVELS_VDDC: c_int = 8;
pub const SMU71_MAX_LEVELS_VDDCI: c_int = 4;
pub const SMU71_MAX_LEVELS_MVDD: c_int = 4;
pub const SMU71_MAX_LEVELS_VDDNB: c_int = 8;

pub const SMU71_MAX_ENTRIES_SMIO: c_int = 32;
pub const DPM_NO_LIMIT: c_int = 0;
pub const DPM_NO_UP: c_int = 1;
pub const DPM_GO_DOWN: c_int = 2;
pub const DPM_GO_UP: c_int = 3;
pub const SMU7_FIRST_DPM_GRAPHICS_LEVEL: c_int = 0;
pub const SMU7_FIRST_DPM_MEMORY_LEVEL: c_int = 0;
pub const GPIO_CLAMP_MODE_VRHOT: c_int = 1;
pub const GPIO_CLAMP_MODE_THERM: c_int = 2;
pub const GPIO_CLAMP_MODE_DC: c_int = 4;
pub const SCRATCH_B_TARG_PCIE_INDEX_SHIFT: c_int = 0;

pub const SCRATCH_B_CURR_PCIE_INDEX_SHIFT: c_int = 3;

pub const SCRATCH_B_TARG_UVD_INDEX_SHIFT: c_int = 6;

pub const SCRATCH_B_CURR_UVD_INDEX_SHIFT: c_int = 9;

pub const SCRATCH_B_TARG_VCE_INDEX_SHIFT: c_int = 12;

pub const SCRATCH_B_CURR_VCE_INDEX_SHIFT: c_int = 15;

pub const SCRATCH_B_TARG_ACP_INDEX_SHIFT: c_int = 18;

pub const SCRATCH_B_CURR_ACP_INDEX_SHIFT: c_int = 21;

pub const SCRATCH_B_TARG_SAMU_INDEX_SHIFT: c_int = 24;

pub const SCRATCH_B_CURR_SAMU_INDEX_SHIFT: c_int = 27;

pub const SMU71_DTE_ITERATIONS: c_int = 5;
pub const SMU71_DTE_SOURCES: c_int = 3;
pub const SMU71_DTE_SINKS: c_int = 1;
pub const SMU71_NUM_CPU_TES: c_int = 0;
pub const SMU71_NUM_GPU_TES: c_int = 1;
pub const SMU71_NUM_NON_TES: c_int = 2;

pub const SMU7_DTE_ITERATIONS: c_int = 5;
pub const SMU7_DTE_SOURCES: c_int = 5;
pub const SMU7_DTE_SINKS: c_int = 3;
pub const SMU7_NUM_CPU_TES: c_int = 2;
pub const SMU7_NUM_GPU_TES: c_int = 1;
pub const SMU7_NUM_NON_TES: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_PIDController {
    pub Ki: u32,
    pub LFWindupUpperLim: i32,
    pub LFWindupLowerLim: i32,
    pub StatePrecision: u32,
    pub LfPrecision: u32,
    pub LfOffset: u32,
    pub MaxState: u32,
    pub MaxLfFraction: u32,
    pub StateShift: u32,
}

pub type SMU71_PIDController = SMU71_PIDController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_LocalDpmScoreboard {
    pub PercentageBusy: u32,
    pub PIDError: i32,
    pub PIDIntegral: i32,
    pub PIDOutput: i32,
    pub SigmaDeltaAccum: u32,
    pub SigmaDeltaOutput: u32,
    pub SigmaDeltaLevel: u32,
    pub UtilizationSetpoint: u32,
    pub TdpClampMode: u8,
    pub TdcClampMode: u8,
    pub ThermClampMode: u8,
    pub VoltageBusy: u8,
    pub CurrLevel: i8,
    pub TargLevel: i8,
    pub LevelChangeInProgress: u8,
    pub UpHyst: u8,
    pub DownHyst: u8,
    pub VoltageDownHyst: u8,
    pub DpmEnable: u8,
    pub DpmRunning: u8,
    pub DpmForce: u8,
    pub DpmForceLevel: u8,
    pub DisplayWatermark: u8,
    pub McArbIndex: u8,
    pub MinimumPerfSclk: u32,
    pub AcpiReq: u8,
    pub AcpiAck: u8,
    pub GfxClkSlow: u8,
    pub GpioClampMode: u8,
    pub FpsFilterWeight: u8,
    pub EnabledLevelsChange: u8,
    pub DteClampMode: u8,
    pub FpsClampMode: u8,
    pub LevelResidencyCounters: [u16; SMU71_MAX_LEVELS_GRAPHICS],
    pub LevelSwitchCounters: [u16; SMU71_MAX_LEVELS_GRAPHICS],
    pub (*TargetStateCalculator)(uint8_t): *mut c_void,
    pub (*SavedTargetStateCalculator)(uint8_t): *mut c_void,
    pub AutoDpmInterval: u16,
    pub AutoDpmRange: u16,
    pub FpsEnabled: u8,
    pub MaxPerfLevel: u8,
    pub AllowLowClkInterruptToHost: u8,
    pub FpsRunning: u8,
    pub MaxAllowedFrequency: u32,
}

pub type SMU7_LocalDpmScoreboard = SMU7_LocalDpmScoreboard;
pub const SMU7_MAX_VOLTAGE_CLIENTS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_VoltageScoreboard {
    pub CurrentVoltage: u16,
    pub HighestVoltage: u16,
    pub MaxVid: u16,
    pub HighestVidOffset: u8,
    pub CurrentVidOffset: u8,

    pub CurrentPhases: u8,
    pub HighestPhases: u8,

    pub AvsOffset: u8,
    pub AvsOffsetApplied: u8,

    pub ControllerBusy: u8,
    pub CurrentVid: u8,
    pub RequestedVoltage: [u16; SMU7_MAX_VOLTAGE_CLIENTS],    pub RequestedPhases: [u8; SMU7_MAX_VOLTAGE_CLIENTS],    pub EnabledRequest: [u8; SMU7_MAX_VOLTAGE_CLIENTS],
    pub TargetIndex: u8,
    pub Delay: u8,
    pub ControllerEnable: u8,
    pub ControllerRunning: u8,
    pub CurrentStdVoltageHiSidd: u16,
    pub CurrentStdVoltageLoSidd: u16,

    pub RequestedVddci: u16,
    pub CurrentVddci: u16,
    pub HighestVddci: u16,
    pub CurrentVddciVid: u8,
    pub TargetVddciIndex: u8,

}

pub type SMU7_VoltageScoreboard = SMU7_VoltageScoreboard;
// -------------------------------------------------------------------------------------------------------------------------

pub type SMU7_PCIeLinkSpeedScoreboard = SMU7_PCIeLinkSpeedScoreboard;
// -------------------------------------------------------- CAC table ------------------------------------------------------
pub const SMU7_LKGE_LUT_NUM_OF_TEMP_ENTRIES: c_int = 16;
pub const SMU7_LKGE_LUT_NUM_OF_VOLT_ENTRIES: c_int = 16;
pub const SMU7_SCALE_I: c_int = 7;
pub const SMU7_SCALE_R: c_int = 12;
pub type SMU7_PowerScoreboard = SMU7_PowerScoreboard;
// --------------------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_ThermalScoreboard {
    pub GpuLimit: i16,
    pub GpuHyst: i16,
    pub CurrGnbTemp: u16,
    pub FilteredGnbTemp: u16,
    pub ControllerEnable: u8,
    pub ControllerRunning: u8,
    pub WaterfallUp: u8,
    pub WaterfallDown: u8,
    pub WaterfallLimit: u8,
    pub padding: [u8; 3],
}

pub type SMU7_ThermalScoreboard = SMU7_ThermalScoreboard;
// For FeatureEnables:
pub const SMU7_SCLK_DPM_CONFIG_MASK: c_uint = 0x01;
pub const SMU7_VOLTAGE_CONTROLLER_CONFIG_MASK: c_uint = 0x02;
pub const SMU7_THERMAL_CONTROLLER_CONFIG_MASK: c_uint = 0x04;
pub const SMU7_MCLK_DPM_CONFIG_MASK: c_uint = 0x08;
pub const SMU7_UVD_DPM_CONFIG_MASK: c_uint = 0x10;
pub const SMU7_VCE_DPM_CONFIG_MASK: c_uint = 0x20;
pub const SMU7_ACP_DPM_CONFIG_MASK: c_uint = 0x40;
pub const SMU7_SAMU_DPM_CONFIG_MASK: c_uint = 0x80;
pub const SMU7_PCIEGEN_DPM_CONFIG_MASK: c_uint = 0x100;
pub const SMU7_ACP_MCLK_HANDSHAKE_DISABLE: c_uint = 0x00000001;
pub const SMU7_ACP_SCLK_HANDSHAKE_DISABLE: c_uint = 0x00000002;
pub const SMU7_UVD_MCLK_HANDSHAKE_DISABLE: c_uint = 0x00000100;
pub const SMU7_UVD_SCLK_HANDSHAKE_DISABLE: c_uint = 0x00000200;
pub const SMU7_VCE_MCLK_HANDSHAKE_DISABLE: c_uint = 0x00010000;
pub const SMU7_VCE_SCLK_HANDSHAKE_DISABLE: c_uint = 0x00020000;
// All 'soft registers' should be uint32_t.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_SoftRegisters {
    pub RefClockFrequency: u32,
    pub PmTimerPeriod: u32,
    pub FeatureEnables: u32,

    pub PreVBlankGap: u32,
    pub VBlankTimeout: u32,
    pub TrainTimeGap: u32,
    pub MvddSwitchTime: u32,
    pub LongestAcpiTrainTime: u32,
    pub AcpiDelay: u32,
    pub G5TrainTime: u32,
    pub DelayMpllPwron: u32,
    pub VoltageChangeTimeout: u32,

    pub HandshakeDisables: u32,
    pub DisplayPhy1Config: u8,
    pub DisplayPhy2Config: u8,
    pub DisplayPhy3Config: u8,
    pub DisplayPhy4Config: u8,
    pub DisplayPhy5Config: u8,
    pub DisplayPhy6Config: u8,
    pub DisplayPhy7Config: u8,
    pub DisplayPhy8Config: u8,
    pub AverageGraphicsActivity: u32,
    pub AverageMemoryActivity: u32,
    pub AverageGioActivity: u32,
    pub SClkDpmEnabledLevels: u8,
    pub MClkDpmEnabledLevels: u8,
    pub LClkDpmEnabledLevels: u8,
    pub PCIeDpmEnabledLevels: u8,
    pub DRAM_LOG_ADDR_H: u32,
    pub DRAM_LOG_ADDR_L: u32,
    pub DRAM_LOG_PHY_ADDR_H: u32,
    pub DRAM_LOG_PHY_ADDR_L: u32,
    pub DRAM_LOG_BUFF_SIZE: u32,
    pub UlvEnterCount: u32,
    pub UlvTime: u32,
    pub UcodeLoadStatus: u32,
    pub DPMFreezeAndForced: u8,
    pub Activity_Weight: u8,
    pub Reserved8: [u8; 2],
    pub Reserved: u32,
}

pub type SMU71_SoftRegisters = SMU71_SoftRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU71_Firmware_Header {
    pub Digest: [u32; 5],
    pub Version: u32,
    pub HeaderSize: u32,
    pub Flags: u32,
    pub EntryPoint: u32,
    pub CodeSize: u32,
    pub ImageSize: u32,
    pub Rtos: u32,
    pub SoftRegisters: u32,
    pub DpmTable: u32,
    pub FanTable: u32,
    pub CacConfigTable: u32,
    pub CacStatusTable: u32,
    pub mcRegisterTable: u32,
    pub mcArbDramTimingTable: u32,
    pub PmFuseTable: u32,
    pub Globals: u32,
    pub UvdDpmTable: u32,
    pub AcpDpmTable: u32,
    pub VceDpmTable: u32,
    pub SamuDpmTable: u32,
    pub UlvSettings: u32,
    pub Reserved: [u32; 37],
    pub Signature: u32,
}

pub type SMU71_Firmware_Header = SMU71_Firmware_Header;
pub type SMU7_HystController_Data = SMU7_HystController_Data;
pub const SMU71_FIRMWARE_HEADER_LOCATION: c_uint = 0x20000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DisplayConfig {
    PowerDown = 1,
    DP54x4,
    DP54x2,
    DP54x1,
    DP27x4,
    DP27x2,
    DP27x1,
    HDMI297,
    HDMI162,
    LVDS,
    DP324x4,
    DP324x2,
    DP324x1
}

// #define SX_BLOCK_COUNT 8
// #define MC_BLOCK_COUNT 1
// #define CPL_BLOCK_COUNT 27

pub const SX_BLOCK_COUNT: c_int = 8;
pub const MC_BLOCK_COUNT: c_int = 1;
pub const CPL_BLOCK_COUNT: c_int = 29;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Local_Cac {
    pub BlockId: u8,
    pub SignalId: u8,
    pub Threshold: u8,
    pub Padding: u8,
}

pub type SMU7_Local_Cac = SMU7_Local_Cac;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Local_Cac_Table {
    pub SxLocalCac: [SMU7_Local_Cac; SX_BLOCK_COUNT],
    pub CplLocalCac: [SMU7_Local_Cac; CPL_BLOCK_COUNT],
    pub McLocalCac: [SMU7_Local_Cac; MC_BLOCK_COUNT],
}

pub type SMU7_Local_Cac_Table = SMU7_Local_Cac_Table;

