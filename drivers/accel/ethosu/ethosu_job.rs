//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ethosu/ethosu_job.h
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
// Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net>
// Copyright 2025 Arm, Ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethosu_job {
    pub base: drm_sched_job,
    pub dev: *mut ethosu_device,
    pub cmd_bo: *mut drm_gem_object,
    pub region_bo: [*mut drm_gem_object; NPU_BASEP_REGION_MAX],
    pub region_bo_num: [u8; NPU_BASEP_REGION_MAX],
    pub region_cnt: u8,
    pub sram_size: u32,
    pub perfmon: *mut ethosu_perfmon,
// Fence to be signaled by drm-sched once its done with the job
    pub inference_done_fence: *mut dma_fence,
// Fence to be signaled by IRQ handler when the job is complete.
    pub done_fence: *mut dma_fence,
    pub refcount: kref,
}

extern "C" {
    pub fn ethosu_ioctl_submit(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ethosu_job_init(dev: *mut ethosu_device) -> c_int;
}
extern "C" {
    pub fn ethosu_job_fini(dev: *mut ethosu_device);
}
extern "C" {
    pub fn ethosu_job_open(ethosu_priv: *mut ethosu_file_priv) -> c_int;
}
extern "C" {
    pub fn ethosu_job_close(ethosu_priv: *mut ethosu_file_priv);
}
