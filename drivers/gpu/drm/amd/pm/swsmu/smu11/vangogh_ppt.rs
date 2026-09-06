//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/smu11/vangogh_ppt.h
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
extern "C" {
    pub fn vangogh_set_ppt_funcs(smu: *mut smu_context);
}
// UMD PState Vangogh Msg Parameters in MHz
pub const VANGOGH_UMD_PSTATE_STANDARD_GFXCLK: c_int = 1100;
pub const VANGOGH_UMD_PSTATE_STANDARD_SOCCLK: c_int = 600;
pub const VANGOGH_UMD_PSTATE_STANDARD_FCLK: c_int = 800;
pub const VANGOGH_UMD_PSTATE_STANDARD_VCLK: c_int = 705;
pub const VANGOGH_UMD_PSTATE_STANDARD_DCLK: c_int = 600;
pub const VANGOGH_UMD_PSTATE_PEAK_GFXCLK: c_int = 1300;
pub const VANGOGH_UMD_PSTATE_PEAK_SOCCLK: c_int = 600;
pub const VANGOGH_UMD_PSTATE_PEAK_FCLK: c_int = 800;
pub const VANGOGH_UMD_PSTATE_PEAK_VCLK: c_int = 705;
pub const VANGOGH_UMD_PSTATE_PEAK_DCLK: c_int = 600;
pub const VANGOGH_UMD_PSTATE_MIN_SCLK_GFXCLK: c_int = 400;
pub const VANGOGH_UMD_PSTATE_MIN_SCLK_SOCCLK: c_int = 1000;
pub const VANGOGH_UMD_PSTATE_MIN_SCLK_FCLK: c_int = 800;
pub const VANGOGH_UMD_PSTATE_MIN_SCLK_VCLK: c_int = 1000;
pub const VANGOGH_UMD_PSTATE_MIN_SCLK_DCLK: c_int = 800;
pub const VANGOGH_UMD_PSTATE_MIN_MCLK_GFXCLK: c_int = 1100;
pub const VANGOGH_UMD_PSTATE_MIN_MCLK_SOCCLK: c_int = 1000;
pub const VANGOGH_UMD_PSTATE_MIN_MCLK_FCLK: c_int = 400;
pub const VANGOGH_UMD_PSTATE_MIN_MCLK_VCLK: c_int = 1000;
pub const VANGOGH_UMD_PSTATE_MIN_MCLK_DCLK: c_int = 800;
// RLC Power Status
pub const RLC_STATUS_OFF: c_int = 0;
pub const RLC_STATUS_NORMAL: c_int = 1;
