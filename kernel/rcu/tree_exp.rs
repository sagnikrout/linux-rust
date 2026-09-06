//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/tree_exp.h
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
// RCU expedited grace periods
//
// Copyright IBM Corporation, 2016
//
// Authors: Paul E. McKenney <paulmck@linux.ibm.com>
//

extern "C" {
    pub fn rcu_exp_handler(unused: *mut c_void) -> static void;
}
extern "C" {
    pub fn rcu_print_task_exp_stall(rnp: *mut rcu_node) -> static int;
}
extern "C" {
    pub fn rcu_exp_print_detail_task_stall_rnp(rnp: *mut rcu_node) -> static void;
}
//
// Record the start of an expedited grace period.
//
// Return the value that the expedited-grace-period counter will have
// at the end of the current grace period.
//
extern "C" {
    pub fn rcu_seq_endval(_arg: &rcu_state.expedited_sequence) -> return;
}
//
// Record the end of an expedited grace period.
//
// Take a snapshot of the expedited-grace-period counter, which is the
// earliest value that will indicate that a full grace period has
// elapsed since the current time.
//
// Given a counter snapshot from rcu_exp_gp_seq_snap(), return true
// if a full expedited grace period has elapsed since that snapshot
// was taken.
//
extern "C" {
    pub fn rcu_seq_done(_arg: &rcu_state.expedited_sequence, _arg: s) -> return;
}
//
// Reset the ->expmaskinit values in the rcu_node tree to reflect any
// recent CPU-online activity.  Note that these masks are not cleared
// when CPUs go offline, so they reflect the union of all CPUs that have
// ever been online.  This means that this function normally takes its
// no-work-to-do fastpath.
//
// If no new CPUs onlined since last time, nothing to do.
//
// Each pass through the following loop propagates newly onlined
// CPUs for the current rcu_node structure up the rcu_node tree.
//
// Update this node's mask, track old value for propagation.
// If was already nonzero, nothing to propagate.
// Propagate the new CPU up the tree.
//
// Reset the ->expmask values in the rcu_node tree in preparation for
// a new expedited grace period.
//
// Need to wait for any blocked tasks as well.	Note that
// additional blocking tasks will also block the expedited GP
// until such time as the ->expmask bits are cleared.
//
// Return non-zero if there is no RCU expedited grace period in progress
// for the specified rcu_node structure, in other words, if all CPUs and
// tasks covered by the specified rcu_node structure have done their bit
// for the current expedited grace period.
//
// Like sync_rcu_exp_done(), but where the caller does not hold the
// rcu_node's ->lock.
//
// Report the exit from RCU read-side critical section for the last task
// that queued itself during or before the current expedited preemptible-RCU
// grace period.  This event is reported either to the rcu_node structure on
// which the task was queued or to one of that rcu_node structure's ancestors,
// recursively up the tree.  (Calm down, calm down, we do the recursion
// iteratively!)
//
// Report expedited quiescent state for specified node.  This is a
// lock-acquisition wrapper function for __rcu_report_exp_rnp().
//
// Report expedited quiescent state for multiple CPUs, all covered by the
// specified leaf rcu_node structure, which is acquired by the caller.
//
// Report expedited quiescent state for specified rcu_data (CPU).
//
// Common code for work-done checking.
//
// Order GP completion with preceding accesses. Order also GP
// completion with post GP update side accesses. Pairs with
// rcu_seq_end().
//
// Funnel-lock acquisition for expedited grace periods.  Returns true
// if some other task completed an expedited grace period that this task
// can piggy-back on, and with no mutex held.  Otherwise, returns false
// with the mutex held, indicating that the caller must actually do the
// expedited grace period.
//
// Low-contention fastpath.
//
// Each pass through the following loop works its way up
// the rcu_node tree, returning if others have done the work or
// otherwise falls through to acquire ->exp_mutex.  The mapping
// from CPU to rcu_node structure can be inexact, as it is just
// promoting locality and is not strictly needed for correctness.
//
// Work not done, either wait here or go up.
// Someone else doing GP, so wait for them.
//
// Select the CPUs within the specified rcu_node that the upcoming
// expedited grace period needs to wait for.
//
// Each pass checks a CPU for identity, offline, and idle.
//
// Full ordering between remote CPU's post idle accesses
// and updater's accesses prior to current GP (and also
// the started GP sequence number) is enforced by
// rcu_seq_start() implicit barrier, relayed by kworkers
// locking and even further by smp_mb__after_unlock_lock()
// barriers chained all the way throughout the rnp locking
// tree since sync_exp_reset_tree() and up to the current
// leaf rnp locking.
//
// Ordering between remote CPU's pre idle accesses and
// post grace period updater's accesses is enforced by the
// below acquire semantic.
//
// IPI the remaining CPUs for expedited quiescent state.
// The CPU will report the QS in response to the IPI.
// Failed, raced with CPU hotplug operation.
// Online, so delay for a bit and try again.
// CPU really is offline, so we must report its QS.
// Report quiescent states for those that went offline.
extern "C" {
    pub fn rcu_exp_sel_wait_wake(s: c_ulong) -> static void;
}
//
// Use rcu_exp_par_gp_kworker, because flushing a work item from
// another work item on the same kthread worker can result in
// deadlock.
//
// Work-queue handler to drive an expedited grace period forward.
//
// Select the nodes that the upcoming expedited grace period needs
// to wait for.
//
// Schedule work for each leaf rcu_node structure.
// No worker started yet or last leaf, do direct call.
// Wait for jobs (if any) to complete.
//
// Wait for the expedited grace period to elapse, within time limit.
// If the time limit is exceeded without the grace period elapsing,
// return false, otherwise return true.
//
// Workqueues should not be signaled.
//
// Print out an expedited RCU CPU stall warning message.
//
// This is invoked from the grace-period worker, so
// a new grace period cannot have started.  And if this
// worker were stalled, we would not get here.  ;-)
//
// Wait for the expedited grace period to elapse, issuing any needed
// RCU CPU stall warnings along the way.
//
// Wait for the current expedited grace period to complete, and then
// wake up everyone who piggybacked on the just-completed expedited
// grace period.  Also update all the ->exp_seq_rq counters as needed
// in order to avoid counter-wrap problems.
//
// Switch over to wakeup mode, allowing the next GP to proceed.
// End the previous grace period only after acquiring the mutex
// to ensure that only one GP runs concurrently with wakeups.
// Recheck, avoid hang in case someone just arrived.
//
// Common code to drive an expedited grace period forward, used by
// workqueues and mid-boot-time tasks.
//
// Initialize the rcu_node tree in preparation for the wait.
// Wait and clean up, including waking everyone.
// Request an expedited quiescent state.
// Store .exp before .rcu_urgent_qs.

//
// Remote handler for smp_call_function_single().  If there is an
// RCU read-side critical section in effect, request that the
// next rcu_read_unlock() record the quiescent state up the
// ->expmask fields in the rcu_node tree.  Otherwise, immediately
// report the quiescent state.
//
// WARN if the CPU is unexpectedly already looking for a
// QS or has already reported one.
//
// Second, the common case of not being in an RCU read-side
// critical section.  If also enabled or idle, immediately
// report the quiescent state, otherwise defer.
//
// Third, the less-common case of being in an RCU read-side
// critical section.  In this case we can count on a future
// rcu_read_unlock().  However, this rcu_read_unlock() might
// execute on some other CPU, but in that case there will be
// a future context switch.  Either way, if the expedited
// grace period is still waiting on this CPU, set ->deferred_qs
// so that the eventual quiescent state will be reported.
// Note that there is a large group of race conditions that
// can have caused this quiescent state to already have been
// reported, so we really do need to check ->expmask.
//
// Fourth and finally, negative nesting depth should not happen.
//
// Scan the current list of tasks blocked within RCU read-side critical
// sections, printing out the tid of each that is blocking the current
// expedited grace period.
//
// Scan the current list of tasks blocked within RCU read-side critical
// sections, dumping the stack of each that is blocking the current
// expedited grace period.
//
// We could be printing a lot while holding a spinlock.
// Avoid triggering hard lockup.
//

// Invoked on each online non-idle CPU for expedited quiescent state.
//
// Because preemptible RCU does not exist, we never have to check for
// tasks blocked within RCU read-side critical sections that are
// blocking the current expedited grace period.
//
// Because preemptible RCU does not exist, we never have to print out
// tasks blocked within RCU read-side critical sections that are blocking
// the current expedited grace period.
//

//
// synchronize_rcu_expedited - Brute-force RCU grace period
//
// Wait for an RCU grace period, but expedite it.  The basic idea is to
// IPI all non-idle non-nohz online CPUs.  The IPI handler checks whether
// the CPU is in an RCU critical section, and if so, it sets a flag that
// causes the outermost rcu_read_unlock() to report the quiescent state
// for RCU-preempt or asks the scheduler for help for RCU-sched.  On the
// other hand, if the CPU is not in an RCU read-side critical section,
// the IPI handler reports the quiescent state immediately.
//
// Although this is a great improvement over previous expedited
// implementations, it is still unfriendly to real-time workloads, so is
// thus not recommended for any sort of common-case code.  In fact, if
// you are using synchronize_rcu_expedited() in a loop, please restructure
// your code to batch your updates, and then use a single synchronize_rcu()
// instead.
//
// This has the same semantics as (but is more brutal than) synchronize_rcu().
//
// Is the state is such that the call is a grace period?
// Note well that this code runs with !PREEMPT && !SMP.
// In addition, all code that advances grace periods runs
// at process level.  Therefore, this expedited GP overlaps
// with other expedited GPs only by being fully nested within
// them, which allows reuse of ->gp_seq_polled_exp_snap.
// If expedited grace periods are prohibited, fall back to normal.
// Take a snapshot of the sequence number.
// Ensure that load happens before action based on it.
// Direct call during scheduler init and early_initcalls().
// Marshall arguments & schedule the expedited grace period.
// Wait for expedited grace period to complete.
// Let the next expedited grace period start.
//
// Ensure that start_poll_synchronize_rcu_expedited() has the expedited
// RCU grace periods that it needs.
//
// start_poll_synchronize_rcu_expedited - Snapshot current RCU state and start expedited grace period
//
// Returns a cookie to pass to a call to cond_synchronize_rcu(),
// cond_synchronize_rcu_expedited(), or poll_state_synchronize_rcu(),
// allowing them to determine whether or not any sort of grace period has
// elapsed in the meantime.  If the needed expedited grace period is not
// already slated to start, initiates that grace period.
//
// start_poll_synchronize_rcu_expedited_full - Take a full snapshot and start expedited grace period
// @gsp: Place to put snapshot of grace-period state
//
// Places the normal and expedited grace-period states in gsp.  This
// state value can be passed to a later call to cond_synchronize_rcu_full()
// or poll_state_synchronize_rcu_full() to determine whether or not a
// grace period (whether normal or expedited) has elapsed in the meantime.
// If the needed expedited grace period is not already slated to start,
// initiates that grace period.
//
// cond_synchronize_rcu_expedited - Conditionally wait for an expedited RCU grace period
//
// @oldstate: value from get_state_synchronize_rcu(), start_poll_synchronize_rcu(), or start_poll_synchronize_rcu_expedited()
//
// If any type of full RCU grace period has elapsed since the earlier
// call to get_state_synchronize_rcu(), start_poll_synchronize_rcu(),
// or start_poll_synchronize_rcu_expedited(), just return.  Otherwise,
// invoke synchronize_rcu_expedited() to wait for a full grace period.
//
// Yes, this function does not take counter wrap into account.
// But counter wrap is harmless.  If the counter wraps, we have waited for
// more than 2 billion grace periods (and way more on a 64-bit system!),
// so waiting for a couple of additional grace periods should be just fine.
//
// This function provides the same memory-ordering guarantees that
// would be provided by a synchronize_rcu() that was invoked at the call
// to the function that provided @oldstate and that returned at the end
// of this function.
//
// cond_synchronize_rcu_expedited_full - Conditionally wait for an expedited RCU grace period
// @gsp: value from get_state_synchronize_rcu_full(), start_poll_synchronize_rcu_full(), or start_poll_synchronize_rcu_expedited_full()
//
// If a full RCU grace period has elapsed since the call to
// get_state_synchronize_rcu_full(), start_poll_synchronize_rcu_full(),
// or start_poll_synchronize_rcu_expedited_full() from which @gsp was
// obtained, just return.  Otherwise, invoke synchronize_rcu_expedited()
// to wait for a full grace period.
//
// Yes, this function does not take counter wrap into account.
// But counter wrap is harmless.  If the counter wraps, we have waited for
// more than 2 billion grace periods (and way more on a 64-bit system!),
// so waiting for a couple of additional grace periods should be just fine.
//
// This function provides the same memory-ordering guarantees that
// would be provided by a synchronize_rcu() that was invoked at the call
// to the function that provided @gsp and that returned at the end of
// this function.
//
