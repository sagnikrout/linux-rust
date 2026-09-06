//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_merge3d.h
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

//
// struct dpu_hw_merge_3d_ops : Interface to the merge_3d Hw driver functions
// Assumption is these functions will be called after clocks are enabled
// @setup_3d_mode : enable 3D merge
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_merge_3d_ops {
    pub mode_3d): dpu_3d_blend_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_merge_3d {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// merge_3d
    pub idx: dpu_merge_3d,
    pub caps: *const dpu_merge_3d_cfg,
// ops
    pub ops: dpu_hw_merge_3d_ops,
}

//
// to_dpu_hw_merge_3d - convert base object dpu_hw_base to container
// @hw: Pointer to base hardware block
// return: Pointer to hardware block container
//
extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_merge_3d: struct, _arg: base) -> return;
}
