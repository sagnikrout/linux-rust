//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/tree_stall.h
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
// RCU CPU stall warnings for normal RCU grace periods
//
// Copyright IBM Corporation, 2019
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//

//
// Controlling CPU stall warnings, including delay calculation.
// panic() on RCU Stall sysctl.

extern "C" {
    pub fn sysfs_emit(_arg: page, _arg: "%u\n", _arg: rcu_stall_count) -> return;
}

pub const RCU_STALL_DELAY_DELTA: c_int = 0;

pub const RCU_STALL_MIGHT_DIV: c_int = 8;

// Zero says to use rcu_cpu_stall_timeout, but in milliseconds.
// Limit check must be consistent with the Kconfig limits for
// CONFIG_RCU_EXP_CPU_STALL_TIMEOUT, so check the allowed range.
// The minimum clamped value is "2UL", because at least one full
// tick has to be guaranteed.

// Add extra ~25% out of till_stall_check.

// Limit-check stall timeouts specified at boottime and runtime.
//
// Limit check must be consistent with the Kconfig limits
// for CONFIG_RCU_CPU_STALL_TIMEOUT.
//
// Don't do RCU CPU stall warnings during long sysrq printouts.
// Don't print RCU CPU stall warnings during a kernel panic.
// If so specified via sysctl, panic, yielding cleaner stall-warning output.
//
// Attempt to kick out the BPF scheduler if it's installed and defer
// the panic to give the system a chance to recover.
//
// rcu_cpu_stall_reset - restart stall-warning timeout for current grace period
//
// To perform the reset request from the caller, disable stall detection until
// 3 fqs loops have passed. This is required to ensure a fresh jiffies is
// loaded.  It should be safe to do from the fqs loop as enough timer
// interrupts and context switches should have passed.
//
// The caller must disable hard irqs.
//
// Interaction with RCU grace periods
// Start of new grace period, so record stall time (and forcing times).
// Zero ->ticks_this_gp and snapshot the number of RCU softirq handlers.
//
// If too much time has passed in the current grace period, and if
// so configured, go kick the relevant kthreads.
//
// Handler for the irq_work request posted about halfway into the RCU CPU
// stall timeout, and used to detect excessive irq disabling.  Set state
// appropriately, but just complain if there is unexpected state on entry.
//
// Printing RCU CPU stall warnings

//
// Dump detailed information for all tasks blocking the current RCU
// grace period on the specified rcu_node structure.
//
// We could be printing a lot while holding a spinlock.
// Avoid triggering hard lockup.
//
// Communicate task state back to the RCU CPU stall warning request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_stall_chk_rdr {
    pub nesting: c_int,
    pub rs: rcu_special,
    pub on_blkd_list: bool,
}

//
// Report out the state of a not-running task that is stalling the
// current RCU grace period.
//
// Scan the current list of tasks blocked within RCU read-side critical
// sections, printing out the tid of each of the first few of them.
//

//
// Because preemptible RCU does not exist, we never have to check for
// tasks blocked within RCU read-side critical sections.
//
// Because preemptible RCU does not exist, we never have to check for
// tasks blocked within RCU read-side critical sections.
//

//
// Dump stacks of all tasks running on stalled CPUs.  First try using
// NMIs, but fall back to manual remote stack tracing on architectures
// that don't support NMI-based stack dumps.  The NMI-triggered stack
// traces are more accurate because they are printed by the target CPU.
//
// Convert a ->gp_state value to a character string.
//
// Is the RCU grace-period kthread being starved of CPU time?
// jp = j;
//
// Print out diagnostic information for the specified stalled CPU.
//
// If the specified CPU is aware of the current RCU grace period, then
// print the number of scheduling clock interrupts the CPU has taken
// during the time that it has been aware.  Otherwise, print the number
// of RCU grace periods that this CPU is ignorant of, for example, "1"
// if the CPU was aware of the previous grace period.
//
// Also print out idle info.
//
// We could be printing a lot while holding a spinlock.  Avoid
// triggering hard lockup.
//
// Print signed value, as negative values indicate a probable bug.
// Complain about starvation of grace-period kthread.
// Complain about missing wakeups from expired fqs wait timer
//
// Order reads of .gp_state and .jiffies_force_qs.
// Matching smp_wmb() is present in rcu_gp_fqs_loop().
//
// Kick and suppress, if so configured.
//
// OK, time to rat on our buddy...
// See Documentation/RCU/stallwarn.rst for info on how to debug
// RCU CPU stall warnings.
//
// Complain about tasks blocking the grace period.
// Rewrite if needed in case of slow consoles.
// Kick and suppress, if so configured.
//
// OK, time to rat on ourselves...
// See Documentation/RCU/stallwarn.rst for info on how to debug
// RCU CPU stall warnings.
//
// Rewrite if needed in case of slow consoles.
//
// Attempt to revive the RCU machinery by forcing a context switch.
//
// A context switch would normally allow the RCU state machine to make
// progress and it could be we're stuck in kernel space without context
// switches for an entirely unreasonable amount of time.
//
// Check if it was requested (via rcu_cpu_stall_reset()) that the FQS
// loop has to set jiffies to ensure a non-stale jiffies value. This
// is required to have good jiffies value after coming out of long
// breaks of jiffies updates. Not doing so can cause false positives.
//
// Lots of memory barriers to reject false positives.
//
// The idea is to pick up rcu_state.gp_seq, then
// rcu_state.jiffies_stall, then rcu_state.gp_start, and finally
// another copy of rcu_state.gp_seq.  These values are updated in
// the opposite order with memory barriers (or equivalent) during
// grace-period initialization and cleanup.  Now, a false positive
// can occur if we get an new value of rcu_state.gp_start and a old
// value of rcu_state.jiffies_stall.  But given the memory barriers,
// the only way that this can happen is if one grace period ends
// and another starts between these two fetches.  This is detected
// by comparing the second fetch of rcu_state.gp_seq with the
// previous fetch from rcu_state.gp_seq.
//
// Given this check, comparisons of jiffies, rcu_state.jiffies_stall,
// and rcu_state.gp_start suffice to forestall false positives.
//
// If a virtual machine is stopped by the host it can look to
// the watchdog like an RCU stall. Check to see if the host
// stopped the vm.
//

// We haven't checked in, so go dump stack.
// They had a few time units to dump stack, so complain.
//
// RCU forward-progress mechanisms, including for callback invocation.
//
// Check to see if a failure to end RCU priority inversion was due to
// a CPU not passing through a quiescent state.  When this happens, there
// is nothing that RCU priority boosting can do to help, so we shouldn't
// count this as an RCU priority boosting failure.  A return of true says
// RCU priority boosting is to blame, and false says otherwise.  If false
// is returned, the first of the CPUs to blame is stored through cpup.
// If there was no CPU blocking the current grace period, but also nothing
// in need of being boosted, *cpup is set to -1.  This can happen in case
// of vCPU preemption while the last CPU is reporting its quiscent state,
// for example.
//
// If cpup is NULL, then a lockless quick check is carried out, suitable
// for high-rate usage.  On the other hand, if cpup is non-NULL, each
// rcu_node structure's ->lock is acquired, ruling out high-rate usage.
//
// cpup = -1;
// No CPUs without quiescent states for this rnp.
// Find the first holdout CPU.
// cpup = cpu;
// Can't blame CPUs, so must blame RCU priority boosting.
//
// Show the state of the grace-period kthreads.
//
// This function checks for grace-period requests that fail to motivate
// RCU to come out of its idle mode.
//
// Hold onto the leaf lock to make others see warned==1.
// irqs remain disabled.
//
// Do a forward-progress check for rcutorture.  This is normally invoked
// due to an OOM event.  The argument "j" gives the time period during
// which rcutorture would like progress to have been made.
//
// Commandeer a sysrq key to dump RCU's tree.
// Dump grace-period-request information due to commandeered sysrq.
extern "C" {
    pub fn register_sysrq_key(_arg: 'y', _arg: &sysrq_rcudump_op) -> return;
}

//
// RCU CPU stall-warning notifiers
extern "C" {
    pub fn ATOMIC_NOTIFIER_HEAD(_arg: rcu_cpu_stall_notifier_list) -> static;
}
//
// rcu_stall_chain_notifier_register - Add an RCU CPU stall notifier
// @n: Entry to add.
//
// Adds an RCU CPU stall notifier to an atomic notifier chain.
// The @action passed to a notifier will be @RCU_STALL_NOTIFY_NORM or
// friends.  The @data will be the duration of the stalled grace period,
// in jiffies, coerced to a void* pointer.
//
// Returns 0 on success, %-EEXIST on error.
//
extern "C" {
    pub fn atomic_notifier_chain_register(_arg: &rcu_cpu_stall_notifier_list, _arg: n) -> return;
}
//
// rcu_stall_chain_notifier_unregister - Remove an RCU CPU stall notifier
// @n: Entry to add.
//
// Removes an RCU CPU stall notifier from an atomic notifier chain.
//
// Returns zero on success, %-ENOENT on failure.
//
extern "C" {
    pub fn atomic_notifier_chain_unregister(_arg: &rcu_cpu_stall_notifier_list, _arg: n) -> return;
}
//
// rcu_stall_notifier_call_chain - Call functions in an RCU CPU stall notifier chain
// @val: Value passed unmodified to notifier function
// @v: Pointer passed unmodified to notifier function
//
// Calls each function in the RCU CPU stall notifier chain in turn, which
// is an atomic call chain.  See atomic_notifier_call_chain() for more
// information.
//
// This is for use within RCU, hence the omission of the extra asterisk
// to indicate a non-kerneldoc format header comment.
//
extern "C" {
    pub fn atomic_notifier_call_chain(_arg: &rcu_cpu_stall_notifier_list, _arg: val, _arg: v) -> return;
}
