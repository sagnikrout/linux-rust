//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/rcu_segcblist.h
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
// RCU segmented callback lists, internal-to-rcu header file
//
// Copyright IBM Corporation, 2017
//
// Authors: Paul E. McKenney <paulmck@linux.ibm.com>
//

// Return number of callbacks in the specified callback list.
extern "C" {
    pub fn READ_ONCE(_arg: rclp->len) -> return;
}
extern "C" {
    pub fn rcu_segcblist_get_seglen(rsclp: *mut rcu_segcblist, seg: c_int) -> c_long;
}
// Return number of callbacks in segmented callback list by summing seglen.
extern "C" {
    pub fn rcu_segcblist_n_segment_cbs(rsclp: *mut rcu_segcblist) -> c_long;
}
extern "C" {
    pub fn rcu_cblist_init(rclp: *mut rcu_cblist);
}
extern "C" {
    pub fn rcu_cblist_enqueue(rclp: *mut rcu_cblist, rhp: *mut rcu_head);
}
//
// Is the specified rcu_segcblist structure empty?
//
// But careful!  The fact that the ->head field is NULL does not
// necessarily imply that there are no callbacks associated with
// this structure.  When callbacks are being invoked, they are
// removed as a group.  If callback invocation must be preempted,
// the remaining callbacks will be added back to the list.  Either
// way, the counts are updated later.
//
// So it is often the case that rcu_segcblist_n_cbs() should be used
// instead.
//
// Return number of callbacks in segmented callback list.

extern "C" {
    pub fn atomic_long_read(_arg: &rsclp->len) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: rsclp->len) -> return;
}

//
// Is the specified rcu_segcblist enabled, for example, not corresponding
// to an offline CPU?
//
extern "C" {
    pub fn rcu_segcblist_test_flags(_arg: rsclp, _arg: SEGCBLIST_ENABLED) -> return;
}
//
// Is the specified rcu_segcblist NOCB offloaded (or in the middle of the
// [de]offloading process)?
//
// Are all segments following the specified segment of the specified
// rcu_segcblist structure empty of callbacks?  (The specified
// segment might well contain callbacks.)
//
// Is the specified segment of the specified rcu_segcblist structure
// empty of callbacks?
//
extern "C" {
    pub fn rcu_segcblist_inc_len(rsclp: *mut rcu_segcblist);
}
extern "C" {
    pub fn rcu_segcblist_add_len(rsclp: *mut rcu_segcblist, v: c_long);
}
extern "C" {
    pub fn rcu_segcblist_init(rsclp: *mut rcu_segcblist);
}
extern "C" {
    pub fn rcu_segcblist_disable(rsclp: *mut rcu_segcblist);
}
extern "C" {
    pub fn rcu_segcblist_ready_cbs(rsclp: *mut rcu_segcblist) -> bool;
}
extern "C" {
    pub fn rcu_segcblist_pend_cbs(rsclp: *mut rcu_segcblist) -> bool;
}
extern "C" {
    pub fn rcu_segcblist_nextgp(rsclp: *mut rcu_segcblist, gsp: *mut rcu_gp_seq) -> bool;
}
extern "C" {
    pub fn rcu_segcblist_advance(rsclp: *mut rcu_segcblist);
}
extern "C" {
    pub fn rcu_segcblist_accelerate(rsclp: *mut rcu_segcblist, gsp: *mut rcu_gp_seq) -> bool;
}
extern "C" {
    pub fn srcu_segcblist_advance(rsclp: *mut rcu_segcblist, seq: c_ulong);
}
extern "C" {
    pub fn srcu_segcblist_accelerate(rsclp: *mut rcu_segcblist, seq: c_ulong) -> bool;
}
