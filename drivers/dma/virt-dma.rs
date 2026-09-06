//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/virt-dma.h
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
// Virtual DMA channel support for DMAengine
//
// Copyright (C) 2012 Russell King
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virt_dma_desc {
    pub tx: dma_async_tx_descriptor,
    pub tx_result: dmaengine_result,
// protected by vc.lock
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virt_dma_chan {
    pub chan: dma_chan,
    pub task: tasklet_struct,
    pub ): *mut *mut void (desc_free)(struct virt_dma_desc,
    pub lock: spinlock_t,
// protected by vc.lock
    pub desc_allocated: list_head,
    pub desc_submitted: list_head,
    pub desc_issued: list_head,
    pub desc_completed: list_head,
    pub desc_terminated: list_head,
    pub cyclic: *mut virt_dma_desc,
}

extern "C" {
    pub fn container_of(_arg: chan, virt_dma_chan: struct, _arg: chan) -> return;
}
extern "C" {
    pub fn vchan_dma_desc_free_list(vc: *mut virt_dma_chan, head: *mut list_head);
}
extern "C" {
    pub fn vchan_init(vc: *mut virt_dma_chan, dmadev: *mut dma_device);
}
extern "C" {
    pub fn vchan_tx_submit(: *mut dma_async_tx_descriptor) -> dma_cookie_t;
}
extern "C" {
    pub fn vchan_tx_desc_free(: *mut dma_async_tx_descriptor) -> c_int;
}
//
// vchan_tx_prep - prepare a descriptor
// @vc: virtual channel allocating this descriptor
// @vd: virtual descriptor to prepare
// @tx_flags: flags argument passed in to prepare function
//
// vchan_issue_pending - move submitted descriptors to issued list
// @vc: virtual channel to update
//
// vc.lock must be held by caller
//
// vchan_cookie_complete - report completion of a descriptor
// @vd: virtual descriptor to update
//
// vc.lock must be held by caller
//
// vchan_vdesc_fini - Free or reuse a descriptor
// @vd: virtual descriptor to free/reuse
//
// vchan_cyclic_callback - report the completion of a period
// @vd: virtual descriptor
//
// vchan_terminate_vdesc - Disable pending cyclic callback
// @vd: virtual descriptor to be terminated
//
// vc.lock must be held by caller
//
// vchan_next_desc - peek at the next descriptor to be processed
// @vc: virtual channel to obtain descriptor from
//
// vc.lock must be held by caller
//
// vchan_get_all_descriptors - obtain all submitted and issued descriptors
// @vc: virtual channel to get descriptors from
// @head: list of descriptors found
//
// vc.lock must be held by caller
//
// Removes all submitted and issued descriptors from internal lists, and
// provides a list of all descriptors found
//
// vchan_synchronize() - synchronize callback execution to the current context
// @vc: virtual channel to synchronize
//
// Makes sure that all scheduled or active callbacks have finished running. For
// proper operation the caller has to ensure that no new callbacks are scheduled
// after the invocation of this function started.
// Free up the terminated cyclic descriptor to prevent memory leakage.
//
