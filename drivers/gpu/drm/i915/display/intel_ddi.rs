//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_ddi.h
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
    pub fn intel_ddi_disable_clock(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn hsw_ddi_disable_clock(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn hsw_ddi_is_clock_enabled(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_wait_ddi_buf_idle(display: *mut intel_display, port: port);
}
extern "C" {
    pub fn intel_ddi_get_hw_state(encoder: *mut intel_encoder, pipe: *mut pipe) -> bool;
}
extern "C" {
    pub fn intel_ddi_disable_transcoder_func(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_ddi_disable_transcoder_clock(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_ddi_connector_get_hw_state(intel_connector: *mut intel_connector) -> bool;
}
extern "C" {
    pub fn intel_ddi_compute_min_voltage_level(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_ddi_sanitize_encoder_pll_mapping(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_ddi_link_symbol_clock(encoder: *mut intel_encoder, clock: c_int) -> c_int;
}
