//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/run-in-irq-context.h
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
// Helper function for testing code in interrupt contexts
//
// Copyright 2025 Google LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_irq_test_state {
    pub test_specific_state): *mut *mut bool (func)(void,
    pub test_specific_state: *mut c_void,
    pub task_func_reported_failure: bool,
    pub hardirq_func_reported_failure: bool,
    pub softirq_func_reported_failure: bool,
    pub task_func_calls: core::sync::atomic::AtomicI32,
    pub hardirq_func_calls: core::sync::atomic::AtomicI32,
    pub softirq_func_calls: core::sync::atomic::AtomicI32,
    pub interval: ktime_t,
    pub timer: hrtimer,
    pub bh_work: work_struct,
}

//
// If the hrtimer is running much faster than the bh_work or the task,
// then it is firing too fast and might be starving those contexts as
// well as the actual system timer tick.  Increase the interval.
//
// Helper function which repeatedly runs the given @func in task, softirq, and
// hardirq context concurrently, and reports a failure to KUnit if any
// invocation of @func in any context returns false.  @func is passed
// @test_specific_state as its argument.  At most 3 invocations of @func will
// run concurrently: one in each of task, softirq, and hardirq context.  @func
// will continue running until either @max_iterations calls have been made (so
// long as at least one each runs in task, softirq, and hardirq contexts), or
// one second has passed.
//
// The main purpose of this interrupt context testing is to validate fallback
// code paths that run in contexts where the normal code path cannot be used,
// typically due to the FPU or vector registers already being in-use in kernel
// mode.  These code paths aren't covered when the test code is executed only by
// the KUnit test runner thread in task context.  The reason for the concurrency
// is because merely using hardirq context is not sufficient to reach a fallback
// code path on some architectures; the hardirq actually has to occur while the
// FPU or vector unit was already in-use in kernel mode.
//
// Another purpose of this testing is to detect issues with the architecture's
// irq_fpu_usable() and kernel_fpu_begin/end() or equivalent functions,
// especially in softirq context when the softirq may have interrupted a task
// already using kernel-mode FPU or vector (if the arch didn't prevent that).
// Crypto functions are often executed in softirqs, so this is important.
//
// Start with a 5us timer interval.  If the system can't keep
// up, kunit_irq_test_timer_func() will increase it.
//
// Set up a hrtimer (the way we access hardirq context) and a work
// struct for the BH workqueue (the way we access softirq context).
//
// Run for up to max_iterations (including at least one task, softirq,
// and hardirq), or 1 second, whichever comes first.
//
// Cancel the timer and work.
// Sanity check: the timer and BH functions should have been run.
// Check for failure reported from any context.
