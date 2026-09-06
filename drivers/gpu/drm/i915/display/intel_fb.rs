//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_fb.h
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
// Copyright © 2020-2021 Intel Corporation
//

pub const INTEL_PLANE_CAP_NONE: c_int = 0;

extern "C" {
    pub fn intel_fb_is_tiled_modifier(modifier: u64) -> bool;
}
extern "C" {
    pub fn intel_fb_is_ccs_modifier(modifier: u64) -> bool;
}
extern "C" {
    pub fn intel_fb_is_rc_ccs_cc_modifier(modifier: u64) -> bool;
}
extern "C" {
    pub fn intel_fb_is_mc_ccs_modifier(modifier: u64) -> bool;
}
extern "C" {
    pub fn intel_fb_needs_64k_phys(modifier: u64) -> bool;
}
extern "C" {
    pub fn intel_fb_is_tile4_modifier(modifier: u64) -> bool;
}
extern "C" {
    pub fn intel_fb_needs_cpu_access(fb: *const drm_framebuffer) -> bool;
}
extern "C" {
    pub fn intel_fb_is_ccs_aux_plane(fb: *const drm_framebuffer, color_plane: c_int) -> bool;
}
extern "C" {
    pub fn intel_fb_rc_ccs_cc_plane(fb: *const drm_framebuffer) -> c_int;
}
extern "C" {
    pub fn intel_fb_plane_supports_modifier(plane: *mut intel_plane, modifier: u64) -> bool;
}
extern "C" {
    pub fn is_surface_linear(fb: *const drm_framebuffer, color_plane: c_int) -> bool;
}
extern "C" {
    pub fn main_to_ccs_plane(fb: *const drm_framebuffer, main_plane: c_int) -> c_int;
}
extern "C" {
    pub fn skl_ccs_to_main_plane(fb: *const drm_framebuffer, ccs_plane: c_int) -> c_int;
}
extern "C" {
    pub fn skl_main_to_aux_plane(fb: *const drm_framebuffer, main_plane: c_int) -> c_int;
}
extern "C" {
    pub fn intel_tile_size(display: *mut intel_display) -> c_uint;
}
extern "C" {
    pub fn intel_tile_width_bytes(fb: *const drm_framebuffer, color_plane: c_int) -> c_uint;
}
extern "C" {
    pub fn intel_tile_height(fb: *const drm_framebuffer, color_plane: c_int) -> c_uint;
}
extern "C" {
    pub fn intel_plane_uses_fence(plane_state: *const intel_plane_state) -> bool;
}
extern "C" {
    pub fn intel_fb_supports_90_270_rotation(fb: *const intel_framebuffer) -> bool;
}
extern "C" {
    pub fn intel_rotation_info_size(rot_info: *const intel_rotation_info) -> c_uint;
}
extern "C" {
    pub fn intel_remapped_info_size(rem_info: *const intel_remapped_info) -> c_uint;
}
extern "C" {
    pub fn intel_fill_fb_info(display: *mut intel_display, fb: *mut intel_framebuffer) -> c_int;
}
extern "C" {
    pub fn intel_plane_compute_gtt(plane_state: *mut intel_plane_state) -> c_int;
}
extern "C" {
    pub fn intel_fb_modifier_uses_dpt(display: *mut intel_display, modifier: u64) -> bool;
}
extern "C" {
    pub fn intel_fb_uses_dpt(fb: *const drm_framebuffer) -> bool;
}
extern "C" {
    pub fn intel_fb_modifier_to_tiling(fb_modifier: u64) -> c_uint;
}
