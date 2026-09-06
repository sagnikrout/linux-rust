//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/skl_watermark.h
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
// Copyright © 2022 Intel Corporation
//

extern "C" {
    pub fn intel_enabled_dbuf_slices_mask(display: *mut intel_display) -> u8;
}
extern "C" {
    pub fn intel_sagv_pre_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_sagv_post_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_crtc_can_enable_sagv(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_has_sagv(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn skl_wm_crtc_disable_noatomic(crtc: *mut intel_crtc);
}
extern "C" {
    pub fn skl_watermark_ipc_init(display: *mut intel_display);
}
extern "C" {
    pub fn skl_watermark_ipc_update(display: *mut intel_display);
}
extern "C" {
    pub fn skl_watermark_ipc_enabled(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn skl_watermark_debugfs_register(display: *mut intel_display);
}
extern "C" {
    pub fn skl_wm_init(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dbuf_num_enabled_slices(dbuf_state: *const intel_dbuf_state) -> c_int;
}
extern "C" {
    pub fn intel_dbuf_num_active_pipes(dbuf_state: *const intel_dbuf_state) -> c_int;
}
extern "C" {
    pub fn intel_dbuf_init(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_dbuf_pre_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_dbuf_post_plane_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_dbuf_mbus_pre_ddb_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_dbuf_mbus_post_ddb_update(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_program_dpkgc_latency(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_dbuf_pmdemand_needs_update(state: *mut intel_atomic_state) -> bool;
}
extern "C" {
    pub fn skl_wm0_prefill_lines_worst(crtc_state: *const intel_crtc_state) -> c_uint;
}
extern "C" {
    pub fn skl_wm0_prefill_lines(crtc_state: *const intel_crtc_state) -> c_uint;
}
