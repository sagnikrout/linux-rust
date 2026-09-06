//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gem/i915_gem_ttm.h
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
// i915_gem_to_ttm - Convert a struct drm_i915_gem_object to a
// struct ttm_buffer_object.
// @obj: Pointer to the gem object.
//
// Return: Pointer to the embedded struct ttm_buffer_object.
//
// i915 ttm gem object destructor. Internal use only.
//
extern "C" {
    pub fn i915_ttm_bo_destroy(bo: *mut ttm_buffer_object);
}
//
// i915_ttm_is_ghost_object - Check if the ttm bo is a ghost object.
// @bo: Pointer to the ttm buffer object
//
// Return: True if the ttm bo is not a i915 object but a ghost ttm object,
// False otherwise.
//
// i915_ttm_to_gem - Convert a struct ttm_buffer_object to an embedding
// struct drm_i915_gem_object.
// @bo: Pointer to the ttm buffer object
//
// Return: Pointer to the embedding struct drm_i915_gem_object.
//
extern "C" {
    pub fn container_of(_arg: bo, drm_i915_gem_object: struct, _arg: __do_not_access) -> return;
}
// Internal I915 TTM declarations and definitions below.

extern "C" {
    pub fn i915_ttm_free_cached_io_rsgt(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_ttm_adjust_lru(obj: *mut drm_i915_gem_object);
}
extern "C" {
    pub fn i915_ttm_purge(obj: *mut drm_i915_gem_object) -> c_int;
}
//
// i915_ttm_gtt_binds_lmem - Should the memory be viewed as LMEM by the GTT?
// @mem: struct ttm_resource representing the memory.
//
// Return: true if memory should be viewed as LMEM for GTT binding purposes,
// false otherwise.
//
// i915_ttm_cpu_maps_iomem - Should the memory be viewed as IOMEM by the CPU?
// @mem: struct ttm_resource representing the memory.
//
// Return: true if memory should be viewed as IOMEM for CPU mapping purposes.
//
// Once / if we support GGTT, this is also false for cached ttm_tts
extern "C" {
    pub fn i915_ttm_resource_mappable(res: *mut ttm_resource) -> bool;
}
