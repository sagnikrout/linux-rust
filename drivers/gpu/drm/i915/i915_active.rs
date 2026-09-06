//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_active.h
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

//
// We treat requests as fences. This is not be to confused with our
// "fence registers" but pipeline synchronisation objects ala GL_ARB_sync.
// We use the fences to synchronize access from the CPU with activity on the
// GPU, for example, we should not rewrite an object's PTE whilst the GPU
// is reading them. We also track fences at a higher level to provide
// implicit synchronisation around GEM objects, e.g. set-domain will wait
// for outstanding GPU rendering before marking the object ready for CPU
// access, or a pageflip will wait until the GPU is complete before showing
// the frame on the scanout.
//
// In order to use a fence, the object must track the fence it needs to
// serialise with. For example, GEM objects want to track both read and
// write access so that we can perform concurrent read operations between
// the CPU and GPU engines, as well as waiting for all rendering to
// complete, or waiting for the last GPU user of a "fence register". The
// object then embeds a #i915_active_fence to track the most recent (in
// retirement order) request relevant for the desired mode of access.
// The #i915_active_fence is updated with i915_active_fence_set() to
// track the most recent fence request, typically this is done as part of
// i915_vma_move_to_active().
//
// When the #i915_active_fence completes (is retired), it will
// signal its completion to the owner through a callback as well as mark
// itself as idle (i915_active_fence.request == NULL). The owner
// can then perform any action, such as delayed freeing of an active
// resource including itself.
//
extern "C" {
    pub fn i915_active_noop(fence: *mut dma_fence, cb: *mut dma_fence_cb);
}
//
// __i915_active_fence_init - prepares the activity tracker for use
// @active: the active tracker
// @fence: initial fence to track, can be NULL
// @fn: a callback when then the tracker is retired (becomes idle),
// can be NULL
//
// i915_active_fence_init() prepares the embedded @active struct for use as
// an activity tracker, that is for tracking the last known active fence
// associated with it. When the last fence becomes idle, when it is retired
// after completion, the optional callback @func is invoked.
//

//
// i915_active_fence_set - updates the tracker to watch the current fence
// @active: the active tracker
// @rq: the request to watch
//
// i915_active_fence_set() watches the given @rq for completion. While
// that @rq is busy, the @active reports busy. When that @rq is signaled
// (or else retired) the @active tracker is updated to report idle.
//
// i915_active_fence_get - return a reference to the active fence
// @active: the active tracker
//
// i915_active_fence_get() returns a reference to the active fence,
// or NULL if the active tracker is idle. The reference is obtained under RCU,
// so no locking is required by the caller.
//
// The reference should be freed with dma_fence_put().
//
// i915_active_fence_isset - report whether the active tracker is assigned
// @active: the active tracker
//
// i915_active_fence_isset() returns true if the active tracker is currently
// assigned to a fence. Due to the lazy retiring, that fence may be idle
// and this may report stale information.
//
extern "C" {
    pub fn rcu_access_pointer(_arg: active->fence) -> return;
}
//
// GPU activity tracking
//
// Each set of commands submitted to the GPU compromises a single request that
// signals a fence upon completion. struct i915_request combines the
// command submission, scheduling and fence signaling roles. If we want to see
// if a particular task is complete, we need to grab the fence (struct
// i915_request) for that task and check or wait for it to be signaled. More
// often though we want to track the status of a bunch of tasks, for example
// to wait for the GPU to finish accessing some memory across a variety of
// different command pipelines from different clients. We could choose to
// track every single request associated with the task, but knowing that
// each request belongs to an ordered timeline (later requests within a
// timeline must wait for earlier requests), we need only track the
// latest request in each timeline to determine the overall status of the
// task.
//
// struct i915_active provides this tracking across timelines. It builds a
// composite shared-fence, and is updated as new work is submitted to the task,
// forming a snapshot of the current status. It should be embedded into the
// different resources that need to track their associated GPU activity to
// provide a callback when that GPU activity has ceased, or otherwise to
// provide a serialisation point either for request submission or for CPU
// synchronisation.
//
// Specialise each class of i915_active to avoid impossible lockdep cycles.

extern "C" {
    pub fn i915_active_add_request(ref: *mut i915_active, rq: *mut i915_request) -> c_int;
}
extern "C" {
    pub fn __i915_active_wait(ref: *mut i915_active, state: c_int) -> c_int;
}
extern "C" {
    pub fn __i915_active_wait(_arg: ref, _arg: TASK_INTERRUPTIBLE) -> return;
}

extern "C" {
    pub fn i915_active_acquire(ref: *mut i915_active) -> c_int;
}
extern "C" {
    pub fn i915_active_acquire_if_busy(ref: *mut i915_active) -> bool;
}
extern "C" {
    pub fn i915_active_release(ref: *mut i915_active);
}
extern "C" {
    pub fn i915_active_fini(ref: *mut i915_active);
}
extern "C" {
    pub fn i915_active_acquire_barrier(ref: *mut i915_active);
}
extern "C" {
    pub fn i915_request_add_active_barriers(rq: *mut i915_request);
}
extern "C" {
    pub fn i915_active_print(ref: *mut i915_active, m: *mut drm_printer);
}
extern "C" {
    pub fn i915_active_unlock_wait(ref: *mut i915_active);
}
extern "C" {
    pub fn i915_active_put(ref: *mut i915_active);
}
extern "C" {
    pub fn i915_active_module_exit();
}
extern "C" {
    pub fn i915_active_module_init() -> c_int;
}
