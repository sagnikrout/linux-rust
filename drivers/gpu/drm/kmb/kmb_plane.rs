//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/kmb/kmb_plane.h
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
// Copyright © 2018-2020 Intel Corporation
//

pub const POSSIBLE_CRTCS: c_int = 1;

pub const POSSIBLE_CRTCS: c_int = 1;
pub const KMB_MAX_PLANES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum layer_id {
    LAYER_0,
    LAYER_1,
    LAYER_2,
    LAYER_3,
// KMB_MAX_PLANES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sub_plane_id {
    Y_PLANE,
    U_PLANE,
    V_PLANE,
    MAX_SUB_PLANES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmb_plane {
    pub base_plane: drm_plane,
    pub id: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layer_status {
    pub disable: bool,
    pub ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disp_cfg {
    pub width: c_uint,
    pub height: c_uint,
    pub format: c_uint,
}

extern "C" {
    pub fn kmb_plane_destroy(plane: *mut drm_plane);
}
