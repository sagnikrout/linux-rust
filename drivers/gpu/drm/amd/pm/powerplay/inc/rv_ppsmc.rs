//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/rv_ppsmc.h
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

pub const PPSMC_Result_OK: c_uint = 0x1;
pub const PPSMC_Result_Failed: c_uint = 0xFF;
pub const PPSMC_Result_UnknownCmd: c_uint = 0xFE;
pub const PPSMC_Result_CmdRejectedPrereq: c_uint = 0xFD;
pub const PPSMC_Result_CmdRejectedBusy: c_uint = 0xFC;
pub const PPSMC_MSG_TestMessage: c_uint = 0x1;
pub const PPSMC_MSG_GetSmuVersion: c_uint = 0x2;
pub const PPSMC_MSG_GetDriverIfVersion: c_uint = 0x3;
pub const PPSMC_MSG_PowerUpGfx: c_uint = 0x6;
pub const PPSMC_MSG_EnableGfxOff: c_uint = 0x7;
pub const PPSMC_MSG_DisableGfxOff: c_uint = 0x8;
pub const PPSMC_MSG_PowerDownIspByTile: c_uint = 0x9;
pub const PPSMC_MSG_PowerUpIspByTile: c_uint = 0xA;
pub const PPSMC_MSG_PowerDownVcn: c_uint = 0xB;
pub const PPSMC_MSG_PowerUpVcn: c_uint = 0xC;
pub const PPSMC_MSG_PowerDownSdma: c_uint = 0xD;
pub const PPSMC_MSG_PowerUpSdma: c_uint = 0xE;
pub const PPSMC_MSG_SetHardMinIspclkByFreq: c_uint = 0xF;
pub const PPSMC_MSG_SetHardMinVcn: c_uint = 0x10;
pub const PPSMC_MSG_SetMinDisplayClock: c_uint = 0x11;
pub const PPSMC_MSG_SetHardMinFclkByFreq: c_uint = 0x12;
pub const PPSMC_MSG_SetAllowFclkSwitch: c_uint = 0x13;
pub const PPSMC_MSG_SetMinVideoGfxclkFreq: c_uint = 0x14;
pub const PPSMC_MSG_ActiveProcessNotify: c_uint = 0x15;
pub const PPSMC_MSG_SetCustomPolicy: c_uint = 0x16;
pub const PPSMC_MSG_SetVideoFps: c_uint = 0x17;
pub const PPSMC_MSG_SetDisplayCount: c_uint = 0x18;
pub const PPSMC_MSG_QueryPowerLimit: c_uint = 0x19;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0x1A;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0x1B;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x1C;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x1D;
pub const PPSMC_MSG_DeviceDriverReset: c_uint = 0x1E;
pub const PPSMC_MSG_SetGfxclkOverdriveByFreqVid: c_uint = 0x1F;
pub const PPSMC_MSG_SetHardMinDcefclkByFreq: c_uint = 0x20;
pub const PPSMC_MSG_SetHardMinSocclkByFreq: c_uint = 0x21;
pub const PPSMC_MSG_SetMinVddcrSocVoltage: c_uint = 0x22;
pub const PPSMC_MSG_SetMinVideoFclkFreq: c_uint = 0x23;
pub const PPSMC_MSG_SetMinDeepSleepDcefclk: c_uint = 0x24;
pub const PPSMC_MSG_ForcePowerDownGfx: c_uint = 0x25;
pub const PPSMC_MSG_SetPhyclkVoltageByFreq: c_uint = 0x26;
pub const PPSMC_MSG_SetDppclkVoltageByFreq: c_uint = 0x27;
pub const PPSMC_MSG_SetSoftMinVcn: c_uint = 0x28;
pub const PPSMC_MSG_GetGfxclkFrequency: c_uint = 0x2A;
pub const PPSMC_MSG_GetFclkFrequency: c_uint = 0x2B;
pub const PPSMC_MSG_GetMinGfxclkFrequency: c_uint = 0x2C;
pub const PPSMC_MSG_GetMaxGfxclkFrequency: c_uint = 0x2D;
pub const PPSMC_MSG_SoftReset: c_uint = 0x2E;
pub const PPSMC_MSG_SetGfxCGPG: c_uint = 0x2F;
pub const PPSMC_MSG_SetSoftMaxGfxClk: c_uint = 0x30;
pub const PPSMC_MSG_SetHardMinGfxClk: c_uint = 0x31;
pub const PPSMC_MSG_SetSoftMaxSocclkByFreq: c_uint = 0x32;
pub const PPSMC_MSG_SetSoftMaxFclkByFreq: c_uint = 0x33;
pub const PPSMC_MSG_SetSoftMaxVcn: c_uint = 0x34;
pub const PPSMC_MSG_PowerGateMmHub: c_uint = 0x35;
pub const PPSMC_MSG_SetRccPfcPmeRestoreRegister: c_uint = 0x36;
pub const PPSMC_MSG_GpuChangeState: c_uint = 0x37;
pub const PPSMC_MSG_GetGfxBusy: c_uint = 0x3D;
pub const PPSMC_Message_Count: c_uint = 0x42;
pub type PPSMC_Result = u16;
pub type PPSMC_Msg = c_int;

