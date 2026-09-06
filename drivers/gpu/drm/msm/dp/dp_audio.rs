//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_audio.h
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

//
// struct msm_dp_audio
// @lane_count: number of lanes configured in current session
// @bw_code: link rate's bandwidth code for current session
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_audio {
    pub lane_count: u32,
    pub bw_code: u32,
}

//
// msm_dp_audio_get()
//
// Creates and instance of dp audio.
//
// @pdev: caller's platform device instance.
// @link_base: pointer to the msm_dp_link resource.
//
// Returns the error code in case of failure, otherwize
// an instance of newly created msm_dp_module.
//
// msm_dp_audio_put()
//
// Cleans the msm_dp_audio instance.
//
// @msm_dp_audio: an instance of msm_dp_audio.
//
extern "C" {
    pub fn msm_dp_audio_put(msm_dp_audio: *mut msm_dp_audio);
}
