//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gem/i915_gem_context.h
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
// Copyright © 2016 Intel Corporation
//

extern "C" {
    pub fn test_bit(_arg: CONTEXT_CLOSED, _arg: &ctx->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: UCONTEXT_NO_ERROR_CAPTURE, _arg: &ctx->user_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: UCONTEXT_BANNABLE, _arg: &ctx->user_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: UCONTEXT_RECOVERABLE, _arg: &ctx->user_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: UCONTEXT_PERSISTENCE, _arg: &ctx->user_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CONTEXT_USER_ENGINES, _arg: &ctx->flags) -> return;
}
// i915_gem_context.c
extern "C" {
    pub fn i915_gem_init__contexts(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_gem_context_close(file: *mut drm_file);
}
extern "C" {
    pub fn i915_gem_context_release(ctx_ref: *mut kref);
}
extern "C" {
    pub fn rcu_dereference_protected(_arg: ctx->vm, _arg: lockdep_is_held(&ctx->mutex)) -> return;
}
extern "C" {
    pub fn i915_gem_context_engines(_arg: ctx) -> return;
}

extern "C" {
    pub fn i915_gem_context_module_exit();
}
extern "C" {
    pub fn i915_gem_context_module_init() -> c_int;
}
extern "C" {
    pub fn i915_lut_handle_free(lut: *mut i915_lut_handle);
}
