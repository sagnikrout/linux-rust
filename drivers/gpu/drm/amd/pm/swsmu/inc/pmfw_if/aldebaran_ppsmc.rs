//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/aldebaran_ppsmc.h
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
pub const PPSMC_MSG_GfxDriverReset: c_uint = 0x3;
pub const PPSMC_MSG_GetDriverIfVersion: c_uint = 0x4;
pub const PPSMC_MSG_spare1: c_uint = 0x5;
pub const PPSMC_MSG_spare2: c_uint = 0x6;
pub const PPSMC_MSG_EnableAllSmuFeatures: c_uint = 0x7;
pub const PPSMC_MSG_DisableAllSmuFeatures: c_uint = 0x8;
pub const PPSMC_MSG_spare3: c_uint = 0x9;
pub const PPSMC_MSG_spare4: c_uint = 0xA;
pub const PPSMC_MSG_spare5: c_uint = 0xB;
pub const PPSMC_MSG_spare6: c_uint = 0xC;
pub const PPSMC_MSG_GetEnabledSmuFeaturesLow: c_uint = 0xD;
pub const PPSMC_MSG_GetEnabledSmuFeaturesHigh: c_uint = 0xE;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0xF;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0x10;
pub const PPSMC_MSG_SetToolsDramAddrHigh: c_uint = 0x11;
pub const PPSMC_MSG_SetToolsDramAddrLow: c_uint = 0x12;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x13;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x14;
pub const PPSMC_MSG_UseDefaultPPTable: c_uint = 0x15;
pub const PPSMC_MSG_SetSystemVirtualDramAddrHigh: c_uint = 0x16;
pub const PPSMC_MSG_SetSystemVirtualDramAddrLow: c_uint = 0x17;
pub const PPSMC_MSG_SetSoftMinByFreq: c_uint = 0x18;
pub const PPSMC_MSG_SetSoftMaxByFreq: c_uint = 0x19;
pub const PPSMC_MSG_SetHardMinByFreq: c_uint = 0x1A;
pub const PPSMC_MSG_SetHardMaxByFreq: c_uint = 0x1B;
pub const PPSMC_MSG_GetMinDpmFreq: c_uint = 0x1C;
pub const PPSMC_MSG_GetMaxDpmFreq: c_uint = 0x1D;
pub const PPSMC_MSG_GetDpmFreqByIndex: c_uint = 0x1E;
pub const PPSMC_MSG_SetWorkloadMask: c_uint = 0x1F;
pub const PPSMC_MSG_GetVoltageByDpm: c_uint = 0x20;
pub const PPSMC_MSG_GetVoltageByDpmOverdrive: c_uint = 0x21;
pub const PPSMC_MSG_SetPptLimit: c_uint = 0x22;
pub const PPSMC_MSG_GetPptLimit: c_uint = 0x23;
pub const PPSMC_MSG_PrepareMp1ForUnload: c_uint = 0x24;
pub const PPSMC_MSG_PrepareMp1ForReset: c_uint = 0x25 //retired in 68.07;
pub const PPSMC_MSG_SoftReset: c_uint = 0x26 //retired in 68.07;
pub const PPSMC_MSG_RunDcBtc: c_uint = 0x27;
pub const PPSMC_MSG_DramLogSetDramAddrHigh: c_uint = 0x28;
pub const PPSMC_MSG_DramLogSetDramAddrLow: c_uint = 0x29;
pub const PPSMC_MSG_DramLogSetDramSize: c_uint = 0x2A;
pub const PPSMC_MSG_GetDebugData: c_uint = 0x2B;
pub const PPSMC_MSG_WaflTest: c_uint = 0x2C;
pub const PPSMC_MSG_spare7: c_uint = 0x2D;
pub const PPSMC_MSG_SetMemoryChannelEnable: c_uint = 0x2E;
pub const PPSMC_MSG_SetNumBadHbmPagesRetired: c_uint = 0x2F;
pub const PPSMC_MSG_DFCstateControl: c_uint = 0x32;
pub const PPSMC_MSG_GetGmiPwrDnHyst: c_uint = 0x33;
pub const PPSMC_MSG_SetGmiPwrDnHyst: c_uint = 0x34;
pub const PPSMC_MSG_GmiPwrDnControl: c_uint = 0x35;
pub const PPSMC_MSG_EnterGfxoff: c_uint = 0x36;
pub const PPSMC_MSG_ExitGfxoff: c_uint = 0x37;
pub const PPSMC_MSG_SetExecuteDMATest: c_uint = 0x38;
pub const PPSMC_MSG_EnableDeterminism: c_uint = 0x39;
pub const PPSMC_MSG_DisableDeterminism: c_uint = 0x3A;
pub const PPSMC_MSG_SetUclkDpmMode: c_uint = 0x3B;
// STB to dram log
pub const PPSMC_MSG_DumpSTBtoDram: c_uint = 0x3C;
pub const PPSMC_MSG_STBtoDramLogSetDramAddrHigh: c_uint = 0x3D;
pub const PPSMC_MSG_STBtoDramLogSetDramAddrLow: c_uint = 0x3E;
pub const PPSMC_MSG_STBtoDramLogSetDramSize: c_uint = 0x3F;
pub const PPSMC_MSG_SetSystemVirtualSTBtoDramAddrHigh: c_uint = 0x40;
pub const PPSMC_MSG_SetSystemVirtualSTBtoDramAddrLow: c_uint = 0x41;
pub const PPSMC_MSG_GfxDriverResetRecovery: c_uint = 0x42;
pub const PPSMC_MSG_BoardPowerCalibration: c_uint = 0x43;
pub const PPSMC_MSG_HeavySBR: c_uint = 0x45;
pub const PPSMC_MSG_SetBadHBMPagesRetiredFlagsPerChannel: c_uint = 0x46;
pub const PPSMC_Message_Count: c_uint = 0x47;
// PPSMC Reset Types
pub const PPSMC_RESET_TYPE_WARM_RESET: c_uint = 0x00;
pub const PPSMC_RESET_TYPE_DRIVER_MODE_1_RESET: c_uint = 0x01 //driver msg argument should be 1 for mode-1;
pub const PPSMC_RESET_TYPE_DRIVER_MODE_2_RESET: c_uint = 0x02 //and 2 for mode-2;
pub const PPSMC_RESET_TYPE_PCIE_LINK_RESET: c_uint = 0x03;
pub const PPSMC_RESET_TYPE_BIF_LINK_RESET: c_uint = 0x04;
pub const PPSMC_RESET_TYPE_PF0_FLR_RESET: c_uint = 0x05;
pub type PPSMC_Result = u32;
pub type PPSMC_Msg = u32;

