//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu9.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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
// Feature Control Defines
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
pub const FEATURE_AVFS_BIT: c_int = 10;
pub const FEATURE_DS_GFXCLK_BIT: c_int = 11;
pub const FEATURE_DS_SOCCLK_BIT: c_int = 12;
pub const FEATURE_DS_LCLK_BIT: c_int = 13;
pub const FEATURE_PPT_BIT: c_int = 14;
pub const FEATURE_TDC_BIT: c_int = 15;
pub const FEATURE_THERMAL_BIT: c_int = 16;
pub const FEATURE_GFX_PER_CU_CG_BIT: c_int = 17;
pub const FEATURE_RM_BIT: c_int = 18;
pub const FEATURE_DS_DCEFCLK_BIT: c_int = 19;
pub const FEATURE_ACDC_BIT: c_int = 20;
pub const FEATURE_VR0HOT_BIT: c_int = 21;
pub const FEATURE_VR1HOT_BIT: c_int = 22;
pub const FEATURE_FW_CTF_BIT: c_int = 23;
pub const FEATURE_LED_DISPLAY_BIT: c_int = 24;
pub const FEATURE_FAN_CONTROL_BIT: c_int = 25;
pub const FEATURE_FAST_PPT_BIT: c_int = 26;
pub const FEATURE_GFX_EDC_BIT: c_int = 27;
pub const FEATURE_ACG_BIT: c_int = 28;
pub const FEATURE_PCC_LIMIT_CONTROL_BIT: c_int = 29;
pub const FEATURE_SPARE_30_BIT: c_int = 30;
pub const FEATURE_SPARE_31_BIT: c_int = 31;
pub const NUM_FEATURES: c_int = 32;

// Workload types
pub const WORKLOAD_VR_BIT: c_int = 0;
pub const WORKLOAD_FRTC_BIT: c_int = 1;
pub const WORKLOAD_VIDEO_BIT: c_int = 2;
pub const WORKLOAD_COMPUTE_BIT: c_int = 3;
pub const NUM_WORKLOADS: c_int = 4;
// ULV Client Masks
pub const ULV_CLIENT_RLC_MASK: c_uint = 0x00000001;
pub const ULV_CLIENT_UVD_MASK: c_uint = 0x00000002;
pub const ULV_CLIENT_VCE_MASK: c_uint = 0x00000004;
pub const ULV_CLIENT_SDMA0_MASK: c_uint = 0x00000008;
pub const ULV_CLIENT_SDMA1_MASK: c_uint = 0x00000010;
pub const ULV_CLIENT_JPEG_MASK: c_uint = 0x00000020;
pub const ULV_CLIENT_GFXCLK_DPM_MASK: c_uint = 0x00000040;
pub const ULV_CLIENT_UVD_DPM_MASK: c_uint = 0x00000080;
pub const ULV_CLIENT_VCE_DPM_MASK: c_uint = 0x00000100;
pub const ULV_CLIENT_MP0CLK_DPM_MASK: c_uint = 0x00000200;
pub const ULV_CLIENT_UCLK_DPM_MASK: c_uint = 0x00000400;
pub const ULV_CLIENT_SOCCLK_DPM_MASK: c_uint = 0x00000800;
pub const ULV_CLIENT_DCEFCLK_DPM_MASK: c_uint = 0x00001000;
// MP1_EXT_SCRATCH0
// MP1_EXT_SCRATCH1
// MP1_EXT_SCRATCH2-7

