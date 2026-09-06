//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/intel_wakeref.h
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
// Copyright © 2019 Intel Corporation
//

pub const INTEL_REFTRACK_DEAD_COUNT: c_int = 16;
pub const INTEL_REFTRACK_PRINT_LIMIT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wakeref_ops {
    pub wf): *mut *mut int (get)(struct intel_wakeref,
    pub wf): *mut *mut int (put)(struct intel_wakeref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wakeref {
    pub count: core::sync::atomic::AtomicI32,
    pub mutex: mutex,
    pub wakeref: intel_wakeref_t,
    pub i915: *mut drm_i915_private,
    pub ops: *const intel_wakeref_ops,
    pub work: delayed_work,

    pub debug: ref_tracker_dir,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wakeref_lockclass {
    pub mutex: lock_class_key,
    pub work: lock_class_key,
}

extern "C" {
    pub fn __intel_wakeref_get_first(wf: *mut intel_wakeref) -> c_int;
}
extern "C" {
    pub fn __intel_wakeref_put_last(wf: *mut intel_wakeref, flags: c_ulong);
}
//
// intel_wakeref_get: Acquire the wakeref
// @wf: the wakeref
//
// Acquire a hold on the wakeref. The first user to do so, will acquire
// the runtime pm wakeref and then call the intel_wakeref_ops->get()
// underneath the wakeref mutex.
//
// Note that intel_wakeref_ops->get() is allowed to fail, in which case
// the runtime-pm wakeref will be released and the acquisition unwound,
// and an error reported.
//
// Returns: 0 if the wakeref was acquired successfully, or a negative error
// code otherwise.
//
extern "C" {
    pub fn __intel_wakeref_get_first(_arg: wf) -> return;
}
//
// __intel_wakeref_get: Acquire the wakeref, again
// @wf: the wakeref
//
// Increment the wakeref counter, only valid if it is already held by
// the caller.
//
// See intel_wakeref_get().
//
// intel_wakeref_get_if_active: Acquire the wakeref
// @wf: the wakeref
//
// Acquire a hold on the wakeref, but only if the wakeref is already
// active.
//
// Returns: true if the wakeref was acquired, false otherwise.
//
extern "C" {
    pub fn atomic_inc_not_zero(_arg: &wf->count) -> return;
}
// flags for __intel_wakeref_put() and __intel_wakeref_put_last

//
// __intel_wakeref_put: Release the wakeref
// @wf: the wakeref
// @flags: control flags
//
// Release our hold on the wakeref. When there are no more users,
// the runtime pm wakeref will be released after the intel_wakeref_ops->put()
// callback is called underneath the wakeref mutex.
//
// Note that intel_wakeref_ops->put() is allowed to fail, in which case the
// runtime-pm wakeref is retained.
//
// intel_wakeref_lock: Lock the wakeref (mutex)
// @wf: the wakeref
//
// Locks the wakeref to prevent it being acquired or released. New users
// can still adjust the counter, but the wakeref itself (and callback)
// cannot be acquired or released.
//
// intel_wakeref_unlock: Unlock the wakeref
// @wf: the wakeref
//
// Releases a previously acquired intel_wakeref_lock().
//
// intel_wakeref_unlock_wait: Wait until the active callback is complete
// @wf: the wakeref
//
// Waits for the active callback (under the @wf->mutex or another CPU) is
// complete.
//
// intel_wakeref_is_active: Query whether the wakeref is currently held
// @wf: the wakeref
//
// Returns: true if the wakeref is currently held.
//
extern "C" {
    pub fn READ_ONCE(_arg: wf->wakeref) -> return;
}
//
// __intel_wakeref_defer_park: Defer the current park callback
// @wf: the wakeref
//
// intel_wakeref_wait_for_idle: Wait until the wakeref is idle
// @wf: the wakeref
//
// Wait for the earlier asynchronous release of the wakeref. Note
// this will wait for any third party as well, so make sure you only wait
// when you have control over the wakeref and trust no one else is acquiring
// it.
//
// Return: 0 on success, error code if killed.
//
extern "C" {
    pub fn intel_wakeref_wait_for_idle(wf: *mut intel_wakeref) -> c_int;
}

extern "C" {
    pub fn intel_ref_tracker_alloc(_arg: &wf->debug) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wakeref_auto {
    pub i915: *mut drm_i915_private,
    pub timer: timer_list,
    pub wakeref: intel_wakeref_t,
    pub lock: spinlock_t,
    pub count: refcount_t,
}

//
// intel_wakeref_auto: Delay the runtime-pm autosuspend
// @wf: the wakeref
// @timeout: relative timeout in jiffies
//
// The runtime-pm core uses a suspend delay after the last wakeref
// is released before triggering runtime suspend of the device. That
// delay is configurable via sysfs with little regard to the device
// characteristics. Instead, we want to tune the autosuspend based on our
// HW knowledge. intel_wakeref_auto() delays the sleep by the supplied
// timeout.
//
// Pass @timeout = 0 to cancel a previous autosuspend by executing the
// suspend immediately.
//
extern "C" {
    pub fn intel_wakeref_auto(wf: *mut intel_wakeref_auto, timeout: c_ulong);
}
extern "C" {
    pub fn intel_wakeref_auto_fini(wf: *mut intel_wakeref_auto);
}
