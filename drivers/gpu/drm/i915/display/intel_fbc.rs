//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_fbc.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_fbc_id {
    INTEL_FBC_A,
    INTEL_FBC_B,
    INTEL_FBC_C,
    INTEL_FBC_D,

    I915_MAX_FBCS,
}

extern "C" {
    pub fn intel_fbc_atomic_check(state: *mut intel_atomic_state) -> c_int;
}
extern "C" {
    pub fn intel_fbc_min_cdclk(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_fbc_init(display: *mut intel_display);
}
extern "C" {
    pub fn intel_fbc_cleanup(display: *mut intel_display);
}
extern "C" {
    pub fn intel_fbc_sanitize(display: *mut intel_display);
}
extern "C" {
    pub fn intel_fbc_disable(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_fbc_add_plane(fbc: *mut intel_fbc, plane: *mut intel_plane);
}
extern "C" {
    pub fn intel_fbc_handle_fifo_underrun_irq(display: *mut intel_display);
}
extern "C" {
    pub fn intel_fbc_reset_underrun(display: *mut intel_display);
}
extern "C" {
    pub fn intel_fbc_crtc_debugfs_add(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_fbc_debugfs_register(display: *mut intel_display);
}
extern "C" {
    pub fn intel_fbc_need_pixel_normalizer(plane_state: *const intel_plane_state) -> bool;
}
