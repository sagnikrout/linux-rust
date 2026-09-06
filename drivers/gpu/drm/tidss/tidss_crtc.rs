//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tidss/tidss_crtc.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tidss_crtc {
    pub crtc: drm_crtc,
    pub hw_videoport: u32,
    pub event: *mut drm_pending_vblank_event,
    pub framedone_completion: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tidss_crtc_state {
// Must be first.
    pub base: drm_crtc_state,
    pub plane_pos_changed: bool,
    pub bus_format: u32,
    pub bus_flags: u32,
}

extern "C" {
    pub fn tidss_crtc_vblank_irq(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn tidss_crtc_framedone_irq(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn tidss_crtc_error_irq(crtc: *mut drm_crtc, irqstatus: u64);
}
