//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v14_0_2_ppsmc.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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
pub const PPSMC_VERSION: c_uint = 0x1;
// SMU Response Codes:
pub const PPSMC_Result_OK: c_uint = 0x1;
pub const PPSMC_Result_Failed: c_uint = 0xFF;
pub const PPSMC_Result_UnknownCmd: c_uint = 0xFE;
pub const PPSMC_Result_CmdRejectedPrereq: c_uint = 0xFD;
pub const PPSMC_Result_CmdRejectedBusy: c_uint = 0xFC;
// Message Definitions:
// BASIC
pub const PPSMC_MSG_TestMessage: c_uint = 0x1;
pub const PPSMC_MSG_GetSmuVersion: c_uint = 0x2;
pub const PPSMC_MSG_GetDriverIfVersion: c_uint = 0x3;
pub const PPSMC_MSG_SetAllowedFeaturesMaskLow: c_uint = 0x4;
pub const PPSMC_MSG_SetAllowedFeaturesMaskHigh: c_uint = 0x5;
pub const PPSMC_MSG_EnableAllSmuFeatures: c_uint = 0x6;
pub const PPSMC_MSG_DisableAllSmuFeatures: c_uint = 0x7;
pub const PPSMC_MSG_EnableSmuFeaturesLow: c_uint = 0x8;
pub const PPSMC_MSG_EnableSmuFeaturesHigh: c_uint = 0x9;
pub const PPSMC_MSG_DisableSmuFeaturesLow: c_uint = 0xA;
pub const PPSMC_MSG_DisableSmuFeaturesHigh: c_uint = 0xB;
pub const PPSMC_MSG_GetRunningSmuFeaturesLow: c_uint = 0xC;
pub const PPSMC_MSG_GetRunningSmuFeaturesHigh: c_uint = 0xD;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0xE;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0xF;
pub const PPSMC_MSG_SetToolsDramAddrHigh: c_uint = 0x10;
pub const PPSMC_MSG_SetToolsDramAddrLow: c_uint = 0x11;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x12;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x13;
pub const PPSMC_MSG_UseDefaultPPTable: c_uint = 0x14;
// BACO/BAMACO/BOMACO
pub const PPSMC_MSG_EnterBaco: c_uint = 0x15;
pub const PPSMC_MSG_ExitBaco: c_uint = 0x16;
pub const PPSMC_MSG_ArmD3: c_uint = 0x17;
pub const PPSMC_MSG_BacoAudioD3PME: c_uint = 0x18;
// DPM
pub const PPSMC_MSG_SetSoftMinByFreq: c_uint = 0x19;
pub const PPSMC_MSG_SetSoftMaxByFreq: c_uint = 0x1A;
pub const PPSMC_MSG_SetHardMinByFreq: c_uint = 0x1B;
pub const PPSMC_MSG_SetHardMaxByFreq: c_uint = 0x1C;
pub const PPSMC_MSG_GetMinDpmFreq: c_uint = 0x1D;
pub const PPSMC_MSG_GetMaxDpmFreq: c_uint = 0x1E;
pub const PPSMC_MSG_GetDpmFreqByIndex: c_uint = 0x1F;
pub const PPSMC_MSG_OverridePcieParameters: c_uint = 0x20;
// DramLog Set DramAddr
pub const PPSMC_MSG_DramLogSetDramAddrHigh: c_uint = 0x21;
pub const PPSMC_MSG_DramLogSetDramAddrLow: c_uint = 0x22;
pub const PPSMC_MSG_DramLogSetDramSize: c_uint = 0x23;
pub const PPSMC_MSG_SetWorkloadMask: c_uint = 0x24;
pub const PPSMC_MSG_GetVoltageByDpm: c_uint = 0x25 // Can be removed;
pub const PPSMC_MSG_SetVideoFps: c_uint = 0x26 // Can be removed;
pub const PPSMC_MSG_GetDcModeMaxDpmFreq: c_uint = 0x27;
// Power Gating
pub const PPSMC_MSG_AllowGfxOff: c_uint = 0x28;
pub const PPSMC_MSG_DisallowGfxOff: c_uint = 0x29;
pub const PPSMC_MSG_PowerUpVcn: c_uint = 0x2A;
pub const PPSMC_MSG_PowerDownVcn: c_uint = 0x2B;
pub const PPSMC_MSG_PowerUpJpeg: c_uint = 0x2C;
pub const PPSMC_MSG_PowerDownJpeg: c_uint = 0x2D;
// Resets
pub const PPSMC_MSG_PrepareMp1ForUnload: c_uint = 0x2E;
// Set SystemVirtual DramAddrHigh
pub const PPSMC_MSG_SetSystemVirtualDramAddrHigh: c_uint = 0x30;
pub const PPSMC_MSG_SetSystemVirtualDramAddrLow: c_uint = 0x31;
// ACDC Power Source
pub const PPSMC_MSG_SetPptLimit: c_uint = 0x32;
pub const PPSMC_MSG_GetPptLimit: c_uint = 0x33;
pub const PPSMC_MSG_ReenableAcDcInterrupt: c_uint = 0x34;
pub const PPSMC_MSG_NotifyPowerSource: c_uint = 0x35;
// BTC
pub const PPSMC_MSG_RunDcBtc: c_uint = 0x36;
// 0x37
// Others
pub const PPSMC_MSG_SetTemperatureInputSelect: c_uint = 0x38 // Can be removed;
pub const PPSMC_MSG_SetFwDstatesMask: c_uint = 0x39;
pub const PPSMC_MSG_SetThrottlerMask: c_uint = 0x3A;
pub const PPSMC_MSG_SetExternalClientDfCstateAllow: c_uint = 0x3B;
pub const PPSMC_MSG_SetMGpuFanBoostLimitRpm: c_uint = 0x3C;
// STB to dram log
pub const PPSMC_MSG_DumpSTBtoDram: c_uint = 0x3D;
pub const PPSMC_MSG_STBtoDramLogSetDramAddress: c_uint = 0x3E;
pub const PPSMC_MSG_DummyUndefined: c_uint = 0x3F;
pub const PPSMC_MSG_STBtoDramLogSetDramSize: c_uint = 0x40;
pub const PPSMC_MSG_SetOBMTraceBufferLogging: c_uint = 0x41;
pub const PPSMC_MSG_UseProfilingMode: c_uint = 0x42;
pub const PPSMC_MSG_AllowGfxDcs: c_uint = 0x43;
pub const PPSMC_MSG_DisallowGfxDcs: c_uint = 0x44;
pub const PPSMC_MSG_EnableAudioStutterWA: c_uint = 0x45;
pub const PPSMC_MSG_PowerUpUmsch: c_uint = 0x46;
pub const PPSMC_MSG_PowerDownUmsch: c_uint = 0x47;
pub const PPSMC_MSG_SetDcsArch: c_uint = 0x48;
pub const PPSMC_MSG_TriggerVFFLR: c_uint = 0x49;
pub const PPSMC_MSG_SetNumBadMemoryPagesRetired: c_uint = 0x4A;
pub const PPSMC_MSG_SetBadMemoryPagesRetiredFlagsPerChannel: c_uint = 0x4B;
pub const PPSMC_MSG_SetPriorityDeltaGain: c_uint = 0x4C;
pub const PPSMC_MSG_AllowIHHostInterrupt: c_uint = 0x4D;
pub const PPSMC_MSG_EnableShadowDpm: c_uint = 0x4E;
pub const PPSMC_MSG_Mode3Reset: c_uint = 0x4F;
pub const PPSMC_MSG_SetDriverDramAddr: c_uint = 0x50;
pub const PPSMC_MSG_SetToolsDramAddr: c_uint = 0x51;
pub const PPSMC_MSG_TransferTableSmu2DramWithAddr: c_uint = 0x52;
pub const PPSMC_MSG_TransferTableDram2SmuWithAddr: c_uint = 0x53;
pub const PPSMC_MSG_GetAllRunningSmuFeatures: c_uint = 0x54;
pub const PPSMC_MSG_GetSvi3Voltage: c_uint = 0x55;
pub const PPSMC_MSG_UpdatePolicy: c_uint = 0x56;
pub const PPSMC_MSG_ExtPwrConnSupport: c_uint = 0x57;
pub const PPSMC_MSG_PreloadSwPstateForUclkOverDrive: c_uint = 0x58;
pub const PPSMC_Message_Count: c_uint = 0x59;
