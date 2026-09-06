//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/rtmutex_common.h
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
// This file contains the private data structure and API definitions.
//

//
// This is a helper for the struct rt_mutex_waiter below. A waiter goes in two
// separate trees and they need their own copy of the sort keys because of
// different locking requirements.
//
// @entry:		rbtree node to enqueue into the waiters tree
// @prio:		Priority of the waiter
// @deadline:		Deadline of the waiter if applicable
//
// See rt_waiter_node_less() and waiter_*_prio().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_waiter_node {
    pub entry: rb_node,
    pub prio: c_int,
    pub deadline: u64,
}

//
// This is the control structure for tasks blocked on a rt_mutex,
// which is allocated on the kernel stack on of the blocked task.
//
// @tree:		node to enqueue into the mutex waiters tree
// @pi_tree:		node to enqueue into the mutex owner waiters tree
// @task:		task reference to the blocked task
// @lock:		Pointer to the rt_mutex on which the waiter blocks
// @wake_state:		Wakeup state to use (TASK_NORMAL or TASK_RTLOCK_WAIT)
// @ww_ctx:		WW context pointer
//
// @tree is ordered by @lock->wait_lock
// @pi_tree is ordered by rt_mutex_owner(@lock)->pi_lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_mutex_waiter {
    pub tree: rt_waiter_node,
    pub pi_tree: rt_waiter_node,
    pub task: *mut task_struct,
    pub lock: *mut rt_mutex_base,
    pub wake_state: c_uint,
    pub ww_ctx: *mut ww_acquire_ctx,
}

//
// struct rt_wake_q_head - Wrapper around regular wake_q_head to support
// "sleeping" spinlocks on RT
// @head:		The regular wake_q_head for sleeping lock variants
// @rtlock_task:	Task pointer for RT lock (spin/rwlock) wakeups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_wake_q_head {
    pub head: wake_q_head,
    pub rtlock_task: *mut task_struct,
}

//
// PI-futex support (proxy locking functions, etc.):
//
extern "C" {
    pub fn rt_mutex_futex_trylock(lock: *mut rt_mutex_base) -> c_int;
}
extern "C" {
    pub fn rt_mutex_futex_unlock(lock: *mut rt_mutex_base);
}
extern "C" {
    pub fn rt_mutex_postunlock(wqh: *mut rt_wake_q_head);
}
//
// Must be guarded because this header is included from rcu/tree_plugin.h
// unconditionally.
//

//
// Lockless speculative check whether @waiter is still the top waiter on
// @lock. This is solely comparing pointers and not derefencing the
// leftmost entry which might be about to vanish.
//
// Constants for rt mutex functions which have a selectable deadlock
// detection.
//
// RT_MUTEX_MIN_CHAINWALK:	Stops the lock chain walk when there are
// no further PI adjustments to be made.
//
// RT_MUTEX_FULL_CHAINWALK:	Invoke deadlock detection with a full
// walk of the lock chain.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtmutex_chainwalk {
    RT_MUTEX_MIN_CHAINWALK,
    RT_MUTEX_FULL_CHAINWALK,
}

// Debug functions

// Used in rcu/tree_plugin.h

