//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_hwconfig.h
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
    pub fn xe_guc_hwconfig_init(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_hwconfig_size(guc: *mut xe_guc) -> u32;
}
extern "C" {
    pub fn xe_guc_hwconfig_copy(guc: *mut xe_guc, dst: *mut c_void);
}
extern "C" {
    pub fn xe_guc_hwconfig_dump(guc: *mut xe_guc, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_hwconfig_lookup_u32(guc: *mut xe_guc, attribute: u32, val: *mut u32) -> c_int;
}
