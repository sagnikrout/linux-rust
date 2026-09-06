//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_suballoc.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2011 Red Hat Inc.
// Copyright © 2022 Intel Corporation
//

pub const DRM_SUBALLOC_MAX_QUEUES: c_int = 32;
//
// struct drm_suballoc_manager - fenced range allocations
// @wq: Wait queue for sleeping allocations on contention.
// @hole: Pointer to first hole node.
// @olist: List of allocated ranges.
// @flist: Array[fence context hash] of queues of fenced allocated ranges.
// @size: Size of the managed range.
// @align: Default alignment for the managed range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_suballoc_manager {
    pub wq: wait_queue_head_t,
    pub hole: *mut list_head,
    pub olist: list_head,
    pub flist: [list_head; DRM_SUBALLOC_MAX_QUEUES],
    pub size: usize,
    pub align: usize,
}

//
// struct drm_suballoc - Sub-allocated range
// @olist: List link for list of allocated ranges.
// @flist: List linkk for the manager fenced allocated ranges queues.
// @manager: The drm_suballoc_manager.
// @soffset: Start offset.
// @eoffset: End offset + 1 so that @eoffset - @soffset = size.
// @fence: The fence protecting the allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_suballoc {
    pub olist: list_head,
    pub flist: list_head,
    pub manager: *mut drm_suballoc_manager,
    pub soffset: usize,
    pub eoffset: usize,
    pub fence: *mut dma_fence,
}

extern "C" {
    pub fn drm_suballoc_manager_fini(sa_manager: *mut drm_suballoc_manager);
}
extern "C" {
    pub fn drm_suballoc_free(sa: *mut drm_suballoc, fence: *mut dma_fence);
}
//
// drm_suballoc_soffset - Range start.
// @sa: The struct drm_suballoc.
//
// Return: The start of the allocated range.
//
// drm_suballoc_eoffset - Range end.
// @sa: The struct drm_suballoc.
//
// Return: The end of the allocated range + 1.
//
// drm_suballoc_size - Range size.
// @sa: The struct drm_suballoc.
//
// Return: The size of the allocated range.
//

