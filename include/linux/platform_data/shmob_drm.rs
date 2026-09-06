//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/shmob_drm.h
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
// shmob_drm.h  --  SH Mobile DRM driver
//
// Copyright (C) 2012 Renesas Corporation
//
// Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shmob_drm_clk_source {
    SHMOB_DRM_CLK_BUS,
    SHMOB_DRM_CLK_PERIPHERAL,
    SHMOB_DRM_CLK_EXTERNAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmob_drm_panel_data {
    pub /: *mut *mut unsigned int width_mm; / Panel width in mm,
    pub /: *mut *mut unsigned int height_mm; / Panel height in mm,
    pub mode: videomode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmob_drm_interface_data {
    pub /: *mut *mut *mut unsigned int bus_fmt; / MEDIA_BUS_FMT_,
    pub clk_div: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmob_drm_platform_data {
    pub clk_source: shmob_drm_clk_source,
    pub iface: shmob_drm_interface_data,
    pub panel: shmob_drm_panel_data,
}
