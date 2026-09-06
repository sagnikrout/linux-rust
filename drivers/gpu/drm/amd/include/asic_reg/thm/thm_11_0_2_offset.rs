//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/thm/thm_11_0_2_offset.h
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
// Copyright (C) 2018  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _thm_11_0_2_OFFSET_HEADER
pub const mmCG_MULT_THERMAL_STATUS: c_uint = 0x005f;
pub const mmCG_MULT_THERMAL_STATUS_BASE_IDX: c_int = 0;
pub const mmCG_FDO_CTRL0: c_uint = 0x0067;
pub const mmCG_FDO_CTRL0_BASE_IDX: c_int = 0;
pub const mmCG_FDO_CTRL1: c_uint = 0x0068;
pub const mmCG_FDO_CTRL1_BASE_IDX: c_int = 0;
pub const mmCG_FDO_CTRL2: c_uint = 0x0069;
pub const mmCG_FDO_CTRL2_BASE_IDX: c_int = 0;
pub const mmCG_TACH_CTRL: c_uint = 0x006a;
pub const mmCG_TACH_CTRL_BASE_IDX: c_int = 0;
pub const mmCG_TACH_STATUS: c_uint = 0x006b;
pub const mmCG_TACH_STATUS_BASE_IDX: c_int = 0;
pub const mmTHM_THERMAL_INT_ENA: c_uint = 0x000a;
pub const mmTHM_THERMAL_INT_ENA_BASE_IDX: c_int = 0;
pub const mmTHM_THERMAL_INT_CTRL: c_uint = 0x000b;
pub const mmTHM_THERMAL_INT_CTRL_BASE_IDX: c_int = 0;
pub const mmTHM_TCON_THERM_TRIP: c_uint = 0x0002;
pub const mmTHM_TCON_THERM_TRIP_BASE_IDX: c_int = 0;
pub const mmTHM_BACO_CNTL: c_uint = 0x0081;
pub const mmTHM_BACO_CNTL_BASE_IDX: c_int = 0;
pub const mmCG_THERMAL_STATUS: c_uint = 0x006C;
pub const mmCG_THERMAL_STATUS_BASE_IDX: c_int = 0;
