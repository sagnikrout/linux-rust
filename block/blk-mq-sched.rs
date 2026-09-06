//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-mq-sched.h
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


// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn blk_mq_sched_mark_restart_hctx(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn __blk_mq_sched_restart(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_sched_dispatch_requests(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_exit_sched(q: *mut request_queue, e: *mut elevator_queue);
}
extern "C" {
    pub fn blk_mq_sched_free_rqs(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_free_sched_ctx_batch(elv_tbl: *mut xarray);
}
//
// blk_mq_alloc_sched_data() - Allocates scheduler specific data
// Returns:
// - Pointer to allocated data on success
// - NULL if no allocation needed
// - ERR_PTR(-ENOMEM) in case of failure
//
extern "C" {
    pub fn test_bit(_arg: BLK_MQ_S_SCHED_RESTART, _arg: &hctx->state) -> return;
}
extern "C" {
    pub fn op_is_sync(!op_is_write(opf: opf) &&) -> return;
}
