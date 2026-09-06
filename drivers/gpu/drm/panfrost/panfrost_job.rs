//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panfrost/panfrost_job.h
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
// Copyright 2019 Collabora ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_job {
    pub base: drm_sched_job,
    pub refcount: kref,
    pub pfdev: *mut panfrost_device,
    pub mmu: *mut panfrost_mmu,
    pub ctx: *mut panfrost_jm_ctx,
// Fence to be signaled by IRQ handler when the job is complete.
    pub done_fence: *mut dma_fence,
    pub jc: __u64,
    pub requirements: __u32,
    pub flush_id: __u32,
    pub mappings: *mut panfrost_gem_mapping,
    pub bos: *mut drm_gem_object,
    pub bo_count: u32,
// Fence to be signaled by drm-sched once its done with the job
    pub render_done_fence: *mut dma_fence,
    pub engine_usage: *mut panfrost_engine_usage,
    pub is_profiled: bool,
    pub start_time: ktime_t,
    pub start_cycles: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_js_ctx {
    pub sched_entity: drm_sched_entity,
    pub enabled: bool,
}

pub const NUM_JOB_SLOTS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_jm_ctx {
    pub refcnt: kref,
    pub destroyed: bool,
    pub slot_entity: [drm_sched_entity; NUM_JOB_SLOTS],
}

extern "C" {
    pub fn panfrost_jm_ctx_destroy(file: *mut drm_file, handle: u32) -> c_int;
}
extern "C" {
    pub fn panfrost_jm_ctx_put(jm_ctx: *mut panfrost_jm_ctx);
}
extern "C" {
    pub fn panfrost_jm_init(pfdev: *mut panfrost_device) -> c_int;
}
extern "C" {
    pub fn panfrost_jm_fini(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_jm_open(file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn panfrost_jm_close(file: *mut drm_file);
}
extern "C" {
    pub fn panfrost_jm_reset_interrupts(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_jm_enable_interrupts(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_jm_suspend_irq(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_jm_is_idle(pfdev: *mut panfrost_device) -> c_int;
}
extern "C" {
    pub fn panfrost_job_get_slot(job: *mut panfrost_job) -> c_int;
}
extern "C" {
    pub fn panfrost_job_push(job: *mut panfrost_job) -> c_int;
}
extern "C" {
    pub fn panfrost_job_put(job: *mut panfrost_job);
}
