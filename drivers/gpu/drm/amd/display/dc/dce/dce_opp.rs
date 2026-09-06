//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_opp.h
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


// Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Macro flag: #define FROM_DCE11_OPP(opp)\
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dce110_opp_reg_type {
    DCE110_OPP_REG_DCP = 0,
    DCE110_OPP_REG_DCFE,
    DCE110_OPP_REG_FMT,

    DCE110_OPP_REG_MAX
}

// Macro flag: #define OPP_COMMON_MASK_SH_LIST_DCE_COMMON_BASE(mask_sh)\
// Macro flag: #define OPP_COMMON_MASK_SH_LIST_DCE_110(mask_sh)\
// Macro flag: #define OPP_COMMON_MASK_SH_LIST_DCE_100(mask_sh)\
// Macro flag: #define OPP_COMMON_MASK_SH_LIST_DCE_112(mask_sh)\
// Macro flag: #define OPP_COMMON_MASK_SH_LIST_DCE_80(mask_sh)\
// Macro flag: #define OPP_COMMON_MASK_SH_LIST_DCE_120(mask_sh)\

// Macro flag: #define OPP_COMMON_MASK_SH_LIST_DCE_60(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_opp_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_opp_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_opp_registers {
    pub FMT_DYNAMIC_EXP_CNTL: u32,
    pub FMT_BIT_DEPTH_CONTROL: u32,
    pub FMT_CONTROL: u32,
    pub FMT_DITHER_RAND_R_SEED: u32,
    pub FMT_DITHER_RAND_G_SEED: u32,
    pub FMT_DITHER_RAND_B_SEED: u32,
    pub FMT_TEMPORAL_DITHER_PATTERN_CONTROL: u32,
    pub FMT_TEMPORAL_DITHER_PROGRAMMABLE_PATTERN_S_MATRIX: u32,
    pub FMT_TEMPORAL_DITHER_PROGRAMMABLE_PATTERN_T_MATRIX: u32,
    pub CONTROL: u32,
    pub FMT_CLAMP_CNTL: u32,
    pub FMT_CLAMP_COMPONENT_R: u32,
    pub FMT_CLAMP_COMPONENT_G: u32,
    pub FMT_CLAMP_COMPONENT_B: u32,
}

// OPP RELATED
// Macro flag: #define TO_DCE110_OPP(opp)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_opp {
    pub base: output_pixel_processor,
    pub regs: *const dce_opp_registers,
    pub opp_shift: *const dce_opp_shift,
    pub opp_mask: *const dce_opp_mask,
}

extern "C" {
    pub fn dce110_opp_destroy(opp: *mut output_pixel_processor);
}
// FORMATTER RELATED
