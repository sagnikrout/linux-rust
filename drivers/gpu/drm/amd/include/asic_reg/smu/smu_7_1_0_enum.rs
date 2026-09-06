//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/smu/smu_7_1_0_enum.h
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
// SMU_7_1_0 Register documentation
//
// Copyright (C) 2014  Advanced Micro Devices, Inc.
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
pub const CG_SRBM_START_ADDR: c_uint = 0x600;
pub const CG_SRBM_END_ADDR: c_uint = 0x8ff;
pub const RCU_CCF_DWORDS0: c_uint = 0x28;
pub const RCU_CCF_BITS0: c_uint = 0x500;
pub const RCU_CCF_DWORDS1: c_uint = 0x7f;
pub const RCU_CCF_BITS1: c_uint = 0x1000;
pub const RCU_SAM_BYTES: c_uint = 0x40;
pub const RCU_SAM_RTL_BYTES: c_uint = 0x40;
pub const KEYS_CHAIN_ADR: c_uint = 0x0;
pub const SAMU_KEY_SADR: c_uint = 0xa0;
pub const SAMU_KEY_EADR: c_uint = 0xdf;
pub const RCU_SMU_BYTES: c_uint = 0x11;
pub const RCU_SMU_RTL_BYTES: c_uint = 0x11;
pub const SMC_MSG_TEST: c_uint = 0x1;
pub const SMC_MSG_PHY_LN_OFF: c_uint = 0x2;
pub const SMC_MSG_PHY_LN_ON: c_uint = 0x3;
pub const SMC_MSG_DDI_PHY_OFF: c_uint = 0x4;
pub const SMC_MSG_DDI_PHY_ON: c_uint = 0x5;
pub const SMC_MSG_CASCADE_PLL_OFF: c_uint = 0x6;
pub const SMC_MSG_CASCADE_PLL_ON: c_uint = 0x7;
pub const SMC_MSG_PWR_OFF_x16: c_uint = 0x8;
pub const SMC_MSG_CONFIG_LCLK_DPM: c_uint = 0x9;
pub const SMC_MSG_FLUSH_DATA_CACHE: c_uint = 0xa;
pub const SMC_MSG_FLUSH_INSTRUCTION_CACHE: c_uint = 0xb;
pub const SMC_MSG_CONFIG_VPC_ACCUMULATOR: c_uint = 0xc;
pub const SMC_MSG_CONFIG_BAPM: c_uint = 0xd;
pub const SMC_MSG_CONFIG_TDC_LIMIT: c_uint = 0xe;
pub const SMC_MSG_CONFIG_LPMx: c_uint = 0xf;
pub const SMC_MSG_CONFIG_HTC_LIMIT: c_uint = 0x10;
pub const SMC_MSG_CONFIG_THERMAL_CNTL: c_uint = 0x11;
pub const SMC_MSG_CONFIG_VOLTAGE_CNTL: c_uint = 0x12;
pub const SMC_MSG_CONFIG_TDP_CNTL: c_uint = 0x13;
pub const SMC_MSG_EN_PM_CNTL: c_uint = 0x14;
pub const SMC_MSG_DIS_PM_CNTL: c_uint = 0x15;
pub const SMC_MSG_CONFIG_NBDPM: c_uint = 0x16;
pub const SMC_MSG_CONFIG_LOADLINE: c_uint = 0x17;
pub const SMC_MSG_ADJUST_LOADLINE: c_uint = 0x18;
pub const SMC_MSG_RESET: c_uint = 0x20;
pub const SMC_MSG_VOLTAGE: c_uint = 0x25;
pub const SMC_VERSION_MAJOR: c_uint = 0x7;
pub const SMC_VERSION_MINOR: c_uint = 0x0;
pub const SMC_HEADER_SIZE: c_uint = 0x40;
pub const ROM_SIGNATURE: c_uint = 0xaa55;
