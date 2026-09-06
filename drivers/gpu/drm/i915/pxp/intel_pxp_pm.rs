//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/pxp/intel_pxp_pm.h
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
// Copyright(c) 2020, Intel Corporation. All rights reserved.
//

extern "C" {
    pub fn intel_pxp_suspend_prepare(pxp: *mut intel_pxp);
}
extern "C" {
    pub fn intel_pxp_suspend(pxp: *mut intel_pxp);
}
extern "C" {
    pub fn intel_pxp_resume_complete(pxp: *mut intel_pxp);
}
extern "C" {
    pub fn intel_pxp_runtime_suspend(pxp: *mut intel_pxp);
}
extern "C" {
    pub fn intel_pxp_runtime_resume(pxp: *mut intel_pxp);
}

