//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/verisilicon/vs_drm.h
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
// Copyright (C) 2025 Icenowy Zheng <uwu@icenowy.me>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vs_drm_dev {
    pub base: drm_device,
    pub dc: *mut vs_dc,
    pub crtcs: [*mut vs_crtc; VSDC_MAX_OUTPUTS],
}

extern "C" {
    pub fn vs_drm_initialize(dc: *mut vs_dc, pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn vs_drm_finalize(dc: *mut vs_dc);
}
extern "C" {
    pub fn vs_drm_shutdown_handler(dc: *mut vs_dc);
}
extern "C" {
    pub fn vs_drm_handle_irq(dc: *mut vs_dc, irqs: u32);
}
