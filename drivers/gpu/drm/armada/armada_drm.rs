//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/armada/armada_drm.h
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
// Copyright (C) 2012 Russell King
//

// 88AP510 spec recommends pitch be a multiple of 128
extern "C" {
    pub fn ALIGN(_arg: pitch, _arg: 128) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_variant {
    pub has_spu_adv_reg: bool,
    pub ): *mut *mut *mut int (init)(struct armada_crtc , struct device,
    pub ): *mut u32,
    pub ): *mut *mut void (disable)(struct armada_crtc,
    pub ): *const *const *const void (enable)(struct armada_crtc , struct drm_display_mode,
}

// Variant ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_private {
    pub drm: drm_device,
    pub dcrtc: [*mut armada_crtc; 2],
    pub /: *mut *mut drm_mm linear; / protected by linear_lock,
    pub linear_lock: mutex,
    pub colorkey_prop: *mut drm_property,
    pub colorkey_min_prop: *mut drm_property,
    pub colorkey_max_prop: *mut drm_property,
    pub colorkey_val_prop: *mut drm_property,
    pub colorkey_alpha_prop: *mut drm_property,
    pub colorkey_mode_prop: *mut drm_property,
    pub brightness_prop: *mut drm_property,
    pub contrast_prop: *mut drm_property,
    pub saturation_prop: *mut drm_property,

    pub de: *mut dentry,

}

extern "C" {
    pub fn armada_overlay_plane_create(: *mut drm_device, long: unsigned) -> c_int;
}
extern "C" {
    pub fn armada_drm_crtc_debugfs_init(dcrtc: *mut armada_crtc);
}
extern "C" {
    pub fn armada_drm_debugfs_init(: *mut drm_minor) -> c_int;
}
