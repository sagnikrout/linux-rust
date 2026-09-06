//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_du_vsp.h
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
// R-Car Display Unit VSP-Based Compositor
//
// Copyright (C) 2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_vsp_plane {
    pub plane: drm_plane,
    pub vsp: *mut rcar_du_vsp,
    pub index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_vsp {
    pub index: c_uint,
    pub vsp: *mut device,
    pub link: *mut device_link,
    pub dev: *mut rcar_du_device,
    pub planes: *mut rcar_du_vsp_plane,
    pub num_planes: c_uint,
}

extern "C" {
    pub fn container_of(_arg: p, rcar_du_vsp_plane: struct, _arg: plane) -> return;
}
//
// struct rcar_du_vsp_plane_state - Driver-specific plane state
// @state: base DRM plane state
// @format: information about the pixel format used by the plane
// @sg_tables: scatter-gather tables for the frame buffer memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_vsp_plane_state {
    pub state: drm_plane_state,
    pub format: *const rcar_du_format_info,
    pub sg_tables: [sg_table; 3],
}

extern "C" {
    pub fn container_of(_arg: state, rcar_du_vsp_plane_state: struct, _arg: state) -> return;
}

extern "C" {
    pub fn rcar_du_vsp_enable(crtc: *mut rcar_du_crtc);
}
extern "C" {
    pub fn rcar_du_vsp_disable(crtc: *mut rcar_du_crtc);
}
extern "C" {
    pub fn rcar_du_vsp_atomic_begin(crtc: *mut rcar_du_crtc);
}
extern "C" {
    pub fn rcar_du_vsp_atomic_flush(crtc: *mut rcar_du_crtc);
}

