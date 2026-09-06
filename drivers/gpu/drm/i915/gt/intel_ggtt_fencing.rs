//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_ggtt_fencing.h
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
//
// Copyright © 2016 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_fence_reg {
    pub link: list_head,
    pub ggtt: *mut i915_ggtt,
    pub vma: *mut i915_vma,
    pub pin_count: core::sync::atomic::AtomicI32,
    pub active: i915_active,
    pub id: c_int,
//
// Whether the tiling parameters for the currently
// associated fence register have changed. Note that
// for the purposes of tracking tiling changes we also
// treat the unfenced register, the register slot that
// the object occupies whilst it executes a fenced
// command (such as BLT on gen2/3), as a "fence".
//
    pub dirty: bool,
    pub start: u32,
    pub size: u32,
    pub tiling: u32,
    pub stride: u32,
}

extern "C" {
    pub fn i915_unreserve_fence(fence: *mut i915_fence_reg);
}
extern "C" {
    pub fn intel_ggtt_restore_fences(ggtt: *mut i915_ggtt);
}
extern "C" {
    pub fn intel_ggtt_init_fences(ggtt: *mut i915_ggtt);
}
extern "C" {
    pub fn intel_ggtt_fini_fences(ggtt: *mut i915_ggtt);
}
extern "C" {
    pub fn intel_gt_init_swizzling(gt: *mut intel_gt);
}
