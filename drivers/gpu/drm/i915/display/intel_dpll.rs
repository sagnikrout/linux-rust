//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dpll.h
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
    pub fn intel_dpll_init_clock_hook(display: *mut intel_display);
}
extern "C" {
    pub fn i9xx_calc_dpll_params(refclk: c_int, clock: *mut dpll) -> c_int;
}
extern "C" {
    pub fn i9xx_dpll_compute_fp(dpll: *const dpll) -> u32;
}
extern "C" {
    pub fn vlv_compute_dpll(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn chv_compute_dpll(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn vlv_force_pll_off(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn chv_enable_pll(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn chv_disable_pll(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn vlv_enable_pll(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn vlv_disable_pll(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn i9xx_enable_pll(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn i9xx_disable_pll(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn chv_calc_dpll_params(refclk: c_int, pll_clock: *mut dpll) -> c_int;
}
extern "C" {
    pub fn i9xx_crtc_clock_get(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn vlv_crtc_clock_get(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn chv_crtc_clock_get(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn assert_pll_enabled(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn assert_pll_disabled(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn intel_dpll_clock_matches(clock1: c_int, clock2: c_int) -> bool;
}
