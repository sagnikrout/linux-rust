//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/trace_recursion.h
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

// Only current can touch trace_recursion
//
// For function tracing recursion:
// The order of these bits are important.
//
// When function tracing occurs, the following steps are made:
// If arch does not support a ftrace feature:
// call internal function (uses INTERNAL bits) which calls...
// The function callback, which can use the FTRACE bits to
// check for recursion.
//
// Function recursion bits
// Internal use recursion bits
// Internal event use recursion bits
//
// Abuse of the trace_recursion.
// As we need a way to maintain state if we are tracing the function
// graph in irq because we want to trace a particular function that
// was called in irq context but we have irq tracing off. Since this
// can only be modified by current, we can reuse trace_recursion.
//
// Used to prevent recursion recording from recursing.

pub const TRACE_CONTEXT_BITS: c_int = 4;

//
// Used for setting context
// NMI     = 0
// IRQ     = 1
// SOFTIRQ = 2
// NORMAL  = 3
//

extern "C" {
    pub fn ftrace_record_recursion(ip: c_ulong, parent_ip: c_ulong);
}

//
// Preemption is promised to be disabled when return bit >= 0.
//
// If an interrupt occurs during a trace, and another trace
// happens in that interrupt but before the preempt_count is
// updated to reflect the new interrupt context, then this
// will think a recursion occurred, and the event will be dropped.
// Let a single instance happen via the TRANSITION_BIT to
// not drop those events.
//
// Preemption will be enabled (if it was previously enabled).
//
// ftrace_test_recursion_trylock - tests for recursion in same context
//
// Use this for ftrace callbacks. This will detect if the function
// tracing recursed in the same context (normal vs interrupt),
//
// Returns: -1 if a recursion happened.
// >= 0 if no recursion.
//
extern "C" {
    pub fn trace_test_and_set_recursion(_arg: ip, _arg: parent_ip, _arg: TRACE_FTRACE_START) -> return;
}
//
// ftrace_test_recursion_unlock - called when function callback is complete
// @bit: The return of a successful ftrace_test_recursion_trylock()
//
// This is used at the end of a ftrace callback.
//

