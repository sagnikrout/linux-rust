//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn401/dalsmc.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.
pub const DALSMC_VERSION: c_uint = 0x1;
// SMU Response Codes:
pub const DALSMC_Result_OK: c_uint = 0x1;
pub const DALSMC_Result_Failed: c_uint = 0xFF;
pub const DALSMC_Result_UnknownCmd: c_uint = 0xFE;
pub const DALSMC_Result_CmdRejectedPrereq: c_uint = 0xFD;
pub const DALSMC_Result_CmdRejectedBusy: c_uint = 0xFC;
// Message Definitions:
pub const DALSMC_MSG_TestMessage: c_uint = 0x1;
pub const DALSMC_MSG_GetSmuVersion: c_uint = 0x2;
pub const DALSMC_MSG_GetDriverIfVersion: c_uint = 0x3;
pub const DALSMC_MSG_GetMsgHeaderVersion: c_uint = 0x4;
pub const DALSMC_MSG_SetDalDramAddrHigh: c_uint = 0x5;
pub const DALSMC_MSG_SetDalDramAddrLow: c_uint = 0x6;
pub const DALSMC_MSG_TransferTableSmu2Dram: c_uint = 0x7;
pub const DALSMC_MSG_TransferTableDram2Smu: c_uint = 0x8;
pub const DALSMC_MSG_SetHardMinByFreq: c_uint = 0x9;
pub const DALSMC_MSG_SetHardMaxByFreq: c_uint = 0xA;
pub const DALSMC_MSG_GetDpmFreqByIndex: c_uint = 0xB;
pub const DALSMC_MSG_GetDcModeMaxDpmFreq: c_uint = 0xC;
pub const DALSMC_MSG_SetMinDeepSleepDcfclk: c_uint = 0xD;
pub const DALSMC_MSG_NumOfDisplays: c_uint = 0xE;
pub const DALSMC_MSG_SetExternalClientDfCstateAllow: c_uint = 0xF;
pub const DALSMC_MSG_BacoAudioD3PME: c_uint = 0x10;
pub const DALSMC_MSG_SetFclkSwitchAllow: c_uint = 0x11;
pub const DALSMC_MSG_SetCabForUclkPstate: c_uint = 0x12;
pub const DALSMC_MSG_SetWorstCaseUclkLatency: c_uint = 0x13;
pub const DALSMC_MSG_DcnExitReset: c_uint = 0x14;
pub const DALSMC_MSG_ReturnHardMinStatus: c_uint = 0x15;
pub const DALSMC_MSG_SetAlwaysWaitDmcubResp: c_uint = 0x16;
pub const DALSMC_MSG_IndicateDrrStatus: c_uint = 0x17  // PMFW 15811;
pub const DALSMC_MSG_ActiveUclkFclk: c_uint = 0x18;
pub const DALSMC_MSG_IdleUclkFclk: c_uint = 0x19;
pub const DALSMC_MSG_SetUclkPstateAllow: c_uint = 0x1A;
pub const DALSMC_MSG_SubvpUclkFclk: c_uint = 0x1B;
pub const DALSMC_MSG_GetNumUmcChannels: c_uint = 0x1C;
pub const DALSMC_Message_Count: c_uint = 0x1D;
