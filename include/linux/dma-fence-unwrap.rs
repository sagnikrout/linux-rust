//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-fence-unwrap.h
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
// Copyright (C) 2022 Advanced Micro Devices, Inc.
// Authors:
// Christian König <christian.koenig@amd.com>
//

//
// struct dma_fence_unwrap - cursor into the container structure
//
// Should be used with dma_fence_unwrap_for_each() iterator macro.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_fence_unwrap {
//
// @chain: potential dma_fence_chain, but can be other fence as well
//
    pub chain: *mut dma_fence,
//
// @array: potential dma_fence_array, but can be other fence as well
//
    pub array: *mut dma_fence,
//
// @index: last returned index if @array is really a dma_fence_array
//
    pub index: c_uint,
}

//
// dma_fence_unwrap_for_each - iterate over all fences in containers
// @fence: current fence
// @cursor: current position inside the containers
// @head: starting point for the iterator
//
// Unwrap dma_fence_chain and dma_fence_array containers and deep dive into all
// potential fences in them. If @head is just a normal fence only that one is
// returned.
//

extern "C" {
    pub fn dma_fence_dedup_array(array: *mut dma_fence, num_fences: usize) -> usize;
}
//
// dma_fence_unwrap_merge - unwrap and merge fences
//
// All fences given as parameters are unwrapped and merged back together as flat
// dma_fence_array. Useful if multiple containers need to be merged together.
//
// Implemented as a macro to allocate the necessary arrays on the stack and
// account the stack frame size to the caller.
//
// Returns NULL on memory allocation failure, a dma_fence object representing
// all the given fences otherwise.
//

