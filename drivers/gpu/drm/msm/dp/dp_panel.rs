//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_panel.h
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
// Copyright (c) 2012-2020, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_display_mode {
    pub drm_mode: drm_display_mode,
    pub bpp: u32,
    pub h_active_low: u32,
    pub v_active_low: u32,
    pub out_fmt_is_yuv_420: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_panel_psr {
    pub version: u8,
    pub capabilities: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_panel {
// dpcd raw data
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub downstream_ports: [u8; DP_MAX_DOWNSTREAM_PORTS],
    pub link_info: msm_dp_link_info,
    pub connector: *mut drm_connector,
    pub msm_dp_mode: msm_dp_display_mode,
    pub psr_cap: msm_dp_panel_psr,
    pub video_test: bool,
    pub vsc_sdp_supported: bool,
    pub hw_revision: u32,
    pub max_bw_code: u32,
}

extern "C" {
    pub fn msm_dp_panel_deinit(msm_dp_panel: *mut msm_dp_panel) -> c_int;
}
extern "C" {
    pub fn msm_dp_panel_timing_cfg(msm_dp_panel: *mut msm_dp_panel, wide_bus_en: bool) -> c_int;
}
extern "C" {
    pub fn msm_dp_panel_tpg_config(msm_dp_panel: *mut msm_dp_panel, enable: bool);
}
extern "C" {
    pub fn msm_dp_panel_clear_dsc_dto(msm_dp_panel: *mut msm_dp_panel);
}
extern "C" {
    pub fn msm_dp_panel_enable_vsc_sdp(msm_dp_panel: *mut msm_dp_panel, vsc_sdp: *mut dp_sdp);
}
extern "C" {
    pub fn msm_dp_panel_disable_vsc_sdp(msm_dp_panel: *mut msm_dp_panel);
}
//
// is_link_rate_valid() - validates the link rate
// @bw_code: link rate requested by the sink
//
// Returns: true if the requested link rate is supported.
//
// is_lane_count_valid() - validates the lane count
// @lane_count: lane count requested by the sink
//
// Returns: true if the requested lane count is supported.
//
