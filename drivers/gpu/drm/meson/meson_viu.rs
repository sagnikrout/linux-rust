//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/meson/meson_viu.h
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
// Copyright (C) 2016 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
//
// Video Input Unit
// OSDx_BLKx_CFG

pub const OSD_CANVAS_SEL: c_int = 16;

// OSDx_CTRL_STAT

pub const OSD_GLOBAL_ALPHA_SHIFT: c_int = 12;
// OSDx_CTRL_STAT2

pub const OSD_REPLACE_SHIFT: c_int = 6;

extern "C" {
    pub fn meson_viu_osd1_reset(priv: *mut meson_drm);
}
extern "C" {
    pub fn meson_viu_g12a_enable_osd1_afbc(priv: *mut meson_drm);
}
extern "C" {
    pub fn meson_viu_g12a_disable_osd1_afbc(priv: *mut meson_drm);
}
extern "C" {
    pub fn meson_viu_gxm_enable_osd1_afbc(priv: *mut meson_drm);
}
extern "C" {
    pub fn meson_viu_gxm_disable_osd1_afbc(priv: *mut meson_drm);
}
extern "C" {
    pub fn meson_viu_init(priv: *mut meson_drm);
}
