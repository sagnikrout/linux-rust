//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt.h
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

// Our devices have up to 4 media slices
pub const MAX_MEDIA_SLICES: c_int = 4;

extern "C" {
    pub fn xe_gt_init_early(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_mmio_init(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_declare_wedged(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_record_default_lrcs(gt: *mut xe_gt) -> c_int;
}
//
// xe_gt_record_user_engines - save data related to engines available to
// userspace
// @gt: GT structure
//
// Walk the available HW engines from gt->info.engine_mask and calculate data
// related to those engines that may be used by userspace. To be used whenever
// available engines change in runtime (e.g. with ccs_mode) or during
// initialization
//
extern "C" {
    pub fn xe_gt_record_user_engines(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_suspend_prepare(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_suspend(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_shutdown(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_resume(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_reset_async(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_runtime_resume(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_runtime_suspend(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sanitize(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_sanitize_freq(gt: *mut xe_gt) -> c_int;
}
//
// xe_gt_wait_for_reset - wait for gt's async reset to finalize.
// @gt: GT structure
// Return:
// %true if it waited for the work to finish execution,
// %false if there was no scheduled reset or it was done.
//
extern "C" {
    pub fn flush_work(_arg: &gt->reset.worker) -> return;
}
//
// xe_gt_reset - perform synchronous reset
// @gt: GT structure
// Return:
// %true if it waited for the reset to finish,
// %false if there was no scheduled reset.
//
extern "C" {
    pub fn xe_gt_wait_for_reset(_arg: gt) -> return;
}
//
// xe_gt_any_hw_engine_by_reset_domain - scan the list of engines and return the
// first that matches the same reset domain as @class
// @gt: GT structure
// @class: hw engine class to lookup
//
// xe_gt_any_hw_engine - scan the list of engines and return the
// first available
// @gt: GT structure
//
// xe_gt_recovery_pending() - GT recovery pending
// @gt: the &xe_gt
//
// Return: True if GT recovery in pending, False otherwise
//
// xe_gt_supports_multi_queue() - Check if gt supports multi queue for the
// specified engine class.
//
// @gt: the GT object
// @class: hwe class type
//
// Return: true if the hw engine class supports multi queue, else false
//
