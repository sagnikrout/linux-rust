//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_cursor_plane.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (c) 2024-2025 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_cursor_update_type {
    VMW_CURSOR_UPDATE_NONE = 0,
    VMW_CURSOR_UPDATE_LEGACY,
    VMW_CURSOR_UPDATE_GB_ONLY,
    VMW_CURSOR_UPDATE_MOB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_cursor_plane_state {
    pub update_type: vmw_cursor_update_type,
    pub changed: bool,
    pub surface_changed: bool,
    pub mob: *mut vmw_bo,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
    pub id: u32,
    pub legacy: },
}

//
// Derived class for cursor plane object
//
// @base DRM plane object
// @cursor.cursor_mobs Cursor mobs available for re-use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_cursor_plane {
    pub base: drm_plane,
    pub cursor_mobs: [*mut vmw_bo; 3],
}

extern "C" {
    pub fn vmw_cursor_plane_destroy(plane: *mut drm_plane);
}
