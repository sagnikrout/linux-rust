//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_display.h
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
// Copyright (c) 2017-2020, The Linux Foundation. All rights reserved.
//

pub const DP_MAX_PIXEL_CLK_KHZ: c_int = 675000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp {
    pub drm_dev: *mut drm_device,
    pub pdev: *mut platform_device,
    pub connector: *mut drm_connector,
    pub next_bridge: *mut drm_bridge,
    pub bridge: *mut drm_bridge,
    pub audio_enabled: bool,
    pub power_on: bool,
    pub connector_type: c_uint,
    pub is_edp: bool,
    pub msm_dp_audio: *mut msm_dp_audio,
    pub psr_supported: bool,
}

extern "C" {
    pub fn msm_dp_display_get_modes(msm_dp_display: *mut msm_dp) -> c_int;
}
extern "C" {
    pub fn msm_dp_display_check_video_test(msm_dp_display: *mut msm_dp) -> bool;
}
extern "C" {
    pub fn msm_dp_display_get_test_bpp(msm_dp_display: *mut msm_dp) -> c_int;
}
extern "C" {
    pub fn msm_dp_display_signal_audio_start(msm_dp_display: *mut msm_dp);
}
extern "C" {
    pub fn msm_dp_display_signal_audio_complete(msm_dp_display: *mut msm_dp);
}
extern "C" {
    pub fn msm_dp_display_set_psr(dp: *mut msm_dp, enter: bool);
}
extern "C" {
    pub fn msm_dp_display_debugfs_init(msm_dp_display: *mut msm_dp, dentry: *mut dentry, is_edp: bool);
}
extern "C" {
    pub fn msm_dp_display_atomic_post_disable(dp_display: *mut msm_dp);
}
extern "C" {
    pub fn msm_dp_display_atomic_disable(dp_display: *mut msm_dp);
}
