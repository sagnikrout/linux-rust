//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/omap_crtc.h
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
// omap_crtc.h -- OMAP DRM CRTC
//
// Copyright (C) 2011 Texas Instruments
// Author: Rob Clark <rob@ti.com>
//

extern "C" {
    pub fn omap_crtc_channel(crtc: *mut drm_crtc) -> omap_channel;
}
extern "C" {
    pub fn omap_crtc_wait_pending(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn omap_crtc_error_irq(crtc: *mut drm_crtc, irqstatus: u32);
}
extern "C" {
    pub fn omap_crtc_vblank_irq(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn omap_crtc_framedone_irq(crtc: *mut drm_crtc, irqstatus: u32);
}
extern "C" {
    pub fn omap_crtc_flush(crtc: *mut drm_crtc);
}
