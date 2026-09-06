//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv740d.h
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
pub const CG_SPLL_FUNC_CNTL: c_uint = 0x600;

pub const CG_SPLL_FUNC_CNTL_2: c_uint = 0x604;

pub const CG_SPLL_FUNC_CNTL_3: c_uint = 0x608;

pub const MPLL_CNTL_MODE: c_uint = 0x61c;

pub const MPLL_AD_FUNC_CNTL: c_uint = 0x624;

pub const MPLL_AD_FUNC_CNTL_2: c_uint = 0x628;

pub const MPLL_DQ_FUNC_CNTL: c_uint = 0x62c;
pub const MPLL_DQ_FUNC_CNTL_2: c_uint = 0x630;
pub const MCLK_PWRMGT_CNTL: c_uint = 0x648;

pub const DLL_CNTL: c_uint = 0x64c;

pub const CG_SPLL_SPREAD_SPECTRUM: c_uint = 0x790;

pub const CG_SPLL_SPREAD_SPECTRUM_2: c_uint = 0x794;

pub const MPLL_SS1: c_uint = 0x85c;

pub const MPLL_SS2: c_uint = 0x860;

