//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_power.h
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

extern "C" {
    pub fn pvr_watchdog_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_watchdog_fini(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_device_lost(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_power_is_idle(pvr_dev: *mut pvr_device) -> bool;
}
extern "C" {
    pub fn pvr_power_device_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pvr_power_device_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pvr_power_device_idle(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pvr_power_reset(pvr_dev: *mut pvr_device, hard_reset: bool) -> c_int;
}
extern "C" {
    pub fn pm_runtime_resume_and_get(_arg: drm_dev->dev) -> return;
}
extern "C" {
    pub fn pvr_power_domains_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_power_domains_fini(pvr_dev: *mut pvr_device);
}
//
// struct pvr_power_sequence_ops - Platform specific power sequence operations.
// @init: Pointer to the platform-specific initialization function.
// @power_on: Pointer to the platform-specific power on function.
// @power_off: Pointer to the platform-specific power off function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_power_sequence_ops {
    pub pvr_dev): *mut *mut int (init)(struct pvr_device,
    pub pvr_dev): *mut *mut int (power_on)(struct pvr_device,
    pub pvr_dev): *mut *mut int (power_off)(struct pvr_device,
}
