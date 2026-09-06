//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_job.h
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

// Forward declaration from "pvr_context.h".
// Forward declarations from "pvr_device.h".
// Forward declarations from "pvr_hwrt.h".
// Forward declaration from "pvr_queue.h".
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_job {
// @base: drm_sched_job object.
    pub base: drm_sched_job,
// @ref_count: Refcount for job.
    pub ref_count: kref,
// @type: Type of job.
    pub type: drm_pvr_job_type,
// @id: Job ID number.
    pub id: u32,
//
// @paired_job: Job paired to this job.
//
// This field is only meaningful for geometry and fragment jobs.
//
// Paired jobs are executed on the same context, and need to be submitted
// atomically to the FW, to make sure the partial render logic has a
// fragment job to execute when the Parameter Manager runs out of memory.
//
// The geometry job should point to the fragment job it's paired with,
// and the fragment job should point to the geometry job it's paired with.
//
    pub paired_job: *mut pvr_job,
// @cccb_fence: Fence used to wait for CCCB space.
    pub cccb_fence: *mut dma_fence,
// @kccb_fence: Fence used to wait for KCCB space.
    pub kccb_fence: *mut dma_fence,
// @done_fence: Fence to signal when the job is done.
    pub done_fence: *mut dma_fence,
// @pvr_dev: Device pointer.
    pub pvr_dev: *mut pvr_device,
// @ctx: Pointer to owning context.
    pub ctx: *mut pvr_context,
// @cmd: Command data. Format depends on @type.
    pub cmd: *mut c_void,
// @cmd_len: Length of command data, in bytes.
    pub cmd_len: u32,
//
// @fw_ccb_cmd_type: Firmware CCB command type. Must be one of %ROGUE_FWIF_CCB_CMD_TYPE_*.
//
    pub fw_ccb_cmd_type: u32,
// @hwrt: HWRT object. Will be NULL for compute and transfer jobs.
    pub hwrt: *mut pvr_hwrt_data,
//
// @has_pm_ref: True if the job has a power ref, thus forcing the GPU to stay on until
// the job is done.
//
    pub has_pm_ref: bool,
}

//
// pvr_job_get() - Take additional reference on job.
// @job: Job pointer.
//
// Call pvr_job_put() to release.
//
// Returns:
// * The requested job on success, or
// * %NULL if no job pointer passed.
//
extern "C" {
    pub fn pvr_job_put(job: *mut pvr_job);
}
//
// pvr_job_release_pm_ref() - Release the PM ref if the job acquired it.
// @job: The job to release the PM ref on.
//
// pvr_job_get_pm_ref() - Get a PM ref and attach it to the job.
// @job: The job to attach the PM ref to.
//
// Return:
// * 0 on success, or
// * Any error returned by pvr_power_get() otherwise.
//
extern "C" {
    pub fn pvr_job_wait_first_non_signaled_native_dep(job: *mut pvr_job) -> c_int;
}
extern "C" {
    pub fn pvr_job_non_native_deps_done(job: *mut pvr_job) -> bool;
}
extern "C" {
    pub fn pvr_job_fits_in_cccb(job: *mut pvr_job, native_dep_count: c_ulong) -> c_int;
}
extern "C" {
    pub fn pvr_job_submit(job: *mut pvr_job);
}
