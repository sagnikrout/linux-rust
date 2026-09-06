//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/vega10_ppsmc.h
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

pub const SMU_UCODE_VERSION: c_uint = 0x001c0800;
// SMU Response Codes:
pub const PPSMC_Result_OK: c_uint = 0x1;
pub const PPSMC_Result_Failed: c_uint = 0xFF;
pub const PPSMC_Result_UnknownCmd: c_uint = 0xFE;
pub const PPSMC_Result_CmdRejectedPrereq: c_uint = 0xFD;
pub const PPSMC_Result_CmdRejectedBusy: c_uint = 0xFC;
pub type PPSMC_Result = u16;
// Message Definitions
pub const PPSMC_MSG_TestMessage: c_uint = 0x1;
pub const PPSMC_MSG_GetSmuVersion: c_uint = 0x2;
pub const PPSMC_MSG_GetDriverIfVersion: c_uint = 0x3;
pub const PPSMC_MSG_EnableSmuFeatures: c_uint = 0x4;
pub const PPSMC_MSG_DisableSmuFeatures: c_uint = 0x5;
pub const PPSMC_MSG_GetEnabledSmuFeatures: c_uint = 0x6;
pub const PPSMC_MSG_SetWorkloadMask: c_uint = 0x7;
pub const PPSMC_MSG_SetPptLimit: c_uint = 0x8;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0x9;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0xA;
pub const PPSMC_MSG_SetToolsDramAddrHigh: c_uint = 0xB;
pub const PPSMC_MSG_SetToolsDramAddrLow: c_uint = 0xC;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0xD;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0xE;
pub const PPSMC_MSG_UseDefaultPPTable: c_uint = 0xF;
pub const PPSMC_MSG_UseBackupPPTable: c_uint = 0x10;
pub const PPSMC_MSG_RunBtc: c_uint = 0x11;
pub const PPSMC_MSG_RequestI2CBus: c_uint = 0x12;
pub const PPSMC_MSG_ReleaseI2CBus: c_uint = 0x13;
pub const PPSMC_MSG_ConfigureTelemetry: c_uint = 0x14;
pub const PPSMC_MSG_SetUlvIpMask: c_uint = 0x15;
pub const PPSMC_MSG_SetSocVidOffset: c_uint = 0x16;
pub const PPSMC_MSG_SetMemVidOffset: c_uint = 0x17;
pub const PPSMC_MSG_GetSocVidOffset: c_uint = 0x18;
pub const PPSMC_MSG_GetMemVidOffset: c_uint = 0x19;
pub const PPSMC_MSG_SetFloorSocVoltage: c_uint = 0x1A;
pub const PPSMC_MSG_SoftReset: c_uint = 0x1B;
pub const PPSMC_MSG_StartBacoMonitor: c_uint = 0x1C;
pub const PPSMC_MSG_CancelBacoMonitor: c_uint = 0x1D;
pub const PPSMC_MSG_EnterBaco: c_uint = 0x1E;
pub const PPSMC_MSG_AllowLowGfxclkInterrupt: c_uint = 0x1F;
pub const PPSMC_MSG_SetLowGfxclkInterruptThreshold: c_uint = 0x20;
pub const PPSMC_MSG_SetSoftMinGfxclkByIndex: c_uint = 0x21;
pub const PPSMC_MSG_SetSoftMaxGfxclkByIndex: c_uint = 0x22;
pub const PPSMC_MSG_GetCurrentGfxclkIndex: c_uint = 0x23;
pub const PPSMC_MSG_SetSoftMinUclkByIndex: c_uint = 0x24;
pub const PPSMC_MSG_SetSoftMaxUclkByIndex: c_uint = 0x25;
pub const PPSMC_MSG_GetCurrentUclkIndex: c_uint = 0x26;
pub const PPSMC_MSG_SetSoftMinUvdByIndex: c_uint = 0x27;
pub const PPSMC_MSG_SetSoftMaxUvdByIndex: c_uint = 0x28;
pub const PPSMC_MSG_GetCurrentUvdIndex: c_uint = 0x29;
pub const PPSMC_MSG_SetSoftMinVceByIndex: c_uint = 0x2A;
pub const PPSMC_MSG_SetSoftMaxVceByIndex: c_uint = 0x2B;
pub const PPSMC_MSG_SetHardMinVceByIndex: c_uint = 0x2C;
pub const PPSMC_MSG_GetCurrentVceIndex: c_uint = 0x2D;
pub const PPSMC_MSG_SetSoftMinSocclkByIndex: c_uint = 0x2E;
pub const PPSMC_MSG_SetHardMinSocclkByIndex: c_uint = 0x2F;
pub const PPSMC_MSG_SetSoftMaxSocclkByIndex: c_uint = 0x30;
pub const PPSMC_MSG_GetCurrentSocclkIndex: c_uint = 0x31;
pub const PPSMC_MSG_SetMinLinkDpmByIndex: c_uint = 0x32;
pub const PPSMC_MSG_GetCurrentLinkIndex: c_uint = 0x33;
pub const PPSMC_MSG_GetAverageGfxclkFrequency: c_uint = 0x34;
pub const PPSMC_MSG_GetAverageSocclkFrequency: c_uint = 0x35;
pub const PPSMC_MSG_GetAverageUclkFrequency: c_uint = 0x36;
pub const PPSMC_MSG_GetAverageGfxActivity: c_uint = 0x37;
pub const PPSMC_MSG_GetTemperatureEdge: c_uint = 0x38;
pub const PPSMC_MSG_GetTemperatureHotspot: c_uint = 0x39;
pub const PPSMC_MSG_GetTemperatureHBM: c_uint = 0x3A;
pub const PPSMC_MSG_GetTemperatureVrSoc: c_uint = 0x3B;
pub const PPSMC_MSG_GetTemperatureVrMem: c_uint = 0x3C;
pub const PPSMC_MSG_GetTemperatureLiquid: c_uint = 0x3D;
pub const PPSMC_MSG_GetTemperaturePlx: c_uint = 0x3E;
pub const PPSMC_MSG_OverDriveSetPercentage: c_uint = 0x3F;
pub const PPSMC_MSG_SetMinDeepSleepDcefclk: c_uint = 0x40;
pub const PPSMC_MSG_SwitchToAC: c_uint = 0x41;
pub const PPSMC_MSG_SetUclkFastSwitch: c_uint = 0x42;
pub const PPSMC_MSG_SetUclkDownHyst: c_uint = 0x43;
pub const PPSMC_MSG_RemoveDCClamp: c_uint = 0x44;
pub const PPSMC_MSG_GfxDeviceDriverReset: c_uint = 0x45;
pub const PPSMC_MSG_GetCurrentRpm: c_uint = 0x46;
pub const PPSMC_MSG_SetVideoFps: c_uint = 0x47;
pub const PPSMC_MSG_SetCustomGfxDpmParameters: c_uint = 0x48;
pub const PPSMC_MSG_SetTjMax: c_uint = 0x49;
pub const PPSMC_MSG_SetFanTemperatureTarget: c_uint = 0x4A;
pub const PPSMC_MSG_PrepareMp1ForUnload: c_uint = 0x4B;
pub const PPSMC_MSG_RequestDisplayClockByFreq: c_uint = 0x4C;
pub const PPSMC_MSG_GetClockFreqMHz: c_uint = 0x4D;
pub const PPSMC_MSG_DramLogSetDramAddrHigh: c_uint = 0x4E;
pub const PPSMC_MSG_DramLogSetDramAddrLow: c_uint = 0x4F;
pub const PPSMC_MSG_DramLogSetDramSize: c_uint = 0x50;
pub const PPSMC_MSG_SetFanMaxRpm: c_uint = 0x51;
pub const PPSMC_MSG_SetFanMinPwm: c_uint = 0x52;
pub const PPSMC_MSG_ConfigureGfxDidt: c_uint = 0x55;
pub const PPSMC_MSG_NumOfDisplays: c_uint = 0x56;
pub const PPSMC_MSG_ReadSerialNumTop32: c_uint = 0x58;
pub const PPSMC_MSG_ReadSerialNumBottom32: c_uint = 0x59;
pub const PPSMC_MSG_SetSystemVirtualDramAddrHigh: c_uint = 0x5A;
pub const PPSMC_MSG_SetSystemVirtualDramAddrLow: c_uint = 0x5B;
pub const PPSMC_MSG_RunAcgBtc: c_uint = 0x5C;
pub const PPSMC_MSG_RunAcgInClosedLoop: c_uint = 0x5D;
pub const PPSMC_MSG_RunAcgInOpenLoop: c_uint = 0x5E;
pub const PPSMC_MSG_InitializeAcg: c_uint = 0x5F;
pub const PPSMC_MSG_GetCurrPkgPwr: c_uint = 0x61;
pub const PPSMC_MSG_GetAverageGfxclkActualFrequency: c_uint = 0x63;
pub const PPSMC_MSG_SetPccThrottleLevel: c_uint = 0x67;
pub const PPSMC_MSG_UpdatePkgPwrPidAlpha: c_uint = 0x68;
pub const PPSMC_Message_Count: c_uint = 0x69;
pub type PPSMC_Msg = c_int;

