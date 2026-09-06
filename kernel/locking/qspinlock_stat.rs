//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/qspinlock_stat.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Authors: Waiman Long <longman@redhat.com>
//

//
// Collect pvqspinlock locking event counts
//

//
// PV specific per-cpu counter
//
extern "C" {
    pub fn DEFINE_PER_CPU(_arg: u64, _arg: pv_kick_time) -> static;
}
//
// Function to read and return the PV qspinlock counts.
//
// The following counters are handled specially:
// 1. pv_latency_kick
// Average kick latency (ns) = pv_latency_kick/pv_kick_unlock
// 2. pv_latency_wake
// Average wake latency (ns) = pv_latency_wake/pv_kick_wake
// 3. pv_hash_hops
// Average hops/hash = pv_hash_hops/pv_kick_unlock
//
// Get the counter ID stored in file->f_inode->i_private
//
// Need to sum additional counters for some of them
//
// Return a X.XX decimal number
//
// Round to the nearest ns
//
extern "C" {
    pub fn simple_read_from_buffer(_arg: user_buf, _arg: count, _arg: ppos, _arg: buf, _arg: len) -> return;
}
//
// PV hash hop count
//
// Replacement function for pv_kick()
//
// Replacement function for pv_wait()
//
// pkick_time = 0;

