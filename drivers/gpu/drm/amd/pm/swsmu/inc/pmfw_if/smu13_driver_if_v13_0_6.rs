//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu13_driver_if_v13_0_6.h
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
// *** IMPORTANT
// PMFW TEAM: Always increment the interface version if
// anything is changed in this file
pub const SMU13_0_6_DRIVER_IF_VERSION: c_uint = 0x08042024;
// I2C Interface
pub const NUM_I2C_CONTROLLERS: c_int = 8;
pub const I2C_CONTROLLER_ENABLED: c_int = 1;
pub const I2C_CONTROLLER_DISABLED: c_int = 0;
pub const MAX_SW_I2C_COMMANDS: c_int = 24;
pub const CMDCONFIG_STOP_BIT: c_int = 0;
pub const CMDCONFIG_RESTART_BIT: c_int = 1;

// MMHUB
// VCN
// VCN VCPU
// VCN JPEG
// VCN MMSCH
// SDMA
// SOC
// SH POISON FED
// GCEA Pin UE_ERR regs
// GCEA Pin, UE_EDC regs
// GC Router
// SOC error codes 40-42 are common with ERR_CODE_e
// TODO confirm if this is used in SMU_13_0_6 PPSMC_MSG_SetUclkDpmMode
// 0-23 SOC, 24-26 SOCIO, 27-29 SOC
// 0-27 GFX, 28-29 SOC
// Defines used for IH-based thermal interrupts to GFX driver - A/X only
pub const IH_INTERRUPT_ID_TO_DRIVER: c_uint = 0xFE;
pub const IH_INTERRUPT_CONTEXT_ID_THERMAL_THROTTLING: c_uint = 0x7;
// thermal over-temp mask defines for IH interrupt to host
pub const THROTTLER_PROCHOT_BIT: c_int = 0;
pub const THROTTLER_PPT_BIT: c_int = 1;

pub const THROTTLER_THERMAL_HBM_BIT: c_int = 4;
pub const ClearMcaOnRead_UE_FLAG_MASK: c_uint = 0x1;
pub const ClearMcaOnRead_CE_POLL_MASK: c_uint = 0x2;
// These defines are used with the following messages:
// SMC_MSG_TransferTableDram2Smu
// SMC_MSG_TransferTableSmu2Dram
// #define TABLE_PPTABLE                 0
// #define TABLE_AVFS_PSM_DEBUG          1
// #define TABLE_AVFS_FUSE_OVERRIDE      2
// #define TABLE_PMSTATUSLOG             3
// #define TABLE_SMU_METRICS             4
// #define TABLE_DRIVER_SMU_CONFIG       5
// #define TABLE_I2C_COMMANDS            6
// #define TABLE_COUNT                   7
// // Table transfer status
// #define TABLE_TRANSFER_OK         0x0
// #define TABLE_TRANSFER_FAILED     0xFF
// #define TABLE_TRANSFER_PENDING    0xAB
