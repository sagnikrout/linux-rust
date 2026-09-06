//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/btcd.h
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
// Copyright 2010 Advanced Micro Devices, Inc.
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
// Authors: Alex Deucher
//
// pm registers
pub const GENERAL_PWRMGT: c_uint = 0x63c;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x66c;

pub const CG_BIF_REQ_AND_RSP: c_uint = 0x7f4;

pub const CG_CLIENT_REQ_SHIFT: c_int = 0;

pub const CG_CLIENT_RESP_SHIFT: c_int = 8;

pub const CLIENT_CG_REQ_SHIFT: c_int = 16;

pub const CLIENT_CG_RESP_SHIFT: c_int = 24;
pub const SCLK_PSKIP_CNTL: c_uint = 0x8c0;

pub const PSKIP_ON_ALLOW_STOP_HI_SHIFT: c_int = 16;
pub const CG_ULV_CONTROL: c_uint = 0x8c8;
pub const CG_ULV_PARAMETER: c_uint = 0x8cc;
pub const MC_ARB_DRAM_TIMING: c_uint = 0x2774;
pub const MC_ARB_DRAM_TIMING2: c_uint = 0x2778;
pub const MC_ARB_RFSH_RATE: c_uint = 0x27b0;

pub const POWERMODE0_SHIFT: c_int = 0;

pub const POWERMODE1_SHIFT: c_int = 8;

pub const POWERMODE2_SHIFT: c_int = 16;

pub const POWERMODE3_SHIFT: c_int = 24;
pub const MC_ARB_BURST_TIME: c_uint = 0x2808;

pub const STATE0_SHIFT: c_int = 0;

pub const STATE1_SHIFT: c_int = 5;

pub const STATE2_SHIFT: c_int = 10;

pub const STATE3_SHIFT: c_int = 15;
pub const MC_SEQ_RAS_TIMING: c_uint = 0x28a0;
pub const MC_SEQ_CAS_TIMING: c_uint = 0x28a4;
pub const MC_SEQ_MISC_TIMING: c_uint = 0x28a8;
pub const MC_SEQ_MISC_TIMING2: c_uint = 0x28ac;
pub const MC_SEQ_RD_CTL_D0: c_uint = 0x28b4;
pub const MC_SEQ_RD_CTL_D1: c_uint = 0x28b8;
pub const MC_SEQ_WR_CTL_D0: c_uint = 0x28bc;
pub const MC_SEQ_WR_CTL_D1: c_uint = 0x28c0;
pub const MC_PMG_AUTO_CFG: c_uint = 0x28d4;
pub const MC_SEQ_STATUS_M: c_uint = 0x29f4;

pub const MC_SEQ_MISC0: c_uint = 0x2a00;
pub const MC_SEQ_MISC0_GDDR5_SHIFT: c_int = 28;
pub const MC_SEQ_MISC0_GDDR5_MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0_GDDR5_VALUE: c_int = 5;
pub const MC_SEQ_MISC1: c_uint = 0x2a04;
pub const MC_SEQ_RESERVE_M: c_uint = 0x2a08;
pub const MC_PMG_CMD_EMRS: c_uint = 0x2a0c;
pub const MC_SEQ_MISC3: c_uint = 0x2a2c;
pub const MC_SEQ_MISC5: c_uint = 0x2a54;
pub const MC_SEQ_MISC6: c_uint = 0x2a58;
pub const MC_SEQ_MISC7: c_uint = 0x2a64;
pub const MC_SEQ_CG: c_uint = 0x2a68;

pub const CG_SEQ_REQ_SHIFT: c_int = 0;

pub const CG_SEQ_RESP_SHIFT: c_int = 8;

pub const SEQ_CG_REQ_SHIFT: c_int = 16;

pub const SEQ_CG_RESP_SHIFT: c_int = 24;
pub const MC_SEQ_RAS_TIMING_LP: c_uint = 0x2a6c;
pub const MC_SEQ_CAS_TIMING_LP: c_uint = 0x2a70;
pub const MC_SEQ_MISC_TIMING_LP: c_uint = 0x2a74;
pub const MC_SEQ_MISC_TIMING2_LP: c_uint = 0x2a78;
pub const MC_SEQ_WR_CTL_D0_LP: c_uint = 0x2a7c;
pub const MC_SEQ_WR_CTL_D1_LP: c_uint = 0x2a80;
pub const MC_SEQ_PMG_CMD_EMRS_LP: c_uint = 0x2a84;
pub const MC_SEQ_PMG_CMD_MRS_LP: c_uint = 0x2a88;
pub const MC_PMG_CMD_MRS: c_uint = 0x2aac;
pub const MC_SEQ_RD_CTL_D0_LP: c_uint = 0x2b1c;
pub const MC_SEQ_RD_CTL_D1_LP: c_uint = 0x2b20;
pub const MC_PMG_CMD_MRS1: c_uint = 0x2b44;
pub const MC_SEQ_PMG_CMD_MRS1_LP: c_uint = 0x2b48;
pub const LB_SYNC_RESET_SEL: c_uint = 0x6b28;

pub const LB_SYNC_RESET_SEL_SHIFT: c_int = 0;
// PCIE link stuff
pub const PCIE_LC_SPEED_CNTL: c_uint = 0xa4 /* PCIE_P */;

