//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_vblank.h
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
// Copyright © 2022-2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vblank_evade_ctx {
    pub crtc: *mut intel_crtc,
    pub vblank_start: int min, max,,
    pub need_vlv_dsi_wa: bool,
}

extern "C" {
    pub fn intel_mode_vdisplay(mode: *const drm_display_mode) -> c_int;
}
extern "C" {
    pub fn intel_mode_vblank_start(mode: *const drm_display_mode) -> c_int;
}
extern "C" {
    pub fn intel_mode_vblank_end(mode: *const drm_display_mode) -> c_int;
}
extern "C" {
    pub fn intel_mode_vtotal(mode: *const drm_display_mode) -> c_int;
}
extern "C" {
    pub fn intel_mode_vblank_delay(mode: *const drm_display_mode) -> c_int;
}
// must be called with vblank interrupt already enabled!
extern "C" {
    pub fn intel_vblank_evade(evade: *mut intel_vblank_evade_ctx) -> c_int;
}
extern "C" {
    pub fn i915_get_vblank_counter(crtc: *mut drm_crtc) -> u32;
}
extern "C" {
    pub fn g4x_get_vblank_counter(crtc: *mut drm_crtc) -> u32;
}
extern "C" {
    pub fn intel_get_crtc_scanline(crtc: *mut intel_crtc) -> c_int;
}
extern "C" {
    pub fn intel_wait_for_pipe_scanline_stopped(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_wait_for_pipe_scanline_moving(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_crtc_scanline_offset(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_crtc_vblank_length(crtc_state: *const intel_crtc_state) -> c_int;
}
