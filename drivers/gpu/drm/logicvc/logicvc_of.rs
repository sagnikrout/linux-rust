//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/logicvc/logicvc_of.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum logicvc_of_property_index {
    LOGICVC_OF_PROPERTY_DISPLAY_INTERFACE = 0,
    LOGICVC_OF_PROPERTY_DISPLAY_COLORSPACE,
    LOGICVC_OF_PROPERTY_DISPLAY_DEPTH,
    LOGICVC_OF_PROPERTY_ROW_STRIDE,
    LOGICVC_OF_PROPERTY_DITHERING,
    LOGICVC_OF_PROPERTY_BACKGROUND_LAYER,
    LOGICVC_OF_PROPERTY_LAYERS_CONFIGURABLE,
    LOGICVC_OF_PROPERTY_LAYERS_COUNT,
    LOGICVC_OF_PROPERTY_LAYER_DEPTH,
    LOGICVC_OF_PROPERTY_LAYER_COLORSPACE,
    LOGICVC_OF_PROPERTY_LAYER_ALPHA_MODE,
    LOGICVC_OF_PROPERTY_LAYER_BASE_OFFSET,
    LOGICVC_OF_PROPERTY_LAYER_BUFFER_OFFSET,
    LOGICVC_OF_PROPERTY_LAYER_PRIMARY,
    LOGICVC_OF_PROPERTY_MAXIMUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_of_property_sv {
    pub string: *const c_char,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_of_property {
    pub name: *mut c_char,
    pub optional: bool,
    pub sv: *mut logicvc_of_property_sv,
    pub range: [u32; 2],
}

extern "C" {
    pub fn logicvc_of_node_is_layer(of_node: *mut device_node) -> bool;
}
