//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_sspp.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
//

pub const DPU_SSPP_MAX_PITCH_SIZE: c_uint = 0xffff;
//
// Flags
//

//
// Component indices
//
// enum dpu_sspp_multirect_index - multirect mode
// @DPU_SSPP_RECT_SOLO: multirect disabled
// @DPU_SSPP_RECT_0: rect0 of a multirect pipe
// @DPU_SSPP_RECT_1: rect1 of a multirect pipe
//
// Note: HW supports multirect with either RECT0 or
// RECT1. Considering no benefit of such configs over
// SOLO mode and to keep the plane management simple,
// we dont support single rect multirect configs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_sspp_multirect_index {
    DPU_SSPP_RECT_SOLO = 0,
    DPU_SSPP_RECT_0,
    DPU_SSPP_RECT_1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_sspp_multirect_mode {
    DPU_SSPP_MULTIRECT_NONE = 0,
    DPU_SSPP_MULTIRECT_PARALLEL,
    DPU_SSPP_MULTIRECT_TIME_MX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_filter {
    DPU_SCALE_FILTER_NEAREST = 0,
    DPU_SCALE_FILTER_BIL,
    DPU_SCALE_FILTER_PCMN,
    DPU_SCALE_FILTER_CA,
    DPU_SCALE_FILTER_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_filter_alpa {
    DPU_SCALE_ALPHA_PIXEL_REP,
    DPU_SCALE_ALPHA_BIL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_filter_yuv {
    DPU_SCALE_2D_4X4,
    DPU_SCALE_2D_CIR,
    DPU_SCALE_1D_SEP,
    DPU_SCALE_BIL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_sharp_cfg {
    pub strength: u32,
    pub edge_thr: u32,
    pub smooth_thr: u32,
    pub noise_thr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_pixel_ext {
// scaling factors are enabled for this input layer
    pub enable_pxl_ext: u8,
    pub init_phase_x: [c_int; DPU_MAX_PLANES],
    pub phase_step_x: [c_int; DPU_MAX_PLANES],
    pub init_phase_y: [c_int; DPU_MAX_PLANES],
    pub phase_step_y: [c_int; DPU_MAX_PLANES],
//
// Number of pixels extension in left, right, top and bottom direction
// for all color components. This pixel value for each color component
// should be sum of fetch + repeat pixels.
//
    pub num_ext_pxls_left: [c_int; DPU_MAX_PLANES],
    pub num_ext_pxls_right: [c_int; DPU_MAX_PLANES],
    pub num_ext_pxls_top: [c_int; DPU_MAX_PLANES],
    pub num_ext_pxls_btm: [c_int; DPU_MAX_PLANES],
//
// Number of pixels needs to be overfetched in left, right, top and
// bottom directions from source image for scaling.
//
    pub left_ftch: [c_int; DPU_MAX_PLANES],
    pub right_ftch: [c_int; DPU_MAX_PLANES],
    pub top_ftch: [c_int; DPU_MAX_PLANES],
    pub btm_ftch: [c_int; DPU_MAX_PLANES],
//
// Number of pixels needs to be repeated in left, right, top and
// bottom directions for scaling.
//
    pub left_rpt: [c_int; DPU_MAX_PLANES],
    pub right_rpt: [c_int; DPU_MAX_PLANES],
    pub top_rpt: [c_int; DPU_MAX_PLANES],
    pub btm_rpt: [c_int; DPU_MAX_PLANES],
    pub roi_w: [u32; DPU_MAX_PLANES],
    pub roi_h: [u32; DPU_MAX_PLANES],
//
// Filter type to be used for scaling in horizontal and vertical
// directions
//
    pub horz_filter: [dpu_hw_filter; DPU_MAX_PLANES],
    pub vert_filter: [dpu_hw_filter; DPU_MAX_PLANES],
}

//
// struct dpu_sw_pipe_cfg : software pipe configuration
// @src_rect:  src ROI, caller takes into account the different operations
// such as decimation, flip etc to program this field
// @dst_rect: destination ROI.
// @rotation: simplified drm rotation hint
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_sw_pipe_cfg {
    pub src_rect: drm_rect,
    pub dst_rect: drm_rect,
    pub rotation: c_uint,
}

//
// struct dpu_hw_pipe_ts_cfg - traffic shaper configuration
// @size: size to prefill in bytes, or zero to disable
// @time: time to prefill in usec, or zero to disable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_pipe_ts_cfg {
    pub size: u64,
    pub time: u64,
}

//
// struct dpu_sw_pipe - software pipe description
// @sspp:      backing SSPP pipe
// @multirect_index:     index of the rectangle of SSPP
// @multirect_mode:      parallel or time multiplex multirect mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_sw_pipe {
    pub sspp: *mut dpu_hw_sspp,
    pub multirect_index: dpu_sspp_multirect_index,
    pub multirect_mode: dpu_sspp_multirect_mode,
}

//
// struct dpu_hw_sspp_ops - interface to the SSPP Hw driver functions
// Caller must call the init function to get the pipe context for each pipe
// Assumption is these functions will be called after clocks are enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_sspp_ops {
//
// @setup_format: setup pixel format cropping rectangle, flip
// @pipe: Pointer to software pipe context
// @cfg: Pointer to pipe config structure
// @flags: Extra flags for format config
//
    pub flags): *const *const msm_format fmt, u32,
//
// @setup_rects: setup pipe ROI rectangles
// @pipe: Pointer to software pipe context
// @cfg: Pointer to pipe config structure
//
    pub cfg): *mut dpu_sw_pipe_cfg,
//
// @setup_pe: setup pipe pixel extension
// @ctx: Pointer to pipe context
// @pe_ext: Pointer to pixel ext settings
//
    pub pe_ext): *mut dpu_hw_pixel_ext,
//
// @setup_sourceaddress: setup pipe source addresses
// @pipe: Pointer to software pipe context
// @layout: format layout information for programming buffer to hardware
//
    pub layout): *mut dpu_hw_fmt_layout,
//
// @setup_csc: setup color space coversion
// @ctx: Pointer to pipe context
// @data: Pointer to config structure
//
    pub data): *const *const *const void (setup_csc)(struct dpu_hw_sspp ctx, struct dpu_csc_cfg,
//
// @setup_solidfill: enable/disable colorfill
// @pipe: Pointer to software pipe context
// @const_color: Fill color value
// @flags: Pipe flags
//
    pub color): *mut *mut *mut void (setup_solidfill)(struct dpu_sw_pipe pipe, u32,
//
// @setup_multirect: setup multirect configuration
// @pipe: Pointer to software pipe context
//
    pub pipe): *mut *mut void (setup_multirect)(struct dpu_sw_pipe,
//
// @setup_sharpening: setup sharpening
// @ctx: Pointer to pipe context
// @cfg: Pointer to config structure
//
    pub cfg): *mut dpu_hw_sharp_cfg,
//
// @setup_qos_lut: setup QoS LUTs
// @ctx: Pointer to pipe context
// @cfg: LUT configuration
//
    pub cfg): *mut dpu_hw_qos_cfg,
//
// @setup_qos_ctrl: setup QoS control
// @ctx: Pointer to pipe context
// @danger_safe_en: flags controlling enabling of danger/safe QoS/LUT
//
    pub danger_safe_en): bool,
//
// @setup_clk_force_ctrl: setup clock force control
// @ctx: Pointer to pipe context
// @enable: enable clock force if true
//
    pub enable): bool,
//
// @setup_histogram: setup histograms
// @ctx: Pointer to pipe context
// @cfg: Pointer to histogram configuration
//
    pub cfg): *mut c_void,
//
// @setup_scaler: setup scaler
// @scaler3_cfg: Pointer to scaler configuration
// @format: pixel format parameters
//
    pub format): *const msm_format,
//
// @setup_cdp: setup client driven prefetch
// @pipe: Pointer to software pipe context
// @fmt: format used by the sw pipe
// @enable: whether the CDP should be enabled for this pipe
//
    pub enable): bool,
}

//
// struct dpu_hw_sspp - pipe description
// @base: hardware block base structure
// @hw: block hardware details
// @ubwc: UBWC configuration data
// @idx: pipe index
// @cap: pointer to layer_cfg
// @mdss_ver: MDSS version info to use for feature checks
// @ops: pointer to operations possible for this pipe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_sspp {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
    pub ubwc: *const qcom_ubwc_cfg_data,
// Pipe
    pub idx: dpu_sspp,
    pub cap: *const dpu_sspp_cfg,
    pub mdss_ver: *const dpu_mdss_version,
// Ops
    pub ops: dpu_hw_sspp_ops,
}

// src and dest rect programming
// rectangle register programming
