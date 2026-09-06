//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_color.h
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
    pub fn intel_color_init_hooks(display: *mut intel_display);
}
extern "C" {
    pub fn intel_color_init(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_color_crtc_init(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_color_cleanup_commit(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_color_uses_dsb(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_color_uses_chained_dsb(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_color_uses_gosub_dsb(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_color_wait_commit(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_color_post_update(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_color_load_luts(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_color_modeset(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_color_get_config(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_color_assert_luts(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_color_crtc_has_3dlut(display: *mut intel_display, pipe: pipe) -> bool;
}
extern "C" {
    pub fn intel_color_background_color_drm_to_hw(drm_background_color: u64) -> u32;
}
extern "C" {
    pub fn intel_color_background_color_hw_to_drm(hw_background_color: u32) -> u64;
}
