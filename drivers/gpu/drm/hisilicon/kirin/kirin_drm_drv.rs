//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/kirin/kirin_drm_drv.h
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
// Copyright (c) 2016 Linaro Limited.
// Copyright (c) 2014-2016 HiSilicon Limited.
//

// kirin-format translate table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kirin_format {
    pub pixel_format: u32,
    pub hw_format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kirin_crtc {
    pub base: drm_crtc,
    pub hw_ctx: *mut c_void,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kirin_plane {
    pub base: drm_plane,
    pub hw_ctx: *mut c_void,
    pub ch: u32,
}

// display controller init/cleanup ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kirin_drm_data {
    pub channel_formats: *const u32,
    pub channel_formats_cnt: u32,
    pub config_max_width: c_int,
    pub config_max_height: c_int,
    pub num_planes: u32,
    pub prim_plane: u32,
    pub driver: *const drm_driver,
    pub crtc_helper_funcs: *const drm_crtc_helper_funcs,
    pub crtc_funcs: *const drm_crtc_funcs,
    pub plane_helper_funcs: *const drm_plane_helper_funcs,
    pub plane_funcs: *const drm_plane_funcs,
    pub mode_config_funcs: *const drm_mode_config_funcs,
    pub crtc): *mut drm_crtc,
    pub hw_ctx): *mut *mut void (cleanup_hw_ctx)(void,
}
