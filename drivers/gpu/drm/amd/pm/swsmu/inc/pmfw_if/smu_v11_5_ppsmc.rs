//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v11_5_ppsmc.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
// SMU Response Codes:
pub const PPSMC_Result_OK: c_uint = 0x1;
pub const PPSMC_Result_Failed: c_uint = 0xFF;
pub const PPSMC_Result_UnknownCmd: c_uint = 0xFE;
pub const PPSMC_Result_CmdRejectedPrereq: c_uint = 0xFD;
pub const PPSMC_Result_CmdRejectedBusy: c_uint = 0xFC;
// Message Definitions:
pub const PPSMC_MSG_TestMessage: c_uint = 0x1;
pub const PPSMC_MSG_GetSmuVersion: c_uint = 0x2;
pub const PPSMC_MSG_GetDriverIfVersion: c_uint = 0x3;
pub const PPSMC_MSG_EnableGfxOff: c_uint = 0x4;
pub const PPSMC_MSG_DisableGfxOff: c_uint = 0x5;
pub const PPSMC_MSG_PowerDownIspByTile: c_uint = 0x6 // ISP is power gated by default;
pub const PPSMC_MSG_PowerUpIspByTile: c_uint = 0x7;
pub const PPSMC_MSG_PowerDownVcn: c_uint = 0x8 // VCN is power gated by default;
pub const PPSMC_MSG_PowerUpVcn: c_uint = 0x9;
pub const PPSMC_MSG_RlcPowerNotify: c_uint = 0xA;
pub const PPSMC_MSG_SetHardMinVcn: c_uint = 0xB // For wireless display;
pub const PPSMC_MSG_SetSoftMinGfxclk: c_uint = 0xC //Sets SoftMin for GFXCLK. Arg is in MHz;
pub const PPSMC_MSG_ActiveProcessNotify: c_uint = 0xD;
pub const PPSMC_MSG_SetHardMinIspiclkByFreq: c_uint = 0xE;
pub const PPSMC_MSG_SetHardMinIspxclkByFreq: c_uint = 0xF;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0x10;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0x11;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x12;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x13;
pub const PPSMC_MSG_GfxDeviceDriverReset: c_uint = 0x14 //mode 2 reset during TDR;
pub const PPSMC_MSG_GetEnabledSmuFeatures: c_uint = 0x15;
pub const PPSMC_MSG_spare1: c_uint = 0x16;
pub const PPSMC_MSG_SetHardMinSocclkByFreq: c_uint = 0x17;
pub const PPSMC_MSG_SetSoftMinFclk: c_uint = 0x18 //Used to be PPSMC_MSG_SetMinVideoFclkFreq;
pub const PPSMC_MSG_SetSoftMinVcn: c_uint = 0x19;
pub const PPSMC_MSG_EnablePostCode: c_uint = 0x1A;
pub const PPSMC_MSG_GetGfxclkFrequency: c_uint = 0x1B;
pub const PPSMC_MSG_GetFclkFrequency: c_uint = 0x1C;
pub const PPSMC_MSG_AllowGfxOff: c_uint = 0x1D;
pub const PPSMC_MSG_DisallowGfxOff: c_uint = 0x1E;
pub const PPSMC_MSG_SetSoftMaxGfxClk: c_uint = 0x1F;
pub const PPSMC_MSG_SetHardMinGfxClk: c_uint = 0x20;
pub const PPSMC_MSG_SetSoftMaxSocclkByFreq: c_uint = 0x21;
pub const PPSMC_MSG_SetSoftMaxFclkByFreq: c_uint = 0x22;
pub const PPSMC_MSG_SetSoftMaxVcn: c_uint = 0x23;
pub const PPSMC_MSG_spare2: c_uint = 0x24;
pub const PPSMC_MSG_SetPowerLimitPercentage: c_uint = 0x25;
pub const PPSMC_MSG_PowerDownJpeg: c_uint = 0x26;
pub const PPSMC_MSG_PowerUpJpeg: c_uint = 0x27;
pub const PPSMC_MSG_SetHardMinFclkByFreq: c_uint = 0x28;
pub const PPSMC_MSG_SetSoftMinSocclkByFreq: c_uint = 0x29;
pub const PPSMC_MSG_PowerUpCvip: c_uint = 0x2A;
pub const PPSMC_MSG_PowerDownCvip: c_uint = 0x2B;
pub const PPSMC_MSG_GetPptLimit: c_uint = 0x2C;
pub const PPSMC_MSG_GetThermalLimit: c_uint = 0x2D;
pub const PPSMC_MSG_GetCurrentTemperature: c_uint = 0x2E;
pub const PPSMC_MSG_GetCurrentPower: c_uint = 0x2F;
pub const PPSMC_MSG_GetCurrentVoltage: c_uint = 0x30;
pub const PPSMC_MSG_GetCurrentCurrent: c_uint = 0x31;
pub const PPSMC_MSG_GetAverageCpuActivity: c_uint = 0x32;
pub const PPSMC_MSG_GetAverageGfxActivity: c_uint = 0x33;
pub const PPSMC_MSG_GetAveragePower: c_uint = 0x34;
pub const PPSMC_MSG_GetAverageTemperature: c_uint = 0x35;
pub const PPSMC_MSG_SetAveragePowerTimeConstant: c_uint = 0x36;
pub const PPSMC_MSG_SetAverageActivityTimeConstant: c_uint = 0x37;
pub const PPSMC_MSG_SetAverageTemperatureTimeConstant: c_uint = 0x38;
pub const PPSMC_MSG_SetMitigationEndHysteresis: c_uint = 0x39;
pub const PPSMC_MSG_GetCurrentFreq: c_uint = 0x3A;
pub const PPSMC_MSG_SetReducedPptLimit: c_uint = 0x3B;
pub const PPSMC_MSG_SetReducedThermalLimit: c_uint = 0x3C;
pub const PPSMC_MSG_DramLogSetDramAddr: c_uint = 0x3D;
pub const PPSMC_MSG_StartDramLogging: c_uint = 0x3E;
pub const PPSMC_MSG_StopDramLogging: c_uint = 0x3F;
pub const PPSMC_MSG_SetSoftMinCclk: c_uint = 0x40;
pub const PPSMC_MSG_SetSoftMaxCclk: c_uint = 0x41;
pub const PPSMC_MSG_SetDfPstateActiveLevel: c_uint = 0x42;
pub const PPSMC_MSG_SetDfPstateSoftMinLevel: c_uint = 0x43;
pub const PPSMC_MSG_SetCclkPolicy: c_uint = 0x44;
pub const PPSMC_MSG_DramLogSetDramAddrHigh: c_uint = 0x45;
pub const PPSMC_MSG_DramLogSetDramBufferSize: c_uint = 0x46;
pub const PPSMC_MSG_RequestActiveWgp: c_uint = 0x47;
pub const PPSMC_MSG_QueryActiveWgp: c_uint = 0x48;
pub const PPSMC_MSG_SetFastPPTLimit: c_uint = 0x49;
pub const PPSMC_MSG_SetSlowPPTLimit: c_uint = 0x4A;
pub const PPSMC_MSG_GetFastPPTLimit: c_uint = 0x4B;
pub const PPSMC_MSG_GetSlowPPTLimit: c_uint = 0x4C;
pub const PPSMC_MSG_GetGfxOffStatus: c_uint = 0x50;
pub const PPSMC_MSG_GetGfxOffEntryCount: c_uint = 0x51;
pub const PPSMC_MSG_GfxOffResidencyLogReadSample: c_uint = 0x52;
pub const PPSMC_MSG_StopGfxOffResidencyLogging: c_uint = 0x53;
pub const PPSMC_MSG_StartGfxOffResidencyLogging: c_uint = 0x56;
pub const PPSMC_Message_Count: c_uint = 0x57;
// Argument for PPSMC_MSG_GfxDeviceDriverReset
