//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/clk/clk_11_0_1_sh_mask.h
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
// Copyright (C) 2021 Advanced Micro Devices, Inc.
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

// Macro flag: #define _clk_11_0_1_SH_MASK_HEADER
// CLK4_0_CLK4_CLK_PLL_REQ
pub const CLK4_0_CLK4_CLK_PLL_REQ__FbMult_int__SHIFT: c_uint = 0x0;
pub const CLK4_0_CLK4_CLK_PLL_REQ__PllSpineDiv__SHIFT: c_uint = 0xc;
pub const CLK4_0_CLK4_CLK_PLL_REQ__FbMult_frac__SHIFT: c_uint = 0x10;
pub const CLK4_0_CLK4_CLK_PLL_REQ__FbMult_int_MASK: c_uint = 0x000001FFL;
pub const CLK4_0_CLK4_CLK_PLL_REQ__PllSpineDiv_MASK: c_uint = 0x0000F000L;
pub const CLK4_0_CLK4_CLK_PLL_REQ__FbMult_frac_MASK: c_uint = 0xFFFF0000L;
// CLK4_0_CLK4_CLK2_CURRENT_CNT
pub const CLK4_0_CLK4_CLK2_CURRENT_CNT__CURRENT_COUNT__SHIFT: c_uint = 0x0;
pub const CLK4_0_CLK4_CLK2_CURRENT_CNT__CURRENT_COUNT_MASK: c_uint = 0xFFFFFFFFL;
