//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv250d.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
pub const R_00000D_SCLK_CNTL_M6: c_uint = 0x00000D;

pub const C_00000D_SCLK_SRC_SEL: c_uint = 0xFFFFFFF8;

pub const C_00000D_CP_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFF7;

pub const C_00000D_HDP_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFEF;

pub const C_00000D_TV_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFDF;

pub const C_00000D_E2_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFBF;

pub const C_00000D_SE_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFF7F;

pub const C_00000D_IDCT_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFEFF;

pub const C_00000D_VIP_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFDFF;

pub const C_00000D_RE_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFBFF;

pub const C_00000D_PB_MAX_DYN_STOP_LAT: c_uint = 0xFFFFF7FF;

pub const C_00000D_TAM_MAX_DYN_STOP_LAT: c_uint = 0xFFFFEFFF;

pub const C_00000D_TDM_MAX_DYN_STOP_LAT: c_uint = 0xFFFFDFFF;

pub const C_00000D_RB_MAX_DYN_STOP_LAT: c_uint = 0xFFFFBFFF;

pub const C_00000D_FORCE_DISP2: c_uint = 0xFFFF7FFF;

pub const C_00000D_FORCE_CP: c_uint = 0xFFFEFFFF;

pub const C_00000D_FORCE_HDP: c_uint = 0xFFFDFFFF;

pub const C_00000D_FORCE_DISP1: c_uint = 0xFFFBFFFF;

pub const C_00000D_FORCE_TOP: c_uint = 0xFFF7FFFF;

pub const C_00000D_FORCE_E2: c_uint = 0xFFEFFFFF;

pub const C_00000D_FORCE_SE: c_uint = 0xFFDFFFFF;

pub const C_00000D_FORCE_IDCT: c_uint = 0xFFBFFFFF;

pub const C_00000D_FORCE_VIP: c_uint = 0xFF7FFFFF;

pub const C_00000D_FORCE_RE: c_uint = 0xFEFFFFFF;

pub const C_00000D_FORCE_PB: c_uint = 0xFDFFFFFF;

pub const C_00000D_FORCE_TAM: c_uint = 0xFBFFFFFF;

pub const C_00000D_FORCE_TDM: c_uint = 0xF7FFFFFF;

pub const C_00000D_FORCE_RB: c_uint = 0xEFFFFFFF;

pub const C_00000D_FORCE_TV_SCLK: c_uint = 0xDFFFFFFF;

pub const C_00000D_FORCE_SUBPIC: c_uint = 0xBFFFFFFF;

pub const C_00000D_FORCE_OV0: c_uint = 0x7FFFFFFF;
