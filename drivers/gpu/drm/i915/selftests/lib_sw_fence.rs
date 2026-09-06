//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/selftests/lib_sw_fence.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// lib_sw_fence.h - library routines for testing N:M synchronisation points
//
// Copyright (C) 2017 Intel Corporation
//

extern "C" {
    pub fn onstack_fence_fini(fence: *mut i915_sw_fence);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timed_fence {
    pub fence: i915_sw_fence,
    pub timer: timer_list,
}

extern "C" {
    pub fn timed_fence_init(tf: *mut timed_fence, expires: c_ulong);
}
extern "C" {
    pub fn timed_fence_fini(tf: *mut timed_fence);
}
extern "C" {
    pub fn heap_fence_put(fence: *mut i915_sw_fence);
}
