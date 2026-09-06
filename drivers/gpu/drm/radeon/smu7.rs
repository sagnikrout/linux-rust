//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/smu7.h
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
// Copyright 2013 Advanced Micro Devices, Inc.
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

pub const SMU7_CONTEXT_ID_SMC: c_int = 1;
pub const SMU7_CONTEXT_ID_VBIOS: c_int = 2;
pub const SMU7_CONTEXT_ID_SMC: c_int = 1;
pub const SMU7_CONTEXT_ID_VBIOS: c_int = 2;
pub const SMU7_MAX_LEVELS_VDDC: c_int = 8;
pub const SMU7_MAX_LEVELS_VDDCI: c_int = 4;
pub const SMU7_MAX_LEVELS_MVDD: c_int = 4;
pub const SMU7_MAX_LEVELS_VDDNB: c_int = 8;

pub const DPM_NO_LIMIT: c_int = 0;
pub const DPM_NO_UP: c_int = 1;
pub const DPM_GO_DOWN: c_int = 2;
pub const DPM_GO_UP: c_int = 3;
pub const SMU7_FIRST_DPM_GRAPHICS_LEVEL: c_int = 0;
pub const SMU7_FIRST_DPM_MEMORY_LEVEL: c_int = 0;
pub const GPIO_CLAMP_MODE_VRHOT: c_int = 1;
pub const GPIO_CLAMP_MODE_THERM: c_int = 2;
pub const GPIO_CLAMP_MODE_DC: c_int = 4;
pub const SCRATCH_B_TARG_PCIE_INDEX_SHIFT: c_int = 0;

pub const SCRATCH_B_CURR_PCIE_INDEX_SHIFT: c_int = 3;

pub const SCRATCH_B_TARG_UVD_INDEX_SHIFT: c_int = 6;

pub const SCRATCH_B_CURR_UVD_INDEX_SHIFT: c_int = 9;

pub const SCRATCH_B_TARG_VCE_INDEX_SHIFT: c_int = 12;

pub const SCRATCH_B_CURR_VCE_INDEX_SHIFT: c_int = 15;

pub const SCRATCH_B_TARG_ACP_INDEX_SHIFT: c_int = 18;

pub const SCRATCH_B_CURR_ACP_INDEX_SHIFT: c_int = 21;

pub const SCRATCH_B_TARG_SAMU_INDEX_SHIFT: c_int = 24;

pub const SCRATCH_B_CURR_SAMU_INDEX_SHIFT: c_int = 27;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_PIDController {
    pub Ki: u32,
    pub LFWindupUL: i32,
    pub LFWindupLL: i32,
    pub StatePrecision: u32,
    pub LfPrecision: u32,
    pub LfOffset: u32,
    pub MaxState: u32,
    pub MaxLfFraction: u32,
    pub StateShift: u32,
}

pub type SMU7_PIDController = SMU7_PIDController;
// -------------------------------------------------------------------------------------------------------------------------

pub const SMU7_SCLK_DPM_CONFIG_MASK: c_uint = 0x01;
pub const SMU7_VOLTAGE_CONTROLLER_CONFIG_MASK: c_uint = 0x02;
pub const SMU7_THERMAL_CONTROLLER_CONFIG_MASK: c_uint = 0x04;
pub const SMU7_MCLK_DPM_CONFIG_MASK: c_uint = 0x08;
pub const SMU7_UVD_DPM_CONFIG_MASK: c_uint = 0x10;
pub const SMU7_VCE_DPM_CONFIG_MASK: c_uint = 0x20;
pub const SMU7_ACP_DPM_CONFIG_MASK: c_uint = 0x40;
pub const SMU7_SAMU_DPM_CONFIG_MASK: c_uint = 0x80;
pub const SMU7_PCIEGEN_DPM_CONFIG_MASK: c_uint = 0x100;
pub const SMU7_ACP_MCLK_HANDSHAKE_DISABLE: c_uint = 0x00000001;
pub const SMU7_ACP_SCLK_HANDSHAKE_DISABLE: c_uint = 0x00000002;
pub const SMU7_UVD_MCLK_HANDSHAKE_DISABLE: c_uint = 0x00000100;
pub const SMU7_UVD_SCLK_HANDSHAKE_DISABLE: c_uint = 0x00000200;
pub const SMU7_VCE_MCLK_HANDSHAKE_DISABLE: c_uint = 0x00010000;
pub const SMU7_VCE_SCLK_HANDSHAKE_DISABLE: c_uint = 0x00020000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU7_Firmware_Header {
    pub Digest: [u32; 5],
    pub Version: u32,
    pub HeaderSize: u32,
    pub Flags: u32,
    pub EntryPoint: u32,
    pub CodeSize: u32,
    pub ImageSize: u32,
    pub Rtos: u32,
    pub SoftRegisters: u32,
    pub DpmTable: u32,
    pub FanTable: u32,
    pub CacConfigTable: u32,
    pub CacStatusTable: u32,
    pub mcRegisterTable: u32,
    pub mcArbDramTimingTable: u32,
    pub PmFuseTable: u32,
    pub Globals: u32,
    pub Reserved: [u32; 42],
    pub Signature: u32,
}

pub type SMU7_Firmware_Header = SMU7_Firmware_Header;
pub const SMU7_FIRMWARE_HEADER_LOCATION: c_uint = 0x20000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DisplayConfig {
    PowerDown = 1,
    DP54x4,
    DP54x2,
    DP54x1,
    DP27x4,
    DP27x2,
    DP27x1,
    HDMI297,
    HDMI162,
    LVDS,
    DP324x4,
    DP324x2,
    DP324x1
}

