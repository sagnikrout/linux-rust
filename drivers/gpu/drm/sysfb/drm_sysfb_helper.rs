//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sysfb/drm_sysfb_helper.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// Input parsing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sysfb_format {
    pub pixel: pixel_format,
    pub fourcc: u32,
}

extern "C" {
    pub fn drm_sysfb_get_width_si(dev: *mut drm_device, si: *const screen_info) -> c_int;
}
extern "C" {
    pub fn drm_sysfb_get_height_si(dev: *mut drm_device, si: *const screen_info) -> c_int;
}

//
// Display modes
//
// Device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sysfb_device {
    pub dev: drm_device,
    pub /: *const *const *const u8 edid; / can be NULL,
// hardware settings
    pub fb_mode: drm_display_mode,
    pub fb_format: *const drm_format_info,
    pub fb_pitch: c_uint,
    pub fb_gamma_lut_size: c_uint,
// hardware-framebuffer kernel address
    pub fb_addr: iosys_map,
}

extern "C" {
    pub fn container_of(_arg: dev, drm_sysfb_device: struct, _arg: dev) -> return;
}
//
// Plane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sysfb_plane_state {
    pub base: drm_shadow_plane_state,
// transfers framebuffer data to scanout buffer in CRTC format
    pub blit_to_crtc: drm_sysfb_blit_func,
}

extern "C" {
    pub fn container_of(_arg: to_drm_shadow_plane_state(base), drm_sysfb_plane_state: struct, _arg: base) -> return;
}

extern "C" {
    pub fn drm_sysfb_plane_reset(plane: *mut drm_plane);
}

//
// CRTC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sysfb_crtc_state {
    pub base: drm_crtc_state,
// CRTC input color format; required for color mgmt.
    pub format: *const drm_format_info,
}

extern "C" {
    pub fn container_of(_arg: base, drm_sysfb_crtc_state: struct, _arg: base) -> return;
}
extern "C" {
    pub fn drm_sysfb_crtc_helper_atomic_check(crtc: *mut drm_crtc, new_state: *mut drm_atomic_commit) -> c_int;
}

extern "C" {
    pub fn drm_sysfb_crtc_reset(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn drm_sysfb_crtc_atomic_destroy_state(crtc: *mut drm_crtc, crtc_state: *mut drm_crtc_state);
}

//
// Connector
//
extern "C" {
    pub fn drm_sysfb_connector_helper_get_modes(connector: *mut drm_connector) -> c_int;
}

//
// Mode config
//

