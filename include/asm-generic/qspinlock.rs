//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/qspinlock.h
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
// Queued spinlock
//
// A 'generic' spinlock implementation that is based on MCS locks. For an
// architecture that's looking for a 'generic' spinlock, please first consider
// ticket-lock.h and only come looking here when you've considered all the
// constraints below and can show your hardware does actually perform better
// with qspinlock.
//
// qspinlock relies on atomic_*_release()/atomic_*_acquire() to be RCsc (or no
// weaker than RCtso if you're power), where regular code only expects atomic_t
// to be RCpc.
//
// qspinlock relies on a far greater (compared to asm-generic/spinlock.h) set
// of atomic operations to behave well together, please audit them carefully to
// ensure they all have forward progress. Many atomic operations may default to
// cmpxchg() loops which will not have good forward progress properties on
// LL/SC architectures.
//
// One notable example is atomic_fetch_or_acquire(), which x86 cannot (cheaply)
// do. Carefully read the patches that introduced
// queued_fetch_set_pending_acquire().
//
// qspinlock also heavily relies on mixed size atomic operations, in specific
// it requires architectures to have xchg16; something which many LL/SC
// architectures need to implement as a 32bit and+or in order to satisfy the
// forward progress guarantees mentioned above.
//
// Further reading on mixed size atomics that might be relevant:
//
// http://www.cl.cam.ac.uk/~pes20/popl17/mixed-size.pdf
//
// (C) Copyright 2013-2015 Hewlett-Packard Development Company, L.P.
// (C) Copyright 2015 Hewlett-Packard Enterprise Development LP
//
// Authors: Waiman Long <waiman.long@hpe.com>
//

//
// queued_spin_is_locked - is the spinlock locked?
// @lock: Pointer to queued spinlock structure
// Return: 1 if it is locked, 0 otherwise
//
// Any !0 state indicates it is locked, even if _Q_LOCKED_VAL
// isn't immediately observable.
//
extern "C" {
    pub fn atomic_read(_arg: &lock->val) -> return;
}

//
// queued_spin_value_unlocked - is the spinlock structure unlocked?
// @lock: queued spinlock structure
// Return: 1 if it is unlocked, 0 otherwise
//
// N.B. Whenever there are tasks waiting for the lock, it is considered
// locked wrt the lockref code to avoid lock stealing by the lockref
// code and change things underneath the lock. This also allows some
// optimizations to be applied without conflict with lockref.
//
// queued_spin_is_contended - check if the lock is contended
// @lock : Pointer to queued spinlock structure
// Return: 1 if lock contended, 0 otherwise
//
// queued_spin_trylock - try to acquire the queued spinlock
// @lock : Pointer to queued spinlock structure
// Return: 1 if lock acquired, 0 if failed
//
extern "C" {
    pub fn likely(_arg: atomic_try_cmpxchg_acquire(&lock->val, _arg: &val, _arg: _Q_LOCKED_VAL)) -> return;
}
extern "C" {
    pub fn queued_spin_lock_slowpath(lock: *mut qspinlock, val: u32);
}

//
// queued_spin_lock - acquire a queued spinlock
// @lock: Pointer to queued spinlock structure
//

//
// queued_spin_release - release a queued spinlock
// @lock : Pointer to queued spinlock structure
//
// unlock() needs release semantics:
//

extern "C" {
    pub fn queued_spin_release_traced(lock: *mut qspinlock);
}
//
// queued_spin_unlock - unlock a queued spinlock
// @lock : Pointer to queued spinlock structure
//
// Generic tracing wrapper around the arch-overridable
// queued_spin_release().
//
// Trace and release are combined in queued_spin_release_traced() so
// the compiler does not need to preserve the lock pointer across the
// function call, avoiding callee-saved register save/restore on the
// hot path. queued_spin_release() is therefore called both here and in
// queued_spin_release_traced(). Keep the two in sync.
//

//
// Remapping spinlock architecture specific functions to the corresponding
// queued spinlock functions.
//

