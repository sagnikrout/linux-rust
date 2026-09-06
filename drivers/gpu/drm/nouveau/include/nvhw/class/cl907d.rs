//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/class/cl907d.h
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
// Copyright (c) 1993-2014, NVIDIA CORPORATION. All rights reserved.
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
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _cl907d_h_
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_4: c_uint = 0x00000004;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_4_DONE_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_4_DONE_TRUE: c_uint = 0x00000001;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20: c_uint = 0x00000014;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_LVDS18_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_LVDS18_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_LVDS24_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_LVDS24_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DUAL_LVDS18_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DUAL_LVDS18_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DUAL_LVDS24_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DUAL_LVDS24_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_TMDS_A_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_TMDS_A_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_TMDS_B_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_SINGLE_TMDS_B_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DUAL_TMDS_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DUAL_TMDS_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DP_A_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DP_A_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DP_B_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DP_B_TRUE: c_uint = 0x00000001;

pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DP_INTERLACE_FALSE: c_uint = 0x00000000;
pub const NV907D_CORE_NOTIFIER_3_CAPABILITIES_CAP_SOR0_20_DP_INTERLACE_TRUE: c_uint = 0x00000001;

// class methods

pub const NV907D_HEAD_SET_CRC_CONTROL_PRIMARY_OUTPUT_DAC__SIZE_1: c_int = 4;

pub const NV907D_HEAD_SET_CRC_CONTROL_PRIMARY_OUTPUT_RG__SIZE_1: c_int = 4;

pub const NV907D_HEAD_SET_CRC_CONTROL_PRIMARY_OUTPUT_SOR__SIZE_1: c_int = 8;

pub const NV907D_HEAD_SET_CRC_CONTROL_PRIMARY_OUTPUT_SF__SIZE_1: c_int = 4;

pub const NV907D_HEAD_SET_CRC_CONTROL_PRIMARY_OUTPUT_PIOR__SIZE_1: c_int = 8;

pub const NV907D_HEAD_SET_CRC_CONTROL_SECONDARY_OUTPUT_DAC__SIZE_1: c_int = 4;

pub const NV907D_HEAD_SET_CRC_CONTROL_SECONDARY_OUTPUT_RG__SIZE_1: c_int = 4;

pub const NV907D_HEAD_SET_CRC_CONTROL_SECONDARY_OUTPUT_SOR__SIZE_1: c_int = 8;

pub const NV907D_HEAD_SET_CRC_CONTROL_SECONDARY_OUTPUT_SF__SIZE_1: c_int = 4;

pub const NV907D_HEAD_SET_CRC_CONTROL_SECONDARY_OUTPUT_PIOR__SIZE_1: c_int = 8;

