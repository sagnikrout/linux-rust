//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_sw_fence.h
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
// i915_sw_fence.h - library routines for N:M synchronisation points
//
// Copyright (C) 2016 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_sw_fence_notify {
    FENCE_COMPLETE,
    FENCE_FREE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_sw_fence {
    pub wait: wait_queue_head_t,
    pub fn: i915_sw_fence_notify_t,

    pub flags: c_ulong,

    pub pending: core::sync::atomic::AtomicI32,
    pub error: c_int,
}

extern "C" {
    pub fn i915_sw_fence_reinit(fence: *mut i915_sw_fence);
}

extern "C" {
    pub fn i915_sw_fence_fini(fence: *mut i915_sw_fence);
}

extern "C" {
    pub fn i915_sw_fence_commit(fence: *mut i915_sw_fence);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_sw_dma_fence_cb {
    pub base: dma_fence_cb,
    pub fence: *mut i915_sw_fence,
}

extern "C" {
    pub fn i915_sw_fence_await(fence: *mut i915_sw_fence) -> bool;
}
extern "C" {
    pub fn i915_sw_fence_complete(fence: *mut i915_sw_fence);
}
