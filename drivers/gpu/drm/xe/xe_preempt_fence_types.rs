//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_preempt_fence_types.h
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
// Copyright © 2022 Intel Corporation
//

//
// struct xe_preempt_fence - Xe preempt fence
//
// hardware and triggers a callback once the xe_engine is complete.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_preempt_fence {
// @base: dma fence base
    pub base: dma_fence,
// @link: link into list of pending preempt fences
    pub link: list_head,
// @q: exec queue for this preempt fence
    pub q: *mut xe_exec_queue,
// @preempt_work: work struct which issues preemption
    pub preempt_work: work_struct,
// @lock: dma-fence fence lock
    pub lock: spinlock_t,
// @error: preempt fence is in error state
    pub error: c_int,
}
