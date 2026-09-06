//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mutex.h
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
// This file contains the main data structure and API definitions.
//

extern "C" {
    pub fn mutex_destroy(lock: *mut mutex);
}

//
// mutex_init - initialize the mutex
// @mutex: the mutex to be initialized
//
// Initialize the mutex to unlocked state.
//
// It is not allowed to initialize an already locked mutex.
//

//
// mutex_init_with_key - initialize a mutex with a given lockdep key
// @mutex: the mutex to be initialized
// @key: the lockdep key to be associated with the mutex
//
// Initialize the mutex to the unlocked state.
//
// It is not allowed to initialize an already locked mutex.
//

extern "C" {
    pub fn mutex_init_lockdep(lock: *mut mutex, name: *const c_char, key: *mut lock_class_key);
}

extern "C" {
    pub fn mutex_init_generic(lock: *mut mutex);
}

//
// mutex_is_locked - is the mutex locked
// @lock: the mutex to be queried
//
// Returns true if the mutex is locked, false if unlocked.
//
extern "C" {
    pub fn mutex_is_locked(lock: *mut mutex) -> bool;
}

//
// Preempt-RT variant based on rtmutexes.
//

extern "C" {
    pub fn mutex_rt_init_generic(mutex: *mut mutex);
}

extern "C" {
    pub fn __devm_mutex_init(dev: *mut device, lock: *mut mutex) -> int __must_check;
}

//
// When CONFIG_DEBUG_MUTEXES is off mutex_destroy() is just a nop so
// no really need to register it in the devm subsystem.
//

//
// See kernel/locking/mutex.c for detailed documentation of these APIs.
// Also see Documentation/locking/mutex-design.rst.
//

extern "C" {
    pub fn mutex_lock_nested(lock: *mut mutex, __acquires(lock: unsigned int subclass));
}
extern "C" {
    pub fn _mutex_lock_nest_lock(lock: *mut mutex, __acquires(lock: *mut *mut lockdep_map nest_lock));
}
extern "C" {
    pub fn mutex_lock_io_nested(lock: *mut mutex, __acquires(lock: unsigned int subclass));
}

extern "C" {
    pub fn mutex_lock(__acquires(lock: *mut *mut mutex lock));
}
extern "C" {
    pub fn mutex_lock_interruptible(__cond_acquires(0: *mut *mut mutex lock), _arg: lock) -> int __must_check;
}
extern "C" {
    pub fn mutex_lock_killable(__cond_acquires(0: *mut *mut mutex lock), _arg: lock) -> int __must_check;
}
extern "C" {
    pub fn mutex_lock_io(__acquires(lock: *mut *mut mutex lock));
}

//
// NOTE: mutex_trylock() follows the spin_trylock() convention,
// not the down_trylock() convention!
//
// Returns 1 if the mutex has been acquired successfully, and 0 on contention.
//

extern "C" {
    pub fn _mutex_trylock_nest_lock(lock: *mut mutex, __cond_acquires(true: *mut *mut lockdep_map nest_lock), _arg: lock) -> c_int;
}

extern "C" {
    pub fn mutex_trylock(__cond_acquires(true: *mut *mut mutex lock), _arg: lock) -> c_int;
}

extern "C" {
    pub fn mutex_unlock(__releases(lock: *mut *mut mutex lock));
}
extern "C" {
    pub fn atomic_dec_and_mutex_lock(cnt: *mut core::sync::atomic::AtomicI32, __cond_acquires(true: *mut *mut mutex lock), _arg: lock) -> c_int;
}

extern "C" {
    pub fn mutex_get_owner(lock: *mut mutex) -> c_ulong;
}
