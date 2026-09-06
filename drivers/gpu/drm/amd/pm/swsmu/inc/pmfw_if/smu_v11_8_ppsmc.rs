//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v11_8_ppsmc.h
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
// Copyright 2021 Advanced Micro Devices, Inc.
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
pub const PPSMC_MSG_SetDriverTableDramAddrHigh: c_uint = 0x4;
pub const PPSMC_MSG_SetDriverTableDramAddrLow: c_uint = 0x5;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x6;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x7;
pub const PPSMC_MSG_Rsvd1: c_uint = 0xA;
pub const PPSMC_MSG_RequestCorePstate: c_uint = 0xB;
pub const PPSMC_MSG_QueryCorePstate: c_uint = 0xC;
pub const PPSMC_MSG_Rsvd2: c_uint = 0xD;
pub const PPSMC_MSG_RequestGfxclk: c_uint = 0xE;
pub const PPSMC_MSG_QueryGfxclk: c_uint = 0xF;
pub const PPSMC_MSG_QueryVddcrSocClock: c_uint = 0x11;
pub const PPSMC_MSG_QueryDfPstate: c_uint = 0x13;
pub const PPSMC_MSG_Rsvd3: c_uint = 0x14;
pub const PPSMC_MSG_ConfigureS3PwrOffRegisterAddressHigh: c_uint = 0x16;
pub const PPSMC_MSG_ConfigureS3PwrOffRegisterAddressLow: c_uint = 0x17;
pub const PPSMC_MSG_RequestActiveWgp: c_uint = 0x18;
pub const PPSMC_MSG_SetMinDeepSleepGfxclkFreq: c_uint = 0x19;
pub const PPSMC_MSG_SetMaxDeepSleepDfllGfxDiv: c_uint = 0x1A;
pub const PPSMC_MSG_StartTelemetryReporting: c_uint = 0x1B;
pub const PPSMC_MSG_StopTelemetryReporting: c_uint = 0x1C;
pub const PPSMC_MSG_ClearTelemetryMax: c_uint = 0x1D;
pub const PPSMC_MSG_QueryActiveWgp: c_uint = 0x1E;
pub const PPSMC_MSG_SetCoreEnableMask: c_uint = 0x2C;
pub const PPSMC_MSG_InitiateGcRsmuSoftReset: c_uint = 0x2E;
pub const PPSMC_MSG_GfxCacWeightOperation: c_uint = 0x2F;
pub const PPSMC_MSG_L3CacWeightOperation: c_uint = 0x30;
pub const PPSMC_MSG_PackCoreCacWeight: c_uint = 0x31;
pub const PPSMC_MSG_SetDriverTableVMID: c_uint = 0x34;
pub const PPSMC_MSG_SetSoftMinCclk: c_uint = 0x35;
pub const PPSMC_MSG_SetSoftMaxCclk: c_uint = 0x36;
pub const PPSMC_MSG_GetGfxFrequency: c_uint = 0x37;
pub const PPSMC_MSG_GetGfxVid: c_uint = 0x38;
pub const PPSMC_MSG_ForceGfxFreq: c_uint = 0x39;
pub const PPSMC_MSG_UnForceGfxFreq: c_uint = 0x3A;
pub const PPSMC_MSG_ForceGfxVid: c_uint = 0x3B;
pub const PPSMC_MSG_UnforceGfxVid: c_uint = 0x3C;
pub const PPSMC_MSG_GetEnabledSmuFeatures: c_uint = 0x3D;
pub const PPSMC_Message_Count: c_uint = 0x3E;
