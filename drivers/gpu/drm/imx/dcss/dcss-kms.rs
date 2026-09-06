//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/dcss/dcss-kms.h
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
// Copyright 2019 NXP.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_plane {
    pub base: drm_plane,
    pub ch_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_crtc {
    pub base: drm_crtc,
    pub state: *mut drm_crtc_state,
    pub plane: [*mut dcss_plane; 3],
    pub irq: c_int,
    pub disable_ctxld_kick_irq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_kms_dev {
    pub base: drm_device,
    pub crtc: dcss_crtc,
    pub encoder: drm_encoder,
    pub connector: *mut drm_connector,
}

extern "C" {
    pub fn dcss_kms_detach(kms: *mut dcss_kms_dev);
}
extern "C" {
    pub fn dcss_kms_shutdown(kms: *mut dcss_kms_dev);
}
extern "C" {
    pub fn dcss_crtc_init(crtc: *mut dcss_crtc, drm: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn dcss_crtc_deinit(crtc: *mut dcss_crtc, drm: *mut drm_device);
}
