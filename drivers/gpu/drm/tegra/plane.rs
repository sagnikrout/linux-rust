//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/plane.h
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
// Copyright (C) 2017 NVIDIA CORPORATION.  All rights reserved.
//
pub const TEGRA_PLANE_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_plane {
    pub base: drm_plane,
    pub dc: *mut tegra_dc,
    pub offset: c_uint,
    pub index: c_uint,
    pub icc_mem: *mut icc_path,
    pub icc_mem_vfilter: *mut icc_path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cursor {
    pub base: tegra_plane,
    pub bo: *mut tegra_bo,
    pub width: c_uint,
    pub height: c_uint,
}

extern "C" {
    pub fn container_of(_arg: plane, tegra_plane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_plane_legacy_blending_state {
    pub alpha: bool,
    pub top: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_plane_state {
    pub base: drm_plane_state,
    pub map: [*mut host1x_bo_mapping; 3],
    pub iova: [dma_addr_t; 3],
    pub tiling: tegra_bo_tiling,
    pub format: u32,
    pub swap: u32,
    pub reflect_x: bool,
    pub reflect_y: bool,
// used for legacy blending support only
    pub blending: [tegra_plane_legacy_blending_state; 2],
    pub opaque: bool,
// bandwidths are in ICC units, i.e. kbytes/sec
    pub total_peak_memory_bandwidth: u32,
    pub peak_memory_bandwidth: u32,
    pub avg_memory_bandwidth: u32,
}

extern "C" {
    pub fn container_of(_arg: state, tegra_plane_state: struct, _arg: base) -> return;
}
extern "C" {
    pub fn to_tegra_plane_state()state: *mut (struct drm_plane_state) -> return;
}
extern "C" {
    pub fn tegra_plane_format(fourcc: u32, format: *mut u32, swap: *mut u32) -> c_int;
}
extern "C" {
    pub fn tegra_plane_format_is_indexed(format: c_uint) -> bool;
}
extern "C" {
    pub fn tegra_plane_format_is_yuv(format: c_uint, planes: *mut c_uint, bpc: *mut c_uint) -> bool;
}
extern "C" {
    pub fn tegra_plane_interconnect_init(plane: *mut tegra_plane) -> c_int;
}
