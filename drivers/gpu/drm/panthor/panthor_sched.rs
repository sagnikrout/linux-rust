//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_sched.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2023 Collabora ltd.
extern "C" {
    pub fn panthor_group_destroy(pfile: *mut panthor_file, group_handle: u32) -> c_int;
}
extern "C" {
    pub fn panthor_job_put(job: *mut drm_sched_job);
}
extern "C" {
    pub fn panthor_job_update_resvs(exec: *mut drm_exec, job: *mut drm_sched_job);
}
extern "C" {
    pub fn panthor_group_pool_create(pfile: *mut panthor_file) -> c_int;
}
extern "C" {
    pub fn panthor_group_pool_destroy(pfile: *mut panthor_file);
}
extern "C" {
    pub fn panthor_sched_init(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_sched_unplug(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_sched_pre_reset(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_sched_post_reset(ptdev: *mut panthor_device, reset_failed: bool);
}
extern "C" {
    pub fn panthor_sched_suspend(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_sched_resume(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_sched_report_mmu_fault(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_sched_prepare_for_vm_destruction(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_sched_report_fw_events(ptdev: *mut panthor_device, events: u32);
}
extern "C" {
    pub fn panthor_fdinfo_gather_group_samples(pfile: *mut panthor_file);
}
