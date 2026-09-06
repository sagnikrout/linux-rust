//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_mdss.h
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
// Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
//

pub const DPU_NONE: c_int = 0;

pub const DPU_CSC_MATRIX_COEFF_SIZE: c_int = 9;

pub const DPU_CSC_CLAMP_SIZE: c_int = 6;

pub const DPU_CSC_BIAS_SIZE: c_int = 3;

pub const DPU_MAX_PLANES: c_int = 4;

pub const STAGES_PER_PLANE: c_int = 1;
pub const PIPES_PER_STAGE: c_int = 2;

pub const DPU_MAX_DE_CURVES: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_vsync_source {
    DPU_VSYNC_SOURCE_GPIO_0,
    DPU_VSYNC_SOURCE_GPIO_1,
    DPU_VSYNC_SOURCE_GPIO_2,
    DPU_VSYNC_SOURCE_INTF_0 = 3,
    DPU_VSYNC_SOURCE_INTF_1,
    DPU_VSYNC_SOURCE_INTF_2,
    DPU_VSYNC_SOURCE_INTF_3,
    DPU_VSYNC_SOURCE_WD_TIMER_4 = 11,
    DPU_VSYNC_SOURCE_WD_TIMER_3,
    DPU_VSYNC_SOURCE_WD_TIMER_2,
    DPU_VSYNC_SOURCE_WD_TIMER_1,
    DPU_VSYNC_SOURCE_WD_TIMER_0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_blk_type {
    DPU_HW_BLK_TOP = 0,
    DPU_HW_BLK_SSPP,
    DPU_HW_BLK_LM,
    DPU_HW_BLK_CTL,
    DPU_HW_BLK_PINGPONG,
    DPU_HW_BLK_DCWB_PINGPONG,
    DPU_HW_BLK_INTF,
    DPU_HW_BLK_WB,
    DPU_HW_BLK_DSPP,
    DPU_HW_BLK_MERGE_3D,
    DPU_HW_BLK_DSC,
    DPU_HW_BLK_CDM,
    DPU_HW_BLK_CWB,
    DPU_HW_BLK_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_sspp {
    SSPP_NONE,
    SSPP_VIG0,
    SSPP_VIG1,
    SSPP_VIG2,
    SSPP_VIG3,
    SSPP_RGB0,
    SSPP_RGB1,
    SSPP_RGB2,
    SSPP_RGB3,
    SSPP_DMA0,
    SSPP_DMA1,
    SSPP_DMA2,
    SSPP_DMA3,
    SSPP_DMA4,
    SSPP_DMA5,
    SSPP_CURSOR0,
    SSPP_CURSOR1,
    SSPP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_sspp_type {
    SSPP_TYPE_VIG,
    SSPP_TYPE_RGB,
    SSPP_TYPE_DMA,
    SSPP_TYPE_CURSOR,
    SSPP_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_lm {
    LM_0 = 1,
    LM_1,
    LM_2,
    LM_3,
    LM_4,
    LM_5,
    LM_6,
    LM_7,
    LM_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_stage {
    DPU_STAGE_BASE = 0,
    DPU_STAGE_0,
    DPU_STAGE_1,
    DPU_STAGE_2,
    DPU_STAGE_3,
    DPU_STAGE_4,
    DPU_STAGE_5,
    DPU_STAGE_6,
    DPU_STAGE_7,
    DPU_STAGE_8,
    DPU_STAGE_9,
    DPU_STAGE_10,
    DPU_STAGE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_dspp {
    DSPP_0 = 1,
    DSPP_1,
    DSPP_2,
    DSPP_3,
    DSPP_4,
    DSPP_5,
    DSPP_6,
    DSPP_7,
    DSPP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_ctl {
    CTL_0 = 1,
    CTL_1,
    CTL_2,
    CTL_3,
    CTL_4,
    CTL_5,
    CTL_6,
    CTL_7,
    CTL_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_dsc {
    DSC_NONE = 0,
    DSC_0,
    DSC_1,
    DSC_2,
    DSC_3,
    DSC_4,
    DSC_5,
    DSC_6,
    DSC_7,
    DSC_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_cdm {
    CDM_0 = 1,
    CDM_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_pingpong {
    PINGPONG_NONE,
    PINGPONG_0,
    PINGPONG_1,
    PINGPONG_2,
    PINGPONG_3,
    PINGPONG_4,
    PINGPONG_5,
    PINGPONG_6,
    PINGPONG_7,
    PINGPONG_CWB_0,
    PINGPONG_CWB_1,
    PINGPONG_CWB_2,
    PINGPONG_CWB_3,
    PINGPONG_S0,
    PINGPONG_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_merge_3d {
    MERGE_3D_0 = 1,
    MERGE_3D_1,
    MERGE_3D_2,
    MERGE_3D_3,
    MERGE_3D_4,
    MERGE_3D_5,
    MERGE_3D_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_intf {
    INTF_0 = 1,
    INTF_1,
    INTF_2,
    INTF_3,
    INTF_4,
    INTF_5,
    INTF_6,
    INTF_7,
    INTF_8,
    INTF_MAX
}

//
// Historically these values correspond to the values written to the
// DISP_INTF_SEL register, which had to programmed manually. On newer MDP
// generations this register is NOP, but we keep the values for historical
// reasons.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_intf_type {
    INTF_NONE = 0x0,
    INTF_DSI = 0x1,
    INTF_HDMI = 0x3,
    INTF_LCDC = 0x5,
// old eDP found on 8x74 and 8x84
    INTF_EDP = 0x9,
// both DP and eDP,  handled by the new DP driver
    INTF_DP = 0xa,

// virtual interfaces
    INTF_WB = 0x100,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_intf_mode {
    INTF_MODE_NONE = 0,
    INTF_MODE_CMD,
    INTF_MODE_VIDEO,
    INTF_MODE_WB_BLOCK,
    INTF_MODE_WB_LINE,
    INTF_MODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_wb {
    WB_0 = 1,
    WB_1,
    WB_2,
    WB_3,
    WB_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_cwb {
    CWB_0 = 0x1,
    CWB_1,
    CWB_2,
    CWB_3,
    CWB_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_wd_timer {
    WD_TIMER_0 = 0x1,
    WD_TIMER_1,
    WD_TIMER_2,
    WD_TIMER_3,
    WD_TIMER_4,
    WD_TIMER_5,
    WD_TIMER_MAX
}

//
// enum dpu_3d_blend_mode
// Desribes how the 3d data is blended
// @BLEND_3D_NONE      : 3d blending not enabled
// @BLEND_3D_FRAME_INT : Frame interleaving
// @BLEND_3D_H_ROW_INT : Horizontal row interleaving
// @BLEND_3D_V_ROW_INT : vertical row interleaving
// @BLEND_3D_COL_INT   : column interleaving
// @BLEND_3D_MAX       :
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_3d_blend_mode {
    BLEND_3D_NONE = 0,
    BLEND_3D_FRAME_INT,
    BLEND_3D_H_ROW_INT,
    BLEND_3D_V_ROW_INT,
    BLEND_3D_COL_INT,
    BLEND_3D_MAX
}

//
// struct dpu_hw_fmt_layout - format information of the source pixel data
// @num_planes: number of planes (including meta data planes)
// @width: image width
// @height: image height
// @total_size: total size in bytes
// @plane_addr: address of each plane
// @plane_size: length of each plane
// @plane_pitch: pitch of each plane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_fmt_layout {
    pub num_planes: u32,
    pub width: u32,
    pub height: u32,
    pub total_size: u32,
    pub plane_addr: [u32; DPU_MAX_PLANES],
    pub plane_size: [u32; DPU_MAX_PLANES],
    pub plane_pitch: [u32; DPU_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_csc_cfg {
// matrix coefficients in S15.16 format
    pub csc_mv: [u32; DPU_CSC_MATRIX_COEFF_SIZE],
    pub csc_pre_bv: [u32; DPU_CSC_BIAS_SIZE],
    pub csc_post_bv: [u32; DPU_CSC_BIAS_SIZE],
    pub csc_pre_lv: [u32; DPU_CSC_CLAMP_SIZE],
    pub csc_post_lv: [u32; DPU_CSC_CLAMP_SIZE],
}

//
// struct dpu_mdss_color - mdss color description
// color 0 : green
// color 1 : blue
// color 2 : red
// color 3 : alpha
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_mdss_color {
    pub color_0: u32,
    pub color_1: u32,
    pub color_2: u32,
    pub color_3: u32,
}

//
// Define bit masks for h/w logging.
//

//
// struct dpu_hw_tear_check - Struct contains parameters to configure
// tear-effect module. This structure is used to configure tear-check
// logic present either in ping-pong or in interface module.
// @vsync_count:        Ratio of MDP VSYNC clk freq(Hz) to refresh rate divided
// by no of lines
// @sync_cfg_height:    Total vertical lines (display height - 1)
// @vsync_init_val:     Init value to which the read pointer gets loaded at
// vsync edge
// @sync_threshold_start:    Read pointer threshold start ROI for write operation
// @sync_threshold_continue: The minimum number of lines the write pointer
// needs to be above the read pointer
// @start_pos:          The position from which the start_threshold value is added
// @rd_ptr_irq:         The read pointer line at which interrupt has to be generated
// @hw_vsync_mode:      Sync with external frame sync input
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_tear_check {
//
// This is ratio of MDP VSYNC clk freq(Hz) to
// refresh rate divided by no of lines
//
    pub vsync_count: u32,
    pub sync_cfg_height: u32,
    pub vsync_init_val: u32,
    pub sync_threshold_start: u32,
    pub sync_threshold_continue: u32,
    pub start_pos: u32,
    pub rd_ptr_irq: u32,
    pub hw_vsync_mode: u8,
}

//
// struct dpu_hw_pp_vsync_info - Struct contains parameters to configure
// read and write pointers for command mode panels
// @rd_ptr_init_val:    Value of rd pointer at vsync edge
// @rd_ptr_frame_count: Num frames sent since enabling interface
// @rd_ptr_line_count:  Current line on panel (rd ptr)
// @wr_ptr_line_count:  Current line within pp fifo (wr ptr)
// @intf_frame_count:   Frames read from intf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_pp_vsync_info {
    pub rd_ptr_init_val: u32,
    pub rd_ptr_frame_count: u32,
    pub rd_ptr_line_count: u32,
    pub wr_ptr_line_count: u32,
    pub intf_frame_count: u32,
}
