//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/srcutree.h
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
// tree variant.
//
// Copyright (C) IBM Corporation, 2017
//
// Author: Paul McKenney <paulmck@linux.ibm.com>
//

// One element of the srcu_data srcu_ctrs array.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_ctr {
    pub /: *mut *mut atomic_long_t srcu_locks; / Locks per CPU.,
    pub /: *mut *mut atomic_long_t srcu_unlocks; / Unlocks per CPU.,
}

//
// Per-CPU structure feeding into leaf srcu_node, similar in function
// to rcu_node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_data {
// Read-side state.
    pub /: *mut *mut srcu_ctr srcu_ctrs[2]; / Locks and unlocks per CPU.,
    pub /: *mut *mut int srcu_reader_flavor; / Reader flavor for srcu_struct structure?,
// Values: SRCU_READ_FLAVOR_.*
// Update-side state.
    pub ____cacheline_internodealigned_in_smp: raw_spinlock_t __private lock,
    pub callbacks.*/: *mut *mut rcu_segcblist srcu_cblist; / List of,
    pub /: *mut *mut unsigned long srcu_gp_seq_needed; / Furthest future GP needed.,
    pub /: *mut *mut unsigned long srcu_gp_seq_needed_exp; / Furthest future exp GP.,
    pub /: *mut *mut bool srcu_cblist_invoking; / Invoking these CBs?,
    pub /: *mut *mut timer_list delay_work; / Delay for CB invoking,
    pub /: *mut *mut work_work; / Context for CB invoking.,
    pub /: *mut *mut rcu_head srcu_barrier_head; / For srcu_barrier() use.,
    pub /: *mut *mut rcu_head srcu_ec_head; / For srcu_expedite_current() use.,
    pub /: *mut *mut int srcu_ec_state; / State for srcu_expedite_current().,
    pub /: *mut *mut *mut srcu_node mynode; / Leaf srcu_node.,
    pub /: *mut *mut unsigned long grpmask; / Mask for leaf srcu_node,
// ->srcu_data_have_cbs[].
    pub cpu: c_int,
    pub ssp: *mut srcu_struct,
}

//
// Node in SRCU combining tree, similar in function to rcu_data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_node {
    pub lock: raw_spinlock_t __private,
    pub /: *mut *mut unsigned long srcu_have_cbs[4]; / GP seq for children having CBs, but only,
// if greater than ->srcu_gp_seq.
    pub /: *mut *mut unsigned long srcu_data_have_cbs[4]; / Which srcu_data structs have CBs for given GP?,
    pub /: *mut *mut unsigned long srcu_gp_seq_needed_exp; / Furthest future exp GP.,
    pub /: *mut *mut *mut srcu_node srcu_parent; / Next up in tree.,
    pub /: *mut *mut int grplo; / Least CPU for node.,
    pub /: *mut *mut int grphi; / Biggest CPU for node.,
}

//
// Per-SRCU-domain structure, update-side data linked from srcu_struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_usage {
    pub /: *mut *mut *mut srcu_node node; / Combining tree.,
    pub 1]: *mut *mut srcu_node level[RCU_NUM_LVLS +,
// First node at each level.
    pub /: *mut *mut int srcu_size_state; / Small-to-big transition state.,
    pub /: *mut *mut mutex srcu_cb_mutex; / Serialize CB preparation.,
    pub /: *mut *mut raw_spinlock_t __private lock; / Protect counters and size state.,
    pub /: *mut *mut mutex srcu_gp_mutex; / Serialize GP work.,
    pub /: *mut *mut unsigned long srcu_gp_seq; / Grace-period seq #.,
    pub /: *mut *mut unsigned long srcu_gp_seq_needed; / Latest gp_seq needed.,
    pub /: *mut *mut unsigned long srcu_gp_seq_needed_exp; / Furthest future exp GP.,
    pub /: *mut *mut unsigned long srcu_gp_start; / Last GP start timestamp (jiffies),
    pub /: *mut *mut unsigned long srcu_last_gp_end; / Last GP end timestamp (ns),
    pub /: *mut *mut unsigned long srcu_size_jiffies; / Current contention-measurement interval.,
    pub /: *mut *mut unsigned long srcu_n_lock_retries; / Contention events in current interval.,
    pub /: *mut *mut unsigned long srcu_n_exp_nodelay; / # expedited no-delays in current GP phase.,
    pub /: *mut *mut bool sda_is_static; / May ->sda be passed to free_percpu()?,
    pub /: *mut *mut unsigned long srcu_barrier_seq; / srcu_barrier seq #.,
    pub /: *mut *mut mutex srcu_barrier_mutex; / Serialize barrier ops.,
    pub srcu_barrier_completion: completion,
// Awaken barrier rq at end.
    pub /: *mut *mut atomic_t srcu_barrier_cpu_cnt; / # CPUs not yet posting a,
// callback for the barrier
// operation.
    pub reschedule_jiffies: c_ulong,
    pub reschedule_count: c_ulong,
    pub work: delayed_work,
    pub irq_work: irq_work,
    pub srcu_ssp: *mut srcu_struct,
}

//
// Per-SRCU-domain structure, similar in function to rcu_state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcu_struct {
    pub srcu_ctrp: *mut srcu_ctr __percpu,
    pub /: *mut *mut *mut srcu_data __percpu sda; / Per-CPU srcu_data array.,
    pub srcu_reader_flavor: u8,
    pub dep_map: lockdep_map,
    pub /: *mut *mut *mut srcu_usage srcu_sup; / Update-side data.,
}

// Values for size state variable (->srcu_size_state).  Once the state
// has been set to SRCU_SIZE_ALLOC, the grace-period code advances through
// this state machine one step per grace period until the SRCU_SIZE_BIG state
// is reached.  Otherwise, the state machine remains in the SRCU_SIZE_SMALL
// state indefinitely.

// and then referenced by ->node.  It will not be used.

// except call_srcu(), especially by srcu_barrier().
// By the end of this state, all CPUs and threads
// are aware of this tree's existence.

// By the end of this state, all of the call_srcu()
// invocations that were running on a non-boot CPU
// and using the boot CPU's callback queue will have
// completed.

// and all aspects of it are being put to use.
// Values for state variable (bottom bits of ->srcu_gp_seq).
pub const SRCU_STATE_IDLE: c_int = 0;
pub const SRCU_STATE_SCAN1: c_int = 1;
pub const SRCU_STATE_SCAN2: c_int = 2;
// Values for srcu_expedite_current() state (->srcu_ec_state).
pub const SRCU_EC_IDLE: c_int = 0;
pub const SRCU_EC_PENDING: c_int = 1;
pub const SRCU_EC_REPOST: c_int = 2;
//
// Values for initializing gp sequence fields. Higher values allow wrap arounds to
// occur earlier.
// The second value with state is useful in the case of static initialization of
// srcu_usage where srcu_gp_seq_needed is expected to have some state value in its
// lower bits (or else it will appear to be already initialized within
// the call check_init_srcu_struct()).
//

//
// Define and initialize a srcu struct at build time.
// Do -not- call init_srcu_struct() nor cleanup_srcu_struct() on it.
//
// Note that although DEFINE_STATIC_SRCU() hides the name from other
// files, the per-CPU variable rules nevertheless require that the
// chosen name be globally unique.  These rules also prohibit use of
// DEFINE_STATIC_SRCU() within a function.  If these rules are too
// restrictive, declare the srcu_struct manually.  For example, in
// each file:
//
// static struct srcu_struct my_srcu;
//
// Then, before the first use of each my_srcu, manually initialize it:
//
// init_srcu_struct(&my_srcu);
//
// See include/linux/percpu-defs.h for the rules on per-CPU variables.
//
// DEFINE_SRCU_FAST() and DEFINE_STATIC_SRCU_FAST create an srcu_struct
// and associated structures whose readers must be of the SRCU-fast variety.
// DEFINE_SRCU_FAST_UPDOWN() and DEFINE_STATIC_SRCU_FAST_UPDOWN() create
// an srcu_struct and associated structures whose readers must be of the
// SRCU-fast-updown variety.  The key point (aside from error checking) with
// both varieties is that the grace periods must use synchronize_rcu()
// instead of smp_mb(), and given that the first (for example)
// srcu_read_lock_fast() might race with the first synchronize_srcu(),
// this different must be specified at initialization time.
//

// not static */)

extern "C" {
    pub fn __srcu_read_lock(__acquires_shared(ssp: *mut *mut srcu_ssp)) -> c_int;
}
extern "C" {
    pub fn synchronize_srcu_expedited(ssp: *mut srcu_struct);
}
extern "C" {
    pub fn srcu_barrier(ssp: *mut srcu_struct);
}
extern "C" {
    pub fn srcu_expedite_current(ssp: *mut srcu_struct);
}
extern "C" {
    pub fn srcu_torture_stats_print(ssp: *mut srcu_struct, tt: *mut c_char, tf: *mut c_char);
}
// Converts a per-CPU pointer to an ->srcu_ctrs[] array element to that
// element's index.
// Converts an integer to a per-CPU pointer to the corresponding
// ->srcu_ctrs[] array element.
//
// Counts the new reader in the appropriate per-CPU element of the
// srcu_struct.  Returns a pointer that must be passed to the matching
// srcu_read_unlock_fast().
//
// Note that both this_cpu_inc() and atomic_long_inc() are RCU read-side
// critical sections either because they disable interrupts, because
// they are a single instruction, or because they are read-modify-write
// atomic operations, depending on the whims of the architecture.
// This matters because the SRCU-fast grace-period mechanism uses either
// synchronize_rcu() or synchronize_rcu_expedited(), that is, RCU,
// *not* SRCU, in order to eliminate the need for the read-side smp_mb()
// invocations that are used by srcu_read_lock() and srcu_read_unlock().
// The __srcu_read_unlock_fast() function also relies on this same RCU
// (again, *not* SRCU) trick to eliminate the need for smp_mb().
//
// The key point behind this RCU trick is that if any part of a given
// RCU reader precedes the beginning of a given RCU grace period, then
// the entirety of that RCU reader and everything preceding it happens
// before the end of that same RCU grace period.  Similarly, if any part
// of a given RCU reader follows the end of a given RCU grace period,
// then the entirety of that RCU reader and everything following it
// happens after the beginning of that same RCU grace period.  Therefore,
// the operations labeled Y in __srcu_read_lock_fast() and those labeled Z
// in __srcu_read_unlock_fast() are ordered against the corresponding SRCU
// read-side critical section from the viewpoint of the SRCU grace period.
// This is all the ordering that is required, hence no calls to smp_mb().
//
// This means that __srcu_read_lock_fast() is not all that fast
// on architectures that support NMIs but do not supply NMI-safe
// implementations of this_cpu_inc().
//
// Removes the count for the old reader from the appropriate
// per-CPU element of the srcu_struct.  Note that this may well be a
// different CPU than that which was incremented by the corresponding
// srcu_read_lock_fast(), but it must be within the same task.
//
// Please see the __srcu_read_lock_fast() function's header comment for
// information on implicit RCU readers and NMI safety.
//
// Counts the new reader in the appropriate per-CPU element of the
// srcu_struct.  Returns a pointer that must be passed to the matching
// srcu_read_unlock_fast_updown().  This type of reader is compatible
// with srcu_down_read_fast() and srcu_up_read_fast().
//
// See the __srcu_read_lock_fast() comment for more details.
//
// Removes the count for the old reader from the appropriate
// per-CPU element of the srcu_struct.  Note that this may well be a
// different CPU than that which was incremented by the corresponding
// srcu_read_lock_fast(), but it must be within the same task.
//
// Please see the __srcu_read_lock_fast() function's header comment for
// information on implicit RCU readers and NMI safety.
//
extern "C" {
    pub fn __srcu_check_read_flavor(ssp: *mut srcu_struct, read_flavor: c_int);
}
// Record SRCU-reader usage type only for CONFIG_PROVE_RCU=y kernels.
//
// srcu_readers_active - returns true if there are readers. and false otherwise.
// @ssp: which srcu_struct to count active readers (holding srcu_read_lock).
//
// Note that this is not an atomic primitive, and can therefore suffer
// severe errors when invoked on an active srcu_struct. That said, it
// can be useful as an error check at cleanup time.
//
