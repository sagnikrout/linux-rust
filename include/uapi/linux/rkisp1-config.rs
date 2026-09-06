//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rkisp1-config.h
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


// SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR MIT)
//
// Rockchip ISP1 userspace API
// Copyright (C) 2017 Rockchip Electronics Co., Ltd.
//

// Defect Pixel Cluster Detection

// Black Level Subtraction

// Sensor De-gamma

// Histogram statistics configuration

// Lens Shade Control

// Auto White Balance Gain

// Filter

// Bayer Demosaic

// Cross Talk

// Gamma Out Curve

// Color Processing

// Auto Focus Control statistics configuration

// Auto White Balancing statistics configuration

// Image Effect

// Auto Exposure Control statistics configuration

// Wide Dynamic Range

// Denoise Pre-Filter

// Denoise Pre-Filter Strength

pub const RKISP1_CIF_ISP_CTK_COEFF_MAX: c_uint = 0x100;
pub const RKISP1_CIF_ISP_CTK_OFFSET_MAX: c_uint = 0x800;
pub const RKISP1_CIF_ISP_AE_MEAN_MAX_V10: c_int = 25;
pub const RKISP1_CIF_ISP_AE_MEAN_MAX_V12: c_int = 81;

pub const RKISP1_CIF_ISP_HIST_BIN_N_MAX_V10: c_int = 16;
pub const RKISP1_CIF_ISP_HIST_BIN_N_MAX_V12: c_int = 32;

pub const RKISP1_CIF_ISP_AFM_MAX_WINDOWS: c_int = 3;
pub const RKISP1_CIF_ISP_DEGAMMA_CURVE_SIZE: c_int = 17;
pub const RKISP1_CIF_ISP_BDM_MAX_TH: c_uint = 0xff;
//
// Black level compensation
//
// maximum value for horizontal start address
pub const RKISP1_CIF_ISP_BLS_START_H_MAX: c_uint = 0x00000fff;
// maximum value for horizontal stop address
pub const RKISP1_CIF_ISP_BLS_STOP_H_MAX: c_uint = 0x00000fff;
// maximum value for vertical start address
pub const RKISP1_CIF_ISP_BLS_START_V_MAX: c_uint = 0x00000fff;
// maximum value for vertical stop address
pub const RKISP1_CIF_ISP_BLS_STOP_V_MAX: c_uint = 0x00000fff;
// maximum is 2^18 = 262144
pub const RKISP1_CIF_ISP_BLS_SAMPLES_MAX: c_uint = 0x00000012;
// maximum value for fixed black level
pub const RKISP1_CIF_ISP_BLS_FIX_SUB_MAX: c_uint = 0x00000fff;
// minimum value for fixed black level
pub const RKISP1_CIF_ISP_BLS_FIX_SUB_MIN: c_uint = 0xfffff000;
// 13 bit range (signed)
pub const RKISP1_CIF_ISP_BLS_FIX_MASK: c_uint = 0x00001fff;
//
// Automatic white balance measurements
//
pub const RKISP1_CIF_ISP_AWB_MAX_GRID: c_int = 1;
pub const RKISP1_CIF_ISP_AWB_MAX_FRAMES: c_int = 7;
//
// Gamma out
//
// Maximum number of color samples supported
pub const RKISP1_CIF_ISP_GAMMA_OUT_MAX_SAMPLES_V10: c_int = 17;
pub const RKISP1_CIF_ISP_GAMMA_OUT_MAX_SAMPLES_V12: c_int = 34;

//
// Lens shade correction
//
pub const RKISP1_CIF_ISP_LSC_SECTORS_TBL_SIZE: c_int = 8;
//
// The following matches the tuning process,
// not the max capabilities of the chip.
//
pub const RKISP1_CIF_ISP_LSC_SAMPLES_MAX: c_int = 17;
//
// Histogram calculation
//
pub const RKISP1_CIF_ISP_HISTOGRAM_WEIGHT_GRIDS_SIZE_V10: c_int = 25;
pub const RKISP1_CIF_ISP_HISTOGRAM_WEIGHT_GRIDS_SIZE_V12: c_int = 81;

//
// Defect Pixel Cluster Correction
//
pub const RKISP1_CIF_ISP_DPCC_METHODS_MAX: c_int = 3;

// 0-2 for sets 1-3

//
// Denoising pre filter
//
pub const RKISP1_CIF_ISP_DPF_MAX_NLF_COEFFS: c_int = 17;
pub const RKISP1_CIF_ISP_DPF_MAX_SPATIAL_COEFFS: c_int = 6;
//
// Compand
//
pub const RKISP1_CIF_ISP_COMPAND_NUM_POINTS: c_int = 64;
//
// Wide Dynamic Range
//
pub const RKISP1_CIF_ISP_WDR_CURVE_NUM_INTERV: c_int = 32;

pub const RKISP1_CIF_ISP_WDR_CURVE_NUM_DY_REGS: c_int = 4;
//
// Measurement types
//

//
// enum rkisp1_cif_isp_version - ISP variants
//
// @RKISP1_V10: Used at least in RK3288 and RK3399.
// @RKISP1_V11: Declared in the original vendor code, but not used. Same number
// of entries in grids and histogram as v10.
// @RKISP1_V12: Used at least in RK3326 and PX30.
// @RKISP1_V13: Used at least in RK1808. Same number of entries in grids and
// histogram as v12.
// @RKISP1_V_IMX8MP: Used in at least i.MX8MP. Same number of entries in grids
// and histogram as v10.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_version {
    RKISP1_V10 = 10,
    RKISP1_V11,
    RKISP1_V12,
    RKISP1_V13,
    RKISP1_V_IMX8MP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_histogram_mode {
    RKISP1_CIF_ISP_HISTOGRAM_MODE_DISABLE,
    RKISP1_CIF_ISP_HISTOGRAM_MODE_RGB_COMBINED,
    RKISP1_CIF_ISP_HISTOGRAM_MODE_R_HISTOGRAM,
    RKISP1_CIF_ISP_HISTOGRAM_MODE_G_HISTOGRAM,
    RKISP1_CIF_ISP_HISTOGRAM_MODE_B_HISTOGRAM,
    RKISP1_CIF_ISP_HISTOGRAM_MODE_Y_HISTOGRAM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_awb_mode_type {
    RKISP1_CIF_ISP_AWB_MODE_MANUAL,
    RKISP1_CIF_ISP_AWB_MODE_RGB,
    RKISP1_CIF_ISP_AWB_MODE_YCBCR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_flt_mode {
    RKISP1_CIF_ISP_FLT_STATIC_MODE,
    RKISP1_CIF_ISP_FLT_DYNAMIC_MODE
}

//
// enum rkisp1_cif_isp_exp_ctrl_autostop - stop modes
// @RKISP1_CIF_ISP_EXP_CTRL_AUTOSTOP_0: continuous measurement
// @RKISP1_CIF_ISP_EXP_CTRL_AUTOSTOP_1: stop measuring after a complete frame
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_exp_ctrl_autostop {
    RKISP1_CIF_ISP_EXP_CTRL_AUTOSTOP_0 = 0,
    RKISP1_CIF_ISP_EXP_CTRL_AUTOSTOP_1 = 1,
}

//
// enum rkisp1_cif_isp_exp_meas_mode - Exposure measure mode
// @RKISP1_CIF_ISP_EXP_MEASURING_MODE_0: Y = 16 + 0.25R + 0.5G + 0.1094B
// @RKISP1_CIF_ISP_EXP_MEASURING_MODE_1: Y = (R + G + B) x (85/256)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_exp_meas_mode {
    RKISP1_CIF_ISP_EXP_MEASURING_MODE_0,
    RKISP1_CIF_ISP_EXP_MEASURING_MODE_1,
}

// ---------- PART1: Input Parameters ------------
//
// struct rkisp1_cif_isp_window -  measurement window.
//
// Measurements are calculated per window inside the frame.
// This struct represents a window for a measurement.
//
// @h_offs: the horizontal offset of the window from the left of the frame in pixels.
// @v_offs: the vertical offset of the window from the top of the frame in pixels.
// @h_size: the horizontal size of the window in pixels
// @v_size: the vertical size of the window in pixels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_window {
    pub h_offs: __u16,
    pub v_offs: __u16,
    pub h_size: __u16,
    pub v_size: __u16,
}

//
// struct rkisp1_cif_isp_bls_fixed_val - BLS fixed subtraction values
//
// The values will be subtracted from the sensor
// values. Therefore a negative value means addition instead of subtraction!
//
// @r: Fixed (signed!) subtraction value for Bayer pattern R
// @gr: Fixed (signed!) subtraction value for Bayer pattern Gr
// @gb: Fixed (signed!) subtraction value for Bayer pattern Gb
// @b: Fixed (signed!) subtraction value for Bayer pattern B
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_bls_fixed_val {
    pub r: __s16,
    pub gr: __s16,
    pub gb: __s16,
    pub b: __s16,
}

//
// struct rkisp1_cif_isp_bls_config - Configuration used by black level subtraction
//
// @enable_auto: Automatic mode activated means that the measured values
// are subtracted. Otherwise the fixed subtraction
// values will be subtracted.
// @en_windows: enabled window
// @bls_window1: Measurement window 1 size
// @bls_window2: Measurement window 2 size
// @bls_samples: Set amount of measured pixels for each Bayer position
// (A, B,C and D) to 2^bls_samples.
// @fixed_val: Fixed subtraction values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_bls_config {
    pub enable_auto: __u8,
    pub en_windows: __u8,
    pub bls_window1: rkisp1_cif_isp_window,
    pub bls_window2: rkisp1_cif_isp_window,
    pub bls_samples: __u8,
    pub fixed_val: rkisp1_cif_isp_bls_fixed_val,
}

//
// struct rkisp1_cif_isp_dpcc_methods_config - DPCC methods set configuration
//
// This structure stores the configuration of one set of methods for the DPCC
// algorithm. Multiple methods can be selected in each set (independently for
// the Green and Red/Blue components) through the @method field, the result is
// the logical AND of all enabled methods. The remaining fields set thresholds
// and factors for each method.
//
// @method: Method enable bits (RKISP1_CIF_ISP_DPCC_METHODS_SET_*)
// @line_thresh: Line threshold (RKISP1_CIF_ISP_DPCC_LINE_THRESH_*)
// @line_mad_fac: Line Mean Absolute Difference factor (RKISP1_CIF_ISP_DPCC_LINE_MAD_FAC_*)
// @pg_fac: Peak gradient factor (RKISP1_CIF_ISP_DPCC_PG_FAC_*)
// @rnd_thresh: Rank Neighbor Difference threshold (RKISP1_CIF_ISP_DPCC_RND_THRESH_*)
// @rg_fac: Rank gradient factor (RKISP1_CIF_ISP_DPCC_RG_FAC_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpcc_methods_config {
    pub method: __u32,
    pub line_thresh: __u32,
    pub line_mad_fac: __u32,
    pub pg_fac: __u32,
    pub rnd_thresh: __u32,
    pub rg_fac: __u32,
}

//
// struct rkisp1_cif_isp_dpcc_config - Configuration used by DPCC
//
// Configuration used by Defect Pixel Cluster Correction. Three sets of methods
// can be configured and selected through the @set_use field. The result is the
// logical OR of all enabled sets.
//
// @mode: DPCC mode (RKISP1_CIF_ISP_DPCC_MODE_*)
// @output_mode: Interpolation output mode (RKISP1_CIF_ISP_DPCC_OUTPUT_MODE_*)
// @set_use: Methods sets selection (RKISP1_CIF_ISP_DPCC_SET_USE_*)
// @methods: Methods sets configuration
// @ro_limits: Rank order limits (RKISP1_CIF_ISP_DPCC_RO_LIMITS_*)
// @rnd_offs: Differential rank offsets for rank neighbor difference (RKISP1_CIF_ISP_DPCC_RND_OFFS_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpcc_config {
    pub mode: __u32,
    pub output_mode: __u32,
    pub set_use: __u32,
    pub methods: [rkisp1_cif_isp_dpcc_methods_config; RKISP1_CIF_ISP_DPCC_METHODS_MAX],
    pub ro_limits: __u32,
    pub rnd_offs: __u32,
}

//
// struct rkisp1_cif_isp_gamma_corr_curve - gamma curve point definition y-axis (output).
//
// The reset values define a linear curve which has the same effect as bypass. Reset values are:
// gamma_y[0] = 0x0000, gamma_y[1] = 0x0100, ... gamma_y[15] = 0x0f00, gamma_y[16] = 0xfff
//
// @gamma_y: the values for the y-axis of gamma curve points. Each value is 12 bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_gamma_corr_curve {
    pub gamma_y: [__u16; RKISP1_CIF_ISP_DEGAMMA_CURVE_SIZE],
}

//
// struct rkisp1_cif_isp_gamma_curve_x_axis_pnts - De-Gamma Curve definition x increments
// (sampling points). gamma_dx0 is for the lower samples (1-8), gamma_dx1 is for the
// higher samples (9-16). The reset values for both fields is 0x44444444. This means
// that each sample is 4 units away from the previous one on the x-axis.
//
// @gamma_dx0: gamma curve sample points definitions. Bits 0:2 for sample 1. Bit 3 unused.
// Bits 4:6 for sample 2. bit 7 unused ... Bits 28:30 for sample 8. Bit 31 unused
// @gamma_dx1: gamma curve sample points definitions. Bits 0:2 for sample 9. Bit 3 unused.
// Bits 4:6 for sample 10. bit 7 unused ... Bits 28:30 for sample 16. Bit 31 unused
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_gamma_curve_x_axis_pnts {
    pub gamma_dx0: __u32,
    pub gamma_dx1: __u32,
}

//
// struct rkisp1_cif_isp_sdg_config - Configuration used by sensor degamma
//
// @curve_r: gamma curve point definition axis for red
// @curve_g: gamma curve point definition axis for green
// @curve_b: gamma curve point definition axis for blue
// @xa_pnts: x axis increments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_sdg_config {
    pub curve_r: rkisp1_cif_isp_gamma_corr_curve,
    pub curve_g: rkisp1_cif_isp_gamma_corr_curve,
    pub curve_b: rkisp1_cif_isp_gamma_corr_curve,
    pub xa_pnts: rkisp1_cif_isp_gamma_curve_x_axis_pnts,
}

//
// struct rkisp1_cif_isp_lsc_config - Configuration used by Lens shading correction
//
// @r_data_tbl: sample table red
// @gr_data_tbl: sample table green (red)
// @gb_data_tbl: sample table green (blue)
// @b_data_tbl: sample table blue
// @x_grad_tbl: gradient table x
// @y_grad_tbl: gradient table y
// @x_size_tbl: size table x
// @y_size_tbl: size table y
// @config_width: not used at the moment
// @config_height: not used at the moment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_lsc_config {
    pub r_data_tbl: [__u16; RKISP1_CIF_ISP_LSC_SAMPLES_MAX][RKISP1_CIF_ISP_LSC_SAMPLES_MAX],
    pub gr_data_tbl: [__u16; RKISP1_CIF_ISP_LSC_SAMPLES_MAX][RKISP1_CIF_ISP_LSC_SAMPLES_MAX],
    pub gb_data_tbl: [__u16; RKISP1_CIF_ISP_LSC_SAMPLES_MAX][RKISP1_CIF_ISP_LSC_SAMPLES_MAX],
    pub b_data_tbl: [__u16; RKISP1_CIF_ISP_LSC_SAMPLES_MAX][RKISP1_CIF_ISP_LSC_SAMPLES_MAX],
    pub x_grad_tbl: [__u16; RKISP1_CIF_ISP_LSC_SECTORS_TBL_SIZE],
    pub y_grad_tbl: [__u16; RKISP1_CIF_ISP_LSC_SECTORS_TBL_SIZE],
    pub x_size_tbl: [__u16; RKISP1_CIF_ISP_LSC_SECTORS_TBL_SIZE],
    pub y_size_tbl: [__u16; RKISP1_CIF_ISP_LSC_SECTORS_TBL_SIZE],
    pub config_width: __u16,
    pub config_height: __u16,
}

//
// struct rkisp1_cif_isp_ie_config - Configuration used by image effects
//
// @effect: values from 'enum v4l2_colorfx'. Possible values are: V4L2_COLORFX_SEPIA,
// V4L2_COLORFX_SET_CBCR, V4L2_COLORFX_AQUA, V4L2_COLORFX_EMBOSS,
// V4L2_COLORFX_SKETCH,   V4L2_COLORFX_BW,   V4L2_COLORFX_NEGATIVE
// @color_sel: bits 0:2 - colors bitmask (001 - blue, 010 - green, 100 - red).
// bits 8:15 - Threshold value of the RGB colors for the color selection effect.
// @eff_mat_1: 3x3 Matrix Coefficients for Emboss Effect 1
// @eff_mat_2: 3x3 Matrix Coefficients for Emboss Effect 2
// @eff_mat_3: 3x3 Matrix Coefficients for Emboss 3/Sketch 1
// @eff_mat_4: 3x3 Matrix Coefficients for Sketch Effect 2
// @eff_mat_5: 3x3 Matrix Coefficients for Sketch Effect 3
// @eff_tint: Chrominance increment values of tint (used for sepia effect)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_ie_config {
    pub effect: __u16,
    pub color_sel: __u16,
    pub eff_mat_1: __u16,
    pub eff_mat_2: __u16,
    pub eff_mat_3: __u16,
    pub eff_mat_4: __u16,
    pub eff_mat_5: __u16,
    pub eff_tint: __u16,
}

//
// struct rkisp1_cif_isp_cproc_config - Configuration used by Color Processing
//
// @c_out_range: Chrominance pixel clipping range at output.
// (0 for limit, 1 for full)
// @y_in_range: Luminance pixel clipping range at output.
// @y_out_range: Luminance pixel clipping range at output.
// @contrast: 00~ff, 0.0~1.992
// @brightness: 80~7F, -128~+127
// @sat: saturation, 00~FF, 0.0~1.992
// @hue: 80~7F, -90~+87.188
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_cproc_config {
    pub c_out_range: __u8,
    pub y_in_range: __u8,
    pub y_out_range: __u8,
    pub contrast: __u8,
    pub brightness: __u8,
    pub sat: __u8,
    pub hue: __u8,
}

//
// struct rkisp1_cif_isp_awb_meas_config - Configuration for the AWB statistics
//
// @awb_mode: the awb meas mode. From enum rkisp1_cif_isp_awb_mode_type.
// @awb_wnd: white balance measurement window (in pixels)
// @max_y: only pixels values < max_y contribute to awb measurement, set to 0
// to disable this feature
// @min_y: only pixels values > min_y contribute to awb measurement
// @max_csum: Chrominance sum maximum value, only consider pixels with Cb+Cr,
// smaller than threshold for awb measurements
// @min_c: Chrominance minimum value, only consider pixels with Cb/Cr
// each greater than threshold value for awb measurements
// @frames: number of frames - 1 used for mean value calculation
// (ucFrames=0 means 1 Frame)
// @awb_ref_cr: reference Cr value for AWB regulation, target for AWB
// @awb_ref_cb: reference Cb value for AWB regulation, target for AWB
// @enable_ymax_cmp: enable Y_MAX compare (Not valid in RGB measurement mode.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_awb_meas_config {
//
// Note: currently the h and v offsets are mapped to grid offsets
//
    pub awb_wnd: rkisp1_cif_isp_window,
    pub awb_mode: __u32,
    pub max_y: __u8,
    pub min_y: __u8,
    pub max_csum: __u8,
    pub min_c: __u8,
    pub frames: __u8,
    pub awb_ref_cr: __u8,
    pub awb_ref_cb: __u8,
    pub enable_ymax_cmp: __u8,
}

//
// struct rkisp1_cif_isp_awb_gain_config - Configuration used by auto white balance gain
//
// All fields in this struct are 10 bit, where:
// 0x100h = 1, unsigned integer value, range 0 to 4 with 8 bit fractional part.
//
// out_data_x = ( AWB_GAIN_X * in_data + 128) >> 8
//
// @gain_red: gain value for red component.
// @gain_green_r: gain value for green component in red line.
// @gain_blue: gain value for blue component.
// @gain_green_b: gain value for green component in blue line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_awb_gain_config {
    pub gain_red: __u16,
    pub gain_green_r: __u16,
    pub gain_blue: __u16,
    pub gain_green_b: __u16,
}

//
// struct rkisp1_cif_isp_flt_config - Configuration used by ISP filtering
//
// All 4 threshold fields (thresh_*) are 10 bits.
// All 6 factor fields (fac_*) are 6 bits.
//
// @mode: ISP_FILT_MODE register fields (from enum rkisp1_cif_isp_flt_mode)
// @grn_stage1: Green filter stage 1 select (range 0x0...0x8)
// @chr_h_mode: Chroma filter horizontal mode
// @chr_v_mode: Chroma filter vertical mode
// @thresh_bl0: If thresh_bl1 < sum_grad < thresh_bl0 then fac_bl0 is selected (blurring th)
// @thresh_bl1: If sum_grad < thresh_bl1 then fac_bl1 is selected (blurring th)
// @thresh_sh0: If thresh_sh0 < sum_grad < thresh_sh1 then thresh_sh0 is selected (sharpening th)
// @thresh_sh1: If thresh_sh1 < sum_grad then thresh_sh1 is selected (sharpening th)
// @lum_weight: Parameters for luminance weight function.
// @fac_sh1: filter factor for sharp1 level
// @fac_sh0: filter factor for sharp0 level
// @fac_mid: filter factor for mid level and for static filter mode
// @fac_bl0: filter factor for blur 0 level
// @fac_bl1: filter factor for blur 1 level (max blur)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_flt_config {
    pub mode: __u32,
    pub grn_stage1: __u8,
    pub chr_h_mode: __u8,
    pub chr_v_mode: __u8,
    pub thresh_bl0: __u32,
    pub thresh_bl1: __u32,
    pub thresh_sh0: __u32,
    pub thresh_sh1: __u32,
    pub lum_weight: __u32,
    pub fac_sh1: __u32,
    pub fac_sh0: __u32,
    pub fac_mid: __u32,
    pub fac_bl0: __u32,
    pub fac_bl1: __u32,
}

//
// struct rkisp1_cif_isp_bdm_config - Configuration used by Bayer DeMosaic
//
// @demosaic_th: threshold for bayer demosaicing texture detection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_bdm_config {
    pub demosaic_th: __u8,
}

//
// struct rkisp1_cif_isp_ctk_config - Configuration used by Cross Talk correction
//
// @coeff: color correction matrix. Values are 11-bit signed fixed-point numbers with 4 bit integer
// and 7 bit fractional part, ranging from -8 (0x400) to +7.992 (0x3FF). 0 is
// represented by 0x000 and a coefficient value of 1 as 0x080.
// @ct_offset: Red, Green, Blue offsets for the crosstalk correction matrix
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_ctk_config {
    pub coeff: [__u16; 3][3],
    pub ct_offset: [__u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_goc_mode {
    RKISP1_CIF_ISP_GOC_MODE_LOGARITHMIC,
    RKISP1_CIF_ISP_GOC_MODE_EQUIDISTANT
}

//
// struct rkisp1_cif_isp_goc_config - Configuration used by Gamma Out correction
//
// @mode: goc mode (from enum rkisp1_cif_isp_goc_mode)
// @gamma_y: gamma out curve y-axis for all color components
//
// The number of entries of @gamma_y depends on the hardware revision
// as is reported by the hw_revision field of the struct media_device_info
// that is returned by ioctl MEDIA_IOC_DEVICE_INFO.
//
// V10 has RKISP1_CIF_ISP_GAMMA_OUT_MAX_SAMPLES_V10 entries, V12 has
// RKISP1_CIF_ISP_GAMMA_OUT_MAX_SAMPLES_V12 entries.
// RKISP1_CIF_ISP_GAMMA_OUT_MAX_SAMPLES is equal to the maximum of the two.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_goc_config {
    pub mode: __u32,
    pub gamma_y: [__u16; RKISP1_CIF_ISP_GAMMA_OUT_MAX_SAMPLES],
}

//
// struct rkisp1_cif_isp_hst_config - Configuration for Histogram statistics
//
// @mode: histogram mode (from enum rkisp1_cif_isp_histogram_mode)
// @histogram_predivider: process every stepsize pixel, all other pixels are
// skipped
// @meas_window: coordinates of the measure window
// @hist_weight: weighting factor for sub-windows
//
// The number of entries of @hist_weight depends on the hardware revision
// as is reported by the hw_revision field of the struct media_device_info
// that is returned by ioctl MEDIA_IOC_DEVICE_INFO.
//
// V10 has RKISP1_CIF_ISP_HISTOGRAM_WEIGHT_GRIDS_SIZE_V10 entries, V12 has
// RKISP1_CIF_ISP_HISTOGRAM_WEIGHT_GRIDS_SIZE_V12 entries.
// RKISP1_CIF_ISP_HISTOGRAM_WEIGHT_GRIDS_SIZE is equal to the maximum of the
// two.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_hst_config {
    pub mode: __u32,
    pub histogram_predivider: __u8,
    pub meas_window: rkisp1_cif_isp_window,
    pub hist_weight: [__u8; RKISP1_CIF_ISP_HISTOGRAM_WEIGHT_GRIDS_SIZE],
}

//
// struct rkisp1_cif_isp_aec_config - Configuration for Auto Exposure statistics
//
// @mode: Exposure measure mode (from enum rkisp1_cif_isp_exp_meas_mode)
// @autostop: stop mode (from enum rkisp1_cif_isp_exp_ctrl_autostop)
// @meas_window: coordinates of the measure window
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_aec_config {
    pub mode: __u32,
    pub autostop: __u32,
    pub meas_window: rkisp1_cif_isp_window,
}

//
// struct rkisp1_cif_isp_afc_config - Configuration for the Auto Focus statistics
//
// @num_afm_win: max RKISP1_CIF_ISP_AFM_MAX_WINDOWS
// @afm_win: coordinates of the meas window
// @thres: threshold used for minimizing the influence of noise
// @var_shift: the number of bits for the shift operation at the end of the
// calculation chain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_afc_config {
    pub num_afm_win: __u8,
    pub afm_win: [rkisp1_cif_isp_window; RKISP1_CIF_ISP_AFM_MAX_WINDOWS],
    pub thres: __u32,
    pub var_shift: __u32,
}

//
// enum rkisp1_cif_isp_dpf_gain_usage - dpf gain usage
// @RKISP1_CIF_ISP_DPF_GAIN_USAGE_DISABLED: don't use any gains in preprocessing stage
// @RKISP1_CIF_ISP_DPF_GAIN_USAGE_NF_GAINS: use only the noise function gains from
// registers DPF_NF_GAIN_R, ...
// @RKISP1_CIF_ISP_DPF_GAIN_USAGE_LSC_GAINS:  use only the gains from LSC module
// @RKISP1_CIF_ISP_DPF_GAIN_USAGE_NF_LSC_GAINS: use the noise function gains and the
// gains from LSC module
// @RKISP1_CIF_ISP_DPF_GAIN_USAGE_AWB_GAINS: use only the gains from AWB module
// @RKISP1_CIF_ISP_DPF_GAIN_USAGE_AWB_LSC_GAINS: use the gains from AWB and LSC module
// @RKISP1_CIF_ISP_DPF_GAIN_USAGE_MAX: upper border (only for an internal evaluation)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_dpf_gain_usage {
    RKISP1_CIF_ISP_DPF_GAIN_USAGE_DISABLED,
    RKISP1_CIF_ISP_DPF_GAIN_USAGE_NF_GAINS,
    RKISP1_CIF_ISP_DPF_GAIN_USAGE_LSC_GAINS,
    RKISP1_CIF_ISP_DPF_GAIN_USAGE_NF_LSC_GAINS,
    RKISP1_CIF_ISP_DPF_GAIN_USAGE_AWB_GAINS,
    RKISP1_CIF_ISP_DPF_GAIN_USAGE_AWB_LSC_GAINS,
    RKISP1_CIF_ISP_DPF_GAIN_USAGE_MAX
}

//
// enum rkisp1_cif_isp_dpf_rb_filtersize - Red and blue filter sizes
// @RKISP1_CIF_ISP_DPF_RB_FILTERSIZE_13x9: red and blue filter kernel size 13x9
// (means 7x5 active pixel)
// @RKISP1_CIF_ISP_DPF_RB_FILTERSIZE_9x9: red and blue filter kernel size 9x9
// (means 5x5 active pixel)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_dpf_rb_filtersize {
    RKISP1_CIF_ISP_DPF_RB_FILTERSIZE_13x9,
    RKISP1_CIF_ISP_DPF_RB_FILTERSIZE_9x9,
}

//
// enum rkisp1_cif_isp_dpf_nll_scale_mode - dpf noise level scale mode
// @RKISP1_CIF_ISP_NLL_SCALE_LINEAR: use a linear scaling
// @RKISP1_CIF_ISP_NLL_SCALE_LOGARITHMIC: use a logarithmic scaling
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_dpf_nll_scale_mode {
    RKISP1_CIF_ISP_NLL_SCALE_LINEAR,
    RKISP1_CIF_ISP_NLL_SCALE_LOGARITHMIC,
}

//
// struct rkisp1_cif_isp_dpf_nll - Noise level lookup
//
// @coeff: Noise level Lookup coefficient
// @scale_mode: dpf noise level scale mode (from enum rkisp1_cif_isp_dpf_nll_scale_mode)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpf_nll {
    pub coeff: [__u16; RKISP1_CIF_ISP_DPF_MAX_NLF_COEFFS],
    pub scale_mode: __u32,
}

//
// struct rkisp1_cif_isp_dpf_rb_flt - Red blue filter config
//
// @fltsize: The filter size for the red and blue pixels
// (from enum rkisp1_cif_isp_dpf_rb_filtersize)
// @spatial_coeff: Spatial weights
// @r_enable: enable filter processing for red pixels
// @b_enable: enable filter processing for blue pixels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpf_rb_flt {
    pub fltsize: __u32,
    pub spatial_coeff: [__u8; RKISP1_CIF_ISP_DPF_MAX_SPATIAL_COEFFS],
    pub r_enable: __u8,
    pub b_enable: __u8,
}

//
// struct rkisp1_cif_isp_dpf_g_flt - Green filter Configuration
//
// @spatial_coeff: Spatial weights
// @gr_enable: enable filter processing for green pixels in green/red lines
// @gb_enable: enable filter processing for green pixels in green/blue lines
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpf_g_flt {
    pub spatial_coeff: [__u8; RKISP1_CIF_ISP_DPF_MAX_SPATIAL_COEFFS],
    pub gr_enable: __u8,
    pub gb_enable: __u8,
}

//
// struct rkisp1_cif_isp_dpf_gain - Noise function Configuration
//
// @mode: dpf gain usage  (from enum rkisp1_cif_isp_dpf_gain_usage)
// @nf_r_gain: Noise function Gain that replaces the AWB gain for red pixels
// @nf_b_gain: Noise function Gain that replaces the AWB gain for blue pixels
// @nf_gr_gain: Noise function Gain that replaces the AWB gain
// for green pixels in a red line
// @nf_gb_gain: Noise function Gain that replaces the AWB gain
// for green pixels in a blue line
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpf_gain {
    pub mode: __u32,
    pub nf_r_gain: __u16,
    pub nf_b_gain: __u16,
    pub nf_gr_gain: __u16,
    pub nf_gb_gain: __u16,
}

//
// struct rkisp1_cif_isp_dpf_config - Configuration used by De-noising pre-filter
//
// @gain: noise function gain
// @g_flt: green filter config
// @rb_flt: red blue filter config
// @nll: noise level lookup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpf_config {
    pub gain: rkisp1_cif_isp_dpf_gain,
    pub g_flt: rkisp1_cif_isp_dpf_g_flt,
    pub rb_flt: rkisp1_cif_isp_dpf_rb_flt,
    pub nll: rkisp1_cif_isp_dpf_nll,
}

//
// struct rkisp1_cif_isp_dpf_strength_config - strength of the filter
//
// @r: filter strength of the RED filter
// @g: filter strength of the GREEN filter
// @b: filter strength of the BLUE filter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_dpf_strength_config {
    pub r: __u8,
    pub g: __u8,
    pub b: __u8,
}

//
// struct rkisp1_cif_isp_isp_other_cfg - Parameters for some blocks in rockchip isp1
//
// @dpcc_config: Defect Pixel Cluster Correction config
// @bls_config: Black Level Subtraction config
// @sdg_config: sensor degamma config
// @lsc_config: Lens Shade config
// @awb_gain_config: Auto White balance gain config
// @flt_config: filter config
// @bdm_config: demosaic config
// @ctk_config: cross talk config
// @goc_config: gamma out config
// @bls_config: black level subtraction config
// @dpf_config: De-noising pre-filter config
// @dpf_strength_config: dpf strength config
// @cproc_config: color process config
// @ie_config: image effects config
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_isp_other_cfg {
    pub dpcc_config: rkisp1_cif_isp_dpcc_config,
    pub bls_config: rkisp1_cif_isp_bls_config,
    pub sdg_config: rkisp1_cif_isp_sdg_config,
    pub lsc_config: rkisp1_cif_isp_lsc_config,
    pub awb_gain_config: rkisp1_cif_isp_awb_gain_config,
    pub flt_config: rkisp1_cif_isp_flt_config,
    pub bdm_config: rkisp1_cif_isp_bdm_config,
    pub ctk_config: rkisp1_cif_isp_ctk_config,
    pub goc_config: rkisp1_cif_isp_goc_config,
    pub dpf_config: rkisp1_cif_isp_dpf_config,
    pub dpf_strength_config: rkisp1_cif_isp_dpf_strength_config,
    pub cproc_config: rkisp1_cif_isp_cproc_config,
    pub ie_config: rkisp1_cif_isp_ie_config,
}

//
// struct rkisp1_cif_isp_isp_meas_cfg - Rockchip ISP1 Measure Parameters
//
// @awb_meas_config: auto white balance config
// @hst_config: histogram config
// @aec_config: auto exposure config
// @afc_config: auto focus config
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_isp_meas_cfg {
    pub awb_meas_config: rkisp1_cif_isp_awb_meas_config,
    pub hst_config: rkisp1_cif_isp_hst_config,
    pub aec_config: rkisp1_cif_isp_aec_config,
    pub afc_config: rkisp1_cif_isp_afc_config,
}

//
// struct rkisp1_params_cfg - Rockchip ISP1 Input Parameters Meta Data
//
// @module_en_update: mask the enable bits of which module should be updated
// @module_ens: mask the enable value of each module, only update the module
// which correspond bit was set in module_en_update
// @module_cfg_update: mask the config bits of which module should be updated
// @meas: measurement config
// @others: other config
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_params_cfg {
    pub module_en_update: __u32,
    pub module_ens: __u32,
    pub module_cfg_update: __u32,
    pub meas: rkisp1_cif_isp_isp_meas_cfg,
    pub others: rkisp1_cif_isp_isp_other_cfg,
}

//
// struct rkisp1_cif_isp_compand_bls_config - Rockchip ISP1 Companding parameters (BLS)
// @r: Fixed subtraction value for Bayer pattern R
// @gr: Fixed subtraction value for Bayer pattern Gr
// @gb: Fixed subtraction value for Bayer pattern Gb
// @b: Fixed subtraction value for Bayer pattern B
//
// The values will be subtracted from the sensor values. Note that unlike the
// dedicated BLS block, the BLS values in the compander are 20-bit unsigned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_compand_bls_config {
    pub r: __u32,
    pub gr: __u32,
    pub gb: __u32,
    pub b: __u32,
}

//
// struct rkisp1_cif_isp_compand_curve_config - Rockchip ISP1 Companding
// parameters (expand and compression curves)
// @px: Compand curve x-values. Each value stores the distance from the
// previous x-value, expressed as log2 of the distance on 5 bits.
// @x: Compand curve x-values. The functionality of these parameters are
// unknown due to do a lack of hardware documentation, but these are left
// here for future compatibility purposes.
// @y: Compand curve y-values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_compand_curve_config {
    pub px: [__u8; RKISP1_CIF_ISP_COMPAND_NUM_POINTS],
    pub x: [__u32; RKISP1_CIF_ISP_COMPAND_NUM_POINTS],
    pub y: [__u32; RKISP1_CIF_ISP_COMPAND_NUM_POINTS],
}

//
// struct rkisp1_cif_isp_wdr_tone_curve - Tone mapping curve definition for WDR.
//
// @dY: the dYn increments for horizontal (input) axis of the tone curve.
// each 3-bit dY value represents an increment of 2**(value+3).
// dY[0] bits 0:2 is increment dY1, bit 3 unused
// dY[0] bits 4:6 is increment dY2, bit 7 unused
// ...
// dY[0] bits 28:30 is increment dY8, bit 31 unused
// ... and so on till dY[3] bits 28:30 is increment dY32, bit 31 unused.
// @ym: the Ym values for the vertical (output) axis of the tone curve.
// each value is 13 bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_wdr_tone_curve {
    pub dY: [__u32; RKISP1_CIF_ISP_WDR_CURVE_NUM_DY_REGS],
    pub ym: [__u16; RKISP1_CIF_ISP_WDR_CURVE_NUM_COEFF],
}

//
// struct rkisp1_cif_isp_wdr_iref_config - Illumination reference config for WDR.
//
// Use illumination reference value as described below, instead of only the
// luminance (Y) value for tone mapping and gain calculations:
// IRef = (rgb_factor * RGBMax_tr + (8 - rgb_factor) * Y)/8
//
// @rgb_factor: defines how much influence the RGBmax approach has in
// comparison to Y (valid values are 0..8).
// @use_y9_8: use Y*9/8 for maximum value calculation along with the
// default of R, G, B for noise reduction.
// @use_rgb7_8: decrease RGBMax by 7/8 for noise reduction.
// @disable_transient: disable transient calculation between Y and RGBY_max.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_wdr_iref_config {
    pub rgb_factor: __u8,
    pub use_y9_8: __u8,
    pub use_rgb7_8: __u8,
    pub disable_transient: __u8,
}

//
// struct rkisp1_cif_isp_wdr_config - Configuration for wide dynamic range.
//
// @tone_curve: tone mapping curve.
// @iref_config: illumination reference configuration. (when use_iref is true)
// @rgb_offset: RGB offset value for RGB operation mode. (12 bits)
// @luma_offset: luminance offset value for RGB operation mode. (12 bits)
// @dmin_thresh: lower threshold for deltaMin value. (12 bits)
// @dmin_strength: strength factor for deltaMin. (valid range is 0x00..0x10)
// @use_rgb_colorspace: use RGB instead of luminance/chrominance colorspace.
// @bypass_chroma_mapping: disable chrominance mapping (only valid if
// use_rgb_colorspace = 0)
// @use_iref: use illumination reference instead of Y for tone mapping
// and gain calculations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_wdr_config {
    pub tone_curve: rkisp1_cif_isp_wdr_tone_curve,
    pub iref_config: rkisp1_cif_isp_wdr_iref_config,
    pub rgb_offset: __u16,
    pub luma_offset: __u16,
    pub dmin_thresh: __u16,
    pub dmin_strength: __u8,
    pub use_rgb_colorspace: __u8,
    pub bypass_chroma_mapping: __u8,
    pub use_iref: __u8,
}

//
// enum rkisp1_cif_isp_cac_h_clip_mode - horizontal clipping mode
//
// @RKISP1_CIF_ISP_CAC_H_CLIP_MODE_4PX: +/- 4 pixels
// @RKISP1_CIF_ISP_CAC_H_CLIP_MODE_4_5PX: +/- 4/5 pixels depending on bayer position
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_cac_h_clip_mode {
    RKISP1_CIF_ISP_CAC_H_CLIP_MODE_4PX = 0,
    RKISP1_CIF_ISP_CAC_H_CLIP_MODE_4_5PX = 1,
}

//
// enum rkisp1_cif_isp_cac_v_clip_mode - vertical clipping mode
//
// @RKISP1_CIF_ISP_CAC_V_CLIP_MODE_2PX: +/- 2 pixels
// @RKISP1_CIF_ISP_CAC_V_CLIP_MODE_3PX: +/- 3 pixels
// @RKISP1_CIF_ISP_CAC_V_CLIP_MODE_3_4PX: +/- 3/4 pixels depending on bayer position
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_cif_isp_cac_v_clip_mode {
    RKISP1_CIF_ISP_CAC_V_CLIP_MODE_2PX = 0,
    RKISP1_CIF_ISP_CAC_V_CLIP_MODE_3PX = 1,
    RKISP1_CIF_ISP_CAC_V_CLIP_MODE_3_4PX = 2,
}

//
// struct rkisp1_cif_isp_cac_config - chromatic aberration correction configuration
//
// The correction is carried out by shifting the red and blue pixels relative
// to the green ones, depending on the distance from the optical center:
//
// @h_count_start: horizontal coordinate of the optical center (13-bit unsigned integer; [1,8191])
// @v_count_start: vertical coordinate of the optical center (13-bit unsigned integer; [1,8191])
//
// For each pixel, the x/y distances from the optical center are calculated and
// then transformed into the [0,255] range based on the following formula:
//
// (((d << 4) >> ns) * nf) >> 5
//
// where `d` is the distance, `ns` and `nf` are the normalization parameters:
//
// @x_nf: horizontal normalization scale parameter (5-bit unsigned integer; [0,31])
// @x_ns: horizontal normalization shift parameter (4-bit unsigned integer; [0,15])
//
// @y_nf: vertical normalization scale parameter (5-bit unsigned integer; [0,31])
// @y_ns: vertical normalization shift parameter (4-bit unsigned integer; [0,15])
//
// These parameters should be chosen based on the image resolution, the position
// of the optical center, and the shape of pixels, so that no normalized distance
// is larger than 255. If the pixels have square shape, the two sets of parameters
// should be equal.
//
// The actual amount of correction is calculated with a third degree polynomial:
//
// c[0] * r + c[1] * r^2 + c[2] * r^3
//
// where `c` is the set of coefficients for the given color, and `r` is distance:
//
// @red: red coefficients (5.4 two's complement; [-16,15.9375])
// @blue: blue coefficients (5.4 two's complement; [-16,15.9375])
//
// Finally, the amount is clipped as requested:
//
// @h_clip_mode: maximum horizontal shift (from enum rkisp1_cif_isp_cac_h_clip_mode)
// @v_clip_mode: maximum vertical shift (from enum rkisp1_cif_isp_cac_v_clip_mode)
//
// A positive result will shift away from the optical center, while a negative
// one will shift towards the optical center. In the latter case, the pixel
// values at the edges are duplicated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_cac_config {
    pub h_clip_mode: __u8,
    pub v_clip_mode: __u8,
    pub h_count_start: __u16,
    pub v_count_start: __u16,
    pub red: [__u16; 3],
    pub blue: [__u16; 3],
    pub x_nf: __u8,
    pub x_ns: __u8,
    pub y_nf: __u8,
    pub y_ns: __u8,
}

// ---------- PART2: Measurement Statistics ------------
//
// struct rkisp1_cif_isp_awb_meas - AWB measured values
//
// @cnt: White pixel count, number of "white pixels" found during last
// measurement
// @mean_y_or_g: Mean value of Y within window and frames,
// Green if RGB is selected.
// @mean_cb_or_b: Mean value of Cb within window and frames,
// Blue if RGB is selected.
// @mean_cr_or_r: Mean value of Cr within window and frames,
// Red if RGB is selected.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_awb_meas {
    pub cnt: __u32,
    pub mean_y_or_g: __u8,
    pub mean_cb_or_b: __u8,
    pub mean_cr_or_r: __u8,
}

//
// struct rkisp1_cif_isp_awb_stat - statistics automatic white balance data
//
// @awb_mean: Mean measured data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_awb_stat {
    pub awb_mean: [rkisp1_cif_isp_awb_meas; RKISP1_CIF_ISP_AWB_MAX_GRID],
}

//
// struct rkisp1_cif_isp_bls_meas_val - BLS measured values
//
// @meas_r: Mean measured value for Bayer pattern R
// @meas_gr: Mean measured value for Bayer pattern Gr
// @meas_gb: Mean measured value for Bayer pattern Gb
// @meas_b: Mean measured value for Bayer pattern B
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_bls_meas_val {
    pub meas_r: __u16,
    pub meas_gr: __u16,
    pub meas_gb: __u16,
    pub meas_b: __u16,
}

//
// struct rkisp1_cif_isp_ae_stat - statistics auto exposure data
//
// @exp_mean: Mean luminance value of block xx
// @bls_val:  BLS measured values
//
// The number of entries of @exp_mean depends on the hardware revision
// as is reported by the hw_revision field of the struct media_device_info
// that is returned by ioctl MEDIA_IOC_DEVICE_INFO.
//
// V10 has RKISP1_CIF_ISP_AE_MEAN_MAX_V10 entries, V12 has
// RKISP1_CIF_ISP_AE_MEAN_MAX_V12 entries. RKISP1_CIF_ISP_AE_MEAN_MAX is equal
// to the maximum of the two.
//
// Image is divided into 5x5 blocks on V10 and 9x9 blocks on V12.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_ae_stat {
    pub exp_mean: [__u8; RKISP1_CIF_ISP_AE_MEAN_MAX],
    pub bls_val: rkisp1_cif_isp_bls_meas_val,
}

//
// struct rkisp1_cif_isp_af_meas_val - AF measured values
//
// @sum: sharpness value
// @lum: luminance value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_af_meas_val {
    pub sum: __u32,
    pub lum: __u32,
}

//
// struct rkisp1_cif_isp_af_stat - statistics auto focus data
//
// @window: AF measured value of window x
//
// The module measures the sharpness in 3 windows of selectable size via
// register settings(ISP_AFM_*_A/B/C)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_af_stat {
    pub window: [rkisp1_cif_isp_af_meas_val; RKISP1_CIF_ISP_AFM_MAX_WINDOWS],
}

//
// struct rkisp1_cif_isp_hist_stat - statistics histogram data
//
// @hist_bins: measured bin counters. Each bin is a 20 bits unsigned fixed point
// type. Bits 0-4 are the fractional part and bits 5-19 are the
// integer part.
//
// The window of the measurements area is divided to 5x5 sub-windows for
// V10 and to 9x9 sub-windows for V12. The histogram is then computed for each
// sub-window independently and the final result is a weighted average of the
// histogram measurements on all sub-windows. The window of the measurements
// area and the weight of each sub-window are configurable using
// struct @rkisp1_cif_isp_hst_config.
//
// The histogram contains 16 bins in V10 and 32 bins in V12.
//
// The number of entries of @hist_bins depends on the hardware revision
// as is reported by the hw_revision field of the struct media_device_info
// that is returned by ioctl MEDIA_IOC_DEVICE_INFO.
//
// V10 has RKISP1_CIF_ISP_HIST_BIN_N_MAX_V10 entries, V12 has
// RKISP1_CIF_ISP_HIST_BIN_N_MAX_V12 entries. RKISP1_CIF_ISP_HIST_BIN_N_MAX is
// equal to the maximum of the two.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_hist_stat {
    pub hist_bins: [__u32; RKISP1_CIF_ISP_HIST_BIN_N_MAX],
}

//
// struct rkisp1_cif_isp_stat - Rockchip ISP1 Statistics Data
//
// @awb: statistics data for automatic white balance
// @ae: statistics data for auto exposure
// @af: statistics data for auto focus
// @hist: statistics histogram data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_cif_isp_stat {
    pub awb: rkisp1_cif_isp_awb_stat,
    pub ae: rkisp1_cif_isp_ae_stat,
    pub af: rkisp1_cif_isp_af_stat,
    pub hist: rkisp1_cif_isp_hist_stat,
}

//
// struct rkisp1_stat_buffer - Rockchip ISP1 Statistics Meta Data
//
// @meas_type: measurement types (RKISP1_CIF_ISP_STAT_* definitions)
// @frame_id: frame ID for sync
// @params: statistics data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_stat_buffer {
    pub meas_type: __u32,
    pub frame_id: __u32,
    pub params: rkisp1_cif_isp_stat,
}

// ---------- PART3: Extensible Configuration Parameters  ------------
//
// enum rkisp1_ext_params_block_type - RkISP1 extensible params block type
//
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_BLS: Black level subtraction
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_DPCC: Defect pixel cluster correction
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_SDG: Sensor de-gamma
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_AWB_GAIN: Auto white balance gains
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_FLT: ISP filtering
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_BDM: Bayer de-mosaic
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_CTK: Cross-talk correction
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_GOC: Gamma out correction
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_DPF: De-noise pre-filter
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_DPF_STRENGTH: De-noise pre-filter strength
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_CPROC: Color processing
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_IE: Image effects
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_LSC: Lens shading correction
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_AWB_MEAS: Auto white balance statistics
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_HST_MEAS: Histogram statistics
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_AEC_MEAS: Auto exposure statistics
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_AFC_MEAS: Auto-focus statistics
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_BLS: BLS in the compand block
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_EXPAND: Companding expand curve
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_COMPRESS: Companding compress curve
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_WDR: Wide dynamic range
// @RKISP1_EXT_PARAMS_BLOCK_TYPE_CAC: Chromatic aberration correction
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_ext_params_block_type {
    RKISP1_EXT_PARAMS_BLOCK_TYPE_BLS,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_DPCC,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_SDG,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_AWB_GAIN,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_FLT,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_BDM,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_CTK,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_GOC,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_DPF,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_DPF_STRENGTH,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_CPROC,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_IE,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_LSC,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_AWB_MEAS,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_HST_MEAS,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_AEC_MEAS,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_AFC_MEAS,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_BLS,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_EXPAND,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_COMPRESS,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_WDR,
    RKISP1_EXT_PARAMS_BLOCK_TYPE_CAC,
}

// For backward compatibility

// A bitmask of parameters blocks supported on the current hardware.

//
// rkisp1_ext_params_block_header - RkISP1 extensible parameters block header
//
// This structure represents the common part of all the ISP configuration
// blocks and is identical to :c:type:`v4l2_isp_params_block_header`.
//
// The type field is one of the values enumerated by
// :c:type:`rkisp1_ext_params_block_type` and specifies how the data should be
// interpreted by the driver.
//
// The flags field is a bitmask of per-block flags RKISP1_EXT_PARAMS_FL_*.
//

//
// struct rkisp1_ext_params_bls_config - RkISP1 extensible params BLS config
//
// RkISP1 extensible parameters Black Level Subtraction configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_BLS`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Black Level Subtraction configuration, see
// :c:type:`rkisp1_cif_isp_bls_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_bls_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_bls_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_dpcc_config - RkISP1 extensible params DPCC config
//
// RkISP1 extensible parameters Defective Pixel Cluster Correction configuration
// block. Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_DPCC`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Defective Pixel Cluster Correction configuration, see
// :c:type:`rkisp1_cif_isp_dpcc_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_dpcc_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_dpcc_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_sdg_config - RkISP1 extensible params SDG config
//
// RkISP1 extensible parameters Sensor Degamma configuration block. Identified
// by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_SDG`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Sensor Degamma configuration, see
// :c:type:`rkisp1_cif_isp_sdg_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_sdg_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_sdg_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_lsc_config - RkISP1 extensible params LSC config
//
// RkISP1 extensible parameters Lens Shading Correction configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_LSC`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Lens Shading Correction configuration, see
// :c:type:`rkisp1_cif_isp_lsc_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_lsc_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_lsc_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_awb_gain_config - RkISP1 extensible params AWB
// gain config
//
// RkISP1 extensible parameters Auto-White Balance Gains configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_AWB_GAIN`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Auto-White Balance Gains configuration, see
// :c:type:`rkisp1_cif_isp_awb_gain_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_awb_gain_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_awb_gain_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_flt_config - RkISP1 extensible params FLT config
//
// RkISP1 extensible parameters Filter configuration block. Identified by
// :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_FLT`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Filter configuration, see :c:type:`rkisp1_cif_isp_flt_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_flt_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_flt_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_bdm_config - RkISP1 extensible params BDM config
//
// RkISP1 extensible parameters Demosaicing configuration block. Identified by
// :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_BDM`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Demosaicing configuration, see :c:type:`rkisp1_cif_isp_bdm_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_bdm_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_bdm_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_ctk_config - RkISP1 extensible params CTK config
//
// RkISP1 extensible parameters Cross-Talk configuration block. Identified by
// :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_CTK`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Cross-Talk configuration, see :c:type:`rkisp1_cif_isp_ctk_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_ctk_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_ctk_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_goc_config - RkISP1 extensible params GOC config
//
// RkISP1 extensible parameters Gamma-Out configuration block. Identified by
// :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_GOC`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Gamma-Out configuration, see :c:type:`rkisp1_cif_isp_goc_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_goc_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_goc_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_dpf_config - RkISP1 extensible params DPF config
//
// RkISP1 extensible parameters De-noise Pre-Filter configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_DPF`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: De-noise Pre-Filter configuration, see
// :c:type:`rkisp1_cif_isp_dpf_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_dpf_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_dpf_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_dpf_strength_config - RkISP1 extensible params DPF
// strength config
//
// RkISP1 extensible parameters De-noise Pre-Filter strength configuration
// block. Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_DPF_STRENGTH`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: De-noise Pre-Filter strength configuration, see
// :c:type:`rkisp1_cif_isp_dpf_strength_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_dpf_strength_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_dpf_strength_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_cproc_config - RkISP1 extensible params CPROC config
//
// RkISP1 extensible parameters Color Processing configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_CPROC`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Color processing configuration, see
// :c:type:`rkisp1_cif_isp_cproc_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_cproc_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_cproc_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_ie_config - RkISP1 extensible params IE config
//
// RkISP1 extensible parameters Image Effect configuration block. Identified by
// :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_IE`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Image Effect configuration, see :c:type:`rkisp1_cif_isp_ie_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_ie_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_ie_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_awb_meas_config - RkISP1 extensible params AWB
// Meas config
//
// RkISP1 extensible parameters Auto-White Balance Measurement configuration
// block. Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_AWB_MEAS`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Auto-White Balance measure configuration, see
// :c:type:`rkisp1_cif_isp_awb_meas_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_awb_meas_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_awb_meas_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_hst_config - RkISP1 extensible params Histogram config
//
// RkISP1 extensible parameters Histogram statistics configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_HST_MEAS`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Histogram statistics configuration, see
// :c:type:`rkisp1_cif_isp_hst_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_hst_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_hst_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_aec_config - RkISP1 extensible params AEC config
//
// RkISP1 extensible parameters Auto-Exposure statistics configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_AEC_MEAS`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Auto-Exposure statistics configuration, see
// :c:type:`rkisp1_cif_isp_aec_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_aec_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_aec_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_afc_config - RkISP1 extensible params AFC config
//
// RkISP1 extensible parameters Auto-Focus statistics configuration block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_AFC_MEAS`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Auto-Focus statistics configuration, see
// :c:type:`rkisp1_cif_isp_afc_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_afc_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_afc_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_compand_bls_config - RkISP1 extensible params
// Compand BLS config
//
// RkISP1 extensible parameters Companding configuration block (black level
// subtraction). Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_BLS`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Companding BLS configuration, see
// :c:type:`rkisp1_cif_isp_compand_bls_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_compand_bls_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_compand_bls_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_compand_curve_config - RkISP1 extensible params
// Compand curve config
//
// RkISP1 extensible parameters Companding configuration block (expand and
// compression curves). Identified by
// :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_EXPAND` or
// :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_COMPAND_COMPRESS`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: Companding curve configuration, see
// :c:type:`rkisp1_cif_isp_compand_curve_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_compand_curve_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_compand_curve_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_wdr_config - RkISP1 extensible params
// Wide dynamic range config
//
// RkISP1 extensible parameters WDR block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_WDR`
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: WDR configuration, see
// :c:type:`rkisp1_cif_isp_wdr_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_wdr_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_wdr_config,
    pub __attribute__((aligned(8))): },
//
// struct rkisp1_ext_params_cac_config - RkISP1 extensible params CAC config
//
// RkISP1 extensible parameters CAC block.
// Identified by :c:type:`RKISP1_EXT_PARAMS_BLOCK_TYPE_CAC`.
//
// @header: The RkISP1 extensible parameters header, see
// :c:type:`rkisp1_ext_params_block_header`
// @config: CAC configuration, see
// :c:type:`rkisp1_cif_isp_cac_config`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_cac_config {
    pub header: rkisp1_ext_params_block_header,
    pub config: rkisp1_cif_isp_cac_config,
    pub __attribute__((aligned(8))): },
//
// The rkisp1_ext_params_compand_curve_config structure is counted twice as it
// is used for both the COMPAND_EXPAND and COMPAND_COMPRESS block types.
//

//
// enum rkisp1_ext_param_buffer_version - RkISP1 extensible parameters version
//
// @RKISP1_EXT_PARAM_BUFFER_V1: First version of RkISP1 extensible parameters
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_ext_param_buffer_version {
    RKISP1_EXT_PARAM_BUFFER_V1 = V4L2_ISP_PARAMS_VERSION_V1,
}

//
// struct rkisp1_ext_params_cfg - RkISP1 extensible parameters configuration
//
// This is the driver-specific implementation of
// :c:type:`v4l2_isp_params_buffer`.
//
// Currently the single RKISP1_EXT_PARAM_BUFFER_V1 version is supported.
// When a new format version will be added, a mechanism for userspace to query
// the supported format versions will be implemented in the form of a read-only
// V4L2 control. If such control is not available, userspace should assume only
// RKISP1_EXT_PARAM_BUFFER_V1 is supported by the driver.
//
// The read-only V4L2 control ``RKISP1_CID_SUPPORTED_PARAMS_BLOCKS`` can be used
// to query the blocks supported by the device. It contains a bitmask where each
// bit represents the availability of the corresponding entry from the
// :c:type:`rkisp1_ext_params_block_type` enum. The current and default values
// of the control represents the blocks supported by the device instance, while
// the maximum value represents the blocks supported by the kernel driver,
// independently of the device instance.
//
// The expected memory layout of the parameters buffer is::
//
// +-------------------- struct rkisp1_ext_params_cfg -------------------+
// | version = RKISP1_EXT_PARAM_BUFFER_V1;                               |
// | data_size = sizeof(struct rkisp1_ext_params_bls_config)             |
// |           + sizeof(struct rkisp1_ext_params_dpcc_config);           |
// | +------------------------- data  ---------------------------------+ |
// | | +------------- struct rkisp1_ext_params_bls_config -----------+ | |
// | | | +-------- struct rkisp1_ext_params_block_header  ---------+ | | |
// | | | | type = RKISP1_EXT_PARAMS_BLOCK_TYPE_BLS;                | | | |
// | | | | flags = RKISP1_EXT_PARAMS_FL_BLOCK_ENABLE;              | | | |
// | | | | size = sizeof(struct rkisp1_ext_params_bls_config);     | | | |
// | | | +---------------------------------------------------------+ | | |
// | | | +---------- struct rkisp1_cif_isp_bls_config -------------+ | | |
// | | | | enable_auto = 0;                                        | | | |
// | | | | fixed_val.r = 256;                                      | | | |
// | | | | fixed_val.gr = 256;                                     | | | |
// | | | | fixed_val.gb = 256;                                     | | | |
// | | | | fixed_val.b = 256;                                      | | | |
// | | | +---------------------------------------------------------+ | | |
// | | +------------ struct rkisp1_ext_params_dpcc_config -----------+ | |
// | | | +-------- struct rkisp1_ext_params_block_header  ---------+ | | |
// | | | | type = RKISP1_EXT_PARAMS_BLOCK_TYPE_DPCC;               | | | |
// | | | | flags = RKISP1_EXT_PARAMS_FL_BLOCK_ENABLE;              | | | |
// | | | | size = sizeof(struct rkisp1_ext_params_dpcc_config);    | | | |
// | | | +---------------------------------------------------------+ | | |
// | | | +---------- struct rkisp1_cif_isp_dpcc_config ------------+ | | |
// | | | | mode = RKISP1_CIF_ISP_DPCC_MODE_STAGE1_ENABLE;          | | | |
// | | | | output_mode =                                           | | | |
// | | | |   RKISP1_CIF_ISP_DPCC_OUTPUT_MODE_STAGE1_INCL_G_CENTER; | | | |
// | | | | set_use = ... ;                                         | | | |
// | | | | ...  = ... ;                                            | | | |
// | | | +---------------------------------------------------------+ | | |
// | | +-------------------------------------------------------------+ | |
// | +-----------------------------------------------------------------+ |
// +---------------------------------------------------------------------+
//
// @version: The RkISP1 extensible parameters buffer version, see
// :c:type:`rkisp1_ext_param_buffer_version`
// @data_size: The RkISP1 configuration data effective size, excluding this
// header
// @data: The RkISP1 extensible configuration data blocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_ext_params_cfg {
    pub version: __u32,
    pub data_size: __u32,
    pub data: [__u8; RKISP1_EXT_PARAMS_MAX_SIZE],
}

// Make sure the header is type-convertible to the generic v4l2 params one

