//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tilcdc/tilcdc_drv.h
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
// Copyright (C) 2012 Texas Instruments
// Author: Rob Clark <robdclark@gmail.com>
//

// Defaulting to pixel clock defined on AM335x
pub const TILCDC_DEFAULT_MAX_PIXELCLOCK: c_int = 126000;
// Maximum display width for LCDC V1
pub const TILCDC_DEFAULT_MAX_WIDTH_V1: c_int = 1024;
// ... and for LCDC V2 found on AM335x:
pub const TILCDC_DEFAULT_MAX_WIDTH_V2: c_int = 2048;
//
// This may need some tweaking, but want to allow at least 1280x1024@60
// with optimized DDR & EMIF settings tweaked 1920x1080@24 appears to
// be supportable
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tilcdc_drm_private {
    pub mmio: *mut void __iomem,
    pub /: *mut *mut *mut clk clk; / functional clock,
    pub /: *mut *mut int rev; / IP revision,
    pub irq: c_uint,
    pub ddev: drm_device,
// don't attempt resolutions w/ higher W * H * Hz:
    pub max_bandwidth: u32,
//
// Pixel Clock will be restricted to some value as
// defined in the device datasheet measured in KHz
//
    pub max_pixelclock: u32,
//
// Max allowable width is limited on a per device basis
// measured in pixels
//
    pub max_width: u32,
    pub fifo_th: u32,
// Supported pixel formats
    pub pixelformats: *const u32,
    pub num_pixelformats: u32,

    pub freq_transition: notifier_block,

    pub wq: *mut workqueue_struct,
    pub crtc: *mut drm_crtc,
    pub encoder: *mut tilcdc_encoder,
    pub connector: *mut drm_connector,
    pub irq_enabled: bool,
}

extern "C" {
    pub fn tilcdc_crtc_create(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn tilcdc_crtc_irq(crtc: *mut drm_crtc) -> irqreturn_t;
}
extern "C" {
    pub fn tilcdc_crtc_update_clk(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn tilcdc_crtc_shutdown(crtc: *mut drm_crtc);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tilcdc_plane {
    pub base: drm_plane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tilcdc_encoder {
    pub base: drm_encoder,
}
