//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/trinityd.h
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
// cg
pub const CG_CGTT_LOCAL_0: c_uint = 0x0;
pub const CG_CGTT_LOCAL_1: c_uint = 0x1;
// smc
pub const SMU_SCLK_DPM_STATE_0_CNTL_0: c_uint = 0x1f000;

pub const SMU_SCLK_DPM_STATE_0_CNTL_1: c_uint = 0x1f004;

pub const SMU_SCLK_DPM_STATE_0_CNTL_3: c_uint = 0x1f00c;

pub const SMU_SCLK_DPM_STATE_0_AT: c_uint = 0x1f010;

pub const SMU_SCLK_DPM_STATE_0_PG_CNTL: c_uint = 0x1f014;

pub const SMU_SCLK_DPM_STATE_1_CNTL_0: c_uint = 0x1f020;
pub const SMU_SCLK_DPM_CNTL: c_uint = 0x1f100;

pub const SMU_SCLK_DPM_TT_CNTL: c_uint = 0x1f108;

pub const SMU_SCLK_DPM_TTT: c_uint = 0x1f10c;

pub const SMU_UVD_DPM_STATES: c_uint = 0x1f1a0;
pub const SMU_UVD_DPM_CNTL: c_uint = 0x1f1a4;
pub const SMU_S_PG_CNTL: c_uint = 0x1f118;

pub const GFX_POWER_GATING_CNTL: c_uint = 0x1f38c;

pub const PM_CONFIG: c_uint = 0x1f428;

pub const PM_I_CNTL_1: c_uint = 0x1f464;

pub const PM_TP: c_uint = 0x1f468;
pub const NB_PSTATE_CONFIG: c_uint = 0x1f5f8;

pub const DC_CAC_VALUE: c_uint = 0x1f908;
pub const GPU_CAC_AVRG_CNTL: c_uint = 0x1f920;

pub const CC_SMU_MISC_FUSES: c_uint = 0xe0001004;

pub const CC_SMU_TST_EFUSE1_MISC: c_uint = 0xe000101c;

pub const SMU_SCRATCH_A: c_uint = 0xe0003024;
pub const SMU_SCRATCH0: c_uint = 0xe0003040;
// mmio
pub const SMC_INT_REQ: c_uint = 0x220;
pub const SMC_MESSAGE_0: c_uint = 0x22c;
pub const SMC_RESP_0: c_uint = 0x230;
pub const GENERAL_PWRMGT: c_uint = 0x670;

pub const SCLK_PWRMGT_CNTL: c_uint = 0x678;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x684;

pub const CG_GIPOTS: c_uint = 0x6d8;

pub const CG_PG_CTRL: c_uint = 0x6e0;

pub const CG_MISC_REG: c_uint = 0x708;
pub const CG_THERMAL_INT_CTRL: c_uint = 0x738;

pub const CG_CG_VOLTAGE_CNTL: c_uint = 0x770;

pub const HW_REV: c_uint = 0x5564;

// 0 = A0, 1 = A1, 2 = B0, 3 = C0, etc.
pub const CGTS_SM_CTRL_REG: c_uint = 0x9150;
pub const GB_ADDR_CONFIG: c_uint = 0x98f8;
