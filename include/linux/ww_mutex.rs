//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ww_mutex.h
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
// Wound/Wait Mutexes: blocking mutual exclusion locks with deadlock avoidance
//
// Original mutex implementation started by Ingo Molnar:
//
// Copyright (C) 2004, 2005, 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//
// Wait/Die implementation:
// Copyright (C) 2013 Canonical Ltd.
// Choice of algorithm:
// Copyright (C) 2018 WMWare Inc.
//
// This file contains the main data structure and API definitions.
//

// Macro flag: #define DEBUG_WW_MUTEXES

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ww_class {
    pub stamp: atomic_long_t,
    pub acquire_key: lock_class_key,
    pub mutex_key: lock_class_key,
    pub acquire_name: *const c_char,
    pub mutex_name: *const c_char,
    pub is_wait_die: c_uint,
}

//
// @first_lock_dep_map: fake lockdep_map for first locked ww_mutex.
//
// lockdep requires the lockdep_map for the first locked ww_mutex
// in a ww transaction to remain in memory until all ww_mutexes of
// the transaction have been unlocked. Ensure this by keeping a
// fake locked ww_mutex lockdep map between ww_acquire_init() and
// ww_acquire_fini().
//

//
// ww_mutex_init - initialize the w/w mutex
// @lock: the mutex to be initialized
// @ww_class: the w/w class the mutex should belong to
//
// Initialize the w/w mutex to unlocked state and associate it with the given
// class. Static define macro for w/w mutex is not provided and this function
// is the only way to properly initialize the w/w mutex.
//
// It is not allowed to initialize an already locked mutex.
//

//
// ww_acquire_init - initialize a w/w acquire context
// @ctx: w/w acquire context to initialize
// @ww_class: w/w class of the context
//
// Initializes an context to acquire multiple mutexes of the given w/w class.
//
// Context-based w/w mutex acquiring can be done in any order whatsoever within
// a given lock class. Deadlocks will be detected and handled with the
// wait/die logic.
//
// Mixing of context-based w/w mutex acquiring and single w/w mutex locking can
// result in undetected deadlocks and is so forbidden. Mixing different contexts
// for the same w/w class when acquiring mutexes can also result in undetected
// deadlocks, and is hence also forbidden. Both types of abuse will be caught by
// enabling CONFIG_PROVE_LOCKING.
//
// Nesting of acquire contexts for _different_ w/w classes is possible, subject
// to the usual locking rules between different lock classes.
//
// An acquire context must be released with ww_acquire_fini by the same task
// before the memory is freed. It is recommended to allocate the context itself
// on the stack.
//

//
// ww_acquire_done - marks the end of the acquire phase
// @ctx: the acquire context
//
// Marks the end of the acquire phase, any further w/w mutex lock calls using
// this context are forbidden.
//
// Calling this function is optional, it is just useful to document w/w mutex
// code and clearly designated the acquire phase from actually using the locked
// data structures.
//

//
// ww_acquire_fini - releases a w/w acquire context
// @ctx: the acquire context to free
//
// Releases a w/w acquire context. This must be called _after_ all acquired w/w
// mutexes have been released with ww_mutex_unlock.
//

//
// lockdep will normally handle this,
// but fail without anyway
//
// ensure ww_acquire_fini will still fail if called twice

//
// ww_mutex_lock - acquire the w/w mutex
// @lock: the mutex to be acquired
// @ctx: w/w acquire context, or NULL to acquire only a single lock.
//
// Lock the w/w mutex exclusively for this task.
//
// Deadlocks within a given w/w class of locks are detected and handled with the
// wait/die algorithm. If the lock isn't immediately available this function
// will either sleep until it is (wait case). Or it selects the current context
// for backing off by returning -EDEADLK (die case). Trying to acquire the
// same lock with the same context twice is also detected and signalled by
// returning -EALREADY. Returns 0 if the mutex was successfully acquired.
//
// In the die case the caller must release all currently held w/w mutexes for
// the given context and then wait for this contending lock to be available by
// calling ww_mutex_lock_slow. Alternatively callers can opt to not acquire this
// lock and proceed with trying to acquire further w/w mutexes (e.g. when
// scanning through lru lists trying to free resources).
//
// The mutex must later on be released by the same task that
// acquired it. The task may not exit without first unlocking the mutex. Also,
// kernel memory where the mutex resides must not be freed with the mutex still
// locked. The mutex must first be initialized (or statically defined) before it
// can be locked. memset()-ing the mutex to 0 is not allowed. The mutex must be
// of the same w/w lock class as was used to initialize the acquire context.
//
// A mutex acquired with this function must be released with ww_mutex_unlock.
//
// ww_mutex_lock_interruptible - acquire the w/w mutex, interruptible
// @lock: the mutex to be acquired
// @ctx: w/w acquire context
//
// Lock the w/w mutex exclusively for this task.
//
// Deadlocks within a given w/w class of locks are detected and handled with the
// wait/die algorithm. If the lock isn't immediately available this function
// will either sleep until it is (wait case). Or it selects the current context
// for backing off by returning -EDEADLK (die case). Trying to acquire the
// same lock with the same context twice is also detected and signalled by
// returning -EALREADY. Returns 0 if the mutex was successfully acquired. If a
// signal arrives while waiting for the lock then this function returns -EINTR.
//
// In the die case the caller must release all currently held w/w mutexes for
// the given context and then wait for this contending lock to be available by
// calling ww_mutex_lock_slow_interruptible. Alternatively callers can opt to
// not acquire this lock and proceed with trying to acquire further w/w mutexes
// (e.g. when scanning through lru lists trying to free resources).
//
// The mutex must later on be released by the same task that
// acquired it. The task may not exit without first unlocking the mutex. Also,
// kernel memory where the mutex resides must not be freed with the mutex still
// locked. The mutex must first be initialized (or statically defined) before it
// can be locked. memset()-ing the mutex to 0 is not allowed. The mutex must be
// of the same w/w lock class as was used to initialize the acquire context.
//
// A mutex acquired with this function must be released with ww_mutex_unlock.
//
// ww_mutex_lock_slow - slowpath acquiring of the w/w mutex
// @lock: the mutex to be acquired
// @ctx: w/w acquire context
//
// Acquires a w/w mutex with the given context after a die case. This function
// will sleep until the lock becomes available.
//
// The caller must have released all w/w mutexes already acquired with the
// context and then call this function on the contended lock.
//
// Afterwards the caller may continue to (re)acquire the other w/w mutexes it
// needs with ww_mutex_lock. Note that the -EALREADY return code from
// ww_mutex_lock can be used to avoid locking this contended mutex twice.
//
// It is forbidden to call this function with any other w/w mutexes associated
// with the context held. It is forbidden to call this on anything else than the
// contending mutex.
//
// Note that the slowpath lock acquiring can also be done by calling
// ww_mutex_lock directly. This function here is simply to help w/w mutex
// locking code readability by clearly denoting the slowpath.
//

//
// ww_mutex_lock_slow_interruptible - slowpath acquiring of the w/w mutex, interruptible
// @lock: the mutex to be acquired
// @ctx: w/w acquire context
//
// Acquires a w/w mutex with the given context after a die case. This function
// will sleep until the lock becomes available and returns 0 when the lock has
// been acquired. If a signal arrives while waiting for the lock then this
// function returns -EINTR.
//
// The caller must have released all w/w mutexes already acquired with the
// context and then call this function on the contended lock.
//
// Afterwards the caller may continue to (re)acquire the other w/w mutexes it
// needs with ww_mutex_lock. Note that the -EALREADY return code from
// ww_mutex_lock can be used to avoid locking this contended mutex twice.
//
// It is forbidden to call this function with any other w/w mutexes associated
// with the given context held. It is forbidden to call this on anything else
// than the contending mutex.
//
// Note that the slowpath lock acquiring can also be done by calling
// ww_mutex_lock_interruptible directly. This function here is simply to help
// w/w mutex locking code readability by clearly denoting the slowpath.
//

extern "C" {
    pub fn ww_mutex_lock_interruptible(_arg: lock, _arg: ctx) -> return;
}
extern "C" {
    pub fn ww_mutex_unlock(__releases(lock: *mut *mut ww_mutex lock));
}
//
// ww_mutex_destroy - mark a w/w mutex unusable
// @lock: the mutex to be destroyed
//
// This function marks the mutex uninitialized, and any subsequent
// use of the mutex is forbidden. The mutex must not be locked when
// this function is called.
//

//
// ww_mutex_is_locked - is the w/w mutex locked
// @lock: the mutex to be queried
//
// Returns 1 if the mutex is locked, 0 if unlocked.
//
extern "C" {
    pub fn ww_mutex_base_is_locked(_arg: &lock->base) -> return;
}
