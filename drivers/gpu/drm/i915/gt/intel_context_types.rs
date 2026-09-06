//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_context_types.h
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
pub struct intel_context_ops {
    pub flags: c_ulong,
pub const COPS_HAS_INFLIGHT_BIT: c_int = 0;

pub const COPS_RUNTIME_CYCLES_BIT: c_int = 1;

    pub ce): *mut *mut int (alloc)(struct intel_context,
    pub preempt_timeout_ms): c_uint,
    pub ce): *mut *mut void (close)(struct intel_context,
    pub vaddr): *mut *mut *mut *mut int (pre_pin)(struct intel_context ce, struct i915_gem_ww_ctx ww, void,
    pub vaddr): *mut *mut *mut int (pin)(struct intel_context ce, void,
    pub ce): *mut *mut void (unpin)(struct intel_context,
    pub ce): *mut *mut void (post_unpin)(struct intel_context,
    pub rq): *mut i915_request,
    pub ce): *mut *mut void (enter)(struct intel_context,
    pub ce): *mut *mut void (exit)(struct intel_context,
    pub ce): *mut *mut void (sched_disable)(struct intel_context,
    pub ce): *mut *mut void (update_stats)(struct intel_context,
    pub ce): *mut *mut void (reset)(struct intel_context,
    pub kref): *mut *mut void (destroy)(struct kref,
// virtual/parallel engine/context interface
    pub flags): c_ulong,
    pub width): c_uint,
    pub sibling): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_context {
//
// Note: Some fields may be accessed under RCU.
//
// Unless otherwise noted a field can safely be assumed to be protected
// by strong reference counting.
//
    pub /: *mut *mut kref ref; / no kref_get_unless_zero()!,
    pub rcu: rcu_head,
}

//
// @signal_lock protects the list of requests that need signaling,
// @signals. While there are any requests that need signaling,
// we add the context to the breadcrumbs worker, and remove it
// upon completion/cancellation of the last request.
//
pub const CONTEXT_BARRIER_BIT: c_int = 0;
pub const CONTEXT_ALLOC_BIT: c_int = 1;
pub const CONTEXT_INIT_BIT: c_int = 2;
pub const CONTEXT_VALID_BIT: c_int = 3;
pub const CONTEXT_CLOSED_BIT: c_int = 4;
pub const CONTEXT_USE_SEMAPHORES: c_int = 5;
pub const CONTEXT_BANNED: c_int = 6;
pub const CONTEXT_FORCE_SINGLE_SUBMISSION: c_int = 7;
pub const CONTEXT_NOPREEMPT: c_int = 8;
pub const CONTEXT_LRCA_DIRTY: c_int = 9;
pub const CONTEXT_GUC_INIT: c_int = 10;
pub const CONTEXT_PERMA_PIN: c_int = 11;
pub const CONTEXT_IS_PARKING: c_int = 12;
pub const CONTEXT_EXITING: c_int = 13;
pub const CONTEXT_LOW_LATENCY: c_int = 14;
pub const CONTEXT_OWN_STATE: c_int = 15;
// stats: Context GPU engine busyness tracking.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_context_stats {
    pub active: u64,
// Time on GPU as tracked by the hw.
    pub avg: ewma_runtime,
    pub total: u64,
    pub last: u32,
    pub num_underflow): I915_SELFTEST_DECLARE(u32,
    pub max_underflow): I915_SELFTEST_DECLARE(u32,
    pub runtime: },
    pub stats: },
    pub /: *mut *mut unsigned int active_count; / protected by timeline->mutex,
    pub pin_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut mutex pin_mutex; / guards pinning and associated on-gpuing,
//
// active: Active tracker for the rq activity (inc. external) on this
// intel_context object.
//
    pub active: i915_active,
    pub ops: *const intel_context_ops,
// sseu: Control eu/slice partitioning
    pub sseu: intel_sseu,
//
// pinned_contexts_link: List link for the engine's pinned contexts.
// This is only used if this is a perma-pinned kernel context and
// the list is assumed to only be manipulated during driver load
// or unload time so no mutex protection currently.
//
    pub pinned_contexts_link: list_head,
    pub /: *mut *mut u8 wa_bb_page; / if set, page num reserved for context workarounds,
// @lock: protects everything in guc_state
    pub lock: spinlock_t,
//
// @sched_state: scheduling state of this context using GuC
// submission
//
    pub sched_state: u32,
//
// @fences: maintains a list of requests that are currently
// being fenced until a GuC operation completes
//
    pub fences: list_head,
//
// @blocked: fence used to signal when the blocking of a
// context's submissions is complete.
//
    pub blocked: i915_sw_fence,
// @requests: list of active requests on this context
    pub requests: list_head,
// @prio: the context's current guc priority
    pub prio: u8,
//
// @prio_count: a counter of the number requests in flight in
// each priority bucket
//
    pub prio_count: [u32; GUC_CLIENT_PRIORITY_NUM],
//
// @sched_disable_delay_work: worker to disable scheduling on this
// context
//
    pub sched_disable_delay_work: delayed_work,
    pub guc_state: },
//
// @id: handle which is used to uniquely identify this context
// with the GuC, protected by guc->submission_state.lock
//
    pub id: u16,
//
// @ref: the number of references to the guc_id, when
// transitioning in and out of zero protected by
// guc->submission_state.lock
//
    pub ref: core::sync::atomic::AtomicI32,
//
// @link: in guc->guc_id_list when the guc_id has no refs but is
// still valid, protected by guc->submission_state.lock
//
    pub link: list_head,
    pub guc_id: },
//
// @destroyed_link: link in guc->submission_state.destroyed_contexts, in
// list when context is pending to be destroyed (deregistered with the
// GuC), protected by guc->submission_state.lock
//
    pub destroyed_link: list_head,
// @parallel: sub-structure for parallel submission members
//
// @child_list: parent's list of children
// contexts, no protection as immutable after context
// creation
//
    pub child_list: list_head,
//
// @child_link: child's link into parent's list of
// children
//
    pub child_link: list_head,
}

// @parent: pointer to parent if child
//
// @last_rq: last request submitted on a parallel context, used
// to insert submit fences between requests in the parallel
// context
//
// @fence_context: fence context composite fence when doing
// parallel submission
//
// @seqno: seqno for composite fence when doing parallel
// submission
//
// @number_children: number of children if parent
// @child_index: index into child_list if child
// @guc: GuC specific members for parallel submission
// @wqi_head: cached head pointer in work queue
// @wqi_tail: cached tail pointer in work queue
// @wq_head: pointer to the actual head in work queue
// @wq_tail: pointer to the actual head in work queue
// @wq_status: pointer to the status in work queue
//
// @parent_page: page in context state (ce->state) used
// by parent for work queue, process descriptor
//

//
// @drop_schedule_enable: Force drop of schedule enable G2H for selftest
//
// @drop_schedule_disable: Force drop of schedule disable G2H for
// selftest
//
// @drop_deregister: Force drop of deregister G2H for selftest
//

