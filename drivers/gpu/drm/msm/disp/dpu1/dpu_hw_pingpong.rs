//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_pingpong.h
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

pub const DITHER_MATRIX_SZ: c_int = 16;
//
// struct dpu_hw_dither_cfg - dither feature structure
// @flags: for customizing operations
// @temporal_en: temperal dither enable
// @c0_bitdepth: c0 component bit depth
// @c1_bitdepth: c1 component bit depth
// @c2_bitdepth: c2 component bit depth
// @c3_bitdepth: c2 component bit depth
// @matrix: dither strength matrix
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_dither_cfg {
    pub flags: u64,
    pub temporal_en: u32,
    pub c0_bitdepth: u32,
    pub c1_bitdepth: u32,
    pub c2_bitdepth: u32,
    pub c3_bitdepth: u32,
    pub matrix: [u32; DITHER_MATRIX_SZ],
}

//
// struct dpu_hw_pingpong_ops : Interface to the pingpong Hw driver functions
// Assumption is these functions will be called after clocks are enabled
// @enable_tearcheck: program and enable tear check block
// @disable_tearcheck: disable able tear check block
// @setup_dither : function to program the dither hw block
// @get_line_count: obtain current vertical line counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_pingpong_ops {
//
// @enable_tearcheck: enables vysnc generation and sets up init value of
// read pointer and programs the tear check cofiguration
//
    pub cfg): *mut dpu_hw_tear_check,
//
// @disable_tearcheck: disables tear check block
//
    pub pp): *mut *mut int (disable_tearcheck)(struct dpu_hw_pingpong,
//
// @connect_external_te: read, modify, write to either set or clear
// listening to external TE
// @Return: 1 if TE was originally connected, 0 if not, or -ERROR
//
    pub enable_external_te): bool,
//
// @get_line_count: Obtain current vertical line counter
//
    pub pp): *mut *mut u32 (get_line_count)(struct dpu_hw_pingpong,
//
// @disable_autorefresh: Disable autorefresh if enabled
//
    pub vdisplay): *mut *mut *mut void (disable_autorefresh)(struct dpu_hw_pingpong pp, uint32_t encoder_id, u16,
//
// @setup_dither: Setup dither matix for pingpong block
//
    pub cfg): *mut dpu_hw_dither_cfg,
//
// @enable_dsc: Enable DSC
//
    pub pp): *mut *mut int (enable_dsc)(struct dpu_hw_pingpong,
//
// @disable_dsc: Disable DSC
//
    pub pp): *mut *mut void (disable_dsc)(struct dpu_hw_pingpong,
//
// @setup_dsc: Setup DSC
//
    pub pp): *mut *mut int (setup_dsc)(struct dpu_hw_pingpong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_pingpong {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// pingpong
    pub idx: dpu_pingpong,
    pub caps: *const dpu_pingpong_cfg,
    pub merge_3d: *mut dpu_hw_merge_3d,
// ops
    pub ops: dpu_hw_pingpong_ops,
}

//
// to_dpu_hw_pingpong - convert base object dpu_hw_base to container
// @hw: Pointer to base hardware block
// return: Pointer to hardware block container
//
extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_pingpong: struct, _arg: base) -> return;
}
