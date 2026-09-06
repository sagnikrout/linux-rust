//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu75_discrete.h
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

pub const NUM_SCLK_RANGE: c_int = 8;
pub const VCO_3_6: c_int = 1;
pub const VCO_2_4: c_int = 3;
pub const POSTDIV_DIV_BY_1: c_int = 0;
pub const POSTDIV_DIV_BY_2: c_int = 1;
pub const POSTDIV_DIV_BY_4: c_int = 2;
pub const POSTDIV_DIV_BY_8: c_int = 3;
pub const POSTDIV_DIV_BY_16: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclkFcwRange_t {
    pub /: *mut *mut uint8_t vco_setting; / 1: 3-6GHz, 3: 2-4GHz,
    pub /: *mut *mut uint8_t postdiv; / divide by 2^n,
    pub fcw_pcc: u16,
    pub fcw_trans_upper: u16,
    pub fcw_trans_lower: u16,
}

pub type sclkFcwRange_t = sclkFcwRange_t;
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
pub struct SMU_SclkSetting {
    pub SclkFrequency: u32,
    pub Fcw_int: u16,
    pub Fcw_frac: u16,
    pub Pcc_fcw_int: u16,
    pub PllRange: u8,
    pub SSc_En: u8,
    pub Sclk_slew_rate: u16,
    pub Pcc_up_slew_rate: u16,
    pub Pcc_down_slew_rate: u16,
    pub Fcw1_int: u16,
    pub Fcw1_frac: u16,
    pub Sclk_ss_slew_rate: u16,
}

pub type SMU_SclkSetting = SMU_SclkSetting;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_GraphicsLevel {
    pub MinVoltage: SMU_VoltageLevel,
    pub pcieDpmLevel: u8,
    pub DeepSleepDivId: u8,
    pub ActivityLevel: u16,
    pub CgSpllFuncCntl3: u32,
    pub CgSpllFuncCntl4: u32,
    pub CcPwrDynRm: u32,
    pub CcPwrDynRm1: u32,
    pub SclkDid: u8,
    pub padding: u8,
    pub EnabledForActivity: u8,
    pub EnabledForThrottle: u8,
    pub UpHyst: u8,
    pub DownHyst: u8,
    pub VoltageDownHyst: u8,
    pub PowerThrottle: u8,
    pub SclkSetting: SMU_SclkSetting,
    pub ScksStretchThreshVid: [u8; NUM_SCKS_STATE_TYPES],
    pub Padding: u16,
}

pub type SMU75_Discrete_GraphicsLevel = SMU75_Discrete_GraphicsLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_ACPILevel {
    pub Flags: u32,
    pub MinVoltage: SMU_VoltageLevel,
    pub SclkFrequency: u32,
    pub SclkDid: u8,
    pub DisplayWatermark: u8,
    pub DeepSleepDivId: u8,
    pub padding: u8,
    pub CcPwrDynRm: u32,
    pub CcPwrDynRm1: u32,
    pub SclkSetting: SMU_SclkSetting,
}

pub type SMU75_Discrete_ACPILevel = SMU75_Discrete_ACPILevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_Ulv {
    pub CcPwrDynRm: u32,
    pub CcPwrDynRm1: u32,
    pub VddcOffset: u16,
    pub VddcOffsetVid: u8,
    pub VddcPhase: u8,
    pub BifSclkDfs: u16,
    pub Reserved: u16,
}

pub type SMU75_Discrete_Ulv = SMU75_Discrete_Ulv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_MemoryLevel {
    pub MinVoltage: SMU_VoltageLevel,
    pub MinMvdd: u32,
    pub MclkFrequency: u32,
    pub StutterEnable: u8,
    pub EnabledForThrottle: u8,
    pub EnabledForActivity: u8,
    pub padding_0: u8,
    pub UpHyst: u8,
    pub DownHyst: u8,
    pub VoltageDownHyst: u8,
    pub padding_1: u8,
    pub ActivityLevel: u16,
    pub DisplayWatermark: u8,
    pub padding_2: u8,
    pub Fcw_int: u16,
    pub Fcw_frac: u16,
    pub Postdiv: u8,
    pub padding_3: [u8; 3],
}

pub type SMU75_Discrete_MemoryLevel = SMU75_Discrete_MemoryLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_LinkLevel {
    pub PcieGenSpeed: u8,
    pub PcieLaneCount: u8,
    pub EnabledForActivity: u8,
    pub SPC: u8,
    pub DownThreshold: u32,
    pub UpThreshold: u32,
    pub BifSclkDfs: u16,
    pub Reserved: u16,
}

pub type SMU75_Discrete_LinkLevel = SMU75_Discrete_LinkLevel;
// MC ARB DRAM Timing registers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_MCArbDramTimingTableEntry {
    pub McArbDramTiming: u32,
    pub McArbDramTiming2: u32,
    pub McArbBurstTime: u32,
    pub McArbRfshRate: u32,
    pub McArbMisc3: u32,
}

pub type SMU75_Discrete_MCArbDramTimingTableEntry = SMU75_Discrete_MCArbDramTimingTableEntry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_MCArbDramTimingTable {
    pub entries: [SMU75_Discrete_MCArbDramTimingTableEntry; SMU__NUM_SCLK_DPM_STATE][SMU__NUM_MCLK_DPM_LEVELS],
}

pub type SMU75_Discrete_MCArbDramTimingTable = SMU75_Discrete_MCArbDramTimingTable;
// UVD VCLK/DCLK state (level) definition.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_UvdLevel {
    pub VclkFrequency: u32,
    pub DclkFrequency: u32,
    pub MinVoltage: SMU_VoltageLevel,
    pub VclkDivider: u8,
    pub DclkDivider: u8,
    pub padding: [u8; 2],
}

pub type SMU75_Discrete_UvdLevel = SMU75_Discrete_UvdLevel;
// Clocks for other external blocks (VCE, ACP, SAMU).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_ExtClkLevel {
    pub Frequency: u32,
    pub MinVoltage: SMU_VoltageLevel,
    pub Divider: u8,
    pub padding: [u8; 3],
}

pub type SMU75_Discrete_ExtClkLevel = SMU75_Discrete_ExtClkLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_StateInfo {
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

pub type SMU75_Discrete_StateInfo = SMU75_Discrete_StateInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_DpmTable {
    pub GraphicsPIDController: SMU75_PIDController,
    pub MemoryPIDController: SMU75_PIDController,
    pub LinkPIDController: SMU75_PIDController,
    pub SystemFlags: u32,
    pub VRConfig: u32,
    pub SmioMask1: u32,
    pub SmioMask2: u32,
    pub SmioTable1: SMIO_Table,
    pub SmioTable2: SMIO_Table,
    pub MvddLevelCount: u32,
    pub [SMU75_MAX_LEVELS_VDDC]: uint8_t BapmVddcVidHiSidd,
    pub [SMU75_MAX_LEVELS_VDDC]: uint8_t BapmVddcVidLoSidd,
    pub [SMU75_MAX_LEVELS_VDDC]: uint8_t BapmVddcVidHiSidd2,
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
    pub BootPhases: u8,
    pub VRHotLevel: u8,
    pub LdoRefSel: u8,
    pub Reserved1: [u8; 2],
    pub FanStartTemperature: u16,
    pub FanStopTemperature: u16,
    pub MaxVoltage: u16,
    pub Reserved2: u16,
    pub Reserved: u32,
    pub [SMU75_MAX_LEVELS_GRAPHICS]: SMU75_Discrete_GraphicsLevel GraphicsLevel,
    pub MemoryACPILevel: SMU75_Discrete_MemoryLevel,
    pub [SMU75_MAX_LEVELS_MEMORY]: SMU75_Discrete_MemoryLevel MemoryLevel,
    pub [SMU75_MAX_LEVELS_LINK]: SMU75_Discrete_LinkLevel LinkLevel,
    pub ACPILevel: SMU75_Discrete_ACPILevel,
    pub [SMU75_MAX_LEVELS_UVD]: SMU75_Discrete_UvdLevel UvdLevel,
    pub [SMU75_MAX_LEVELS_VCE]: SMU75_Discrete_ExtClkLevel VceLevel,
    pub [SMU75_MAX_LEVELS_ACP]: SMU75_Discrete_ExtClkLevel AcpLevel,
    pub [SMU75_MAX_LEVELS_SAMU]: SMU75_Discrete_ExtClkLevel SamuLevel,
    pub Ulv: SMU75_Discrete_Ulv,
    pub [SMU75_MAX_LEVELS_MEMORY][SMU75_MAX_LEVELS_GRAPHICS]: uint8_t DisplayWatermark,
    pub SclkStepSize: u32,
    pub [SMU75_MAX_ENTRIES_SMIO]: uint32_t Smio,
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
    pub [SMU75_DTE_ITERATIONS][SMU75_DTE_SOURCES][SMU75_DTE_SINKS]: uint16_t BAPMTI_R,
    pub [SMU75_DTE_ITERATIONS][SMU75_DTE_SOURCES][SMU75_DTE_SINKS]: uint16_t BAPMTI_RC,
    pub TemperatureLimitEdge: u16,
    pub TemperatureLimitHotspot: u16,
    pub BootVddc: u16,
    pub BootVddci: u16,
    pub FanGainEdge: u16,
    pub FanGainHotspot: u16,
    pub LowSclkInterruptThreshold: u32,
    pub VddGfxReChkWait: u32,
    pub ClockStretcherAmount: u8,
    pub Sclk_CKS_masterEn0_7: u8,
    pub Sclk_CKS_masterEn8_15: u8,
    pub DPMFreezeAndForced: u8,
    pub Sclk_voltageOffset: [u8; 8],
    pub ClockStretcherDataTable: SMU_ClockStretcherDataTable,
    pub CKS_LOOKUPTable: SMU_CKS_LOOKUPTable,
    pub CurrSclkPllRange: u32,
    pub SclkFcwRangeTable: [sclkFcwRange_t; NUM_SCLK_RANGE],
    pub BTCGB_VDROOP_TABLE: [GB_VDROOP_TABLE_t; BTCGB_VDROOP_TABLE_MAX_ENTRIES],
    pub AVFSGB_FUSE_TABLE: [SMU_QuadraticCoeffs; AVFSGB_VDROOP_TABLE_MAX_ENTRIES],
}

pub type SMU75_Discrete_DpmTable = SMU75_Discrete_DpmTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_FanTable {
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

pub type SMU75_Discrete_FanTable = SMU75_Discrete_FanTable;
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
    pub padding2: u8,
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
    pub [SMU75_MAX_LEVELS_MEMORY]: uint16_t LevelResidencyCounters,
    pub [SMU75_MAX_LEVELS_MEMORY]: uint16_t LevelSwitchCounters,
    pub (*TargetStateCalculator)(uint8_t): *mut c_void,
    pub (*SavedTargetStateCalculator)(uint8_t): *mut c_void,
    pub AutoDpmInterval: u16,
    pub AutoDpmRange: u16,
    pub VbiTimeoutCount: u16,
    pub MclkSwitchingTime: u16,
    pub fastSwitch: u8,
    pub Save_PIC_VDDGFX_EXIT: u8,
    pub Save_PIC_VDDGFX_ENTER: u8,
    pub VbiTimeout: u8,
    pub HbmTempRegBackup: u32,
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
    pub PowerSharingEnabled: u8,
    pub PowerSharingCounter: u8,
    pub PowerSharingINTEnabled: u8,
    pub GFXActivityCounterEnabled: u8,
    pub EnergyCount: u32,
    pub PSACTCount: u32,
    pub RollOverRequired: u8,
    pub RollOverCount: u8,
    pub padding: [u8; 2],
    pub HystControllerData: SMU7_HystController_Data,
}

pub type SMU7_PkgPwrLimitScoreboard = SMU7_PkgPwrLimitScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_BapmScoreboard {
    pub source_powers: [u32; SMU75_DTE_SOURCES],
    pub source_powers_last: [u32; SMU75_DTE_SOURCES],
    pub entity_temperatures: [i32; SMU75_NUM_GPU_TES],
    pub initial_entity_temperatures: [i32; SMU75_NUM_GPU_TES],
    pub Limit: i32,
    pub Hyst: i32,
    pub 2]: *mut *mut *mut *mut int32_t therm_influence_coeff_table[SMU75_DTE_ITERATIONS  SMU75_DTE_SOURCES  SMU75_DTE_SINKS,
    pub SMU75_DTE_SINKS]: *mut *mut *mut int32_t therm_node_table[SMU75_DTE_ITERATIONS  SMU75_DTE_SOURCES,
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
    pub D0Level: SMU75_Discrete_ACPILevel,
}

pub type SMU7_AcpiScoreboard = SMU7_AcpiScoreboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU75_Discrete_PmFuses {
    pub BapmVddCVidHiSidd: [u8; 8],
    pub BapmVddCVidLoSidd: [u8; 8],
    pub VddCVid: [u8; 8],
    pub SviLoadLineEn: u8,
    pub SviLoadLineVddC: u8,
    pub SviLoadLineTrimVddC: u8,
    pub SviLoadLineOffsetVddC: u8,
    pub TDC_VDDC_PkgLimit: u16,
    pub TDC_VDDC_ThrottleReleaseLimitPerc: u8,
    pub TDC_MAWt: u8,
    pub TdcWaterfallCtl: u8,
    pub LPMLTemperatureMin: u8,
    pub LPMLTemperatureMax: u8,
    pub Reserved: u8,
    pub LPMLTemperatureScaler: [u8; 16],
    pub FuzzyFan_ErrorSetDelta: i16,
    pub FuzzyFan_ErrorRateSetDelta: i16,
    pub FuzzyFan_PwmSetDelta: i16,
    pub Reserved6: u16,
    pub GnbLPML: [u8; 16],
    pub GnbLPMLMaxVid: u8,
    pub GnbLPMLMinVid: u8,
    pub Reserved1: [u8; 2],
    pub BapmVddCBaseLeakageHiSidd: u16,
    pub BapmVddCBaseLeakageLoSidd: u16,
    pub VFT_Temp: [u16; 3],
    pub Version: u8,
    pub padding: u8,
    pub VFT_ATE: [SMU_QuadraticCoeffs; 3],
    pub AVFS_GB: SMU_QuadraticCoeffs,
    pub ATE_ACBTC_GB: SMU_QuadraticCoeffs,
    pub P2V: SMU_QuadraticCoeffs,
    pub PsmCharzFreq: u32,
    pub InversionVoltage: u16,
    pub PsmCharzTemp: u16,
    pub EnabledAvfsModules: u32,
    pub BtcGbv_CksOff: SMU_QuadraticCoeffs,
}

pub type SMU75_Discrete_PmFuses = SMU75_Discrete_PmFuses;
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
    pub T_meas_max: [i32; SMU75_THERMAL_INPUT_LOOP_COUNT],
    pub T_meas_acc: [i32; SMU75_THERMAL_INPUT_LOOP_COUNT],
    pub I_calc_max: u32,
    pub I_calc_acc: u32,
    pub P_meas_acc: u32,
    pub V_meas_load_acc: u32,
    pub I_meas_acc: u32,
    pub P_meas_acc_vddci: u32,
    pub V_meas_load_acc_vddci: u32,
    pub I_meas_acc_vddci: u32,
    pub Sclk_dpm_residency: [u16; 8],
    pub Uvd_dpm_residency: [u16; 8],
    pub Vce_dpm_residency: [u16; 8],
    pub Mclk_dpm_residency: [u16; 4],
    pub P_roc_acc: u32,
    pub PkgPwr_max: u32,
    pub PkgPwr_acc: u32,
    pub MclkSwitchingTime_max: u32,
    pub MclkSwitchingTime_acc: u32,
    pub FanPwm_acc: u32,
    pub FanRpm_acc: u32,
    pub Gfx_busy_acc: u32,
    pub Mc_busy_acc: u32,
    pub Fps_acc: u32,
    pub AccCnt: u32,
}

pub type SMU7_Discrete_Pm_Status_Table = SMU7_Discrete_Pm_Status_Table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Discrete_AutoWattMan_Status_Table {
    pub T_meas_acc: [i32; SMU75_THERMAL_INPUT_LOOP_COUNT],
    pub Sclk_dpm_residency: [u16; 8],
    pub Mclk_dpm_residency: [u16; 4],
    pub TgpPwr_acc: u32,
    pub Gfx_busy_acc: u32,
    pub Mc_busy_acc: u32,
    pub AccCnt: u32,
}

pub type SMU7_Discrete_AutoWattMan_Status_Table = SMU7_Discrete_AutoWattMan_Status_Table;
pub const SMU7_MAX_GFX_CU_COUNT: c_int = 24;
pub const SMU7_MIN_GFX_CU_COUNT: c_int = 8;
pub const SMU7_GFX_CU_PG_ENABLE_DC_MAX_CU_SHIFT: c_int = 0;

pub const SMU7_GFX_CU_PG_ENABLE_AC_MAX_CU_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_GfxCuPgScoreboard {
    pub Enabled: u8,
    pub WaterfallUp: u8,
    pub WaterfallDown: u8,
    pub WaterfallLimit: u8,
    pub CurrMaxCu: u8,
    pub TargMaxCu: u8,
    pub ClampMode: u8,
    pub Active: u8,
    pub MaxSupportedCu: u8,
    pub MinSupportedCu: u8,
    pub PendingGfxCuHostInterrupt: u8,
    pub LastFilteredMaxCuInteger: u8,
    pub FilteredMaxCu: u16,
    pub FilteredMaxCuAlpha: u16,
    pub FilterResetCount: u16,
    pub FilterResetCountLimit: u16,
    pub ForceCu: u8,
    pub ForceCuCount: u8,
    pub AcModeMaxCu: u8,
    pub DcModeMaxCu: u8,
}

pub type SMU7_GfxCuPgScoreboard = SMU7_GfxCuPgScoreboard;
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
pub const SMU7_VDDCI_STROBE_PWR: c_int = 1331;
pub const SMU7_VDDR1_CONST: c_int = 693;
pub const SMU7_VDDR1_CAC_WEIGHT: c_int = 20;
pub const SMU7_VDDR1_CAC_WEIGHT_SHIFT: c_int = 19;
pub const SMU7_VDDR1_STROBE_PWR: c_int = 512;
pub const SMU7_AREA_COEFF_UVD: c_uint = 0xA78;
pub const SMU7_AREA_COEFF_VCE: c_uint = 0x190A;
pub const SMU7_AREA_COEFF_ACP: c_uint = 0x22D1;
pub const SMU7_AREA_COEFF_SAMU: c_uint = 0x534;
pub const SMU7_THERM_OUT_MODE_DISABLE: c_uint = 0x0;
pub const SMU7_THERM_OUT_MODE_THERM_ONLY: c_uint = 0x1;
pub const SMU7_THERM_OUT_MODE_THERM_VRHOT: c_uint = 0x2;
pub const SQ_Enable_MASK: c_uint = 0x1;
pub const SQ_IR_MASK: c_uint = 0x2;
pub const SQ_PCC_MASK: c_uint = 0x4;
pub const SQ_EDC_MASK: c_uint = 0x8;
pub const TCP_Enable_MASK: c_uint = 0x100;
pub const TCP_IR_MASK: c_uint = 0x200;
pub const TCP_PCC_MASK: c_uint = 0x400;
pub const TCP_EDC_MASK: c_uint = 0x800;
pub const TD_Enable_MASK: c_uint = 0x10000;
pub const TD_IR_MASK: c_uint = 0x20000;
pub const TD_PCC_MASK: c_uint = 0x40000;
pub const TD_EDC_MASK: c_uint = 0x80000;
pub const DB_Enable_MASK: c_uint = 0x1000000;
pub const DB_IR_MASK: c_uint = 0x2000000;
pub const DB_PCC_MASK: c_uint = 0x4000000;
pub const DB_EDC_MASK: c_uint = 0x8000000;
pub const SQ_Enable_SHIFT: c_int = 0;
pub const SQ_IR_SHIFT: c_int = 1;
pub const SQ_PCC_SHIFT: c_int = 2;
pub const SQ_EDC_SHIFT: c_int = 3;
pub const TCP_Enable_SHIFT: c_int = 8;
pub const TCP_IR_SHIFT: c_int = 9;
pub const TCP_PCC_SHIFT: c_int = 10;
pub const TCP_EDC_SHIFT: c_int = 11;
pub const TD_Enable_SHIFT: c_int = 16;
pub const TD_IR_SHIFT: c_int = 17;
pub const TD_PCC_SHIFT: c_int = 18;
pub const TD_EDC_SHIFT: c_int = 19;
pub const DB_Enable_SHIFT: c_int = 24;
pub const DB_IR_SHIFT: c_int = 25;
pub const DB_PCC_SHIFT: c_int = 26;
pub const DB_EDC_SHIFT: c_int = 27;
pub const PMFUSES_AVFSSIZE: c_int = 104;
pub const BTCGB0_Vdroop_Enable_MASK: c_uint = 0x1;
pub const BTCGB1_Vdroop_Enable_MASK: c_uint = 0x2;
pub const AVFSGB0_Vdroop_Enable_MASK: c_uint = 0x4;
pub const AVFSGB1_Vdroop_Enable_MASK: c_uint = 0x8;
pub const BTCGB0_Vdroop_Enable_SHIFT: c_int = 0;
pub const BTCGB1_Vdroop_Enable_SHIFT: c_int = 1;
pub const AVFSGB0_Vdroop_Enable_SHIFT: c_int = 2;
pub const AVFSGB1_Vdroop_Enable_SHIFT: c_int = 3;

