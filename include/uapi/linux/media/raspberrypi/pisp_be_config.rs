//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media/raspberrypi/pisp_be_config.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// PiSP Back End configuration definitions.
//
// Copyright (C) 2021 - Raspberry Pi Ltd
//

// byte alignment for inputs

// alignment for compressed inputs

// minimum required byte alignment for outputs

// preferred byte alignment for outputs

// minimum allowed tile sizes anywhere in the pipeline

pub const PISP_BACK_END_NUM_OUTPUTS: c_int = 2;
pub const PISP_BACK_END_HOG_OUTPUT: c_int = 1;
pub const PISP_BACK_END_NUM_TILES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_be_bayer_enable {
    PISP_BE_BAYER_ENABLE_INPUT = 0x000001,
    PISP_BE_BAYER_ENABLE_DECOMPRESS = 0x000002,
    PISP_BE_BAYER_ENABLE_DPC = 0x000004,
    PISP_BE_BAYER_ENABLE_GEQ = 0x000008,
    PISP_BE_BAYER_ENABLE_TDN_INPUT = 0x000010,
    PISP_BE_BAYER_ENABLE_TDN_DECOMPRESS = 0x000020,
    PISP_BE_BAYER_ENABLE_TDN = 0x000040,
    PISP_BE_BAYER_ENABLE_TDN_COMPRESS = 0x000080,
    PISP_BE_BAYER_ENABLE_TDN_OUTPUT = 0x000100,
    PISP_BE_BAYER_ENABLE_SDN = 0x000200,
    PISP_BE_BAYER_ENABLE_BLC = 0x000400,
    PISP_BE_BAYER_ENABLE_STITCH_INPUT = 0x000800,
    PISP_BE_BAYER_ENABLE_STITCH_DECOMPRESS = 0x001000,
    PISP_BE_BAYER_ENABLE_STITCH = 0x002000,
    PISP_BE_BAYER_ENABLE_STITCH_COMPRESS = 0x004000,
    PISP_BE_BAYER_ENABLE_STITCH_OUTPUT = 0x008000,
    PISP_BE_BAYER_ENABLE_WBG = 0x010000,
    PISP_BE_BAYER_ENABLE_CDN = 0x020000,
    PISP_BE_BAYER_ENABLE_LSC = 0x040000,
    PISP_BE_BAYER_ENABLE_TONEMAP = 0x080000,
    PISP_BE_BAYER_ENABLE_CAC = 0x100000,
    PISP_BE_BAYER_ENABLE_DEBIN = 0x200000,
    PISP_BE_BAYER_ENABLE_DEMOSAIC = 0x400000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_be_rgb_enable {
    PISP_BE_RGB_ENABLE_INPUT = 0x000001,
    PISP_BE_RGB_ENABLE_CCM = 0x000002,
    PISP_BE_RGB_ENABLE_SAT_CONTROL = 0x000004,
    PISP_BE_RGB_ENABLE_YCBCR = 0x000008,
    PISP_BE_RGB_ENABLE_FALSE_COLOUR = 0x000010,
    PISP_BE_RGB_ENABLE_SHARPEN = 0x000020,
// Preferred colours would occupy 0x000040
    PISP_BE_RGB_ENABLE_YCBCR_INVERSE = 0x000080,
    PISP_BE_RGB_ENABLE_GAMMA = 0x000100,
    PISP_BE_RGB_ENABLE_CSC0 = 0x000200,
    PISP_BE_RGB_ENABLE_CSC1 = 0x000400,
    PISP_BE_RGB_ENABLE_DOWNSCALE0 = 0x001000,
    PISP_BE_RGB_ENABLE_DOWNSCALE1 = 0x002000,
    PISP_BE_RGB_ENABLE_RESAMPLE0 = 0x008000,
    PISP_BE_RGB_ENABLE_RESAMPLE1 = 0x010000,
    PISP_BE_RGB_ENABLE_OUTPUT0 = 0x040000,
    PISP_BE_RGB_ENABLE_OUTPUT1 = 0x080000,
    PISP_BE_RGB_ENABLE_HOG = 0x200000
}

//
// We use the enable flags to show when blocks are "dirty", but we need some
// extra ones too.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_be_dirty {
    PISP_BE_DIRTY_GLOBAL = 0x0001,
    PISP_BE_DIRTY_SH_FC_COMBINE = 0x0002,
    PISP_BE_DIRTY_CROP = 0x0004
}

//
// struct pisp_be_global_config - PiSP global enable bitmaps
// @bayer_enables:	Bayer input enable flags
// @rgb_enables:	RGB output enable flags
// @bayer_order:	Bayer input format ordering
// @pad:		Padding bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_global_config {
    pub bayer_enables: __u32,
    pub rgb_enables: __u32,
    pub bayer_order: __u8,
    pub pad: [__u8; 3],
    pub __attribute__((packed)): },
//
// struct pisp_be_input_buffer_config - PiSP Back End input buffer
// @addr:		Input buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_input_buffer_config {
// low 32 bits followed by high 32 bits (for each of up to 3 planes)
    pub addr: [__u32; 3][2],
    pub __attribute__((packed)): },
//
// struct pisp_be_dpc_config - PiSP Back End DPC config
//
// Defective Pixel Correction configuration
//
// @coeff_level:	Coefficient for the darkest neighbouring pixel value
// @coeff_range:	Coefficient for the range of pixels for this Bayer channel
// @pad:		Padding byte
// @flags:		DPC configuration flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_dpc_config {
    pub coeff_level: __u8,
    pub coeff_range: __u8,
    pub pad: __u8,
pub const PISP_BE_DPC_FLAG_FOLDBACK: c_int = 1;
    pub flags: __u8,
    pub __attribute__((packed)): },
//
// struct pisp_be_geq_config - PiSP Back End GEQ config
//
// Green Equalisation configuration
//
// @offset:		Offset value for threshold calculation
// @slope_sharper:	Slope/Sharper configuration
// @min:		Minimum value the threshold may have
// @max:		Maximum value the threshold may have
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_geq_config {
    pub offset: __u16,

// top bit is the "sharper" flag, slope value is bottom 10 bits
    pub slope_sharper: __u16,
    pub min: __u16,
    pub max: __u16,
    pub __attribute__((packed)): },
//
// struct pisp_be_tdn_input_buffer_config - PiSP Back End TDN input buffer
// @addr:		TDN input buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_tdn_input_buffer_config {
// low 32 bits followed by high 32 bits
    pub addr: [__u32; 2],
    pub __attribute__((packed)): },
//
// struct pisp_be_tdn_config - PiSP Back End TDN config
//
// Temporal Denoise configuration
//
// @black_level:	Black level value subtracted from pixels
// @ratio:		Multiplier for the LTA input frame
// @noise_constant:	Constant offset value used in noise estimation
// @noise_slope:	Noise estimation multiplier
// @threshold:		Threshold for TDN operations
// @reset:		Disable TDN operations
// @pad:		Padding byte
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_tdn_config {
    pub black_level: __u16,
    pub ratio: __u16,
    pub noise_constant: __u16,
    pub noise_slope: __u16,
    pub threshold: __u16,
    pub reset: __u8,
    pub pad: __u8,
    pub __attribute__((packed)): },
//
// struct pisp_be_tdn_output_buffer_config - PiSP Back End TDN output buffer
// @addr:		TDN output buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_tdn_output_buffer_config {
// low 32 bits followed by high 32 bits
    pub addr: [__u32; 2],
    pub __attribute__((packed)): },
//
// struct pisp_be_sdn_config - PiSP Back End SDN config
//
// Spatial Denoise configuration
//
// @black_level:	Black level subtracted from pixel for noise estimation
// @leakage:		Proportion of the original undenoised value to mix in
// denoised output
// @pad:		Padding byte
// @noise_constant:	Noise constant used for noise estimation
// @noise_slope:	Noise slope value used for noise estimation
// @noise_constant2:	Second noise constant used for noise estimation
// @noise_slope2:	Second slope value used for noise estimation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_sdn_config {
    pub black_level: __u16,
    pub leakage: __u8,
    pub pad: __u8,
    pub noise_constant: __u16,
    pub noise_slope: __u16,
    pub noise_constant2: __u16,
    pub noise_slope2: __u16,
    pub __attribute__((packed)): },
//
// struct pisp_be_stitch_input_buffer_config - PiSP Back End Stitch input
// @addr:		Stitch input buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_stitch_input_buffer_config {
// low 32 bits followed by high 32 bits
    pub addr: [__u32; 2],
    pub __attribute__((packed)): },
pub const PISP_BE_STITCH_STREAMING_LONG: c_uint = 0x8000;
pub const PISP_BE_STITCH_EXPOSURE_RATIO_MASK: c_uint = 0x7fff;
//
// struct pisp_be_stitch_config - PiSP Back End Stitch config
//
// Stitch block configuration
//
// @threshold_lo:		Low threshold value
// @threshold_diff_power:	Low and high threshold difference
// @pad:			Padding bytes
// @exposure_ratio:		Multiplier to convert long exposure pixels into
// short exposure pixels
// @motion_threshold_256:	Motion threshold above which short exposure
// pixels are used
// @motion_threshold_recip:	Reciprocal of motion_threshold_256 value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_stitch_config {
    pub threshold_lo: __u16,
    pub threshold_diff_power: __u8,
    pub pad: __u8,
// top bit indicates whether streaming input is the long exposure
    pub exposure_ratio: __u16,
    pub motion_threshold_256: __u8,
    pub motion_threshold_recip: __u8,
    pub __attribute__((packed)): },
//
// struct pisp_be_stitch_output_buffer_config - PiSP Back End Stitch output
// @addr:		Stitch input buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_stitch_output_buffer_config {
// low 32 bits followed by high 32 bits
    pub addr: [__u32; 2],
    pub __attribute__((packed)): },
//
// struct pisp_be_cdn_config - PiSP Back End CDN config
//
// Colour Denoise configuration
//
// @thresh:		Constant for noise estimation
// @iir_strength:	Relative strength of the IIR part of the filter
// @g_adjust:		Proportion of the change assigned to the G channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_cdn_config {
    pub thresh: __u16,
    pub iir_strength: __u8,
    pub g_adjust: __u8,
    pub __attribute__((packed)): },
pub const PISP_BE_LSC_LOG_GRID_SIZE: c_int = 5;

pub const PISP_BE_LSC_STEP_PRECISION: c_int = 18;
//
// struct pisp_be_lsc_config - PiSP Back End LSC config
//
// Lens Shading Correction configuration
//
// @grid_step_x:	Reciprocal of cell size width
// @grid_step_y:	Reciprocal of cell size height
// @lut_packed:		Jointly-coded RGB gains for each LSC grid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_lsc_config {
// (1<<18) / grid_cell_width
    pub grid_step_x: __u16,
// (1<<18) / grid_cell_height
    pub grid_step_y: __u16,
// RGB gains jointly encoded in 32 bits
    pub lut_packed: [__u32; PISP_BE_LSC_LUT_SIZE][PISP_BE_LSC_LUT_SIZE],
    pub __attribute__((packed)): },
//
// struct pisp_be_lsc_extra - PiSP Back End LSC Extra config
// @offset_x:		Horizontal offset into the LSC table of this tile
// @offset_y:		Vertical offset into the LSC table of this tile
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_lsc_extra {
    pub offset_x: __u16,
    pub offset_y: __u16,
    pub __attribute__((packed)): },
pub const PISP_BE_CAC_LOG_GRID_SIZE: c_int = 3;

pub const PISP_BE_CAC_STEP_PRECISION: c_int = 20;
//
// struct pisp_be_cac_config - PiSP Back End CAC config
//
// Chromatic Aberration Correction config
//
// @grid_step_x:	Reciprocal of cell size width
// @grid_step_y:	Reciprocal of cell size height
// @lut:		Pixel shift for the CAC grid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_cac_config {
// (1<<20) / grid_cell_width
    pub grid_step_x: __u16,
// (1<<20) / grid_cell_height
    pub grid_step_y: __u16,
// [gridy][gridx][rb][xy]
    pub lut: [__s8; PISP_BE_CAC_LUT_SIZE][PISP_BE_CAC_LUT_SIZE][2][2],
    pub __attribute__((packed)): },
//
// struct pisp_be_cac_extra - PiSP Back End CAC extra config
// @offset_x:		Horizontal offset into the CAC table of this tile
// @offset_y:		Horizontal offset into the CAC table of this tile
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_cac_extra {
    pub offset_x: __u16,
    pub offset_y: __u16,
    pub __attribute__((packed)): },
pub const PISP_BE_DEBIN_NUM_COEFFS: c_int = 4;
//
// struct pisp_be_debin_config - PiSP Back End Debin config
//
// Debinning configuration
//
// @coeffs:		Filter coefficients for debinning
// @h_enable:		Horizontal debinning enable
// @v_enable:		Vertical debinning enable
// @pad:		Padding bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_debin_config {
    pub coeffs: [__s8; PISP_BE_DEBIN_NUM_COEFFS],
    pub h_enable: __s8,
    pub v_enable: __s8,
    pub pad: [__s8; 2],
    pub __attribute__((packed)): },
pub const PISP_BE_TONEMAP_LUT_SIZE: c_int = 64;
//
// struct pisp_be_tonemap_config - PiSP Back End Tonemap config
//
// Tonemapping configuration
//
// @detail_constant:	Constant value for threshold calculation
// @detail_slope:	Slope value for threshold calculation
// @iir_strength:	Relative strength of the IIR fiter
// @strength:		Strength factor
// @lut:		Look-up table for tonemap curve
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_tonemap_config {
    pub detail_constant: __u16,
    pub detail_slope: __u16,
    pub iir_strength: __u16,
    pub strength: __u16,
    pub lut: [__u32; PISP_BE_TONEMAP_LUT_SIZE],
    pub __attribute__((packed)): },
//
// struct pisp_be_demosaic_config - PiSP Back End Demosaic config
//
// Demosaic configuration
//
// @sharper:		Use other Bayer channels to increase sharpness
// @fc_mode:		Built-in false colour suppression mode
// @pad:		Padding bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_demosaic_config {
    pub sharper: __u8,
    pub fc_mode: __u8,
    pub pad: [__u8; 2],
    pub __attribute__((packed)): },
//
// struct pisp_be_ccm_config - PiSP Back End CCM config
//
// Colour Correction Matrix configuration
//
// @coeffs:		Matrix coefficients
// @pad:		Padding bytes
// @offsets:		Offsets triplet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_ccm_config {
    pub coeffs: [__s16; 9],
    pub pad: [__u8; 2],
    pub offsets: [__s32; 3],
    pub __attribute__((packed)): },
//
// struct pisp_be_sat_control_config - PiSP Back End SAT config
//
// Saturation Control configuration
//
// @shift_r:		Left shift for Red colour channel
// @shift_g:		Left shift for Green colour channel
// @shift_b:		Left shift for Blue colour channel
// @pad:		Padding byte
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_sat_control_config {
    pub shift_r: __u8,
    pub shift_g: __u8,
    pub shift_b: __u8,
    pub pad: __u8,
    pub __attribute__((packed)): },
//
// struct pisp_be_false_colour_config - PiSP Back End False Colour config
//
// False Colour configuration
//
// @distance:		Distance of neighbouring pixels, either 1 or 2
// @pad:		Padding bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_false_colour_config {
    pub distance: __u8,
    pub pad: [__u8; 3],
    pub __attribute__((packed)): },
pub const PISP_BE_SHARPEN_SIZE: c_int = 5;
pub const PISP_BE_SHARPEN_FUNC_NUM_POINTS: c_int = 9;
//
// struct pisp_be_sharpen_config - PiSP Back End Sharpening config
//
// Sharpening configuration
//
// @kernel0:		Coefficient for filter 0
// @pad0:		Padding byte
// @kernel1:		Coefficient for filter 1
// @pad1:		Padding byte
// @kernel2:		Coefficient for filter 2
// @pad2:		Padding byte
// @kernel3:		Coefficient for filter 3
// @pad3:		Padding byte
// @kernel4:		Coefficient for filter 4
// @pad4:		Padding byte
// @threshold_offset0:	Offset for filter 0 response calculation
// @threshold_slope0:	Slope multiplier for the filter 0 response calculation
// @scale0:		Scale factor for filter 0 response calculation
// @pad5:		Padding byte
// @threshold_offset1:	Offset for filter 0 response calculation
// @threshold_slope1:	Slope multiplier for the filter 0 response calculation
// @scale1:		Scale factor for filter 0 response calculation
// @pad6:		Padding byte
// @threshold_offset2:	Offset for filter 0 response calculation
// @threshold_slope2:	Slope multiplier for the filter 0 response calculation
// @scale2:		Scale factor for filter 0 response calculation
// @pad7:		Padding byte
// @threshold_offset3:	Offset for filter 0 response calculation
// @threshold_slope3:	Slope multiplier for the filter 0 response calculation
// @scale3:		Scale factor for filter 0 response calculation
// @pad8:		Padding byte
// @threshold_offset4:	Offset for filter 0 response calculation
// @threshold_slope4:	Slope multiplier for the filter 0 response calculation
// @scale4:		Scale factor for filter 0 response calculation
// @pad9:		Padding byte
// @positive_strength:	Factor to scale the positive sharpening strength
// @positive_pre_limit:	Maximum allowed possible positive sharpening value
// @positive_func:	Gain factor applied to positive sharpening response
// @positive_limit:	Final gain factor applied to positive sharpening
// @negative_strength:	Factor to scale the negative sharpening strength
// @negative_pre_limit:	Maximum allowed possible negative sharpening value
// @negative_func:	Gain factor applied to negative sharpening response
// @negative_limit:	Final gain factor applied to negative sharpening
// @enables:		Filter enable mask
// @white:		White output pixel filter mask
// @black:		Black output pixel filter mask
// @grey:		Grey output pixel filter mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_sharpen_config {
    pub PISP_BE_SHARPEN_SIZE]: *mut *mut __s8 kernel0[PISP_BE_SHARPEN_SIZE,
    pub pad0: [__s8; 3],
    pub PISP_BE_SHARPEN_SIZE]: *mut *mut __s8 kernel1[PISP_BE_SHARPEN_SIZE,
    pub pad1: [__s8; 3],
    pub PISP_BE_SHARPEN_SIZE]: *mut *mut __s8 kernel2[PISP_BE_SHARPEN_SIZE,
    pub pad2: [__s8; 3],
    pub PISP_BE_SHARPEN_SIZE]: *mut *mut __s8 kernel3[PISP_BE_SHARPEN_SIZE,
    pub pad3: [__s8; 3],
    pub PISP_BE_SHARPEN_SIZE]: *mut *mut __s8 kernel4[PISP_BE_SHARPEN_SIZE,
    pub pad4: [__s8; 3],
    pub threshold_offset0: __u16,
    pub threshold_slope0: __u16,
    pub scale0: __u16,
    pub pad5: __u16,
    pub threshold_offset1: __u16,
    pub threshold_slope1: __u16,
    pub scale1: __u16,
    pub pad6: __u16,
    pub threshold_offset2: __u16,
    pub threshold_slope2: __u16,
    pub scale2: __u16,
    pub pad7: __u16,
    pub threshold_offset3: __u16,
    pub threshold_slope3: __u16,
    pub scale3: __u16,
    pub pad8: __u16,
    pub threshold_offset4: __u16,
    pub threshold_slope4: __u16,
    pub scale4: __u16,
    pub pad9: __u16,
    pub positive_strength: __u16,
    pub positive_pre_limit: __u16,
    pub positive_func: [__u16; PISP_BE_SHARPEN_FUNC_NUM_POINTS],
    pub positive_limit: __u16,
    pub negative_strength: __u16,
    pub negative_pre_limit: __u16,
    pub negative_func: [__u16; PISP_BE_SHARPEN_FUNC_NUM_POINTS],
    pub negative_limit: __u16,
    pub enables: __u8,
    pub white: __u8,
    pub black: __u8,
    pub grey: __u8,
    pub __attribute__((packed)): },
//
// struct pisp_be_sh_fc_combine_config - PiSP Back End Sharpening and
// False Colour config
//
// Sharpening and False Colour configuration
//
// @y_factor:		Control amount of desaturation of pixels being darkened
// @c1_factor:		Control amount of brightening of a pixel for the Cb
// channel
// @c2_factor:		Control amount of brightening of a pixel for the Cr
// channel
// @pad:		Padding byte
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_sh_fc_combine_config {
    pub y_factor: __u8,
    pub c1_factor: __u8,
    pub c2_factor: __u8,
    pub pad: __u8,
    pub __attribute__((packed)): },
pub const PISP_BE_GAMMA_LUT_SIZE: c_int = 64;
//
// struct pisp_be_gamma_config - PiSP Back End Gamma configuration
// @lut:		Gamma curve look-up table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_gamma_config {
    pub lut: [__u32; PISP_BE_GAMMA_LUT_SIZE],
    pub __attribute__((packed)): },
//
// struct pisp_be_crop_config - PiSP Back End Crop config
//
// Crop configuration
//
// @offset_x:		Number of pixels cropped from the left of the tile
// @offset_y:		Number of pixels cropped from the top of the tile
// @width:		Width of the cropped tile output
// @height:		Height of the cropped tile output
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_crop_config {
    pub offset_y: __u16 offset_x,,
    pub height: __u16 width,,
    pub __attribute__((packed)): },
pub const PISP_BE_RESAMPLE_FILTER_SIZE: c_int = 96;
//
// struct pisp_be_resample_config - PiSP Back End Resampling config
//
// Resample configuration
//
// @scale_factor_h:	Horizontal scale factor
// @scale_factor_v:	Vertical scale factor
// @coef:		Resample coefficients
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_resample_config {
    pub scale_factor_v: __u16 scale_factor_h,,
    pub coef: [__s16; PISP_BE_RESAMPLE_FILTER_SIZE],
    pub __attribute__((packed)): },
//
// struct pisp_be_resample_extra - PiSP Back End Resample config
//
// Resample configuration
//
// @scaled_width:	Width in pixels of the scaled output
// @scaled_height:	Height in pixels of the scaled output
// @initial_phase_h:	Initial horizontal phase
// @initial_phase_v:	Initial vertical phase
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_resample_extra {
    pub scaled_width: __u16,
    pub scaled_height: __u16,
    pub initial_phase_h: [__s16; 3],
    pub initial_phase_v: [__s16; 3],
    pub __attribute__((packed)): },
//
// struct pisp_be_downscale_config - PiSP Back End Downscale config
//
// Downscale configuration
//
// @scale_factor_h:	Horizontal scale factor
// @scale_factor_v:	Vertical scale factor
// @scale_recip_h:	Horizontal reciprocal factor
// @scale_recip_v:	Vertical reciprocal factor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_downscale_config {
    pub scale_factor_h: __u16,
    pub scale_factor_v: __u16,
    pub scale_recip_h: __u16,
    pub scale_recip_v: __u16,
    pub __attribute__((packed)): },
//
// struct pisp_be_downscale_extra - PiSP Back End Downscale Extra config
// @scaled_width:	Scaled image width
// @scaled_height:	Scaled image height
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_downscale_extra {
    pub scaled_width: __u16,
    pub scaled_height: __u16,
    pub __attribute__((packed)): },
//
// struct pisp_be_hog_config - PiSP Back End HOG config
//
// Histogram of Oriented Gradients configuration
//
// @compute_signed:	Set 0 for unsigned gradients, 1 for signed
// @channel_mix:	Channels proportions to use
// @stride:		Stride in bytes between blocks directly below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_hog_config {
    pub compute_signed: __u8,
    pub channel_mix: [__u8; 3],
    pub stride: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_axi_config {
    pub /: *mut *mut __u8 r_qos; / Read QoS,
    pub /: *mut *mut __u8 r_cache_prot; / Read { prot[2:0], cache[3:0] },
    pub /: *mut *mut __u8 w_qos; / Write QoS,
    pub /: *mut *mut __u8 w_cache_prot; / Write { prot[2:0], cache[3:0] },
    pub __attribute__((packed)): },
//
// enum pisp_be_transform - PiSP Back End Transform flags
// @PISP_BE_TRANSFORM_NONE:	No transform
// @PISP_BE_TRANSFORM_HFLIP:	Horizontal flip
// @PISP_BE_TRANSFORM_VFLIP:	Vertical flip
// @PISP_BE_TRANSFORM_ROT180:	180 degress rotation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_be_transform {
    PISP_BE_TRANSFORM_NONE = 0x0,
    PISP_BE_TRANSFORM_HFLIP = 0x1,
    PISP_BE_TRANSFORM_VFLIP = 0x2,
    PISP_BE_TRANSFORM_ROT180 =
    (PISP_BE_TRANSFORM_HFLIP | PISP_BE_TRANSFORM_VFLIP)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_output_format_config {
    pub image: pisp_image_format_config,
    pub transform: __u8,
    pub pad: [__u8; 3],
    pub lo: __u16,
    pub hi: __u16,
    pub lo2: __u16,
    pub hi2: __u16,
    pub __attribute__((packed)): },
//
// struct pisp_be_output_buffer_config - PiSP Back End Output buffer
// @addr:		Output buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_output_buffer_config {
// low 32 bits followed by high 32 bits (for each of 3 planes)
    pub addr: [__u32; 3][2],
    pub __attribute__((packed)): },
//
// struct pisp_be_hog_buffer_config - PiSP Back End HOG buffer
// @addr:		HOG buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_hog_buffer_config {
// low 32 bits followed by high 32 bits
    pub addr: [__u32; 2],
    pub __attribute__((packed)): },
//
// struct pisp_be_config - RaspberryPi PiSP Back End Processing configuration
//
// @input_buffer:		Input buffer addresses
// @tdn_input_buffer:		TDN input buffer addresses
// @stitch_input_buffer:	Stitch input buffer addresses
// @tdn_output_buffer:		TDN output buffer addresses
// @stitch_output_buffer:	Stitch output buffer addresses
// @output_buffer:		Output buffers addresses
// @hog_buffer:			HOG buffer addresses
// @global:			Global PiSP configuration
// @input_format:		Input image format
// @decompress:			Decompress configuration
// @dpc:			Defective Pixel Correction configuration
// @geq:			Green Equalisation configuration
// @tdn_input_format:		Temporal Denoise input format
// @tdn_decompress:		Temporal Denoise decompress configuration
// @tdn:			Temporal Denoise configuration
// @tdn_compress:		Temporal Denoise compress configuration
// @tdn_output_format:		Temporal Denoise output format
// @sdn:			Spatial Denoise configuration
// @blc:			Black Level Correction configuration
// @stitch_compress:		Stitch compress configuration
// @stitch_output_format:	Stitch output format
// @stitch_input_format:	Stitch input format
// @stitch_decompress:		Stitch decompress configuration
// @stitch:			Stitch configuration
// @lsc:			Lens Shading Correction configuration
// @wbg:			White Balance Gain configuration
// @cdn:			Colour Denoise configuration
// @cac:			Colour Aberration Correction configuration
// @debin:			Debinning configuration
// @tonemap:			Tonemapping configuration
// @demosaic:			Demosaicing configuration
// @ccm:			Colour Correction Matrix configuration
// @sat_control:		Saturation Control configuration
// @ycbcr:			YCbCr colour correction configuration
// @sharpen:			Sharpening configuration
// @false_colour:		False colour correction
// @sh_fc_combine:		Sharpening and False Colour correction
// @ycbcr_inverse:		Inverse YCbCr colour correction
// @gamma:			Gamma curve configuration
// @csc:			Color Space Conversion configuration
// @downscale:			Downscale configuration
// @resample:			Resampling configuration
// @output_format:		Output format configuration
// @hog:			HOG configuration
// @axi:			AXI bus configuration
// @lsc_extra:			LSC extra info
// @cac_extra:			CAC extra info
// @downscale_extra:		Downscaler extra info
// @resample_extra:		Resample extra info
// @crop:			Crop configuration
// @hog_format:			HOG format info
// @dirty_flags_bayer:		Bayer enable dirty flags
// (:c:type:`pisp_be_bayer_enable`)
// @dirty_flags_rgb:		RGB enable dirty flags
// (:c:type:`pisp_be_rgb_enable`)
// @dirty_flags_extra:		Extra dirty flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_config {
// I/O configuration:
    pub input_buffer: pisp_be_input_buffer_config,
    pub tdn_input_buffer: pisp_be_tdn_input_buffer_config,
    pub stitch_input_buffer: pisp_be_stitch_input_buffer_config,
    pub tdn_output_buffer: pisp_be_tdn_output_buffer_config,
    pub stitch_output_buffer: pisp_be_stitch_output_buffer_config,
    pub hog_buffer: pisp_be_hog_buffer_config,
// Processing configuration:
    pub global: pisp_be_global_config,
    pub input_format: pisp_image_format_config,
    pub decompress: pisp_decompress_config,
    pub dpc: pisp_be_dpc_config,
    pub geq: pisp_be_geq_config,
    pub tdn_input_format: pisp_image_format_config,
    pub tdn_decompress: pisp_decompress_config,
    pub tdn: pisp_be_tdn_config,
    pub tdn_compress: pisp_compress_config,
    pub tdn_output_format: pisp_image_format_config,
    pub sdn: pisp_be_sdn_config,
    pub blc: pisp_bla_config,
    pub stitch_compress: pisp_compress_config,
    pub stitch_output_format: pisp_image_format_config,
    pub stitch_input_format: pisp_image_format_config,
    pub stitch_decompress: pisp_decompress_config,
    pub stitch: pisp_be_stitch_config,
    pub lsc: pisp_be_lsc_config,
    pub wbg: pisp_wbg_config,
    pub cdn: pisp_be_cdn_config,
    pub cac: pisp_be_cac_config,
    pub debin: pisp_be_debin_config,
    pub tonemap: pisp_be_tonemap_config,
    pub demosaic: pisp_be_demosaic_config,
    pub ccm: pisp_be_ccm_config,
    pub sat_control: pisp_be_sat_control_config,
    pub ycbcr: pisp_be_ccm_config,
    pub sharpen: pisp_be_sharpen_config,
    pub false_colour: pisp_be_false_colour_config,
    pub sh_fc_combine: pisp_be_sh_fc_combine_config,
    pub ycbcr_inverse: pisp_be_ccm_config,
    pub gamma: pisp_be_gamma_config,
    pub csc: [pisp_be_ccm_config; PISP_BACK_END_NUM_OUTPUTS],
    pub downscale: [pisp_be_downscale_config; PISP_BACK_END_NUM_OUTPUTS],
    pub resample: [pisp_be_resample_config; PISP_BACK_END_NUM_OUTPUTS],
    pub hog: pisp_be_hog_config,
    pub axi: pisp_be_axi_config,
// Non-register fields:
    pub lsc_extra: pisp_be_lsc_extra,
    pub cac_extra: pisp_be_cac_extra,
    pub resample_extra: [pisp_be_resample_extra; PISP_BACK_END_NUM_OUTPUTS],
    pub crop: pisp_be_crop_config,
    pub hog_format: pisp_image_format_config,
    pub /: *mut *mut __u32 dirty_flags_bayer; / these use pisp_be_bayer_enable,
    pub /: *mut *mut __u32 dirty_flags_rgb; / use pisp_be_rgb_enable,
    pub /: *mut *mut __u32 dirty_flags_extra; / these use pisp_be_dirty_t,
    pub __attribute__((packed)): },
//
// enum pisp_tile_edge - PiSP Back End Tile position
// @PISP_LEFT_EDGE:		Left edge tile
// @PISP_RIGHT_EDGE:		Right edge tile
// @PISP_TOP_EDGE:		Top edge tile
// @PISP_BOTTOM_EDGE:		Bottom edge tile
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_tile_edge {
    PISP_LEFT_EDGE = (1 << 0),
    PISP_RIGHT_EDGE = (1 << 1),
    PISP_TOP_EDGE = (1 << 2),
    PISP_BOTTOM_EDGE = (1 << 3)
}

//
// struct pisp_tile - Raspberry Pi PiSP Back End tile configuration
//
// Tile parameters: each set of tile parameters is a 160-bytes block of data
// which contains the tile processing parameters.
//
// @edge:			Edge tile flag
// @pad0:			Padding bytes
// @input_addr_offset:		Top-left pixel offset, in bytes
// @input_addr_offset2:		Top-left pixel offset, in bytes for the second
// third image planes
// @input_offset_x:		Horizontal offset in pixels of this tile in the
// input image
// @input_offset_y:		Vertical offset in pixels of this tile in the
// input image
// @input_width:		Width in pixels of this tile
// @input_height:		Height in pixels of the this tile
// @tdn_input_addr_offset:	TDN input image offset, in bytes
// @tdn_output_addr_offset:	TDN output image offset, in bytes
// @stitch_input_addr_offset:	Stitch input image offset, in bytes
// @stitch_output_addr_offset:	Stitch output image offset, in bytes
// @lsc_grid_offset_x:		Horizontal offset in the LSC table for this tile
// @lsc_grid_offset_y:		Vertical offset in the LSC table for this tile
// @cac_grid_offset_x:		Horizontal offset in the CAC table for this tile
// @cac_grid_offset_y:		Horizontal offset in the CAC table for this tile
// @crop_x_start:		Number of pixels cropped from the left of the
// tile
// @crop_x_end:			Number of pixels cropped from the right of the
// tile
// @crop_y_start:		Number of pixels cropped from the top of the
// tile
// @crop_y_end:			Number of pixels cropped from the bottom of the
// tile
// @downscale_phase_x:		Initial horizontal phase in pixels
// @downscale_phase_y:		Initial vertical phase in pixels
// @resample_in_width:		Width in pixels of the tile entering the
// Resample block
// @resample_in_height:		Height in pixels of the tile entering the
// Resample block
// @resample_phase_x:		Initial horizontal phase for the Resample block
// @resample_phase_y:		Initial vertical phase for the Resample block
// @output_offset_x:		Horizontal offset in pixels where the tile will
// be written into the output image
// @output_offset_y:		Vertical offset in pixels where the tile will be
// written into the output image
// @output_width:		Width in pixels in the output image of this tile
// @output_height:		Height in pixels in the output image of this tile
// @output_addr_offset:		Offset in bytes into the output buffer
// @output_addr_offset2:	Offset in bytes into the output buffer for the
// second and third plane
// @output_hog_addr_offset:	Offset in bytes into the HOG buffer where
// results of this tile are to be written
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_tile {
    pub /: *mut *mut __u8 edge; / enum pisp_tile_edge,
    pub pad0: [__u8; 3],
// 4 bytes
    pub input_addr_offset: __u32,
    pub input_addr_offset2: __u32,
    pub input_offset_x: __u16,
    pub input_offset_y: __u16,
    pub input_width: __u16,
    pub input_height: __u16,
// 20 bytes
    pub tdn_input_addr_offset: __u32,
    pub tdn_output_addr_offset: __u32,
    pub stitch_input_addr_offset: __u32,
    pub stitch_output_addr_offset: __u32,
// 36 bytes
    pub lsc_grid_offset_x: __u32,
    pub lsc_grid_offset_y: __u32,
// 44 bytes
    pub cac_grid_offset_x: __u32,
    pub cac_grid_offset_y: __u32,
// 52 bytes
    pub crop_x_start: [__u16; PISP_BACK_END_NUM_OUTPUTS],
    pub crop_x_end: [__u16; PISP_BACK_END_NUM_OUTPUTS],
    pub crop_y_start: [__u16; PISP_BACK_END_NUM_OUTPUTS],
    pub crop_y_end: [__u16; PISP_BACK_END_NUM_OUTPUTS],
// 68 bytes
// Ordering is planes then branches
    pub PISP_BACK_END_NUM_OUTPUTS]: *mut *mut __u16 downscale_phase_x[3,
    pub PISP_BACK_END_NUM_OUTPUTS]: *mut *mut __u16 downscale_phase_y[3,
// 92 bytes
    pub resample_in_width: [__u16; PISP_BACK_END_NUM_OUTPUTS],
    pub resample_in_height: [__u16; PISP_BACK_END_NUM_OUTPUTS],
// 100 bytes
// Ordering is planes then branches
    pub PISP_BACK_END_NUM_OUTPUTS]: *mut *mut __u16 resample_phase_x[3,
    pub PISP_BACK_END_NUM_OUTPUTS]: *mut *mut __u16 resample_phase_y[3,
// 124 bytes
    pub output_offset_x: [__u16; PISP_BACK_END_NUM_OUTPUTS],
    pub output_offset_y: [__u16; PISP_BACK_END_NUM_OUTPUTS],
    pub output_width: [__u16; PISP_BACK_END_NUM_OUTPUTS],
    pub output_height: [__u16; PISP_BACK_END_NUM_OUTPUTS],
// 140 bytes
    pub output_addr_offset: [__u32; PISP_BACK_END_NUM_OUTPUTS],
    pub output_addr_offset2: [__u32; PISP_BACK_END_NUM_OUTPUTS],
// 156 bytes
    pub output_hog_addr_offset: __u32,
// 160 bytes
    pub __attribute__((packed)): },
//
// struct pisp_be_tiles_config - Raspberry Pi PiSP Back End configuration
// @tiles:	Tile descriptors
// @num_tiles:	Number of tiles
// @config:	PiSP Back End configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_tiles_config {
    pub config: pisp_be_config,
    pub tiles: [pisp_tile; PISP_BACK_END_NUM_TILES],
    pub num_tiles: __u32,
    pub __attribute__((packed)): },
