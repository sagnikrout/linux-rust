//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/smu15/smu_v15_0_8_ppt.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
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
pub const SMU_15_0_8_NUM_XGMI_LINKS: c_int = 8;
pub const SMU_15_0_8_MAX_GFX_CLKS: c_int = 8;
pub const SMU_15_0_8_MAX_CLKS: c_int = 4;
pub const SMU_15_0_8_MAX_XCC: c_int = 8;
pub const SMU_15_0_8_MAX_VCN: c_int = 4;
pub const SMU_15_0_8_MAX_JPEG: c_int = 40;
pub const SMU_15_0_8_MAX_AID: c_int = 2;
pub const SMU_15_0_8_MAX_MID: c_int = 2;
pub const SMU_15_0_8_MAX_HBM_STACKS: c_int = 12;
extern "C" {
    pub fn smu_v15_0_8_set_ppt_funcs(smu: *mut smu_context);
}

// SMUv 15.0.8 GPU metrics

// Maximum temperature sensor counts for system metrics
pub const SMU_15_0_8_MAX_SYSTEM_TEMP_ENTRIES: c_int = 32;
pub const SMU_15_0_8_MAX_NODE_TEMP_ENTRIES: c_int = 12;
pub const SMU_15_0_8_MAX_VR_TEMP_ENTRIES: c_int = 22;
// SMUv 15.0.8 GPU board temperature metrics

// SMUv 15.0.8 Baseboard temperature metrics - ID-based approach

