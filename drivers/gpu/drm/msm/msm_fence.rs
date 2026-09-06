//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_fence.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013-2016 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

//
// struct msm_fence_context - fence context for gpu
//
// Each ringbuffer has a single fence context, with the GPU writing an
// incrementing fence seqno at the end of each submit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_fence_context {
// @dev: the drm device
    pub dev: *mut drm_device,
// @name: human readable name for fence timeline
    pub name: [c_char; 32],
// @context: see dma_fence_context_alloc()
    pub context: unsigned,
// @index: similar to context, but local to msm_fence_context's
    pub index: unsigned,
//
// @last_fence:
// Last assigned fence, incremented each time a fence is created
// on this fence context.  If last_fence == completed_fence,
// there is no remaining pending work
//
    pub last_fence: u32,
//
// @completed_fence:
// The last completed fence, updated from the CPU after interrupt
// from GPU
//
    pub completed_fence: u32,
//
// @fenceptr:
// The address that the GPU directly writes with completed fence
// seqno.  This can be ahead of completed_fence.  We can peek at
// this to see if a fence has already signaled but the CPU hasn't
// gotten around to handling the irq and updating completed_fence
//
    pub fenceptr: *mut volatile uint32_t,
//
// @spinlock: fence context spinlock
//
    pub spinlock: spinlock_t,
//
// TODO this doesn't really deal with multiple deadlines, like
// if userspace got multiple frames ahead.. OTOH atomic updates
// don't queue, so maybe that is ok
//
// @next_deadline: Time of next deadline
    pub next_deadline: ktime_t,
//
// @next_deadline_fence:
// Fence value for next pending deadline.  The deadline timer is
// canceled when this fence is signaled.
//
    pub next_deadline_fence: u32,
//
// @deadline_timer: tracks nearest deadline of a fence timeline and
// expires just before it.
//
    pub deadline_timer: hrtimer,
//
// @deadline_work: work to do after deadline_timer expires
//
    pub deadline_work: kthread_work,
}

extern "C" {
    pub fn msm_fence_context_free(fctx: *mut msm_fence_context);
}
extern "C" {
    pub fn msm_fence_completed(fctx: *mut msm_fence_context, fence: u32) -> bool;
}
extern "C" {
    pub fn msm_update_fence(fctx: *mut msm_fence_context, fence: u32);
}
extern "C" {
    pub fn msm_fence_alloc() -> *mut dma_fence;
}
extern "C" {
    pub fn msm_fence_init(fence: *mut dma_fence, fctx: *mut msm_fence_context);
}
