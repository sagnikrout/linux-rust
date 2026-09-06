//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_bw.h
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
    pub fn intel_bw_init_hw(display: *mut intel_display);
}
extern "C" {
    pub fn intel_bw_init(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_bw_atomic_check(state: *mut intel_atomic_state) -> c_int;
}
extern "C" {
    pub fn intel_bw_update_hw_state(display: *mut intel_display);
}
extern "C" {
    pub fn intel_bw_crtc_disable_noatomic(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_bw_pmdemand_needs_update(state: *mut intel_atomic_state) -> bool;
}
extern "C" {
    pub fn icl_sagv_pre_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn icl_sagv_post_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_bw_qgv_point_peakbw(bw_state: *const intel_bw_state) -> c_int;
}
