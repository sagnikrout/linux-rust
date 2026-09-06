//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v14_0_0_ppsmc.h
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
pub const PPSMC_MSG_PowerDownVcn1: c_uint = 0x04 ///< Power down VCN1;
pub const PPSMC_MSG_PowerUpVcn1: c_uint = 0x05 ///< Power up VCN1; VCN1 is power gated by default;
pub const PPSMC_MSG_PowerDownVcn0: c_uint = 0x06 ///< Power down VCN0;
pub const PPSMC_MSG_PowerUpVcn0: c_uint = 0x07 ///< Power up VCN0; VCN0 is power gated by default;
pub const PPSMC_MSG_SetHardMinVcn0: c_uint = 0x08 ///< For wireless display;
pub const PPSMC_MSG_SetSoftMinGfxclk: c_uint = 0x09 ///< Set SoftMin for GFXCLK, argument is frequency in MHz;
pub const PPSMC_MSG_SetHardMinVcn1: c_uint = 0x0A ///< For wireless display;
pub const PPSMC_MSG_SetSoftMinVcn1: c_uint = 0x0B ///< Set soft min for VCN1 clocks (VCLK1 and DCLK1);
pub const PPSMC_MSG_PrepareMp1ForUnload: c_uint = 0x0C ///< Prepare PMFW for GFX driver unload;
pub const PPSMC_MSG_SetDriverDramAddrHigh: c_uint = 0x0D ///< Set high 32 bits of DRAM address for Driver table transfer;
pub const PPSMC_MSG_SetDriverDramAddrLow: c_uint = 0x0E ///< Set low 32 bits of DRAM address for Driver table transfer;
pub const PPSMC_MSG_TransferTableSmu2Dram: c_uint = 0x0F ///< Transfer driver interface table from PMFW SRAM to DRAM;
pub const PPSMC_MSG_TransferTableDram2Smu: c_uint = 0x10 ///< Transfer driver interface table from DRAM to PMFW SRAM;
pub const PPSMC_MSG_GfxDeviceDriverReset: c_uint = 0x11 ///< Request GFX mode 2 reset;
pub const PPSMC_MSG_GetEnabledSmuFeatures: c_uint = 0x12 ///< Get enabled features in PMFW;
pub const PPSMC_MSG_SetHardMinSocclkByFreq: c_uint = 0x13 ///< Set hard min for SOC CLK;
pub const PPSMC_MSG_SetSoftMinFclk: c_uint = 0x14 ///< Set hard min for FCLK;
pub const PPSMC_MSG_SetSoftMinVcn0: c_uint = 0x15 ///< Set soft min for VCN0 clocks (VCLK0 and DCLK0);
pub const PPSMC_MSG_EnableGfxImu: c_uint = 0x16 ///< Enable GFX IMU;
pub const PPSMC_MSG_spare_0x17: c_uint = 0x17 ///< Get GFX clock frequency;
pub const PPSMC_MSG_spare_0x18: c_uint = 0x18 ///< Get FCLK frequency;
pub const PPSMC_MSG_AllowGfxOff: c_uint = 0x19 ///< Inform PMFW of allowing GFXOFF entry;
pub const PPSMC_MSG_DisallowGfxOff: c_uint = 0x1A ///< Inform PMFW of disallowing GFXOFF entry;
pub const PPSMC_MSG_SetSoftMaxGfxClk: c_uint = 0x1B ///< Set soft max for GFX CLK;
pub const PPSMC_MSG_SetHardMinGfxClk: c_uint = 0x1C ///< Set hard min for GFX CLK;
pub const PPSMC_MSG_SetSoftMaxSocclkByFreq: c_uint = 0x1D ///< Set soft max for SOC CLK;
pub const PPSMC_MSG_SetSoftMaxFclkByFreq: c_uint = 0x1E ///< Set soft max for FCLK;
pub const PPSMC_MSG_SetSoftMaxVcn0: c_uint = 0x1F ///< Set soft max for VCN0 clocks (VCLK0 and DCLK0);
pub const PPSMC_MSG_spare_0x20: c_uint = 0x20 ///< Set power limit percentage;
pub const PPSMC_MSG_PowerDownJpeg0: c_uint = 0x21 ///< Power down Jpeg of VCN0;
pub const PPSMC_MSG_PowerUpJpeg0: c_uint = 0x22 ///< Power up Jpeg of VCN0; VCN0 is power gated by default;
pub const PPSMC_MSG_SetHardMinFclkByFreq: c_uint = 0x23 ///< Set hard min for FCLK;
pub const PPSMC_MSG_SetSoftMinSocclkByFreq: c_uint = 0x24 ///< Set soft min for SOC CLK;
pub const PPSMC_MSG_AllowZstates: c_uint = 0x25 ///< Inform PMFM of allowing Zstate entry, i.e. no Miracast activity;
pub const PPSMC_MSG_PowerDownJpeg1: c_uint = 0x26 ///< Power down Jpeg of VCN1;
pub const PPSMC_MSG_PowerUpJpeg1: c_uint = 0x27 ///< Power up Jpeg of VCN1; VCN1 is power gated by default;
pub const PPSMC_MSG_SetSoftMaxVcn1: c_uint = 0x28 ///< Set soft max for VCN1 clocks (VCLK1 and DCLK1);
pub const PPSMC_MSG_PowerDownIspByTile: c_uint = 0x29 ///< ISP is power gated by default;
pub const PPSMC_MSG_PowerUpIspByTile: c_uint = 0x2A ///< This message is used to power up ISP tiles and enable the ISP DPM;
pub const PPSMC_MSG_SetHardMinIspiclkByFreq: c_uint = 0x2B ///< Set HardMin by frequency for ISPICLK;
pub const PPSMC_MSG_SetHardMinIspxclkByFreq: c_uint = 0x2C ///< Set HardMin by frequency for ISPXCLK;
pub const PPSMC_MSG_PowerDownUmsch: c_uint = 0x2D ///< Power down VCN0.UMSCH (aka VSCH) scheduler;
pub const PPSMC_MSG_PowerUpUmsch: c_uint = 0x2E ///< Power up VCN0.UMSCH (aka VSCH) scheduler;
pub const PPSMC_Message_IspStutterOn_MmhubPgDis: c_uint = 0x2F ///< ISP StutterOn mmHub PgDis;
pub const PPSMC_Message_IspStutterOff_MmhubPgEn: c_uint = 0x30 ///< ISP StufferOff mmHub PgEn;
pub const PPSMC_MSG_PowerUpVpe: c_uint = 0x31 ///< Power up VPE;
pub const PPSMC_MSG_PowerDownVpe: c_uint = 0x32 ///< Power down VPE;
pub const PPSMC_MSG_GetVpeDpmTable: c_uint = 0x33 ///< Get VPE DPM table;
pub const PPSMC_MSG_EnableLSdma: c_uint = 0x34 ///< Enable LSDMA;
pub const PPSMC_MSG_DisableLSdma: c_uint = 0x35 ///< Disable LSDMA;
pub const PPSMC_MSG_SetSoftMaxVpe: c_uint = 0x36 ///<;
pub const PPSMC_MSG_SetSoftMinVpe: c_uint = 0x37 ///<;
pub const PPSMC_MSG_MALLPowerController: c_uint = 0x38 ///< Set MALL control;
pub const PPSMC_MSG_MALLPowerState: c_uint = 0x39 ///< Enter/Exit MALL PG;
pub const PPSMC_Message_Count: c_uint = 0x3A ///< Total number of PPSMC messages;
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
