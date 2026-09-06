//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcutiny.h
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
// Read-Copy Update mechanism for mutual exclusion, the Bloatwatch edition.
//
// Copyright IBM Corporation, 2008
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//
// For detailed explanation of Read-Copy Update mechanism see -
// Documentation/RCU
//

// Maximum number of rcu_gp_seq values corresponding to
// not-yet-completed RCU grace periods.
pub const NUM_ACTIVE_RCU_POLL_FULL_OLDSTATE: c_int = 2;
//
// Are the two oldstate values the same?  See the Tree RCU version for
// docbook header.
//
extern "C" {
    pub fn get_state_synchronize_rcu() -> c_ulong;
}
extern "C" {
    pub fn start_poll_synchronize_rcu() -> c_ulong;
}
extern "C" {
    pub fn poll_state_synchronize_rcu(oldstate: c_ulong) -> bool;
}
extern "C" {
    pub fn poll_state_synchronize_rcu(_arg: gsp->norm) -> return;
}
extern "C" {
    pub fn start_poll_synchronize_rcu() -> return;
}
extern "C" {
    pub fn rcu_barrier();
}
extern "C" {
    pub fn rcu_qs();
}

//
// Take advantage of the fact that there is only one CPU, which
// allows us to ignore virtualization-based context switches.
//
extern "C" {
    pub fn rcu_scheduler_starting();
}
// Avoid RCU read-side critical sections leaking across.
// RCUtree hotplug events

