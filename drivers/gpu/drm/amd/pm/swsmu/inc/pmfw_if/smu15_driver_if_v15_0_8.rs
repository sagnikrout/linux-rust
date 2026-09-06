//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu15_driver_if_v15_0_8.h
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
// I2C Interface
pub const NUM_I2C_CONTROLLERS: c_int = 8;
pub const I2C_CONTROLLER_ENABLED: c_int = 1;
pub const I2C_CONTROLLER_DISABLED: c_int = 0;
pub const MAX_SW_I2C_COMMANDS: c_int = 24;
// 50  Kbits/s not supported anymore!
// 100 Kbits/s
// 400 Kbits/s
// 1   Mbits/s (in fast mode)
// 1   Mbits/s (in high speed mode)  not supported anymore!
// 2.3 Mbits/s  not supported anymore!
pub const CMDCONFIG_STOP_BIT: c_int = 0;
pub const CMDCONFIG_RESTART_BIT: c_int = 1;
// bit should be 0 for read, 1 for write
pub const CMDCONFIG_READWRITE_BIT: c_int = 2;

// 64 Bit register offsets for PPSMC_MSG_McaBankDumpDW, PPSMC_MSG_McaBankCeDumpDW messages
// eg to read MCA_BANK_OFFSET_SYND for CE index, call PPSMC_MSG_McaBankCeDumpDW twice,
// (index << 16 + MCA_BANK_OFFSET_SYND*8) argument for 1st DWORD, and
// ((index << 16 ) + MCA_BANK_OFFSET_SYND*8 + 4) argument for 2nd DWORD
// Firmware MP1 AID MCA Error Codes stored in MCA_MP_MP1:MCMP1_SYNDT0 errorinformation
// MMHUB
// VCN VCPU
// VCN JPEG
// VCN MMSCH
// SDMA
// SOC
// Firmware MP5 XCD MCA Error Codes stored in MCA_MP_MP5:MCMP5_SYNDT0 errorinformation
// SH POISON FED
// GCEA Pin UE_ERR regs
// GCEA Pin, UE_EDC regs
// GC Router
// SOC error codes 41-43 are common with ERR_CODE_e
// SW I2C Command Table
// Return data for read. Data to send for write
// Includes whether associated command should have a stop or restart command,
// and is a read or write
// SW I2C Request Table
// CKSVII2C0(0) or //CKSVII2C1(1)
// Use I2cSpeed_e to indicate speed to select
// Slave address of device
// Number of commands
// SMU internal use
// TODO confirm if this is used in MI300 PPSMC_MSG_SetUclkDpmMode
// 2 AVFS.PSM chains
// For voltage conversions, these are the array indexes
// 0:SOCIO
// 1:065_UCIE
// 2:075_UCIE
// 3:11_GTA
// 4:075_GTA
// 7 AVFS.PSM chains - not including TRO
// For voltage conversions, these are the array indexes
// 0:VDDX
// 0-27 GFX, 28-29 SOC
// Defines used for IH-based thermal interrupts to GFX driver - A/X only
pub const IH_INTERRUPT_ID_TO_DRIVER: c_uint = 0xFE;
pub const IH_INTERRUPT_CONTEXT_ID_THERMAL_THROTTLING: c_uint = 0x7;
pub const IH_INTERRUPT_VFFLR_INT: c_uint = 0xA;
// thermal over-temp mask defines for IH interrup to host
pub const THROTTLER_PROCHOT_BIT: c_int = 0;
pub const THROTTLER_RESERVED: c_int = 1;
// AID, XCD, CCD throttling
pub const THROTTLER_THERMAL_SOCKET_BIT: c_int = 2;
// VRHOT
pub const THROTTLER_THERMAL_VR_BIT: c_int = 3;
pub const THROTTLER_THERMAL_HBM_BIT: c_int = 4;
// UEs are always reported, set flag to 0 to prevent clearing of UEs
pub const ClearMcaOnRead_UE_FLAG_MASK: c_uint = 0x1;
// Enable CE logging and clearing to driver
pub const ClearMcaOnRead_CE_POLL_MASK: c_uint = 0x2;
// AID MMHUB client IP CE Logging and clearing
pub const ClearMcaOnRead_MMHUB_POLL_MASK: c_uint = 0x4;
