//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_request.h
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
// Copyright © 2008-2018 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_capture_list {
    pub vma_res: *mut i915_vma_resource,
    pub next: *mut i915_capture_list,
}

extern "C" {
    pub fn i915_request_free_capture_list(capture: *mut i915_capture_list);
}

//
// I915_FENCE_FLAG_ACTIVE - this request is currently submitted to HW.
//
// Set by __i915_request_submit() on handing over to HW, and cleared
// by __i915_request_unsubmit() if we preempt this request.
//
// Finally cleared for consistency on retiring the request, when
// we know the HW is no longer running this request.
//
// See i915_request_is_active()
//
// I915_FENCE_FLAG_PQUEUE - this request is ready for execution
//
// Using the scheduler, when a request is ready for execution it is put
// into the priority queue, and removed from that queue when transferred
// to the HW runlists. We want to track its membership within the
// priority queue so that we can easily check before rescheduling.
//
// See i915_request_in_priority_queue()
//
// I915_FENCE_FLAG_HOLD - this request is currently on hold
//
// This request has been suspended, pending an ongoing investigation.
//
// I915_FENCE_FLAG_INITIAL_BREADCRUMB - this request has the initial
// breadcrumb that marks the end of semaphore waits and start of the
// user payload.
//
// I915_FENCE_FLAG_SIGNAL - this request is currently on signal_list
//
// Internal bookkeeping used by the breadcrumb code to track when
// a request is on the various signal_list.
//
// I915_FENCE_FLAG_NOPREEMPT - this request should not be preempted
//
// The execution of some requests should not be interrupted. This is
// a sensitive operation as it makes the request super important,
// blocking other higher priority work. Abuse of this flag will
// lead to quality of service issues.
//
// I915_FENCE_FLAG_SENTINEL - this request should be last in the queue
//
// A high priority sentinel request may be submitted to clear the
// submission queue. As it will be the only request in-flight, upon
// execution all other active requests will have been preempted and
// unsubmitted. This preemptive pulse is used to re-evaluate the
// in-flight requests, particularly in cases where an active context
// is banned and those active requests need to be cancelled.
//
// I915_FENCE_FLAG_BOOST - upclock the gpu for this request
//
// Some requests are more important than others! In particular, a
// request that the user is waiting on is typically required for
// interactive latency, for which we want to minimise by upclocking
// the GPU. Here we track such boost requests on a per-request basis.
//
// I915_FENCE_FLAG_SUBMIT_PARALLEL - request with a context in a
// parent-child relationship (parallel submission, multi-lrc) should
// trigger a submission to the GuC rather than just moving the context
// tail.
//
// I915_FENCE_FLAG_SKIP_PARALLEL - request with a context in a
// parent-child relationship (parallel submission, multi-lrc) that
// hit an error while generating requests in the execbuf IOCTL.
// Indicates this request should be skipped as another request in
// submission / relationship encountered an error.
//
// I915_FENCE_FLAG_COMPOSITE - Indicates fence is part of a composite
// fence (dma_fence_array) and i915 generated for parallel submission.
//
// Request queue structure.
//
// The request queue allows us to note sequence numbers that have been emitted
// and may be associated with active buffers to be retired.
//
// By keeping this list, we can avoid having to do questionable sequence
// number comparisons on buffer last_read|write_seqno. It also allows an
// emission time to be associated with the request for tracking how far ahead
// of the GPU the submission is.
//
// When modifying this structure be very aware that we perform a lockless
// RCU lookup of it that may race against reallocation of the struct
// from the slab freelist. We intentionally do not zero the structure on
// allocation so that the lookup can use the dangling pointers (and is
// cognisant that those pointers may be wrong). Instead, everything that
// needs to be initialised must be done so explicitly.
//
// The requests are reference counted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_request {
    pub fence: dma_fence,
    pub lock: spinlock_t,
    pub i915: *mut drm_i915_private,
//
// Context and ring buffer related to this request
// Contexts are refcounted, so when this request is associated with a
// context, we must increment the context's refcount, to guarantee that
// it persists while any request is linked to it. Requests themselves
// are also refcounted, so the request will only be freed when the last
// reference to it is dismissed, and the code in
// i915_request_free() will then decrement the refcount on the
// context.
//
    pub engine: *mut intel_engine_cs,
    pub context: *mut intel_context,
    pub ring: *mut intel_ring,
    pub timeline: *mut intel_timeline __rcu,
    pub signal_link: list_head,
    pub signal_node: llist_node,
//
// The rcu epoch of when this request was allocated. Used to judiciously
// apply backpressure on future allocations to ensure that under
// mempressure there is sufficient RCU ticks for us to reclaim our
// RCU protected slabs.
//
    pub rcustate: c_ulong,
//
// We pin the timeline->mutex while constructing the request to
// ensure that no caller accidentally drops it during construction.
// The timeline->mutex must be held to ensure that only this caller
// can use the ring and manipulate the associated timeline during
// construction.
//
    pub cookie: pin_cookie,
//
// Fences for the various phases in the request's lifetime.
//
// The submit fence is used to await upon all of the request's
// dependencies. When it is signaled, the request is ready to run.
// It is used by the driver to then queue the request for execution.
//
    pub submit: i915_sw_fence,
    pub submitq: wait_queue_entry_t,
    pub dmaq: i915_sw_dma_fence_cb,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_request_duration_cb {
    pub cb: dma_fence_cb,
    pub emitted: ktime_t,
    pub duration: },
}

//
// complete submit fence from an IRQ if needed for locking hierarchy
// reasons.
//
// A list of everyone we wait upon, and everyone who waits upon us.
// Even though we will not be submitted to the hardware before the
// submit fence is signaled (it waits for all external events as well
// as our own requests), the scheduler still needs to know the
// dependency tree for the lifetime of the request (from execbuf
// to retirement), i.e. bidirectional dependency information for the
// request not tied to individual fences.
//
// A convenience pointer to the current breadcrumb value stored in
// the HW status page (or our timeline's local equivalent). The full
// path would be rq->hw_context->ring->timeline->hwsp_seqno.
//
// Position in the ring of the start of the request
// Position in the ring of the start of the user packets
//
// Position in the ring of the start of the postfix.
// This is required to calculate the maximum available ring space
// without overwriting the postfix.
//
// Position in the ring of the end of the whole request
// Position in the ring of the end of any workarounds after the tail
// Preallocate space in the ring for the emitting the request
// Batch buffer pointer for selftest internal use.

//
// Additional buffers requested by userspace to be captured upon
// a GPU hang. The vma/obj on this list are protected by their
// active reference - all objects on this list must also be
// on the active_list (of their final request).
//

// Time at which this request was emitted, in jiffies.
// timeline->request entry for this request
// Watchdog support fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_request_watchdog {
    pub link: llist_node,
    pub timer: hrtimer,
    pub watchdog: },
//
// Requests may need to be stalled when using GuC submission waiting for
// certain GuC operations to complete. If that is the case, stalled
// requests are added to a per context list of stalled requests. The
// below list_head is the link in that list. Protected by
// ce->guc_state.lock.
//
    pub guc_fence_link: list_head,
//
// Priority level while the request is in flight. Differs
// from i915 scheduler priority. See comment above
// I915_SCHEDULER_CAP_STATIC_PRIORITY_MAP for details. Protected by
// ce->guc_active.lock. Two special values (GUC_PRIO_INIT and
// GUC_PRIO_FINI) outside the GuC priority range are used to indicate
// if the priority has not been initialized yet or if no more updates
// are possible because the request has completed.
//
pub const GUC_PRIO_INIT: c_uint = 0xff;
pub const GUC_PRIO_FINI: c_uint = 0xfe;
    pub guc_prio: u8,
//
// wait queue entry used to wait on the HuC load to complete
//
    pub hucq: wait_queue_entry_t,
    pub link: list_head,
    pub delay: c_ulong,
    pub mock;): },
}

extern "C" {
    pub fn __i915_request_skip(rq: *mut i915_request);
}
extern "C" {
    pub fn i915_request_set_error_once(rq: *mut i915_request, error: c_int) -> bool;
}
extern "C" {
    pub fn __i915_request_queue_bh(rq: *mut i915_request);
}
extern "C" {
    pub fn i915_request_retire(rq: *mut i915_request) -> bool;
}
extern "C" {
    pub fn i915_request_retire_upto(rq: *mut i915_request);
}
// We assume that NULL fence/request are interoperable
extern "C" {
    pub fn container_of(_arg: fence, i915_request: struct, _arg: fence) -> return;
}
extern "C" {
    pub fn to_request(_arg: dma_fence_get(&rq->fence)) -> return;
}
extern "C" {
    pub fn to_request(_arg: dma_fence_get_rcu(&rq->fence)) -> return;
}
extern "C" {
    pub fn i915_request_await_deps(rq: *mut i915_request, deps: *const i915_deps) -> c_int;
}
extern "C" {
    pub fn i915_request_add(rq: *mut i915_request);
}
extern "C" {
    pub fn __i915_request_submit(request: *mut i915_request) -> bool;
}
extern "C" {
    pub fn i915_request_submit(request: *mut i915_request);
}
extern "C" {
    pub fn __i915_request_unsubmit(request: *mut i915_request);
}
extern "C" {
    pub fn i915_request_unsubmit(request: *mut i915_request);
}
extern "C" {
    pub fn i915_request_cancel(rq: *mut i915_request, error: c_int);
}

// The request may live longer than its HWSP, so check flags first!
extern "C" {
    pub fn test_bit(_arg: DMA_FENCE_FLAG_SIGNALED_BIT, _arg: &rq->fence.flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: I915_FENCE_FLAG_ACTIVE, _arg: &rq->fence.flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: I915_FENCE_FLAG_PQUEUE, _arg: &rq->fence.flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: I915_FENCE_FLAG_INITIAL_BREADCRUMB, _arg: &rq->fence.flags) -> return;
}
//
// Returns true if seq1 is later than seq2.
//
extern "C" {
    pub fn READ_ONCE(_arg: *mut hwsp) -> return;
}
//
// hwsp_seqno - the current breadcrumb value in the HW status page
// @rq: the request, to chase the relevant HW status page
//
// The emphasis in naming here is that hwsp_seqno() is not a property of the
// request, but an indication of the current HW state (associated with this
// request). Its value will change as the GPU executes more requests.
//
// Returns the current breadcrumb value in the associated HW status page (or
// the local timeline's equivalent) for this request. The request itself
// has the associated breadcrumb value of rq->fence.seqno, when the HW
// status page has that breadcrumb or later, this request is complete.
//
extern "C" {
    pub fn i915_seqno_passed(_arg: __hwsp_seqno(rq), 1: rq->fence.seqno -) -> return;
}
//
// i915_request_started - check if the request has begun being executed
// @rq: the request
//
// If the timeline is not using initial breadcrumbs, a request is
// considered started if the previous request on its timeline (i.e.
// context) has been signaled.
//
// If the timeline is using semaphores, it will also be emitting an
// "initial breadcrumb" after the semaphores are complete and just before
// it began executing the user payload. A request can therefore be active
// on the HW and not yet started as it is still busywaiting on its
// dependencies (via HW semaphores).
//
// If the request has started, its dependencies will have been signaled
// (either by fences or by semaphores) and it will have begun processing
// the user payload.
//
// However, even if a request has started, it may have been preempted and
// so no longer active, or it may have already completed.
//
// See also i915_request_is_active().
//
// Returns true if the request has begun executing the user payload, or
// has completed:
//
// Remember: started but may have since been preempted!
//
// i915_request_is_running - check if the request may actually be executing
// @rq: the request
//
// Returns true if the request is currently submitted to hardware, has passed
// its start point (i.e. the context is setup and not busywaiting). Note that
// it may no longer be running by the time the function returns!
//
// i915_request_is_ready - check if the request is ready for execution
// @rq: the request
//
// Upon construction, the request is instructed to wait upon various
// signals before it is ready to be executed by the HW. That is, we do
// not want to start execution and read data before it is written. In practice,
// this is controlled with a mixture of interrupts and semaphores. Once
// the submit fence is completed, the backend scheduler will place the
// request into its queue and from there submit it for execution. So we
// can detect when a request is eligible for execution (and is under control
// of the scheduler) by querying where it is in any of the scheduler's lists.
//
// Returns true if the request is ready for execution (it may be inflight),
// false otherwise.
//
extern "C" {
    pub fn i915_seqno_passed(_arg: __hwsp_seqno(rq), _arg: rq->fence.seqno) -> return;
}
extern "C" {
    pub fn test_bit(_arg: I915_FENCE_FLAG_BOOST, _arg: &rq->fence.flags) -> return;
}
// Preemption should only be disabled very rarely
extern "C" {
    pub fn unlikely(_arg: test_bit(I915_FENCE_FLAG_NOPREEMPT, _arg: &rq->fence.flags)) -> return;
}
extern "C" {
    pub fn unlikely(_arg: test_bit(I915_FENCE_FLAG_SENTINEL, _arg: &rq->fence.flags)) -> return;
}
extern "C" {
    pub fn unlikely(_arg: test_bit(I915_FENCE_FLAG_HOLD, _arg: &rq->fence.flags)) -> return;
}
// Valid only while the request is being constructed (or retired).
extern "C" {
    pub fn rcu_dereference_protected(_arg: rq->context->gem_context, _arg: true) -> return;
}
//
// When in use during submission, we are protected by a guarantee that
// the context/timeline is pinned and must remain pinned until after
// this submission.
//
// Because of wraparound, we cannot simply take tl->hwsp_offset,
// but instead use the fact that the relative for vaddr is the
// offset as for hwsp_offset. Take the top bits from tl->hwsp_offset
// and combine them with the relative offset in rq->hwsp_seqno.
//
// As rw->hwsp_seqno is rewritten when signaled, this only works
// when the request isn't signaled yet, but at that point you
// no longer need the offset.
//
extern "C" {
    pub fn i915_request_notify_execute_cb_imm(rq: *mut i915_request);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_request_state {
    I915_REQUEST_UNKNOWN = 0,
    I915_REQUEST_COMPLETE,
    I915_REQUEST_PENDING,
    I915_REQUEST_QUEUED,
    I915_REQUEST_ACTIVE,
}

extern "C" {
    pub fn i915_test_request_state(rq: *mut i915_request) -> i915_request_state;
}
extern "C" {
    pub fn i915_request_module_exit();
}
extern "C" {
    pub fn i915_request_module_init() -> c_int;
}
