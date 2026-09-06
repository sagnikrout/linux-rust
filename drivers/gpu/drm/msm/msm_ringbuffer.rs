//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_ringbuffer.h
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpu_submit_stats {
    pub cpcycles_start: u64,
    pub cpcycles_end: u64,
    pub alwayson_start: u64,
    pub alwayson_end: u64,
}

pub const MSM_GPU_SUBMIT_STATS_COUNT: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_rbmemptrs {
    pub rptr: volatile uint32_t,
    pub fence: volatile uint32_t,
// Introduced on A7xx
    pub bv_rptr: volatile uint32_t,
    pub bv_fence: volatile uint32_t,
    pub stats: [volatile struct msm_gpu_submit_stats; MSM_GPU_SUBMIT_STATS_COUNT],
    pub ttbr0: volatile u64,
    pub context_idr: volatile u32,
    pub perfcntr_fence: volatile u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_cp_state {
    pub ib2_base: uint64_t ib1_base,,
    pub ib2_rem: uint32_t ib1_rem,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_ringbuffer {
    pub gpu: *mut msm_gpu,
    pub id: c_int,
    pub bo: *mut drm_gem_object,
    pub next: *mut *mut *mut *mut uint32_t start, end, cur,,
//
// The job scheduler for this ring.
//
    pub sched: drm_gpu_scheduler,
    pub sched_initialized: bool,
//
// List of in-flight submits on this ring.  Protected by submit_lock.
//
// Currently just submits that are already written into the ring, not
// submits that are still in drm_gpu_scheduler's queues.  At a later
// step we could probably move to letting drm_gpu_scheduler manage
// hangcheck detection and keep track of submit jobs that are in-
// flight.
//
    pub submits: list_head,
    pub submit_lock: spinlock_t,
    pub iova: u64,
    pub hangcheck_fence: u32,
    pub memptrs: *mut msm_rbmemptrs,
    pub memptrs_iova: u64,
    pub fctx: *mut msm_fence_context,
//
// hangcheck_progress_retries:
//
// The number of extra hangcheck duration cycles that we have given
// due to it appearing that the GPU is making forward progress.
//
// For GPU generations which support progress detection (see.
// msm_gpu_funcs::progress()), if the GPU appears to be making progress
// (ie. the CP has advanced in the command stream, we'll allow up to
// DRM_MSM_HANGCHECK_PROGRESS_RETRIES expirations of the hangcheck timer
// before killing the job.  But to detect progress we need two sample
// points, so the duration of the hangcheck timer is halved.  In other
// words we'll let the submit run for up to:
//
// (DRM_MSM_HANGCHECK_DEFAULT_PERIOD / 2) * (DRM_MSM_HANGCHECK_PROGRESS_RETRIES + 1)
//
    pub hangcheck_progress_retries: c_int,
//
// last_cp_state: The state of the CP at the last call to gpu->progress()
//
    pub last_cp_state: msm_cp_state,
//
// preempt_lock protects preemption and serializes wptr updates against
// preemption.  Can be aquired from irq context.
//
    pub preempt_lock: spinlock_t,
//
// Whether we skipped writing wptr and it needs to be updated in the
// future when the ring becomes current.
//
    pub restore_wptr: bool,
//
// cur_ctx_seqno:
//
// The ctx->seqno value of the last context to submit to this ring
// Tracked by seqno rather than pointer value to avoid dangling
// pointers, and cases where a ctx can be freed and a new one created
// with the same address.
//
    pub cur_ctx_seqno: c_int,
}

extern "C" {
    pub fn msm_ringbuffer_destroy(ring: *mut msm_ringbuffer);
}
// ringbuffer helpers (the parts that are same for a3xx/a2xx/z180..)
//
// ring->next points to the current command being written - it won't be
// committed as ring->cur until the flush
//
// (ring->next++) = data;
