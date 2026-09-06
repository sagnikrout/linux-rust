//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mutex_types.h
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
// Simple, straightforward mutexes with strict semantics:
//
// - only one task can hold the mutex at a time
// - only the owner can unlock the mutex
// - multiple unlocks are not permitted
// - recursive locking is not permitted
// - a mutex object must be initialized via the API
// - a mutex object must not be initialized via memset or copying
// - task may not exit with mutex held
// - memory areas where held locks reside must not be freed
// - held mutexes must not be reinitialized
// - mutexes may not be used in hardware or software interrupt
// contexts such as tasklets and timers
//
// These semantics are fully enforced when DEBUG_MUTEXES is
// enabled. Furthermore, besides enforcing the above rules, the mutex
// debugging code also implements a number of additional features
// that make lock debugging easier and faster:
//
// - uses symbolic names of mutexes, whenever they are printed in debug output
// - point-of-acquire tracking, symbolic lookup of function names
// - list of all locks held in the system, printout of them
// - owner tracking
// - detects self-recursing locks and prints out all relevant info
// - detects multi-task circular deadlocks and prints out all affected
// locks and tasks (and only those tasks)
//

extern "C" {
    pub fn __guarded_by(_arg: &wait_lock) -> *mut mutex_waiter	first_waiter;
}

//
// Preempt-RT variant based on rtmutexes.
//

