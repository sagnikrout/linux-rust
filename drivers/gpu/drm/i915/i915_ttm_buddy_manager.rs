//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_ttm_buddy_manager.h
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
// struct i915_ttm_buddy_resource
//
// @base: struct ttm_resource base class we extend
// @blocks: the list of struct i915_buddy_block for this resource/allocation
// @flags: DRM_BUDDY_*_ALLOCATION flags
// @used_visible_size: How much of this resource, if any, uses the CPU visible
// portion, in pages.
// @mm: the struct i915_buddy_mm for this resource
//
// Extends the struct ttm_resource to manage an address space allocation with
// one or more struct i915_buddy_block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_ttm_buddy_resource {
    pub base: ttm_resource,
    pub blocks: list_head,
    pub flags: c_ulong,
    pub used_visible_size: c_ulong,
    pub mm: *mut gpu_buddy,
}

//
// to_ttm_buddy_resource
//
// @res: the resource to upcast
//
// Upcast the struct ttm_resource object into a struct i915_ttm_buddy_resource.
//
extern "C" {
    pub fn container_of(_arg: res, i915_ttm_buddy_resource: struct, _arg: base) -> return;
}
extern "C" {
    pub fn i915_ttm_buddy_man_visible_size(man: *mut ttm_resource_manager) -> u64;
}

