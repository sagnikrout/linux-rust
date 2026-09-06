//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_du_plane.h
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
// R-Car Display Unit Planes
//
// Copyright (C) 2013-2014 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

//
// The R-Car DU has 8 hardware planes, shared between primary and overlay planes.
// As using overlay planes requires at least one of the CRTCs being enabled, no
// more than 7 overlay planes can be available. We thus create 1 primary plane
// per CRTC and 7 overlay planes, for a total of up to 9 KMS planes.
//
pub const RCAR_DU_NUM_KMS_PLANES: c_int = 9;
pub const RCAR_DU_NUM_HW_PLANES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcar_du_plane_source {
    RCAR_DU_PLANE_MEMORY,
    RCAR_DU_PLANE_VSPD0,
    RCAR_DU_PLANE_VSPD1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_plane {
    pub plane: drm_plane,
    pub group: *mut rcar_du_group,
}

extern "C" {
    pub fn container_of(_arg: plane, rcar_du_plane: struct, _arg: plane) -> return;
}
//
// struct rcar_du_plane_state - Driver-specific plane state
// @state: base DRM plane state
// @format: information about the pixel format used by the plane
// @hwindex: 0-based hardware plane index, -1 means unused
// @colorkey: value of the plane colorkey property
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_plane_state {
    pub state: drm_plane_state,
    pub format: *const rcar_du_format_info,
    pub hwindex: c_int,
    pub source: rcar_du_plane_source,
    pub colorkey: c_uint,
}

extern "C" {
    pub fn container_of(_arg: state, rcar_du_plane_state: struct, _arg: state) -> return;
}
extern "C" {
    pub fn rcar_du_planes_init(rgrp: *mut rcar_du_group) -> c_int;
}
extern "C" {
    pub fn __rcar_du_plane_setup(_arg: plane->group, _arg: state) -> return;
}
