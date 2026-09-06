//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/selftests/igt_spinner.h
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


//
// SPDX-License-Identifier: MIT
//
// Copyright © 2018 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igt_spinner {
    pub gt: *mut intel_gt,
    pub hws: *mut drm_i915_gem_object,
    pub obj: *mut drm_i915_gem_object,
    pub ce: *mut intel_context,
    pub batch_vma: *mut *mut i915_vma hws_vma,,
    pub batch: *mut u32,
    pub seqno: *mut c_void,
}

extern "C" {
    pub fn igt_spinner_init(spin: *mut igt_spinner, gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn igt_spinner_fini(spin: *mut igt_spinner);
}
extern "C" {
    pub fn igt_spinner_end(spin: *mut igt_spinner);
}
extern "C" {
    pub fn igt_wait_for_spinner(spin: *mut igt_spinner, rq: *mut i915_request) -> bool;
}
