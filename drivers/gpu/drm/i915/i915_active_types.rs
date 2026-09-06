//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_active_types.h
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
pub struct i915_active_fence {
    pub fence: *mut dma_fence __rcu,
    pub cb: dma_fence_cb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_active {
    pub count: core::sync::atomic::AtomicI32,
    pub mutex: mutex,
    pub tree_lock: spinlock_t,
    pub cache: *mut active_node,
    pub tree: rb_root,
// Preallocated "exclusive" node
    pub excl: i915_active_fence,
    pub flags: c_ulong,

    pub ref): *mut *mut int (active)(struct i915_active,
    pub ref): *mut *mut void (retire)(struct i915_active,
    pub work: work_struct,
    pub preallocated_barriers: llist_head,
}
