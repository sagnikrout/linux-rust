//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/vlv_iosf_sb.h
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
// Copyright © 2013-2021 Intel Corporation
//

extern "C" {
    pub fn vlv_iosf_sb_init(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn vlv_iosf_sb_fini(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn vlv_iosf_sb_get(drm: *mut drm_device, unit_mask: c_ulong);
}
extern "C" {
    pub fn vlv_iosf_sb_put(drm: *mut drm_device, unit_mask: c_ulong);
}
extern "C" {
    pub fn vlv_iosf_sb_read(drm: *mut drm_device, unit: vlv_iosf_sb_unit, addr: u32) -> u32;
}
extern "C" {
    pub fn vlv_iosf_sb_write(drm: *mut drm_device, unit: vlv_iosf_sb_unit, addr: u32, val: u32) -> c_int;
}
