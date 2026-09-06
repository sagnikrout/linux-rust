//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_context.h
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

extern "C" {
    pub fn intel_context_fini(ce: *mut intel_context);
}
extern "C" {
    pub fn i915_context_module_exit();
}
extern "C" {
    pub fn i915_context_module_init() -> c_int;
}
extern "C" {
    pub fn intel_context_alloc_state(ce: *mut intel_context) -> c_int;
}
extern "C" {
    pub fn intel_context_free(ce: *mut intel_context);
}

extern "C" {
    pub fn intel_context_is_pinned(ce: *mut intel_context) -> bool;
}
//
// The parent holds ref count to the child so it is always safe
// for the parent to access the child, but the child has a
// pointer to the parent without a ref. To ensure this is safe
// the child should only access the parent pointer while the
// parent is pinned.
//
extern "C" {
    pub fn intel_context_is_child(intel_context_is_parent(ce: ce) ||) -> return;
}

//
// intel_context_lock_pinned - Stablises the 'pinned' status of the HW context
// @ce: the context
//
// Acquire a lock on the pinned status of the HW context, such that the context
// can neither be bound to the GPU or unbound whilst the lock is held, i.e.
// intel_context_is_pinned() remains stable.
//
extern "C" {
    pub fn mutex_lock_interruptible(_arg: &ce->pin_mutex) -> return;
}
//
// intel_context_is_pinned - Reports the 'pinned' status
// @ce: the context
//
// While in use by the GPU, the context, along with its ring and page
// tables is pinned into memory and the GTT.
//
// Returns: true if the context is currently pinned for use by the GPU.
//
extern "C" {
    pub fn atomic_read(_arg: &ce->pin_count) -> return;
}
//
// intel_context_unlock_pinned - Releases the earlier locking of 'pinned' status
// @ce: the context
//
// Releases the lock earlier acquired by intel_context_unlock_pinned().
//
extern "C" {
    pub fn __intel_context_do_pin(ce: *mut intel_context) -> c_int;
}
extern "C" {
    pub fn atomic_inc_not_zero(_arg: &ce->pin_count) -> return;
}
extern "C" {
    pub fn __intel_context_do_pin(_arg: ce) -> return;
}
extern "C" {
    pub fn __intel_context_do_pin_ww(_arg: ce, _arg: ww) -> return;
}
extern "C" {
    pub fn __intel_context_do_unpin(ce: *mut intel_context, sub: c_int);
}
//
// Move ownership of this pin to the scheduling disable which is
// an async operation. When that operation completes the above
// intel_context_sched_disable_unpin is called potentially
// unpinning the context.
//
extern "C" {
    pub fn intel_context_enter_engine(ce: *mut intel_context);
}
extern "C" {
    pub fn intel_context_exit_engine(ce: *mut intel_context);
}
extern "C" {
    pub fn ERR_PTR(_arg: err) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_BARRIER_BIT, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_CLOSED_BIT, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: COPS_HAS_INFLIGHT_BIT, _arg: &ce->ops->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_USE_SEMAPHORES, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_BANNED, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: CONTEXT_BANNED, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn intel_context_ban(ce: *mut intel_context, rq: *mut i915_request) -> bool;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_EXITING, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: CONTEXT_EXITING, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn intel_context_revoke(ce: *mut intel_context) -> bool;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_FORCE_SINGLE_SUBMISSION, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_NOPREEMPT, _arg: &ce->flags) -> return;
}

extern "C" {
    pub fn test_bit(_arg: CONTEXT_OWN_STATE, _arg: &ce->flags) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: CONTEXT_OWN_STATE, _arg: &ce->flags) -> return;
}

extern "C" {
    pub fn intel_context_get_total_runtime_ns(ce: *mut intel_context) -> u64;
}
extern "C" {
    pub fn intel_context_get_avg_runtime_ns(ce: *mut intel_context) -> u64;
}
// As we mix CS cycles with CPU clocks, use the raw monotonic clock.
extern "C" {
    pub fn ktime_get_raw_fast_ns() -> return;
}
