//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/qspinlock.h
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
// Queued spinlock defines
//
// This file contains macro definitions and functions shared between different
// qspinlock slow path implementations.
//

pub const _Q_MAX_NODES: c_int = 4;
//
// The pending bit spinning loop count.
// This heuristic is used to limit the number of lockword accesses
// made by atomic_cond_read_relaxed when waiting for the lock to
// transition out of the "== _Q_PENDING_VAL" state. We don't spin
// indefinitely because there's no guarantee that we'll make forward
// progress.
//
pub const _Q_PENDING_LOOPS: c_int = 1;

//
// On 64-bit architectures, the mcs_spinlock structure will be 16 bytes in
// size and four of them will fit nicely in one 64-byte cacheline. For
// pvqspinlock, however, we need more space for extra data. To accommodate
// that, we insert two more long words to pad it up to 32 bytes. IOW, only
// two of them can fit in a cacheline in this case. That is OK as it is rare
// to have more than 2 levels of slowpath nesting in actual use. We don't
// want to penalize pvqspinlocks to optimize for a rare case in native
// qspinlocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnode {
    pub mcs: mcs_spinlock,
    pub reserved: [c_long; 2],
}

//
// We must be able to distinguish between no-tail and the tail at 0:0,
// therefore increment the cpu number by one.
//
extern "C" {
    pub fn per_cpu_ptr(_arg: &qnodes[idx].mcs, _arg: cpu) -> return;
}

//
// clear_pending - clear the pending bit.
// @lock: Pointer to queued spinlock structure
//
// *,1,* -> *,0,
//
// clear_pending_set_locked - take ownership and clear the pending bit.
// @lock: Pointer to queued spinlock structure
//
// *,1,0 -> *,0,1
//
// Lock stealing is not allowed if this function is used.
//
// xchg_tail - Put in the new queue tail code word & retrieve previous one
// @lock : Pointer to queued spinlock structure
// @tail : The new queue tail code word
// Return: The previous queue tail code word
//
// xchg(lock, tail), which heads an address dependency
//
// p,*,* -> n,*,* ; prev = xchg(lock, node)
//
// We can use relaxed semantics since the caller ensures that the
// MCS node is properly initialized before updating the tail.
//

//
// clear_pending - clear the pending bit.
// @lock: Pointer to queued spinlock structure
//
// *,1,* -> *,0,
//
// clear_pending_set_locked - take ownership and clear the pending bit.
// @lock: Pointer to queued spinlock structure
//
// *,1,0 -> *,0,1
//
// xchg_tail - Put in the new queue tail code word & retrieve previous one
// @lock : Pointer to queued spinlock structure
// @tail : The new queue tail code word
// Return: The previous queue tail code word
//
// xchg(lock, tail)
//
// p,*,* -> n,*,* ; prev = xchg(lock, node)
//
// We can use relaxed semantics since the caller ensures that
// the MCS node is properly initialized before updating the
// tail.
//

//
// queued_fetch_set_pending_acquire - fetch the whole lock value and set pending
// @lock : Pointer to queued spinlock structure
// Return: The previous lock value
//
// *,*,* -> *,1,
//

extern "C" {
    pub fn atomic_fetch_or_acquire(_arg: _Q_PENDING_VAL, _arg: &lock->val) -> return;
}

//
// set_locked - Set the lock bit and own the lock
// @lock: Pointer to queued spinlock structure
//
// *,*,0 -> *,0,1
//
