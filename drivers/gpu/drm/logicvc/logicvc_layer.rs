//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/logicvc/logicvc_layer.h
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

pub const LOGICVC_LAYER_COLORSPACE_RGB: c_int = 0;
pub const LOGICVC_LAYER_COLORSPACE_YUV: c_int = 1;
pub const LOGICVC_LAYER_ALPHA_LAYER: c_int = 0;
pub const LOGICVC_LAYER_ALPHA_PIXEL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_layer_buffer_setup {
    pub buffer_sel: u8,
    pub voffset: u16,
    pub hoffset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_layer_config {
    pub colorspace: u32,
    pub depth: u32,
    pub alpha_mode: u32,
    pub base_offset: u32,
    pub buffer_offset: u32,
    pub primary: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_layer_formats {
    pub colorspace: u32,
    pub depth: u32,
    pub alpha: bool,
    pub formats: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_layer {
    pub config: logicvc_layer_config,
    pub formats: *mut logicvc_layer_formats,
    pub of_node: *mut device_node,
    pub drm_plane: drm_plane,
    pub list: list_head,
    pub index: u32,
}

extern "C" {
    pub fn logicvc_layers_attach_crtc(logicvc: *mut logicvc_drm);
}
extern "C" {
    pub fn logicvc_layers_init(logicvc: *mut logicvc_drm) -> c_int;
}
