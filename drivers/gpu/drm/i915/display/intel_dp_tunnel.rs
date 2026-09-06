//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dp_tunnel.h
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
// Copyright © 2023 Intel Corporation
//

extern "C" {
    pub fn intel_dp_tunnel_detect(intel_dp: *mut intel_dp, ctx: *mut drm_modeset_acquire_ctx) -> c_int;
}
extern "C" {
    pub fn intel_dp_tunnel_disconnect(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_tunnel_destroy(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_tunnel_suspend(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_tunnel_bw_alloc_is_enabled(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_dp_tunnel_pr_optimization_supported(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_dp_tunnel_atomic_alloc_bw(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_dp_tunnel_uhbr_lanes_wa_apply(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_tunnel_uhbr_lanes_wa_setup(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_dp_tunnel_uhbr_lanes_wa_reset(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_tunnel_mgr_init(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_dp_tunnel_mgr_cleanup(display: *mut intel_display);
}

