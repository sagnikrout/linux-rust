//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/verisilicon/vs_plane.h
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
// Based on vs_dc_hw.h, which is:
// Copyright (C) 2023 VeriSilicon Holdings Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vs_color_format {
    VSDC_COLOR_FORMAT_X4R4G4B4,
    VSDC_COLOR_FORMAT_A4R4G4B4,
    VSDC_COLOR_FORMAT_X1R5G5B5,
    VSDC_COLOR_FORMAT_A1R5G5B5,
    VSDC_COLOR_FORMAT_R5G6B5,
    VSDC_COLOR_FORMAT_X8R8G8B8,
    VSDC_COLOR_FORMAT_A8R8G8B8,
    VSDC_COLOR_FORMAT_YUY2,
    VSDC_COLOR_FORMAT_UYVY,
    VSDC_COLOR_FORMAT_INDEX8,
    VSDC_COLOR_FORMAT_MONOCHROME,
    VSDC_COLOR_FORMAT_YV12 = 0xf,
    VSDC_COLOR_FORMAT_A8,
    VSDC_COLOR_FORMAT_NV12,
    VSDC_COLOR_FORMAT_NV16,
    VSDC_COLOR_FORMAT_RG16,
    VSDC_COLOR_FORMAT_R8,
    VSDC_COLOR_FORMAT_NV12_10BIT,
    VSDC_COLOR_FORMAT_A2R10G10B10,
    VSDC_COLOR_FORMAT_NV16_10BIT,
    VSDC_COLOR_FORMAT_INDEX1,
    VSDC_COLOR_FORMAT_INDEX2,
    VSDC_COLOR_FORMAT_INDEX4,
    VSDC_COLOR_FORMAT_P010,
    VSDC_COLOR_FORMAT_YUV444,
    VSDC_COLOR_FORMAT_YUV444_10BIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vs_swizzle {
    VSDC_SWIZZLE_ARGB,
    VSDC_SWIZZLE_RGBA,
    VSDC_SWIZZLE_ABGR,
    VSDC_SWIZZLE_BGRA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vs_format {
    pub color: vs_color_format,
    pub swizzle: vs_swizzle,
    pub uv_swizzle: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vs_plane_state {
    pub base: drm_plane_state,
    pub format: vs_format,
}

extern "C" {
    pub fn container_of(_arg: state, vs_plane_state: struct, _arg: base) -> return;
}
extern "C" {
    pub fn drm_format_to_vs_format(drm_format: u32, vs_format: *mut vs_format) -> c_int;
}
extern "C" {
    pub fn vs_plane_reset(plane: *mut drm_plane);
}
