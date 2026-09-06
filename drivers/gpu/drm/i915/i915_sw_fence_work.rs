//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_sw_fence_work.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_fence_work_ops {
    pub name: *const c_char,
    pub f): *mut *mut void (work)(struct dma_fence_work,
    pub f): *mut *mut void (release)(struct dma_fence_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_fence_work {
    pub dma: dma_fence,
    pub lock: spinlock_t,
    pub chain: i915_sw_fence,
    pub cb: i915_sw_dma_fence_cb,
    pub work: work_struct,
    pub ops: *const dma_fence_work_ops,
}

extern "C" {
    pub fn dma_fence_work_chain(f: *mut dma_fence_work, signal: *mut dma_fence) -> c_int;
}
//
// dma_fence_work_commit_imm: Commit the fence, and if possible execute locally.
// @f: the fenced worker
//
// Instead of always scheduling a worker to execute the callback (see
// dma_fence_work_commit()), we try to execute the callback immediately in
// the local context. It is required that the fence be committed before it
// is published, and that no other threads try to tamper with the number
// of asynchronous waits on the fence (or else the callback will be
// executed in the wrong context, i.e. not the callers).
//
