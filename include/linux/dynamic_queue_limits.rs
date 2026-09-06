//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dynamic_queue_limits.h
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
// Dynamic queue limits (dql) - Definitions
//
// Copyright (c) 2011, Tom Herbert <therbert@google.com>
//
// This header file contains the definitions for dynamic queue limits (dql).
// dql would be used in conjunction with a producer/consumer type queue
// (possibly a HW queue).  Such a queue would have these general properties:
//
// 1) Objects are queued up to some limit specified as number of objects.
// 2) Periodically a completion process executes which retires consumed
// objects.
// 3) Starvation occurs when limit has been reached, all queued data has
// actually been consumed, but completion processing has not yet run
// so queuing new data is blocked.
// 4) Minimizing the amount of queued data is desirable.
//
// The goal of dql is to calculate the limit as the minimum number of objects
// needed to prevent starvation.
//
// The primary functions of dql are:
// dql_queued - called when objects are enqueued to record number of objects
// dql_avail - returns how many objects are available to be queued based
// on the object limit and how many objects are already enqueued
// dql_completed - called at completion time to indicate how many objects
// were retired from the queue
//
// The dql implementation does not implement any locking for the dql data
// structures, the higher layer should provide this.  dql_queued should
// be serialized to prevent concurrent execution of the function; this
// is also true for  dql_completed.  However, dql_queued and dlq_completed  can
// be executed concurrently (i.e. they can be protected by different locks).
//

pub const DQL_HIST_LEN: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dql {
// Fields accessed in enqueue path (dql_queued)
    pub /: *mut *mut unsigned int num_queued; / Total ever queued,
    pub /: *mut *mut unsigned int adj_limit; / limit + num_completed,
    pub /: *mut *mut unsigned int last_obj_cnt; / Count at last queuing,
// Stall threshold (in jiffies), defined by user
    pub stall_thrs: c_ushort,
    pub /: *mut *mut unsigned long history_head; / top 58 bits of jiffies,
// stall entries, a bit per entry
    pub history: [c_ulong; DQL_HIST_LEN],
// Fields accessed only by completion path (dql_completed)
    pub /: *mut *mut unsigned int limit ____cacheline_aligned_in_smp; / Current limit,
    pub /: *mut *mut unsigned int num_completed; / Total ever completed,
    pub /: *mut *mut unsigned int prev_ovlimit; / Previous over limit,
    pub /: *mut *mut unsigned int prev_num_queued; / Previous queue total,
    pub /: *mut *mut unsigned int prev_last_obj_cnt; / Previous queuing cnt,
    pub /: *mut *mut unsigned int lowest_slack; / Lowest slack found,
    pub /: *mut *mut unsigned long slack_start_time; / Time slacks seen,
// Configuration
    pub /: *mut *mut unsigned int max_limit; / Max limit,
    pub /: *mut *mut unsigned int min_limit; / Minimum limit,
    pub /: *mut *mut unsigned int slack_hold_time; / Time to measure slack,
// Longest stall detected, reported to user
    pub stall_max: c_ushort,
    pub /: *mut *mut unsigned long last_reap; / Last reap (in jiffies),
    pub /: *mut *mut unsigned long stall_cnt; / Number of stalls,
}

// Set some static maximums

// Populate the bitmap to be processed later in dql_check_stall()
// The following code set a bit in the ring buffer, where each
// bit trackes time the packet was queued. The dql->history buffer
// tracks DQL_HIST_LEN * BITS_PER_LONG time (jiffies) slot
//
// About to reuse slots, clear them
// Multiplication masks high bits
// pairs with smp_rmb() in dql_check_stall()
// __set_bit() does not guarantee WRITE_ONCE() semantics
// Populate the history with an entry (bit) per queued
//
// Record number of objects queued. Assumes that caller has already checked
// availability in the queue with dql_avail.
//
// We want to force a write first, so that cpu do not attempt
// to get cache line containing last_obj_cnt, num_queued, adj_limit
// in Shared state, but directly does a Request For Ownership
// It is only a hint, we use barrier() only.
//
// Only populate stall information if the threshold is set
// Returns how many objects can be queued, < 0 indicates over limit.
extern "C" {
    pub fn READ_ONCE(READ_ONCE(dql->num_queued: dql->adj_limit) -) -> return;
}
// Record number of completed objects and recalculate the limit.
extern "C" {
    pub fn dql_completed(dql: *mut dql, count: c_uint);
}
// Reset dql state
extern "C" {
    pub fn dql_reset(dql: *mut dql);
}
// Initialize dql state
extern "C" {
    pub fn dql_init(dql: *mut dql, hold_time: c_uint);
}

