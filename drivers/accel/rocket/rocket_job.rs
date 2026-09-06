//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/rocket/rocket_job.h
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
// Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocket_task {
    pub regcmd: u64,
    pub regcmd_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocket_job {
    pub base: drm_sched_job,
    pub rdev: *mut rocket_device,
    pub in_bos: *mut drm_gem_object,
    pub out_bos: *mut drm_gem_object,
    pub in_bo_count: u32,
    pub out_bo_count: u32,
    pub tasks: *mut rocket_task,
    pub task_count: u32,
    pub next_task_idx: u32,
// Fence to be signaled by drm-sched once its done with the job
    pub inference_done_fence: *mut dma_fence,
// Fence to be signaled by IRQ handler when the job is complete.
    pub done_fence: *mut dma_fence,
    pub domain: *mut rocket_iommu_domain,
    pub refcount: kref,
}

extern "C" {
    pub fn rocket_ioctl_submit(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn rocket_job_init(core: *mut rocket_core) -> c_int;
}
extern "C" {
    pub fn rocket_job_fini(core: *mut rocket_core);
}
extern "C" {
    pub fn rocket_job_open(rocket_priv: *mut rocket_file_priv) -> c_int;
}
extern "C" {
    pub fn rocket_job_close(rocket_priv: *mut rocket_file_priv);
}
extern "C" {
    pub fn rocket_job_is_idle(core: *mut rocket_core) -> c_int;
}
