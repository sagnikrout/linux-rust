//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_lrc_types.h
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
// struct xe_lrc - Logical ring context (LRC) and submission ring object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_lrc {
//
// @bo: buffer object (memory) for logical ring context, per process HW
// status page, and submission ring.
//
    pub bo: *mut xe_bo,
//
// @seqno_bo: Buffer object (memory) for seqno numbers. Always in system
// memory as this a CPU read, GPU write path object.
//
    pub seqno_bo: *mut xe_bo,
// @size: size of the lrc and optional indirect ring state
    pub size: u32,
// @replay_size: Size LRC needed for replaying a hang
    pub replay_size: u32,
// @gt: gt which this LRC belongs to
    pub gt: *mut xe_gt,
// @flags: LRC flags
pub const XE_LRC_FLAG_INDIRECT_CTX: c_uint = 0x1;
pub const XE_LRC_FLAG_INDIRECT_RING_STATE: c_uint = 0x2;
    pub flags: u32,
// @refcount: ref count of this lrc
    pub refcount: kref,
// @ring: submission ring state
// @ring.size: size of submission ring
    pub size: u32,
// @ring.tail: tail of submission ring
    pub tail: u32,
// @ring.old_tail: shadow of tail
    pub old_tail: u32,
    pub ring: },
// @desc: LRC descriptor
    pub desc: u64,
// @fence_ctx: context for hw fence
    pub fence_ctx: xe_hw_fence_ctx,
// @ctx_timestamp: readout value of CTX_TIMESTAMP on last update
    pub ctx_timestamp: u64,
// @queue_timestamp: value of QUEUE_TIMESTAMP on last update
    pub queue_timestamp: u64,
// @multi_queue: Multi queue LRC related information
// @multi_queue.primary_lrc: Primary lrc of this multi-queue group
    pub primary_lrc: *mut xe_lrc,
// @multi_queue.pos: Position of LRC within the multi-queue group
    pub pos: u8,
    pub multi_queue: },
}
