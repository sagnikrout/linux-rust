//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtmutex.h
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
// RT Mutexes: blocking mutual exclusion locks with PI support
//
// started by Ingo Molnar and Thomas Gleixner:
//
// Copyright (C) 2004-2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
// Copyright (C) 2006, Timesys Corp., Thomas Gleixner <tglx@timesys.com>
//
// This file contains the public data structure and API definitions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_mutex_base {
    pub wait_lock: raw_spinlock_t,
    pub __guarded_by(&wait_lock): rb_root_cached waiters,
    pub __guarded_by(&wait_lock): *mut *mut task_owner,
}

//
// rt_mutex_base_is_locked - is the rtmutex locked
// @lock: the mutex to be queried
//
// Returns true if the mutex is locked, false if unlocked.
//
extern "C" {
    pub fn data_race(NULL: READ_ONCE(lock->owner) !=) -> return;
}

extern "C" {
    pub fn rt_mutex_base_init(rtb: *mut rt_mutex_base);
}
//
// The rt_mutex structure
//
// @wait_lock:	spinlock to protect the structure
// @waiters:	rbtree root to enqueue waiters in priority order;
// caches top-waiter (leftmost node).
// @owner:	the mutex owner
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_mutex {
    pub rtmutex: rt_mutex_base,

    pub dep_map: lockdep_map,

}

extern "C" {
    pub fn rt_mutex_debug_task_free(tsk: *mut task_struct);
}

// Macro flag: #define __DEP_MAP_RT_MUTEX_INITIALIZER(mutexname)

extern "C" {
    pub fn __rt_mutex_init(lock: *mut rt_mutex, name: *const c_char, key: *mut lock_class_key);
}

extern "C" {
    pub fn rt_mutex_lock(__acquires(lock: *mut *mut rt_mutex lock));
}

