//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pxp.h
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
// Copyright(c) 2024, Intel Corporation. All rights reserved.
//

extern "C" {
    pub fn xe_pxp_is_supported(xe: *const xe_device) -> bool;
}
extern "C" {
    pub fn xe_pxp_is_enabled(pxp: *const xe_pxp) -> bool;
}
extern "C" {
    pub fn xe_pxp_get_readiness_status(pxp: *mut xe_pxp) -> c_int;
}
extern "C" {
    pub fn xe_pxp_init(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pxp_irq_handler(xe: *mut xe_device, iir: u16);
}
extern "C" {
    pub fn xe_pxp_pm_suspend(pxp: *mut xe_pxp) -> c_int;
}
extern "C" {
    pub fn xe_pxp_pm_resume(pxp: *mut xe_pxp);
}
extern "C" {
    pub fn xe_pxp_exec_queue_set_type(pxp: *mut xe_pxp, q: *mut xe_exec_queue, type: u8) -> c_int;
}
extern "C" {
    pub fn xe_pxp_exec_queue_add(pxp: *mut xe_pxp, q: *mut xe_exec_queue) -> c_int;
}
extern "C" {
    pub fn xe_pxp_exec_queue_remove(pxp: *mut xe_pxp, q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_pxp_key_assign(pxp: *mut xe_pxp, bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_pxp_bo_key_check(pxp: *mut xe_pxp, bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_pxp_obj_key_check(obj: *mut drm_gem_object) -> c_int;
}
