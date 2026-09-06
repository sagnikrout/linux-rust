//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_cdclk.h
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
#[derive(Copy, Clone)]
pub struct intel_cdclk_config {
    pub bypass: unsigned int cdclk, vco, ref,,
    pub voltage_level: u8,
// This field is only valid for Xe2LPD and above.
    pub joined_mbus: bool,
}

extern "C" {
    pub fn intel_cdclk_ppc(display: *mut intel_display, double_wide: bool) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_init_hw(display: *mut intel_display);
}
extern "C" {
    pub fn intel_cdclk_uninit_hw(display: *mut intel_display);
}
extern "C" {
    pub fn intel_init_cdclk_hooks(display: *mut intel_display);
}
extern "C" {
    pub fn intel_update_max_cdclk(display: *mut intel_display);
}
extern "C" {
    pub fn intel_update_cdclk(display: *mut intel_display);
}
extern "C" {
    pub fn intel_read_rawclk(display: *mut intel_display) -> u32;
}
extern "C" {
    pub fn intel_cdclk_is_decreasing_later(state: *mut intel_atomic_state) -> bool;
}
extern "C" {
    pub fn intel_set_cdclk_pre_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_set_cdclk_post_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_cdclk_atomic_check(state: *mut intel_atomic_state) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_state_set_joined_mbus(state: *mut intel_atomic_state, joined_mbus: bool) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_update_hw_state(display: *mut intel_display);
}
extern "C" {
    pub fn intel_cdclk_crtc_disable_noatomic(crtc: *mut intel_crtc);
}

extern "C" {
    pub fn intel_cdclk_init(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_debugfs_register(display: *mut intel_display);
}
extern "C" {
    pub fn intel_cdclk_logical(cdclk_state: *const intel_cdclk_state) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_actual(cdclk_state: *const intel_cdclk_state) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_actual_voltage_level(cdclk_state: *const intel_cdclk_state) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_min_cdclk(cdclk_state: *const intel_cdclk_state, pipe: pipe) -> c_int;
}
extern "C" {
    pub fn intel_cdclk_pmdemand_needs_update(state: *mut intel_atomic_state) -> bool;
}
extern "C" {
    pub fn intel_cdclk_force_min_cdclk(cdclk_state: *mut intel_cdclk_state, force_min_cdclk: c_int);
}
extern "C" {
    pub fn intel_cdclk_read_hw(display: *mut intel_display);
}
extern "C" {
    pub fn intel_cdclk_prefill_adjustment(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn intel_cdclk_prefill_adjustment_worst(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn intel_crtc_min_cdclk(crtc_state: *const intel_crtc_state) -> c_int;
}
