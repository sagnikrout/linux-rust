//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stacktrace.h
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
// typedef stack_trace_consume_fn - Callback for arch_stack_walk()
// @cookie:	Caller supplied pointer handed back by arch_stack_walk()
// @addr:	The stack entry address to consume
//
// Return:	True, if the entry was consumed or skipped
// False, if there is no space left to store
//
extern "C" {
    pub fn bool(cookie: *mut *mut stack_trace_consume_fn)(void, addr: c_ulong) -> typedef;
}
//
// arch_stack_walk - Architecture specific function to walk the stack
// @consume_entry:	Callback which is invoked by the architecture code for
// each entry.
// @cookie:		Caller supplied pointer which is handed back to
// @consume_entry
// @task:		Pointer to a task struct, can be NULL
// @regs:		Pointer to registers, can be NULL
//
// ============ ======= ============================================
// task	        regs
// ============ ======= ============================================
// task		NULL	Stack trace from task (can be current)
// current	regs	Stack trace starting on regs->stackpointer
// ============ ======= ============================================
//
// arch_stack_walk_reliable - Architecture specific function to walk the
// stack reliably
//
// @consume_entry:	Callback which is invoked by the architecture code for
// each entry.
// @cookie:		Caller supplied pointer which is handed back to
// @consume_entry
// @task:		Pointer to a task struct, can be NULL
//
// Returns: a negative error code if it detects any unreliable
// features of the stack. Otherwise it guarantees that the stack
// trace is reliable and returns %0.
//
// If the task is not 'current', the caller *must* ensure the task is
// inactive and its stack is pinned.
//

extern "C" {
    pub fn stack_trace_save_user(store: *mut c_ulong, size: c_uint) -> c_uint;
}
extern "C" {
    pub fn filter_irq_stacks(entries: *mut c_ulong, nr_entries: c_uint) -> c_uint;
}

// Internal interfaces. Do not use in generic code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_trace {
    pub max_entries: unsigned int nr_entries,,
    pub entries: *mut c_ulong,
    pub /: *mut *mut unsigned int skip; / input argument: How many entries to skip,
}

extern "C" {
    pub fn save_stack_trace(trace: *mut stack_trace);
}
extern "C" {
    pub fn save_stack_trace_user(trace: *mut stack_trace);
}

