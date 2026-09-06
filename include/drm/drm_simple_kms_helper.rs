//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_simple_kms_helper.h
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
// Copyright (C) 2016 Noralf Trønnes
//
// Simple KMS helpers are deprected in favor of regular atomic helpers. Do not
// use the min new code.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_simple_display_pipe_funcs {
    pub mode): *const drm_display_mode,
    pub plane_state): *mut drm_plane_state,
    pub pipe): *mut *mut void (disable)(struct drm_simple_display_pipe,
    pub crtc_state): *mut drm_crtc_state,
    pub old_plane_state): *mut drm_plane_state,
    pub plane_state): *mut drm_plane_state,
    pub plane_state): *mut drm_plane_state,
    pub new_plane_state): *mut drm_plane_state,
    pub plane_state): *mut drm_plane_state,
    pub pipe): *mut *mut int (enable_vblank)(struct drm_simple_display_pipe,
    pub pipe): *mut *mut void (disable_vblank)(struct drm_simple_display_pipe,
    pub pipe): *mut *mut void (reset_crtc)(struct drm_simple_display_pipe,
    pub pipe): *mut *mut *mut drm_crtc_state  (duplicate_crtc_state)(drm_simple_display_pipe,
    pub crtc_state): *mut drm_crtc_state,
    pub pipe): *mut *mut void (reset_plane)(struct drm_simple_display_pipe,
    pub pipe): *mut *mut *mut drm_plane_state  (duplicate_plane_state)(drm_simple_display_pipe,
    pub plane_state): *mut drm_plane_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_simple_display_pipe {
    pub crtc: drm_crtc,
    pub plane: drm_plane,
    pub encoder: drm_encoder,
    pub connector: *mut drm_connector,
    pub funcs: *const drm_simple_display_pipe_funcs,
}

