//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/hw_shared.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

//
// Data types shared between different Virtual HW blocks
//
pub const MAX_AUDIOS: c_int = 7;
//
// @MAX_PIPES:
//
// Every ASIC support a fixed number of pipes; MAX_PIPES defines a large number
// to be used inside loops and for determining array sizes.
//
pub const MAX_PIPES: c_int = 6;

pub const MAX_DPIA: c_int = 6;
pub const MAX_CONNECTOR: c_int = 6;
pub const MAX_VIRTUAL_LINKS: c_int = 4;

//
// define MAX_DIG_LINK_ENCODERS - maximum number of digital encoders
//
// Digital encoders are ENGINE_ID_DIGA...G, there are at most 7,
// although not every GPU may have that many.
//
pub const MAX_DIG_LINK_ENCODERS: c_int = 7;
//
// define MAX_DAC_LINK_ENCODERS - maximum number of analog link encoders
//
// Analog encoders are ENGINE_ID_DACA/B, there are at most 2,
// although not every GPU may have that many. Modern GPUs typically
// don't have analog encoders.
//
pub const MAX_DAC_LINK_ENCODERS: c_int = 2;
//
// define MAX_LINK_ENCODERS - maximum number link encoders in total
//
// This includes both analog and digital encoders.
//

pub const MAX_DWB_PIPES: c_int = 1;
pub const MAX_HDMI_FRL_ENCODERS: c_int = 2;
pub const MAX_HPO_DP2_ENCODERS: c_int = 4;
pub const MAX_HPO_DP2_LINK_ENCODERS: c_int = 4;
// Pipe topology snapshot structures
pub const MAX_TOPOLOGY_SNAPSHOTS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_topology_line {
    pub is_phantom_pipe: bool,
    pub plane_idx: c_int,
    pub slice_idx: c_int,
    pub stream_idx: c_int,
    pub dpp_inst: c_int,
    pub opp_inst: c_int,
    pub tg_inst: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_topology_snapshot {
    pub pipe_log_lines: [pipe_topology_line; MAX_PIPES],
    pub line_count: c_int,
    pub timestamp_us: u64,
    pub stream_count: c_int,
    pub phantom_stream_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_topology_history {
    pub snapshots: [pipe_topology_snapshot; MAX_TOPOLOGY_SNAPSHOTS],
    pub current_snapshot_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gamma_curve {
    pub offset: u32,
    pub segments_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct curve_points {
    pub x: fixed31_32,
    pub y: fixed31_32,
    pub offset: fixed31_32,
    pub slope: fixed31_32,
    pub custom_float_x: u32,
    pub custom_float_y: u32,
    pub custom_float_offset: u32,
    pub custom_float_slope: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct curve_points3 {
    pub red: curve_points,
    pub green: curve_points,
    pub blue: curve_points,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwl_result_data {
    pub red: fixed31_32,
    pub green: fixed31_32,
    pub blue: fixed31_32,
    pub delta_red: fixed31_32,
    pub delta_green: fixed31_32,
    pub delta_blue: fixed31_32,
    pub red_reg: u32,
    pub green_reg: u32,
    pub blue_reg: u32,
    pub delta_red_reg: u32,
    pub delta_green_reg: u32,
    pub delta_blue_reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_rgb {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tetrahedral_33x33x33 {
    pub lut0: [dc_rgb; 8985],
    pub lut1: [dc_rgb; 8984],
    pub lut2: [dc_rgb; 8984],
    pub lut3: [dc_rgb; 8984],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tetrahedral_17x17x17 {
    pub lut0: [dc_rgb; 1229],
    pub lut1: [dc_rgb; 1228],
    pub lut2: [dc_rgb; 1228],
    pub lut3: [dc_rgb; 1228],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tetrahedral_9x9x9 {
    pub lut0: [dc_rgb; 183],
    pub lut1: [dc_rgb; 182],
    pub lut2: [dc_rgb; 182],
    pub lut3: [dc_rgb; 182],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lut_dimension {
    LUT_DIM_INVALID = 0,
    LUT_DIM_9 = 9,
    LUT_DIM_17 = 17,
    LUT_DIM_33 = 33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tetrahedral_params {
// TODO: Uncomment when in use.
// struct tetrahedral_33x33x33 tetrahedral_33;
    pub tetrahedral_17: tetrahedral_17x17x17,
    pub tetrahedral_9: tetrahedral_9x9x9,
}

// arr_curve_points - regamma regions/segments specification
// arr_points - beginning and end point specified separately (only one on DCE)
// corner_points - beginning and end point for all 3 colors (DCN)
// rgb_resulted - final curve
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwl_params {
    pub arr_curve_points: [gamma_curve; 34],
    pub arr_points: [curve_points; 2],
    pub corner_points: [curve_points3; 2],
}

// move to dpp
// while we are moving functionality out of opp to dpp to align
// HW programming to HW IP, we define these struct in hw_shared
// so we can still compile while refactoring
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lb_pixel_depth {
// do not change the values because it is used as bit vector
    LB_PIXEL_DEPTH_18BPP = 1,
    LB_PIXEL_DEPTH_24BPP = 2,
    LB_PIXEL_DEPTH_30BPP = 4,
    LB_PIXEL_DEPTH_36BPP = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum graphics_csc_adjust_type {
    GRAPHICS_CSC_ADJUST_TYPE_BYPASS = 0,
    GRAPHICS_CSC_ADJUST_TYPE_HW, /* without adjustments */
    GRAPHICS_CSC_ADJUST_TYPE_SW  /*use adjustments */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipp_degamma_mode {
    IPP_DEGAMMA_MODE_BYPASS,
    IPP_DEGAMMA_MODE_HW_sRGB,
    IPP_DEGAMMA_MODE_HW_xvYCC,
    IPP_DEGAMMA_MODE_USER_PWL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gamcor_mode {
    GAMCOR_MODE_BYPASS,
    GAMCOR_MODE_RESERVED_1,
    GAMCOR_MODE_USER_PWL,
    GAMCOR_MODE_RESERVED_3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipp_output_format {
    IPP_OUTPUT_FORMAT_12_BIT_FIX,
    IPP_OUTPUT_FORMAT_16_BIT_BYPASS,
    IPP_OUTPUT_FORMAT_FLOAT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum expansion_mode {
    EXPANSION_MODE_DYNAMIC,
    EXPANSION_MODE_ZERO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct default_adjustment {
    pub lb_color_depth: lb_pixel_depth,
    pub out_color_space: dc_color_space,
    pub in_color_space: dc_color_space,
    pub color_depth: dc_color_depth,
    pub surface_pixel_format: dc_pixel_format,
    pub csc_adjust_type: graphics_csc_adjust_type,
    pub force_hw_default: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct out_csc_color_matrix {
    pub color_space: dc_color_space,
    pub regval: [u16; 12],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gamut_remap_select {
    GAMUT_REMAP_BYPASS = 0,
    GAMUT_REMAP_COEFF,
    GAMUT_REMAP_COMA_COEFF,
    GAMUT_REMAP_COMB_COEFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opp_regamma {
    OPP_REGAMMA_BYPASS = 0,
    OPP_REGAMMA_SRGB,
    OPP_REGAMMA_XVYCC,
    OPP_REGAMMA_USER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum optc_dsc_mode {
    OPTC_DSC_DISABLED = 0,
    OPTC_DSC_ENABLED_444 = 1, /* 'RGB 444' or 'Simple YCbCr 4:2:2' (4:2:2 upsampled to 4:4:4) */
    OPTC_DSC_ENABLED_NATIVE_SUBSAMPLED = 2 /* Native 4:2:2 or 4:2:0 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_bias_and_scale {
    pub scale_red: u32,
    pub bias_red: u32,
    pub scale_green: u32,
    pub bias_green: u32,
    pub scale_blue: u32,
    pub bias_blue: u32,
    pub bias_and_scale_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_pattern_dyn_range {
    TEST_PATTERN_DYN_RANGE_VESA = 0,
    TEST_PATTERN_DYN_RANGE_CEA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_pattern_mode {
    TEST_PATTERN_MODE_COLORSQUARES_RGB = 0,
    TEST_PATTERN_MODE_COLORSQUARES_YCBCR601,
    TEST_PATTERN_MODE_COLORSQUARES_YCBCR709,
    TEST_PATTERN_MODE_VERTICALBARS,
    TEST_PATTERN_MODE_HORIZONTALBARS,
    TEST_PATTERN_MODE_SINGLERAMP_RGB,
    TEST_PATTERN_MODE_DUALRAMP_RGB,
    TEST_PATTERN_MODE_XR_BIAS_RGB
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_pattern_color_format {
    TEST_PATTERN_COLOR_FORMAT_BPC_6 = 0,
    TEST_PATTERN_COLOR_FORMAT_BPC_8,
    TEST_PATTERN_COLOR_FORMAT_BPC_10,
    TEST_PATTERN_COLOR_FORMAT_BPC_12
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum controller_dp_test_pattern {
    CONTROLLER_DP_TEST_PATTERN_D102 = 0,
    CONTROLLER_DP_TEST_PATTERN_SYMBOLERROR,
    CONTROLLER_DP_TEST_PATTERN_PRBS7,
    CONTROLLER_DP_TEST_PATTERN_COLORSQUARES,
    CONTROLLER_DP_TEST_PATTERN_VERTICALBARS,
    CONTROLLER_DP_TEST_PATTERN_HORIZONTALBARS,
    CONTROLLER_DP_TEST_PATTERN_COLORRAMP,
    CONTROLLER_DP_TEST_PATTERN_VIDEOMODE,
    CONTROLLER_DP_TEST_PATTERN_RESERVED_8,
    CONTROLLER_DP_TEST_PATTERN_RESERVED_9,
    CONTROLLER_DP_TEST_PATTERN_RESERVED_A,
    CONTROLLER_DP_TEST_PATTERN_COLORSQUARES_CEA,
    CONTROLLER_DP_TEST_PATTERN_SOLID_COLOR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum controller_dp_color_space {
    CONTROLLER_DP_COLOR_SPACE_RGB,
    CONTROLLER_DP_COLOR_SPACE_YCBCR601,
    CONTROLLER_DP_COLOR_SPACE_YCBCR709,
    CONTROLLER_DP_COLOR_SPACE_UDEFINED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_lut_mode {
    LUT_BYPASS,
    LUT_RAM_A,
    LUT_RAM_B
}

//
// speakersToChannels
//
// @brief
// translate speakers to channels
//
// FL  - Front Left
// FR  - Front Right
// RL  - Rear Left
// RR  - Rear Right
// RC  - Rear Center
// FC  - Front Center
// FLC - Front Left Center
// FRC - Front Right Center
// RLC - Rear Left Center
// RRC - Rear Right Center
// LFE - Low Freq Effect
//
// FC
// FLC      FRC
// FL                    FR
//
// LFE
// ()
//
// RL                    RR
// RLC      RRC
// RC
//
// ch  8   7   6   5   4   3   2   1
// 0b00000011      -   -   -   -   -   -   FR  FL
// 0b00000111      -   -   -   -   -   LFE FR  FL
// 0b00001011      -   -   -   -   FC  -   FR  FL
// 0b00001111      -   -   -   -   FC  LFE FR  FL
// 0b00010011      -   -   -   RC  -   -   FR  FL
// 0b00010111      -   -   -   RC  -   LFE FR  FL
// 0b00011011      -   -   -   RC  FC  -   FR  FL
// 0b00011111      -   -   -   RC  FC  LFE FR  FL
// 0b00110011      -   -   RR  RL  -   -   FR  FL
// 0b00110111      -   -   RR  RL  -   LFE FR  FL
// 0b00111011      -   -   RR  RL  FC  -   FR  FL
// 0b00111111      -   -   RR  RL  FC  LFE FR  FL
// 0b01110011      -   RC  RR  RL  -   -   FR  FL
// 0b01110111      -   RC  RR  RL  -   LFE FR  FL
// 0b01111011      -   RC  RR  RL  FC  -   FR  FL
// 0b01111111      -   RC  RR  RL  FC  LFE FR  FL
// 0b11110011      RRC RLC RR  RL  -   -   FR  FL
// 0b11110111      RRC RLC RR  RL  -   LFE FR  FL
// 0b11111011      RRC RLC RR  RL  FC  -   FR  FL
// 0b11111111      RRC RLC RR  RL  FC  LFE FR  FL
// 0b11000011      FRC FLC -   -   -   -   FR  FL
// 0b11000111      FRC FLC -   -   -   LFE FR  FL
// 0b11001011      FRC FLC -   -   FC  -   FR  FL
// 0b11001111      FRC FLC -   -   FC  LFE FR  FL
// 0b11010011      FRC FLC -   RC  -   -   FR  FL
// 0b11010111      FRC FLC -   RC  -   LFE FR  FL
// 0b11011011      FRC FLC -   RC  FC  -   FR  FL
// 0b11011111      FRC FLC -   RC  FC  LFE FR  FL
// 0b11110011      FRC FLC RR  RL  -   -   FR  FL
// 0b11110111      FRC FLC RR  RL  -   LFE FR  FL
// 0b11111011      FRC FLC RR  RL  FC  -   FR  FL
// 0b11111111      FRC FLC RR  RL  FC  LFE FR  FL
//
// @param
// speakers - speaker information as it comes from CEA audio block
//
// translate speakers to channels
#[repr(C)]
#[derive(Copy, Clone)]
pub union audio_cea_channels {
    pub all: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_cea_channels_bits {
    pub FL:1: u32,
    pub FR:1: u32,
    pub LFE:1: u32,
    pub FC:1: u32,
    pub RL_RC:1: u32,
    pub RR:1: u32,
    pub RC_RLC_FLC:1: u32,
    pub RRC_FRC:1: u32,
    pub channels: },
}
