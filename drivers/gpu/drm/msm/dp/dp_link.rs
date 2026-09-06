//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_link.h
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

pub const DS_PORT_STATUS_CHANGED: c_uint = 0x200;
pub const DP_TEST_BIT_DEPTH_UNKNOWN: c_uint = 0xFFFFFFFF;

pub const DP_MAX_NUM_DP_LANES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_link_info {
    pub revision: c_uchar,
    pub rate: c_uint,
    pub supported_rates: [c_uint; DP_MAX_SUPPORTED_RATES],
    pub rate_set: c_uint,
    pub use_rate_set: bool,
    pub num_lanes: c_uint,
    pub capabilities: c_ulong,
}

pub const DP_TRAIN_LEVEL_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_link_test_video {
    pub test_video_pattern: u32,
    pub test_bit_depth: u32,
    pub test_dyn_range: u32,
    pub test_h_total: u32,
    pub test_v_total: u32,
    pub test_h_start: u32,
    pub test_v_start: u32,
    pub test_hsync_pol: u32,
    pub test_hsync_width: u32,
    pub test_vsync_pol: u32,
    pub test_vsync_width: u32,
    pub test_h_width: u32,
    pub test_v_height: u32,
    pub test_rr_d: u32,
    pub test_rr_n: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_link_test_audio {
    pub test_audio_sampling_rate: u32,
    pub test_audio_channel_count: u32,
    pub test_audio_pattern_type: u32,
    pub test_audio_period_ch_1: u32,
    pub test_audio_period_ch_2: u32,
    pub test_audio_period_ch_3: u32,
    pub test_audio_period_ch_4: u32,
    pub test_audio_period_ch_5: u32,
    pub test_audio_period_ch_6: u32,
    pub test_audio_period_ch_7: u32,
    pub test_audio_period_ch_8: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_link_phy_params {
    pub phy_test_pattern_sel: u32,
    pub v_level: u8,
    pub p_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_link {
    pub lttpr_common_caps: [u8; DP_LTTPR_COMMON_CAP_SIZE],
    pub lttpr_count: c_int,
    pub sink_request: u32,
    pub test_response: u32,
    pub sink_count: u8,
    pub test_video: msm_dp_link_test_video,
    pub test_audio: msm_dp_link_test_audio,
    pub phy_params: msm_dp_link_phy_params,
    pub link_params: msm_dp_link_info,
    pub lane_map: [u32; DP_MAX_NUM_DP_LANES],
    pub max_dp_lanes: u32,
    pub max_dp_link_rate: u32,
}

//
// msm_dp_link_bit_depth_to_bpp() - convert test bit depth to bpp
// @tbd: test bit depth
//
// Returns: the bits per pixel (bpp) to be used corresponding to the
// bit depth value. This function assumes that bit depth has
// already been validated.
//
// Few simplistic rules and assumptions made here:
// 1. Bit depth is per color component
// 2. If bit depth is unknown return 0
// 3. Assume 3 color components
//
extern "C" {
    pub fn msm_dp_link_reset_phy_params_vx_px(msm_dp_link: *mut msm_dp_link);
}
extern "C" {
    pub fn msm_dp_link_get_test_bits_depth(msm_dp_link: *mut msm_dp_link, bpp: u32) -> u32;
}
extern "C" {
    pub fn msm_dp_link_process_request(msm_dp_link: *mut msm_dp_link) -> c_int;
}
extern "C" {
    pub fn msm_dp_link_get_colorimetry_config(msm_dp_link: *mut msm_dp_link) -> c_int;
}
extern "C" {
    pub fn msm_dp_link_adjust_levels(msm_dp_link: *mut msm_dp_link, link_status: *mut u8) -> c_int;
}
extern "C" {
    pub fn msm_dp_link_send_test_response(msm_dp_link: *mut msm_dp_link) -> bool;
}
extern "C" {
    pub fn msm_dp_link_send_edid_checksum(msm_dp_link: *mut msm_dp_link, checksum: u8) -> bool;
}
//
// msm_dp_link_get() - get the functionalities of dp test module
// @dev: kernel device structure
// @aux: DisplayPort AUX channel
//
// return: a pointer to msm_dp_link struct
//
