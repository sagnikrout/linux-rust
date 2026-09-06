//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcu_segcblist.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// RCU segmented callback lists
//
// This seemingly RCU-private file must be available to SRCU users
// because the size of the TREE SRCU srcu_struct structure depends
// on these definitions.
//
// Copyright IBM Corporation, 2017
//
// Authors: Paul E. McKenney <paulmck@linux.net.ibm.com>
//

// Simple unsegmented callback lists.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_cblist {
    pub head: *mut rcu_head,
    pub tail: *mut rcu_head,
    pub len: c_long,
}

// Complicated segmented callback lists.  ;-)
//
// Index values for segments in rcu_segcblist structure.
//
// The segments are as follows:
//
// [head, *tails[RCU_DONE_TAIL]):
// Callbacks whose grace period has elapsed, and thus can be invoked.
// [*tails[RCU_DONE_TAIL], *tails[RCU_WAIT_TAIL]):
// Callbacks waiting for the current GP from the current CPU's viewpoint.
// [*tails[RCU_WAIT_TAIL], *tails[RCU_NEXT_READY_TAIL]):
// Callbacks that arrived before the next GP started, again from
// the current CPU's viewpoint.  These can be handled by the next GP.
// [*tails[RCU_NEXT_READY_TAIL], *tails[RCU_NEXT_TAIL]):
// Callbacks that might have arrived after the next GP started.
// There is some uncertainty as to when a given GP starts and
// ends, but a CPU knows the exact times if it is the one starting
// or ending the GP.  Other CPUs know that the previous GP ends
// before the next one starts.
//
// Note that RCU_WAIT_TAIL cannot be empty unless RCU_NEXT_READY_TAIL is also
// empty.
//
// The ->gp_seq[] array contains the grace-period state at which the
// corresponding segment of callbacks will be ready to invoke.  This tracks
// both normal and expedited grace periods, allowing callbacks to complete
// when either type of GP finishes.  A given element of this array is
// meaningful only when the corresponding segment is non-empty, and it is
// never valid for RCU_DONE_TAIL (whose callbacks are already ready to
// invoke) or for RCU_NEXT_TAIL (whose callbacks have not yet been assigned
// a grace-period state).
//

pub const RCU_NEXT_TAIL: c_int = 3;
pub const RCU_CBLIST_NSEGS: c_int = 4;
//
// ==NOCB Offloading state machine==
//
// ----------------------------------------------------------------------------
// |                              SEGCBLIST_RCU_CORE                          |
// |                                                                          |
// |  Callbacks processed by rcu_core() from softirqs or local                |
// |  rcuc kthread, without holding nocb_lock.                                |
// ----------------------------------------------------------------------------
// |
// v
// ----------------------------------------------------------------------------
// |       SEGCBLIST_RCU_CORE | SEGCBLIST_LOCKING | SEGCBLIST_OFFLOADED       |
// |                                                                          |
// | Callbacks processed by rcu_core() from softirqs or local                 |
// | rcuc kthread, while holding nocb_lock. Waking up CB and GP kthreads.     |
// ----------------------------------------------------------------------------
// |
// v
// ----------------------------------------------------------------------------
// |        SEGCBLIST_RCU_CORE | SEGCBLIST_LOCKING | SEGCBLIST_OFFLOADED      |
// |                              + unparked CB kthread                       |
// |                                                                          |
// | CB kthread got unparked and processes callbacks concurrently with        |
// | rcu_core(), holding nocb_lock.                                           |
// ---------------------------------------------------------------------------
// |
// v
// ---------------------------------------------------------------------------|
// |                           SEGCBLIST_RCU_CORE |                           |
// |                           SEGCBLIST_LOCKING |                            |
// |                           SEGCBLIST_OFFLOADED |                          |
// |                           SEGCBLIST_KTHREAD_GP                           |
// |                           + unparked CB kthread                          |
// |                                                                          |
// | GP kthread woke up and acknowledged nocb_lock.                           |
// ---------------------------------------- -----------------------------------
// |
// v
// |--------------------------------------------------------------------------|
// |                           SEGCBLIST_LOCKING |                            |
// |                           SEGCBLIST_OFFLOADED |                          |
// |                           SEGCBLIST_KTHREAD_GP |                         |
// |                           + unparked CB kthread                          |
// |                                                                          |
// |   Kthreads handle callbacks holding nocb_lock, local rcu_core() stops    |
// |   handling callbacks. Enable bypass queueing.                            |
// ----------------------------------------------------------------------------
//
// ==NOCB De-Offloading state machine==
//
// |--------------------------------------------------------------------------|
// |                           SEGCBLIST_LOCKING    |                         |
// |                           SEGCBLIST_OFFLOADED  |                         |
// |                           SEGCBLIST_KTHREAD_GP                           |
// |                           + unparked CB kthread                          |
// |                                                                          |
// |   CB/GP kthreads handle callbacks holding nocb_lock, local rcu_core()    |
// |   ignores callbacks. Bypass enqueue is enabled.                          |
// ----------------------------------------------------------------------------
// |
// v
// |--------------------------------------------------------------------------|
// |                           SEGCBLIST_RCU_CORE   |                         |
// |                           SEGCBLIST_LOCKING    |                         |
// |                           SEGCBLIST_OFFLOADED  |                         |
// |                           SEGCBLIST_KTHREAD_GP                           |
// |                           + unparked CB kthread                          |
// |                                                                          |
// |   CB/GP kthreads handle callbacks holding nocb_lock, local rcu_core()    |
// |   handles callbacks concurrently. Bypass enqueue is disabled.            |
// |   Invoke RCU core so we make sure not to preempt it in the middle with   |
// |   leaving some urgent work unattended within a jiffy.                    |
// ----------------------------------------------------------------------------
// |
// v
// |--------------------------------------------------------------------------|
// |                           SEGCBLIST_RCU_CORE   |                         |
// |                           SEGCBLIST_LOCKING    |                         |
// |                           SEGCBLIST_KTHREAD_GP                           |
// |                           + unparked CB kthread                          |
// |                                                                          |
// |   CB/GP kthreads and local rcu_core() handle callbacks concurrently      |
// |   holding nocb_lock. Wake up GP kthread if necessary.                    |
// ----------------------------------------------------------------------------
// |
// v
// |--------------------------------------------------------------------------|
// |                           SEGCBLIST_RCU_CORE   |                         |
// |                           SEGCBLIST_LOCKING    |                         |
// |                           + unparked CB kthread                          |
// |                                                                          |
// |   GP kthread woke up and acknowledged the fact that SEGCBLIST_OFFLOADED  |
// |   got cleared. The callbacks from the target CPU will be ignored from the|
// |   GP kthread loop.                                                       |
// ----------------------------------------------------------------------------
// |
// v
// ----------------------------------------------------------------------------
// |                SEGCBLIST_RCU_CORE | SEGCBLIST_LOCKING                    |
// |                          + parked CB kthread                             |
// |                                                                          |
// | CB kthread is parked. Callbacks processed by rcu_core() from softirqs or |
// | local rcuc kthread, while holding nocb_lock.                             |
// ----------------------------------------------------------------------------
// |
// v
// ----------------------------------------------------------------------------
// |                         SEGCBLIST_RCU_CORE                               |
// |                                                                          |
// |  Callbacks processed by rcu_core() from softirqs or local                |
// |  rcuc kthread, without holding nocb_lock.                                |
// ----------------------------------------------------------------------------
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_segcblist {
    pub head: *mut rcu_head,
    pub tails: [*mut rcu_head; RCU_CBLIST_NSEGS],
    pub gp_seq: [rcu_gp_seq; RCU_CBLIST_NSEGS],
    pub len: atomic_long_t,

    pub len: c_long,
    pub seglen: [c_long; RCU_CBLIST_NSEGS],
    pub flags: u8,
}

