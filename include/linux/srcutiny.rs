//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/srcutiny.h
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
// Sleepable Read-Copy Update mechanism for mutual exclusion,
// tiny variant.
//
// Copyright (C) IBM Corporation, 2017
//
// Author: Paul McKenney <paulmck@linux.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_struct {
    pub /: *mut *mut short srcu_lock_nesting[2]; / srcu_read_lock() nesting depth.,
    pub /: *mut *mut u8 srcu_gp_running; / GP workqueue running?,
    pub /: *mut *mut u8 srcu_gp_waiting; / GP waiting for readers?,
    pub /: *mut *mut unsigned long srcu_idx; / Current reader array element in bit 0x2.,
    pub /: *mut *mut unsigned long srcu_idx_max; / Furthest future srcu_idx request.,
    pub srcu_wq: swait_queue_head,
// Last srcu_read_unlock() wakes GP.
    pub /: *mut *mut *mut rcu_head srcu_cb_head; / Pending callbacks: Head.,
    pub /: *mut *mut *mut *mut rcu_head srcu_cb_tail; / Pending callbacks: Tail.,
    pub /: *mut *mut work_srcu_work; / For driving grace periods.,
    pub /: *mut *mut irq_work srcu_irq_work; / Defer schedule_work() to irq work.,

    pub dep_map: lockdep_map,

}

extern "C" {
    pub fn srcu_drive_gp(wp: *mut work_struct);
}
extern "C" {
    pub fn srcu_tiny_irq_work(irq_work: *mut irq_work);
}

//
// This odd _STATIC_ arrangement is needed for API compatibility with
// Tree SRCU, which needs some per-CPU data.
//

// Dummy structure for srcu_notifier_head.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_usage {

    pub ssp): *mut void synchronize_srcu(struct srcu_struct,
//
// Counts the new reader in the appropriate per-CPU element of the
// srcu_struct.  Can be invoked from irq/bh handlers, but the matching
// __srcu_read_unlock() must be in the same handler instance.  Returns an
// index that must be passed to the matching srcu_read_unlock().
//
    pub idx: c_int,
    pub PREEMPT_LAZY: preempt_disable(); // Needed for,
    pub 1: idx = ((READ_ONCE(ssp->srcu_idx) + 1) & 0x2) >>,
    pub 1): WRITE_ONCE(ssp->srcu_lock_nesting[idx], READ_ONCE(ssp->srcu_lock_nesting[idx]) +,
    pub idx: return,
    pub srcu_ctr: struct,
    pub )scpp: *mut return (int)(intptr_t)(struct srcu_ctr  __kernel,
    pub )(intptr_t)idx: *mut return (struct srcu_ctr __percpu,
    pub __srcu_read_lock(ssp)): return __srcu_ctr_to_ptr(ssp,,
    pub scp)): __srcu_read_unlock(ssp, __srcu_ptr_to_ctr(ssp,,
    pub __srcu_read_lock(ssp)): return __srcu_ctr_to_ptr(ssp,,
    pub scp)): __srcu_read_unlock(ssp, __srcu_ptr_to_ctr(ssp,,

// Defined here to avoid size increase for non-torture kernels.
    pub idx: c_int,
    pub 1: idx = ((data_race(READ_ONCE(ssp->srcu_idx)) + 1) & 0x2) >>,
//
// srcu_readers_active - returns true if there are readers. and false otherwise.
// @ssp: which srcu_struct to count active readers (holding srcu_read_lock).
//
// Note that this is not an atomic primitive, and can therefore suffer
// severe errors when invoked on an active srcu_struct. That said, it
// can be useful as an error check at cleanup time.
//
    pub READ_ONCE(ssp->srcu_lock_nesting[1]): return READ_ONCE(ssp->srcu_lock_nesting[0]) ||,
