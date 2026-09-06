//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_reset_types.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_reset {
//
// flags: Control various stages of the GPU reset
//
// #I915_RESET_BACKOFF - When we start a global reset, we need to
// serialise with any other users attempting to do the same, and
// any global resources that may be clobber by the reset (such as
// FENCE registers).
//
// #I915_RESET_ENGINE[num_engines] - Since the driver doesn't need to
// acquire a global lock to reset an engine, we need an explicit
// flag to prevent two concurrent reset attempts in the same engine.
// As the number of engines continues to grow, allocate the flags from
// the most significant bits.
//
// #I915_WEDGED - If reset fails and we can no longer use the GPU,
// we set the #I915_WEDGED bit. Prior to command submission, e.g.
// i915_request_alloc(), this bit is checked and the sequence
// aborted (with -EIO reported to userspace) if set.
//
// #I915_WEDGED_ON_INIT - If we fail to initialize the GPU we can no
// longer use the GPU - similar to #I915_WEDGED bit. The difference in
// the way we're handling "forced" unwedged (e.g. through debugfs),
// which is not allowed in case we failed to initialize.
//
// #I915_WEDGED_ON_FINI - Similar to #I915_WEDGED_ON_INIT, except we
// use it to mark that the GPU is no longer available (and prevent
// users from using it).
//
    pub flags: c_ulong,
pub const I915_RESET_BACKOFF: c_int = 0;
pub const I915_RESET_ENGINE: c_int = 1;

    pub /: *mut *mut mutex mutex; / serialises wedging/unwedging,
//
// Waitqueue to signal when the reset has completed. Used by clients
// that wait for i915->mm.wedged to settle.
//
    pub queue: wait_queue_head_t,
    pub backoff_srcu: srcu_struct,
}
