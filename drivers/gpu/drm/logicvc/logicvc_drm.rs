//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/logicvc/logicvc_drm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2019-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

pub const LOGICVC_DISPLAY_INTERFACE_RGB: c_int = 0;
pub const LOGICVC_DISPLAY_INTERFACE_ITU656: c_int = 1;
pub const LOGICVC_DISPLAY_INTERFACE_LVDS_4BITS: c_int = 2;
pub const LOGICVC_DISPLAY_INTERFACE_LVDS_4BITS_CAMERA: c_int = 3;
pub const LOGICVC_DISPLAY_INTERFACE_LVDS_3BITS: c_int = 4;
pub const LOGICVC_DISPLAY_INTERFACE_DVI: c_int = 5;
pub const LOGICVC_DISPLAY_COLORSPACE_RGB: c_int = 0;
pub const LOGICVC_DISPLAY_COLORSPACE_YUV422: c_int = 1;
pub const LOGICVC_DISPLAY_COLORSPACE_YUV444: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_drm_config {
    pub display_interface: u32,
    pub display_colorspace: u32,
    pub display_depth: u32,
    pub row_stride: u32,
    pub dithering: bool,
    pub background_layer: bool,
    pub layers_configurable: bool,
    pub layers_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_drm_caps {
    pub major: c_uint,
    pub minor: c_uint,
    pub level: c_char,
    pub layer_address: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_drm {
    pub caps: *const logicvc_drm_caps,
    pub config: logicvc_drm_config,
    pub drm_dev: drm_device,
    pub reserved_mem_base: phys_addr_t,
    pub regmap: *mut regmap,
    pub vclk: *mut clk,
    pub vclk2: *mut clk,
    pub lvdsclk: *mut clk,
    pub lvdsclkn: *mut clk,
    pub layers_list: list_head,
    pub crtc: *mut logicvc_crtc,
    pub interface: *mut logicvc_interface,
}
