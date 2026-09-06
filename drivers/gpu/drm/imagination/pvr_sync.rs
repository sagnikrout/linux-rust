//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_sync.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declaration from <linux/xarray.h>.
// Forward declaration from <drm/drm_file.h>.
// Forward declaration from <drm/gpu_scheduler.h>.
// Forward declaration from "pvr_device.h".
//
// struct pvr_sync_signal - Object encoding a syncobj signal operation
//
// The job submission logic collects all signal operations in an array of
// pvr_sync_signal objects. This array also serves as a cache to get the
// latest dma_fence when multiple jobs are submitted at once, and one job
// signals a syncobj point that's later waited on by a subsequent job.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_sync_signal {
// @handle: Handle of the syncobj to signal.
    pub handle: u32,
//
// @point: Point to signal in the syncobj.
//
// Only relevant for timeline syncobjs.
//
    pub point: u64,
// @syncobj: Syncobj retrieved from the handle.
    pub syncobj: *mut drm_syncobj,
//
// @chain: Chain object used to link the new fence with the
// existing timeline syncobj.
//
// Should be zero when manipulating a regular syncobj.
//
    pub chain: *mut dma_fence_chain,
//
// @fence: New fence object to attach to the syncobj.
//
// This pointer starts with the current fence bound to
// the <handle,point> pair.
//
    pub fence: *mut dma_fence,
}
