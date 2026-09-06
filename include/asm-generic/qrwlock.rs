//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/qrwlock.h
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
// Queue read/write lock
//
// These use generic atomic and locking routines, but depend on a fair spinlock
// implementation in order to be fair themselves.  The implementation in
// asm-generic/spinlock.h meets these requirements.
//
// (C) Copyright 2013-2014 Hewlett-Packard Development Company, L.P.
//
// Authors: Waiman Long <waiman.long@hp.com>
//

// Must be included from asm/spinlock.h after defining arch_spin_is_locked.
//
// Writer states & reader shift and bias.
//
pub const _QW_WAITING: c_uint = 0x100		/* A writer is waiting	   */;
pub const _QW_LOCKED: c_uint = 0x0ff		/* A writer holds the lock */;
pub const _QW_WMASK: c_uint = 0x1ff		/* Writer mask		   */;

//
// External function declarations
//
extern "C" {
    pub fn queued_read_lock_slowpath(lock: *mut qrwlock);
}
extern "C" {
    pub fn queued_write_lock_slowpath(lock: *mut qrwlock);
}
//
// queued_read_trylock - try to acquire read lock of a queued rwlock
// @lock : Pointer to queued rwlock structure
// Return: 1 if lock acquired, 0 if failed
//
// queued_write_trylock - try to acquire write lock of a queued rwlock
// @lock : Pointer to queued rwlock structure
// Return: 1 if lock acquired, 0 if failed
//
// queued_read_lock - acquire read lock of a queued rwlock
// @lock: Pointer to queued rwlock structure
//
// The slowpath will decrement the reader count, if necessary.
//
// queued_write_lock - acquire write lock of a queued rwlock
// @lock : Pointer to queued rwlock structure
//
// Optimize for the unfair lock case where the fair flag is 0.
//
// queued_read_unlock - release read lock of a queued rwlock
// @lock : Pointer to queued rwlock structure
//
// Atomically decrement the reader count
//
// queued_write_unlock - release write lock of a queued rwlock
// @lock : Pointer to queued rwlock structure
//
// queued_rwlock_is_contended - check if the lock is contended
// @lock : Pointer to queued rwlock structure
// Return: 1 if lock contended, 0 otherwise
//
extern "C" {
    pub fn arch_spin_is_locked(_arg: &lock->wait_lock) -> return;
}
//
// Remapping rwlock architecture specific functions to the corresponding
// queued rwlock functions.
//

