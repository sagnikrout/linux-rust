//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_job.h
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
// Copyright (C) 2020-2025 Intel Corporation
//

//
// struct ivpu_cmdq - Represents a command queue for submitting jobs to the VPU.
// Tracks queue memory, preemption buffers, and metadata for job management.
// @jobq:                Pointer to job queue memory shared with the device
// @primary_preempt_buf: Primary preemption buffer for this queue (optional)
// @secondary_preempt_buf: Secondary preemption buffer for this queue (optional)
// @mem:                 Memory allocated for the job queue, shared with device
// @entry_count:         Number of job entries in the queue
// @id:                  Unique command queue ID
// @db_id:               Doorbell ID assigned to this job queue
// @priority:            Priority level of the command queue
// @is_legacy:           True if this is a legacy command queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_cmdq {
    pub jobq: *mut vpu_job_queue,
    pub primary_preempt_buf: *mut ivpu_bo,
    pub secondary_preempt_buf: *mut ivpu_bo,
    pub mem: *mut ivpu_bo,
    pub entry_count: u32,
    pub id: u32,
    pub db_id: u32,
    pub priority: u8,
    pub is_legacy: bool,
}

//
// struct ivpu_job - Representing a batch or DMA buffer submitted to the VPU.
// Each job is a unit of execution, tracked by job_id for status reporting from VPU FW.
// The structure holds all resources and metadata needed for job submission, execution,
// and completion handling.
// @vdev:                Pointer to the VPU device
// @file_priv:           The client context that submitted this job
// @done_fence:          Fence signaled when job completes
// @destroy_node:        List node for deferred resource cleanup after job completion
// @cmd_buf_vpu_addr:    VPU address of the command buffer for this job
// @cmdq_id:             Command queue ID used for submission
// @job_id:              Unique job ID for tracking and status reporting
// @engine_idx:          Engine index for job execution
// @job_status:          Status reported by firmware for this job
// @primary_preempt_buf: Primary preemption buffer for job
// @secondary_preempt_buf: Secondary preemption buffer for job (optional)
// @bo_count:            Number of buffer objects associated with this job
// @bos:                 Array of buffer objects used by the job (batch buffer is at index 0)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_job {
    pub vdev: *mut ivpu_device,
    pub file_priv: *mut ivpu_file_priv,
    pub done_fence: *mut dma_fence,
    pub destroy_node: llist_node,
    pub cmd_buf_vpu_addr: u64,
    pub cmdq_id: u32,
    pub job_id: u32,
    pub engine_idx: u32,
    pub job_status: u32,
    pub primary_preempt_buf: *mut ivpu_bo,
    pub secondary_preempt_buf: *mut ivpu_bo,
    pub bo_count: usize,
    pub __counted_by(bo_count): *mut *mut ivpu_bo bos[],
}

extern "C" {
    pub fn ivpu_submit_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_cmdq_create_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_cmdq_destroy_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_cmdq_submit_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_context_abort_locked(file_priv: *mut ivpu_file_priv);
}
extern "C" {
    pub fn ivpu_cmdq_release_all_locked(file_priv: *mut ivpu_file_priv);
}
extern "C" {
    pub fn ivpu_cmdq_reset_all_contexts(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_cmdq_abort_all_jobs(vdev: *mut ivpu_device, ctx_id: u32, cmdq_id: u32);
}
extern "C" {
    pub fn ivpu_job_done_consumer_init(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_job_done_consumer_fini(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_job_handle_engine_error(vdev: *mut ivpu_device, job_id: u32, job_status: u32) -> bool;
}
extern "C" {
    pub fn ivpu_context_abort_work_fn(work: *mut work_struct);
}
extern "C" {
    pub fn ivpu_job_destroy_work_fn(work: *mut work_struct);
}
extern "C" {
    pub fn ivpu_jobs_abort_all(vdev: *mut ivpu_device);
}
