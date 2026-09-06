//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_timeline_types.h
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
pub struct intel_timeline {
    pub fence_context: u64,
    pub seqno: u32,
    pub /: *mut *mut mutex mutex; / protects the flow of requests,
//
// pin_count and active_count track essentially the same thing:
// How many requests are in flight or may be under construction.
//
// We need two distinct counters so that we can assign different
// lifetimes to the events for different use-cases. For example,
// we want to permanently keep the timeline pinned for the kernel
// context so that we can issue requests at any time without having
// to acquire space in the GGTT. However, we want to keep tracking
// the activity (to be able to detect when we become idle) along that
// permanently pinned timeline and so end up requiring two counters.
//
// Note that the active_count is protected by the intel_timeline.mutex,
// but the pin_count is protected by a combination of serialisation
// from the intel_context caller plus internal atomicity.
//
    pub pin_count: core::sync::atomic::AtomicI32,
    pub active_count: core::sync::atomic::AtomicI32,
    pub hwsp_map: *mut c_void,
    pub hwsp_seqno: *const u32,
    pub hwsp_ggtt: *mut i915_vma,
    pub hwsp_offset: u32,
    pub has_initial_breadcrumb: bool,
//
// List of breadcrumbs associated with GPU requests currently
// outstanding.
//
    pub requests: list_head,
//
// Contains an RCU guarded pointer to the last request. No reference is
// held to the request, users must carefully acquire a reference to
// the request using i915_active_fence_get(), or manage the RCU
// protection themselves (cf the i915_active_fence API).
//
    pub last_request: i915_active_fence,
    pub active: i915_active,
// A chain of completed timelines ready for early retirement.
    pub retire: *mut intel_timeline,
//
// We track the most recent seqno that we wait on in every context so
// that we only have to emit a new await and dependency on a more
// recent sync point. As the contexts may be executed out-of-order, we
// have to track each individually and can not rely on an absolute
// global_seqno. When we know that all tracked fences are completed
// (i.e. when the driver is idle), we know that the syncmap is
// redundant and we can discard it without loss of generality.
//
    pub sync: *mut i915_syncmap,
    pub link: list_head,
    pub gt: *mut intel_gt,
    pub engine_link: list_head,
    pub kref: kref,
    pub rcu: rcu_head,
}
