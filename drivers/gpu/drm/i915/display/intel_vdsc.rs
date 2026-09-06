//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_vdsc.h
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
    pub fn intel_dsc_source_support(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_dsc_line_slice_count(config: *const intel_dsc_slice_config) -> c_int;
}
extern "C" {
    pub fn intel_uncompressed_joiner_enable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_dsc_enable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_dsc_disable(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_dsc_compute_params(pipe_config: *mut intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_dsc_enable_on_crtc(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_dsc_enabled_on_link(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_dsc_get_config(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_dsc_get_num_vdsc_instances(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vdsc_min_cdclk(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_vdsc_prefill_lines(crtc_state: *const intel_crtc_state) -> c_uint;
}
