//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xlnx/zynqmp_kms.h
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
// ZynqMP DisplayPort Subsystem - KMS API
//
// Copyright (C) 2017 - 2021 Xilinx, Inc.
//
// Authors:
// - Hyun Woo Kwon <hyun.kwon@xilinx.com>
// - Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

//
// struct zynqmp_dpsub_drm - ZynqMP DisplayPort Subsystem DRM/KMS data
// @dpsub: Backpointer to the DisplayPort subsystem
// @dev: The DRM/KMS device
// @planes: The DRM planes
// @crtc: The DRM CRTC
// @encoder: The dummy DRM encoder
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_dpsub_drm {
    pub dpsub: *mut zynqmp_dpsub,
    pub dev: drm_device,
    pub planes: [drm_plane; ZYNQMP_DPSUB_NUM_LAYERS],
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
}

extern "C" {
    pub fn zynqmp_dpsub_drm_handle_vblank(dpsub: *mut zynqmp_dpsub);
}
extern "C" {
    pub fn zynqmp_dpsub_drm_init(dpsub: *mut zynqmp_dpsub) -> c_int;
}
extern "C" {
    pub fn zynqmp_dpsub_drm_cleanup(dpsub: *mut zynqmp_dpsub);
}
