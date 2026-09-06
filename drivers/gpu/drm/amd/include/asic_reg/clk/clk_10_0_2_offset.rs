//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/clk/clk_10_0_2_offset.h
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
// Copyright (C) 2019  Advanced Micro Devices, Inc.
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

// Macro flag: #define _clk_10_0_2_OFFSET_HEADER
// addressBlock: clk_clk1_0_SmuClkDec
// base address: 0x5b800
pub const mmCLK1_CLK_PLL_REQ: c_uint = 0x000f;
pub const mmCLK1_CLK_PLL_REQ_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK0_BYPASS_CNTL: c_uint = 0x0049;
pub const mmCLK1_CLK0_BYPASS_CNTL_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK1_BYPASS_CNTL: c_uint = 0x0053;
pub const mmCLK1_CLK1_BYPASS_CNTL_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK2_BYPASS_CNTL: c_uint = 0x005d;
pub const mmCLK1_CLK2_BYPASS_CNTL_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK2_STATUS: c_uint = 0x005e;
pub const mmCLK1_CLK2_STATUS_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK3_DFS_CNTL: c_uint = 0x005f;
pub const mmCLK1_CLK3_DFS_CNTL_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK3_DS_CNTL: c_uint = 0x0060;
pub const mmCLK1_CLK3_DS_CNTL_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK3_ALLOW_DS: c_uint = 0x0061;
pub const mmCLK1_CLK3_ALLOW_DS_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK3_BYPASS_CNTL: c_uint = 0x0067;
pub const mmCLK1_CLK3_BYPASS_CNTL_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK0_CURRENT_CNT: c_uint = 0x008a;
pub const mmCLK1_CLK0_CURRENT_CNT_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK1_CURRENT_CNT: c_uint = 0x008b;
pub const mmCLK1_CLK1_CURRENT_CNT_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK2_CURRENT_CNT: c_uint = 0x008c;
pub const mmCLK1_CLK2_CURRENT_CNT_BASE_IDX: c_int = 1;
pub const mmCLK1_CLK3_CURRENT_CNT: c_uint = 0x008d;
pub const mmCLK1_CLK3_CURRENT_CNT_BASE_IDX: c_int = 1;
