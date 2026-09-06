//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu72_discrete.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMIO_Pattern {
    pub Voltage: u16,
    pub Smio: u8,
    pub padding: u8,
}

pub type SMIO_Pattern = SMIO_Pattern;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMIO_Table {
    pub Pattern: [SMIO_Pattern; SMU_MAX_SMIO_LEVELS],
}

pub type SMIO_Table = SMIO_Table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_GraphicsLevel {
    pub MinVoltage: SMU_VoltageLevel,
    pub SclkFrequency: u32,
    pub pcieDpmLevel: u8,
    pub DeepSleepDivId: u8,
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
    pub UpHyst: u8,
    pub DownHyst: u8,
    pub VoltageDownHyst: u8,
    pub PowerThrottle: u8,
}

pub type SMU72_Discrete_GraphicsLevel = SMU72_Discrete_GraphicsLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_ACPILevel {
    pub Flags: u32,
    pub MinVoltage: SMU_VoltageLevel,
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

pub type SMU72_Discrete_ACPILevel = SMU72_Discrete_ACPILevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_Ulv {
    pub CcPwrDynRm: u32,
    pub CcPwrDynRm1: u32,
    pub VddcOffset: u16,
    pub VddcOffsetVid: u8,
    pub VddcPhase: u8,
    pub Reserved: u32,
}

pub type SMU72_Discrete_Ulv = SMU72_Discrete_Ulv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_MemoryLevel {
    pub MinVoltage: SMU_VoltageLevel,
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
    pub UpHyst: u8,
    pub DownHyst: u8,
    pub VoltageDownHyst: u8,
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

pub type SMU72_Discrete_MemoryLevel = SMU72_Discrete_MemoryLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_LinkLevel {
    pub /: *mut *mut uint8_t PcieGenSpeed; /< 0:PciE-gen1 1:PciE-gen2 2:PciE-gen3,
    pub /: *mut *mut uint8_t PcieLaneCount; /< 1=x1, 2=x2, 3=x4, 4=x8, 5=x12, 6=x16,
    pub EnabledForActivity: u8,
    pub SPC: u8,
    pub DownThreshold: u32,
    pub UpThreshold: u32,
    pub Reserved: u32,
}

pub type SMU72_Discrete_LinkLevel = SMU72_Discrete_LinkLevel;
// MC ARB DRAM Timing registers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_MCArbDramTimingTableEntry {
    pub McArbDramTiming: u32,
    pub McArbDramTiming2: u32,
    pub McArbBurstTime: u8,
    pub padding: [u8; 3],
}

pub type SMU72_Discrete_MCArbDramTimingTableEntry = SMU72_Discrete_MCArbDramTimingTableEntry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_MCArbDramTimingTable {
    pub entries: [SMU72_Discrete_MCArbDramTimingTableEntry; SMU__NUM_SCLK_DPM_STATE][SMU__NUM_MCLK_DPM_LEVELS],
}

pub type SMU72_Discrete_MCArbDramTimingTable = SMU72_Discrete_MCArbDramTimingTable;
// UVD VCLK/DCLK state (level) definition.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_UvdLevel {
    pub VclkFrequency: u32,
    pub DclkFrequency: u32,
    pub MinVoltage: SMU_VoltageLevel,
    pub VclkDivider: u8,
    pub DclkDivider: u8,
    pub padding: [u8; 2],
}

pub type SMU72_Discrete_UvdLevel = SMU72_Discrete_UvdLevel;
// Clocks for other external blocks (VCE, ACP, SAMU).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_ExtClkLevel {
    pub Frequency: u32,
    pub MinVoltage: SMU_VoltageLevel,
    pub Divider: u8,
    pub padding: [u8; 3],
}

pub type SMU72_Discrete_ExtClkLevel = SMU72_Discrete_ExtClkLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_StateInfo {
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

pub type SMU72_Discrete_StateInfo = SMU72_Discrete_StateInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_DpmTable {
// Multi-DPM controller settings
    pub GraphicsPIDController: SMU72_PIDController,
    pub MemoryPIDController: SMU72_PIDController,
    pub LinkPIDController: SMU72_PIDController,
    pub SystemFlags: u32,
// SMIO masks for voltage and phase controls
    pub VRConfig: u32,
    pub SmioMask1: u32,
    pub SmioMask2: u32,
    pub SmioTable1: SMIO_Table,
    pub SmioTable2: SMIO_Table,
    pub VddcLevelCount: u32,
    pub VddciLevelCount: u32,
    pub VddGfxLevelCount: u32,
    pub MvddLevelCount: u32,
    pub VddcTable: [u16; SMU72_MAX_LEVELS_VDDC],
    pub VddGfxTable: [u16; SMU72_MAX_LEVELS_VDDGFX],
    pub VddciTable: [u16; SMU72_MAX_LEVELS_VDDCI],
    pub BapmVddGfxVidHiSidd: [u8; SMU72_MAX_LEVELS_VDDGFX],
    pub BapmVddGfxVidLoSidd: [u8; SMU72_MAX_LEVELS_VDDGFX],
    pub BapmVddGfxVidHiSidd2: [u8; SMU72_MAX_LEVELS_VDDGFX],
    pub BapmVddcVidHiSidd: [u8; SMU72_MAX_LEVELS_VDDC],
    pub BapmVddcVidLoSidd: [u8; SMU72_MAX_LEVELS_VDDC],
    pub BapmVddcVidHiSidd2: [u8; SMU72_MAX_LEVELS_VDDC],
    pub GraphicsDpmLevelCount: u8,
    pub MemoryDpmLevelCount: u8,
    pub LinkLevelCount: u8,
    pub MasterDeepSleepControl: u8,
    pub UvdLevelCount: u8,
    pub VceLevelCount: u8,
    pub AcpLevelCount: u8,
    pub SamuLevelCount: u8,
    pub ThermOutGpio: u8,
    pub ThermOutPolarity: u8,
    pub ThermOutMode: u8,
    pub DPMFreezeAndForced: u8,
    pub Reserved: [u32; 4],
// State table entries for each DPM state
    pub GraphicsLevel: [SMU72_Discrete_GraphicsLevel; SMU72_MAX_LEVELS_GRAPHICS],
    pub MemoryACPILevel: SMU72_Discrete_MemoryLevel,
    pub MemoryLevel: [SMU72_Discrete_MemoryLevel; SMU72_MAX_LEVELS_MEMORY],
    pub LinkLevel: [SMU72_Discrete_LinkLevel; SMU72_MAX_LEVELS_LINK],
    pub ACPILevel: SMU72_Discrete_ACPILevel,
    pub UvdLevel: [SMU72_Discrete_UvdLevel; SMU72_MAX_LEVELS_UVD],
    pub VceLevel: [SMU72_Discrete_ExtClkLevel; SMU72_MAX_LEVELS_VCE],
    pub AcpLevel: [SMU72_Discrete_ExtClkLevel; SMU72_MAX_LEVELS_ACP],
    pub SamuLevel: [SMU72_Discrete_ExtClkLevel; SMU72_MAX_LEVELS_SAMU],
    pub Ulv: SMU72_Discrete_Ulv,
    pub SclkStepSize: u32,
    pub Smio: [u32; SMU72_MAX_ENTRIES_SMIO],
    pub UvdBootLevel: u8,
    pub VceBootLevel: u8,
    pub AcpBootLevel: u8,
    pub SamuBootLevel: u8,
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
    pub BootMVdd: u16,
    pub MemoryInterval: u8,
    pub MemoryThermThrottleEnable: u8,
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
    pub FpsHighThreshold: u16,
    pub FpsLowThreshold: u16,
    pub BAPMTI_R: [u16; SMU72_DTE_ITERATIONS][SMU72_DTE_SOURCES][SMU72_DTE_SINKS],
    pub BAPMTI_RC: [u16; SMU72_DTE_ITERATIONS][SMU72_DTE_SOURCES][SMU72_DTE_SINKS],
    pub DTEAmbientTempBase: u8,
    pub DTETjOffset: u8,
    pub GpuTjMax: u8,
    pub GpuTjHyst: u8,
    pub BootVoltage: SMU_VoltageLevel,
    pub BAPM_TEMP_GRADIENT: u32,
    pub LowSclkInterruptThreshold: u32,
    pub VddGfxReChkWait: u32,
    pub ClockStretcherAmount: u8,
    pub Sclk_CKS_masterEn0_7: u8,
    pub Sclk_CKS_masterEn8_15: u8,
    pub padding: [u8; 1],
    pub Sclk_voltageOffset: [u8; 8],
    pub ClockStretcherDataTable: SMU_ClockStretcherDataTable,
    pub CKS_LOOKUPTable: SMU_CKS_LOOKUPTable,
}

pub type SMU72_Discrete_DpmTable = SMU72_Discrete_DpmTable;
// --------------------------------------------------- AC Timing Parameters ------------------------------------------------
pub const SMU72_DISCRETE_MC_REGISTER_ARRAY_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_MCRegisterAddress {
    pub s0: u16,
    pub s1: u16,
}

pub type SMU72_Discrete_MCRegisterAddress = SMU72_Discrete_MCRegisterAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_MCRegisterSet {
    pub value: [u32; SMU72_DISCRETE_MC_REGISTER_ARRAY_SIZE],
}

pub type SMU72_Discrete_MCRegisterSet = SMU72_Discrete_MCRegisterSet;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_MCRegisters {
    pub last: u8,
    pub reserved: [u8; 3],
    pub address: [SMU72_Discrete_MCRegisterAddress; SMU72_DISCRETE_MC_REGISTER_ARRAY_SIZE],
    pub data: [SMU72_Discrete_MCRegisterSet; SMU72_DISCRETE_MC_REGISTER_ARRAY_SET_COUNT],
}

pub type SMU72_Discrete_MCRegisters = SMU72_Discrete_MCRegisters;
// --------------------------------------------------- Fan Table -----------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_FanTable {
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
    pub FanControl_GL_Flag: i8,
}

pub type SMU72_Discrete_FanTable = SMU72_Discrete_FanTable;
pub const SMU7_DISCRETE_GPIO_SCLK_DEBUG: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_MclkDpmScoreboard {
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
    pub MinimumPerfMclk: u32,
    pub AcpiReq: u8,
    pub AcpiAck: u8,
    pub MclkSwitchInProgress: u8,
    pub MclkSwitchCritical: u8,
    pub IgnoreVBlank: u8,
    pub TargetMclkIndex: u8,
    pub TargetMvddIndex: u8,
    pub MclkSwitchResult: u8,
    pub VbiFailureCount: u16,
    pub VbiWaitCounter: u8,
    pub EnabledLevelsChange: u8,
    pub LevelResidencyCountersN: [u16; SMU72_MAX_LEVELS_MEMORY],
    pub LevelSwitchCounters: [u16; SMU72_MAX_LEVELS_MEMORY],
    pub (*TargetStateCalculator)(uint8_t): *mut c_void,
    pub (*SavedTargetStateCalculator)(uint8_t): *mut c_void,
    pub AutoDpmInterval: u16,
    pub AutoDpmRange: u16,
    pub VbiTimeoutCount: u16,
    pub MclkSwitchingTime: u16,
    pub fastSwitch: u8,
    pub Save_PIC_VDDGFX_EXIT: u8,
    pub Save_PIC_VDDGFX_ENTER: u8,
    pub padding: u8,
}

pub type SMU7_MclkDpmScoreboard = SMU7_MclkDpmScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_UlvScoreboard {
    pub EnterUlv: u8,
    pub ExitUlv: u8,
    pub UlvActive: u8,
    pub WaitingForUlv: u8,
    pub UlvEnable: u8,
    pub UlvRunning: u8,
    pub UlvMasterEnable: u8,
    pub padding: u8,
    pub UlvAbortedCount: u32,
    pub UlvTimeStamp: u32,
}

pub type SMU7_UlvScoreboard = SMU7_UlvScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VddgfxSavedRegisters {
    pub GPU_DBG: [u32; 3],
    pub MEC_BaseAddress_Hi: u32,
    pub MEC_BaseAddress_Lo: u32,
    pub THM_TMON0_CTRL2__RDIR_PRESENT: u32,
    pub THM_TMON1_CTRL2__RDIR_PRESENT: u32,
    pub CP_INT_CNTL: u32,
}

pub type VddgfxSavedRegisters = VddgfxSavedRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_VddGfxScoreboard {
    pub VddGfxEnable: u8,
    pub VddGfxActive: u8,
    pub VPUResetOccured: u8,
    pub padding: u8,
    pub VddGfxEnteredCount: u32,
    pub VddGfxAbortedCount: u32,
    pub VddGfxVid: u32,
    pub SavedRegisters: VddgfxSavedRegisters,
}

pub type SMU7_VddGfxScoreboard = SMU7_VddGfxScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_TdcLimitScoreboard {
    pub Enable: u8,
    pub Running: u8,
    pub Alpha: u16,
    pub FilteredIddc: u32,
    pub IddcLimit: u32,
    pub IddcHyst: u32,
    pub HystControllerData: SMU7_HystController_Data,
}

pub type SMU7_TdcLimitScoreboard = SMU7_TdcLimitScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_PkgPwrLimitScoreboard {
    pub Enable: u8,
    pub Running: u8,
    pub Alpha: u16,
    pub FilteredPkgPwr: u32,
    pub Limit: u32,
    pub Hyst: u32,
    pub LimitFromDriver: u32,
    pub HystControllerData: SMU7_HystController_Data,
}

pub type SMU7_PkgPwrLimitScoreboard = SMU7_PkgPwrLimitScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_BapmScoreboard {
    pub source_powers: [u32; SMU72_DTE_SOURCES],
    pub source_powers_last: [u32; SMU72_DTE_SOURCES],
    pub entity_temperatures: [i32; SMU72_NUM_GPU_TES],
    pub initial_entity_temperatures: [i32; SMU72_NUM_GPU_TES],
    pub Limit: i32,
    pub Hyst: i32,
    pub 2]: *mut *mut *mut *mut int32_t therm_influence_coeff_table[SMU72_DTE_ITERATIONS  SMU72_DTE_SOURCES  SMU72_DTE_SINKS,
    pub SMU72_DTE_SINKS]: *mut *mut *mut int32_t therm_node_table[SMU72_DTE_ITERATIONS  SMU72_DTE_SOURCES,
    pub ConfigTDPPowerScalar: u16,
    pub FanSpeedPowerScalar: u16,
    pub OverDrivePowerScalar: u16,
    pub OverDriveLimitScalar: u16,
    pub FinalPowerScalar: u16,
    pub VariantID: u8,
    pub spare997: u8,
    pub HystControllerData: SMU7_HystController_Data,
    pub temperature_gradient_slope: i32,
    pub temperature_gradient: i32,
    pub measured_temperature: u32,
}

pub type SMU7_BapmScoreboard = SMU7_BapmScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_AcpiScoreboard {
    pub SavedInterruptMask: [u32; 2],
    pub LastACPIRequest: u8,
    pub CgBifResp: u8,
    pub RequestType: u8,
    pub Padding: u8,
    pub D0Level: SMU72_Discrete_ACPILevel,
}

pub type SMU7_AcpiScoreboard = SMU7_AcpiScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU72_Discrete_PmFuses {
// dw1
    pub SviLoadLineEn: u8,
    pub SviLoadLineVddC: u8,
    pub SviLoadLineTrimVddC: u8,
    pub SviLoadLineOffsetVddC: u8,
// dw2
    pub TDC_VDDC_PkgLimit: u16,
    pub TDC_VDDC_ThrottleReleaseLimitPerc: u8,
    pub TDC_MAWt: u8,
// dw3
    pub TdcWaterfallCtl: u8,
    pub LPMLTemperatureMin: u8,
    pub LPMLTemperatureMax: u8,
    pub Reserved: u8,
// dw4-dw7
    pub LPMLTemperatureScaler: [u8; 16],
// dw8-dw9
    pub FuzzyFan_ErrorSetDelta: i16,
    pub FuzzyFan_ErrorRateSetDelta: i16,
    pub FuzzyFan_PwmSetDelta: i16,
    pub Reserved6: u16,
// dw10-dw14
    pub GnbLPML: [u8; 16],
// dw15
    pub GnbLPMLMaxVid: u8,
    pub GnbLPMLMinVid: u8,
    pub Reserved1: [u8; 2],
// dw16
    pub BapmVddCBaseLeakageHiSidd: u16,
    pub BapmVddCBaseLeakageLoSidd: u16,
}

pub type SMU72_Discrete_PmFuses = SMU72_Discrete_PmFuses;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_Log_Header_Table {
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

pub type SMU7_Discrete_Log_Header_Table = SMU7_Discrete_Log_Header_Table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_Log_Cntl {
    pub Enabled: u8,
    pub Type: u8,
    pub padding: [u8; 2],
    pub BufferSize: u32,
    pub SamplesLogged: u32,
    pub SampleSize: u32,
    pub AddrL: u32,
    pub AddrH: u32,
}

pub type SMU7_Discrete_Log_Cntl = SMU7_Discrete_Log_Cntl;
pub const CAC_ACC_NW_NUM_OF_SIGNALS: c_int = 87;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_Cac_Collection_Table {
    pub temperature: u32,
    pub cac_acc_nw: [u32; CAC_ACC_NW_NUM_OF_SIGNALS],
}

pub type SMU7_Discrete_Cac_Collection_Table = SMU7_Discrete_Cac_Collection_Table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_Cac_Verification_Table {
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
    pub spare: [u32; 4],
    pub temperature: u32,
}

pub type SMU7_Discrete_Cac_Verification_Table = SMU7_Discrete_Cac_Verification_Table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_Pm_Status_Table {
// Thermal entities
    pub T_meas_max: i32,
    pub T_meas_acc: i32,
    pub T_calc_max: i32,
    pub T_calc_acc: i32,
    pub P_scalar_acc: u32,
    pub P_calc_max: u32,
    pub P_calc_acc: u32,
// Voltage domains
    pub I_calc_max: u32,
    pub I_calc_acc: u32,
    pub I_calc_acc_vddci: u32,
    pub V_calc_noload_acc: u32,
    pub V_calc_load_acc: u32,
    pub V_calc_noload_acc_vddci: u32,
    pub P_meas_acc: u32,
    pub V_meas_noload_acc: u32,
    pub V_meas_load_acc: u32,
    pub I_meas_acc: u32,
    pub P_meas_acc_vddci: u32,
    pub V_meas_noload_acc_vddci: u32,
    pub V_meas_load_acc_vddci: u32,
    pub I_meas_acc_vddci: u32,
// Frequency
    pub Sclk_dpm_residency: [u16; 8],
    pub Uvd_dpm_residency: [u16; 8],
    pub Vce_dpm_residency: [u16; 8],
    pub Mclk_dpm_residency: [u16; 4],
// Chip
    pub P_vddci_acc: u32,
    pub P_vddr1_acc: u32,
    pub P_nte1_acc: u32,
    pub PkgPwr_max: u32,
    pub PkgPwr_acc: u32,
    pub MclkSwitchingTime_max: u32,
    pub MclkSwitchingTime_acc: u32,
    pub FanPwm_acc: u32,
    pub FanRpm_acc: u32,
    pub AccCnt: u32,
}

pub type SMU7_Discrete_Pm_Status_Table = SMU7_Discrete_Pm_Status_Table;
// FIXME THESE NEED TO BE UPDATED
pub const SMU7_SCLK_CAC: c_uint = 0x561;
pub const SMU7_MCLK_CAC: c_uint = 0xF9;
pub const SMU7_VCLK_CAC: c_uint = 0x2DE;
pub const SMU7_DCLK_CAC: c_uint = 0x2DE;
pub const SMU7_ECLK_CAC: c_uint = 0x25E;
pub const SMU7_ACLK_CAC: c_uint = 0x25E;
pub const SMU7_SAMCLK_CAC: c_uint = 0x25E;
pub const SMU7_DISPCLK_CAC: c_uint = 0x100;
pub const SMU7_CAC_CONSTANT: c_uint = 0x2EE3430;
pub const SMU7_CAC_CONSTANT_SHIFT: c_int = 18;
pub const SMU7_VDDCI_MCLK_CONST: c_int = 1765;
pub const SMU7_VDDCI_MCLK_CONST_SHIFT: c_int = 16;
pub const SMU7_VDDCI_VDDCI_CONST: c_int = 50958;
pub const SMU7_VDDCI_VDDCI_CONST_SHIFT: c_int = 14;
pub const SMU7_VDDCI_CONST: c_int = 11781;
pub const SMU7_12C_VDDCI_MCLK_CONST: c_int = 1623;
pub const SMU7_12C_VDDCI_MCLK_CONST_SHIFT: c_int = 15;
pub const SMU7_12C_VDDCI_VDDCI_CONST: c_int = 40088;
pub const SMU7_12C_VDDCI_VDDCI_CONST_SHIFT: c_int = 13;
pub const SMU7_12C_VDDCI_CONST: c_int = 20856;
pub const SMU7_VDDCI_STROBE_PWR: c_int = 1331;
pub const SMU7_VDDR1_CONST: c_int = 693;
pub const SMU7_VDDR1_CAC_WEIGHT: c_int = 20;
pub const SMU7_VDDR1_CAC_WEIGHT_SHIFT: c_int = 19;
pub const SMU7_VDDR1_STROBE_PWR: c_int = 512;
pub const SMU7_AREA_COEFF_UVD: c_uint = 0xA78;
pub const SMU7_AREA_COEFF_VCE: c_uint = 0x190A;
pub const SMU7_AREA_COEFF_ACP: c_uint = 0x22D1;
pub const SMU7_AREA_COEFF_SAMU: c_uint = 0x534;
// ThermOutMode values
pub const SMU7_THERM_OUT_MODE_DISABLE: c_uint = 0x0;
pub const SMU7_THERM_OUT_MODE_THERM_ONLY: c_uint = 0x1;
pub const SMU7_THERM_OUT_MODE_THERM_VRHOT: c_uint = 0x2;

