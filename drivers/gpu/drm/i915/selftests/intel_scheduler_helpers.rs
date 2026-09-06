//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/selftests/intel_scheduler_helpers.h
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
// Copyright © 2021 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_selftest_saved_policy {
    pub flags: u32,
    pub reset: u32,
    pub timeslice: u64,
    pub preempt_timeout: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum selftest_scheduler_modify {
    SELFTEST_SCHEDULER_MODIFY_NO_HANGCHECK = 0,
    SELFTEST_SCHEDULER_MODIFY_FAST_RESET,
}

extern "C" {
    pub fn intel_selftest_wait_for_rq(rq: *mut i915_request) -> c_int;
}
