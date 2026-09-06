//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/mutex.h
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
// Mutexes: blocking mutual exclusion locks
//
// started by Ingo Molnar:
//
// Copyright (C) 2004, 2005, 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//

//
// This is the control structure for tasks blocked on mutex, which resides
// on the blocked task's kernel stack:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mutex_waiter {
    pub list: list_head,
    pub task: *mut task_struct,
    pub ww_ctx: *mut ww_acquire_ctx,

    pub magic: *mut c_void,

}

//
// @owner: contains: 'struct task_struct *' to the current lock owner,
// NULL means not owned. Since task_struct pointers are aligned at
// at least L1_CACHE_BYTES, we have low bits to store extra state.
//
// Bit0 indicates a non-empty waiter list; unlock must issue a wakeup.
// Bit1 indicates unlock needs to hand the lock to the top-waiter
// Bit2 indicates handoff has been done and we're waiting for pickup.
//
pub const MUTEX_FLAG_WAITERS: c_uint = 0x01;
pub const MUTEX_FLAG_HANDOFF: c_uint = 0x02;
pub const MUTEX_FLAG_PICKUP: c_uint = 0x04;
pub const MUTEX_FLAGS: c_uint = 0x07;
//
// Internal helper function; C doesn't allow us to hide it :
//
// DO NOT USE (outside of mutex & scheduler code).
//
extern "C" {
    pub fn __get_task_blocked_on(_arg: p) -> return;
}

extern "C" {
    pub fn debug_mutex_free_waiter(waiter: *mut mutex_waiter);
}
extern "C" {
    pub fn debug_mutex_unlock(lock: *mut mutex);
}
extern "C" {
    pub fn debug_mutex_init(lock: *mut mutex);
}

