//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/selftests/igt_live_test.h
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


//
// SPDX-License-Identifier: MIT
//
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igt_live_test {
    pub i915: *mut drm_i915_private,
    pub func: *const c_char,
    pub name: *const c_char,
    pub reset_global: c_uint,
    pub reset_engine: [c_uint; I915_MAX_GT][I915_NUM_ENGINES],
}

//
// Flush the GPU state before and after the test to ensure that no residual
// code is running on the GPU that may affect this test. Also compare the
// state before and after the test and alert if it unexpectedly changes,
// e.g. if the GPU was reset.
//
extern "C" {
    pub fn igt_live_test_end(t: *mut igt_live_test) -> c_int;
}
