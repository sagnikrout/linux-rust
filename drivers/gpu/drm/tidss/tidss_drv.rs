//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tidss/tidss_drv.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

pub const TIDSS_MAX_PORTS: c_int = 4;
pub const TIDSS_MAX_PLANES: c_int = 4;
pub const TIDSS_MAX_OLDI_TXES: c_int = 2;
pub type dispc_irq_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tidss_device {
    pub /: *mut *mut drm_device ddev; / DRM device for DSS,
    pub /: *mut *mut *mut device dev; / Underlying DSS device,
    pub feat: *const dispc_features,
    pub dispc: *mut dispc_device,
    pub is_ext_vp_clk: [bool; TIDSS_MAX_PORTS],
    pub num_crtcs: c_uint,
    pub crtcs: [*mut drm_crtc; TIDSS_MAX_PORTS],
    pub num_planes: c_uint,
    pub planes: [*mut drm_plane; TIDSS_MAX_PLANES],
    pub num_oldis: c_uint,
    pub oldis: [*mut tidss_oldi; TIDSS_MAX_OLDI_TXES],
    pub irq: c_uint,
// protects the irq masks field and irqenable/irqstatus registers
    pub irq_lock: spinlock_t,
    pub /: *mut *mut dispc_irq_t irq_mask; / enabled irqs,
}

extern "C" {
    pub fn tidss_runtime_get(tidss: *mut tidss_device) -> c_int;
}
extern "C" {
    pub fn tidss_runtime_put(tidss: *mut tidss_device);
}
