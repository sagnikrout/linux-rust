//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv6xxd.h
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
// RV6xx power management
pub const SPLL_CNTL_MODE: c_uint = 0x60c;

pub const GENERAL_PWRMGT: c_uint = 0x618;

pub const MCLK_PWRMGT_CNTL: c_uint = 0x624;

pub const MPLL_FREQ_LEVEL_0: c_uint = 0x6e8;

pub const VID_RT: c_uint = 0x6f8;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x70c;

pub const VID_UPPER_GPIO_CNTL: c_uint = 0x740;

pub const CG_DISPLAY_GAP_CNTL: c_uint = 0x7dc;

pub const CG_THERMAL_CTRL: c_uint = 0x7f0;

pub const CG_SPLL_SPREAD_SPECTRUM_LOW: c_uint = 0x820;

pub const CG_MPLL_SPREAD_SPECTRUM: c_uint = 0x830;
pub const CITF_CNTL: c_uint = 0x200c;

pub const RAMCFG: c_uint = 0x2408;
pub const NOOFBANK_SHIFT: c_int = 0;
pub const NOOFBANK_MASK: c_uint = 0x00000001;
pub const NOOFRANK_SHIFT: c_int = 1;
pub const NOOFRANK_MASK: c_uint = 0x00000002;
pub const NOOFROWS_SHIFT: c_int = 2;
pub const NOOFROWS_MASK: c_uint = 0x0000001C;
pub const NOOFCOLS_SHIFT: c_int = 5;
pub const NOOFCOLS_MASK: c_uint = 0x00000060;
pub const CHANSIZE_SHIFT: c_int = 7;
pub const CHANSIZE_MASK: c_uint = 0x00000080;
pub const BURSTLENGTH_SHIFT: c_int = 8;
pub const BURSTLENGTH_MASK: c_uint = 0x00000100;

pub const SQM_RATIO: c_uint = 0x2424;

pub const ARB_RFSH_CNTL: c_uint = 0x2460;

pub const ARB_RFSH_RATE: c_uint = 0x2464;

pub const MC_SEQ_DRAM: c_uint = 0x2608;

pub const MC_SEQ_CMD: c_uint = 0x26c4;
pub const MC_SEQ_RESERVE_S: c_uint = 0x2890;
pub const MC_SEQ_RESERVE_M: c_uint = 0x2894;
pub const LVTMA_DATA_SYNCHRONIZATION: c_uint = 0x7adc;

pub const DCE3_LVTMA_DATA_SYNCHRONIZATION: c_uint = 0x7f98;
// PCIE indirect regs
pub const PCIE_P_CNTL: c_uint = 0x40;

// PCIE PORT indirect regs
pub const PCIE_LC_CNTL: c_uint = 0xa0;

pub const PCIE_LC_SPEED_CNTL: c_uint = 0xa4;

