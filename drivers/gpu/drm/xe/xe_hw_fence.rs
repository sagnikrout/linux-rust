//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_hw_fence.h
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

// Cause an early wrap to catch wrapping errors

extern "C" {
    pub fn xe_hw_fence_module_init() -> c_int;
}
extern "C" {
    pub fn xe_hw_fence_module_exit();
}
extern "C" {
    pub fn xe_hw_fence_irq_init(irq: *mut xe_hw_fence_irq);
}
extern "C" {
    pub fn xe_hw_fence_irq_finish(irq: *mut xe_hw_fence_irq);
}
extern "C" {
    pub fn xe_hw_fence_irq_run(irq: *mut xe_hw_fence_irq);
}
extern "C" {
    pub fn xe_hw_fence_ctx_finish(ctx: *mut xe_hw_fence_ctx);
}
extern "C" {
    pub fn xe_hw_fence_free(fence: *mut dma_fence);
}
