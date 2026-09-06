//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu13_driver_if_v13_0_5.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
pub const SMU13_0_5_DRIVER_IF_VERSION: c_int = 5;
// Throttler Status Bitmask
pub const THROTTLER_STATUS_BIT_SPL: c_int = 0;
pub const THROTTLER_STATUS_BIT_FPPT: c_int = 1;
pub const THROTTLER_STATUS_BIT_SPPT: c_int = 2;
pub const THROTTLER_STATUS_BIT_SPPT_APU: c_int = 3;
pub const THROTTLER_STATUS_BIT_THM_CORE: c_int = 4;
pub const THROTTLER_STATUS_BIT_THM_GFX: c_int = 5;
pub const THROTTLER_STATUS_BIT_THM_SOC: c_int = 6;
pub const THROTTLER_STATUS_BIT_TDC_VDD: c_int = 7;
pub const THROTTLER_STATUS_BIT_TDC_SOC: c_int = 8;
pub const THROTTLER_STATUS_BIT_PROCHOT_CPU: c_int = 9;
pub const THROTTLER_STATUS_BIT_PROCHOT_GFX: c_int = 10;
pub const THROTTLER_STATUS_BIT_EDC_CPU: c_int = 11;
pub const THROTTLER_STATUS_BIT_EDC_GFX: c_int = 12;
pub const NUM_DCFCLK_DPM_LEVELS: c_int = 4;
pub const NUM_DISPCLK_DPM_LEVELS: c_int = 4;
pub const NUM_DPPCLK_DPM_LEVELS: c_int = 4;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 4;
pub const NUM_VCN_DPM_LEVELS: c_int = 4;
pub const NUM_SOC_VOLTAGE_LEVELS: c_int = 4;
pub const NUM_DF_PSTATE_LEVELS: c_int = 4;
pub const NUM_WM_RANGES: c_int = 4;
pub const WM_PSTATE_CHG: c_int = 0;
pub const WM_RETRAINING: c_int = 1;
// Watermarks
// Freq in MHz
// Voltage in milli volts with 2 fractional bits

pub const TABLE_SPARE1: c_int = 3;

pub const TABLE_COUNT: c_int = 8;
