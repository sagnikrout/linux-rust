//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/sspl/dc_spl_types.h
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
//
// Copyright 2024 Advanced Micro Devices, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_size {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_rect {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_ratios {
    pub horz: spl_fixed31_32,
    pub vert: spl_fixed31_32,
    pub horz_c: spl_fixed31_32,
    pub vert_c: spl_fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_inits {
    pub h: spl_fixed31_32,
    pub h_c: spl_fixed31_32,
    pub v: spl_fixed31_32,
    pub v_c: spl_fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_taps {
    pub v_taps: u32,
    pub h_taps: u32,
    pub v_taps_c: u32,
    pub h_taps_c: u32,
    pub integer_scaling: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spl_view_3d {
    SPL_VIEW_3D_NONE = 0,
    SPL_VIEW_3D_FRAME_SEQUENTIAL,
    SPL_VIEW_3D_SIDE_BY_SIDE,
    SPL_VIEW_3D_TOP_AND_BOTTOM,
    SPL_VIEW_3D_COUNT,
    SPL_VIEW_3D_FIRST = SPL_VIEW_3D_FRAME_SEQUENTIAL
}

// Pixel format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spl_pixel_format {
// graph
    SPL_PIXEL_FORMAT_UNINITIALIZED,
    SPL_PIXEL_FORMAT_INDEX8,
    SPL_PIXEL_FORMAT_RGB565,
    SPL_PIXEL_FORMAT_ARGB8888,
    SPL_PIXEL_FORMAT_ARGB2101010,
    SPL_PIXEL_FORMAT_ARGB2101010_XRBIAS,
    SPL_PIXEL_FORMAT_FP16,
// video
    SPL_PIXEL_FORMAT_420BPP8,
    SPL_PIXEL_FORMAT_420BPP10,
    SPL_PIXEL_FORMAT_422BPP8,
    SPL_PIXEL_FORMAT_422BPP10,
    SPL_PIXEL_FORMAT_422BPP12,
    SPL_PIXEL_FORMAT_444BPP8,
    SPL_PIXEL_FORMAT_444BPP10,
// end of pixel format definition
    SPL_PIXEL_FORMAT_GRPH_BEGIN = SPL_PIXEL_FORMAT_INDEX8,
    SPL_PIXEL_FORMAT_GRPH_END = SPL_PIXEL_FORMAT_FP16,
    SPL_PIXEL_FORMAT_SUBSAMPLED_BEGIN = SPL_PIXEL_FORMAT_420BPP8,
    SPL_PIXEL_FORMAT_SUBSAMPLED_END = SPL_PIXEL_FORMAT_422BPP12,
    SPL_PIXEL_FORMAT_VIDEO_BEGIN = SPL_PIXEL_FORMAT_420BPP8,
    SPL_PIXEL_FORMAT_VIDEO_END = SPL_PIXEL_FORMAT_444BPP10,
    SPL_PIXEL_FORMAT_INVALID,
    SPL_PIXEL_FORMAT_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lb_memory_config {
// Enable all 3 pieces of memory
    LB_MEMORY_CONFIG_0 = 0,

// Enable only the first piece of memory
    LB_MEMORY_CONFIG_1 = 1,

// Enable only the second piece of memory
    LB_MEMORY_CONFIG_2 = 2,

// Only applicable in 4:2:0 mode, enable all 3 pieces of memory and the
// last piece of chroma memory used for the luma storage
//
    LB_MEMORY_CONFIG_3 = 3
}

// Rotation angle
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spl_rotation_angle {
    SPL_ROTATION_ANGLE_0 = 0,
    SPL_ROTATION_ANGLE_90,
    SPL_ROTATION_ANGLE_180,
    SPL_ROTATION_ANGLE_270,
    SPL_ROTATION_ANGLE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spl_color_space {
    SPL_COLOR_SPACE_UNKNOWN,
    SPL_COLOR_SPACE_SRGB,
    SPL_COLOR_SPACE_XR_RGB,
    SPL_COLOR_SPACE_SRGB_LIMITED,
    SPL_COLOR_SPACE_MSREF_SCRGB,
    SPL_COLOR_SPACE_YCBCR601,
    SPL_COLOR_SPACE_YCBCR709,
    SPL_COLOR_SPACE_XV_YCC_709,
    SPL_COLOR_SPACE_XV_YCC_601,
    SPL_COLOR_SPACE_YCBCR601_LIMITED,
    SPL_COLOR_SPACE_YCBCR709_LIMITED,
    SPL_COLOR_SPACE_2020_RGB_FULLRANGE,
    SPL_COLOR_SPACE_2020_RGB_LIMITEDRANGE,
    SPL_COLOR_SPACE_2020_YCBCR,
    SPL_COLOR_SPACE_ADOBERGB,
    SPL_COLOR_SPACE_DCIP3,
    SPL_COLOR_SPACE_DISPLAYNATIVE,
    SPL_COLOR_SPACE_DOLBYVISION,
    SPL_COLOR_SPACE_APPCTRL,
    SPL_COLOR_SPACE_CUSTOMPOINTS,
    SPL_COLOR_SPACE_YCBCR709_BLACK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chroma_cositing {
    CHROMA_COSITING_NONE,
    CHROMA_COSITING_LEFT,
    CHROMA_COSITING_TOPLEFT,
    CHROMA_COSITING_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum upsp_mode {
    UPSP_BYPASS = 0,
    UPSP_HORIZONTAL_UPSAMPLING_ONLY,
    UPSP_VERTICAL_UPSAMPLING_ONLY,
    UPSP_HORIZONTAL_VERTICAL_UPSAMPLING
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum upsp_num_taps {
    UPSP_2_TAPS,
    UPSP_4_TAPS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum upsp_boundary_mode {
    UPSP_BOUNDARY_EDGE, //Replace out of bound samples with the edge samples
    UPSP_BOUNDARY_BLACK //Replace out of bound samples with black as 12bpc(0x800)
}

// Scratch space for calculating scaler params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_scaler_data {
    pub h_active: c_int,
    pub v_active: c_int,
    pub taps: spl_taps,
    pub viewport: spl_rect,
    pub viewport_c: spl_rect,
    pub recout: spl_rect,
    pub ratios: spl_ratios,
    pub recip_ratios: spl_ratios,
    pub inits: spl_inits,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spl_transfer_func_type {
    SPL_TF_TYPE_PREDEFINED,
    SPL_TF_TYPE_DISTRIBUTED_POINTS,
    SPL_TF_TYPE_BYPASS,
    SPL_TF_TYPE_HWPWL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spl_transfer_func_predefined {
    SPL_TRANSFER_FUNCTION_SRGB,
    SPL_TRANSFER_FUNCTION_BT709,
    SPL_TRANSFER_FUNCTION_PQ,
    SPL_TRANSFER_FUNCTION_LINEAR,
    SPL_TRANSFER_FUNCTION_UNITY,
    SPL_TRANSFER_FUNCTION_HLG,
    SPL_TRANSFER_FUNCTION_HLG12,
    SPL_TRANSFER_FUNCTION_GAMMA22,
    SPL_TRANSFER_FUNCTION_GAMMA24,
    SPL_TRANSFER_FUNCTION_GAMMA26
}

// ==============================================================
// Below structs are defined to hold hw register data
// SPL output is used to set below registers
// MPC_SIZE - set based on scl_data h_active and v_active
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_size {
    pub width: u32,
    pub height: u32,
}

// SCL_MODE - set based on scl_data.ratios and always_scale
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scl_mode {
    SCL_MODE_SCALING_444_BYPASS = 0,
    SCL_MODE_SCALING_444_RGB_ENABLE = 1,
    SCL_MODE_SCALING_444_YCBCR_ENABLE = 2,
    SCL_MODE_SCALING_420_YCBCR_ENABLE = 3,
    SCL_MODE_SCALING_420_LUMA_BYPASS = 4,
    SCL_MODE_SCALING_420_CHROMA_BYPASS = 5,
    SCL_MODE_DSCL_BYPASS = 6
}

// SCL_BLACK_COLOR - set based on scl_data.format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scl_black_color {
    pub offset_rgb_y: u32,
    pub offset_rgb_cbcr: u32,
}

// RATIO - set based on scl_data.ratios
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ratio {
    pub h_scale_ratio: u32,
    pub v_scale_ratio: u32,
    pub h_scale_ratio_c: u32,
    pub v_scale_ratio_c: u32,
}

// INIT - set based on scl_data.init
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init {
// SCL_HORZ_FILTER_INIT
    pub SCL_H_INIT_FRAC: uint32_t h_filter_init_frac; //,
    pub SCL_H_INIT_INT: uint32_t h_filter_init_int; //,
// SCL_HORZ_FILTER_INIT_C
    pub SCL_H_INIT_FRAC_C: uint32_t h_filter_init_frac_c; //,
    pub SCL_H_INIT_INT_C: uint32_t h_filter_init_int_c; //,
// SCL_VERT_FILTER_INIT
    pub SCL_V_INIT_FRAC: uint32_t v_filter_init_frac; //,
    pub SCL_V_INIT_INT: uint32_t v_filter_init_int; //,
// SCL_VERT_FILTER_INIT_C
    pub SCL_V_INIT_FRAC_C: uint32_t v_filter_init_frac_c; //,
    pub SCL_V_INIT_INT_C: uint32_t v_filter_init_int_c; //,
// SCL_VERT_FILTER_INIT_BOT
    pub SCL_V_INIT_FRAC_BOT: uint32_t v_filter_init_bot_frac; //,
    pub SCL_V_INIT_INT_BOT: uint32_t v_filter_init_bot_int; //,
// SCL_VERT_FILTER_INIT_BOT_C
    pub SCL_V_INIT_FRAC_BOT_C: uint32_t v_filter_init_bot_frac_c; //,
    pub SCL_V_INIT_INT_BOT_C: uint32_t v_filter_init_bot_int_c; //,
}

// FILTER - calculated based on scl_data ratios and taps
// iSHARP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isharp_noise_det {
    pub ISHARP_NOISEDET_EN: uint32_t enable; //,
    pub ISHARP_NOISEDET_MODE: uint32_t mode; //,
    pub ISHARP_NOISEDET_UTHRE: uint32_t uthreshold; //,
    pub ISHARP_NOISEDET_DTHRE: uint32_t dthreshold; //,
    pub ISHARP_NOISEDET_PWL_START_IN: uint32_t pwl_start_in; //,
    pub ISHARP_NOISEDET_PWL_END_IN: uint32_t pwl_end_in; //,
    pub ISHARP_NOISEDET_PWL_SLOPE: uint32_t pwl_slope; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isharp_lba {
    pub ISHARP_LBA_MODE: uint32_t mode; //,
    pub in_seg: [u32; 6],
    pub base_seg: [u32; 6],
    pub slope_seg: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isharp_fmt {
    pub ISHARP_FMT_MODE: uint32_t mode; //,
    pub ISHARP_FMT_NORM: uint32_t norm; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isharp_nldelta_sclip {
    pub ISHARP_NLDELTA_SCLIP_EN_P: uint32_t enable_p; //,
    pub ISHARP_NLDELTA_SCLIP_PIVOT_P: uint32_t pivot_p; //,
    pub ISHARP_NLDELTA_SCLIP_SLOPE_P: uint32_t slope_p; //,
    pub ISHARP_NLDELTA_SCLIP_EN_N: uint32_t enable_n; //,
    pub ISHARP_NLDELTA_SCLIP_PIVOT_N: uint32_t pivot_n; //,
    pub ISHARP_NLDELTA_SCLIP_SLOPE_N: uint32_t slope_n; //,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isharp_en {
    ISHARP_DISABLE,
    ISHARP_ENABLE
}

pub const ISHARP_LUT_TABLE_SIZE: c_int = 32;
// Below struct holds values that can be directly used to program
// hardware registers. No conversion/clamping is required
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dscl_prog_data {
    pub scl_data.recout: spl_rect recout; // RECOUT - set based on,
    pub mpc_size: mpc_size,
    pub dscl_mode: u32,
    pub scl_black_color: scl_black_color,
    pub ratios: ratio,
    pub init: init,
    pub scl_data.taps: spl_taps taps; // TAPS - set based on,
    pub viewport: spl_rect,
    pub viewport_c: spl_rect,
// raw filter
    pub filter_h: *const u16,
    pub filter_v: *const u16,
    pub filter_h_c: *const u16,
    pub filter_v_c: *const u16,
// EASF registers
    pub easf_matrix_mode: u32,
    pub easf_ltonl_en: u32,
    pub easf_v_en: u32,
    pub easf_v_sharp_factor: u32,
    pub easf_v_ring: u32,
    pub easf_v_bf1_en: u32,
    pub easf_v_bf2_mode: u32,
    pub easf_v_bf3_mode: u32,
    pub easf_v_bf2_flat1_gain: u32,
    pub easf_v_bf2_flat2_gain: u32,
    pub easf_v_bf2_roc_gain: u32,
    pub easf_v_ringest_3tap_dntilt_uptilt: u32,
    pub easf_v_ringest_3tap_uptilt_max: u32,
    pub easf_v_ringest_3tap_dntilt_slope: u32,
    pub easf_v_ringest_3tap_uptilt1_slope: u32,
    pub easf_v_ringest_3tap_uptilt2_slope: u32,
    pub easf_v_ringest_3tap_uptilt2_offset: u32,
    pub easf_v_ringest_eventap_reduceg1: u32,
    pub easf_v_ringest_eventap_reduceg2: u32,
    pub easf_v_ringest_eventap_gain1: u32,
    pub easf_v_ringest_eventap_gain2: u32,
    pub easf_v_bf_maxa: u32,
    pub easf_v_bf_maxb: u32,
    pub easf_v_bf_mina: u32,
    pub easf_v_bf_minb: u32,
    pub easf_v_bf1_pwl_in_seg0: u32,
    pub easf_v_bf1_pwl_base_seg0: u32,
    pub easf_v_bf1_pwl_slope_seg0: u32,
    pub easf_v_bf1_pwl_in_seg1: u32,
    pub easf_v_bf1_pwl_base_seg1: u32,
    pub easf_v_bf1_pwl_slope_seg1: u32,
    pub easf_v_bf1_pwl_in_seg2: u32,
    pub easf_v_bf1_pwl_base_seg2: u32,
    pub easf_v_bf1_pwl_slope_seg2: u32,
    pub easf_v_bf1_pwl_in_seg3: u32,
    pub easf_v_bf1_pwl_base_seg3: u32,
    pub easf_v_bf1_pwl_slope_seg3: u32,
    pub easf_v_bf1_pwl_in_seg4: u32,
    pub easf_v_bf1_pwl_base_seg4: u32,
    pub easf_v_bf1_pwl_slope_seg4: u32,
    pub easf_v_bf1_pwl_in_seg5: u32,
    pub easf_v_bf1_pwl_base_seg5: u32,
    pub easf_v_bf1_pwl_slope_seg5: u32,
    pub easf_v_bf1_pwl_in_seg6: u32,
    pub easf_v_bf1_pwl_base_seg6: u32,
    pub easf_v_bf1_pwl_slope_seg6: u32,
    pub easf_v_bf1_pwl_in_seg7: u32,
    pub easf_v_bf1_pwl_base_seg7: u32,
    pub easf_v_bf3_pwl_in_set0: u32,
    pub easf_v_bf3_pwl_base_set0: u32,
    pub easf_v_bf3_pwl_slope_set0: u32,
    pub easf_v_bf3_pwl_in_set1: u32,
    pub easf_v_bf3_pwl_base_set1: u32,
    pub easf_v_bf3_pwl_slope_set1: u32,
    pub easf_v_bf3_pwl_in_set2: u32,
    pub easf_v_bf3_pwl_base_set2: u32,
    pub easf_v_bf3_pwl_slope_set2: u32,
    pub easf_v_bf3_pwl_in_set3: u32,
    pub easf_v_bf3_pwl_base_set3: u32,
    pub easf_v_bf3_pwl_slope_set3: u32,
    pub easf_v_bf3_pwl_in_set4: u32,
    pub easf_v_bf3_pwl_base_set4: u32,
    pub easf_v_bf3_pwl_slope_set4: u32,
    pub easf_v_bf3_pwl_in_set5: u32,
    pub easf_v_bf3_pwl_base_set5: u32,
    pub easf_h_en: u32,
    pub easf_h_sharp_factor: u32,
    pub easf_h_ring: u32,
    pub easf_h_bf1_en: u32,
    pub easf_h_bf2_mode: u32,
    pub easf_h_bf3_mode: u32,
    pub easf_h_bf2_flat1_gain: u32,
    pub easf_h_bf2_flat2_gain: u32,
    pub easf_h_bf2_roc_gain: u32,
    pub easf_h_ringest_eventap_reduceg1: u32,
    pub easf_h_ringest_eventap_reduceg2: u32,
    pub easf_h_ringest_eventap_gain1: u32,
    pub easf_h_ringest_eventap_gain2: u32,
    pub easf_h_bf_maxa: u32,
    pub easf_h_bf_maxb: u32,
    pub easf_h_bf_mina: u32,
    pub easf_h_bf_minb: u32,
    pub easf_h_bf1_pwl_in_seg0: u32,
    pub easf_h_bf1_pwl_base_seg0: u32,
    pub easf_h_bf1_pwl_slope_seg0: u32,
    pub easf_h_bf1_pwl_in_seg1: u32,
    pub easf_h_bf1_pwl_base_seg1: u32,
    pub easf_h_bf1_pwl_slope_seg1: u32,
    pub easf_h_bf1_pwl_in_seg2: u32,
    pub easf_h_bf1_pwl_base_seg2: u32,
    pub easf_h_bf1_pwl_slope_seg2: u32,
    pub easf_h_bf1_pwl_in_seg3: u32,
    pub easf_h_bf1_pwl_base_seg3: u32,
    pub easf_h_bf1_pwl_slope_seg3: u32,
    pub easf_h_bf1_pwl_in_seg4: u32,
    pub easf_h_bf1_pwl_base_seg4: u32,
    pub easf_h_bf1_pwl_slope_seg4: u32,
    pub easf_h_bf1_pwl_in_seg5: u32,
    pub easf_h_bf1_pwl_base_seg5: u32,
    pub easf_h_bf1_pwl_slope_seg5: u32,
    pub easf_h_bf1_pwl_in_seg6: u32,
    pub easf_h_bf1_pwl_base_seg6: u32,
    pub easf_h_bf1_pwl_slope_seg6: u32,
    pub easf_h_bf1_pwl_in_seg7: u32,
    pub easf_h_bf1_pwl_base_seg7: u32,
    pub easf_h_bf3_pwl_in_set0: u32,
    pub easf_h_bf3_pwl_base_set0: u32,
    pub easf_h_bf3_pwl_slope_set0: u32,
    pub easf_h_bf3_pwl_in_set1: u32,
    pub easf_h_bf3_pwl_base_set1: u32,
    pub easf_h_bf3_pwl_slope_set1: u32,
    pub easf_h_bf3_pwl_in_set2: u32,
    pub easf_h_bf3_pwl_base_set2: u32,
    pub easf_h_bf3_pwl_slope_set2: u32,
    pub easf_h_bf3_pwl_in_set3: u32,
    pub easf_h_bf3_pwl_base_set3: u32,
    pub easf_h_bf3_pwl_slope_set3: u32,
    pub easf_h_bf3_pwl_in_set4: u32,
    pub easf_h_bf3_pwl_base_set4: u32,
    pub easf_h_bf3_pwl_slope_set4: u32,
    pub easf_h_bf3_pwl_in_set5: u32,
    pub easf_h_bf3_pwl_base_set5: u32,
    pub easf_matrix_c0: u32,
    pub easf_matrix_c1: u32,
    pub easf_matrix_c2: u32,
    pub easf_matrix_c3: u32,
// UPSP registers
    pub upsp_mode;//UPSP_MODE: u32,
    pub upsp_v_num_taps: u32,
    pub upsp_v_init_int: u32,
    pub upsp_v_init_frac: u32,
    pub upsp_h_num_taps: u32,
    pub upsp_h_init_int: u32,
    pub upsp_h_init_frac: u32,
    pub upsp_boundary_mode: u32,
    pub upsp_v_coef_tap0_p0;//UPSP_V_COEF_P0: u32,
    pub upsp_v_coef_tap1_p0: u32,
    pub upsp_v_coef_tap2_p0: u32,
    pub upsp_v_coef_tap3_p0: u32,
    pub upsp_v_coef_tap0_p1;//UPSP_V_COEF_P1: u32,
    pub upsp_v_coef_tap1_p1: u32,
    pub upsp_v_coef_tap2_p1: u32,
    pub upsp_v_coef_tap3_p1: u32,
    pub upsp_h_coef_tap0_p0;//UPSP_H_COEF_P0: u32,
    pub upsp_h_coef_tap1_p0: u32,
    pub upsp_h_coef_tap2_p0: u32,
    pub upsp_h_coef_tap3_p0: u32,
    pub upsp_h_coef_tap0_p1;//UPSP_H_COEF_P1: u32,
    pub upsp_h_coef_tap1_p1: u32,
    pub upsp_h_coef_tap2_p1: u32,
    pub upsp_h_coef_tap3_p1: u32,
    pub upsp_clamp_max;//UPSP_CLAMP: u32,
    pub upsp_clamp_min: u32,
// iSharp
    pub ISHARP_EN: uint32_t isharp_en; //,
    pub ISHARP_NOISEDET: isharp_noise_det isharp_noise_det; //,
    pub this: uint32_t isharp_nl_en; // ISHARP_NL_EN ? TODO:check,
    pub ISHARP_LBA: isharp_lba isharp_lba; //,
    pub ISHARP_FMT: isharp_fmt isharp_fmt; //,
    pub isharp_delta: [u32; ISHARP_LUT_TABLE_SIZE],
    pub ISHARP_NLDELTA_SCLIP: isharp_nldelta_sclip isharp_nldelta_sclip; //,
// blur and scale filter
    pub filter_blur_scale_v: *const u16,
    pub filter_blur_scale_h: *const u16,
    pub /: *mut *mut int sharpness_level; / Track sharpness level,
}

// SPL input and output definitions
// SPL scratch struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_scratch {
// Pack all SPL outputs in scl_data
    pub scl_data: spl_scaler_data,
}

// SPL input and output definitions
// SPL outputs struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_out {
// Pack all output need to program hw registers
    pub dscl_prog_data: *mut dscl_prog_data,
}

// end of SPL outputs
// SPL inputs
// opp extra adjustment for rect
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_opp_adjust {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
}

// Basic input information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_in {
    pub Format: spl_pixel_format format; // Pixel,
    pub /: *mut *mut chroma_cositing cositing; / Chroma Subsampling Offset,
    pub rect: spl_rect src_rect; // Source,
    pub Rect: spl_rect dst_rect; // Destination,
    pub rect: spl_rect clip_rect; // Clip,
    pub Rotation: spl_rotation_angle rotation; //,
    pub mirror: bool horizontal_mirror; // Horizontal,
    pub use_recout_width_aligned: bool,
    pub mpc_num_h_slices: c_int,
    pub mpc_recout_width_align: c_int,
    pub num_slices_recout_width: },
    pub num_h_slices_recout_width_align: },
    pub split_idx: int mpc_h_slice_index; // previous mpc_combine_v -,
    pub opp_recout_adjust: spl_opp_adjust,
// Inputs for adaptive scaler - TODO
    pub /: *mut *mut spl_transfer_func_type tf_type; / Transfer function type,
    pub /: *mut *mut spl_transfer_func_predefined tf_predefined_type; / Transfer function predefined type,
// enum dc_transfer_func_predefined tf;
    pub Space: spl_color_space color_space; // Color,
    pub is_sdr: unsigned int max_luminance; // Max Luminance TODO: Is determined in dc_hw_sequencer.c,
    pub this?: bool film_grain_applied; // Film Grain Applied // TODO: To check from where to get,
    pub 0: int custom_width; // Width for non-standard segmentation - used when !=,
    pub 0: int custom_x; // Start x for non-standard segmentation - used when custom_width !=,
}

// Basic output information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_out {
    pub Size: spl_size output_size; // Output,
    pub Rect: spl_rect dst_rect; // Destination,
    pub rect: spl_rect src_rect; // Source,
    pub deprecated: int odm_combine_factor; //,
    pub active: spl_rect odm_slice_rect; // OPP input rect in timing,
    pub subsampling: spl_view_3d view_format; // TODO: View format Check if it is chroma,
    pub SCL_MODE: bool always_scale; // Is always scale enabled? Required for getting,
    pub taps: int max_downscale_src_width; // Required to get optimal no of,
    pub alpha_en: bool,
    pub use_two_pixels_per_container: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sharpness_setting {
    SHARPNESS_HW_OFF = 0,
    SHARPNESS_ZERO,
    SHARPNESS_CUSTOM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sharpness_range_source {
    SHARPNESS_RANGE_DCN = 0,
    SHARPNESS_RANGE_DCN_OVERRIDE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_sharpness_range {
    pub sdr_rgb_min: c_int,
    pub sdr_rgb_max: c_int,
    pub sdr_rgb_mid: c_int,
    pub sdr_yuv_min: c_int,
    pub sdr_yuv_max: c_int,
    pub sdr_yuv_mid: c_int,
    pub hdr_rgb_min: c_int,
    pub hdr_rgb_max: c_int,
    pub hdr_rgb_mid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adaptive_sharpness {
    pub enable: bool,
    pub sharpness_level: c_uint,
    pub sharpness_range: spl_sharpness_range,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum linear_light_scaling {
    LLS_PREF_DONT_CARE = 0,
    LLS_PREF_YES,
    LLS_PREF_NO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sharpen_policy {
    SHARPEN_ALWAYS = 0,
    SHARPEN_YUV = 1,
    SHARPEN_RGB_FULLSCREEN_YUV = 2,
    SHARPEN_FULLSCREEN_ALL = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scale_to_sharpness_policy {
    NO_SCALE_TO_SHARPNESS_ADJ = 0,
    SCALE_TO_SHARPNESS_ADJ_YUV = 1,
    SCALE_TO_SHARPNESS_ADJ_ALL = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_callbacks {
    pub num_part_c): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_debug {
    pub visual_confirm_base_offset: c_int,
    pub visual_confirm_dpp_offset: c_int,
    pub scale_to_sharpness_policy: scale_to_sharpness_policy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_in {
    pub basic_out: basic_out,
    pub basic_in: basic_in,
// Basic slice information
    pub get_odm_split_index: int odm_slice_index; // ODM Slice Index using,
    pub Quality: spl_taps scaling_quality; // Explicit Scaling,
    pub callbacks: spl_callbacks,
// Inputs for isharp and EASF
    pub Sharpness: adaptive_sharpness adaptive_sharpness; // Adaptive,
    pub Scaling: linear_light_scaling lls_pref; // Linear Light,
    pub prefer_easf: bool,
    pub disable_easf: bool,
    pub /: *mut *mut bool override_easf; / If true, keep EASF enabled but use provided in_taps,
    pub debug: spl_debug,
    pub is_fullscreen: bool,
    pub is_hdr_on: bool,
    pub h_active: c_int,
    pub v_active: c_int,
    pub min_viewport_size: c_int,
    pub sdr_white_level_nits: c_int,
    pub sharpen_policy: sharpen_policy,
    pub upsp_mode: upsp_mode,
}

// end of SPL inputs
