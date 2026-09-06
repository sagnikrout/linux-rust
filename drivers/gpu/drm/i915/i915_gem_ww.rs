//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_gem_ww.h
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
pub struct i915_gem_ww_ctx {
    pub ctx: ww_acquire_ctx,
    pub obj_list: list_head,
    pub contended: *mut drm_i915_gem_object,
    pub intr: bool,
}

extern "C" {
    pub fn i915_gem_ww_ctx_init(ctx: *mut i915_gem_ww_ctx, intr: bool);
}
extern "C" {
    pub fn i915_gem_ww_ctx_fini(ctx: *mut i915_gem_ww_ctx);
}
extern "C" {
    pub fn i915_gem_ww_ctx_backoff(ctx: *mut i915_gem_ww_ctx) -> int __must_check;
}
extern "C" {
    pub fn i915_gem_ww_unlock_single(obj: *mut drm_i915_gem_object);
}
// Internal function used by the inlines! Don't use.

