//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gpu_scheduler.h
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

extern "C" {
    pub fn xe_sched_fini(sched: *mut xe_gpu_scheduler);
}
extern "C" {
    pub fn xe_sched_submission_start(sched: *mut xe_gpu_scheduler);
}
extern "C" {
    pub fn xe_sched_submission_stop(sched: *mut xe_gpu_scheduler);
}
extern "C" {
    pub fn xe_sched_submission_resume_tdr(sched: *mut xe_gpu_scheduler);
}
extern "C" {
    pub fn drm_sched_invalidate_job(_arg: &job->drm, _arg: threshold) -> return;
}
//
// xe_sched_first_pending_job() - Find first pending job which is unsignaled
// @sched: Xe GPU scheduler
//
// Return first unsignaled job in pending list or NULL
//
extern "C" {
    pub fn to_xe_sched_job(_arg: job) -> return;
}

