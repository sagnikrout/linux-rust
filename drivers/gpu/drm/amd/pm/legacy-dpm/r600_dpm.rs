//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/legacy-dpm/r600_dpm.h
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
// Copyright 2011 Advanced Micro Devices, Inc.
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
pub const R600_ASI_DFLT: c_int = 10000;
pub const R600_BSP_DFLT: c_uint = 0x41EB;
pub const R600_BSU_DFLT: c_uint = 0x2;
pub const R600_AH_DFLT: c_int = 5;
pub const R600_RLP_DFLT: c_int = 25;
pub const R600_RMP_DFLT: c_int = 65;
pub const R600_LHP_DFLT: c_int = 40;
pub const R600_LMP_DFLT: c_int = 15;
pub const R600_TD_DFLT: c_int = 0;
pub const R600_UTC_DFLT_00: c_uint = 0x24;
pub const R600_UTC_DFLT_01: c_uint = 0x22;
pub const R600_UTC_DFLT_02: c_uint = 0x22;
pub const R600_UTC_DFLT_03: c_uint = 0x22;
pub const R600_UTC_DFLT_04: c_uint = 0x22;
pub const R600_UTC_DFLT_05: c_uint = 0x22;
pub const R600_UTC_DFLT_06: c_uint = 0x22;
pub const R600_UTC_DFLT_07: c_uint = 0x22;
pub const R600_UTC_DFLT_08: c_uint = 0x22;
pub const R600_UTC_DFLT_09: c_uint = 0x22;
pub const R600_UTC_DFLT_10: c_uint = 0x22;
pub const R600_UTC_DFLT_11: c_uint = 0x22;
pub const R600_UTC_DFLT_12: c_uint = 0x22;
pub const R600_UTC_DFLT_13: c_uint = 0x22;
pub const R600_UTC_DFLT_14: c_uint = 0x22;
pub const R600_DTC_DFLT_00: c_uint = 0x24;
pub const R600_DTC_DFLT_01: c_uint = 0x22;
pub const R600_DTC_DFLT_02: c_uint = 0x22;
pub const R600_DTC_DFLT_03: c_uint = 0x22;
pub const R600_DTC_DFLT_04: c_uint = 0x22;
pub const R600_DTC_DFLT_05: c_uint = 0x22;
pub const R600_DTC_DFLT_06: c_uint = 0x22;
pub const R600_DTC_DFLT_07: c_uint = 0x22;
pub const R600_DTC_DFLT_08: c_uint = 0x22;
pub const R600_DTC_DFLT_09: c_uint = 0x22;
pub const R600_DTC_DFLT_10: c_uint = 0x22;
pub const R600_DTC_DFLT_11: c_uint = 0x22;
pub const R600_DTC_DFLT_12: c_uint = 0x22;
pub const R600_DTC_DFLT_13: c_uint = 0x22;
pub const R600_DTC_DFLT_14: c_uint = 0x22;
pub const R600_VRC_DFLT: c_uint = 0x0000C003;
pub const R600_VOLTAGERESPONSETIME_DFLT: c_int = 1000;
pub const R600_BACKBIASRESPONSETIME_DFLT: c_int = 1000;
pub const R600_VRU_DFLT: c_uint = 0x3;
pub const R600_SPLLSTEPTIME_DFLT: c_uint = 0x1000;
pub const R600_SPLLSTEPUNIT_DFLT: c_uint = 0x3;
pub const R600_TPU_DFLT: c_int = 0;
pub const R600_TPC_DFLT: c_uint = 0x200;
pub const R600_SSTU_DFLT: c_int = 0;
pub const R600_SST_DFLT: c_uint = 0x00C8;
pub const R600_GICST_DFLT: c_uint = 0x200;
pub const R600_FCT_DFLT: c_uint = 0x0400;
pub const R600_FCTU_DFLT: c_int = 0;
pub const R600_CTXCGTT3DRPHC_DFLT: c_uint = 0x20;
pub const R600_CTXCGTT3DRSDC_DFLT: c_uint = 0x40;
pub const R600_VDDC3DOORPHC_DFLT: c_uint = 0x100;
pub const R600_VDDC3DOORSDC_DFLT: c_uint = 0x7;
pub const R600_VDDC3DOORSU_DFLT: c_int = 0;
pub const R600_MPLLLOCKTIME_DFLT: c_int = 100;
pub const R600_MPLLRESETTIME_DFLT: c_int = 150;
pub const R600_VCOSTEPPCT_DFLT: c_int = 20;
pub const R600_ENDINGVCOSTEPPCT_DFLT: c_int = 5;
pub const R600_REFERENCEDIVIDER_DFLT: c_int = 4;
pub const R600_PM_NUMBER_OF_TC: c_int = 15;
pub const R600_PM_NUMBER_OF_SCLKS: c_int = 20;
pub const R600_PM_NUMBER_OF_MCLKS: c_int = 4;
pub const R600_PM_NUMBER_OF_VOLTAGE_LEVELS: c_int = 4;
pub const R600_PM_NUMBER_OF_ACTIVITY_LEVELS: c_int = 3;
// XXX are these ok?

pub const FDO_PWM_MODE_STATIC: c_int = 1;
pub const FDO_PWM_MODE_STATIC_RPM: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_power_level {
    R600_POWER_LEVEL_LOW = 0,
    R600_POWER_LEVEL_MEDIUM = 1,
    R600_POWER_LEVEL_HIGH = 2,
    R600_POWER_LEVEL_CTXSW = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_td {
    R600_TD_AUTO,
    R600_TD_UP,
    R600_TD_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_display_watermark {
    R600_DISPLAY_WATERMARK_LOW = 0,
    R600_DISPLAY_WATERMARK_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r600_display_gap {
    R600_PM_DISPLAY_GAP_VBLANK_OR_WM = 0,
    R600_PM_DISPLAY_GAP_VBLANK       = 1,
    R600_PM_DISPLAY_GAP_WATERMARK    = 2,
    R600_PM_DISPLAY_GAP_IGNORE       = 3,
}
