//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_drm_drv.h
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
// Copyright (c) 2015 MediaTek Inc.
//

pub const MAX_CONNECTOR: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_crtc_path {
    CRTC_MAIN,
    CRTC_EXT,
    CRTC_THIRD,
    MAX_CRTC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_drm_route {
    pub crtc_id: c_uint,
    pub route_ddp: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mmsys_driver_data {
    pub main_path: *const c_uint,
    pub main_len: c_uint,
    pub ext_path: *const c_uint,
    pub ext_len: c_uint,
    pub third_path: *const c_uint,
    pub third_len: c_uint,
    pub conn_routes: *const mtk_drm_route,
    pub num_conn_routes: c_uint,
    pub shadow_register: bool,
    pub mmsys_id: c_uint,
    pub mmsys_dev_num: c_uint,
    pub max_width: u16,
    pub min_width: u16,
    pub min_height: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_drm_private {
    pub drm: *mut drm_device,
    pub mtk_drm_bound: bool,
    pub drm_master: bool,
    pub dev: *mut device,
    pub mutex_node: *mut device_node,
    pub mutex_dev: *mut device,
    pub mmsys_dev: *mut device,
    pub comp_node: [*mut device_node; DDP_COMPONENT_DRM_ID_MAX],
    pub ddp_comp: [mtk_ddp_comp; DDP_COMPONENT_DRM_ID_MAX],
    pub data: *mut mtk_mmsys_driver_data,
    pub suspend_state: *mut drm_atomic_commit,
    pub mbox_index: c_uint,
    pub all_drm_private: *mut mtk_drm_private,
}
