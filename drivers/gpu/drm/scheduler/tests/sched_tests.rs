//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/scheduler/tests/sched_tests.h
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
// Copyright (c) 2025 Valve Corporation

//
// DOC: Mock DRM scheduler data structures
//
// drm_mock_* data structures are used to implement a mock "GPU".
//
// They subclass the core DRM scheduler objects and add their data on top, which
// enables tracking the submitted jobs and simulating their execution with the
// attributes as specified by the test case.
//
// struct drm_mock_scheduler - implements a trivial mock GPU execution engine
//
// @base: DRM scheduler base class
// @test: Backpointer to owning the kunit test case
// @lock: Lock to protect the simulated @hw_timeline and @job_list
// @job_list: List of jobs submitted to the mock GPU
// @hw_timeline: Simulated hardware timeline has a @context, @next_seqno and
// @cur_seqno for implementing a struct dma_fence signaling the
// simulated job completion.
//
// Trivial mock GPU execution engine tracks submitted jobs and enables
// completing them strictly in submission order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mock_scheduler {
    pub base: drm_gpu_scheduler,
    pub test: *mut kunit,
    pub lock: spinlock_t,
    pub job_list: list_head,
    pub context: u64,
    pub next_seqno: core::sync::atomic::AtomicI32,
    pub cur_seqno: c_uint,
    pub hw_timeline: },
}

//
// struct drm_mock_sched_entity - implements a mock GPU sched entity
//
// @base: DRM scheduler entity base class
// @test: Backpointer to owning the kunit test case
//
// Mock GPU sched entity is used by the test cases to submit jobs to the mock
// scheduler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mock_sched_entity {
    pub base: drm_sched_entity,
    pub test: *mut kunit,
}

//
// struct drm_mock_sched_job - implements a mock GPU job
//
// @base: DRM sched job base class
// @done: Completion signaling job completion.
// @flags: Flags designating job state.
// @link: List head element used by job tracking by the drm_mock_scheduler
// @timer: Timer used for simulating job execution duration
// @duration_us: Simulated job duration in micro seconds, or zero if in manual
// timeline advance mode
// @finish_at: Absolute time when the jobs with set duration will complete
// @lock: Lock used for @hw_fence
// @hw_fence: Fence returned to DRM scheduler as the hardware fence
// @test: Backpointer to owning the kunit test case
//
// Mock GPU sched job is used by the test cases to submit jobs to the mock
// scheduler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mock_sched_job {
    pub base: drm_sched_job,
    pub done: completion,
pub const DRM_MOCK_SCHED_JOB_DONE: c_uint = 0x1;
pub const DRM_MOCK_SCHED_JOB_TIMEDOUT: c_uint = 0x2;
pub const DRM_MOCK_SCHED_JOB_DONT_RESET: c_uint = 0x4;
pub const DRM_MOCK_SCHED_JOB_RESET_SKIPPED: c_uint = 0x8;
    pub flags: c_ulong,
    pub link: list_head,
    pub timer: hrtimer,
    pub duration_us: c_uint,
    pub finish_at: ktime_t,
    pub hw_fence: dma_fence,
    pub test: *mut kunit,
}

extern "C" {
    pub fn container_of(_arg: sched, drm_mock_scheduler: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: sched_entity, drm_mock_sched_entity: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: sched_job, drm_mock_sched_job: struct, _arg: base) -> return;
}
extern "C" {
    pub fn drm_mock_sched_fini(sched: *mut drm_mock_scheduler);
}
extern "C" {
    pub fn drm_mock_sched_entity_free(entity: *mut drm_mock_sched_entity);
}
//
// drm_mock_sched_job_submit - Arm and submit a job in one go
//
// @job: Job to arm and submit
//
// drm_mock_sched_job_set_duration_us - Set a job duration
//
// @job: Job to set the duration for
// @duration_us: Duration in micro seconds
//
// Jobs with duration set will be automatically completed by the mock scheduler
// as the timeline progresses, unless a job without a set duration is
// encountered in the timelime in which case calling drm_mock_sched_advance()
// will be required to bump the timeline.
//
// drm_mock_sched_job_is_finished - Check if a job is finished
//
// @job: Job to check
//
// Returns: true if finished
//
// drm_mock_sched_job_wait_finished - Wait until a job is finished
//
// @job: Job to wait for
// @timeout: Wait time in jiffies
//
// Returns: true if finished within the timeout provided, otherwise false
//
// drm_mock_sched_job_wait_scheduled - Wait until a job is scheduled
//
// @job: Job to wait for
// @timeout: Wait time in jiffies
//
// Returns: true if scheduled within the timeout provided, otherwise false
//
