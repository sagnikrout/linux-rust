//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/sumod.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
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
// rcu
pub const RCU_FW_VERSION: c_uint = 0x30c;
pub const RCU_PWR_GATING_SEQ0: c_uint = 0x408;
pub const RCU_PWR_GATING_SEQ1: c_uint = 0x40c;
pub const RCU_PWR_GATING_CNTL: c_uint = 0x410;

pub const RCU_ALTVDDNB_NOTIFY: c_uint = 0x430;
pub const RCU_LCLK_SCALING_CNTL: c_uint = 0x434;

pub const RCU_PWR_GATING_CNTL_2: c_uint = 0x4a0;

pub const RCU_PWR_GATING_CNTL_3: c_uint = 0x4a4;

pub const RCU_PWR_GATING_CNTL_4: c_uint = 0x4a8;

// yes these two have the same address
pub const RCU_PWR_GATING_CNTL_5: c_uint = 0x504;
pub const RCU_GPU_BOOST_DISABLE: c_uint = 0x508;
pub const MCU_M3ARB_INDEX: c_uint = 0x504;
pub const MCU_M3ARB_PARAMS: c_uint = 0x508;
pub const RCU_GNB_PWR_REP_TIMER_CNTL: c_uint = 0x50C;
pub const RCU_SclkDpmTdpLimit01: c_uint = 0x514;
pub const RCU_SclkDpmTdpLimit23: c_uint = 0x518;
pub const RCU_SclkDpmTdpLimit47: c_uint = 0x51C;
pub const RCU_SclkDpmTdpLimitPG: c_uint = 0x520;
pub const GNB_TDP_LIMIT: c_uint = 0x540;
pub const RCU_BOOST_MARGIN: c_uint = 0x544;
pub const RCU_THROTTLE_MARGIN: c_uint = 0x548;
pub const SMU_PCIE_PG_ARGS: c_uint = 0x58C;
pub const SMU_PCIE_PG_ARGS_2: c_uint = 0x598;
pub const SMU_PCIE_PG_ARGS_3: c_uint = 0x59C;
// mmio
pub const RCU_STATUS: c_uint = 0x11c;

pub const GFX_INT_REQ: c_uint = 0x120;

pub const GFX_INT_STATUS: c_uint = 0x124;

pub const CG_SCLK_CNTL: c_uint = 0x600;

pub const CG_SCLK_STATUS: c_uint = 0x604;

pub const CG_DCLK_CNTL: c_uint = 0x610;

pub const CG_DCLK_STATUS: c_uint = 0x614;

pub const CG_VCLK_CNTL: c_uint = 0x618;

pub const CG_VCLK_STATUS: c_uint = 0x61c;
pub const GENERAL_PWRMGT: c_uint = 0x63c;

pub const SCLK_PWRMGT_CNTL: c_uint = 0x644;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x66c;

pub const CG_SCLK_DPM_CTRL: c_uint = 0x684;

pub const CG_SCLK_DPM_CTRL_2: c_uint = 0x688;
pub const CG_GCOOR: c_uint = 0x68c;

pub const CG_FTV: c_uint = 0x690;
pub const CG_FFCT_0: c_uint = 0x694;

pub const CG_GIT: c_uint = 0x6d8;

pub const CG_SCLK_DPM_CTRL_3: c_uint = 0x6e0;

pub const CG_SSP: c_uint = 0x6e8;

pub const CG_ACPI_CNTL: c_uint = 0x70c;

pub const CG_SCLK_DPM_CTRL_4: c_uint = 0x71c;

pub const CG_SCLK_DPM_CTRL_5: c_uint = 0x720;

pub const CG_SCLK_DPM_CTRL_6: c_uint = 0x724;
pub const CG_AT_0: c_uint = 0x728;

pub const CG_AT_1: c_uint = 0x72c;
pub const CG_AT_2: c_uint = 0x730;
pub const CG_THERMAL_INT: c_uint = 0x734;

pub const DIG_THERM_INTH_MASK: c_uint = 0x0000FF00;
pub const DIG_THERM_INTH_SHIFT: c_int = 8;

pub const DIG_THERM_INTL_MASK: c_uint = 0x00FF0000;
pub const DIG_THERM_INTL_SHIFT: c_int = 16;

pub const CG_AT_3: c_uint = 0x738;
pub const CG_AT_4: c_uint = 0x73c;
pub const CG_AT_5: c_uint = 0x740;
pub const CG_AT_6: c_uint = 0x744;
pub const CG_AT_7: c_uint = 0x748;
pub const CG_BSP_0: c_uint = 0x750;

pub const CG_CG_VOLTAGE_CNTL: c_uint = 0x770;

pub const CG_ACPI_VOLTAGE_CNTL: c_uint = 0x780;

pub const CG_DPM_VOLTAGE_CNTL: c_uint = 0x788;

pub const CG_PWR_GATING_CNTL: c_uint = 0x7ac;

pub const CG_CGTT_LOCAL_0: c_uint = 0x7d0;
pub const CG_CGTT_LOCAL_1: c_uint = 0x7d4;
pub const DEEP_SLEEP_CNTL: c_uint = 0x818;

pub const DEEP_SLEEP_CNTL2: c_uint = 0x81c;

pub const CG_SCRATCH2: c_uint = 0x824;
pub const CG_SCLK_DPM_CTRL_11: c_uint = 0x830;
pub const HW_REV: c_uint = 0x5564;

// 0 = A0, 1 = A1, 2 = B0, 3 = C0, etc.
pub const DOUT_SCRATCH3: c_uint = 0x611c;
pub const GB_ADDR_CONFIG: c_uint = 0x98f8;
