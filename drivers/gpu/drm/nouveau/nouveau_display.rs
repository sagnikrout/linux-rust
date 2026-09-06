//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_display.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_display {
    pub priv: *mut c_void,
    pub ): *mut *mut void (dtor)(struct drm_device,
    pub runtime): *mut *mut *mut int (init)(struct drm_device , bool resume, bool,
    pub runtime): *mut *mut *mut void (fini)(struct drm_device , bool suspend, bool,
    pub disp: nvif_disp,
    pub dithering_mode: *mut drm_property,
    pub dithering_depth: *mut drm_property,
    pub underscan_property: *mut drm_property,
    pub underscan_hborder_property: *mut drm_property,
    pub underscan_vborder_property: *mut drm_property,
// not really hue and saturation:
    pub vibrant_hue_property: *mut drm_property,
    pub color_vibrance_property: *mut drm_property,
    pub suspend: *mut drm_atomic_commit,
    pub format_modifiers: *const u64,
}

extern "C" {
    pub fn nouveau_display_create(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn nouveau_display_destroy(dev: *mut drm_device);
}
extern "C" {
    pub fn nouveau_display_init(dev: *mut drm_device, resume: bool, runtime: bool) -> c_int;
}
extern "C" {
    pub fn nouveau_display_hpd_resume(: *mut nouveau_drm);
}
extern "C" {
    pub fn nouveau_display_fini(dev: *mut drm_device, suspend: bool, runtime: bool);
}
extern "C" {
    pub fn nouveau_display_suspend(dev: *mut drm_device, runtime: bool) -> c_int;
}
extern "C" {
    pub fn nouveau_display_resume(dev: *mut drm_device, runtime: bool);
}
extern "C" {
    pub fn nouveau_display_vblank_enable(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn nouveau_display_vblank_disable(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn nouveau_hdmi_mode_set(: *mut drm_encoder, : *mut drm_display_mode);
}
