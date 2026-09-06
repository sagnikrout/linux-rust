//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_vrr.h
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
    pub fn intel_vrr_is_capable(connector: *mut intel_connector) -> bool;
}
extern "C" {
    pub fn intel_vrr_is_in_range(connector: *mut intel_connector, vrefresh: c_int) -> bool;
}
extern "C" {
    pub fn intel_vrr_possible(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_vrr_check_modeset(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_vrr_compute_guardband(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_set_transcoder_timings(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_enable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_is_push_sent(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_vrr_disable(old_crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_psr_frame_change_enable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_get_config(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_vmax_vtotal(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_vmin_vtotal(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_vmax_vblank_start(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_vmin_vblank_start(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_is_fixed_rr(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_vrr_transcoder_enable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_transcoder_disable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_vrr_always_use_vrr_tg(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_vrr_safe_window_start(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_vmin_safe_window_end(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_dcb_vmin_vblank_start_next(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_dcb_vmax_vblank_start_next(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_dcb_vmin_vblank_start_final(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vrr_dcb_vmax_vblank_start_final(crtc_state: *const intel_crtc_state) -> c_int;
}
