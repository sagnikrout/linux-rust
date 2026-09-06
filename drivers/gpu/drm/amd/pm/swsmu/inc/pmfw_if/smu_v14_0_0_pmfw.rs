//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v14_0_0_pmfw.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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

// Macro flag: #define ENABLE_DEBUG_FEATURES
// Firmware features
// Feature Control Defines
pub const FEATURE_CCLK_DPM_BIT: c_int = 0;
pub const FEATURE_FAN_CONTROLLER_BIT: c_int = 1;
pub const FEATURE_DATA_CALCULATION_BIT: c_int = 2;
pub const FEATURE_PPT_BIT: c_int = 3;
pub const FEATURE_TDC_BIT: c_int = 4;
pub const FEATURE_THERMAL_BIT: c_int = 5;
pub const FEATURE_FIT_BIT: c_int = 6;
pub const FEATURE_EDC_BIT: c_int = 7;
pub const FEATURE_PLL_POWER_DOWN_BIT: c_int = 8;
pub const FEATURE_VDDOFF_BIT: c_int = 9;

pub const FEATURE_DS_MPM_BIT: c_int = 11;
pub const FEATURE_FCLK_DPM_BIT: c_int = 12;
pub const FEATURE_SOCCLK_DPM_BIT: c_int = 13;
pub const FEATURE_DS_MPIO_BIT: c_int = 14;
pub const FEATURE_LCLK_DPM_BIT: c_int = 15;
pub const FEATURE_SHUBCLK_DPM_BIT: c_int = 16;
pub const FEATURE_DCFCLK_DPM_BIT: c_int = 17;
pub const FEATURE_ISP_DPM_BIT: c_int = 18;
pub const FEATURE_IPU_DPM_BIT: c_int = 19;
pub const FEATURE_GFX_DPM_BIT: c_int = 20;
pub const FEATURE_DS_GFXCLK_BIT: c_int = 21;
pub const FEATURE_DS_SOCCLK_BIT: c_int = 22;
pub const FEATURE_DS_LCLK_BIT: c_int = 23;
pub const FEATURE_LOW_POWER_DCNCLKS_BIT: c_int = 24;
pub const FEATURE_DS_SHUBCLK_BIT: c_int = 25;
pub const FEATURE_RESERVED0_BIT: c_int = 26;
pub const FEATURE_ZSTATES_BIT: c_int = 27;
pub const FEATURE_IOMMUL2_PG_BIT: c_int = 28;
pub const FEATURE_DS_FCLK_BIT: c_int = 29;
pub const FEATURE_DS_SMNCLK_BIT: c_int = 30;
pub const FEATURE_DS_MP1CLK_BIT: c_int = 31;
pub const FEATURE_WHISPER_MODE_BIT: c_int = 32;
pub const FEATURE_SMU_LOW_POWER_BIT: c_int = 33;

pub const FEATURE_PSI_BIT: c_int = 36;
pub const FEATURE_PROCHOT_BIT: c_int = 37;
pub const FEATURE_CPUOFF_BIT: c_int = 38;
pub const FEATURE_STAPM_BIT: c_int = 39;
pub const FEATURE_S0I3_BIT: c_int = 40;
pub const FEATURE_DF_LIGHT_CSTATE: c_int = 41;
pub const FEATURE_PERF_LIMIT_BIT: c_int = 42;
pub const FEATURE_CORE_DLDO_BIT: c_int = 43;
pub const FEATURE_DVO_BIT: c_int = 44;

pub const FEATURE_CPPC_BIT: c_int = 46;
pub const FEATURE_CPPC_PREFERRED_CORES: c_int = 47;
pub const FEATURE_DF_CSTATES_BIT: c_int = 48;

pub const FEATURE_ATHUB_PG_BIT: c_int = 50;
pub const FEATURE_VDDOFF_ECO_BIT: c_int = 51;
pub const FEATURE_ZSTATES_ECO_BIT: c_int = 52;
pub const FEATURE_CC6_BIT: c_int = 53;
pub const FEATURE_DS_UMCCLK_BIT: c_int = 54;
pub const FEATURE_DS_ISPCLK_BIT: c_int = 55;
pub const FEATURE_DS_HSPCLK_BIT: c_int = 56;
pub const FEATURE_P3T_BIT: c_int = 57;
pub const FEATURE_DS_IPUCLK_BIT: c_int = 58;
pub const FEATURE_DS_VPECLK_BIT: c_int = 59;
pub const FEATURE_VPE_DPM_BIT: c_int = 60;

pub const NUM_FEATURES: c_int = 63;
// Firmware Header/Footer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU14_Firmware_Footer {
    pub Signature: u32,
}

pub type SMU14_Firmware_Footer = SMU14_Firmware_Footer;
// PSP3.0 Header Definition
// MP1_EXT_SCRATCH0
// MP1_EXT_SCRATCH1
// MP1_EXT_SCRATCH2
// MP1_EXT_SCRATCH3
// MP1_EXT_SCRATCH4
// MP1_EXT_SCRATCH5
// MP1_EXT_SCRATCH6 = RTOS threads' status
// MP1_EXT_SCRATCH7 = RTOS Current Job
// MP1_EXT_SCRATCH0
// MP1_EXT_SCRATCH1
// MP1_EXT_SCRATCH2
// MP1_EXT_SCRATCH3
// MP1_EXT_SCRATCH4
// MP1_EXT_SCRATCH5
// MP1_EXT_SCRATCH6 = RTOS threads' status
// MP1_EXT_SCRATCH7 = RTOS Current Job

