//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/dc/dc-drv.h
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
// Copyright 2024 NXP
//

//
// struct dc_drm_device - DC specific drm_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_drm_device {
// @base: base drm_device structure
    pub base: drm_device,
// @dc_crtc: DC specific CRTC list
    pub dc_crtc: [dc_crtc; DC_DISPLAYS],
// @dc_primary: DC specific primary plane list
    pub dc_primary: [dc_plane; DC_DISPLAYS],
// @encoder: encoder list
    pub encoder: [drm_encoder; DC_DISPLAYS],
// @cf_safe: constframe list(safety stream)
    pub cf_safe: [*mut dc_cf; DC_DISPLAYS],
// @cf_cont: constframe list(content stream)
    pub cf_cont: [*mut dc_cf; DC_DISPLAYS],
// @de: display engine list
    pub de: [*mut dc_de; DC_DISPLAYS],
// @ed_safe: extdst list(safety stream)
    pub ed_safe: [*mut dc_ed; DC_DISPLAYS],
// @ed_cont: extdst list(content stream)
    pub ed_cont: [*mut dc_ed; DC_DISPLAYS],
// @fg: framegen list
    pub fg: [*mut dc_fg; DC_DISPLAYS],
// @fu_disp: fetchunit list(used by display engine)
    pub fu_disp: [*mut dc_fu; DC_DISP_FU_CNT],
// @lb: layerblend list
    pub lb: [*mut dc_lb; DC_LB_CNT],
// @pe: pixel engine
    pub pe: *mut dc_pe,
// @tc: tcon list
    pub tc: [*mut dc_tc; DC_DISPLAYS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_subdev_info {
    pub reg_start: resource_size_t,
    pub id: c_int,
}

extern "C" {
    pub fn container_of(_arg: drm, dc_drm_device: struct, _arg: base) -> return;
}
extern "C" {
    pub fn dc_crtc_init(dc_drm: *mut dc_drm_device, crtc_index: c_int) -> c_int;
}
extern "C" {
    pub fn dc_crtc_post_init(dc_drm: *mut dc_drm_device, crtc_index: c_int) -> c_int;
}
extern "C" {
    pub fn dc_kms_init(dc_drm: *mut dc_drm_device) -> c_int;
}
extern "C" {
    pub fn dc_kms_uninit(dc_drm: *mut dc_drm_device);
}
extern "C" {
    pub fn dc_plane_init(dc_drm: *mut dc_drm_device, dc_plane: *mut dc_plane) -> c_int;
}
extern "C" {
    pub fn dc_de_post_bind(dc_drm: *mut dc_drm_device);
}
extern "C" {
    pub fn dc_pe_post_bind(dc_drm: *mut dc_drm_device);
}
