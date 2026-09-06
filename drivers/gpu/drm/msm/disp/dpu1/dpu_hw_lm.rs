//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_lm.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_mixer_cfg {
    pub out_width: u32,
    pub out_height: u32,
    pub right_mixer: bool,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_color3_cfg {
    pub keep_fg: [u8; DPU_STAGE_MAX],
}

//
// struct dpu_hw_lm_ops : Interface to the mixer Hw driver functions
// Assumption is these functions will be called after clocks are enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_lm_ops {
//
// @setup_mixer_out: Sets up mixer output width and height
// and border color if enabled
//
    pub cfg): *mut dpu_hw_mixer_cfg,
//
// @setup_blend_config: Alpha blending configuration
// for the specified stage
//
    pub blend_op): u16 fg_alpha, u16 bg_alpha, uint32_t,
//
// @setup_alpha_out: Alpha color component selection from either fg or bg
//
    pub mixer_op): *mut *mut *mut void (setup_alpha_out)(struct dpu_hw_mixer ctx, uint32_t,
//
// @clear_all_blendstages: Clear layer mixer to pipe configuration
// @ctx		: mixer ctx pointer
// Returns: 0 on success or -error
//
    pub ctx): *mut *mut int (clear_all_blendstages)(struct dpu_hw_mixer,
//
// @setup_blendstage: Configure layer mixer to pipe configuration
// @ctx		: mixer ctx pointer
// @lm		: layer mixer enumeration
// @stage_cfg	: blend stage configuration
// Returns: 0 on success or -error
//
    pub stage_cfg): *mut dpu_hw_stage_cfg,
//
// @setup_border_color : enable/disable border color
//
    pub border_en): u8,
//
// @setup_misr: Enable/disable MISR
//
    pub ctx): *mut *mut void (setup_misr)(struct dpu_hw_mixer,
//
// @collect_misr: Read MISR signature
//
    pub misr_value): *mut *mut *mut int (collect_misr)(struct dpu_hw_mixer ctx, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_mixer {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// lm
    pub idx: dpu_lm,
    pub cap: *const dpu_lm_cfg,
    pub mdp: *const dpu_mdp_cfg,
    pub ctl: *const dpu_ctl_cfg,
// ops
    pub ops: dpu_hw_lm_ops,
// store mixer info specific to display
    pub cfg: dpu_hw_mixer_cfg,
}

//
// to_dpu_hw_mixer - convert base object dpu_hw_base to container
// @hw: Pointer to base hardware block
// return: Pointer to hardware block container
//
extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_mixer: struct, _arg: base) -> return;
}
