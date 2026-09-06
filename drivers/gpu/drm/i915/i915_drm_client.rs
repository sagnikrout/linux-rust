//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_drm_client.h
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
// Copyright © 2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_drm_client {
    pub kref: kref,
    pub /: *mut *mut spinlock_t ctx_lock; / For add/remove from ctx_list.,
    pub /: *mut *mut list_head ctx_list; / List of contexts belonging to client.,

//
// @objects_lock: lock protecting @objects_list
//
    pub objects_lock: spinlock_t,
//
// @objects_list: list of objects created by this client
//
// Protected by @objects_lock.
//
    pub objects_list: list_head,

//
// @past_runtime: Accumulation of pphwsp runtimes from closed contexts.
//
    pub 1]: atomic64_t past_runtime[I915_LAST_UABI_ENGINE_CLASS +,
}

extern "C" {
    pub fn __i915_drm_client_free(kref: *mut kref);
}
extern "C" {
    pub fn i915_drm_client_fdinfo(p: *mut drm_printer, file: *mut drm_file);
}

extern "C" {
    pub fn i915_drm_client_remove_object(obj: *mut drm_i915_gem_object);
}

