//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/opp.h
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
// DOC: overview
//
// The Output Plane Processor (OPP) block groups have functions that format
// pixel streams such that they are suitable for display at the display device.
// The key functions contained in the OPP are:
//
// - Adaptive Backlight Modulation (ABM)
// - Formatter (FMT) which provide pixel-by-pixel operations for format the
// incoming pixel stream.
// - Output Buffer that provide pixel replication, and overlapping.
// - Interface between MPC and OPTC.
// - Clock and reset generation.
// - CRC generation.
//

// TODO: Need cleanup
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clamping_range {
    CLAMPING_FULL_RANGE = 0,	   /* No Clamping */
    CLAMPING_LIMITED_RANGE_8BPC,   /* 8  bpc: Clamping 1  to FE */
    CLAMPING_LIMITED_RANGE_10BPC, /* 10 bpc: Clamping 4  to 3FB */
    CLAMPING_LIMITED_RANGE_12BPC, /* 12 bpc: Clamping 10 to FEF */
// Use programmable clampping value on FMT_CLAMP_COMPONENT_R/G/B.
    CLAMPING_LIMITED_RANGE_PROGRAMMABLE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clamping_and_pixel_encoding_params {
    pub /: *mut *mut dc_pixel_encoding pixel_encoding; / Pixel Encoding,
    pub /: *mut *mut clamping_range clamping_level; / Clamping identifier,
    pub /: *mut *mut dc_color_depth c_depth; / Deep color use.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_depth_reduction_params {
// truncate/round
// trunc/round enabled
    pub TRUNCATE_ENABLED:1: u32,
// 2 bits: 0=6 bpc, 1=8 bpc, 2 = 10bpc
    pub TRUNCATE_DEPTH:2: u32,
// truncate or round
    pub TRUNCATE_MODE:1: u32,
// spatial dither
// Spatial Bit Depth Reduction enabled
    pub SPATIAL_DITHER_ENABLED:1: u32,
// 2 bits: 0=6 bpc, 1 = 8 bpc, 2 = 10bpc
    pub SPATIAL_DITHER_DEPTH:2: u32,
// 0-3 to select patterns
    pub SPATIAL_DITHER_MODE:2: u32,
// Enable RGB random dithering
    pub RGB_RANDOM:1: u32,
// Enable Frame random dithering
    pub FRAME_RANDOM:1: u32,
// Enable HighPass random dithering
    pub HIGHPASS_RANDOM:1: u32,
// temporal dither
// frame modulation enabled
    pub FRAME_MODULATION_ENABLED:1: u32,
// same as for trunc/spatial
    pub FRAME_MODULATION_DEPTH:2: u32,
// 2/4 gray levels
    pub TEMPORAL_LEVEL:1: u32,
    pub FRC25:2: u32,
    pub FRC50:2: u32,
    pub FRC75:2: u32,
    pub flags: },
    pub r_seed_value: u32,
    pub b_seed_value: u32,
    pub g_seed_value: u32,
    pub pixel_encoding: dc_pixel_encoding,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wide_gamut_regamma_mode {
// 0x0  - BITS2:0 Bypass
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_BYPASS,
// 0x1  - Fixed curve sRGB 2.4
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_SRGB24,
// 0x2  - Fixed curve xvYCC 2.22
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_XYYCC22,
// 0x3  - Programmable control A
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_MATRIX_A,
// 0x4  - Programmable control B
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_MATRIX_B,
// 0x0  - BITS6:4 Bypass
    WIDE_GAMUT_REGAMMA_MODE_OVL_BYPASS,
// 0x1  - Fixed curve sRGB 2.4
    WIDE_GAMUT_REGAMMA_MODE_OVL_SRGB24,
// 0x2  - Fixed curve xvYCC 2.22
    WIDE_GAMUT_REGAMMA_MODE_OVL_XYYCC22,
// 0x3  - Programmable control A
    WIDE_GAMUT_REGAMMA_MODE_OVL_MATRIX_A,
// 0x4  - Programmable control B
    WIDE_GAMUT_REGAMMA_MODE_OVL_MATRIX_B
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gamma_pixel {
    pub r: fixed31_32,
    pub g: fixed31_32,
    pub b: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum channel_name {
    CHANNEL_NAME_RED,
    CHANNEL_NAME_GREEN,
    CHANNEL_NAME_BLUE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct custom_float_format {
    pub mantissa_bits: u32,
    pub exponenta_bits: u32,
    pub sign: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct custom_float_value {
    pub mantissa: u32,
    pub exponenta: u32,
    pub value: u32,
    pub negative: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_x_point {
    pub custom_float_x: u32,
    pub x: fixed31_32,
    pub regamma_y_red: fixed31_32,
    pub regamma_y_green: fixed31_32,
    pub regamma_y_blue: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwl_float_data_ex {
    pub r: fixed31_32,
    pub g: fixed31_32,
    pub b: fixed31_32,
    pub delta_r: fixed31_32,
    pub delta_g: fixed31_32,
    pub delta_b: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_point_position {
// hw point sits between left and right sw points
    HW_POINT_POSITION_MIDDLE,
// hw point lays left from left (smaller) sw point
    HW_POINT_POSITION_LEFT,
// hw point lays stays from right (bigger) sw point
    HW_POINT_POSITION_RIGHT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gamma_point {
    pub left_index: i32,
    pub right_index: i32,
    pub pos: hw_point_position,
    pub coeff: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_gamma_point {
    pub r: gamma_point,
    pub g: gamma_point,
    pub b: gamma_point,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gamma_coefficients {
    pub a0: [fixed31_32; 3],
    pub a1: [fixed31_32; 3],
    pub a2: [fixed31_32; 3],
    pub a3: [fixed31_32; 3],
    pub user_gamma: [fixed31_32; 3],
    pub user_contrast: fixed31_32,
    pub user_brightness: fixed31_32,
}

//
// struct pwl_float_data - Fixed point RGB color
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwl_float_data {
//
// @r: Component Red.
//
    pub r: fixed31_32,
//
// @g: Component Green.
//
    pub g: fixed31_32,
//
// @b: Component Blue.
//
    pub b: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_tree_cfg {
    pub num_pipes: c_int,
    pub dpp: [c_int; MAX_PIPES],
    pub mpcc: [c_int; MAX_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct output_pixel_processor {
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub regamma_params: pwl_params,
    pub mpc_tree_params: mpc_tree,
    pub mpcc_disconnect_pending: [bool; MAX_PIPES],
    pub funcs: *const opp_funcs,
    pub dyn_expansion: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fmt_stereo_action {
    FMT_STEREO_ACTION_ENABLE = 0,
    FMT_STEREO_ACTION_DISABLE,
    FMT_STEREO_ACTION_UPDATE_POLARITY
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_grph_csc_adjustment {
// enum grph_color_adjust_option color_adjust_option;
    pub c_space: dc_color_space,
    pub /: *mut *mut dc_color_depth color_depth; / clean up to uint32_t,
    pub csc_adjust_type: graphics_csc_adjust_type,
    pub adjust_divider: i32,
    pub grph_cont: i32,
    pub grph_sat: i32,
    pub grph_bright: i32,
    pub grph_hue: i32,
}

// Underlay related types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_adjustment_range {
    pub hw_default: i32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub /: *mut *mut uint32_t divider; / (actually HW range is min/divider; divider !=0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovl_csc_adjust_item {
    OVERLAY_BRIGHTNESS = 0,
    OVERLAY_GAMMA,
    OVERLAY_CONTRAST,
    OVERLAY_SATURATION,
    OVERLAY_HUE,
    OVERLAY_ALPHA,
    OVERLAY_ALPHA_PER_PIX,
    OVERLAY_COLOR_TEMPERATURE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oppbuf_display_segmentation {
    OPPBUF_DISPLAY_SEGMENTATION_1_SEGMENT = 0,
    OPPBUF_DISPLAY_SEGMENTATION_2_SEGMENT = 1,
    OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT = 2,
    OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_LEFT = 3,
    OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_RIGHT = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oppbuf_params {
    pub active_width: u32,
    pub mso_segmentation: oppbuf_display_segmentation,
    pub mso_overlap_pixel_num: u32,
    pub pixel_repetition: u32,
    pub num_segment_padded_pixels: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_opp_reg_state {
    pub dpg_control: u32,
    pub fmt_control: u32,
    pub oppbuf_control: u32,
    pub opp_pipe_control: u32,
    pub opp_pipe_crc_control: u32,
    pub opp_abm_control: u32,
    pub dscrm_dsc_forward_config: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_funcs {
// FORMATTER RELATED
    pub clamping): *mut clamping_and_pixel_encoding_params,
    pub signal): signal_type,
    pub params): *const bit_depth_reduction_params,
// underlay related
    pub range): *mut hw_adjustment_range,
    pub opp): *mut *mut void (opp_destroy)(struct output_pixel_processor,
    pub timing): *const dc_crtc_timing,
    pub enable): bool,
    pub offset): c_int,
    pub height): u32,
    pub opp): *mut output_pixel_processor,
    pub opp): *mut *mut bool (dpg_is_pending)(struct output_pixel_processor,
    pub color): *const tg_color,
    pub is_primary): bool,
    pub is_primary): bool,
    pub opp_reg_state): *mut *mut output_pixel_processor opp, dcn_opp_reg_state,
}
