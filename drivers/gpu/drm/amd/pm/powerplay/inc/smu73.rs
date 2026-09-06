//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu73.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SID_OPTION {
    SID_OPTION_HI,
    SID_OPTION_LO,
    SID_OPTION_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Poly3rdOrderCoeff {
    LEAKAGE_TEMPERATURE_SCALAR,
    LEAKAGE_VOLTAGE_SCALAR,
    DYNAMIC_VOLTAGE_SCALAR,
    POLY_3RD_ORDER_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Poly3rdOrder_Data {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub a_shift: u8,
    pub b_shift: u8,
    pub c_shift: u8,
    pub x_shift: u8,
}

pub type SMU7_Poly3rdOrder_Data = SMU7_Poly3rdOrder_Data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Power_Calculator_Data {
    pub NoLoadVoltage: u16,
    pub LoadVoltage: u16,
    pub Resistance: u16,
    pub Temperature: u16,
    pub BaseLeakage: u16,
    pub LkgTempScalar: u16,
    pub LkgVoltScalar: u16,
    pub LkgAreaScalar: u16,
    pub LkgPower: u16,
    pub DynVoltScalar: u16,
    pub Cac: u32,
    pub DynPower: u32,
    pub TotalCurrent: u32,
    pub TotalPower: u32,
}

pub type PowerCalculatorData_t = Power_Calculator_Data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Gc_Cac_Weight_Data {
    pub index: u8,
    pub value: u32,
}

pub type GcCacWeight_Data = Gc_Cac_Weight_Data;
pub const SMU__NUM_SCLK_DPM_STATE: c_int = 8;
pub const SMU__NUM_MCLK_DPM_LEVELS: c_int = 4;
pub const SMU__NUM_LCLK_DPM_LEVELS: c_int = 8;
pub const SMU__NUM_PCIE_DPM_LEVELS: c_int = 8;
pub const SMU7_CONTEXT_ID_SMC: c_int = 1;
pub const SMU7_CONTEXT_ID_VBIOS: c_int = 2;
pub const SMU73_MAX_LEVELS_VDDC: c_int = 16;
pub const SMU73_MAX_LEVELS_VDDGFX: c_int = 16;
pub const SMU73_MAX_LEVELS_VDDCI: c_int = 8;
pub const SMU73_MAX_LEVELS_MVDD: c_int = 4;
pub const SMU_MAX_SMIO_LEVELS: c_int = 4;

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

// Virtualization Defines
pub const CG_XDMA_MASK: c_uint = 0x1;
pub const CG_XDMA_SHIFT: c_int = 0;
pub const CG_UVD_MASK: c_uint = 0x2;
pub const CG_UVD_SHIFT: c_int = 1;
pub const CG_VCE_MASK: c_uint = 0x4;
pub const CG_VCE_SHIFT: c_int = 2;
pub const CG_SAMU_MASK: c_uint = 0x8;
pub const CG_SAMU_SHIFT: c_int = 3;
pub const CG_GFX_MASK: c_uint = 0x10;
pub const CG_GFX_SHIFT: c_int = 4;
pub const CG_SDMA_MASK: c_uint = 0x20;
pub const CG_SDMA_SHIFT: c_int = 5;
pub const CG_HDP_MASK: c_uint = 0x40;
pub const CG_HDP_SHIFT: c_int = 6;
pub const CG_MC_MASK: c_uint = 0x80;
pub const CG_MC_SHIFT: c_int = 7;
pub const CG_DRM_MASK: c_uint = 0x100;
pub const CG_DRM_SHIFT: c_int = 8;
pub const CG_ROM_MASK: c_uint = 0x200;
pub const CG_ROM_SHIFT: c_int = 9;
pub const CG_BIF_MASK: c_uint = 0x400;
pub const CG_BIF_SHIFT: c_int = 10;
pub const SMU73_DTE_ITERATIONS: c_int = 5;
pub const SMU73_DTE_SOURCES: c_int = 3;
pub const SMU73_DTE_SINKS: c_int = 1;
pub const SMU73_NUM_CPU_TES: c_int = 0;
pub const SMU73_NUM_GPU_TES: c_int = 1;
pub const SMU73_NUM_NON_TES: c_int = 2;
pub const SMU73_DTE_FAN_SCALAR_MIN: c_uint = 0x100;
pub const SMU73_DTE_FAN_SCALAR_MAX: c_uint = 0x166;
pub const SMU73_DTE_FAN_TEMP_MAX: c_int = 93;
pub const SMU73_DTE_FAN_TEMP_MIN: c_int = 83;
pub const SMU73_THERMAL_INPUT_LOOP_COUNT: c_int = 6;
pub const SMU73_THERMAL_CLAMP_MODE_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_HystController_Data {
    pub waterfall_up: u16,
    pub waterfall_down: u16,
    pub waterfall_limit: u16,
    pub release_cnt: u16,
    pub release_limit: u16,
    pub spare: u16,
}

pub type SMU7_HystController_Data = SMU7_HystController_Data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU73_PIDController {
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

pub type SMU73_PIDController = SMU73_PIDController;
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
    pub spare2: u8,
    pub EnabledLevelsChange: u8,
    pub DteClampMode: u8,
    pub FpsClampMode: u8,
    pub LevelResidencyCounters: [u16; SMU73_MAX_LEVELS_GRAPHICS],
    pub LevelSwitchCounters: [u16; SMU73_MAX_LEVELS_GRAPHICS],
    pub (*TargetStateCalculator)(uint8_t): *mut c_void,
    pub (*SavedTargetStateCalculator)(uint8_t): *mut c_void,
    pub AutoDpmInterval: u16,
    pub AutoDpmRange: u16,
    pub FpsEnabled: u8,
    pub MaxPerfLevel: u8,
    pub AllowLowClkInterruptToHost: u8,
    pub FpsRunning: u8,
    pub MaxAllowedFrequency: u32,
    pub FilteredSclkFrequency: u32,
    pub LastSclkFrequency: u32,
    pub FilteredSclkFrequencyCnt: u32,
    pub LedEnable: u8,
    pub LedPin0: u8,
    pub LedPin1: u8,
    pub LedPin2: u8,
    pub LedAndMask: u32,
    pub FpsAlpha: u16,
    pub DeltaTime: u16,
    pub CurrentFps: u32,
    pub FilteredFps: u32,
    pub FrameCount: u32,
    pub FrameCountLast: u32,
    pub FpsTargetScalar: u16,
    pub FpsWaterfallLimitScalar: u16,
    pub FpsAlphaScalar: u16,
    pub spare8: u16,
    pub HystControllerData: SMU7_HystController_Data,
}

pub type SMU7_LocalDpmScoreboard = SMU7_LocalDpmScoreboard;
pub const SMU7_MAX_VOLTAGE_CLIENTS: c_int = 12;
extern "C" {
    pub fn uint8_t(_arg: *mut VoltageChangeHandler_t)(uint16_t, _arg: u8) -> typedef;
}
pub const VDDC_MASK: c_uint = 0x00007FFF;
pub const VDDC_SHIFT: c_int = 0;
pub const VDDCI_MASK: c_uint = 0x3FFF8000;
pub const VDDCI_SHIFT: c_int = 15;
pub const PHASES_MASK: c_uint = 0xC0000000;
pub const PHASES_SHIFT: c_int = 30;
pub type SMU_VoltageLevel = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_VoltageScoreboard {
    pub TargetVoltage: SMU_VoltageLevel,
    pub MaxVid: u16,
    pub HighestVidOffset: u8,
    pub CurrentVidOffset: u8,
    pub CurrentVddc: u16,
    pub CurrentVddci: u16,
    pub ControllerBusy: u8,
    pub CurrentVid: u8,
    pub CurrentVddciVid: u8,
    pub padding: u8,
    pub RequestedVoltage: [SMU_VoltageLevel; SMU7_MAX_VOLTAGE_CLIENTS],
    pub TargetVoltageState: SMU_VoltageLevel,
    pub EnabledRequest: [u8; SMU7_MAX_VOLTAGE_CLIENTS],
    pub padding2: u8,
    pub padding3: u8,
    pub ControllerEnable: u8,
    pub ControllerRunning: u8,
    pub CurrentStdVoltageHiSidd: u16,
    pub CurrentStdVoltageLoSidd: u16,
    pub OverrideVoltage: u8,
    pub padding4: u8,
    pub padding5: u8,
    pub CurrentPhases: u8,
    pub ChangeVddc: VoltageChangeHandler_t,
    pub ChangeVddci: VoltageChangeHandler_t,
    pub ChangePhase: VoltageChangeHandler_t,
    pub ChangeMvdd: VoltageChangeHandler_t,
    pub functionLinks: [VoltageChangeHandler_t; 6],
    pub VddcFollower1: *mut u16,
    pub Driver_OD_RequestedVidOffset1: i16,
    pub Driver_OD_RequestedVidOffset2: i16,
}

pub type SMU7_VoltageScoreboard = SMU7_VoltageScoreboard;
// -------------------------------------------------------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_PCIeLinkSpeedScoreboard {
    pub DpmEnable: u8,
    pub DpmRunning: u8,
    pub DpmForce: u8,
    pub DpmForceLevel: u8,
    pub CurrentLinkSpeed: u8,
    pub EnabledLevelsChange: u8,
    pub AutoDpmInterval: u16,
    pub AutoDpmRange: u16,
    pub AutoDpmCount: u16,
    pub DpmMode: u8,
    pub AcpiReq: u8,
    pub AcpiAck: u8,
    pub CurrentLinkLevel: u8,
}

pub type SMU7_PCIeLinkSpeedScoreboard = SMU7_PCIeLinkSpeedScoreboard;
// -------------------------------------------------------- CAC table ------------------------------------------------------
pub const SMU7_LKGE_LUT_NUM_OF_TEMP_ENTRIES: c_int = 16;
pub const SMU7_LKGE_LUT_NUM_OF_VOLT_ENTRIES: c_int = 16;
pub const SMU7_SCALE_I: c_int = 7;
pub const SMU7_SCALE_R: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_PowerScoreboard {
    pub GpuPower: u32,
    pub VddcPower: u32,
    pub VddcVoltage: u32,
    pub VddcCurrent: u32,
    pub MvddPower: u32,
    pub MvddVoltage: u32,
    pub MvddCurrent: u32,
    pub RocPower: u32,
    pub Telemetry_1_slope: u16,
    pub Telemetry_2_slope: u16,
    pub Telemetry_1_offset: i32,
    pub Telemetry_2_offset: i32,
}

pub type SMU7_PowerScoreboard = SMU7_PowerScoreboard;
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
pub struct SMU73_SoftRegisters {
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
    pub UVDDpmEnabledLevels: u8,
    pub SAMUDpmEnabledLevels: u8,
    pub ACPDpmEnabledLevels: u8,
    pub VCEDpmEnabledLevels: u8,
    pub DRAM_LOG_ADDR_H: u32,
    pub DRAM_LOG_ADDR_L: u32,
    pub DRAM_LOG_PHY_ADDR_H: u32,
    pub DRAM_LOG_PHY_ADDR_L: u32,
    pub DRAM_LOG_BUFF_SIZE: u32,
    pub UlvEnterCount: u32,
    pub UlvTime: u32,
    pub UcodeLoadStatus: u32,
    pub Reserved: [u32; 2],
}

pub type SMU73_SoftRegisters = SMU73_SoftRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU73_Firmware_Header {
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
    pub ClockStretcherTable: u32,
    pub Reserved: [u32; 41],
    pub Signature: u32,
}

pub type SMU73_Firmware_Header = SMU73_Firmware_Header;
pub const SMU7_FIRMWARE_HEADER_LOCATION: c_uint = 0x20000;
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

pub const MC_BLOCK_COUNT: c_int = 1;
pub const CPL_BLOCK_COUNT: c_int = 5;
pub const SE_BLOCK_COUNT: c_int = 15;
pub const GC_BLOCK_COUNT: c_int = 24;
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
    pub CplLocalCac: [SMU7_Local_Cac; CPL_BLOCK_COUNT],
    pub McLocalCac: [SMU7_Local_Cac; MC_BLOCK_COUNT],
    pub SeLocalCac: [SMU7_Local_Cac; SE_BLOCK_COUNT],
    pub GcLocalCac: [SMU7_Local_Cac; GC_BLOCK_COUNT],
}

pub type SMU7_Local_Cac_Table = SMU7_Local_Cac_Table;

// Description of Clock Gating bitmask for Tonga:
// System Clock Gating

pub const CG_SYS_BIF_MGLS_SHIFT: c_int = 0;
pub const CG_SYS_ROM_SHIFT: c_int = 1;
pub const CG_SYS_MC_MGCG_SHIFT: c_int = 2;
pub const CG_SYS_MC_MGLS_SHIFT: c_int = 3;
pub const CG_SYS_SDMA_MGCG_SHIFT: c_int = 4;
pub const CG_SYS_SDMA_MGLS_SHIFT: c_int = 5;
pub const CG_SYS_DRM_MGCG_SHIFT: c_int = 6;
pub const CG_SYS_HDP_MGCG_SHIFT: c_int = 7;
pub const CG_SYS_HDP_MGLS_SHIFT: c_int = 8;
pub const CG_SYS_DRM_MGLS_SHIFT: c_int = 9;
pub const CG_SYS_BIF_MGLS_MASK: c_uint = 0x1;
pub const CG_SYS_ROM_MASK: c_uint = 0x2;
pub const CG_SYS_MC_MGCG_MASK: c_uint = 0x4;
pub const CG_SYS_MC_MGLS_MASK: c_uint = 0x8;
pub const CG_SYS_SDMA_MGCG_MASK: c_uint = 0x10;
pub const CG_SYS_SDMA_MGLS_MASK: c_uint = 0x20;
pub const CG_SYS_DRM_MGCG_MASK: c_uint = 0x40;
pub const CG_SYS_HDP_MGCG_MASK: c_uint = 0x80;
pub const CG_SYS_HDP_MGLS_MASK: c_uint = 0x100;
pub const CG_SYS_DRM_MGLS_MASK: c_uint = 0x200;
// Graphics Clock Gating

pub const CG_GFX_CGCG_SHIFT: c_int = 16;
pub const CG_GFX_CGLS_SHIFT: c_int = 17;
pub const CG_CPF_MGCG_SHIFT: c_int = 18;
pub const CG_RLC_MGCG_SHIFT: c_int = 19;
pub const CG_GFX_OTHERS_MGCG_SHIFT: c_int = 20;
pub const CG_GFX_CGCG_MASK: c_uint = 0x00010000;
pub const CG_GFX_CGLS_MASK: c_uint = 0x00020000;
pub const CG_CPF_MGCG_MASK: c_uint = 0x00040000;
pub const CG_RLC_MGCG_MASK: c_uint = 0x00080000;
pub const CG_GFX_OTHERS_MGCG_MASK: c_uint = 0x00100000;
// Voltage Regulator Configuration
// VR Config info is contained in dpmTable.VRConfig
pub const VRCONF_VDDC_MASK: c_uint = 0x000000FF;
pub const VRCONF_VDDC_SHIFT: c_int = 0;
pub const VRCONF_VDDGFX_MASK: c_uint = 0x0000FF00;
pub const VRCONF_VDDGFX_SHIFT: c_int = 8;
pub const VRCONF_VDDCI_MASK: c_uint = 0x00FF0000;
pub const VRCONF_VDDCI_SHIFT: c_int = 16;
pub const VRCONF_MVDD_MASK: c_uint = 0xFF000000;
pub const VRCONF_MVDD_SHIFT: c_int = 24;
pub const VR_MERGED_WITH_VDDC: c_int = 0;
pub const VR_SVI2_PLANE_1: c_int = 1;
pub const VR_SVI2_PLANE_2: c_int = 2;
pub const VR_SMIO_PATTERN_1: c_int = 3;
pub const VR_SMIO_PATTERN_2: c_int = 4;
pub const VR_STATIC_VOLTAGE: c_int = 5;
// Clock Stretcher Configuration
pub const CLOCK_STRETCHER_MAX_ENTRIES: c_uint = 0x4;
pub const CKS_LOOKUPTable_MAX_ENTRIES: c_uint = 0x4;
// The 'settings' field is subdivided in the following way:
pub const CLOCK_STRETCHER_SETTING_DDT_MASK: c_uint = 0x01;
pub const CLOCK_STRETCHER_SETTING_DDT_SHIFT: c_uint = 0x0;
pub const CLOCK_STRETCHER_SETTING_STRETCH_AMOUNT_MASK: c_uint = 0x1E;
pub const CLOCK_STRETCHER_SETTING_STRETCH_AMOUNT_SHIFT: c_uint = 0x1;
pub const CLOCK_STRETCHER_SETTING_ENABLE_MASK: c_uint = 0x80;
pub const CLOCK_STRETCHER_SETTING_ENABLE_SHIFT: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_ClockStretcherDataTableEntry {
    pub minVID: u8,
    pub maxVID: u8,
    pub setting: u16,
}

pub type SMU_ClockStretcherDataTableEntry = SMU_ClockStretcherDataTableEntry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_ClockStretcherDataTable {
    pub ClockStretcherDataTableEntry: [SMU_ClockStretcherDataTableEntry; CLOCK_STRETCHER_MAX_ENTRIES],
}

pub type SMU_ClockStretcherDataTable = SMU_ClockStretcherDataTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_CKS_LOOKUPTableEntry {
    pub minFreq: u16,
    pub maxFreq: u16,
    pub setting: u8,
    pub padding: [u8; 3],
}

pub type SMU_CKS_LOOKUPTableEntry = SMU_CKS_LOOKUPTableEntry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_CKS_LOOKUPTable {
    pub CKS_LOOKUPTableEntry: [SMU_CKS_LOOKUPTableEntry; CKS_LOOKUPTable_MAX_ENTRIES],
}

pub type SMU_CKS_LOOKUPTable = SMU_CKS_LOOKUPTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AgmAvfsData_t {
    pub avgPsmCount: [u16; 28],
    pub minPsmCount: [u16; 28],
}

pub type AgmAvfsData_t = AgmAvfsData_t;
// AVFS DEFINES
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VFT_COLUMNS {
    SCLK0,
    SCLK1,
    SCLK2,
    SCLK3,
    SCLK4,
    SCLK5,
    SCLK6,
    SCLK7,

    NUM_VFT_COLUMNS
}

pub const TEMP_RANGE_MAXSTEPS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VFT_CELL_t {
    pub Voltage: u16,
}

pub type VFT_CELL_t = VFT_CELL_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VFT_TABLE_t {
    pub Cell: [VFT_CELL_t; TEMP_RANGE_MAXSTEPS][NUM_VFT_COLUMNS],
    pub AvfsGbv: [u16; NUM_VFT_COLUMNS],
    pub BtcGbv: [u16; NUM_VFT_COLUMNS],
    pub Temperature: [u16; TEMP_RANGE_MAXSTEPS],
    pub NumTemperatureSteps: u8,
    pub padding: [u8; 3],
}

pub type VFT_TABLE_t = VFT_TABLE_t;
