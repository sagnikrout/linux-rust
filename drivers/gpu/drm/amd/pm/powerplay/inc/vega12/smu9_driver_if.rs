//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/vega12/smu9_driver_if.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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
// IMPORTANT
// SMU TEAM: Always increment the interface version if
// any structure is changed in this file
//
pub const SMU9_DRIVER_IF_VERSION: c_uint = 0x10;
pub const PPTABLE_V12_SMU_VERSION: c_int = 1;
pub const NUM_GFXCLK_DPM_LEVELS: c_int = 16;
pub const NUM_VCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DCLK_DPM_LEVELS: c_int = 8;
pub const NUM_ECLK_DPM_LEVELS: c_int = 8;
pub const NUM_MP0CLK_DPM_LEVELS: c_int = 2;
pub const NUM_UCLK_DPM_LEVELS: c_int = 4;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DCEFCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DISPCLK_DPM_LEVELS: c_int = 8;
pub const NUM_PIXCLK_DPM_LEVELS: c_int = 8;
pub const NUM_PHYCLK_DPM_LEVELS: c_int = 8;
pub const NUM_LINK_LEVELS: c_int = 2;

pub const PPSMC_GeminiModeNone: c_int = 0;
pub const PPSMC_GeminiModeMaster: c_int = 1;
pub const PPSMC_GeminiModeSlave: c_int = 2;
pub const FEATURE_DPM_PREFETCHER_BIT: c_int = 0;
pub const FEATURE_DPM_GFXCLK_BIT: c_int = 1;
pub const FEATURE_DPM_UCLK_BIT: c_int = 2;
pub const FEATURE_DPM_SOCCLK_BIT: c_int = 3;
pub const FEATURE_DPM_UVD_BIT: c_int = 4;
pub const FEATURE_DPM_VCE_BIT: c_int = 5;
pub const FEATURE_ULV_BIT: c_int = 6;
pub const FEATURE_DPM_MP0CLK_BIT: c_int = 7;
pub const FEATURE_DPM_LINK_BIT: c_int = 8;
pub const FEATURE_DPM_DCEFCLK_BIT: c_int = 9;
pub const FEATURE_DS_GFXCLK_BIT: c_int = 10;
pub const FEATURE_DS_SOCCLK_BIT: c_int = 11;
pub const FEATURE_DS_LCLK_BIT: c_int = 12;
pub const FEATURE_PPT_BIT: c_int = 13;
pub const FEATURE_TDC_BIT: c_int = 14;
pub const FEATURE_THERMAL_BIT: c_int = 15;
pub const FEATURE_GFX_PER_CU_CG_BIT: c_int = 16;
pub const FEATURE_RM_BIT: c_int = 17;
pub const FEATURE_DS_DCEFCLK_BIT: c_int = 18;
pub const FEATURE_ACDC_BIT: c_int = 19;
pub const FEATURE_VR0HOT_BIT: c_int = 20;
pub const FEATURE_VR1HOT_BIT: c_int = 21;
pub const FEATURE_FW_CTF_BIT: c_int = 22;
pub const FEATURE_LED_DISPLAY_BIT: c_int = 23;
pub const FEATURE_FAN_CONTROL_BIT: c_int = 24;
pub const FEATURE_GFX_EDC_BIT: c_int = 25;
pub const FEATURE_GFXOFF_BIT: c_int = 26;
pub const FEATURE_CG_BIT: c_int = 27;
pub const FEATURE_ACG_BIT: c_int = 28;
pub const FEATURE_SPARE_29_BIT: c_int = 29;
pub const FEATURE_SPARE_30_BIT: c_int = 30;
pub const FEATURE_SPARE_31_BIT: c_int = 31;
pub const NUM_FEATURES: c_int = 32;

pub const DPM_OVERRIDE_DISABLE_SOCCLK_PID: c_uint = 0x00000001;
pub const DPM_OVERRIDE_DISABLE_UCLK_PID: c_uint = 0x00000002;
pub const DPM_OVERRIDE_ENABLE_VOLT_LINK_UVD_SOCCLK: c_uint = 0x00000004;
pub const DPM_OVERRIDE_ENABLE_VOLT_LINK_UVD_UCLK: c_uint = 0x00000008;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_VCLK_SOCCLK: c_uint = 0x00000010;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_VCLK_UCLK: c_uint = 0x00000020;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_DCLK_SOCCLK: c_uint = 0x00000040;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_DCLK_UCLK: c_uint = 0x00000080;
pub const DPM_OVERRIDE_ENABLE_VOLT_LINK_VCE_SOCCLK: c_uint = 0x00000100;
pub const DPM_OVERRIDE_ENABLE_VOLT_LINK_VCE_UCLK: c_uint = 0x00000200;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_ECLK_SOCCLK: c_uint = 0x00000400;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_ECLK_UCLK: c_uint = 0x00000800;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_GFXCLK_SOCCLK: c_uint = 0x00001000;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_GFXCLK_UCLK: c_uint = 0x00002000;
pub const DPM_OVERRIDE_ENABLE_GFXOFF_GFXCLK_SWITCH: c_uint = 0x00004000;
pub const DPM_OVERRIDE_ENABLE_GFXOFF_SOCCLK_SWITCH: c_uint = 0x00008000;
pub const DPM_OVERRIDE_ENABLE_GFXOFF_UCLK_SWITCH: c_uint = 0x00010000;
pub const VR_MAPPING_VR_SELECT_MASK: c_uint = 0x01;
pub const VR_MAPPING_VR_SELECT_SHIFT: c_uint = 0x00;
pub const VR_MAPPING_PLANE_SELECT_MASK: c_uint = 0x02;
pub const VR_MAPPING_PLANE_SELECT_SHIFT: c_uint = 0x01;
pub const PSI_SEL_VR0_PLANE0_PSI0: c_uint = 0x01;
pub const PSI_SEL_VR0_PLANE0_PSI1: c_uint = 0x02;
pub const PSI_SEL_VR0_PLANE1_PSI0: c_uint = 0x04;
pub const PSI_SEL_VR0_PLANE1_PSI1: c_uint = 0x08;
pub const PSI_SEL_VR1_PLANE0_PSI0: c_uint = 0x10;
pub const PSI_SEL_VR1_PLANE0_PSI1: c_uint = 0x20;
pub const PSI_SEL_VR1_PLANE1_PSI0: c_uint = 0x40;
pub const PSI_SEL_VR1_PLANE1_PSI1: c_uint = 0x80;
pub const THROTTLER_STATUS_PADDING_BIT: c_int = 0;
pub const THROTTLER_STATUS_TEMP_EDGE_BIT: c_int = 1;
pub const THROTTLER_STATUS_TEMP_HOTSPOT_BIT: c_int = 2;
pub const THROTTLER_STATUS_TEMP_HBM_BIT: c_int = 3;
pub const THROTTLER_STATUS_TEMP_VR_GFX_BIT: c_int = 4;
pub const THROTTLER_STATUS_TEMP_VR_MEM_BIT: c_int = 5;
pub const THROTTLER_STATUS_TEMP_LIQUID_BIT: c_int = 6;
pub const THROTTLER_STATUS_TEMP_PLX_BIT: c_int = 7;
pub const THROTTLER_STATUS_TEMP_SKIN_BIT: c_int = 8;
pub const THROTTLER_STATUS_TDC_GFX_BIT: c_int = 9;
pub const THROTTLER_STATUS_TDC_SOC_BIT: c_int = 10;
pub const THROTTLER_STATUS_PPT_BIT: c_int = 11;
pub const THROTTLER_STATUS_FIT_BIT: c_int = 12;
pub const THROTTLER_STATUS_PPM_BIT: c_int = 13;
pub const TABLE_TRANSFER_OK: c_uint = 0x0;
pub const TABLE_TRANSFER_FAILED: c_uint = 0xFF;
pub const WORKLOAD_DEFAULT_BIT: c_int = 0;
pub const WORKLOAD_PPLIB_FULL_SCREEN_3D_BIT: c_int = 1;
pub const WORKLOAD_PPLIB_POWER_SAVING_BIT: c_int = 2;
pub const WORKLOAD_PPLIB_VIDEO_BIT: c_int = 3;
pub const WORKLOAD_PPLIB_VR_BIT: c_int = 4;
pub const WORKLOAD_PPLIB_COMPUTE_BIT: c_int = 5;
pub const WORKLOAD_PPLIB_CUSTOM_BIT: c_int = 6;
pub const WORKLOAD_PPLIB_COUNT: c_int = 7;

pub const NUM_WM_RANGES: c_int = 4;
pub const TABLE_PPTABLE: c_int = 0;
pub const TABLE_WATERMARKS: c_int = 1;
pub const TABLE_AVFS: c_int = 2;
pub const TABLE_AVFS_PSM_DEBUG: c_int = 3;
pub const TABLE_AVFS_FUSE_OVERRIDE: c_int = 4;
pub const TABLE_PMSTATUSLOG: c_int = 5;
pub const TABLE_SMU_METRICS: c_int = 6;
pub const TABLE_DRIVER_SMU_CONFIG: c_int = 7;
pub const TABLE_ACTIVITY_MONITOR_COEFF: c_int = 8;
pub const TABLE_OVERDRIVE: c_int = 9;
pub const TABLE_COUNT: c_int = 10;
pub const UCLK_SWITCH_SLOW: c_int = 0;
pub const UCLK_SWITCH_FAST: c_int = 1;
pub const SQ_Enable_MASK: c_uint = 0x1;
pub const SQ_IR_MASK: c_uint = 0x2;
pub const SQ_PCC_MASK: c_uint = 0x4;
pub const SQ_EDC_MASK: c_uint = 0x8;
pub const TCP_Enable_MASK: c_uint = 0x100;
pub const TCP_IR_MASK: c_uint = 0x200;
pub const TCP_PCC_MASK: c_uint = 0x400;
pub const TCP_EDC_MASK: c_uint = 0x800;
pub const TD_Enable_MASK: c_uint = 0x10000;
pub const TD_IR_MASK: c_uint = 0x20000;
pub const TD_PCC_MASK: c_uint = 0x40000;
pub const TD_EDC_MASK: c_uint = 0x80000;
pub const DB_Enable_MASK: c_uint = 0x1000000;
pub const DB_IR_MASK: c_uint = 0x2000000;
pub const DB_PCC_MASK: c_uint = 0x4000000;
pub const DB_EDC_MASK: c_uint = 0x8000000;
pub const SQ_Enable_SHIFT: c_int = 0;
pub const SQ_IR_SHIFT: c_int = 1;
pub const SQ_PCC_SHIFT: c_int = 2;
pub const SQ_EDC_SHIFT: c_int = 3;
pub const TCP_Enable_SHIFT: c_int = 8;
pub const TCP_IR_SHIFT: c_int = 9;
pub const TCP_PCC_SHIFT: c_int = 10;
pub const TCP_EDC_SHIFT: c_int = 11;
pub const TD_Enable_SHIFT: c_int = 16;
pub const TD_IR_SHIFT: c_int = 17;
pub const TD_PCC_SHIFT: c_int = 18;
pub const TD_EDC_SHIFT: c_int = 19;
pub const DB_Enable_SHIFT: c_int = 24;
pub const DB_IR_SHIFT: c_int = 25;
pub const DB_PCC_SHIFT: c_int = 26;
pub const DB_EDC_SHIFT: c_int = 27;
pub const REMOVE_FMAX_MARGIN_BIT: c_uint = 0x0;
pub const REMOVE_DCTOL_MARGIN_BIT: c_uint = 0x1;
pub const REMOVE_PLATFORM_MARGIN_BIT: c_uint = 0x2;
