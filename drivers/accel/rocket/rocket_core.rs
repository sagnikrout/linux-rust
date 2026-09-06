//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/rocket/rocket_core.h
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
pub struct rocket_core {
    pub dev: *mut device,
    pub rdev: *mut rocket_device,
    pub index: c_uint,
    pub irq: c_int,
    pub pc_iomem: *mut void __iomem,
    pub cna_iomem: *mut void __iomem,
    pub core_iomem: *mut void __iomem,
    pub clks: [clk_bulk_data; 4],
    pub resets: [reset_control_bulk_data; 2],
    pub iommu_group: *mut iommu_group,
    pub job_lock: mutex,
    pub in_flight_job: *mut rocket_job,
    pub fence_lock: spinlock_t,
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub pending: core::sync::atomic::AtomicI32,
    pub reset: },
    pub sched: drm_gpu_scheduler,
    pub fence_context: u64,
    pub emit_seqno: u64,
}

extern "C" {
    pub fn rocket_core_init(core: *mut rocket_core) -> c_int;
}
extern "C" {
    pub fn rocket_core_fini(core: *mut rocket_core);
}
extern "C" {
    pub fn rocket_core_reset(core: *mut rocket_core);
}
