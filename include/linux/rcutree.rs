//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcutree.h
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
// Read-Copy Update mechanism for mutual exclusion (tree-based version)
//
// Copyright IBM Corporation, 2008
//
// Author: Dipankar Sarma <dipankar@in.ibm.com>
// Paul E. McKenney <paulmck@linux.ibm.com> Hierarchical algorithm
//
// Based on the original work by Paul McKenney <paulmck@linux.ibm.com>
// and inputs from Rusty Russell, Andrea Arcangeli and Andi Kleen.
//
// For detailed explanation of Read-Copy Update mechanism see -
// Documentation/RCU
//
extern "C" {
    pub fn rcu_softirq_qs();
}
extern "C" {
    pub fn rcu_note_context_switch(preempt: bool);
}
extern "C" {
    pub fn rcu_needs_cpu() -> c_int;
}
extern "C" {
    pub fn rcu_cpu_stall_reset();
}
extern "C" {
    pub fn rcu_request_urgent_qs_task(t: *mut task_struct);
}
//
// Note a virtualization-based context switch.  This is simply a
// wrapper around rcu_note_context_switch(), which allows TINY_RCU
// to save a few bytes. The caller must have disabled interrupts.
//
extern "C" {
    pub fn synchronize_rcu_expedited();
}
extern "C" {
    pub fn rcu_barrier();
}
extern "C" {
    pub fn rcu_momentary_eqs();
}
// Maximum number of rcu_gp_seq values corresponding to
// not-yet-completed RCU grace periods.
pub const NUM_ACTIVE_RCU_POLL_FULL_OLDSTATE: c_int = 4;
//
// same_state_synchronize_rcu_full - Are two old-state values identical?
// @rgosp1: First old-state value.
// @rgosp2: Second old-state value.
//
// The two old-state values must have been obtained from either
// get_state_synchronize_rcu_full(), start_poll_synchronize_rcu_full(),
// or get_completed_synchronize_rcu_full().  Returns @true if the two
// values are identical and @false otherwise.  This allows structures
// whose lifetimes are tracked by old-state values to push these values
// to a list header, allowing those structures to be slightly smaller.
//
// Note that equality is judged on a bitwise basis, so that an
// @rcu_gp_seq structure with an already-completed state in one field
// will compare not-equal to a structure with an already-completed state
// in the other field.  After all, the @rcu_gp_seq structure is opaque
// so how did such a situation come to pass in the first place?
//
extern "C" {
    pub fn start_poll_synchronize_rcu_expedited() -> c_ulong;
}
extern "C" {
    pub fn start_poll_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq);
}
extern "C" {
    pub fn cond_synchronize_rcu_expedited(oldstate: c_ulong);
}
extern "C" {
    pub fn cond_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq);
}
extern "C" {
    pub fn get_state_synchronize_rcu() -> c_ulong;
}
extern "C" {
    pub fn get_state_synchronize_rcu_full(gsp: *mut rcu_gp_seq);
}
extern "C" {
    pub fn start_poll_synchronize_rcu() -> c_ulong;
}
extern "C" {
    pub fn start_poll_synchronize_rcu_full(gsp: *mut rcu_gp_seq);
}
extern "C" {
    pub fn poll_state_synchronize_rcu(oldstate: c_ulong) -> bool;
}
extern "C" {
    pub fn poll_state_synchronize_rcu_full(gsp: *mut rcu_gp_seq) -> bool;
}
extern "C" {
    pub fn cond_synchronize_rcu(oldstate: c_ulong);
}
extern "C" {
    pub fn cond_synchronize_rcu_full(gsp: *mut rcu_gp_seq);
}

extern "C" {
    pub fn rcu_irq_exit_check_preempt();
}

extern "C" {
    pub fn rcu_preempt_deferred_qs(t: *mut task_struct);
}
extern "C" {
    pub fn exit_rcu();
}
extern "C" {
    pub fn rcu_scheduler_starting();
}
extern "C" {
    pub fn rcu_end_inkernel_boot();
}
extern "C" {
    pub fn rcu_inkernel_boot_has_ended() -> bool;
}
extern "C" {
    pub fn rcu_is_watching() -> bool;
}

extern "C" {
    pub fn rcu_all_qs();
}

// RCUtree hotplug events
extern "C" {
    pub fn rcutree_prepare_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn rcutree_online_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn rcutree_report_cpu_starting(cpu: c_uint);
}

extern "C" {
    pub fn rcutree_dead_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn rcutree_dying_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn rcutree_offline_cpu(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn rcutree_migrate_callbacks(cpu: c_int);
}
// Called from hotplug and also arm64 early secondary boot failure
extern "C" {
    pub fn rcutree_report_cpu_dead();
}
