//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_gt.h
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
// Copyright © 2019 Intel Corporation
//

//
// Check that the GT is a graphics GT and has an IP version within the
// specified range (inclusive).
//

//
// Check that the GT is a media GT and has an IP version within the
// specified range (inclusive).
//
// Only usable on platforms with a standalone media design (i.e., IP version 13
// and higher).
//

//
// Check that the GT is a graphics GT with a specific IP version and has
// a stepping in the range [from, until).  The lower stepping bound is
// inclusive, the upper bound is exclusive.  The most common use-case of this
// macro is for checking bounds for workarounds, which usually have a stepping
// ("from") at which the hardware issue is first present and another stepping
// ("until") at which a hardware fix is present and the software workaround is
// no longer necessary.  E.g.,
//
// IS_GFX_GT_IP_STEP(gt, IP_VER(12, 70), STEP_A0, STEP_B0)
// IS_GFX_GT_IP_STEP(gt, IP_VER(12, 71), STEP_B1, STEP_FOREVER)
//
// "STEP_FOREVER" can be passed as "until" for workarounds that have no upper
// stepping bound for the specified IP version.
//

//
// Check that the GT is a media GT with a specific IP version and has
// a stepping in the range [from, until).  The lower stepping bound is
// inclusive, the upper bound is exclusive.  The most common use-case of this
// macro is for checking bounds for workarounds, which usually have a stepping
// ("from") at which the hardware issue is first present and another stepping
// ("until") at which a hardware fix is present and the software workaround is
// no longer necessary.  "STEP_FOREVER" can be passed as "until" for
// workarounds that have no upper stepping bound for the specified IP version.
//
// This macro may only be used to match on platforms that have a standalone
// media design (i.e., media version 13 or higher).
//

extern "C" {
    pub fn intel_gt_needs_wa_16018031267(gt: *mut intel_gt) -> bool;
}
extern "C" {
    pub fn intel_gt_needs_wa_22016122933(gt: *mut intel_gt) -> bool;
}

extern "C" {
    pub fn container_of(_arg: uc, intel_gt: struct, _arg: uc) -> return;
}
extern "C" {
    pub fn container_of(_arg: guc, intel_gt: struct, _arg: uc.guc) -> return;
}
extern "C" {
    pub fn container_of(_arg: huc, intel_gt: struct, _arg: uc.huc) -> return;
}
extern "C" {
    pub fn container_of(_arg: gsc_uc, intel_gt: struct, _arg: uc.gsc) -> return;
}
extern "C" {
    pub fn container_of(_arg: gsc, intel_gt: struct, _arg: gsc) -> return;
}
extern "C" {
    pub fn intel_gt_common_init_early(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_root_gt_init_early(i915: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn intel_gt_assign_ggtt(gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn intel_gt_init_mmio(gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn intel_gt_init_hw(gt: *mut intel_gt) -> int __must_check;
}
extern "C" {
    pub fn intel_gt_init(gt: *mut intel_gt) -> c_int;
}
extern "C" {
    pub fn intel_gt_driver_register(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_driver_unregister(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_driver_remove(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_driver_release(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_driver_late_release_all(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_gt_wait_for_idle(gt: *mut intel_gt, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn intel_gt_check_and_clear_faults(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_perf_limit_reasons_reg(gt: *mut intel_gt) -> i915_reg_t;
}
extern "C" {
    pub fn intel_gt_flush_ggtt_writes(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_chipset_flush(gt: *mut intel_gt);
}
extern "C" {
    pub fn unlikely(_arg: test_bit(I915_WEDGED, _arg: &gt->reset.flags)) -> return;
}
extern "C" {
    pub fn intel_gt_probe_all(i915: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn intel_gt_tiles_init(i915: *mut drm_i915_private) -> c_int;
}

// Simple iterator over all initialised engines

// Iterator over subset of engines selected by mask

extern "C" {
    pub fn intel_gt_watchdog_work(work: *mut work_struct);
}
extern "C" {
    pub fn intel_gt_bind_context_set_ready(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_bind_context_set_unready(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_is_bind_context_ready(gt: *mut intel_gt) -> bool;
}
