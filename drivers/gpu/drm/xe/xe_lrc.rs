//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_lrc.h
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
// Copyright © 2021 Intel Corporation
//

//
// Sentinel value stored in lrc->ctx_timestamp while a context is starting.
// The hardware hasn't yet written the real CTX_TIMESTAMP, so this is not a
// valid elapsed-time sample and must not be used as one.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_lrc_snapshot {
    pub lrc_bo: *mut xe_bo,
    pub lrc_snapshot: *mut c_void,
    pub lrc_offset: unsigned long lrc_size,,
    pub replay_offset: unsigned long replay_size,,
    pub context_desc: u32,
    pub ring_addr: u32,
    pub indirect_context_desc: u32,
    pub head: u32,
    pub start: u32,
    pub internal: u32,
    pub memory: u32,
    pub tail: },
    pub start_seqno: u32,
    pub seqno: u32,
    pub ctx_timestamp: u64,
    pub ctx_timestamp_ms: u64,
    pub queue_timestamp: u64,
    pub queue_timestamp_ms: u64,
    pub ctx_job_timestamp: u32,
}

extern "C" {
    pub fn xe_lrc_destroy(ref: *mut kref);
}
//
// xe_lrc_get - Get reference to the LRC
// @lrc: Logical Ring Context
//
// Increment reference count of @lrc
//
// xe_lrc_put - Put reference of the LRC
// @lrc: Logical Ring Context
//
// Decrement reference count of @lrc, call xe_lrc_destroy when
// reference count reaches 0.
//
// xe_lrc_ring_size() - Xe LRC ring size
//
// Return: Size of LRC ring buffer
//
extern "C" {
    pub fn xe_gt_lrc_hang_replay_size(gt: *mut xe_gt, class: xe_engine_class) -> usize;
}
extern "C" {
    pub fn xe_gt_lrc_size(gt: *mut xe_gt, class: xe_engine_class) -> usize;
}
extern "C" {
    pub fn xe_lrc_pphwsp_offset(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_regs_offset(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_set_ring_tail(lrc: *mut xe_lrc, tail: u32);
}
extern "C" {
    pub fn xe_lrc_ring_tail(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_set_ring_head(lrc: *mut xe_lrc, head: u32);
}
extern "C" {
    pub fn xe_lrc_ring_head(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_ring_space(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_write_ring(lrc: *mut xe_lrc, data: *const c_void, size: usize);
}
extern "C" {
    pub fn xe_lrc_ring_is_idle(lrc: *mut xe_lrc) -> bool;
}
extern "C" {
    pub fn xe_lrc_indirect_ring_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_update_hwctx_regs_with_address(lrc: *mut xe_lrc);
}
extern "C" {
    pub fn xe_default_lrc_update_memirq_regs_with_address(hwe: *mut xe_hw_engine);
}
extern "C" {
    pub fn xe_lrc_read_ctx_reg(lrc: *mut xe_lrc, reg_nr: c_int) -> u32;
}
extern "C" {
    pub fn xe_lrc_write_ctx_reg(lrc: *mut xe_lrc, reg_nr: c_int, val: u32);
}
extern "C" {
    pub fn xe_lrc_descriptor(lrc: *mut xe_lrc) -> u64;
}
extern "C" {
    pub fn xe_lrc_seqno_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_free_seqno_fence(fence: *mut dma_fence);
}
extern "C" {
    pub fn xe_lrc_init_seqno_fence(lrc: *mut xe_lrc, fence: *mut dma_fence);
}
extern "C" {
    pub fn xe_lrc_seqno(lrc: *mut xe_lrc) -> i32;
}
extern "C" {
    pub fn xe_lrc_start_seqno_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_start_seqno(lrc: *mut xe_lrc) -> i32;
}
extern "C" {
    pub fn xe_lrc_parallel_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_parallel_map(lrc: *mut xe_lrc) -> iosys_map;
}
extern "C" {
    pub fn xe_lrc_reg_size(xe: *mut xe_device) -> usize;
}
extern "C" {
    pub fn xe_lrc_engine_state_size(gt: *mut xe_gt, class: xe_engine_class) -> usize;
}
extern "C" {
    pub fn xe_lrc_set_multi_queue_priority(lrc: *mut xe_lrc, priority: xe_multi_queue_priority);
}
extern "C" {
    pub fn xe_lrc_snapshot_capture_delayed(snapshot: *mut xe_lrc_snapshot);
}
extern "C" {
    pub fn xe_lrc_snapshot_print(snapshot: *mut xe_lrc_snapshot, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_lrc_snapshot_free(snapshot: *mut xe_lrc_snapshot);
}
extern "C" {
    pub fn xe_lrc_ctx_timestamp_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_ctx_timestamp_udw_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_ctx_job_timestamp_ggtt_addr(lrc: *mut xe_lrc) -> u32;
}
extern "C" {
    pub fn xe_lrc_ctx_job_timestamp(lrc: *mut xe_lrc) -> u32;
}
//
// xe_lrc_update_timestamp - readout LRC timestamp and update cached value
// @lrc: logical ring context for this exec queue
// @old_ts: pointer where to save the previous timestamp
//
// Read the current timestamp for this LRC and update the cached value. The
// previous cached value is also returned in @old_ts so the caller can calculate
// the delta between 2 updates. Note that this is not intended to be called from
// any place, but just by the paths updating the drm client utilization.
//
// Returns the current LRC timestamp
//
extern "C" {
    pub fn xe_lrc_update_timestamp(lrc: *mut xe_lrc, old_ts: *mut u64) -> u64;
}
extern "C" {
    pub fn xe_lrc_timestamp(lrc: *mut xe_lrc) -> u64;
}
