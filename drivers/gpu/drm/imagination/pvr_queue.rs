//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_queue.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// struct pvr_queue_fence_ctx - Queue fence context
//
// Used to implement dma_fence_ops for pvr_job::{done,cccb}_fence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_queue_fence_ctx {
// @id: Fence context ID allocated with dma_fence_context_alloc().
    pub id: u64,
// @seqno: Sequence number incremented each time a fence is created.
    pub seqno: core::sync::atomic::AtomicI32,
// @lock: Lock used to synchronize access to fences allocated by this context.
    pub lock: spinlock_t,
}

//
// struct pvr_queue_cccb_fence_ctx - CCCB fence context
//
// Context used to manage fences controlling access to the CCCB. No fences are
// issued if there's enough space in the CCCB to push job commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_queue_cccb_fence_ctx {
// @base: Base queue fence context.
    pub base: pvr_queue_fence_ctx,
//
// @job: Job waiting for CCCB space.
//
// Thanks to the serializationg done at the drm_sched_entity level,
// there's no more than one job waiting for CCCB at a given time.
//
// This field is NULL if no jobs are currently waiting for CCCB space.
//
// Must be accessed with @job_lock held.
//
    pub job: *mut pvr_job,
// @job_lock: Lock protecting access to the job object.
    pub job_lock: mutex,
}

//
// struct pvr_queue_fence - Queue fence object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_queue_fence {
// @base: Base dma_fence.
    pub base: dma_fence,
// @queue: Queue that created this fence.
    pub queue: *mut pvr_queue,
// @release_work: Fence release work structure.
    pub release_work: work_struct,
}

//
// struct pvr_queue - Job queue
//
// Used to queue and track execution of pvr_job objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_queue {
// @scheduler: Single entity scheduler use to push jobs to this queue.
    pub scheduler: drm_gpu_scheduler,
// @entity: Scheduling entity backing this queue.
    pub entity: drm_sched_entity,
// @type: Type of jobs queued to this queue.
    pub type: drm_pvr_job_type,
// @ctx: Context object this queue is bound to.
    pub ctx: *mut pvr_context,
// @node: Used to add the queue to the active/idle queue list.
    pub node: list_head,
//
// @in_flight_job_count: Number of jobs submitted to the CCCB that
// have not been processed yet.
//
    pub in_flight_job_count: core::sync::atomic::AtomicI32,
//
// @cccb_fence_ctx: CCCB fence context.
//
// Used to control access to the CCCB is full, such that we don't
// end up trying to push commands to the CCCB if there's not enough
// space to receive all commands needed for a job to complete.
//
    pub cccb_fence_ctx: pvr_queue_cccb_fence_ctx,
// @job_fence_ctx: Job fence context object.
    pub job_fence_ctx: pvr_queue_fence_ctx,
// @timeline_ufo: Timeline UFO for the context queue.
// @fw_obj: FW object representing the UFO value.
    pub fw_obj: *mut pvr_fw_object,
// @value: CPU mapping of the UFO value.
    pub value: *mut u32,
    pub timeline_ufo: },
//
// @last_queued_job_scheduled_fence: The scheduled fence of the last
// job queued to this queue.
//
// We use it to insert frag -> geom dependencies when issuing combined
// geom+frag jobs, to guarantee that the fragment job that's part of
// the combined operation comes after all fragment jobs that were queued
// before it.
//
    pub last_queued_job_scheduled_fence: *mut dma_fence,
// @cccb: Client Circular Command Buffer.
    pub cccb: pvr_cccb,
// @reg_state_obj: FW object representing the register state of this queue.
    pub reg_state_obj: *mut pvr_fw_object,
// @ctx_offset: Offset of the queue context in the FW context object.
    pub ctx_offset: u32,
// @callstack_addr: Initial call stack address for register state object.
    pub callstack_addr: u64,
}

extern "C" {
    pub fn pvr_queue_fence_is_native(f: *mut dma_fence) -> bool;
}
extern "C" {
    pub fn pvr_queue_job_init(job: *mut pvr_job, drm_client_id: u64) -> c_int;
}
extern "C" {
    pub fn pvr_queue_job_cleanup(job: *mut pvr_job);
}
extern "C" {
    pub fn pvr_queue_job_push(job: *mut pvr_job);
}
extern "C" {
    pub fn pvr_queue_kill(queue: *mut pvr_queue);
}
extern "C" {
    pub fn pvr_queue_destroy(queue: *mut pvr_queue, cleanup_queue_entity: bool);
}
extern "C" {
    pub fn pvr_queue_process(queue: *mut pvr_queue);
}
extern "C" {
    pub fn pvr_queue_device_pre_reset(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_queue_device_post_reset(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_queue_device_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_queue_device_fini(pvr_dev: *mut pvr_device);
}
