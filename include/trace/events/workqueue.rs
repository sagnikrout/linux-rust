//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/workqueue.h
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
// workqueue_queue_work - called when a work gets queued
// @req_cpu:	the requested cpu
// @pwq:	pointer to struct pool_workqueue
// @work:	pointer to struct work_struct
//
// This event occurs when a work is queued immediately or once a
// delayed work is actually queued on a workqueue (ie: once the delay
// has been reached).
//
// workqueue_activate_work - called when a work gets activated
// @work:	pointer to struct work_struct
//
// This event occurs when a queued work is put on the active queue,
// which happens immediately after queueing unless @max_active limit
// is reached.
//
// workqueue_execute_start - called immediately before the workqueue callback
// @work:	pointer to struct work_struct
//
// Allows to track workqueue execution.
//
// workqueue_execute_end - called immediately after the workqueue callback
// @work:	pointer to struct work_struct
// @function:   pointer to worker function
//
// Allows to track workqueue execution.
//

// This part must be outside protection
