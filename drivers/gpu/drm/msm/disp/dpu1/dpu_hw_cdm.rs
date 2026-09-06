//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_cdm.h
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
// Copyright (c) 2023, The Linux Foundation. All rights reserved.
//

//
// struct dpu_hw_cdm_cfg : current configuration of CDM block
//
// @output_width:         output ROI width of CDM block
// @output_height:        output ROI height of CDM block
// @output_bit_depth:     output bit-depth of CDM block
// @h_cdwn_type:          downsample type used for horizontal pixels
// @v_cdwn_type:          downsample type used for vertical pixels
// @output_fmt:           handle to msm_format of CDM block
// @csc_cfg:              handle to CSC matrix programmed for CDM block
// @output_type:          interface to which CDM is paired (HDMI/WB)
// @pp_id:                ping-pong block to which CDM is bound to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_cdm_cfg {
    pub output_width: u32,
    pub output_height: u32,
    pub output_bit_depth: u32,
    pub h_cdwn_type: u32,
    pub v_cdwn_type: u32,
    pub output_fmt: *const msm_format,
    pub csc_cfg: *const dpu_csc_cfg,
    pub output_type: u32,
    pub pp_id: c_int,
}

//
// These values are used indicate which type of downsample is used
// in the horizontal/vertical direction for the CDM block.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_cdwn_type {
    CDM_CDWN_DISABLE,
    CDM_CDWN_PIXEL_DROP,
    CDM_CDWN_AVG,
    CDM_CDWN_COSITE,
    CDM_CDWN_OFFSITE,
}

//
// CDM block can be paired with WB or HDMI block. These values match
// the input with which the CDM block is paired.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_cdwn_output_type {
    CDM_CDWN_OUTPUT_HDMI,
    CDM_CDWN_OUTPUT_WB,
}

//
// CDM block can give an 8-bit or 10-bit output. These values
// are used to indicate the output bit depth of CDM block
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_cdwn_output_bit_depth {
    CDM_CDWN_OUTPUT_8BIT,
    CDM_CDWN_OUTPUT_10BIT,
}

//
// CDM block can downsample using different methods. These values
// are used to indicate the downsample method which can be used
// either in the horizontal or vertical direction.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_cdwn_op_mode_method_h_v {
    CDM_CDWN2_METHOD_PIXEL_DROP,
    CDM_CDWN2_METHOD_AVG,
    CDM_CDWN2_METHOD_COSITE,
    CDM_CDWN2_METHOD_OFFSITE
}

//
// struct dpu_hw_cdm_ops : Interface to the chroma down Hw driver functions
// Assumption is these functions will be called after
// clocks are enabled
// @enable:               Enables the output to interface and programs the
// output packer
// @bind_pingpong_blk:    enable/disable the connection with pingpong which
// will feed pixels to this cdm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_cdm_ops {
//
// @enable: Enable the CDM module
// @cdm         Pointer to chroma down context
//
    pub cfg): *mut *mut *mut int (enable)(struct dpu_hw_cdm cdm, struct dpu_hw_cdm_cfg,
//
// @bind_pingpong_blk: Enable/disable the connection with pingpong
// @cdm         Pointer to chroma down context
// @pp          pingpong block id.
//
    pub pp): *const *const *const void (bind_pingpong_blk)(struct dpu_hw_cdm cdm, enum dpu_pingpong,
}

//
// struct dpu_hw_cdm - cdm description
// @base: Hardware block base structure
// @hw: Block hardware details
// @idx: CDM index
// @caps: Pointer to cdm_cfg
// @ops: handle to operations possible for this CDM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_cdm {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// chroma down
    pub caps: *const dpu_cdm_cfg,
    pub idx: dpu_cdm,
// ops
    pub ops: dpu_hw_cdm_ops,
}

extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_cdm: struct, _arg: base) -> return;
}
