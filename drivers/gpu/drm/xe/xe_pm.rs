//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pm.h
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
// Copyright © 2022 Intel Corporation
//

extern "C" {
    pub fn xe_pm_suspend(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_resume(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_init_early(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_probe(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_init(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_fini(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_pm_runtime_suspended(xe: *mut xe_device) -> bool;
}
extern "C" {
    pub fn xe_pm_runtime_suspend(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_runtime_resume(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_runtime_get(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_pm_runtime_get_ioctl(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_runtime_put(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_pm_runtime_get_if_active(xe: *mut xe_device) -> bool;
}
extern "C" {
    pub fn xe_pm_runtime_get_if_in_use(xe: *mut xe_device) -> bool;
}
extern "C" {
    pub fn xe_pm_runtime_get_noresume(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_pm_runtime_resume_and_get(xe: *mut xe_device) -> bool;
}
extern "C" {
    pub fn xe_pm_assert_unbounded_bridge(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_pm_set_vram_threshold(xe: *mut xe_device, threshold: u32) -> c_int;
}
extern "C" {
    pub fn xe_pm_d3cold_allowed_toggle(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_rpm_reclaim_safe(xe: *const xe_device) -> bool;
}
extern "C" {
    pub fn xe_pm_block_on_suspend(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pm_might_block_on_suspend();
}
extern "C" {
    pub fn xe_pm_module_init() -> c_int;
}
//
// Used when a function needs to release runtime PM in all possible cases
// and error paths, but the wakeref was already acquired by a different
// function (i.e., get() has already happened so only a put() is needed).
//
