//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/exynos_drm_drv.h
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
// exynos_drm_drv.h
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Authors:
// Inki Dae <inki.dae@samsung.com>
// Joonyoung Shim <jy0922.shim@samsung.com>
// Seung-Woo Kim <sw0312.kim@samsung.com>
//

pub const MAX_CRTC: c_int = 3;
pub const MAX_PLANE: c_int = 5;
pub const DEFAULT_WIN: c_int = 0;

// this enumerates display type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exynos_drm_output_type {
    EXYNOS_DISPLAY_TYPE_NONE,
// RGB or CPU Interface.
    EXYNOS_DISPLAY_TYPE_LCD,
// HDMI Interface.
    EXYNOS_DISPLAY_TYPE_HDMI,
// Virtual Display Interface.
    EXYNOS_DISPLAY_TYPE_VIDI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_rect {
    pub y: unsigned int x,,
    pub h: unsigned int w,,
}

//
// Exynos drm plane state structure.
//
// @base: plane_state object (contains drm_framebuffer pointer)
// @src: rectangle of the source image data to be displayed (clipped to
// visible part).
// @crtc: rectangle of the target image position on hardware screen
// (clipped to visible part).
// @h_ratio: horizontal scaling ratio, 16.16 fixed point
// @v_ratio: vertical scaling ratio, 16.16 fixed point
//
// this structure consists plane state data that will be applied to hardware
// specific overlay info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_plane_state {
    pub base: drm_plane_state,
    pub crtc: exynos_drm_rect,
    pub src: exynos_drm_rect,
    pub h_ratio: c_uint,
    pub v_ratio: c_uint,
}

extern "C" {
    pub fn container_of(_arg: state, exynos_drm_plane_state: struct, _arg: base) -> return;
}
//
// Exynos drm common overlay structure.
//
// @base: plane object
// @index: hardware index of the overlay layer
//
// this structure is common to exynos SoC and its contents would be copied
// to hardware specific overlay info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_plane {
    pub base: drm_plane,
    pub config: *const exynos_drm_plane_config,
    pub index: c_uint,
}

//
// Exynos DRM plane configuration structure.
//
// @zpos: initial z-position of the plane.
// @type: type of the plane (primary, cursor or overlay).
// @pixel_formats: supported pixel formats.
// @num_pixel_formats: number of elements in 'pixel_formats'.
// @capabilities: supported features (see EXYNOS_DRM_PLANE_CAP_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_plane_config {
    pub zpos: c_uint,
    pub type: drm_plane_type,
    pub pixel_formats: *const u32,
    pub num_pixel_formats: c_uint,
    pub capabilities: c_uint,
}

//
// Exynos drm crtc ops
//
// @atomic_enable: enable the device
// @atomic_disable: disable the device
// @enable_vblank: specific driver callback for enabling vblank interrupt.
// @disable_vblank: specific driver callback for disabling vblank interrupt.
// @mode_valid: specific driver callback for mode validation
// @atomic_check: validate state
// @atomic_begin: prepare device to receive an update
// @atomic_flush: mark the end of device update
// @update_plane: apply hardware specific overlay data to registers.
// @disable_plane: disable hardware specific overlay.
// @te_handler: trigger to transfer video image at the tearing effect
// synchronization signal if there is a page flip request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_crtc_ops {
    pub crtc): *mut *mut void (atomic_enable)(struct exynos_drm_crtc,
    pub crtc): *mut *mut void (atomic_disable)(struct exynos_drm_crtc,
    pub crtc): *mut *mut int (enable_vblank)(struct exynos_drm_crtc,
    pub crtc): *mut *mut void (disable_vblank)(struct exynos_drm_crtc,
    pub mode): *const drm_display_mode,
    pub adjusted_mode): *mut drm_display_mode,
    pub state): *mut drm_crtc_state,
    pub crtc): *mut *mut void (atomic_begin)(struct exynos_drm_crtc,
    pub plane): *mut exynos_drm_plane,
    pub plane): *mut exynos_drm_plane,
    pub crtc): *mut *mut void (atomic_flush)(struct exynos_drm_crtc,
    pub crtc): *mut *mut void (te_handler)(struct exynos_drm_crtc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_clk {
    pub enable): *mut *mut *mut void (enable)(struct exynos_drm_clk clk, bool,
}

//
// Exynos specific crtc structure.
//
// @base: crtc object.
// @type: one of EXYNOS_DISPLAY_TYPE_LCD and HDMI.
// @ops: pointer to callbacks for exynos drm specific functionality
// @ctx: A pointer to the crtc's implementation specific context
// @pipe_clk: A pointer to the crtc's pipeline clock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_crtc {
    pub base: drm_crtc,
    pub type: exynos_drm_output_type,
    pub ops: *const exynos_drm_crtc_ops,
    pub ctx: *mut c_void,
    pub pipe_clk: *mut exynos_drm_clk,
    pub 1: bool i80_mode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_file_private {
// for g2d api
    pub inuse_cmdlist: list_head,
    pub event_list: list_head,
    pub userptr_list: list_head,
}

//
// Exynos drm private structure.
//
// @pending: the crtcs that have pending updates to finish
// @lock: protect access to @pending
// @wait: wait an atomic commit to finish
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_private {
    pub g2d_dev: *mut device,
    pub vidi_dev: *mut device,
    pub mapping: *mut c_void,
// for atomic commit
    pub pending: u32,
    pub lock: spinlock_t,
    pub wait: wait_queue_head_t,
}

extern "C" {
    pub fn exynos_drm_cleanup_dma(drm: *mut drm_device);
}

extern "C" {
    pub fn exynos_dpi_remove(encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn exynos_dpi_bind(dev: *mut drm_device, encoder: *mut drm_encoder) -> c_int;
}

extern "C" {
    pub fn exynos_drm_check_fimc_device(dev: *mut device) -> c_int;
}

