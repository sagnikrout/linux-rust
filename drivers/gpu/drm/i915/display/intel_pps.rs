//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_pps.h
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
    pub fn intel_pps_backlight_on(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_backlight_off(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_backlight_power(connector: *mut intel_connector, enable: bool);
}
extern "C" {
    pub fn intel_pps_vdd_on_unlocked(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_pps_vdd_off_unlocked(intel_dp: *mut intel_dp, sync: bool);
}
extern "C" {
    pub fn intel_pps_on_unlocked(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_off_unlocked(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_check_power_unlocked(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_vdd_on(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_vdd_off(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_on(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_off(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_vdd_off_sync(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_have_panel_power_or_vdd(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_pps_wait_power_cycle(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_init(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_pps_init_late(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_pps_encoder_reset(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn vlv_pps_pipe_init(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn vlv_pps_pipe_reset(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn vlv_pps_backlight_initial_pipe(intel_dp: *mut intel_dp) -> pipe;
}
extern "C" {
    pub fn vlv_pps_reset_all(display: *mut intel_display);
}
extern "C" {
    pub fn bxt_pps_reset_all(display: *mut intel_display);
}
extern "C" {
    pub fn intel_pps_unlock_regs_wa(display: *mut intel_display);
}
extern "C" {
    pub fn intel_pps_setup(display: *mut intel_display);
}
extern "C" {
    pub fn intel_pps_connector_debugfs_add(connector: *mut intel_connector);
}
extern "C" {
    pub fn assert_pps_unlocked(display: *mut intel_display, pipe: pipe);
}
