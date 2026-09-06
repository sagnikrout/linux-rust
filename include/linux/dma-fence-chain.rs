//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-fence-chain.h
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
// fence-chain: chain fences together in a timeline
//
// Copyright (C) 2018 Advanced Micro Devices, Inc.
// Authors:
// Christian König <christian.koenig@amd.com>
//

//
// struct dma_fence_chain - fence to represent an node of a fence chain
// @base: fence base class
// @prev: previous fence of the chain
// @prev_seqno: original previous seqno before garbage collection
// @fence: encapsulated fence
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_fence_chain {
    pub base: dma_fence,
    pub prev: *mut dma_fence __rcu,
    pub prev_seqno: u64,
    pub fence: *mut dma_fence,
//
// @cb: callback for signaling
//
// This is used to add the callback for signaling the
// complection of the fence chain. Never used at the same time
// as the irq work.
//
    pub cb: dma_fence_cb,
//
// @work: irq work item for signaling
//
// Irq work structure to allow us to add the callback without
// running into lock inversion. Never used at the same time as
// the callback.
//
    pub work: irq_work,
}

//
// to_dma_fence_chain - cast a fence to a dma_fence_chain
// @fence: fence to cast to a dma_fence_array
//
// Returns NULL if the fence is not a dma_fence_chain,
// or the dma_fence_chain otherwise.
//
extern "C" {
    pub fn container_of(_arg: fence, dma_fence_chain: struct, _arg: base) -> return;
}
//
// dma_fence_chain_contained - return the contained fence
// @fence: the fence to test
//
// If the fence is a dma_fence_chain the function returns the fence contained
// inside the chain object, otherwise it returns the fence itself.
//
// dma_fence_chain_alloc - Returns a new &struct dma_fence_chain object or
// %NULL on failure.
//
// This specialized allocator has to be a macro for its allocations to be
// accounted separately (to have a separate alloc_tag). The typecast is
// intentional to enforce typesafety.
//

//
// dma_fence_chain_free - Frees an allocated but not used
// &struct dma_fence_chain object.
// @chain: chain node to free
//
// Frees up an allocated but not used struct dma_fence_chain object. This
// doesn't need an RCU grace period since the fence was never initialized nor
// published. After dma_fence_chain_init() has been called the fence must be
// released by calling dma_fence_put(), and not through this function.
//
// dma_fence_chain_for_each - iterate over all fences in chain
// @iter: current fence
// @head: starting point
//
// Iterate over all fences in the chain. We keep a reference to the current
// fence while inside the loop which must be dropped when breaking out.
//
// For a deep dive iterator see dma_fence_unwrap_for_each().
//

extern "C" {
    pub fn dma_fence_chain_find_seqno(pfence: *mut dma_fence, seqno: u64) -> c_int;
}
