//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_workarounds.h
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
// Copyright © 2014-2018 Intel Corporation
//

extern "C" {
    pub fn intel_engine_init_ctx_wa(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_emit_ctx_wa(rq: *mut i915_request) -> c_int;
}
extern "C" {
    pub fn intel_gt_init_workarounds(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_apply_workarounds(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_gt_verify_workarounds(gt: *mut intel_gt, from: *const c_char) -> bool;
}
extern "C" {
    pub fn intel_engine_init_whitelist(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_apply_whitelist(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_init_workarounds(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_engine_apply_workarounds(engine: *mut intel_engine_cs);
}
