//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_crtc.h
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
// Copyright © 2020 Intel Corporation
//

//
// FIXME: We should instead only take spinlocks once for the entire update
// instead of once per mmio.
//

pub const VBLANK_EVASION_TIME_US: c_int = 250;

pub const VBLANK_EVASION_TIME_US: c_int = 100;

extern "C" {
    pub fn intel_crtc_arm_vblank_event(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_crtc_max_vblank_count(crtc_state: *const intel_crtc_state) -> u32;
}
extern "C" {
    pub fn intel_crtc_init(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_crtc_get_vblank_counter(crtc: *mut intel_crtc) -> u32;
}
extern "C" {
    pub fn intel_crtc_vblank_on(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_crtc_vblank_off(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_wait_for_vblank_workers(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_crtc_wait_for_next_vblank(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_any_crtc_enable_changed(state: *mut intel_atomic_state) -> bool;
}
extern "C" {
    pub fn intel_any_crtc_active_changed(state: *mut intel_atomic_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_bw_num_active_planes(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn intel_crtc_bw_data_rate(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn intel_crtc_bw_min_cdclk(crtc_state: *const intel_crtc_state) -> c_int;
}
