//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/ext/inlines.h
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


// SPDX-License-Identifier: GPL-2.0
//
// BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
//
// Inline definitions layered on top of internal.h and cid.h.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Tejun Heo <tj@kernel.org>
//

// what dispatch concluded, consumed by the pick that follows
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_dsp_verdict {
    SCX_DSP_NONE,		/* nothing to run */
    SCX_DSP_LOCAL,		/* local DSQ has tasks */
    SCX_DSP_PREV,		/* keep running @prev */
    SCX_DSP_RETRY,		/* pick helpers only: restart the pick */
}

//
// One user of this function is scx_bpf_sub_dispatch() which can be called
// recursively as sub-sched dispatches nest. Always inline to reduce stack usage
// from the call frame.
//
// if @sch is bypassing, only the bypass DSQs are active

//
// If @sch isn't bypassing but its children are, @sch is
// responsible for making forward progress for both its own
// tasks that aren't bypassing and the bypassing descendants'
// tasks. The following implements a simple built-in behavior -
// let each CPU try to run the bypass DSQ every Nth time.
//
// Later, if necessary, we can add an ops flag to suppress the
// auto-consumption and a kfunc to consume the bypass DSQ and,
// so that the BPF scheduler can fully control scheduling of
// bypassed tasks.
//

//
// The dispatch loop. Because scx_flush_dispatch_buf() may drop the rq
// lock, the local DSQ might still end up empty after a successful
// ops.dispatch(). If the local DSQ is empty even after ops.dispatch()
// produced some tasks, retry. The BPF scheduler may depend on this
// looping behavior to simplify its implementation.
//

// stash @prev so that nested invocations can access it

//
// ops.dispatch() can trap us in this loop by repeatedly
// dispatching ineligible tasks. Break out once in a while to
// allow the watchdog to run. As IRQ can't be enabled in
// dispatch, we want to complete this scheduling cycle and then
// start a new one. IOW, we want to call resched_curr() on the
// next, most likely idle, task, not the current one. Use
// __scx_bpf_kick_cpu() for deferred kicking.
//
// Prevent the CPU from going idle while bypassed descendants have tasks
// queued. Without this fallback, bypassed tasks could stall if the host
// scheduler's ops.dispatch() doesn't yield any tasks.
//
