//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_plane.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

//
// struct dpu_plane_state: Define dpu extension of drm plane state object
// @base:	base drm plane state object
// @pipe:	software pipe description array
// @pipe_cfg:	software pipe configuration array
// @stage:	assigned by crtc blender
// @needs_qos_remap: qos remap settings need to be updated
// @multirect_index: index of the rectangle of SSPP
// @multirect_mode: parallel or time multiplex multirect mode
// @pending:	whether the current update is still pending
// @plane_fetch_bw: calculated BW per plane
// @plane_clk: calculated clk per plane
// @needs_dirtyfb: whether attached CRTC needs pixel data explicitly flushed
// @layout:     framebuffer memory layout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_plane_state {
    pub base: drm_plane_state,
    pub pipe: [dpu_sw_pipe; PIPES_PER_PLANE],
    pub pipe_cfg: [dpu_sw_pipe_cfg; PIPES_PER_PLANE],
    pub stage: dpu_stage,
    pub needs_qos_remap: bool,
    pub pending: bool,
    pub plane_fetch_bw: u64,
    pub plane_clk: u64,
    pub needs_dirtyfb: bool,
    pub layout: dpu_hw_fmt_layout,
}

extern "C" {
    pub fn dpu_plane_flush(plane: *mut drm_plane);
}
extern "C" {
    pub fn dpu_plane_set_error(plane: *mut drm_plane, error: bool);
}

extern "C" {
    pub fn dpu_plane_danger_signal_ctrl(plane: *mut drm_plane, enable: bool);
}

