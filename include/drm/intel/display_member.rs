//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/display_member.h
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

//
// A dummy device struct to define the relative offsets of drm and display
// members. With the members identically placed in struct drm_i915_private and
// struct xe_device, this allows figuring out the struct intel_display pointer
// without the definition of either driver specific structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __intel_generic_device {
    pub drm: drm_device,
    pub display: *mut intel_display,
}

//
// INTEL_DISPLAY_MEMBER_STATIC_ASSERT() - ensure correct placing of drm and display members
// @type: The struct to check
// @drm_member: Name of the struct drm_device member
// @display_member: Name of the struct intel_display * member.
//
// Use this static assert macro to ensure the struct drm_i915_private and struct
// xe_device struct drm_device and struct intel_display * members are at the
// same relative offsets.
//

