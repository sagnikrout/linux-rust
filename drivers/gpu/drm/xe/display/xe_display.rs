//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/display/xe_display.h
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
    pub fn xe_display_driver_probe_defer(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn xe_display_probe(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_display_init_early(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_display_init(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_display_register(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_unregister(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_shutdown(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_shutdown_late(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_irq_handler(xe: *mut xe_device, master_ctl: u32);
}
extern "C" {
    pub fn xe_display_irq_enable(xe: *mut xe_device, gu_misc_iir: u32);
}
extern "C" {
    pub fn xe_display_irq_reset(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_irq_postinstall(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_suspend(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_suspend_late(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_resume_early(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_resume(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_runtime_suspend(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_runtime_suspend_late(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_runtime_resume_early(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_display_pm_runtime_resume(xe: *mut xe_device);
}

pub const XE_DISPLAY_DRIVER_FEATURES: c_int = 0;

