//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_hw_fence_types.h
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
// struct xe_hw_fence_irq - hardware fence IRQ handler
//
// One per engine class, signals completed xe_hw_fences, triggered via hw engine
// interrupt. On each trigger, search list of pending fences and signal.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_hw_fence_irq {
// @lock: protects all xe_hw_fences + pending list
    pub lock: spinlock_t,
// @work: IRQ worker run to signal the fences
    pub work: irq_work,
// @pending: list of pending xe_hw_fences
    pub pending: list_head,
// @enabled: fence signaling enabled
    pub enabled: bool,
}

pub const MAX_FENCE_NAME_LEN: c_int = 16;
//
// struct xe_hw_fence_ctx - hardware fence context
//
// The context for a hardware fence. 1 to 1 relationship with xe_engine. Points
// to a xe_hw_fence_irq, maintains serial seqno.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_hw_fence_ctx {
// @gt: GT structure of hardware fence context
    pub gt: *mut xe_gt,
// @irq: fence irq handler
    pub irq: *mut xe_hw_fence_irq,
// @dma_fence_ctx: dma fence context for hardware fence
    pub dma_fence_ctx: u64,
// @next_seqno: next seqno for hardware fence
    pub next_seqno: u32,
// @name: name of hardware fence context
    pub name: [c_char; MAX_FENCE_NAME_LEN],
}

//
// struct xe_hw_fence - hardware fence
//
// Used to indicate a xe_sched_job is complete via a seqno written to memory.
// Signals on error or seqno past.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_hw_fence {
// @dma: base dma fence for hardware fence context
    pub dma: dma_fence,
// @xe: Xe device for hw fence driver name
    pub xe: *mut xe_device,
// @name: name of hardware fence context
    pub name: [c_char; MAX_FENCE_NAME_LEN],
// @seqno_map: I/O map for seqno
    pub seqno_map: iosys_map,
// @irq_link: Link in struct xe_hw_fence_irq.pending
    pub irq_link: list_head,
}
