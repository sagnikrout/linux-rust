//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_gt_mcr.h
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
// Copyright © 2022 Intel Corporation
//

extern "C" {
    pub fn intel_gt_mcr_init(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_mcr_lock(gt: *mut intel_gt, flags: *mut c_ulong);
}
extern "C" {
    pub fn intel_gt_mcr_unlock(gt: *mut intel_gt, flags: c_ulong);
}
extern "C" {
    pub fn intel_gt_mcr_lock_sanitize(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_mcr_read_any_fw(gt: *mut intel_gt, reg: i915_mcr_reg_t) -> u32;
}
extern "C" {
    pub fn intel_gt_mcr_read_any(gt: *mut intel_gt, reg: i915_mcr_reg_t) -> u32;
}
//
// Helper for for_each_ss_steering loop.  On pre-Xe_HP platforms, subslice
// presence is determined by using the group/instance as direct lookups in the
// slice/subslice topology.  On Xe_HP and beyond, the steering is unrelated to
// the topology, so we lookup the DSS ID directly in "slice 0."
//

//
// Loop over each subslice/DSS and determine the group and instance IDs that
// should be used to steer MCR accesses toward this DSS.
//

