//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/scheduler/sched_internal.h
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
// struct drm_sched_entity_stats - execution stats for an entity.
// @kref: reference count for the object.
// @lock: lock guarding the @runtime updates.
// @runtime: time entity spent on the GPU.
// @prev_runtime: previous @runtime used to get the runtime delta.
// @vruntime: virtual runtime as accumulated by the fair algorithm.
// @avg_job_us: average job duration.
//
// Because jobs and entities have decoupled lifetimes, ie. we cannot access the
// entity once the job has been de-queued, and we do need know how much GPU time
// each entity has spent, we need to track this in a separate object which is
// reference counted by both entities and jobs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sched_entity_stats {
    pub kref: kref,
    pub /: *mut *mut spinlock_t lock; / Protects the below fields.,
    pub runtime: ktime_t,
    pub prev_runtime: ktime_t,
    pub vruntime: ktime_t,
    pub avg_job_us: ewma_drm_sched_avgtime,
}

// Used to choose between FIFO and RR job-scheduling
pub const DRM_SCHED_POLICY_RR: c_int = 0;
pub const DRM_SCHED_POLICY_FIFO: c_int = 1;
pub const DRM_SCHED_POLICY_FAIR: c_int = 2;
extern "C" {
    pub fn drm_sched_wakeup(sched: *mut drm_gpu_scheduler);
}
extern "C" {
    pub fn drm_sched_rq_pop_entity(entity: *mut drm_sched_entity);
}
extern "C" {
    pub fn drm_sched_entity_select_rq(entity: *mut drm_sched_entity);
}
extern "C" {
    pub fn drm_sched_fence_free(fence: *mut drm_sched_fence);
}
extern "C" {
    pub fn drm_sched_fence_finished(fence: *mut drm_sched_fence, result: c_int);
}
//
// drm_sched_entity_queue_pop - Low level helper for popping queued jobs
//
// @entity: scheduler entity
//
// Low level helper for popping queued jobs.
//
// Returns: The job dequeued or NULL.
//
extern "C" {
    pub fn container_of(_arg: node, drm_sched_job: struct, _arg: queue_node) -> return;
}
//
// drm_sched_entity_queue_peek - Low level helper for peeking at the job queue
//
// @entity: scheduler entity
//
// Low level helper for peeking at the job queue
//
// Returns: The job at the head of the queue or NULL.
//
extern "C" {
    pub fn container_of(_arg: node, drm_sched_job: struct, _arg: queue_node) -> return;
}
// Return true if entity could provide a job.
extern "C" {
    pub fn drm_sched_entity_stats_release(kref: *mut kref);
}
//
// drm_sched_entity_stats_get - Obtain a reference count on &struct drm_sched_entity_stats object
// @stats: struct drm_sched_entity_stats pointer
//
// Return: struct drm_sched_entity_stats pointer
//
// drm_sched_entity_stats_put - Release a reference count on &struct drm_sched_entity_stats object
// @stats: struct drm_sched_entity_stats pointer
//
extern "C" {
    pub fn drm_sched_entity_stats_job_add_gpu_time(job: *mut drm_sched_job) -> ktime_t;
}
