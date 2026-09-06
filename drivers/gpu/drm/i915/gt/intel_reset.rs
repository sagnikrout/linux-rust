//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_reset.h
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
// Copyright © 2008-2018 Intel Corporation
//

extern "C" {
    pub fn intel_gt_init_reset(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_fini_reset(gt: *mut intel_gt);
}

extern "C" {
    pub fn intel_gt_gpu_reset_clobbers_display(gt: *mut intel_gt) -> bool;
}
extern "C" {
    pub fn __i915_request_reset(rq: *mut i915_request, guilty: bool);
}
extern "C" {
    pub fn intel_gt_reset_trylock(gt: *mut intel_gt, srcu: *mut c_int) -> int __must_check;
}
extern "C" {
    pub fn intel_gt_reset_lock_interruptible(gt: *mut intel_gt, srcu: *mut c_int) -> int __must_check;
}
extern "C" {
    pub fn intel_gt_reset_unlock(gt: *mut intel_gt, tag: c_int);
}
extern "C" {
    pub fn intel_gt_set_wedged(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_unset_wedged(gt: *mut intel_gt) -> bool;
}
extern "C" {
    pub fn intel_gt_terminally_wedged(gt: *mut intel_gt) -> c_int;
}
//
// There's no unset_wedged_on_init paired with this one.
// Once we're wedged on init, there's no going back.
// Same thing for unset_wedged_on_fini.
//
extern "C" {
    pub fn intel_gt_set_wedged_on_init(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_set_wedged_on_fini(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_reset_engine(engine: *mut intel_engine_cs) -> c_int;
}
extern "C" {
    pub fn intel_gt_reset_all_engines(gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn intel_reset_guc(gt: *mut intel_gt) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wedge_me {
    pub work: delayed_work,
    pub gt: *mut intel_gt,
    pub name: *const c_char,
}

extern "C" {
    pub fn __intel_fini_wedge(w: *mut intel_wedge_me);
}

extern "C" {
    pub fn intel_has_gpu_reset(gt: *const intel_gt) -> bool;
}
extern "C" {
    pub fn intel_has_reset_engine(gt: *const intel_gt) -> bool;
}
extern "C" {
    pub fn intel_engine_reset_needs_wa_22011802037(gt: *mut intel_gt) -> bool;
}
