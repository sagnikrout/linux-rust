//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-fence-array.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// fence-array: aggregates fence to be waited together
//
// Copyright (C) 2016 Collabora Ltd
// Copyright (C) 2016 Advanced Micro Devices, Inc.
// Authors:
// Gustavo Padovan <gustavo@padovan.org>
// Christian König <christian.koenig@amd.com>
//

//
// struct dma_fence_array_cb - callback helper for fence array
// @cb: fence callback structure for signaling
// @array: reference to the parent fence array object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_fence_array_cb {
    pub cb: dma_fence_cb,
    pub array: *mut dma_fence_array,
}

//
// struct dma_fence_array - fence to represent an array of fences
// @base: fence base class
// @num_fences: number of fences in the array
// @num_pending: fences in the array still pending
// @fences: array of the fences
// @work: internal irq_work function
// @callbacks: array of callback helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_fence_array {
    pub base: dma_fence,
    pub num_fences: unsigned,
    pub num_pending: core::sync::atomic::AtomicI32,
    pub fences: *mut dma_fence,
    pub work: irq_work,
    pub __counted_by(num_fences): dma_fence_array_cb callbacks[],
}

//
// to_dma_fence_array - cast a fence to a dma_fence_array
// @fence: fence to cast to a dma_fence_array
//
// Returns NULL if the fence is not a dma_fence_array,
// or the dma_fence_array otherwise.
//
extern "C" {
    pub fn container_of(_arg: fence, dma_fence_array: struct, _arg: base) -> return;
}
//
// dma_fence_array_for_each - iterate over all fences in array
// @fence: current fence
// @index: index into the array
// @head: potential dma_fence_array object
//
// Test if @array is a dma_fence_array object and if yes iterate over all fences
// in the array. If not just iterate over the fence in @array itself.
//
// For a deep dive iterator see dma_fence_unwrap_for_each().
//

extern "C" {
    pub fn dma_fence_match_context(fence: *mut dma_fence, context: u64) -> bool;
}
