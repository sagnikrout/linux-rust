//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/trace_gfs2.h
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

pub const TRACE_RS_DELETE: c_int = 0;
pub const TRACE_RS_TREEDEL: c_int = 1;
pub const TRACE_RS_INSERT: c_int = 2;
pub const TRACE_RS_CLAIM: c_int = 3;

// Macro flag: #define NUMPTY

// Section 1 - Locking
//
// Objectives:
// Latency: Remote demote request to state change
// Latency: Local lock request to state change
// Latency: State change to lock grant
// Correctness: Ordering of local lock state vs. I/O requests
// Correctness: Responses to remote demote requests
//
// General glock state change (DLM lock request completes)
// State change -> unlocked, glock is being deallocated
// Callback (local or remote) requesting lock demotion
// Promotion/grant of a glock
// Queue/dequeue a lock request
// DLM sends a reply to GFS2
// Section 2 - Log/journal
//
// Objectives:
// Latency: Log flush time
// Correctness: pin/unpin vs. disk I/O ordering
// Performance: Log usage stats
//
// Pin/unpin a block in the log
// Flushing the log
// Reserving/releasing blocks in the log
// Writing back the AIL
// Section 3 - bmap
//
// Objectives:
// Latency: Bmap request time
// Performance: Block allocator tracing
// Correctness: Test of disard generation vs. blocks allocated
//
// Map an extent of blocks, possibly a new allocation
// Keep track of blocks as they are allocated/freed
// Keep track of multi-block reservations as they are allocated/freed

// This part must be outside protection

