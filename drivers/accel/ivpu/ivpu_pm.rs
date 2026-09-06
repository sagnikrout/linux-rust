//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_pm.h
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
// Copyright (C) 2020-2024 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_pm_info {
    pub vdev: *mut ivpu_device,
    pub job_timeout_work: delayed_work,
    pub recovery_work: work_struct,
    pub reset_lock: rw_semaphore,
    pub reset_counter: core::sync::atomic::AtomicI32,
    pub reset_pending: core::sync::atomic::AtomicI32,
    pub engine_reset_counter: core::sync::atomic::AtomicI32,
    pub dct_active_percent: u8,
}

extern "C" {
    pub fn ivpu_pm_init(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_pm_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_pm_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_pm_disable_recovery(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_pm_suspend_cb(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ivpu_pm_resume_cb(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ivpu_pm_runtime_suspend_cb(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ivpu_pm_runtime_resume_cb(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ivpu_pm_reset_prepare_cb(pdev: *mut pci_dev);
}
extern "C" {
    pub fn ivpu_pm_reset_done_cb(pdev: *mut pci_dev);
}
extern "C" {
    pub fn ivpu_rpm_get(vdev: *mut ivpu_device) -> int __must_check;
}
extern "C" {
    pub fn ivpu_rpm_put(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_pm_trigger_recovery(vdev: *mut ivpu_device, reason: *const c_char);
}
extern "C" {
    pub fn ivpu_start_job_timeout_detection(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_stop_job_timeout_detection(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_pm_dct_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_pm_dct_enable(vdev: *mut ivpu_device, active_percent: u8) -> c_int;
}
extern "C" {
    pub fn ivpu_pm_dct_disable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_pm_irq_dct_work_fn(work: *mut work_struct);
}
