//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/meson/meson_venc.h
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
// Video Encoders
// - ENCI : Interlace Video Encoder
// - ENCI_DVI : Interlace Video Encoder for DVI/HDMI
// - ENCP : Progressive Video Encoder
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_cvbs_enci_mode {
    pub mode_tag: c_uint,
    pub /: *mut *mut unsigned int hso_begin; / HSO begin position,
    pub /: *mut *mut unsigned int hso_end; / HSO end position,
    pub /: *mut *mut unsigned int vso_even; / VSO even line,
    pub /: *mut *mut unsigned int vso_odd; / VSO odd line,
    pub /: *mut *mut unsigned int macv_max_amp; / Macrovision max amplitude,
    pub video_prog_mode: c_uint,
    pub video_mode: c_uint,
    pub sch_adjust: c_uint,
    pub yc_delay: c_uint,
    pub pixel_start: c_uint,
    pub pixel_end: c_uint,
    pub top_field_line_start: c_uint,
    pub top_field_line_end: c_uint,
    pub bottom_field_line_start: c_uint,
    pub bottom_field_line_end: c_uint,
    pub video_saturation: c_uint,
    pub video_contrast: c_uint,
    pub video_brightness: c_uint,
    pub video_hue: c_uint,
    pub analog_sync_adj: c_uint,
}

// LCD Encoder gamma setup
extern "C" {
    pub fn meson_encl_load_gamma(priv: *mut meson_drm);
}
// HDMI Clock parameters
extern "C" {
    pub fn meson_venc_hdmi_supported_vic(vic: c_int) -> bool;
}
extern "C" {
    pub fn meson_venc_hdmi_venc_repeat(vic: c_int) -> bool;
}
// CVBS Timings and Parameters
extern "C" {
    pub fn meson_venci_get_field(priv: *mut meson_drm) -> c_uint;
}
extern "C" {
    pub fn meson_venc_enable_vsync(priv: *mut meson_drm);
}
extern "C" {
    pub fn meson_venc_disable_vsync(priv: *mut meson_drm);
}
extern "C" {
    pub fn meson_venc_init(priv: *mut meson_drm);
}
