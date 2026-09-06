//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sched_job_types.h
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
// Copyright © 2022 Intel Corporation
//

//
// struct xe_job_ptrs - Per hw engine instance data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_job_ptrs {
// @lrc_fence: Pre-allocated uninitialized lrc fence.
    pub lrc_fence: *mut dma_fence,
// @chain_fence: Pre-allocated uninitialized fence chain node.
    pub chain_fence: *mut dma_fence_chain,
// @batch_addr: Batch buffer address.
    pub batch_addr: u64,
//
// @head: The tail pointer of the LRC (so head pointer of job) when the
// job was submitted
//
    pub head: u32,
}

//
// struct xe_sched_job - Xe schedule job (batch buffer tracking)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sched_job {
// @drm: base DRM scheduler job
    pub drm: drm_sched_job,
// @q: Exec queue
    pub q: *mut xe_exec_queue,
// @refcount: ref count of this job
    pub refcount: kref,
//
// @fence: dma fence to indicate completion. 1 way relationship - job
// can safely reference fence, fence cannot safely reference job.
//
    pub fence: *mut dma_fence,
// @user_fence: write back value when BB is complete
// @user_fence.used: user fence is used
    pub used: bool,
// @user_fence.addr: address to write to
    pub addr: u64,
// @user_fence.value: write back value
    pub value: u64,
    pub user_fence: },
// @lrc_seqno: LRC seqno
    pub lrc_seqno: u32,
// @migrate_flush_flags: Additional flush flags for migration jobs
    pub migrate_flush_flags: u32,
// @sample_timestamp: Sampling of job timestamp in TDR
    pub sample_timestamp: u64,
// @ring_ops_flush_tlb: The ring ops need to flush TLB before payload.
    pub ring_ops_flush_tlb: bool,
// @ring_ops_force_reset: The ring ops need to trigger a reset before payload.
    pub ring_ops_force_reset: bool,
// @ggtt: mapped in ggtt.
    pub ggtt: bool,
// @restore_replay: job being replayed for restore
    pub restore_replay: bool,
// @last_replay: last job being replayed
    pub last_replay: bool,
// @ptrs: per instance pointers.
    pub ptrs: [xe_job_ptrs; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sched_job_snapshot {
    pub batch_addr_len: u16,
    pub __counted_by(batch_addr_len): u64 batch_addr[],
}
