//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/omap_drv.h
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
// Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com
// Author: Rob Clark <rob@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_drm_pipeline {
    pub crtc: *mut drm_crtc,
    pub encoder: *mut drm_encoder,
    pub connector: *mut drm_connector,
    pub output: *mut omap_dss_device,
    pub alias_id: c_uint,
}

//
// Global private object state for tracking resources that are shared across
// multiple kms objects (planes/crtcs/etc).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_global_state {
    pub base: drm_private_state,
// global atomic state of assignment between overlays and planes
    pub hwoverlay_to_plane: [*mut drm_plane; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_drm_private {
    pub ddev: *mut drm_device,
    pub dev: *mut device,
    pub omaprev: u32,
    pub dss: *mut dss_device,
    pub dispc: *mut dispc_device,
    pub irq_enabled: bool,
    pub num_pipes: c_uint,
    pub pipes: [omap_drm_pipeline; 8],
    pub channels: [*mut omap_drm_pipeline; 8],
    pub num_planes: c_uint,
    pub planes: [*mut drm_plane; 8],
    pub num_ovls: c_uint,
    pub overlays: [*mut omap_hw_overlay; 8],
    pub glob_obj: drm_private_obj,
    pub wq: *mut workqueue_struct,
// lock for obj_list below
    pub list_lock: mutex,
// list of GEM objects:
    pub obj_list: list_head,
    pub usergart: *mut omap_drm_usergart,
    pub has_dmm: bool,
// properties:
    pub zorder_prop: *mut drm_property,
// irq handling:
    pub /: *mut *mut spinlock_t wait_lock; / protects the wait_list,
    pub /: *mut *mut list_head wait_list; / list of omap_irq_wait,
    pub /: *mut *mut u32 irq_mask; / enabled irqs in addition to wait_list,
// memory bandwidth limit if it is needed on the platform
    pub max_bandwidth: c_uint,
    pub fbdev: *mut omap_fbdev,
}

extern "C" {
    pub fn omap_debugfs_init(minor: *mut drm_minor);
}
extern "C" {
    pub fn omap_get_global_state(s: *mut drm_atomic_commit) -> *mut omap_global_state  __must_check;
}
