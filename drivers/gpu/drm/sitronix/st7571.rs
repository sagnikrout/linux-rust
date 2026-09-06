//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sitronix/st7571.h
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
// Header file for:
// Driver for Sitronix ST7571, a 4 level gray scale dot matrix LCD controller
//
// Copyright (C) 2025 Marcus Folkesson <marcus.folkesson@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st7571_color_mode {
    ST7571_COLOR_MODE_GRAY = 0,
    ST7571_COLOR_MODE_BLACKWHITE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st7571_panel_constraints {
    pub min_nlines: u32,
    pub max_nlines: u32,
    pub min_ncols: u32,
    pub max_ncols: u32,
    pub support_grayscale: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st7571_panel_data {
    pub st7571): *mut *mut int (init)(struct st7571_device,
    pub st7571): *mut *mut int (parse_dt)(struct st7571_device,
    pub constraints: st7571_panel_constraints,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st7571_panel_format {
    pub fmtcnv_state): *mut drm_format_conv_state,
    pub rect): *mut *mut *mut int (update_rect)(struct drm_framebuffer fb, struct drm_rect,
    pub mode: st7571_color_mode,
    pub nformats: u8,
    pub formats: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st7571_device {
    pub drm: drm_device,
    pub dev: *mut device,
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub mode: drm_display_mode,
    pub pformat: *const st7571_panel_format,
    pub pdata: *const st7571_panel_data,
    pub reset: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub grayscale: bool,
    pub inverted: bool,
    pub height_mm: u32,
    pub width_mm: u32,
    pub startline: u32,
    pub nlines: u32,
    pub ncols: u32,
    pub bpp: u32,
// Intermediate buffer in LCD friendly format
    pub hwbuf: *mut u8,
// Row of (transformed) pixels ready to be written to the display
    pub row: *mut u8,
}

extern "C" {
    pub fn st7571_remove(st7571: *mut st7571_device);
}
