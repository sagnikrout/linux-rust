//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_timeline.h
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

extern "C" {
    pub fn __intel_timeline_create(_arg: gt, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn __intel_timeline_free(kref: *mut kref);
}
extern "C" {
    pub fn i915_syncmap_set(_arg: &tl->sync, _arg: context, _arg: seqno) -> return;
}
extern "C" {
    pub fn __intel_timeline_sync_set(_arg: tl, _arg: fence->context, _arg: fence->seqno) -> return;
}
extern "C" {
    pub fn i915_syncmap_is_later(_arg: &tl->sync, _arg: context, _arg: seqno) -> return;
}
extern "C" {
    pub fn __intel_timeline_sync_is_later(_arg: tl, _arg: fence->context, _arg: fence->seqno) -> return;
}
extern "C" {
    pub fn __intel_timeline_pin(tl: *mut intel_timeline);
}
extern "C" {
    pub fn intel_timeline_pin(tl: *mut intel_timeline, ww: *mut i915_gem_ww_ctx) -> c_int;
}
extern "C" {
    pub fn intel_timeline_enter(tl: *mut intel_timeline);
}
extern "C" {
    pub fn intel_timeline_exit(tl: *mut intel_timeline);
}
extern "C" {
    pub fn intel_timeline_unpin(tl: *mut intel_timeline);
}
extern "C" {
    pub fn intel_timeline_reset_seqno(tl: *const intel_timeline);
}
extern "C" {
    pub fn intel_gt_init_timelines(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_fini_timelines(gt: *mut intel_gt);
}
extern "C" {
    pub fn list_is_last_rcu(_arg: &rq->link, _arg: &tl->requests) -> return;
}
