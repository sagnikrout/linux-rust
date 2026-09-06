//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_psr.h
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
    pub fn intel_encoder_can_psr(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_psr_init_dpcd(intel_dp: *mut intel_dp, connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_psr_panel_replay_enable_sink(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_psr_debug_set(intel_dp: *mut intel_dp, value: u64) -> c_int;
}
extern "C" {
    pub fn intel_psr_init(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_psr_irq_handler(intel_dp: *mut intel_dp, psr_iir: u32);
}
extern "C" {
    pub fn intel_psr_short_pulse(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_psr_wait_for_idle_locked(new_crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_psr_enabled(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_psr2_panic_force_full_update(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_psr_pause(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_psr_resume(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_psr_needs_vblank_notification(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_psr_notify_dc5_dc6(display: *mut intel_display);
}
extern "C" {
    pub fn intel_psr_dc5_dc6_wa_init(display: *mut intel_display);
}
extern "C" {
    pub fn intel_psr_link_ok(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_psr_lock(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_psr_unlock(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_psr_min_set_context_latency(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_psr_connector_debugfs_add(connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_psr_debugfs_register(display: *mut intel_display);
}
extern "C" {
    pub fn intel_psr_needs_alpm(intel_dp: *mut intel_dp, crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_psr_min_guardband(crtc_state: *mut intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_psr_use_trans_push(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_psr_pr_async_video_timing_supported(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_psr2_in_deep_sleep(intel_dp: *mut intel_dp) -> bool;
}
