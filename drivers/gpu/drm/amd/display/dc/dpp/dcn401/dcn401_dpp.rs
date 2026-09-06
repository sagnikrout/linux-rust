//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dpp/dcn401/dcn401_dpp.h
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


// SPDX-License-Identifier: MIT
// Copyright 2023-2026 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

// Macro flag: #define TO_DCN401_DPP(dpp)\
// Macro flag: #define DPP_REG_LIST_SH_MASK_DCN401_COMMON(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dpp_registers {
    pub ALPHA_2BIT_LUT01: u32,
    pub ALPHA_2BIT_LUT23: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dpp_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dpp_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn401_dpp_registers,
    pub tf_shift: *const dcn401_dpp_shift,
    pub tf_mask: *const dcn401_dpp_mask,
    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: c_int,
    pub lb_memory_size: c_int,
    pub lb_bits_per_entry: c_int,
    pub is_write_to_ram_a_safe: bool,
    pub scl_data: scaler_data,
    pub pwl_data: pwl_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn401_dscl_mode_sel {
    DCN401_DSCL_MODE_SCALING_444_BYPASS = 0,
    DCN401_DSCL_MODE_SCALING_444_RGB_ENABLE = 1,
    DCN401_DSCL_MODE_SCALING_444_YCBCR_ENABLE = 2,
    DCN401_DSCL_MODE_SCALING_420_YCBCR_ENABLE = 3,
    DCN401_DSCL_MODE_SCALING_420_LUMA_BYPASS = 4,
    DCN401_DSCL_MODE_SCALING_420_CHROMA_BYPASS = 5,
    DCN401_DSCL_MODE_DSCL_BYPASS = 6
}

extern "C" {
    pub fn dpp401_read_state(dpp_base: *mut dpp, s: *mut dcn_dpp_state);
}
