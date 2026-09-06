//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/smu_types.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_message_type {
    SMU_MESSAGE_TYPES
    SMU_MSG_MAX_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_clk_type {
    SMU_GFXCLK,
    SMU_VCLK,
    SMU_DCLK,
    SMU_VCLK1,
    SMU_DCLK1,
    SMU_ECLK,
    SMU_SOCCLK,
    SMU_UCLK,
    SMU_DCEFCLK,
    SMU_DISPCLK,
    SMU_PIXCLK,
    SMU_PHYCLK,
    SMU_FCLK,
    SMU_SCLK,
    SMU_MCLK,
    SMU_PCIE,
    SMU_LCLK,
    SMU_ISPICLK,
    SMU_ISPXCLK,
    SMU_OD_CCLK,
    SMU_OD_SCLK,
    SMU_OD_MCLK,
    SMU_OD_FCLK,
    SMU_OD_VDDC_CURVE,
    SMU_OD_RANGE,
    SMU_OD_VDDGFX_OFFSET,
    SMU_OD_FAN_CURVE,
    SMU_OD_ACOUSTIC_LIMIT,
    SMU_OD_ACOUSTIC_TARGET,
    SMU_OD_FAN_TARGET_TEMPERATURE,
    SMU_OD_FAN_MINIMUM_PWM,
    SMU_OD_FAN_ZERO_RPM_ENABLE,
    SMU_OD_FAN_ZERO_RPM_STOP_TEMP,
    SMU_GL2CLK,
    SMU_CLK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_feature_mask {
    SMU_FEATURE_MASKS
    SMU_FEATURE_COUNT,
}

// Message category flags

// Firmware capability flags

