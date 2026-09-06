//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/seqlock_types.h
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
// Sequence counters (seqcount_t)
//
// This is the raw counting mechanism, without any writer protection.
//
// Write side critical sections must be serialized and non-preemptible.
//
// If readers can be invoked from hardirq or softirq contexts,
// interrupts or bottom halves must also be respectively disabled before
// entering the write section.
//
// This mechanism can't be used if the protected data contains pointers,
// as the writer can invalidate a pointer that a reader is following.
//
// If the write serialization mechanism is one of the common kernel
// locking primitives, use a sequence counter with associated lock
// (seqcount_LOCKNAME_t) instead.
//
// If it's desired to automatically handle the sequence counter writer
// serialization and non-preemptibility requirements, use a sequential
// lock (seqlock_t) instead.
//
// See Documentation/locking/seqlock.rst
//

//
// For PREEMPT_RT, seqcount_LOCKNAME_t write side critical sections cannot
// disable preemption. It can lead to higher latencies, and the write side
// sections will not be able to acquire locks which become sleeping locks
// (e.g. spinlock_t).
//
// To remain preemptible while avoiding a possible livelock caused by the
// reader preempting the writer, use a different technique: let the reader
// detect if a seqcount_LOCKNAME_t writer is in progress. If that is the
// case, acquire then release the associated LOCKNAME writer serialization
// lock. This will allow any possibly-preempted writer to make progress
// until the end of its writer serialization lock critical section.
//
// This lock-unlock technique must be implemented for all of PREEMPT_RT
// sleeping locks.  See Documentation/locking/locktypes.rst
//

// Macro flag: #define __SEQ_LOCK(expr)

//
// Sequential locks (seqlock_t)
//
// Sequence counters with an embedded spinlock for writer serialization
// and non-preemptibility.
//
// For more info, see:
// - Comments on top of seqcount_t
// - Documentation/locking/seqlock.rst
//
// Make sure that readers don't starve writers on PREEMPT_RT: use
// seqcount_spinlock_t instead of seqcount_t. Check __SEQ_LOCK().
//
pub type seqlock_t = seqlock;
