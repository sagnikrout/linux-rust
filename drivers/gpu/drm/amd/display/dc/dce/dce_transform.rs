//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_transform.h
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
// Copyright 2012-16 Advanced Micro Devices, Inc.
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

// Macro flag: #define TO_DCE_TRANSFORM(transform)\
pub const LB_TOTAL_NUMBER_OF_ENTRIES: c_int = 1712;
pub const LB_BITS_PER_ENTRY: c_int = 144;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_transform_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_transform_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_transform_registers {

    pub DATA_FORMAT: u32,

    pub LB_DATA_FORMAT: u32,
    pub GAMUT_REMAP_CONTROL: u32,
    pub GAMUT_REMAP_C11_C12: u32,
    pub GAMUT_REMAP_C13_C14: u32,
    pub GAMUT_REMAP_C21_C22: u32,
    pub GAMUT_REMAP_C23_C24: u32,
    pub GAMUT_REMAP_C31_C32: u32,
    pub GAMUT_REMAP_C33_C34: u32,
    pub OUTPUT_CSC_C11_C12: u32,
    pub OUTPUT_CSC_C13_C14: u32,
    pub OUTPUT_CSC_C21_C22: u32,
    pub OUTPUT_CSC_C23_C24: u32,
    pub OUTPUT_CSC_C31_C32: u32,
    pub OUTPUT_CSC_C33_C34: u32,
    pub OUTPUT_CSC_CONTROL: u32,
    pub DCFE_MEM_LIGHT_SLEEP_CNTL: u32,
    pub REGAMMA_CNTLA_START_CNTL: u32,
    pub REGAMMA_CNTLA_SLOPE_CNTL: u32,
    pub REGAMMA_CNTLA_END_CNTL1: u32,
    pub REGAMMA_CNTLA_END_CNTL2: u32,
    pub REGAMMA_CNTLA_REGION_0_1: u32,
    pub REGAMMA_CNTLA_REGION_2_3: u32,
    pub REGAMMA_CNTLA_REGION_4_5: u32,
    pub REGAMMA_CNTLA_REGION_6_7: u32,
    pub REGAMMA_CNTLA_REGION_8_9: u32,
    pub REGAMMA_CNTLA_REGION_10_11: u32,
    pub REGAMMA_CNTLA_REGION_12_13: u32,
    pub REGAMMA_CNTLA_REGION_14_15: u32,
    pub REGAMMA_LUT_WRITE_EN_MASK: u32,
    pub REGAMMA_LUT_INDEX: u32,
    pub REGAMMA_LUT_DATA: u32,
    pub REGAMMA_CONTROL: u32,
    pub DENORM_CONTROL: u32,
    pub DCP_SPATIAL_DITHER_CNTL: u32,
    pub OUT_ROUND_CONTROL: u32,
    pub OUT_CLAMP_CONTROL_R_CR: u32,
    pub OUT_CLAMP_CONTROL_G_Y: u32,
    pub OUT_CLAMP_CONTROL_B_CB: u32,
    pub SCL_MODE: u32,
    pub SCL_TAP_CONTROL: u32,
    pub SCL_CONTROL: u32,
    pub SCL_BYPASS_CONTROL: u32,
    pub EXT_OVERSCAN_LEFT_RIGHT: u32,
    pub EXT_OVERSCAN_TOP_BOTTOM: u32,
    pub SCL_VERT_FILTER_CONTROL: u32,
    pub SCL_HORZ_FILTER_CONTROL: u32,
    pub DCFE_MEM_PWR_CTRL: u32,
    pub DCFE_MEM_PWR_STATUS: u32,
    pub SCL_COEF_RAM_SELECT: u32,
    pub SCL_COEF_RAM_TAP_DATA: u32,
    pub VIEWPORT_START: u32,
    pub VIEWPORT_SIZE: u32,
    pub SCL_HORZ_FILTER_SCALE_RATIO: u32,
    pub SCL_VERT_FILTER_SCALE_RATIO: u32,
    pub SCL_HORZ_FILTER_INIT: u32,

    pub SCL_SCALER_ENABLE: u32,
    pub SCL_HORZ_FILTER_INIT_RGB_LUMA: u32,
    pub SCL_HORZ_FILTER_INIT_CHROMA: u32,

    pub SCL_VERT_FILTER_INIT: u32,
    pub SCL_AUTOMATIC_MODE_CONTROL: u32,

    pub DC_LB_MEMORY_SPLIT: u32,
    pub DC_LB_MEM_SIZE: u32,

    pub LB_MEMORY_CTRL: u32,
    pub SCL_UPDATE: u32,
    pub SCL_F_SHARP_CONTROL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_int_and_frac {
    pub integer: u32,
    pub fraction: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scl_ratios_inits {
    pub h_int_scale_ratio: u32,
    pub v_int_scale_ratio: u32,
    pub h_init: init_int_and_frac,
    pub v_init: init_int_and_frac,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclh_ratios_inits {
    pub h_int_scale_ratio: u32,
    pub v_int_scale_ratio: u32,
    pub h_init_luma: init_int_and_frac,
    pub h_init_chroma: init_int_and_frac,
    pub v_init: init_int_and_frac,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ram_filter_type {
    FILTER_TYPE_RGB_Y_VERTICAL	= 0, /* 0 - RGB/Y Vertical filter */
    FILTER_TYPE_CBCR_VERTICAL	= 1, /* 1 - CbCr  Vertical filter */
    FILTER_TYPE_RGB_Y_HORIZONTAL	= 2, /* 1 - RGB/Y Horizontal filter */
    FILTER_TYPE_CBCR_HORIZONTAL	= 3, /* 3 - CbCr  Horizontal filter */
    FILTER_TYPE_ALPHA_VERTICAL	= 4, /* 4 - Alpha Vertical filter. */
    FILTER_TYPE_ALPHA_HORIZONTAL	= 5, /* 5 - Alpha Horizontal filter. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_transform {
    pub base: transform,
    pub regs: *const dce_transform_registers,
    pub xfm_shift: *const dce_transform_shift,
    pub xfm_mask: *const dce_transform_mask,
    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: c_int,
    pub lb_memory_size: c_int,
    pub lb_bits_per_entry: c_int,
    pub prescaler_on: bool,
}

// REGAMMA RELATED
