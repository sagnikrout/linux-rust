//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/transform.h
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

pub const CSC_TEMPERATURE_MATRIX_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transform {
    pub funcs: *const transform_funcs,
    pub ctx: *mut dc_context,
    pub inst: c_int,
    pub caps: *mut dpp_caps,
    pub regamma_params: pwl_params,
}

// Colorimetry
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum colorimetry {
    COLORIMETRY_NO_DATA = 0,
    COLORIMETRY_ITU601 = 1,
    COLORIMETRY_ITU709 = 2,
    COLORIMETRY_EXTENDED = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum colorimetry_ext {
    COLORIMETRYEX_XVYCC601 = 0,
    COLORIMETRYEX_XVYCC709 = 1,
    COLORIMETRYEX_SYCC601 = 2,
    COLORIMETRYEX_ADOBEYCC601 = 3,
    COLORIMETRYEX_ADOBERGB = 4,
    COLORIMETRYEX_BT2020YCC = 5,
    COLORIMETRYEX_BT2020RGBYCBCR = 6,
    COLORIMETRYEX_RESERVED = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum active_format_info {
    ACTIVE_FORMAT_NO_DATA = 0,
    ACTIVE_FORMAT_VALID = 1
}

// Active format aspect ratio
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum active_format_aspect_ratio {
    ACTIVE_FORMAT_ASPECT_RATIO_SAME_AS_PICTURE = 8,
    ACTIVE_FORMAT_ASPECT_RATIO_4_3 = 9,
    ACTIVE_FORMAT_ASPECT_RATIO_16_9 = 0XA,
    ACTIVE_FORMAT_ASPECT_RATIO_14_9 = 0XB
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bar_info {
    BAR_INFO_NOT_VALID = 0,
    BAR_INFO_VERTICAL_VALID = 1,
    BAR_INFO_HORIZONTAL_VALID = 2,
    BAR_INFO_BOTH_VALID = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum picture_scaling {
    PICTURE_SCALING_UNIFORM = 0,
    PICTURE_SCALING_HORIZONTAL = 1,
    PICTURE_SCALING_VERTICAL = 2,
    PICTURE_SCALING_BOTH = 3
}

// RGB quantization range
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rgb_quantization_range {
    RGB_QUANTIZATION_DEFAULT_RANGE = 0,
    RGB_QUANTIZATION_LIMITED_RANGE = 1,
    RGB_QUANTIZATION_FULL_RANGE = 2,
    RGB_QUANTIZATION_RESERVED = 3
}

// YYC quantization range
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yyc_quantization_range {
    YYC_QUANTIZATION_LIMITED_RANGE = 0,
    YYC_QUANTIZATION_FULL_RANGE = 1,
    YYC_QUANTIZATION_RESERVED2 = 2,
    YYC_QUANTIZATION_RESERVED3 = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum graphics_gamut_adjust_type {
    GRAPHICS_GAMUT_ADJUST_TYPE_BYPASS = 0,
    GRAPHICS_GAMUT_ADJUST_TYPE_HW, /* without adjustments */
    GRAPHICS_GAMUT_ADJUST_TYPE_SW /* use adjustments */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfm_grph_csc_adjustment {
    pub temperature_matrix: [fixed31_32; CSC_TEMPERATURE_MATRIX_SIZE],
    pub gamut_adjust_type: graphics_gamut_adjust_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct overscan_info {
    pub left: c_int,
    pub right: c_int,
    pub top: c_int,
    pub bottom: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaling_ratios {
    pub horz: fixed31_32,
    pub vert: fixed31_32,
    pub horz_c: fixed31_32,
    pub vert_c: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sharpness_adj {
    pub horz: c_int,
    pub vert: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_buffer_params {
    pub alpha_en: bool,
    pub pixel_expan_mode: bool,
    pub interleave_en: bool,
    pub dynamic_pixel_depth: c_int,
    pub depth: lb_pixel_depth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scl_inits {
    pub h: fixed31_32,
    pub h_c: fixed31_32,
    pub v: fixed31_32,
    pub v_c: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaler_data {
    pub h_active: c_int,
    pub v_active: c_int,
    pub taps: scaling_taps,
    pub viewport: rect,
    pub viewport_c: rect,
    pub recout: rect,
    pub ratios: scaling_ratios,
    pub inits: scl_inits,
    pub sharpness: sharpness_adj,
    pub format: dc_pixel_format,
    pub lb_params: line_buffer_params,
// Below struct holds the scaler values to program hw registers
    pub dscl_prog_data: dscl_prog_data,
    pub upsp: upsp_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transform_funcs {
    pub xfm): *mut *mut void (transform_reset)(struct transform,
    pub scl_data): *const scaler_data,
    pub bit_depth_params): *const bit_depth_reduction_params,
    pub in_taps): *const scaling_taps,
    pub adjust): *const xfm_grph_csc_adjustment,
    pub default_adjust): *const default_adjustment,
    pub tbl_entry): *const out_csc_color_matrix,
    pub power_on): bool,
    pub num): u32,
    pub is_ram_a): bool,
    pub params): *const pwl_params,
    pub params): *const pwl_params,
    pub params): *const *const transform xfm, pwl_params,
    pub mode): opp_regamma,
    pub mode): ipp_degamma_mode,
    pub gamma): *const dc_gamma,
    pub params): *const pwl_params,
    pub input_color_space): dc_color_space,
    pub xfm_base): *mut *mut void (ipp_full_bypass)(struct transform,
    pub attr): *const dc_cursor_attributes,
}

// Defines the pixel processing capability of the DSCL
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dscl_data_processing_format {
    DSCL_DATA_PRCESSING_FIXED_FORMAT,	/* The DSCL processes pixel data in fixed format */
    DSCL_DATA_PRCESSING_FLOAT_FORMAT,	/* The DSCL processes pixel data in float format */
}

//
// The DPP capabilities structure contains enumerations to specify the
// HW processing features and an associated function pointers to
// provide the function interface that can be overloaded for implementations
// based on different capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_caps {
// DSCL processing pixel data in fixed or float format
    pub dscl_data_proc_format: dscl_data_processing_format,
// max LB partitions
    pub max_lb_partitions: c_uint,
// Calculates the number of partitions in the line buffer.
// The implementation of this function is overloaded for
// different versions of DSCL LB.
//
    pub num_part_c): *mut c_int,
}
