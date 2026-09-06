//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/rockchip_drm_drv.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author:Mark Yao <mark.yao@rock-chips.com>
//
// based on exynos_drm_drv.h
//

pub const ROCKCHIP_MAX_FB_BUFFER: c_int = 3;
pub const ROCKCHIP_MAX_CONNECTOR: c_int = 2;
pub const ROCKCHIP_MAX_CRTC: c_int = 4;
//
// display output interface supported by rockchip lcdc
//
pub const ROCKCHIP_OUT_MODE_P888: c_int = 0;
pub const ROCKCHIP_OUT_MODE_BT1120: c_int = 0;
pub const ROCKCHIP_OUT_MODE_P666: c_int = 1;
pub const ROCKCHIP_OUT_MODE_P565: c_int = 2;
pub const ROCKCHIP_OUT_MODE_BT656: c_int = 5;
pub const ROCKCHIP_OUT_MODE_S888: c_int = 8;
pub const ROCKCHIP_OUT_MODE_YUV422: c_int = 9;
pub const ROCKCHIP_OUT_MODE_S888_DUMMY: c_int = 12;
pub const ROCKCHIP_OUT_MODE_YUV420: c_int = 14;
// for use special outface
pub const ROCKCHIP_OUT_MODE_AAAA: c_int = 15;
// SoC specific output modes
pub const ROCKCHIP_OUT_MODE_YUV422_RK3576_DP: c_int = 12;
pub const ROCKCHIP_OUT_MODE_YUV422_RK3576_HDMI: c_int = 13;
// output flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_crtc_state {
    pub base: drm_crtc_state,
    pub output_type: c_int,
    pub output_mode: c_int,
    pub output_bpc: c_int,
    pub output_flags: c_int,
    pub enable_afbc: bool,
    pub yuv_overlay: bool,
    pub bus_format: u32,
    pub bus_flags: u32,
    pub color_space: c_int,
}

//
// Rockchip drm private structure.
//
// @crtc: array of enabled CRTCs, used to map from "pipe" to drm_crtc.
// @num_pipe: number of pipes for this device.
// @mm_lock: protect drm_mm on multi-threads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_drm_private {
    pub domain: *mut iommu_domain,
    pub iommu_dev: *mut device,
    pub mm_lock: mutex,
    pub mm: drm_mm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_encoder {
    pub crtc_endpoint_id: c_int,
    pub encoder: drm_encoder,
}

extern "C" {
    pub fn rockchip_drm_wait_vact_end(crtc: *mut drm_crtc, mstimeout: c_uint) -> c_int;
}
extern "C" {
    pub fn rockchip_drm_endpoint_is_subdriver(ep: *mut device_node) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: encoder, rockchip_encoder: struct, _arg: encoder) -> return;
}
