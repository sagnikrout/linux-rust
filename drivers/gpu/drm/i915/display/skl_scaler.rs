//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/skl_scaler.h
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
extern "C" {
    pub fn skl_update_scaler_crtc(crtc_state: *mut intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn skl_pfit_enable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn skl_scaler_disable(old_crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn skl_scaler_get_config(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn adl_scaler_ecc_mask(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn adl_scaler_ecc_unmask(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn skl_scaler_max_total_scale(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_max_scale(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_max_hscale(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_1st_prefill_adjustment_worst(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_2nd_prefill_adjustment_worst(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_1st_prefill_lines_worst(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_2nd_prefill_lines_worst(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_1st_prefill_adjustment(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_2nd_prefill_adjustment(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_1st_prefill_lines(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_scaler_2nd_prefill_lines(crtc_state: *const intel_crtc_state) -> c_uint;
}
