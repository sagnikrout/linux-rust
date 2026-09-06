//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/fiji_ppsmc.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

pub const PPSMC_SWSTATE_FLAG_DC: c_uint = 0x01;
pub const PPSMC_SWSTATE_FLAG_UVD: c_uint = 0x02;
pub const PPSMC_SWSTATE_FLAG_VCE: c_uint = 0x04;
pub const PPSMC_THERMAL_PROTECT_TYPE_INTERNAL: c_uint = 0x00;
pub const PPSMC_THERMAL_PROTECT_TYPE_EXTERNAL: c_uint = 0x01;
pub const PPSMC_THERMAL_PROTECT_TYPE_NONE: c_uint = 0xff;
pub const PPSMC_SYSTEMFLAG_GPIO_DC: c_uint = 0x01;
pub const PPSMC_SYSTEMFLAG_STEPVDDC: c_uint = 0x02;
pub const PPSMC_SYSTEMFLAG_GDDR5: c_uint = 0x04;
pub const PPSMC_SYSTEMFLAG_DISABLE_BABYSTEP: c_uint = 0x08;
pub const PPSMC_SYSTEMFLAG_REGULATOR_HOT: c_uint = 0x10;
pub const PPSMC_SYSTEMFLAG_REGULATOR_HOT_ANALOG: c_uint = 0x20;
pub const PPSMC_EXTRAFLAGS_AC2DC_ACTION_MASK: c_uint = 0x07;
pub const PPSMC_EXTRAFLAGS_AC2DC_DONT_WAIT_FOR_VBLANK: c_uint = 0x08;
pub const PPSMC_EXTRAFLAGS_AC2DC_ACTION_GOTODPMLOWSTATE: c_uint = 0x00;
pub const PPSMC_EXTRAFLAGS_AC2DC_ACTION_GOTOINITIALSTATE: c_uint = 0x01;
// Defines for DPM 2.0
pub const PPSMC_DPM2FLAGS_TDPCLMP: c_uint = 0x01;
pub const PPSMC_DPM2FLAGS_PWRSHFT: c_uint = 0x02;
pub const PPSMC_DPM2FLAGS_OCP: c_uint = 0x04;
// Defines for display watermark level
pub const PPSMC_DISPLAY_WATERMARK_LOW: c_int = 0;
pub const PPSMC_DISPLAY_WATERMARK_HIGH: c_int = 1;
// In the HW performance level's state flags:
pub const PPSMC_STATEFLAG_AUTO_PULSE_SKIP: c_uint = 0x01;
pub const PPSMC_STATEFLAG_POWERBOOST: c_uint = 0x02;
pub const PPSMC_STATEFLAG_PSKIP_ON_TDP_FAULT: c_uint = 0x04;
pub const PPSMC_STATEFLAG_POWERSHIFT: c_uint = 0x08;
pub const PPSMC_STATEFLAG_SLOW_READ_MARGIN: c_uint = 0x10;
pub const PPSMC_STATEFLAG_DEEPSLEEP_THROTTLE: c_uint = 0x20;
pub const PPSMC_STATEFLAG_DEEPSLEEP_BYPASS: c_uint = 0x40;
// Fan control algorithm:
pub const FDO_MODE_HARDWARE: c_int = 0;
pub const FDO_MODE_PIECE_WISE_LINEAR: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FAN_CONTROL {
    FAN_CONTROL_FUZZY,
    FAN_CONTROL_TABLE
}

// Gemini Modes

// Return codes for driver to SMC communication.

// Trinity Specific Messages

// AVFS Only - Remove Later

// If the SMC firmware has an event status soft register this is what the individual bits mean.
pub const PPSMC_EVENT_STATUS_THERMAL: c_uint = 0x00000001;
pub const PPSMC_EVENT_STATUS_REGULATORHOT: c_uint = 0x00000002;
pub const PPSMC_EVENT_STATUS_DC: c_uint = 0x00000004;
pub type PPSMC_Msg = u16;

