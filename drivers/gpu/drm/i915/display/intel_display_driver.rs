//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_driver.h
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
// Copyright © 2022-2023 Intel Corporation
//

extern "C" {
    pub fn intel_display_driver_probe_defer(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn intel_display_driver_init_hw(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_early_probe(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_probe_noirq(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_display_driver_probe_nogem(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_display_driver_probe(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_display_driver_register(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_remove(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_remove_noirq(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_remove_nogem(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_unregister(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_shutdown(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_shutdown_late(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_pm_suspend(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_display_driver_pm_suspend_late(display: *mut intel_display, s2idle: bool);
}
extern "C" {
    pub fn intel_display_driver_pm_resume_early(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_pm_resume(display: *mut intel_display);
}
// interface for intel_display_reset.c
extern "C" {
    pub fn intel_display_driver_enable_user_access(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_disable_user_access(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_suspend_access(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_resume_access(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_check_access(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_display_driver_runtime_pm_enable(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_runtime_pm_disable(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_pm_runtime_suspend(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_pm_runtime_suspend_late(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_pm_runtime_resume_early(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_driver_pm_runtime_resume(display: *mut intel_display);
}
