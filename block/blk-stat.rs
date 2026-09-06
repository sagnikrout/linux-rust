//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-stat.h
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

//
// struct blk_stat_callback - Block statistics callback.
//
// A &struct blk_stat_callback is associated with a &struct request_queue. While
// @timer is active, that queue's request completion latencies are sorted into
// buckets by @bucket_fn and added to a per-cpu buffer, @cpu_stat. When the
// timer fires, @cpu_stat is flushed to @stat and @timer_fn is invoked.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_stat_callback {
//
// @list: RCU list of callbacks for a &struct request_queue.
//
    pub list: list_head,
//
// @timer: Timer for the next callback invocation.
//
    pub timer: timer_list,
//
// @cpu_stat: Per-cpu statistics buckets.
//
    pub cpu_stat: *mut blk_rq_stat __percpu,
//
// @bucket_fn: Given a request, returns which statistics bucket it
// should be accounted under. Return -1 for no bucket for this
// request.
//
    pub ): *const *const int (bucket_fn)(struct request,
//
// @buckets: Number of statistics buckets.
//
    pub buckets: c_uint,
//
// @stat: Array of statistics buckets.
//
    pub stat: *mut blk_rq_stat,
//
// @timer_fn: Callback function.
//
    pub ): *mut *mut void (timer_fn)(struct blk_stat_callback,
//
// @data: Private pointer for the user.
//
    pub data: *mut c_void,
//
// @rcu: rcu list head
//
    pub rcu: rcu_head,
}

extern "C" {
    pub fn blk_free_queue_stats(: *mut blk_queue_stats);
}
extern "C" {
    pub fn blk_stat_add(rq: *mut request, now: u64);
}
// record time/size info in request but not add a callback
extern "C" {
    pub fn blk_stat_enable_accounting(q: *mut request_queue);
}
extern "C" {
    pub fn blk_stat_disable_accounting(q: *mut request_queue);
}
//
// blk_stat_alloc_callback() - Allocate a block statistics callback.
// @timer_fn: Timer callback function.
// @bucket_fn: Bucket callback function.
// @buckets: Number of statistics buckets.
// @data: Value for the @data field of the &struct blk_stat_callback.
//
// See &struct blk_stat_callback for details on the callback functions.
//
// Return: &struct blk_stat_callback on success or NULL on ENOMEM.
//
// blk_stat_add_callback() - Add a block statistics callback to be run on a
// request queue.
// @q: The request queue.
// @cb: The callback.
//
// Note that a single &struct blk_stat_callback can only be added to a single
// &struct request_queue.
//
// blk_stat_remove_callback() - Remove a block statistics callback from a
// request queue.
// @q: The request queue.
// @cb: The callback.
//
// When this returns, the callback is not running on any CPUs and will not be
// called again unless readded.
//
// blk_stat_free_callback() - Free a block statistics callback.
// @cb: The callback.
//
// @cb may be NULL, in which case this does nothing. If it is not NULL, @cb must
// not be associated with a request queue. I.e., if it was previously added with
// blk_stat_add_callback(), it must also have been removed since then with
// blk_stat_remove_callback().
//
extern "C" {
    pub fn blk_stat_free_callback(cb: *mut blk_stat_callback);
}
//
// blk_stat_is_active() - Check if a block statistics callback is currently
// gathering statistics.
// @cb: The callback.
//
// Returns: %true iff the callback is active.
//
extern "C" {
    pub fn timer_pending(_arg: &cb->timer) -> return;
}
//
// blk_stat_activate_nsecs() - Gather block statistics during a time window in
// nanoseconds.
// @cb: The callback.
// @nsecs: Number of nanoseconds to gather statistics for.
//
// The timer callback will be called when the window expires.
//
// blk_stat_activate_msecs() - Gather block statistics during a time window in
// milliseconds.
// @cb: The callback.
// @msecs: Number of milliseconds to gather statistics for.
//
// The timer callback will be called when the window expires.
//
extern "C" {
    pub fn blk_rq_stat_add(: *mut blk_rq_stat, _arg: u64);
}
extern "C" {
    pub fn blk_rq_stat_sum(: *mut blk_rq_stat, : *mut blk_rq_stat);
}
extern "C" {
    pub fn blk_rq_stat_init(: *mut blk_rq_stat);
}
