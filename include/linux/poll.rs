//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/poll.h
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

// ~832 bytes of stack space used max in sys_select/sys_poll before allocating
pub const MAX_STACK_ALLOC: c_int = 832;
pub const FRONTEND_STACK_ALLOC: c_int = 256;

//
// structures and helpers for f_op->poll implementations
//
extern "C" {
    pub fn void(: *mut *mut poll_queue_proc)(struct file, : *mut wait_queue_head_t, : *mut poll_table_struct) -> typedef;
}
//
// Do not touch the structure directly, use the access function
// poll_requested_events() instead.
//
// This memory barrier is paired in the wq_has_sleeper().
// See the comment above prepare_to_wait(), we need to
// ensure that subsequent tests in this thread can't be
// reordered with __add_wait_queue() in _qproc() paths.
//
// Return the set of events that the application wants to poll for.
// This is useful for drivers that need to know whether a DMA transfer has
// to be started implicitly on poll(). You typically only want to do that
// if the application is actually polling for POLLIN and/or POLLOUT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct poll_table_entry {
    pub filp: *mut file,
    pub key: __poll_t,
    pub wait: wait_queue_entry_t,
    pub wait_address: *mut wait_queue_head_t,
}

//
// Structures and helpers for select/poll syscall
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct poll_wqueues {
    pub pt: poll_table,
    pub table: *mut poll_table_page,
    pub polling_task: *mut task_struct,
    pub triggered: c_int,
    pub error: c_int,
    pub inline_index: c_int,
    pub inline_entries: [poll_table_entry; N_INLINE_POLL_ENTRIES],
}

extern "C" {
    pub fn poll_initwait(pwq: *mut poll_wqueues);
}
extern "C" {
    pub fn poll_freewait(pwq: *mut poll_wqueues);
}
extern "C" {
    pub fn select_estimate_accuracy(tv: *mut timespec64) -> u64;
}

