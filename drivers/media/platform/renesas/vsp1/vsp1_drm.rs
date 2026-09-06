//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_drm.h
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
// vsp1_drm.h  --  R-Car VSP1 DRM/KMS Interface
//
// Copyright (C) 2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

//
// struct vsp1_drm_pipeline - State for the API exposed to the DRM driver
// @pipe: the VSP1 pipeline used for display
// @partition: the pre-calculated partition used by the pipeline
// @width: output display width
// @height: output display height
// @force_brx_release: when set, release the BRx during the next reconfiguration
// @wait_queue: wait queue to wait for BRx release completion
// @uif: UIF entity if available for the pipeline
// @crc: CRC computation configuration
// @du_complete: frame completion callback for the DU driver (optional)
// @du_private: data to be passed to the du_complete callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_drm_pipeline {
    pub pipe: vsp1_pipeline,
    pub partition: vsp1_partition,
    pub width: c_uint,
    pub height: c_uint,
    pub force_brx_release: bool,
    pub wait_queue: wait_queue_head_t,
    pub uif: *mut vsp1_entity,
    pub crc: vsp1_du_crc_config,
// Frame synchronisation
    pub crc): *mut *mut *mut void (du_complete)(void data, unsigned int status, u32,
    pub du_private: *mut c_void,
}

//
// struct vsp1_drm - State for the API exposed to the DRM driver
// @pipe: the VSP1 DRM pipeline used for display
// @lock: protects the BRU and BRS allocation
// @inputs: source crop rectangle, destination compose rectangle, z-order
// position and colorspace for every input (indexed by RPF index)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_drm {
    pub pipe: [vsp1_drm_pipeline; VSP1_MAX_LIF],
    pub lock: mutex,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_drm_input {
    pub crop: v4l2_rect,
    pub compose: v4l2_rect,
    pub zpos: c_uint,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub inputs: [}; VSP1_MAX_RPF],
}

extern "C" {
    pub fn container_of(_arg: pipe, vsp1_drm_pipeline: struct, _arg: pipe) -> return;
}
extern "C" {
    pub fn vsp1_drm_init(vsp1: *mut vsp1_device) -> c_int;
}
extern "C" {
    pub fn vsp1_drm_cleanup(vsp1: *mut vsp1_device);
}
