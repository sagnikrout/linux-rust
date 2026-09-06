//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/dpp.h
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
// Copyright 2012-2026 Advanced Micro Devices, Inc.
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
// The DPP (Display Pipe and Plane) block is the unified display data
// processing engine in DCN for processing graphic or video data on per DPP
// rectangle base. This rectangle can be a part of SLS (Single Large Surface),
// or a layer to be blended with other DPP, or a rectangle associated with a
// display tile.
//
// It provides various functions including:
// - graphic color keyer
// - graphic cursor compositing
// - graphic or video image source to destination scaling
// - image sharping
// - video format conversion from 4:2:0 or 4:2:2 to 4:4:4
// - Color Space Conversion
// - Host LUT gamma adjustment
// - Color Gamut Remap
// - brightness and contrast adjustment.
//
// DPP pipe consists of Converter and Cursor (CNVC), Scaler (DSCL), Color
// Management (CM), Output Buffer (OBUF) and Digital Bypass (DPB) module
// connected in a video/graphics pipeline.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union defer_reg_writes {
    pub disable_blnd_lut:1: bool,
    pub disable_3dlut:1: bool,
    pub disable_shaper:1: bool,
    pub disable_gamcor:1: bool,
    pub disable_dscl:1: bool,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp {
    pub funcs: *const dpp_funcs,
    pub ctx: *mut dc_context,
//
// @inst:
//
// inst stands for "instance," and it is an id number that references a
// specific DPP.
//
    pub inst: c_int,
    pub caps: *mut dpp_caps,
    pub regamma_params: pwl_params,
    pub degamma_params: pwl_params,
    pub cur_attr: dpp_cursor_attributes,
    pub deferred_reg_writes: defer_reg_writes,
    pub shaper_params: pwl_params,
    pub cm_bypass_mode: bool,
    pub cursor_offload: bool,
    pub pos: cursor_position_cache_dpp,
    pub att: cursor_attribute_cache_dpp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_input_csc_matrix {
    pub color_space: dc_color_space,
    pub regval: [u16; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_grph_csc_adjustment {
    pub temperature_matrix: [fixed31_32; CSC_TEMPERATURE_MATRIX_SIZE],
    pub gamut_adjust_type: graphics_gamut_adjust_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnv_color_keyer_params {
    pub color_keyer_en: c_int,
    pub color_keyer_mode: c_int,
    pub color_keyer_alpha_low: c_int,
    pub color_keyer_alpha_high: c_int,
    pub color_keyer_red_low: c_int,
    pub color_keyer_red_high: c_int,
    pub color_keyer_green_low: c_int,
    pub color_keyer_green_high: c_int,
    pub color_keyer_blue_low: c_int,
    pub color_keyer_blue_high: c_int,
}

//
// struct cnv_alpha_2bit_lut - Set the 8bit alpha values based on the 2 bit alpha
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnv_alpha_2bit_lut {
//
// @lut0: ALPHA_2BIT_LUT. ALPHA_2BIT_LUT0. Default: 0b00000000
//
    pub lut0: c_int,
//
// @lut1: ALPHA_2BIT_LUT. ALPHA_2BIT_LUT1. Default: 0b01010101
//
    pub lut1: c_int,
//
// @lut2: ALPHA_2BIT_LUT. ALPHA_2BIT_LUT2. Default: 0b10101010
//
    pub lut2: c_int,
//
// @lut3: ALPHA_2BIT_LUT. ALPHA_2BIT_LUT3. Default: 0b11111111
//
    pub lut3: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_dpp_state {
    pub is_enabled: u32,
    pub igam_lut_mode: u32,
    pub igam_input_format: u32,
    pub dgam_lut_mode: u32,
    pub rgam_lut_mode: u32,
// gamut_remap data for dcn10_get_cm_states()
    pub gamut_remap_mode: u32,
    pub gamut_remap_c11_c12: u32,
    pub gamut_remap_c13_c14: u32,
    pub gamut_remap_c21_c22: u32,
    pub gamut_remap_c23_c24: u32,
    pub gamut_remap_c31_c32: u32,
    pub gamut_remap_c33_c34: u32,
// gamut_remap data for dcn*_log_color_state()
    pub gamut_remap: dpp_grph_csc_adjustment,
    pub shaper_lut_mode: u32,
    pub lut3d_mode: u32,
    pub lut3d_bit_depth: u32,
    pub lut3d_size: u32,
    pub blnd_lut_mode: u32,
    pub pre_dgam_mode: u32,
    pub pre_dgam_select: u32,
    pub gamcor_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_dpp_reg_state {
    pub recout_start: u32,
    pub recout_size: u32,
    pub scl_horz_filter_scale_ratio: u32,
    pub scl_vert_filter_scale_ratio: u32,
    pub scl_mode: u32,
    pub cm_control: u32,
    pub dpp_control: u32,
    pub dscl_control: u32,
    pub obuf_control: u32,
    pub mpc_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CM_bias_params {
    pub cm_bias_cr_r: u32,
    pub cm_bias_y_g: u32,
    pub cm_bias_cb_b: u32,
    pub cm_bias_format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_funcs {
    pub params): *const *const dpp dpp_base, pwl_params,
    pub tr): dc_transfer_func_predefined,
    pub additive_blending): uint32_t enable, uint32_t,
    pub bias_params): *mut CM_bias_params,
    pub s): *mut *mut *mut void (dpp_read_state)(struct dpp dpp, struct dcn_dpp_state,
    pub dpp_reg_state): *mut *mut *mut void (dpp_read_reg_state)(struct dpp dpp, struct dcn_dpp_reg_state,
    pub dpp): *mut *mut void (dpp_reset)(struct dpp,
    pub scl_data): *const scaler_data,
    pub bit_depth_params): *const bit_depth_reduction_params,
    pub in_taps): *const scaling_taps,
    pub adjust): *const dpp_grph_csc_adjustment,
    pub colorspace): dc_color_space,
    pub regval): *const u16,
    pub power_on): bool,
    pub num): u32,
    pub is_ram_a): bool,
    pub params): *const pwl_params,
    pub params): *const pwl_params,
    pub mode): opp_regamma,
    pub params): *mut dc_bias_and_scale,
    pub mode): ipp_degamma_mode,
    pub gamma): *const dc_gamma,
    pub params): *const pwl_params,
    pub alpha_2bit_lut): *mut cnv_alpha_2bit_lut,
    pub dpp_base): *mut *mut void (dpp_full_bypass)(struct dpp,
    pub cursor_attributes): *mut dc_cursor_attributes,
    pub multiplier): u32,
    pub attr): *mut dpp_cursor_attributes,
    pub enable): bool,
    pub dpp): *mut dpp,
    pub params): *const pwl_params,
    pub params): *const pwl_params,
    pub params): *const tetrahedral_params,
    pub color_keyer): *mut cnv_color_keyer_params,
    pub adjust): *mut dpp_grph_csc_adjustment,
    pub cursor_csc_color_matrix): dc_csc_transform,
    pub dpp_base): *mut *mut void (dpp_force_disable_cursor)(struct dpp,
    pub color_space): dc_color_space,
    pub cm_hist): *mut cm_hist,
    pub scaling): dc_scaling_linearity,
    pub dscl_prog_data): *const dscl_prog_data,
}
