//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rz-du/rzg2l_du_vsp.h
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
// RZ/G2L Display Unit VSP-Based Compositor
//
// Copyright (C) 2023 Renesas Electronics Corporation
//
// Based on rcar_du_vsp.h
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_vsp_plane {
    pub plane: drm_plane,
    pub vsp: *mut rzg2l_du_vsp,
    pub index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_vsp {
    pub index: c_uint,
    pub vsp: *mut device,
    pub link: *mut device_link,
    pub dev: *mut rzg2l_du_device,
}

extern "C" {
    pub fn container_of(_arg: p, rzg2l_du_vsp_plane: struct, _arg: plane) -> return;
}
//
// struct rzg2l_du_vsp_plane_state - Driver-specific plane state
// @state: base DRM plane state
// @format: information about the pixel format used by the plane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_vsp_plane_state {
    pub state: drm_plane_state,
    pub format: *const rzg2l_du_format_info,
}

extern "C" {
    pub fn container_of(_arg: state, rzg2l_du_vsp_plane_state: struct, _arg: state) -> return;
}

extern "C" {
    pub fn rzg2l_du_vsp_enable(crtc: *mut rzg2l_du_crtc);
}
extern "C" {
    pub fn rzg2l_du_vsp_disable(crtc: *mut rzg2l_du_crtc);
}
extern "C" {
    pub fn rzg2l_du_vsp_atomic_flush(crtc: *mut rzg2l_du_crtc);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}

