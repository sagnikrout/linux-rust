//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sched_job.h
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
// Copyright © 2021 Intel Corporation
//

pub const XE_SCHED_HANG_LIMIT: c_int = 1;

extern "C" {
    pub fn xe_sched_job_module_init() -> c_int;
}
extern "C" {
    pub fn xe_sched_job_module_exit();
}
extern "C" {
    pub fn xe_sched_job_destroy(ref: *mut kref);
}
//
// xe_sched_job_get - get reference to Xe schedule job
// @job: Xe schedule job object
//
// Increment Xe schedule job's reference count
//
// xe_sched_job_put - put reference to Xe schedule job
// @job: Xe schedule job object
//
// Decrement Xe schedule job's reference count, call xe_sched_job_destroy when
// reference count == 0.
//
extern "C" {
    pub fn xe_sched_job_set_error(job: *mut xe_sched_job, error: c_int);
}
extern "C" {
    pub fn xe_sched_job_started(job: *mut xe_sched_job) -> bool;
}
extern "C" {
    pub fn xe_sched_job_completed(job: *mut xe_sched_job) -> bool;
}
extern "C" {
    pub fn xe_sched_job_arm(job: *mut xe_sched_job);
}
extern "C" {
    pub fn xe_sched_job_push(job: *mut xe_sched_job);
}
extern "C" {
    pub fn container_of(_arg: drm, xe_sched_job: struct, _arg: drm) -> return;
}
extern "C" {
    pub fn xe_sched_job_is_migration(q: *mut xe_exec_queue) -> bool;
}
extern "C" {
    pub fn xe_sched_job_snapshot_free(snapshot: *mut xe_sched_job_snapshot);
}
extern "C" {
    pub fn xe_sched_job_snapshot_print(snapshot: *mut xe_sched_job_snapshot, p: *mut drm_printer);
}
