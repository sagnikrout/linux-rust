//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_breadcrumbs_types.h
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

//
// Rather than have every client wait upon all user interrupts,
// with the herd waking after every interrupt and each doing the
// heavyweight seqno dance, we delegate the task (of being the
// bottom-half of the user interrupt) to the first client. After
// every interrupt, we wake up one client, who does the heavyweight
// coherent seqno read and either goes back to sleep (if incomplete),
// or wakes up all the completed clients in parallel, before then
// transferring the bottom-half status to the next client in the queue.
//
// Compared to walking the entire list of waiters in a single dedicated
// bottom-half, we reduce the latency of the first waiter by avoiding
// a context switch, but incur additional coherent seqno reads when
// following the chain of request breadcrumbs. Since it is most likely
// that we have a single client waiting on each seqno, then reducing
// the overhead of waking that client is much preferred.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_breadcrumbs {
    pub ref: kref,
    pub active: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t signalers_lock; / protects the list of signalers,
    pub signalers: list_head,
    pub signaled_requests: llist_head,
    pub signaler_active: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t irq_lock; / protects the interrupt from hardirq context,
    pub /: *mut *mut irq_work irq_work; / for use from inside irq_lock,
    pub irq_enabled: c_uint,
    pub irq_armed: intel_wakeref_t,
// Not all breadcrumbs are attached to physical HW
    pub engine_mask: intel_engine_mask_t,
    pub irq_engine: *mut intel_engine_cs,
    pub b): *mut *mut bool (irq_enable)(struct intel_breadcrumbs,
    pub b): *mut *mut void (irq_disable)(struct intel_breadcrumbs,
}
