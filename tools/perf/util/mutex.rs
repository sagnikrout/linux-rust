//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/mutex.h
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
// A function-like feature checking macro that is a wrapper around
// `__has_attribute`, which is defined by GCC 5+ and Clang and evaluates to a
// nonzero constant integer if the attribute is supported or 0 if not.
//

pub const HAVE_ATTRIBUTE(x): c_int = 0;

// Documents if a shared field or global variable needs to be protected by a mutex.

//
// Documents if the memory location pointed to by a pointer should be guarded by
// a mutex when dereferencing the pointer.
//

// Documents if a type is a lockable type.

// Documents a function that expects a lock not to be held prior to entry.

// Documents a function that returns a lock.

// Documents functions that acquire a lock in the body of a function, and do not release it.

//
// Documents functions that acquire a shared (reader) lock in the body of a
// function, and do not release it.
//

//
// Documents functions that expect a lock to be held on entry to the function,
// and release it in the body of the function.
//

// Documents functions that try to acquire a lock, and return success or failure.

// Documents a function that expects a mutex to be held prior to entry.

// Documents a function that expects a shared (reader) lock to be held prior to entry.

// Turns off thread safety checking within the body of a particular function.

// Macro flag: #define GUARDED_BY(x)
// Macro flag: #define PT_GUARDED_BY(x)
// Macro flag: #define LOCKABLE
// Macro flag: #define LOCKS_EXCLUDED(...)
// Macro flag: #define LOCK_RETURNED(x)
// Macro flag: #define EXCLUSIVE_LOCK_FUNCTION(...)
// Macro flag: #define SHARED_LOCK_FUNCTION(...)
// Macro flag: #define UNLOCK_FUNCTION(...)
// Macro flag: #define EXCLUSIVE_TRYLOCK_FUNCTION(...)
// Macro flag: #define EXCLUSIVE_LOCKS_REQUIRED(...)
// Macro flag: #define SHARED_LOCKS_REQUIRED(...)
// Macro flag: #define NO_THREAD_SAFETY_ANALYSIS

//
// A wrapper around the mutex implementation that allows perf to error check
// usage, etc.
//
// A wrapper around the condition variable implementation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond {
    pub cond: pthread_cond_t,
}

// Default initialize the mtx struct.
extern "C" {
    pub fn mutex_init(mtx: *mut mutex);
}
//
// Initialize the mtx struct and set the process-shared rather than default
// process-private attribute.
//
extern "C" {
    pub fn mutex_init_pshared(mtx: *mut mutex);
}
// Initializes a mutex that may be recursively held on the same thread.
extern "C" {
    pub fn mutex_init_recursive(mtx: *mut mutex);
}
extern "C" {
    pub fn mutex_destroy(mtx: *mut mutex);
}
extern "C" {
    pub fn mutex_lock(EXCLUSIVE_LOCK_FUNCTION(*mtx: *mut *mut mutex mtx));
}
extern "C" {
    pub fn mutex_unlock(UNLOCK_FUNCTION(*mtx: *mut *mut mutex mtx));
}
// Tries to acquire the lock and returns true on success.
extern "C" {
    pub fn mutex_trylock(EXCLUSIVE_TRYLOCK_FUNCTION(true: *mut *mut mutex mtx), _arg: *mut mtx) -> bool;
}
// Default initialize the cond struct.
extern "C" {
    pub fn cond_init(cnd: *mut cond);
}
//
// Initialize the cond struct and specify the process-shared rather than default
// process-private attribute.
//
extern "C" {
    pub fn cond_init_pshared(cnd: *mut cond);
}
extern "C" {
    pub fn cond_destroy(cnd: *mut cond);
}
extern "C" {
    pub fn cond_wait(cnd: *mut cond, EXCLUSIVE_LOCKS_REQUIRED(mtx: *mut *mut mutex mtx));
}
extern "C" {
    pub fn cond_signal(cnd: *mut cond);
}
extern "C" {
    pub fn cond_broadcast(cnd: *mut cond);
}
