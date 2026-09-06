//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gem_atomic_helper.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Plane Helpers
//
extern "C" {
    pub fn drm_gem_plane_helper_prepare_fb(plane: *mut drm_plane, state: *mut drm_plane_state) -> c_int;
}
//
// Helpers for planes with shadow buffers
//
// DRM_SHADOW_PLANE_MAX_WIDTH - Maximum width of a plane's shadow buffer in pixels
//
// For drivers with shadow planes, the maximum width of the framebuffer is
// usually independent from hardware limitations. Drivers can initialize struct
// drm_mode_config.max_width from DRM_SHADOW_PLANE_MAX_WIDTH.
//

//
// DRM_SHADOW_PLANE_MAX_HEIGHT - Maximum height of a plane's shadow buffer in scanlines
//
// For drivers with shadow planes, the maximum height of the framebuffer is
// usually independent from hardware limitations. Drivers can initialize struct
// drm_mode_config.max_height from DRM_SHADOW_PLANE_MAX_HEIGHT.
//

//
// struct drm_shadow_plane_state - plane state for planes with shadow buffers
//
// For planes that use a shadow buffer, struct drm_shadow_plane_state
// provides the regular plane state plus mappings of the shadow buffer
// into kernel address space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_shadow_plane_state {
// @base: plane state
    pub base: drm_plane_state,
//
// @fmtcnv_state: Format-conversion state
//
// Per-plane state for format conversion.
// Flags for copying shadow buffers into backend storage. Also holds
// temporary storage for format conversion.
//
    pub fmtcnv_state: drm_format_conv_state,
// Transitional state - do not export or duplicate
//
// @map: Mappings of the plane's framebuffer BOs in to kernel address space
//
// The memory mappings stored in map should be established in the plane's
// prepare_fb callback and removed in the cleanup_fb callback.
//
    pub map: [iosys_map; DRM_FORMAT_MAX_PLANES],
//
// @data: Address of each framebuffer BO's data
//
// The address of the data stored in each mapping. This is different
// for framebuffers with non-zero offset fields.
//
    pub data: [iosys_map; DRM_FORMAT_MAX_PLANES],
}

//
// to_drm_shadow_plane_state - upcasts from struct drm_plane_state
// @state: the plane state
//
extern "C" {
    pub fn container_of(_arg: state, drm_shadow_plane_state: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __drm_gem_destroy_shadow_plane_state(shadow_plane_state: *mut drm_shadow_plane_state);
}
extern "C" {
    pub fn drm_gem_reset_shadow_plane(plane: *mut drm_plane);
}
//
// DRM_GEM_SHADOW_PLANE_FUNCS -
// Initializes struct drm_plane_funcs for shadow-buffered planes
//
// Drivers may use GEM BOs as shadow buffers over the framebuffer memory. This
// macro initializes struct drm_plane_funcs to use the rsp helper functions.
//

extern "C" {
    pub fn drm_gem_begin_shadow_fb_access(plane: *mut drm_plane, plane_state: *mut drm_plane_state) -> c_int;
}
extern "C" {
    pub fn drm_gem_end_shadow_fb_access(plane: *mut drm_plane, plane_state: *mut drm_plane_state);
}
//
// DRM_GEM_SHADOW_PLANE_HELPER_FUNCS -
// Initializes struct drm_plane_helper_funcs for shadow-buffered planes
//
// Drivers may use GEM BOs as shadow buffers over the framebuffer memory. This
// macro initializes struct drm_plane_helper_funcs to use the rsp helper
// functions.
//

extern "C" {
    pub fn drm_gem_simple_kms_reset_shadow_plane(pipe: *mut drm_simple_display_pipe);
}
//
// DRM_GEM_SIMPLE_DISPLAY_PIPE_SHADOW_PLANE_FUNCS -
// Initializes struct drm_simple_display_pipe_funcs for shadow-buffered planes
//
// Drivers may use GEM BOs as shadow buffers over the framebuffer memory. This
// macro initializes struct drm_simple_display_pipe_funcs to use the rsp helper
// functions.
//

