//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_deps.h
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

//
// struct i915_deps - Collect dependencies into a single dma-fence
// @single: Storage for pointer if the collection is a single fence.
// @fences: Allocated array of fence pointers if more than a single fence;
// otherwise points to the address of @single.
// @num_deps: Current number of dependency fences.
// @fences_size: Size of the @fences array in number of pointers.
// @gfp: Allocation mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_deps {
    pub single: *mut dma_fence,
    pub fences: *mut dma_fence,
    pub num_deps: c_uint,
    pub fences_size: c_uint,
    pub gfp: gfp_t,
}

extern "C" {
    pub fn i915_deps_init(deps: *mut i915_deps, gfp: gfp_t);
}
extern "C" {
    pub fn i915_deps_fini(deps: *mut i915_deps);
}
