//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_tlb_inval_types.h
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
// Copyright © 2023 Intel Corporation
//

// struct xe_tlb_inval_ops - TLB invalidation ops (backend)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_tlb_inval_ops {
//
// @all: Invalidate all TLBs
// @tlb_inval: TLB invalidation client
// @seqno: Seqno of TLB invalidation
//
// Return 0 on success, -ECANCELED if backend is mid-reset, error on
// failure
//
    pub seqno): *mut *mut *mut int (all)(struct xe_tlb_inval tlb_inval, u32,
//
// @ggtt: Invalidate global translation TLBs
// @tlb_inval: TLB invalidation client
// @seqno: Seqno of TLB invalidation
//
// Return 0 on success, -ECANCELED if backend is mid-reset, error on
// failure
//
    pub seqno): *mut *mut *mut int (ggtt)(struct xe_tlb_inval tlb_inval, u32,
//
// @ppgtt: Invalidate per-process translation TLBs
// @tlb_inval: TLB invalidation client
// @seqno: Seqno of TLB invalidation
// @start: Start address
// @end: End address
// @asid: Address space ID
// @prl_sa: Suballocation for page reclaim list
//
// Return 0 on success, -ECANCELED if backend is mid-reset, error on
// failure
//
    pub prl_sa): *mut u64 end, u32 asid, struct drm_suballoc,
//
// @initialized: Backend is initialized
// @tlb_inval: TLB invalidation client
//
// Return: True if back is initialized, False otherwise
//
    pub tlb_inval): *mut *mut bool (initialized)(struct xe_tlb_inval,
//
// @flush: Flush pending TLB invalidations
// @tlb_inval: TLB invalidation client
//
    pub tlb_inval): *mut *mut void (flush)(struct xe_tlb_inval,
//
// @timeout_delay: Timeout delay for TLB invalidation
// @tlb_inval: TLB invalidation client
//
// Return: Timeout delay for TLB invalidation in jiffies
//
    pub tlb_inval): *mut *mut long (timeout_delay)(struct xe_tlb_inval,
}

// struct xe_tlb_inval - TLB invalidation client (frontend)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_tlb_inval {
// @private: Backend private pointer
    pub private: *mut c_void,
// @xe: Pointer to Xe device
    pub xe: *mut xe_device,
// @ops: TLB invalidation ops
    pub ops: *const xe_tlb_inval_ops,
// @tlb_inval.seqno: TLB invalidation seqno, protected by CT lock
pub const TLB_INVALIDATION_SEQNO_MAX: c_uint = 0x100000;

    pub seqno: c_int,
// @tlb_invalidation.seqno_lock: protects @tlb_invalidation.seqno
    pub seqno_lock: mutex,
//
// @seqno_recv: last received TLB invalidation seqno, protected by
// CT lock
//
    pub seqno_recv: c_int,
//
// @pending_fences: list of pending fences waiting TLB invaliations,
// protected CT lock
//
    pub pending_fences: list_head,
//
// @pending_lock: protects @pending_fences and updating @seqno_recv.
//
    pub pending_lock: spinlock_t,
//
// @fence_tdr: schedules a delayed call to xe_tlb_fence_timeout after
// the timeout interval is over.
//
    pub fence_tdr: delayed_work,
// @job_wq: schedules TLB invalidation jobs
    pub job_wq: *mut workqueue_struct,
// @tlb_inval.lock: protects TLB invalidation fences
    pub lock: spinlock_t,
// @timeout_wq: schedules TLB invalidation fence timeouts
    pub timeout_wq: *mut workqueue_struct,
}

//
// struct xe_tlb_inval_fence - TLB invalidation fence
//
// Optionally passed to xe_tlb_inval* functions and will be signaled upon TLB
// invalidation completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_tlb_inval_fence {
// @base: dma fence base
    pub base: dma_fence,
// @tlb_inval: TLB invalidation client which fence belong to
    pub tlb_inval: *mut xe_tlb_inval,
// @link: link into list of pending tlb fences
    pub link: list_head,
// @seqno: seqno of TLB invalidation to signal fence one
    pub seqno: c_int,
// @inval_time: time of TLB invalidation
    pub inval_time: ktime_t,
}

//
// struct xe_tlb_inval_batch - Batch of TLB invalidation fences
//
// Holds one fence per GT covered by a TLB invalidation request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_tlb_inval_batch {
// @fence: per-GT TLB invalidation fences
    pub XE_MAX_GT_PER_TILE]: *mut *mut xe_tlb_inval_fence fence[XE_MAX_TILES_PER_DEVICE,
// @num_fences: number of valid entries in @fence
    pub num_fences: c_uint,
}
