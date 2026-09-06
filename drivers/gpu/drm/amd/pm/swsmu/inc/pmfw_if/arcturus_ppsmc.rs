//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/arcturus_ppsmc.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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
pub const PPSMC_MSG_GetEnabledSmuFeaturesLow: c_uint = 0xC;
pub const PPSMC_MSG_GetEnabledSmuFeaturesHigh: c_uint = 0xD;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0xE;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0xF;
pub const PPSMC_MSG_SetToolsDramAddrHigh: c_uint = 0x10;
pub const PPSMC_MSG_SetToolsDramAddrLow: c_uint = 0x11;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x12;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x13;
pub const PPSMC_MSG_UseDefaultPPTable: c_uint = 0x14;
pub const PPSMC_MSG_UseBackupPPTable: c_uint = 0x15;
pub const PPSMC_MSG_SetSystemVirtualDramAddrHigh: c_uint = 0x16;
pub const PPSMC_MSG_SetSystemVirtualDramAddrLow: c_uint = 0x17;
// BACO/BAMACO/BOMACO
pub const PPSMC_MSG_EnterBaco: c_uint = 0x18;
pub const PPSMC_MSG_ExitBaco: c_uint = 0x19;
pub const PPSMC_MSG_ArmD3: c_uint = 0x1A;
// DPM
pub const PPSMC_MSG_SetSoftMinByFreq: c_uint = 0x1B;
pub const PPSMC_MSG_SetSoftMaxByFreq: c_uint = 0x1C;
pub const PPSMC_MSG_SetHardMinByFreq: c_uint = 0x1D;
pub const PPSMC_MSG_SetHardMaxByFreq: c_uint = 0x1E;
pub const PPSMC_MSG_GetMinDpmFreq: c_uint = 0x1F;
pub const PPSMC_MSG_GetMaxDpmFreq: c_uint = 0x20;
pub const PPSMC_MSG_GetDpmFreqByIndex: c_uint = 0x21;
pub const PPSMC_MSG_SetWorkloadMask: c_uint = 0x22;
pub const PPSMC_MSG_SetDfSwitchType: c_uint = 0x23;
pub const PPSMC_MSG_GetVoltageByDpm: c_uint = 0x24;
pub const PPSMC_MSG_GetVoltageByDpmOverdrive: c_uint = 0x25;
pub const PPSMC_MSG_SetPptLimit: c_uint = 0x26;
pub const PPSMC_MSG_GetPptLimit: c_uint = 0x27;
// Power Gating
pub const PPSMC_MSG_PowerUpVcn0: c_uint = 0x28;
pub const PPSMC_MSG_PowerDownVcn0: c_uint = 0x29;
pub const PPSMC_MSG_PowerUpVcn1: c_uint = 0x2A;
pub const PPSMC_MSG_PowerDownVcn1: c_uint = 0x2B;
// Resets and reload
pub const PPSMC_MSG_PrepareMp1ForUnload: c_uint = 0x2C;
pub const PPSMC_MSG_PrepareMp1ForReset: c_uint = 0x2D;
pub const PPSMC_MSG_PrepareMp1ForShutdown: c_uint = 0x2E;
pub const PPSMC_MSG_SoftReset: c_uint = 0x2F;
// BTC
pub const PPSMC_MSG_RunAfllBtc: c_uint = 0x30;
pub const PPSMC_MSG_RunDcBtc: c_uint = 0x31;
// Debug
pub const PPSMC_MSG_DramLogSetDramAddrHigh: c_uint = 0x33;
pub const PPSMC_MSG_DramLogSetDramAddrLow: c_uint = 0x34;
pub const PPSMC_MSG_DramLogSetDramSize: c_uint = 0x35;
pub const PPSMC_MSG_GetDebugData: c_uint = 0x36;
// WAFL and XGMI
pub const PPSMC_MSG_WaflTest: c_uint = 0x37;
pub const PPSMC_MSG_SetXgmiMode: c_uint = 0x38;
// Others
pub const PPSMC_MSG_SetMemoryChannelEnable: c_uint = 0x39;
// OOB
pub const PPSMC_MSG_SetNumBadHbmPagesRetired: c_uint = 0x3A;
pub const PPSMC_MSG_DFCstateControl: c_uint = 0x3B;
pub const PPSMC_MSG_GmiPwrDnControl: c_uint = 0x3D;
pub const PPSMC_Message_Count: c_uint = 0x3E;
pub const PPSMC_MSG_ReadSerialNumTop32: c_uint = 0x40;
pub const PPSMC_MSG_ReadSerialNumBottom32: c_uint = 0x41;
// parameter for MSG_LightSBR
// 1 -- Enable light secondary bus reset, only do nbio respond without further handling,
// leave driver to handle the real reset
// 0 -- Disable LightSBR, default behavior, SMU will pass the reset to PSP
//
pub const PPSMC_MSG_LightSBR: c_uint = 0x42;
pub type PPSMC_Result = u32;
pub type PPSMC_Msg = u32;

