//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_format_caps.h
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
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

// afbc layerout

// afbc features

// layer_type

pub const AFBC_TH_LAYOUT_ALIGNMENT: c_int = 8;
pub const AFBC_HEADER_SIZE: c_int = 16;
pub const AFBC_SUPERBLK_ALIGNMENT: c_int = 128;
pub const AFBC_SUPERBLK_PIXELS: c_int = 256;
pub const AFBC_BODY_START_ALIGNMENT: c_int = 1024;
pub const AFBC_TH_BODY_START_ALIGNMENT: c_int = 4096;
//
// struct komeda_format_caps
//
// komeda_format_caps is for describing ARM display specific features and
// limitations for a specific format, and format_caps will be linked into
// &komeda_framebuffer like a extension of &drm_format_info.
//
// NOTE: one fourcc may has two different format_caps items for fourcc and
// fourcc+modifier
//
// @hw_id: hw format id, hw specific value.
// @fourcc: drm fourcc format.
// @supported_layer_types: indicate which layer supports this format
// @supported_rots: allowed rotations for this format
// @supported_afbc_layouts: supported afbc layerout
// @supported_afbc_features: supported afbc features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_format_caps {
    pub hw_id: u32,
    pub fourcc: u32,
    pub supported_layer_types: u32,
    pub supported_rots: u32,
    pub supported_afbc_layouts: u32,
    pub supported_afbc_features: u64,
}

//
// struct komeda_format_caps_table - format_caps mananger
//
// @n_formats: the size of format_caps list.
// @format_caps: format_caps list.
// @format_mod_supported: Optional. Some HW may have special requirements or
// limitations which can not be described by format_caps, this func supply HW
// the ability to do the further HW specific check.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_format_caps_table {
    pub n_formats: u32,
    pub format_caps: *const komeda_format_caps,
    pub rot): u32 layer_type, u64 modifier, u32,
}

extern "C" {
    pub fn komeda_put_fourcc_list(fourcc_list: *mut u32);
}
