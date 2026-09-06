//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_encoder.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

pub const MAX_H_TILES_PER_DISPLAY: c_int = 2;
//
// struct msm_display_info - defines display properties
// @intf_type:          INTF_ type
// @num_of_h_tiles:     Number of horizontal tiles in case of split interface
// @h_tile_instance:    Controller instance used per tile. Number of elements is
// based on num_of_h_tiles
// @is_cmd_mode		Boolean to indicate if the CMD mode is requested
// @vsync_source:	Source of the TE signal for DSI CMD devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_display_info {
    pub intf_type: dpu_intf_type,
    pub num_of_h_tiles: u32,
    pub h_tile_instance: [u32; MAX_H_TILES_PER_DISPLAY],
    pub is_cmd_mode: bool,
    pub vsync_source: dpu_vsync_source,
}

extern "C" {
    pub fn dpu_encoder_prepare_for_kickoff(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn dpu_encoder_trigger_kickoff_pending(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn dpu_encoder_kickoff(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn dpu_encoder_vsync_time(drm_enc: *mut drm_encoder, wakeup_time: *mut ktime_t) -> c_int;
}
extern "C" {
    pub fn dpu_encoder_wait_for_commit_done(drm_encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn dpu_encoder_wait_for_tx_complete(drm_encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn dpu_encoder_get_intf_mode(encoder: *mut drm_encoder) -> dpu_intf_mode;
}
extern "C" {
    pub fn dpu_encoder_virt_runtime_resume(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn dpu_encoder_get_clones(drm_enc: *mut drm_encoder) -> u32;
}
extern "C" {
    pub fn dpu_encoder_get_linecount(drm_enc: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn dpu_encoder_get_vsync_count(drm_enc: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn dpu_encoder_is_widebus_enabled(drm_enc: *const drm_encoder) -> bool;
}
extern "C" {
    pub fn dpu_encoder_is_dsc_enabled(drm_enc: *const drm_encoder) -> bool;
}
extern "C" {
    pub fn dpu_encoder_get_crc_values_cnt(drm_enc: *const drm_encoder) -> c_int;
}
extern "C" {
    pub fn dpu_encoder_setup_misr(drm_encoder: *const drm_encoder);
}
extern "C" {
    pub fn dpu_encoder_get_crc(drm_enc: *const drm_encoder, crcs: *mut u32, pos: c_int) -> c_int;
}
extern "C" {
    pub fn dpu_encoder_use_dsc_merge(drm_enc: *mut drm_encoder) -> bool;
}
extern "C" {
    pub fn dpu_encoder_needs_modeset(drm_enc: *mut drm_encoder, state: *mut drm_atomic_commit) -> bool;
}
extern "C" {
    pub fn dpu_encoder_is_valid_for_commit(drm_enc: *mut drm_encoder) -> bool;
}
extern "C" {
    pub fn dpu_encoder_start_frame_done_timer(drm_enc: *mut drm_encoder);
}
