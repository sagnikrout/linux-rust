//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dsb.h
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
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_dsb_id {
    INTEL_DSB_0,
    INTEL_DSB_1,
    INTEL_DSB_2,

    I915_MAX_DSBS,
}

extern "C" {
    pub fn intel_dsb_size(dsb: *mut intel_dsb) -> c_uint;
}
extern "C" {
    pub fn intel_dsb_head(dsb: *mut intel_dsb) -> c_uint;
}
extern "C" {
    pub fn intel_dsb_finish(dsb: *mut intel_dsb);
}
extern "C" {
    pub fn intel_dsb_gosub_finish(dsb: *mut intel_dsb);
}
extern "C" {
    pub fn intel_dsb_cleanup(dsb: *mut intel_dsb);
}
extern "C" {
    pub fn intel_dsb_exec_time_us() -> c_int;
}
extern "C" {
    pub fn intel_dsb_noop(dsb: *mut intel_dsb, count: c_int);
}
extern "C" {
    pub fn intel_dsb_nonpost_start(dsb: *mut intel_dsb);
}
extern "C" {
    pub fn intel_dsb_nonpost_end(dsb: *mut intel_dsb);
}
extern "C" {
    pub fn intel_dsb_interrupt(dsb: *mut intel_dsb);
}
extern "C" {
    pub fn intel_dsb_wait_usec(dsb: *mut intel_dsb, count: c_int);
}
extern "C" {
    pub fn intel_dsb_wait_vblanks(dsb: *mut intel_dsb, count: c_int);
}
extern "C" {
    pub fn intel_dsb_commit(dsb: *mut intel_dsb);
}
extern "C" {
    pub fn intel_dsb_wait(dsb: *mut intel_dsb);
}
