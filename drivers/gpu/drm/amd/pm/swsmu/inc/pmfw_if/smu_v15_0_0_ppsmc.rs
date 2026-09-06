//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v15_0_0_ppsmc.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
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
// @file ppsmc.h
//
// @brief pplib (driver/host) to PMFW Interface
//
// Clients:  Driver/Host via pplib.
// Protocols:
//
// @date 2016 - 2024
//
// ! @mainpage PMFW-PPS (PPLib) Message Interface
//
// @def PPS_PMFW_IF_VER
// PPS (PPLib) to PMFW IF version 1.0
//

// @defgroup ResponseCodes PMFW Response Codes
// @{
//
// SMU Response Codes:
pub const PPSMC_Result_OK: c_uint = 0x1  ///< Message Response OK;
pub const PPSMC_Result_Failed: c_uint = 0xFF ///< Message Response Failed;
pub const PPSMC_Result_UnknownCmd: c_uint = 0xFE ///< Message Response Unknown Command;
pub const PPSMC_Result_CmdRejectedPrereq: c_uint = 0xFD ///< Message Response Command Failed Prerequisite;
pub const PPSMC_Result_CmdRejectedBusy: c_uint = 0xFC ///< Message Response Command Rejected due to PMFW is busy. Sender should retry sending this message;
// @}
// @defgroup definitions Message definitions
// @{
//
// Message Definitions:
pub const PPSMC_MSG_TestMessage: c_uint = 0x01 ///< To check if PMFW is alive and responding. Requirement specified by PMFW team;
pub const PPSMC_MSG_GetPmfwVersion: c_uint = 0x02 ///< Get PMFW version;
pub const PPSMC_MSG_GetDriverIfVersion: c_uint = 0x03 ///< Get PMFW_DRIVER_IF version;
pub const PPSMC_MSG_PowerDownVcn: c_uint = 0x04 ///< Power down VCN;
pub const PPSMC_MSG_PowerUpVcn: c_uint = 0x05 ///< Power up VCN; VCN is power gated by default;
pub const PPSMC_MSG_SetSoftMinGfxclk: c_uint = 0x06 ///< Set SoftMin for GFXCLK, argument is frequency in MHz;
pub const PPSMC_MSG_PrepareMp1ForUnload: c_uint = 0x07 ///< Prepare PMFW for GFX driver unload;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x08 ///< Transfer driver interface table from PMFW SRAM to DRAM;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x09 ///< Transfer driver interface table from DRAM to PMFW SRAM;
pub const PPSMC_MSG_GfxDeviceDriverReset: c_uint = 0x0A ///< Request GFX mode 2 reset;
pub const PPSMC_MSG_GetEnabledSmuFeatures: c_uint = 0x0B ///< Get enabled features in PMFW;
pub const PPSMC_MSG_SetSoftMinFclk: c_uint = 0x0C ///< Set hard min for FCLK;
pub const PPSMC_MSG_SetSoftMinVcn: c_uint = 0x0D ///< Set soft min for VCN clocks (VCLK and DCLK);
pub const PPSMC_MSG_EnableGfxImu: c_uint = 0x0E ///< Enable GFX IMU;
pub const PPSMC_MSG_AllowGfxOff: c_uint = 0x0F ///< Inform PMFW of allowing GFXOFF entry;
pub const PPSMC_MSG_DisallowGfxOff: c_uint = 0x10 ///< Inform PMFW of disallowing GFXOFF entry;
pub const PPSMC_MSG_SetSoftMaxGfxClk: c_uint = 0x11 ///< Set soft max for GFX CLK;
pub const PPSMC_MSG_SetSoftMaxSocclkByFreq: c_uint = 0x12 ///< Set soft max for SOC CLK;
pub const PPSMC_MSG_SetSoftMaxFclkByFreq: c_uint = 0x13 ///< Set soft max for FCLK;
pub const PPSMC_MSG_SetSoftMaxVcn: c_uint = 0x14 ///< Set soft max for VCN clocks (VCLK and DCLK);
pub const PPSMC_MSG_PowerDownJpeg: c_uint = 0x15 ///< Power down Jpeg;
pub const PPSMC_MSG_PowerUpJpeg: c_uint = 0x16 ///< Power up Jpeg; VCN is power gated by default;
pub const PPSMC_MSG_SetSoftMinSocclkByFreq: c_uint = 0x17 ///< Set soft min for SOC CLK;
pub const PPSMC_MSG_AllowZstates: c_uint = 0x18 ///< Inform PMFM of allowing Zstate entry, i.e. no Miracast activity;
pub const PPSMC_MSG_GetSmartShiftStatus: c_uint = 0x19 ///< Returns SmartShift enable vs disable;
pub const PPSMC_MSG_PowerDownUmsch: c_uint = 0x1A ///< Power down VCN.UMSCH (aka VSCH) scheduler;
pub const PPSMC_MSG_PowerUpUmsch: c_uint = 0x1B ///< Power up VCN.UMSCH (aka VSCH) scheduler;
pub const PPSMC_MSG_PowerUpVpe: c_uint = 0x1C ///< Power up VPE;
pub const PPSMC_MSG_PowerDownVpe: c_uint = 0x1D ///< Power down VPE;
pub const PPSMC_MSG_EnableLSdma: c_uint = 0x1E ///< Enable LSDMA;
pub const PPSMC_MSG_DisableLSdma: c_uint = 0x1F ///< Disable LSDMA;
pub const PPSMC_MSG_SetSoftMaxVpe: c_uint = 0x20 ///<;
pub const PPSMC_MSG_SetSoftMinVpe: c_uint = 0x21 ///<;
pub const PPSMC_MSG_GetMetricsTableVersion: c_uint = 0x22;
pub const PPSMC_MSG_GetMetricsTableLogSample: c_uint = 0x23;
pub const PPSMC_MSG_GetMetricsTableLogDramAddr: c_uint = 0x24;
pub const PPSMC_Message_Count: c_uint = 0x25 ///< Total number of PPSMC messages;
// @}
//
// @defgroup enums Enum Definitions
// @{
//
// @enum Mode_Reset_e
// Mode reset type, argument for PPSMC_MSG_GfxDeviceDriverReset
//
// argument for PPSMC_MSG_GfxDeviceDriverReset
// @}
// @enum ZStates_e
// Zstate types, argument for PPSMC_MSG_AllowZstates
//
// Argument for PPSMC_MSG_AllowZstates
// @}
