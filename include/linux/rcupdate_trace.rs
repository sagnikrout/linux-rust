//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcupdate_trace.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Read-Copy Update mechanism for mutual exclusion, adapted for tracing.
//
// Copyright (C) 2020 Paul E. McKenney.
//

extern "C" {
    pub fn srcu_read_lock_held(_arg: &rcu_tasks_trace_srcu_struct) -> return;
}

//
// rcu_read_lock_tasks_trace - mark beginning of RCU-trace read-side critical section
//
// When synchronize_rcu_tasks_trace() is invoked by one task, then that
// task is guaranteed to block until all other tasks exit their read-side
// critical sections.  Similarly, if call_rcu_trace() is invoked on one
// task while other tasks are within RCU read-side critical sections,
// invocation of the corresponding RCU callback is deferred until after
// the all the other tasks exit their critical sections.
//
// For more details, please see the documentation for
// srcu_read_lock_fast().  For a description of how implicit RCU
// readers provide the needed ordering for architectures defining the
// ARCH_WANTS_NO_INSTR Kconfig option (and thus promising never to trace
// code where RCU is not watching), please see the __srcu_read_lock_fast()
// (non-kerneldoc) header comment.  Otherwise, the smp_mb() below provided
// the needed ordering.
//
// rcu_read_unlock_tasks_trace - mark end of RCU-trace read-side critical section
// @scp: return value from corresponding rcu_read_lock_tasks_trace().
//
// Pairs with the preceding call to rcu_read_lock_tasks_trace() that
// returned the value passed in via scp.
//
// For more details, please see the documentation for rcu_read_unlock().
// For memory-ordering information, please see the header comment for the
// rcu_read_lock_tasks_trace() function.
//
// rcu_read_lock_trace - mark beginning of RCU-trace read-side critical section
//
// When synchronize_rcu_tasks_trace() is invoked by one task, then that
// task is guaranteed to block until all other tasks exit their read-side
// critical sections.  Similarly, if call_rcu_trace() is invoked on one
// task while other tasks are within RCU read-side critical sections,
// invocation of the corresponding RCU callback is deferred until after
// the all the other tasks exit their critical sections.
//
// For more details, please see the documentation for rcu_read_lock().
//
// In case we interrupted a Tasks Trace RCU reader.
//
// rcu_read_unlock_trace - mark end of RCU-trace read-side critical section
//
// Pairs with a preceding call to rcu_read_lock_trace(), and nesting is
// allowed.  Invoking a rcu_read_unlock_trace() when there is no matching
// rcu_read_lock_trace() is verboten, and will result in lockdep complaints.
//
// For more details, please see the documentation for rcu_read_unlock().
//
// call_rcu_tasks_trace() - Queue a callback trace task-based grace period
// @rhp: structure to be used for queueing the RCU updates.
// @func: actual callback function to be invoked after the grace period
//
// The callback function will be invoked some time after a trace rcu-tasks
// grace period elapses, in other words after all currently executing
// trace rcu-tasks read-side critical sections have completed. These
// read-side critical sections are delimited by calls to rcu_read_lock_trace()
// and rcu_read_unlock_trace().
//
// See the description of call_rcu() for more detailed information on
// memory ordering guarantees.
//
// synchronize_rcu_tasks_trace - wait for a trace rcu-tasks grace period
//
// Control will return to the caller some time after a trace rcu-tasks
// grace period has elapsed, in other words after all currently executing
// trace rcu-tasks read-side critical sections have elapsed. These read-side
// critical sections are delimited by calls to rcu_read_lock_trace()
// and rcu_read_unlock_trace().
//
// This is a very specialized primitive, intended only for a few uses in
// tracing and other situations requiring manipulation of function preambles
// and profiling hooks.  The synchronize_rcu_tasks_trace() function is not
// (yet) intended for heavy use from multiple CPUs.
//
// See the description of synchronize_rcu() for more detailed information
// on memory ordering guarantees.
//
// rcu_barrier_tasks_trace - Wait for in-flight call_rcu_tasks_trace() callbacks.
//
// Note that rcu_barrier_tasks_trace() is not obligated to actually wait,
// for example, if there are no pending callbacks.
//
// rcu_tasks_trace_expedite_current - Expedite the current Tasks Trace RCU grace period
//
// Cause the current Tasks Trace RCU grace period to become expedited.
// The grace period following the current one might also be expedited.
// If there is no current grace period, one might be created.  If the
// current grace period is currently sleeping, that sleep will complete
// before expediting will take effect.
//
extern "C" {
    pub fn rcu_tasks_trace_batches_completed() -> c_ulong;
}
// Placeholders to enable stepwise transition.
extern "C" {
    pub fn rcu_tasks_trace_suppress_unused() -> void __init;
}

//
// The BPF JIT forms these addresses even when it doesn't call these
// functions, so provide definitions that result in runtime errors.
//

