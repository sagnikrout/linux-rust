//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/smu7_discrete.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_SoftRegisters {
    pub RefClockFrequency: u32,
    pub PmTimerP: u32,
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
pub struct SMU7_Discrete_VoltageLevel {
    pub Voltage: u16,
    pub StdVoltageHiSidd: u16,
    pub StdVoltageLoSidd: u16,
    pub Smio: u8,
    pub padding: u8,
}

pub type SMU7_Discrete_VoltageLevel = SMU7_Discrete_VoltageLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_GraphicsLevel {
    pub Flags: u32,
    pub MinVddc: u32,
    pub MinVddcPhases: u32,
    pub SclkFrequency: u32,
    pub padding1: [u8; 2],
    pub ActivityLevel: u16,
    pub CgSpllFuncCntl3: u32,
    pub CgSpllFuncCntl4: u32,
    pub SpllSpreadSpectrum: u32,
    pub SpllSpreadSpectrum2: u32,
    pub CcPwrDynRm: u32,
    pub CcPwrDynRm1: u32,
    pub SclkDid: u8,
    pub DisplayWatermark: u8,
    pub EnabledForActivity: u8,
    pub EnabledForThrottle: u8,
    pub UpH: u8,
    pub DownH: u8,
    pub VoltageDownH: u8,
    pub PowerThrottle: u8,
    pub DeepSleepDivId: u8,
    pub padding: [u8; 3],
}

pub type SMU7_Discrete_GraphicsLevel = SMU7_Discrete_GraphicsLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_ACPILevel {
    pub Flags: u32,
    pub MinVddc: u32,
    pub MinVddcPhases: u32,
    pub SclkFrequency: u32,
    pub SclkDid: u8,
    pub DisplayWatermark: u8,
    pub DeepSleepDivId: u8,
    pub padding: u8,
    pub CgSpllFuncCntl: u32,
    pub CgSpllFuncCntl2: u32,
    pub CgSpllFuncCntl3: u32,
    pub CgSpllFuncCntl4: u32,
    pub SpllSpreadSpectrum: u32,
    pub SpllSpreadSpectrum2: u32,
    pub CcPwrDynRm: u32,
    pub CcPwrDynRm1: u32,
}

pub type SMU7_Discrete_ACPILevel = SMU7_Discrete_ACPILevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_Ulv {
    pub CcPwrDynRm: u32,
    pub CcPwrDynRm1: u32,
    pub VddcOffset: u16,
    pub VddcOffsetVid: u8,
    pub VddcPhase: u8,
    pub Reserved: u32,
}

pub type SMU7_Discrete_Ulv = SMU7_Discrete_Ulv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_MemoryLevel {
    pub MinVddc: u32,
    pub MinVddcPhases: u32,
    pub MinVddci: u32,
    pub MinMvdd: u32,
    pub MclkFrequency: u32,
    pub EdcReadEnable: u8,
    pub EdcWriteEnable: u8,
    pub RttEnable: u8,
    pub StutterEnable: u8,
    pub StrobeEnable: u8,
    pub StrobeRatio: u8,
    pub EnabledForThrottle: u8,
    pub EnabledForActivity: u8,
    pub UpH: u8,
    pub DownH: u8,
    pub VoltageDownH: u8,
    pub padding: u8,
    pub ActivityLevel: u16,
    pub DisplayWatermark: u8,
    pub padding1: u8,
    pub MpllFuncCntl: u32,
    pub MpllFuncCntl_1: u32,
    pub MpllFuncCntl_2: u32,
    pub MpllAdFuncCntl: u32,
    pub MpllDqFuncCntl: u32,
    pub MclkPwrmgtCntl: u32,
    pub DllCntl: u32,
    pub MpllSs1: u32,
    pub MpllSs2: u32,
}

pub type SMU7_Discrete_MemoryLevel = SMU7_Discrete_MemoryLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_LinkLevel {
    pub PcieGenSpeed: u8,
    pub PcieLaneCount: u8,
    pub EnabledForActivity: u8,
    pub Padding: u8,
    pub DownT: u32,
    pub UpT: u32,
    pub Reserved: u32,
}

pub type SMU7_Discrete_LinkLevel = SMU7_Discrete_LinkLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_MCArbDramTimingTableEntry {
    pub McArbDramTiming: u32,
    pub McArbDramTiming2: u32,
    pub McArbBurstTime: u8,
    pub padding: [u8; 3],
}

pub type SMU7_Discrete_MCArbDramTimingTableEntry = SMU7_Discrete_MCArbDramTimingTableEntry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_MCArbDramTimingTable {
    pub entries: [SMU7_Discrete_MCArbDramTimingTableEntry; SMU__NUM_SCLK_DPM_STATE][SMU__NUM_MCLK_DPM_LEVELS],
}

pub type SMU7_Discrete_MCArbDramTimingTable = SMU7_Discrete_MCArbDramTimingTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_UvdLevel {
    pub VclkFrequency: u32,
    pub DclkFrequency: u32,
    pub MinVddc: u16,
    pub MinVddcPhases: u8,
    pub VclkDivider: u8,
    pub DclkDivider: u8,
    pub padding: [u8; 3],
}

pub type SMU7_Discrete_UvdLevel = SMU7_Discrete_UvdLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_ExtClkLevel {
    pub Frequency: u32,
    pub MinVoltage: u16,
    pub MinPhases: u8,
    pub Divider: u8,
}

pub type SMU7_Discrete_ExtClkLevel = SMU7_Discrete_ExtClkLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_StateInfo {
    pub SclkFrequency: u32,
    pub MclkFrequency: u32,
    pub VclkFrequency: u32,
    pub DclkFrequency: u32,
    pub SamclkFrequency: u32,
    pub AclkFrequency: u32,
    pub EclkFrequency: u32,
    pub MvddVoltage: u16,
    pub padding16: u16,
    pub DisplayWatermark: u8,
    pub McArbIndex: u8,
    pub McRegIndex: u8,
    pub SeqIndex: u8,
    pub SclkDid: u8,
    pub SclkIndex: i8,
    pub MclkIndex: i8,
    pub PCIeGen: u8,
}

pub type SMU7_Discrete_StateInfo = SMU7_Discrete_StateInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_DpmTable {
    pub GraphicsPIDController: SMU7_PIDController,
    pub MemoryPIDController: SMU7_PIDController,
    pub LinkPIDController: SMU7_PIDController,
    pub SystemFlags: u32,
    pub SmioMaskVddcVid: u32,
    pub SmioMaskVddcPhase: u32,
    pub SmioMaskVddciVid: u32,
    pub SmioMaskMvddVid: u32,
    pub VddcLevelCount: u32,
    pub VddciLevelCount: u32,
    pub MvddLevelCount: u32,
    pub [SMU7_MAX_LEVELS_VDDC]: SMU7_Discrete_VoltageLevel VddcLevel,
// SMU7_Discrete_VoltageLevel          VddcStandardReference   [SMU7_MAX_LEVELS_VDDC];
    pub [SMU7_MAX_LEVELS_VDDCI]: SMU7_Discrete_VoltageLevel VddciLevel,
    pub [SMU7_MAX_LEVELS_MVDD]: SMU7_Discrete_VoltageLevel MvddLevel,
    pub GraphicsDpmLevelCount: u8,
    pub MemoryDpmLevelCount: u8,
    pub LinkLevelCount: u8,
    pub UvdLevelCount: u8,
    pub VceLevelCount: u8,
    pub AcpLevelCount: u8,
    pub SamuLevelCount: u8,
    pub MasterDeepSleepControl: u8,
    pub Reserved: [u32; 5],
// uint32_t                            SamuDefaultLevel;
    pub [SMU7_MAX_LEVELS_GRAPHICS]: SMU7_Discrete_GraphicsLevel GraphicsLevel,
    pub MemoryACPILevel: SMU7_Discrete_MemoryLevel,
    pub [SMU7_MAX_LEVELS_MEMORY]: SMU7_Discrete_MemoryLevel MemoryLevel,
    pub [SMU7_MAX_LEVELS_LINK]: SMU7_Discrete_LinkLevel LinkLevel,
    pub ACPILevel: SMU7_Discrete_ACPILevel,
    pub [SMU7_MAX_LEVELS_UVD]: SMU7_Discrete_UvdLevel UvdLevel,
    pub [SMU7_MAX_LEVELS_VCE]: SMU7_Discrete_ExtClkLevel VceLevel,
    pub [SMU7_MAX_LEVELS_ACP]: SMU7_Discrete_ExtClkLevel AcpLevel,
    pub [SMU7_MAX_LEVELS_SAMU]: SMU7_Discrete_ExtClkLevel SamuLevel,
    pub Ulv: SMU7_Discrete_Ulv,
    pub SclkStepSize: u32,
    pub [SMU7_MAX_ENTRIES_SMIO]: uint32_t Smio,
    pub UvdBootLevel: u8,
    pub VceBootLevel: u8,
    pub AcpBootLevel: u8,
    pub SamuBootLevel: u8,
    pub UVDInterval: u8,
    pub VCEInterval: u8,
    pub ACPInterval: u8,
    pub SAMUInterval: u8,
    pub GraphicsBootLevel: u8,
    pub GraphicsVoltageChangeEnable: u8,
    pub GraphicsThermThrottleEnable: u8,
    pub GraphicsInterval: u8,
    pub VoltageInterval: u8,
    pub ThermalInterval: u8,
    pub TemperatureLimitHigh: u16,
    pub TemperatureLimitLow: u16,
    pub MemoryBootLevel: u8,
    pub MemoryVoltageChangeEnable: u8,
    pub MemoryInterval: u8,
    pub MemoryThermThrottleEnable: u8,
    pub VddcVddciDelta: u16,
    pub VoltageResponseTime: u16,
    pub PhaseResponseTime: u16,
    pub PCIeBootLinkLevel: u8,
    pub PCIeGenInterval: u8,
    pub DTEInterval: u8,
    pub DTEMode: u8,
    pub SVI2Enable: u8,
    pub VRHotGpio: u8,
    pub AcDcGpio: u8,
    pub ThermGpio: u8,
    pub PPM_PkgPwrLimit: u16,
    pub PPM_TemperatureLimit: u16,
    pub DefaultTdp: u16,
    pub TargetTdp: u16,
    pub FpsHighT: u16,
    pub FpsLowT: u16,
    pub [SMU7_DTE_ITERATIONS][SMU7_DTE_SOURCES][SMU7_DTE_SINKS]: uint16_t BAPMTI_R,
    pub [SMU7_DTE_ITERATIONS][SMU7_DTE_SOURCES][SMU7_DTE_SINKS]: uint16_t BAPMTI_RC,
    pub DTEAmbientTempBase: u8,
    pub DTETjOffset: u8,
    pub GpuTjMax: u8,
    pub GpuTjHyst: u8,
    pub BootVddc: u16,
    pub BootVddci: u16,
    pub BootMVdd: u16,
    pub padding: u16,
    pub BAPM_TEMP_GRADIENT: u32,
    pub LowSclkInterruptT: u32,
}

pub type SMU7_Discrete_DpmTable = SMU7_Discrete_DpmTable;
pub const SMU7_DISCRETE_MC_REGISTER_ARRAY_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_MCRegisterAddress {
    pub s0: u16,
    pub s1: u16,
}

pub type SMU7_Discrete_MCRegisterAddress = SMU7_Discrete_MCRegisterAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_MCRegisterSet {
    pub value: [u32; SMU7_DISCRETE_MC_REGISTER_ARRAY_SIZE],
}

pub type SMU7_Discrete_MCRegisterSet = SMU7_Discrete_MCRegisterSet;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_MCRegisters {
    pub last: u8,
    pub reserved: [u8; 3],
    pub address: [SMU7_Discrete_MCRegisterAddress; SMU7_DISCRETE_MC_REGISTER_ARRAY_SIZE],
    pub data: [SMU7_Discrete_MCRegisterSet; SMU7_DISCRETE_MC_REGISTER_ARRAY_SET_COUNT],
}

pub type SMU7_Discrete_MCRegisters = SMU7_Discrete_MCRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_FanTable {
    pub FdoMode: u16,
    pub TempMin: i16,
    pub TempMed: i16,
    pub TempMax: i16,
    pub Slope1: i16,
    pub Slope2: i16,
    pub FdoMin: i16,
    pub HystUp: i16,
    pub HystDown: i16,
    pub HystSlope: i16,
    pub TempRespLim: i16,
    pub TempCurr: i16,
    pub SlopeCurr: i16,
    pub PwmCurr: i16,
    pub RefreshPeriod: u32,
    pub FdoMax: i16,
    pub TempSrc: u8,
    pub Padding: i8,
}

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

