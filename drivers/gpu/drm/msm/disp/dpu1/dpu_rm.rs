//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_rm.h
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
// Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
//

//
// struct dpu_rm - DPU dynamic hardware resource manager
// @pingpong_blks: array of pingpong hardware resources
// @mixer_blks: array of layer mixer hardware resources
// @ctl_blks: array of ctl hardware resources
// @hw_intf: array of intf hardware resources
// @hw_wb: array of wb hardware resources
// @hw_cwb: array of cwb hardware resources
// @dspp_blks: array of dspp hardware resources
// @hw_sspp: array of sspp hardware resources
// @cdm_blk: cdm hardware resource
// @has_legacy_ctls: DPU uses pre-ACTIVE CTL blocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_rm {
    pub PINGPONG_0]: *mut *mut dpu_hw_blk pingpong_blks[PINGPONG_MAX -,
    pub LM_0]: *mut *mut dpu_hw_blk mixer_blks[LM_MAX -,
    pub CTL_0]: *mut *mut dpu_hw_blk ctl_blks[CTL_MAX -,
    pub INTF_0]: *mut *mut dpu_hw_intf hw_intf[INTF_MAX -,
    pub WB_0]: *mut *mut dpu_hw_wb hw_wb[WB_MAX -,
    pub CWB_0]: *mut *mut dpu_hw_blk cwb_blks[CWB_MAX -,
    pub DSPP_0]: *mut *mut dpu_hw_blk dspp_blks[DSPP_MAX -,
    pub MERGE_3D_0]: *mut *mut dpu_hw_blk merge_3d_blks[MERGE_3D_MAX -,
    pub DSC_0]: *mut *mut dpu_hw_blk dsc_blks[DSC_MAX -,
    pub SSPP_NONE]: *mut *mut dpu_hw_sspp hw_sspp[SSPP_MAX -,
    pub cdm_blk: *mut dpu_hw_blk,
    pub has_legacy_ctls: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_rm_sspp_requirements {
    pub yuv: bool,
    pub scale: bool,
    pub rot90: bool,
}

//
// struct msm_display_topology - defines a display topology pipeline
// @num_lm:       number of layer mixers used
// @num_intf:     number of interfaces the panel is mounted on
// @num_dspp:     number of dspp blocks used
// @num_dsc:      number of Display Stream Compression (DSC) blocks used
// @num_cdm:      indicates how many outputs are requesting cdm block for
// this display topology
// @cwb_enabled:  indicates whether CWB is enabled for this display topology
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_display_topology {
    pub num_lm: u32,
    pub num_intf: u32,
    pub num_dspp: u32,
    pub num_dsc: u32,
    pub num_cdm: c_int,
    pub cwb_enabled: bool,
}

//
// dpu_rm_get_intf - Return a struct dpu_hw_intf instance given it's index.
// @rm: DPU Resource Manager handle
// @intf_idx: INTF's index
//
// dpu_rm_get_wb - Return a struct dpu_hw_wb instance given it's index.
// @rm: DPU Resource Manager handle
// @wb_idx: WB index
//
// dpu_rm_get_sspp - Return a struct dpu_hw_sspp instance given it's index.
// @rm: DPU Resource Manager handle
// @sspp_idx: SSPP index
//
