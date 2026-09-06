//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc_ads.h
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
// Copyright © 2014-2019 Intel Corporation
//

extern "C" {
    pub fn intel_guc_ads_create(guc: *mut intel_guc) -> c_int;
}
extern "C" {
    pub fn intel_guc_ads_destroy(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_ads_init_late(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_ads_reset(guc: *mut intel_guc);
}
extern "C" {
    pub fn intel_guc_engine_usage_record_map(engine: *mut intel_engine_cs) -> iosys_map;
}
extern "C" {
    pub fn intel_guc_engine_usage_offset(guc: *mut intel_guc) -> u32;
}
