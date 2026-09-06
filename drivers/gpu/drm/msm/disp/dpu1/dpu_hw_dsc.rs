//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_dsc.h
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
// Copyright (c) 2020-2022, Linaro Limited
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved
//

//
// struct dpu_hw_dsc_ops - interface to the dsc hardware driver functions
// Assumption is these functions will be called after clocks are enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_dsc_ops {
//
// @dsc_disable: disable dsc
// @hw_dsc: Pointer to dsc context
//
    pub hw_dsc): *mut *mut void (dsc_disable)(struct dpu_hw_dsc,
//
// @dsc_config: configures dsc encoder
// @hw_dsc: Pointer to dsc context
// @dsc: panel dsc parameters
// @mode: dsc topology mode to be set
// @initial_lines: amount of initial lines to be used
//
    pub initial_lines): u32,
//
// @dsc_config_thresh: programs panel thresholds
// @hw_dsc: Pointer to dsc context
// @dsc: panel dsc parameters
//
    pub dsc): *mut drm_dsc_config,
//
// @dsc_bind_pingpong_blk: binds pixel output from a DSC block
// to a pingpong block
//
    pub pp): dpu_pingpong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_dsc {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// dsc
    pub idx: dpu_dsc,
    pub caps: *const dpu_dsc_cfg,
// ops
    pub ops: dpu_hw_dsc_ops,
}

extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_dsc: struct, _arg: base) -> return;
}
