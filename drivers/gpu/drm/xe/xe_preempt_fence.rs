//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_preempt_fence.h
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

extern "C" {
    pub fn xe_preempt_fence_free(pfence: *mut xe_preempt_fence);
}
extern "C" {
    pub fn container_of(_arg: fence, xe_preempt_fence: struct, _arg: base) -> return;
}
//
// xe_preempt_fence_link() - Return a link used to keep unarmed preempt
// fences on a list.
// @pfence: Pointer to the preempt fence.
//
// The link is embedded in the struct xe_preempt_fence. Use
// link_to_preempt_fence() to convert back to the preempt fence.
//
// Return: A pointer to an embedded struct list_head.
//
// to_preempt_fence_from_link() - Convert back to a preempt fence pointer
// from a link obtained with xe_preempt_fence_link().
// @link: The struct list_head obtained from xe_preempt_fence_link().
//
// Return: A pointer to the embedding struct xe_preempt_fence.
//
extern "C" {
    pub fn container_of(_arg: link, xe_preempt_fence: struct, _arg: link) -> return;
}
extern "C" {
    pub fn xe_fence_is_xe_preempt(fence: *const dma_fence) -> bool;
}
