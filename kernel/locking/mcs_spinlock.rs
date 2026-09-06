//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/mcs_spinlock.h
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
// MCS lock defines
//
// This file contains the main data structure and API definitions of MCS lock.
//
// The MCS lock (proposed by Mellor-Crummey and Scott) is a simple spin-lock
// with the desirable properties of being fair, and with each cpu trying
// to acquire the lock spinning on a local variable.
// It avoids expensive cache bounces that common test-and-set spin-lock
// implementations incur.
//

//
// Using smp_cond_load_acquire() provides the acquire semantics
// required so that subsequent operations happen after the
// lock is acquired. Additionally, some architectures such as
// ARM64 would like to do spin-waiting instead of purely
// spinning, and smp_cond_load_acquire() provides that behavior.
//

//
// smp_store_release() provides a memory barrier to ensure all
// operations in the critical section has been completed before
// unlocking.
//

//
// Note: the smp_load_acquire/smp_store_release pair is not
// sufficient to form a full memory barrier across
// cpus for many architectures (except x86) for mcs_unlock and mcs_lock.
// For applications that need a full barrier across multiple cpus
// with mcs_unlock and mcs_lock pair, smp_mb__after_unlock_lock() should be
// used after mcs_lock.
//
// In order to acquire the lock, the caller should declare a local node and
// pass a reference of the node to this function in addition to the lock.
// If the lock has already been acquired, then this will proceed to spin
// on this node->locked until the previous lock holder sets the node->locked
// in mcs_spin_unlock().
//
// Init node
//
// We rely on the full barrier with global transitivity implied by the
// below xchg() to order the initialization stores above against any
// observation of @node. And to provide the ACQUIRE ordering associated
// with a LOCK primitive.
//
// Lock acquired, don't need to set node->locked to 1. Threads
// only spin on its own node->locked value for lock acquisition.
// However, since this thread can immediately acquire the lock
// and does not proceed to spin on its own node->locked, this
// value won't be used. If a debug mode is needed to
// audit lock status, then set node->locked value here.
//
// Wait until the lock holder passes the lock down.
//
// Releases the lock. The caller should pass in the corresponding node that
// was used to acquire the lock.
//
// Release the lock by setting it to NULL
//
// Wait until the next pointer is set
// Pass lock to next waiter.
