//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu9_driver_if.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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

// IMPORTANT
// SMU TEAM: Always increment the interface version if
// any structure is changed in this file
//
pub const SMU9_DRIVER_IF_VERSION: c_uint = 0xE;
pub const PPTABLE_V10_SMU_VERSION: c_int = 1;
pub const NUM_GFXCLK_DPM_LEVELS: c_int = 8;
pub const NUM_UVD_DPM_LEVELS: c_int = 8;
pub const NUM_VCE_DPM_LEVELS: c_int = 8;
pub const NUM_MP0CLK_DPM_LEVELS: c_int = 8;
pub const NUM_UCLK_DPM_LEVELS: c_int = 4;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DCEFCLK_DPM_LEVELS: c_int = 8;
pub const NUM_LINK_LEVELS: c_int = 2;

pub const MIN_GFXCLK_DPM_LEVEL: c_int = 0;
pub const MIN_UVD_DPM_LEVEL: c_int = 0;
pub const MIN_VCE_DPM_LEVEL: c_int = 0;
pub const MIN_MP0CLK_DPM_LEVEL: c_int = 0;
pub const MIN_UCLK_DPM_LEVEL: c_int = 0;
pub const MIN_SOCCLK_DPM_LEVEL: c_int = 0;
pub const MIN_DCEFCLK_DPM_LEVEL: c_int = 0;
pub const MIN_LINK_DPM_LEVEL: c_int = 0;
pub const NUM_EVV_VOLTAGE_LEVELS: c_int = 8;

pub const MIN_EVV_VOLTAGE_LEVEL: c_int = 0;
pub const NUM_PSP_LEVEL_MAP: c_int = 4;
// Gemini Modes

// Voltage Modes for DPMs
pub const VOLTAGE_MODE_AVFS_INTERPOLATE: c_int = 0;
pub const VOLTAGE_MODE_AVFS_WORST_CASE: c_int = 1;
pub const VOLTAGE_MODE_STATIC: c_int = 2;
pub const NUM_DSPCLK_LEVELS: c_int = 8;

// PowerTune
// External Component Communication Settings
// ULV Settings
// VDDCR_SOC Voltages
// This is the minimum voltage needed to run the SOC.
// SOC Frequencies
// Alpha parameters for clock averages. ("255"=1)
// UCLK States
// CKS Settings
// MP0 Mapping Table
// Link DPM Settings
// Fan Control
// The following are AFC override parameters. Leave at 0 to use FW defaults.
// GPIO Settings
// LED Display Settings
// AVFS
// Ageing Guardband Parameters
// ACG Frequency Table, in Mhz
// Padding - ignore

pub const NUM_WM_RANGES: c_int = 4;
// Watermarks

// These defines are used with the following messages:
// SMC_MSG_TransferTableDram2Smu
// SMC_MSG_TransferTableSmu2Dram
//
pub const TABLE_PPTABLE: c_int = 0;
pub const TABLE_WATERMARKS: c_int = 1;
pub const TABLE_AVFS: c_int = 2;
pub const TABLE_AVFS_PSM_DEBUG: c_int = 3;
pub const TABLE_AVFS_FUSE_OVERRIDE: c_int = 4;
pub const TABLE_PMSTATUSLOG: c_int = 5;
pub const TABLE_COUNT: c_int = 6;
// These defines are used with the SMC_MSG_SetUclkFastSwitch message.
pub const UCLK_SWITCH_SLOW: c_int = 0;
pub const UCLK_SWITCH_FAST: c_int = 1;
// GFX DIDT Configuration
pub const SQ_Enable_MASK: c_uint = 0x1;
pub const SQ_IR_MASK: c_uint = 0x2;
pub const SQ_PCC_MASK: c_uint = 0x4;
pub const SQ_EDC_MASK: c_uint = 0x8;
pub const TCP_Enable_MASK: c_uint = 0x100;
pub const TCP_IR_MASK: c_uint = 0x200;
pub const TCP_PCC_MASK: c_uint = 0x400;
pub const TCP_EDC_MASK: c_uint = 0x800;
pub const TD_Enable_MASK: c_uint = 0x10000;
pub const TD_IR_MASK: c_uint = 0x20000;
pub const TD_PCC_MASK: c_uint = 0x40000;
pub const TD_EDC_MASK: c_uint = 0x80000;
pub const DB_Enable_MASK: c_uint = 0x1000000;
pub const DB_IR_MASK: c_uint = 0x2000000;
pub const DB_PCC_MASK: c_uint = 0x4000000;
pub const DB_EDC_MASK: c_uint = 0x8000000;
pub const SQ_Enable_SHIFT: c_int = 0;
pub const SQ_IR_SHIFT: c_int = 1;
pub const SQ_PCC_SHIFT: c_int = 2;
pub const SQ_EDC_SHIFT: c_int = 3;
pub const TCP_Enable_SHIFT: c_int = 8;
pub const TCP_IR_SHIFT: c_int = 9;
pub const TCP_PCC_SHIFT: c_int = 10;
pub const TCP_EDC_SHIFT: c_int = 11;
pub const TD_Enable_SHIFT: c_int = 16;
pub const TD_IR_SHIFT: c_int = 17;
pub const TD_PCC_SHIFT: c_int = 18;
pub const TD_EDC_SHIFT: c_int = 19;
pub const DB_Enable_SHIFT: c_int = 24;
pub const DB_IR_SHIFT: c_int = 25;
pub const DB_PCC_SHIFT: c_int = 26;
pub const DB_EDC_SHIFT: c_int = 27;
pub const REMOVE_FMAX_MARGIN_BIT: c_uint = 0x0;
pub const REMOVE_DCTOL_MARGIN_BIT: c_uint = 0x1;
pub const REMOVE_PLATFORM_MARGIN_BIT: c_uint = 0x2;
