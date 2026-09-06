//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_gtt_view_types.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2025 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_remapped_plane_info {
// in gtt pages
    pub offset:31: u32,
    pub linear:1: u32,
// in gtt pages for !linear
    pub width: u16,
    pub height: u16,
    pub src_stride: u16,
    pub dst_stride: u16,
}

// in gtt pages for linear
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_rotation_info {
    pub plane: [intel_remapped_plane_info; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_partial_info {
    pub offset: u64,
    pub size: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_remapped_info {
    pub plane: [intel_remapped_plane_info; 4],
// in gtt pages
    pub plane_alignment: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_gtt_view_type {
    I915_GTT_VIEW_NORMAL = 0,
    I915_GTT_VIEW_ROTATED = sizeof(struct intel_rotation_info),
    I915_GTT_VIEW_PARTIAL = sizeof(struct intel_partial_info),
    I915_GTT_VIEW_REMAPPED = sizeof(struct intel_remapped_info),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gtt_view {
    pub type: i915_gtt_view_type,
// Members need to contain no holes/padding
    pub partial: intel_partial_info,
    pub rotated: intel_rotation_info,
    pub remapped: intel_remapped_info,
}
