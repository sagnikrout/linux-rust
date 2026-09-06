//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcupdate_wait.h
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
// RCU synchronization types and methods:
//

//
// Structure allowing asynchronous waiting on RCU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_synchronize {
    pub head: rcu_head,
    pub completion: completion,
// This is for debugging.
    pub oldstate: rcu_gp_seq,
}

extern "C" {
    pub fn wakeme_after_rcu(head: *mut rcu_head);
}

//
// synchronize_rcu_mult - Wait concurrently for multiple grace periods
// @...: List of call_rcu() functions for different grace periods to wait on
//
// This macro waits concurrently for multiple types of RCU grace periods.
// For example, synchronize_rcu_mult(call_rcu, call_rcu_tasks) would wait
// on concurrent RCU and RCU-tasks grace periods.  Waiting on a given SRCU
// domain requires you to write a wrapper function for that SRCU domain's
// call_srcu() function, with this wrapper supplying the pointer to the
// corresponding srcu_struct.
//
// Note that call_rcu_hurry() should be used instead of call_rcu()
// because in kernels built with CONFIG_RCU_LAZY=y the delay between the
// invocation of call_rcu() and that of the corresponding RCU callback
// can be multiple seconds.
//
// The first argument tells Tiny RCU's _wait_rcu_gp() not to
// bother waiting for RCU.  The reason for this is because anywhere
// synchronize_rcu_mult() can be called is automatically already a full
// grace period.
//

// Has the current task blocked within its current RCU read-side
// critical section?

