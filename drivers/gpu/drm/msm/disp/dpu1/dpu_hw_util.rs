//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_util.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022-2023 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2015-2021, The Linux Foundation. All rights reserved.
//

pub const MISR_FRAME_COUNT: c_uint = 0x1;

pub const MDP_TICK_COUNT: c_int = 16;
pub const XO_CLK_RATE: c_int = 19200;
pub const MS_TICKS_IN_SEC: c_int = 1000;

//
// This is the common struct maintained by each sub block
// for mapping the register offsets in this block to the
// absoulute IO address
// @blk_addr:     hw block register mapped address
// @log_mask:     log mask for this block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_blk_reg_map {
    pub blk_addr: *mut void __iomem,
    pub log_mask: u32,
}

//
// struct dpu_hw_blk - opaque hardware block object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_blk {
// opaque
}

//
// struct dpu_hw_scaler3_de_cfg : QSEEDv3 detail enhancer configuration
// @enable:         detail enhancer enable/disable
// @sharpen_level1: sharpening strength for noise
// @sharpen_level2: sharpening strength for signal
// @ clip:          clip shift
// @ limit:         limit value
// @ thr_quiet:     quiet threshold
// @ thr_dieout:    dieout threshold
// @ thr_high:      low threshold
// @ thr_high:      high threshold
// @ prec_shift:    precision shift
// @ adjust_a:      A-coefficients for mapping curve
// @ adjust_b:      B-coefficients for mapping curve
// @ adjust_c:      C-coefficients for mapping curve
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_scaler3_de_cfg {
    pub enable: u32,
    pub sharpen_level1: i16,
    pub sharpen_level2: i16,
    pub clip: u16,
    pub limit: u16,
    pub thr_quiet: u16,
    pub thr_dieout: u16,
    pub thr_low: u16,
    pub thr_high: u16,
    pub prec_shift: u16,
    pub adjust_a: [i16; DPU_MAX_DE_CURVES],
    pub adjust_b: [i16; DPU_MAX_DE_CURVES],
    pub adjust_c: [i16; DPU_MAX_DE_CURVES],
}

//
// struct dpu_hw_scaler3_cfg : QSEEDv3 configuration
// @enable:        scaler enable
// @dir_en:        direction detection block enable
// @ init_phase_x: horizontal initial phase
// @ phase_step_x: horizontal phase step
// @ init_phase_y: vertical initial phase
// @ phase_step_y: vertical phase step
// @ preload_x:    horizontal preload value
// @ preload_y:    vertical preload value
// @ src_width:    source width
// @ src_height:   source height
// @ dst_width:    destination width
// @ dst_height:   destination height
// @ y_rgb_filter_cfg: y/rgb plane filter configuration
// @ uv_filter_cfg: uv plane filter configuration
// @ alpha_filter_cfg: alpha filter configuration
// @ blend_cfg:    blend coefficients configuration
// @ lut_flag:     scaler LUT update flags
// 0x1 swap LUT bank
// 0x2 update 2D filter LUT
// 0x4 update y circular filter LUT
// 0x8 update uv circular filter LUT
// 0x10 update y separable filter LUT
// 0x20 update uv separable filter LUT
// @ dir_lut_idx:  2D filter LUT index
// @ y_rgb_cir_lut_idx: y circular filter LUT index
// @ uv_cir_lut_idx: uv circular filter LUT index
// @ y_rgb_sep_lut_idx: y circular filter LUT index
// @ uv_sep_lut_idx: uv separable filter LUT index
// @ dir_lut:      pointer to 2D LUT
// @ cir_lut:      pointer to circular filter LUT
// @ sep_lut:      pointer to separable filter LUT
// @ de: detail enhancer configuration
// @ dir_weight:   Directional weight
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_scaler3_cfg {
    pub enable: u32,
    pub dir_en: u32,
    pub init_phase_x: [i32; DPU_MAX_PLANES],
    pub phase_step_x: [i32; DPU_MAX_PLANES],
    pub init_phase_y: [i32; DPU_MAX_PLANES],
    pub phase_step_y: [i32; DPU_MAX_PLANES],
    pub preload_x: [u32; DPU_MAX_PLANES],
    pub preload_y: [u32; DPU_MAX_PLANES],
    pub src_width: [u32; DPU_MAX_PLANES],
    pub src_height: [u32; DPU_MAX_PLANES],
    pub dst_width: u32,
    pub dst_height: u32,
    pub y_rgb_filter_cfg: u32,
    pub uv_filter_cfg: u32,
    pub alpha_filter_cfg: u32,
    pub blend_cfg: u32,
    pub lut_flag: u32,
    pub dir_lut_idx: u32,
    pub y_rgb_cir_lut_idx: u32,
    pub uv_cir_lut_idx: u32,
    pub y_rgb_sep_lut_idx: u32,
    pub uv_sep_lut_idx: u32,
    pub dir_lut: *mut u32,
    pub dir_len: usize,
    pub cir_lut: *mut u32,
    pub cir_len: usize,
    pub sep_lut: *mut u32,
    pub sep_len: usize,
//
// Detail enhancer settings
//
    pub de: dpu_hw_scaler3_de_cfg,
    pub dir_weight: u32,
}

//
// struct dpu_drm_pix_ext_v1 - version 1 of pixel ext structure
// @num_ext_pxls_lr: Number of total horizontal pixels
// @num_ext_pxls_tb: Number of total vertical lines
// @left_ftch:       Number of extra pixels to overfetch from left
// @right_ftch:      Number of extra pixels to overfetch from right
// @top_ftch:        Number of extra lines to overfetch from top
// @btm_ftch:        Number of extra lines to overfetch from bottom
// @left_rpt:        Number of extra pixels to repeat from left
// @right_rpt:       Number of extra pixels to repeat from right
// @top_rpt:         Number of extra lines to repeat from top
// @btm_rpt:         Number of extra lines to repeat from bottom
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_drm_pix_ext_v1 {
//
// Number of pixels ext in left, right, top and bottom direction
// for all color components.
//
    pub num_ext_pxls_lr: [i32; DPU_MAX_PLANES],
    pub num_ext_pxls_tb: [i32; DPU_MAX_PLANES],
//
// Number of pixels needs to be overfetched in left, right, top
// and bottom directions from source image for scaling.
//
    pub left_ftch: [i32; DPU_MAX_PLANES],
    pub right_ftch: [i32; DPU_MAX_PLANES],
    pub top_ftch: [i32; DPU_MAX_PLANES],
    pub btm_ftch: [i32; DPU_MAX_PLANES],
//
// Number of pixels needs to be repeated in left, right, top and
// bottom directions for scaling.
//
    pub left_rpt: [i32; DPU_MAX_PLANES],
    pub right_rpt: [i32; DPU_MAX_PLANES],
    pub top_rpt: [i32; DPU_MAX_PLANES],
    pub btm_rpt: [i32; DPU_MAX_PLANES],
}

//
// struct dpu_drm_de_v1 - version 1 of detail enhancer structure
// @enable:         Enables/disables detail enhancer
// @sharpen_level1: Sharpening strength for noise
// @sharpen_level2: Sharpening strength for context
// @clip:           Clip coefficient
// @limit:          Detail enhancer limit factor
// @thr_quiet:      Quite zone threshold
// @thr_dieout:     Die-out zone threshold
// @thr_low:        Linear zone left threshold
// @thr_high:       Linear zone right threshold
// @prec_shift:     Detail enhancer precision
// @adjust_a:       Mapping curves A coefficients
// @adjust_b:       Mapping curves B coefficients
// @adjust_c:       Mapping curves C coefficients
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_drm_de_v1 {
    pub enable: u32,
    pub sharpen_level1: i16,
    pub sharpen_level2: i16,
    pub clip: u16,
    pub limit: u16,
    pub thr_quiet: u16,
    pub thr_dieout: u16,
    pub thr_low: u16,
    pub thr_high: u16,
    pub prec_shift: u16,
    pub adjust_a: [i16; DPU_MAX_DE_CURVES],
    pub adjust_b: [i16; DPU_MAX_DE_CURVES],
    pub adjust_c: [i16; DPU_MAX_DE_CURVES],
}

//
// struct dpu_drm_scaler_v2 - version 2 of struct dpu_drm_scaler
// @enable:            Scaler enable
// @dir_en:            Detail enhancer enable
// @pe:                Pixel extension settings
// @horz_decimate:     Horizontal decimation factor
// @vert_decimate:     Vertical decimation factor
// @init_phase_x:      Initial scaler phase values for x
// @phase_step_x:      Phase step values for x
// @init_phase_y:      Initial scaler phase values for y
// @phase_step_y:      Phase step values for y
// @preload_x:         Horizontal preload value
// @preload_y:         Vertical preload value
// @src_width:         Source width
// @src_height:        Source height
// @dst_width:         Destination width
// @dst_height:        Destination height
// @y_rgb_filter_cfg:  Y/RGB plane filter configuration
// @uv_filter_cfg:     UV plane filter configuration
// @alpha_filter_cfg:  Alpha filter configuration
// @blend_cfg:         Selection of blend coefficients
// @lut_flag:          LUT configuration flags
// @dir_lut_idx:       2d 4x4 LUT index
// @y_rgb_cir_lut_idx: Y/RGB circular LUT index
// @uv_cir_lut_idx:    UV circular LUT index
// @y_rgb_sep_lut_idx: Y/RGB separable LUT index
// @uv_sep_lut_idx:    UV separable LUT index
// @de:                Detail enhancer settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_drm_scaler_v2 {
//
// General definitions
//
    pub enable: u32,
    pub dir_en: u32,
//
// Pix ext settings
//
    pub pe: dpu_drm_pix_ext_v1,
//
// Decimation settings
//
    pub horz_decimate: u32,
    pub vert_decimate: u32,
//
// Phase settings
//
    pub init_phase_x: [i32; DPU_MAX_PLANES],
    pub phase_step_x: [i32; DPU_MAX_PLANES],
    pub init_phase_y: [i32; DPU_MAX_PLANES],
    pub phase_step_y: [i32; DPU_MAX_PLANES],
    pub preload_x: [u32; DPU_MAX_PLANES],
    pub preload_y: [u32; DPU_MAX_PLANES],
    pub src_width: [u32; DPU_MAX_PLANES],
    pub src_height: [u32; DPU_MAX_PLANES],
    pub dst_width: u32,
    pub dst_height: u32,
    pub y_rgb_filter_cfg: u32,
    pub uv_filter_cfg: u32,
    pub alpha_filter_cfg: u32,
    pub blend_cfg: u32,
    pub lut_flag: u32,
    pub dir_lut_idx: u32,
// for Y(RGB) and UV planes
    pub y_rgb_cir_lut_idx: u32,
    pub uv_cir_lut_idx: u32,
    pub y_rgb_sep_lut_idx: u32,
    pub uv_sep_lut_idx: u32,
//
// Detail enhancer settings
//
    pub de: dpu_drm_de_v1,
}

//
// struct dpu_hw_qos_cfg: pipe QoS configuration
// @danger_lut: LUT for generate danger level based on fill level
// @safe_lut: LUT for generate safe level based on fill level
// @creq_lut: LUT for generate creq level based on fill level
// @creq_vblank: creq value generated to vbif during vertical blanking
// @danger_vblank: danger value generated during vertical blanking
// @vblank_en: enable creq_vblank and danger_vblank during vblank
// @danger_safe_en: enable danger safe generation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_qos_cfg {
    pub danger_lut: u32,
    pub safe_lut: u32,
    pub creq_lut: u64,
    pub danger_safe_en: bool,
}

extern "C" {
    pub fn dpu_reg_read(c: *mut dpu_hw_blk_reg_map, reg_off: u32) -> c_int;
}

