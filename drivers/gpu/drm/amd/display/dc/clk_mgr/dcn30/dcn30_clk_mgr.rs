//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn30/dcn30_clk_mgr.h
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
// Authors: AMD
//
// CLK1_CLK_PLL_REQ

pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_int__SHIFT: c_uint = 0x0;
pub const CLK11_CLK1_CLK_PLL_REQ__PllSpineDiv__SHIFT: c_uint = 0xc;
pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_frac__SHIFT: c_uint = 0x10;
pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_int_MASK: c_uint = 0x000001FFL;
pub const CLK11_CLK1_CLK_PLL_REQ__PllSpineDiv_MASK: c_uint = 0x0000F000L;
pub const CLK11_CLK1_CLK_PLL_REQ__FbMult_frac_MASK: c_uint = 0xFFFF0000L;
// CLK1_CLK0_DFS_CNTL
pub const CLK11_CLK1_CLK0_DFS_CNTL__CLK0_DIVIDER__SHIFT: c_uint = 0x0;
pub const CLK11_CLK1_CLK0_DFS_CNTL__CLK0_DIVIDER_MASK: c_uint = 0x0000007FL;
// DPREF clock related
pub const CLK0_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: c_uint = 0x0;
pub const CLK0_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: c_uint = 0x0000007FL;
pub const CLK1_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: c_uint = 0x0;
pub const CLK1_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: c_uint = 0x0000007FL;
pub const CLK2_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: c_uint = 0x0;
pub const CLK2_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: c_uint = 0x0000007FL;
pub const CLK3_CLK3_DFS_CNTL__CLK3_DIVIDER__SHIFT: c_uint = 0x0;
pub const CLK3_CLK3_DFS_CNTL__CLK3_DIVIDER_MASK: c_uint = 0x0000007FL;
// CLK3_0_CLK3_CLK_PLL_REQ
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_int__SHIFT: c_uint = 0x0;
pub const CLK3_0_CLK3_CLK_PLL_REQ__PllSpineDiv__SHIFT: c_uint = 0xc;
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_frac__SHIFT: c_uint = 0x10;
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_int_MASK: c_uint = 0x000001FFL;
pub const CLK3_0_CLK3_CLK_PLL_REQ__PllSpineDiv_MASK: c_uint = 0x0000F000L;
pub const CLK3_0_CLK3_CLK_PLL_REQ__FbMult_frac_MASK: c_uint = 0xFFFF0000L;
pub const mmCLK0_CLK2_DFS_CNTL: c_uint = 0x16C55;
pub const mmCLK00_CLK0_CLK2_DFS_CNTL: c_uint = 0x16C55;
pub const mmCLK01_CLK0_CLK2_DFS_CNTL: c_uint = 0x16E55;
pub const mmCLK02_CLK0_CLK2_DFS_CNTL: c_uint = 0x17055;
pub const mmCLK0_CLK3_DFS_CNTL: c_uint = 0x16C60;
pub const mmCLK00_CLK0_CLK3_DFS_CNTL: c_uint = 0x16C60;
pub const mmCLK01_CLK0_CLK3_DFS_CNTL: c_uint = 0x16E60;
pub const mmCLK02_CLK0_CLK3_DFS_CNTL: c_uint = 0x17060;
pub const mmCLK03_CLK0_CLK3_DFS_CNTL: c_uint = 0x17260;
pub const mmCLK0_CLK_PLL_REQ: c_uint = 0x16C10;
pub const mmCLK00_CLK0_CLK_PLL_REQ: c_uint = 0x16C10;
pub const mmCLK01_CLK0_CLK_PLL_REQ: c_uint = 0x16E10;
pub const mmCLK02_CLK0_CLK_PLL_REQ: c_uint = 0x17010;
pub const mmCLK03_CLK0_CLK_PLL_REQ: c_uint = 0x17210;
pub const mmCLK1_CLK_PLL_REQ: c_uint = 0x1B00D;
pub const mmCLK10_CLK1_CLK_PLL_REQ: c_uint = 0x1B00D;
pub const mmCLK11_CLK1_CLK_PLL_REQ: c_uint = 0x1B20D;
pub const mmCLK12_CLK1_CLK_PLL_REQ: c_uint = 0x1B40D;
pub const mmCLK13_CLK1_CLK_PLL_REQ: c_uint = 0x1B60D;
pub const mmCLK2_CLK_PLL_REQ: c_uint = 0x17E0D;
// AMCLK
pub const mmCLK11_CLK1_CLK0_DFS_CNTL: c_uint = 0x1B23F;
pub const mmCLK11_CLK1_CLK_PLL_REQ: c_uint = 0x1B20D;

extern "C" {
    pub fn dcn3_init_clocks(clk_mgr_base: *mut clk_mgr);
}
extern "C" {
    pub fn dcn3_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal);
}
