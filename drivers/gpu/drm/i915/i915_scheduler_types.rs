//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_scheduler_types.h
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
// Copyright © 2018 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_sched_attr {
//
// @priority: execution and service priority
//
// All clients are equal, but some are more equal than others!
//
// Requests from a context with a greater (more positive) value of
// @priority will be executed before those with a lower @priority
// value, forming a simple QoS.
//
// The &drm_i915_private.kernel_context is assigned the lowest priority.
//
    pub priority: c_int,
}

//
// "People assume that time is a strict progression of cause to effect, but
// actually, from a nonlinear, non-subjective viewpoint, it's more like a big
// ball of wibbly-wobbly, timey-wimey ... stuff." -The Doctor, 2015
//
// Requests exist in a complex web of interdependencies. Each request
// has to wait for some other request to complete before it is ready to be run
// (e.g. we have to wait until the pixels have been rendering into a texture
// before we can copy from it). We track the readiness of a request in terms
// of fences, but we also need to keep the dependency tree for the lifetime
// of the request (beyond the life of an individual fence). We use the tree
// at various points to reorder the requests whilst keeping the requests
// in order with respect to their various dependencies.
//
// There is no active component to the "scheduler". As we know the dependency
// DAG of each request, we are able to insert it into a sorted queue when it
// is ready, and are able to reorder its portion of the graph to accommodate
// dynamic priority changes.
//
// Ok, there is now one active element to the "scheduler" in the backends.
// We let a new context run for a small amount of time before re-evaluating
// the run order. As we re-evaluate, we maintain the strict ordering of
// dependencies, but attempt to rotate the active contexts (the current context
// is put to the back of its priority queue, then reshuffling its dependents).
// This provides minimal timeslicing and prevents a userspace hog (e.g.
// something waiting on a user semaphore [VkEvent]) from denying service to
// others.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_sched_node {
    pub /: *mut *mut list_head signalers_list; / those before us, we depend upon,
    pub /: *mut *mut list_head waiters_list; / those after us, they depend upon us,
    pub link: list_head,
    pub attr: i915_sched_attr,
    pub flags: c_uint,

    pub semaphores: intel_engine_mask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_dependency {
    pub signaler: *mut i915_sched_node,
    pub waiter: *mut i915_sched_node,
    pub signal_link: list_head,
    pub wait_link: list_head,
    pub dfs_link: list_head,
    pub flags: c_ulong,

}

//
// struct i915_sched_engine - scheduler engine
//
// A schedule engine represents a submission queue with different priority
// bands. It contains all the common state (relative to the backend) to queue,
// track, and submit a request.
//
// This object at the moment is quite i915 specific but will transition into a
// container for the drm_gpu_scheduler plus a few other variables once the i915
// is integrated with the DRM scheduler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_sched_engine {
//
// @ref: reference count of schedule engine object
//
    pub ref: kref,
//
// @lock: protects requests in priority lists, requests, hold and
// tasklet while running
//
    pub lock: spinlock_t,
//
// @requests: list of requests inflight on this schedule engine
//
    pub requests: list_head,
//
// @hold: list of ready requests, but on hold
//
    pub hold: list_head,
//
// @tasklet: softirq tasklet for submission
//
    pub tasklet: tasklet_struct,
//
// @default_priolist: priority list for I915_PRIORITY_NORMAL
//
    pub default_priolist: i915_priolist,
//
// @queue_priority_hint: Highest pending priority.
//
// When we add requests into the queue, or adjust the priority of
// executing requests, we compute the maximum priority of those
// pending requests. We can then use this value to determine if
// we need to preempt the executing requests to service the queue.
// However, since the we may have recorded the priority of an inflight
// request we wanted to preempt but since completed, at the time of
// dequeuing the priority hint may no longer may match the highest
// available request priority.
//
    pub queue_priority_hint: c_int,
//
// @queue: queue of requests, in priority lists
//
    pub queue: rb_root_cached,
//
// @no_priolist: priority lists disabled
//
    pub no_priolist: bool,
//
// @private_data: private data of the submission backend
//
    pub private_data: *mut c_void,
//
// @destroy: destroy schedule engine / cleanup in backend
//
    pub kref): *mut *mut void (destroy)(struct kref,
//
// @disabled: check if backend has disabled submission
//
    pub sched_engine): *mut *mut bool (disabled)(struct i915_sched_engine,
//
// @kick_backend: kick backend after a request's priority has changed
//
    pub prio): c_int,
//
// @bump_inflight_request_prio: update priority of an inflight request
//
    pub prio): c_int,
//
// @retire_inflight_request_prio: indicate request is retired to
// priority tracking
//
    pub rq): *mut *mut void (retire_inflight_request_prio)(struct i915_request,
//
// @schedule: adjust priority of request
//
// Call when the priority on a request has changed and it and its
// dependencies may need rescheduling. Note the request itself may
// not be ready to run!
//
    pub attr): *const i915_sched_attr,
}
