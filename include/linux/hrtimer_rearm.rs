//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hrtimer_rearm.h
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
    pub fn __hrtimer_rearm_deferred();
}
//
// This is purely CPU local, so check the TIF bit first to avoid the overhead of
// the atomic test_and_clear_bit() operation for the common case where the bit
// is not set.
//

// Invoked from the exit to user before invoking exit_to_user_mode_loop()
// Help the compiler to optimize the function out for syscall returns
//
// Rearm the timer if none of the resched flags is set before going into
// the loop which re-enables interrupts.
//
// Don't go into the loop if HRTIMER_REARM was the only flag
// tif_work &= ~TIF_HRTIMER_REARM;
// Invoked from the time slice extension decision function
//
// This is to be called on all irqentry_exit() paths that will enable
// interrupts.
//
// Invoked from the scheduler on entry to __schedule() so it can defer
// rearming after the load balancing callbacks which might change hrtick.
//
extern "C" {
    pub fn hrtimer_test_and_clear_rearm_deferred_tif(_arg: read_thread_flags()) -> return;
}

