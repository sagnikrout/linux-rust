//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gem/i915_gem_region.h
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

//
// struct i915_gem_apply_to_region_ops - ops to use when iterating over all
// region objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_apply_to_region_ops {
//
// @process_obj: Process the current object
//
// Note that if this function is part of a ww transaction, and
// if returns -EDEADLK for one of the objects, it may be
// rerun for that same object in the same pass.
//
    pub obj): *mut drm_i915_gem_object,
}

//
// struct i915_gem_apply_to_region - Argument to the struct
// i915_gem_apply_to_region_ops functions.
// @ops: The ops for the operation.
// @ww: Locking context used for the transaction.
// @interruptible: Whether to perform object locking interruptible.
//
// This structure is intended to be embedded in a private struct if needed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_apply_to_region {
    pub ops: *const i915_gem_apply_to_region_ops,
    pub ww: *mut i915_gem_ww_ctx,
    pub interruptible:1: u32,
}

extern "C" {
    pub fn i915_gem_object_release_memory_region(obj: *mut drm_i915_gem_object);
}
