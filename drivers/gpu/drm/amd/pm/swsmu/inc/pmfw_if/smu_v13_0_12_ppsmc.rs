//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v13_0_12_ppsmc.h
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
pub const PPSMC_MSG_GfxDriverReset: c_uint = 0x3;
pub const PPSMC_MSG_GetDriverIfVersion: c_uint = 0x4;
pub const PPSMC_MSG_EnableAllSmuFeatures: c_uint = 0x5;
pub const PPSMC_MSG_DisableAllSmuFeatures: c_uint = 0x6;
pub const PPSMC_MSG_RequestI2cTransaction: c_uint = 0x7;
pub const PPSMC_MSG_GetMetricsVersion: c_uint = 0x8;
pub const PPSMC_MSG_GetMetricsTable: c_uint = 0x9;
pub const PPSMC_MSG_GetEccInfoTable: c_uint = 0xA;
pub const PPSMC_MSG_GetEnabledSmuFeaturesLow: c_uint = 0xB;
pub const PPSMC_MSG_GetEnabledSmuFeaturesHigh: c_uint = 0xC;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0xD;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0xE;
pub const PPSMC_MSG_SetToolsDramAddrHigh: c_uint = 0xF;
pub const PPSMC_MSG_SetToolsDramAddrLow: c_uint = 0x10;
pub const PPSMC_MSG_SetSystemVirtualDramAddrHigh: c_uint = 0x11;
pub const PPSMC_MSG_SetSystemVirtualDramAddrLow: c_uint = 0x12;
pub const PPSMC_MSG_SetSoftMinByFreq: c_uint = 0x13;
pub const PPSMC_MSG_SetSoftMaxByFreq: c_uint = 0x14;
pub const PPSMC_MSG_GetMinDpmFreq: c_uint = 0x15;
pub const PPSMC_MSG_GetMaxDpmFreq: c_uint = 0x16;
pub const PPSMC_MSG_GetDpmFreqByIndex: c_uint = 0x17;
pub const PPSMC_MSG_SetPptLimit: c_uint = 0x18;
pub const PPSMC_MSG_GetPptLimit: c_uint = 0x19;
pub const PPSMC_MSG_DramLogSetDramAddrHigh: c_uint = 0x1A;
pub const PPSMC_MSG_DramLogSetDramAddrLow: c_uint = 0x1B;
pub const PPSMC_MSG_DramLogSetDramSize: c_uint = 0x1C;
pub const PPSMC_MSG_GetDebugData: c_uint = 0x1D;
pub const PPSMC_MSG_HeavySBR: c_uint = 0x1E;
pub const PPSMC_MSG_SetNumBadHbmPagesRetired: c_uint = 0x1F;
pub const PPSMC_MSG_DFCstateControl: c_uint = 0x20;
pub const PPSMC_MSG_GetGmiPwrDnHyst: c_uint = 0x21;
pub const PPSMC_MSG_SetGmiPwrDnHyst: c_uint = 0x22;
pub const PPSMC_MSG_GmiPwrDnControl: c_uint = 0x23;
pub const PPSMC_MSG_EnterGfxoff: c_uint = 0x24;
pub const PPSMC_MSG_ExitGfxoff: c_uint = 0x25;
pub const PPSMC_MSG_EnableDeterminism: c_uint = 0x26;
pub const PPSMC_MSG_DisableDeterminism: c_uint = 0x27;
pub const PPSMC_MSG_DumpSTBtoDram: c_uint = 0x28;
pub const PPSMC_MSG_STBtoDramLogSetDramAddrHigh: c_uint = 0x29;
pub const PPSMC_MSG_STBtoDramLogSetDramAddrLow: c_uint = 0x2A;
pub const PPSMC_MSG_STBtoDramLogSetDramSize: c_uint = 0x2B;
pub const PPSMC_MSG_SetSystemVirtualSTBtoDramAddrHigh: c_uint = 0x2C;
pub const PPSMC_MSG_SetSystemVirtualSTBtoDramAddrLow: c_uint = 0x2D;
pub const PPSMC_MSG_GfxDriverResetRecovery: c_uint = 0x2E;
pub const PPSMC_MSG_TriggerVFFLR: c_uint = 0x2F;
pub const PPSMC_MSG_SetSoftMinGfxClk: c_uint = 0x30;
pub const PPSMC_MSG_SetSoftMaxGfxClk: c_uint = 0x31;
pub const PPSMC_MSG_GetMinGfxDpmFreq: c_uint = 0x32;
pub const PPSMC_MSG_GetMaxGfxDpmFreq: c_uint = 0x33;
pub const PPSMC_MSG_PrepareForDriverUnload: c_uint = 0x34;
pub const PPSMC_MSG_ReadThrottlerLimit: c_uint = 0x35;
pub const PPSMC_MSG_QueryValidMcaCount: c_uint = 0x36;
pub const PPSMC_MSG_McaBankDumpDW: c_uint = 0x37;
pub const PPSMC_MSG_GetCTFLimit: c_uint = 0x38;
pub const PPSMC_MSG_ClearMcaOnRead: c_uint = 0x39;
pub const PPSMC_MSG_QueryValidMcaCeCount: c_uint = 0x3A;
pub const PPSMC_MSG_McaBankCeDumpDW: c_uint = 0x3B;
pub const PPSMC_MSG_SelectPLPDMode: c_uint = 0x40;
pub const PPSMC_MSG_PmLogReadSample: c_uint = 0x41;
pub const PPSMC_MSG_PmLogGetTableVersion: c_uint = 0x42;
pub const PPSMC_MSG_RmaDueToBadPageThreshold: c_uint = 0x43;
pub const PPSMC_MSG_SetThrottlingPolicy: c_uint = 0x44;
pub const PPSMC_MSG_SetPhaseDetectCSBWThreshold: c_uint = 0x45;
pub const PPSMC_MSG_SetPhaseDetectFreqHigh: c_uint = 0x46;
pub const PPSMC_MSG_SetPhaseDetectFreqLow: c_uint = 0x47;
pub const PPSMC_MSG_SetPhaseDetectDownHysterisis: c_uint = 0x48;
pub const PPSMC_MSG_SetPhaseDetectAlphaX1e6: c_uint = 0x49;
pub const PPSMC_MSG_SetPhaseDetectOnOff: c_uint = 0x4A;
pub const PPSMC_MSG_GetPhaseDetectResidency: c_uint = 0x4B;
pub const PPSMC_MSG_UpdatePccWaitDecMaxStr: c_uint = 0x4C;
pub const PPSMC_MSG_ResetSDMA: c_uint = 0x4D;
pub const PPSMC_MSG_GetRasTableVersion: c_uint = 0x4E;
pub const PPSMC_MSG_GetBadPageCount: c_uint = 0x50;
pub const PPSMC_MSG_GetBadPageMcaAddress: c_uint = 0x51;
pub const PPSMC_MSG_SetTimestamp: c_uint = 0x53;
pub const PPSMC_MSG_SetTimestampHi: c_uint = 0x54;
pub const PPSMC_MSG_GetTimestamp: c_uint = 0x55;
pub const PPSMC_MSG_GetBadPageIpIdLoHi: c_uint = 0x57;
pub const PPSMC_MSG_EraseRasTable: c_uint = 0x58;
pub const PPSMC_MSG_GetStaticMetricsTable: c_uint = 0x59;
pub const PPSMC_MSG_ResetVfArbitersByIndex: c_uint = 0x5A;
pub const PPSMC_MSG_GetSystemMetricsTable: c_uint = 0x5C;
pub const PPSMC_MSG_GetSystemMetricsVersion: c_uint = 0x5D;
pub const PPSMC_MSG_ResetVCN: c_uint = 0x5E;
pub const PPSMC_MSG_SetFastPptLimit: c_uint = 0x5F;
pub const PPSMC_MSG_GetFastPptLimit: c_uint = 0x60;
pub const PPSMC_Message_Count: c_uint = 0x61;
// PPSMC Reset Types for driver msg argument
pub const PPSMC_RESET_TYPE_DRIVER_MODE_1_RESET: c_uint = 0x1;
pub const PPSMC_RESET_TYPE_DRIVER_MODE_2_RESET: c_uint = 0x2;
pub const PPSMC_RESET_TYPE_DRIVER_MODE_3_RESET: c_uint = 0x3;
// PPSMC Reset Types for driver msg argument
pub const PPSMC_THROTTLING_LIMIT_TYPE_SOCKET: c_uint = 0x1;
pub const PPSMC_THROTTLING_LIMIT_TYPE_HBM: c_uint = 0x2;
// CTF/Throttle Limit types
pub const PPSMC_AID_THM_TYPE: c_uint = 0x1;
pub const PPSMC_CCD_THM_TYPE: c_uint = 0x2;
pub const PPSMC_XCD_THM_TYPE: c_uint = 0x3;
pub const PPSMC_HBM_THM_TYPE: c_uint = 0x4;
// PLPD modes
pub const PPSMC_PLPD_MODE_DEFAULT: c_uint = 0x1;
pub const PPSMC_PLPD_MODE_OPTIMIZED: c_uint = 0x2;
pub type PPSMC_Result = u32;
pub type PPSMC_MSG = u32;
