//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dp_link_training.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2019 Intel Corporation
//

extern "C" {
    pub fn intel_dp_read_dprx_caps(intel_dp: *mut intel_dp, dpcd[DP_RECEIVER_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn intel_dp_init_lttpr_and_dprx_caps(intel_dp: *mut intel_dp) -> c_int;
}
extern "C" {
    pub fn intel_dp_lttpr_transparent_mode_enabled(intel_dp: *mut intel_dp) -> bool;
}
// Get the TPSx symbol type of the value programmed to DP_TRAINING_PATTERN_SET
extern "C" {
    pub fn intel_dp_link_training_get_force_retrain(link_training: *mut intel_dp_link_training) -> bool;
}
extern "C" {
    pub fn intel_dp_link_check(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_dp_check_link_state(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_link_training_debugfs_add(connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_dp_link_training_reset(link_training: *mut intel_dp_link_training);
}
extern "C" {
    pub fn intel_dp_link_training_cleanup(link_training: *mut intel_dp_link_training);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_link_training_test_ops {
}

