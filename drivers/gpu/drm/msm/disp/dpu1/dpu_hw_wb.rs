//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_wb.h
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
// Copyright (c) 2022 Qualcomm Innovation Center, Inc. All rights reserved
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_wb_cfg {
    pub dest: dpu_hw_fmt_layout,
    pub intf_mode: dpu_intf_mode,
    pub roi: drm_rect,
    pub crop: drm_rect,
}

//
// struct dpu_hw_wb_ops : Interface to the wb hw driver functions
// Assumption is these functions will be called after clocks are enabled
// @setup_outaddress: setup output address from the writeback job
// @setup_outformat: setup output format of writeback block from writeback job
// @setup_roi:       setup ROI (Region of Interest) parameters
// @setup_qos_lut:   setup qos LUT for writeback block based on input
// @setup_cdp:       setup chroma down prefetch block for writeback block
// @setup_clk_force_ctrl: setup clock force control
// @bind_pingpong_blk: enable/disable the connection with ping-pong block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_wb_ops {
    pub wb): *mut dpu_hw_wb_cfg,
    pub fmt): *const msm_format,
    pub wb): *mut dpu_hw_wb_cfg,
    pub cfg): *mut dpu_hw_qos_cfg,
    pub enable): bool,
    pub enable): bool,
    pub pp): dpu_pingpong,
}

//
// struct dpu_hw_wb : WB driver object
// @hw: block hardware details
// @idx: hardware index number within type
// @caps: hardware capabilities
// @ops: function pointers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_wb {
    pub hw: dpu_hw_blk_reg_map,
// wb path
    pub idx: c_int,
    pub caps: *const dpu_wb_cfg,
// ops
    pub ops: dpu_hw_wb_ops,
}
