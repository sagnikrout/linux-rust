//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/tree_plugin.h
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
// Internal non-public definitions that provide either classic
// or preemptible semantics.
//
// Copyright Red Hat, 2009
// Copyright IBM Corporation, 2009
//
// Author: Ingo Molnar <mingo@elte.hu>
// Paul E. McKenney <paulmck@linux.ibm.com>
//

//
// In order to read the offloaded state of an rdp in a safe
// and stable way and prevent from its value to be changed
// under us, we must either hold the barrier mutex, the cpu
// hotplug lock (read or write) or the nocb lock. Local
// non-preemptible reads are also safe. NOCB kthreads and
// timers have their own means of synchronization against the
// offloaded state updaters.
//
extern "C" {
    pub fn rcu_segcblist_is_offloaded(_arg: &rdp->cblist) -> return;
}
//
// Check the RCU kernel configuration parameters and print informative
// messages about anything out of the ordinary.
//

extern "C" {
    pub fn rcu_report_exp_rnp(rnp: *mut rcu_node, wake: bool) -> static void;
}
extern "C" {
    pub fn rcu_read_unlock_special(t: *mut task_struct) -> static void;
}
//
// Tell them what RCU they are running.
//
// Flags for rcu_preempt_ctxt_queue() decision table.
pub const RCU_GP_TASKS: c_uint = 0x8;
pub const RCU_EXP_TASKS: c_uint = 0x4;
pub const RCU_GP_BLKD: c_uint = 0x2;
pub const RCU_EXP_BLKD: c_uint = 0x1;
//
// Queues a task preempted within an RCU-preempt read-side critical
// section into the appropriate location within the ->blkd_tasks list,
// depending on the states of any ongoing normal and expedited grace
// periods.  The ->gp_tasks pointer indicates which element the normal
// grace period is waiting on (NULL if none), and the ->exp_tasks pointer
// indicates which element the expedited grace period is waiting on (again,
// NULL if none).  If a grace period is waiting on a given element in the
// ->blkd_tasks list, it also waits on all subsequent elements.  Thus,
// adding a task to the tail of the list blocks any grace period that is
// already waiting on one of the elements.  In contrast, adding a task
// to the head of the list won't block any grace period that is already
// waiting on one of the elements.
//
// This queuing is imprecise, and can sometimes make an ongoing grace
// period wait for a task that is not strictly speaking blocking it.
// Given the choice, we needlessly block a normal grace period rather than
// blocking an expedited grace period.
//
// Note that an endless sequence of expedited grace periods still cannot
// indefinitely postpone a normal grace period.  Eventually, all of the
// fixed number of preempted tasks blocking the normal grace period that are
// not also blocking the expedited grace period will resume and complete
// their RCU read-side critical sections.  At that point, the ->gp_tasks
// pointer will equal the ->exp_tasks pointer, at which point the end of
// the corresponding expedited grace period will also be the end of the
// normal grace period.
//
// RCU better not be waiting on newly onlined CPUs!
//
// Decide where to queue the newly blocked task.  In theory,
// this could be an if-statement.  In practice, when I tried
// that, it was quite messy.
//
// Blocking neither GP, or first task blocking the normal
// GP but not blocking the already-waiting expedited GP.
// Queue at the head of the list to avoid unnecessarily
// blocking the already-waiting GPs.
//
// First task arriving that blocks either GP, or first task
// arriving that blocks the expedited GP (with the normal
// GP already waiting), or a task arriving that blocks
// both GPs with both GPs already waiting.  Queue at the
// tail of the list to avoid any GP waiting on any of the
// already queued tasks that are not blocking it.
//
// Second or subsequent task blocking the expedited GP.
// The task either does not block the normal GP, or is the
// first task blocking the normal GP.  Queue just after
// the first task blocking the expedited GP.
//
// Second or subsequent task blocking the normal GP.
// The task does not block the expedited GP. Queue just
// after the first task blocking the normal GP.
//
// Yet another exercise in excessive paranoia.
//
// We have now queued the task.  If it was the first one to
// block either grace period, update the ->gp_tasks and/or
// ->exp_tasks pointers, respectively, to reference the newly
// blocked tasks.
//
// Report the quiescent state for the expedited GP.  This expedited
// GP should not be able to end until we report, so there should be
// no need to check for a subsequent expedited GP.  (Though we are
// still in a quiescent state in any case.)
//
// Interrupts are disabled, so ->cpu_no_qs.b.exp cannot change.
//
// Record a preemptible-RCU quiescent state for the specified CPU.
// Note that this does not necessarily mean that the task currently running
// on the CPU is in a quiescent state:  Instead, it means that the current
// grace period need not wait on any RCU read-side critical section that
// starts later on this CPU.  It also means that if the current task is
// in an RCU read-side critical section, it has already added itself to
// some leaf rcu_node structure's ->blkd_tasks list.  In addition to the
// current task, there might be any number of other tasks blocked while
// in an RCU read-side critical section.
//
// Unlike non-preemptible-RCU, quiescent state reports for expedited
// grace periods are handled separately via deferred quiescent states
// and context switch events.
//
// Callers to this function must disable preemption.
//
// We have entered the scheduler, and the current task might soon be
// context-switched away from.  If this task is in an RCU read-side
// critical section, we will no longer be able to rely on the CPU to
// record that fact, so we enqueue the task on the blkd_tasks list.
// The task will dequeue itself when it exits the outermost enclosing
// RCU read-side critical section.  Therefore, the current grace period
// cannot be permitted to complete until the blkd_tasks list entries
// predating the current grace period drain, in other words, until
// rnp->gp_tasks becomes NULL.
//
// Caller must disable interrupts.
//
// Possibly blocking in an RCU read-side critical section.
//
// Verify the CPU's sanity, trace the preemption, and
// then queue the task as required based on the states
// of any ongoing and expedited grace periods.
//
// Either we were not in an RCU read-side critical section to
// begin with, or we have now recorded that critical section
// globally.  Either way, we can now note a quiescent state
// for this CPU.  Again, if we were in an RCU read-side critical
// section, and if that critical section was blocking the current
// grace period, then the fact that the task has been enqueued
// means that we continue to block the current grace period.
//
// Check for preempted RCU readers blocking the current grace period
// for the specified rcu_node structure.  If the caller needs a reliable
// answer, it must hold the rcu_node's ->lock.
//
// limit value for ->rcu_read_lock_nesting.

//
// Preemptible RCU implementation for rcu_read_lock().
// Just increment ->rcu_read_lock_nesting, shared state will be updated
// if we block.
//
// Preemptible RCU implementation for rcu_read_unlock().
// Decrement ->rcu_read_lock_nesting.  If the result is zero (outermost
// rcu_read_unlock()) and ->rcu_read_unlock_special is non-zero, then
// invoke rcu_read_unlock_special() to clean up after a context switch
// in an RCU read-side critical section and other special cases.
//
// Advance a ->blkd_tasks-list pointer to the next entry, instead
// returning NULL if at the end of the list.
//
// Return true if the specified rcu_node structure has tasks that were
// preempted within an RCU read-side critical section.
//
// Report deferred quiescent states.  The deferral time can
// be quite short, for example, in the case of the call from
// rcu_read_unlock_special().
//
// If RCU core is waiting for this CPU to exit its critical section,
// report the fact that it has exited.  Because irqs are disabled,
// t->rcu_read_unlock_special cannot change.
//
// Respond to a request by an expedited grace period for a
// quiescent state from this CPU.  Note that requests from
// tasks are handled when removing the task from the
// blocked-tasks list below.
//
// Clean up if blocked during RCU read-side critical section.
//
// Remove this task from the list it blocked on.  The task
// now remains queued on the rcu_node corresponding to the
// CPU it first blocked on, so there is no longer any need
// to loop.  Retain a WARN_ON_ONCE() out of sheer paranoia.
//
// Snapshot ->boost_mtx ownership w/rnp->lock held.
//
// If this was the last task on the current list, and if
// we aren't waiting on any CPUs, report the quiescent state.
// Note that rcu_report_unblock_qs_rnp() releases rnp->lock,
// so we must take a snapshot of the expedited state.
//
// If this was the last task on the expedited lists,
// then we need to report up the rcu_node hierarchy.
//
// Unboost if we were boosted.
//
// Is a deferred quiescent-state pending, and are we also not in
// an RCU read-side critical section?  It is the caller's responsibility
// to ensure it is otherwise safe to report any deferred quiescent
// states.  The reason for this is that it is safe to report a
// quiescent state during context switch even though preemption
// is disabled.  This function cannot be expected to understand these
// nuances, so the caller must handle them.
//
// Report a deferred quiescent state if needed and safe to do so.
// As with rcu_preempt_need_deferred_qs(), "safe" involves only
// not being in an RCU read-side critical section.  The caller must
// evaluate safety in terms of interrupt, softirq, and preemption
// disabling.
//
// If we got here from a softirq/irq_work that fired while
// rcu_preempt_depth() > 0, the deferred-QS mechanism has been
// consumed without doing any work: rcu_preempt_need_deferred_qs()
// just returned false because the task is still in a reader, so
// the actual QS report has to wait for the next
// rcu_read_unlock().
//
// Clear ->defer_qs_pending here so the next outer
// rcu_read_unlock_special() can re-arm a fresh mechanism (in
// particular the irq_work path, which the local_irq_enable()
// recovery boundary cannot itself reschedule from).
//
// Recursion safety: rcu_preempt_depth() > 0 means we are inside
// an outer reader, so any inner rcu_read_unlock() reached via
// tracing (bpf programs attached to trace points) brings
// nesting to outer (> 0), never to 0, so no recursive
// raise_softirq_irqoff()/irq_work_queue_on() can be triggered
// by this clear.
//
// Minimal handler to give the scheduler a chance to re-evaluate.
//
// If the IRQ work handler happens to run in the middle of RCU read-side
// critical section, it could be ineffective in getting the scheduler's
// attention to report a deferred quiescent state (the whole point of the
// IRQ work). For this reason, requeue the IRQ work.
//
// Basically, we want to avoid following situation:
// 1. rcu_read_unlock() queues IRQ work (state -> DEFER_QS_PENDING)
// 2. CPU enters new rcu_read_lock()
// 3. IRQ work runs but cannot report QS due to rcu_preempt_depth() > 0
// 4. rcu_read_unlock() does not re-queue work (state still PENDING)
// 5. Deferred QS reporting does not happen.
//
// Check if expedited grace period processing during unlock is needed.
//
// This function determines whether expedited handling is required based on:
// 1. Task blocking an expedited grace period (based on a heuristic, could be
// false-positive, see below.)
// 2. CPU participating in an expedited grace period
// 3. Strict grace period mode requiring expedited handling
// 4. RCU priority deboosting needs when interrupts were disabled
//
// @t: The task being checked
// @rdp: The per-CPU RCU data
// @rnp: The RCU node for this CPU
// @irqs_were_disabled: Whether interrupts were disabled before rcu_read_unlock()
//
// Returns true if expedited processing of the rcu_read_unlock() is needed.
//
// Check if this task is blocking an expedited grace period. If the
// task was preempted within an RCU read-side critical section and is
// on the expedited grace period blockers list (exp_tasks), we need
// expedited handling to unblock the expedited GP. This is not an exact
// check because 't' might not be on the exp_tasks list at all - its
// just a fast heuristic that can be false-positive sometimes.
//
// Check if this CPU is participating in an expedited grace period.
// The expmask bitmap tracks which CPUs need to check in for the
// current expedited GP. If our CPU's bit is set, we need expedited
// handling to help complete the expedited GP.
//
// In CONFIG_RCU_STRICT_GRACE_PERIOD=y kernels, all grace periods
// are treated as short for testing purposes even if that means
// disturbing the system more. Check if either:
// - This CPU has not yet reported a quiescent state, or
// - This task was preempted within an RCU critical section
// In either case, require expedited handling for strict GP mode.
//
// RCU priority boosting case: If a task is subject to RCU priority
// boosting and exits an RCU read-side critical section with interrupts
// disabled, we need expedited handling to ensure timely deboosting.
// Without this, a low-priority task could incorrectly run at high
// real-time priority for an extended period degrading real-time
// responsiveness. This applies to all CONFIG_RCU_BOOST=y kernels,
// not just to PREEMPT_RT.
//
// Handle special cases during rcu_read_unlock(), such as needing to
// notify RCU core processing or task having blocked during the RCU
// read-side critical section.
//
// NMI handlers cannot block and cannot safely manipulate state.
// Need to defer quiescent state until everything is enabled.
// Using softirq, safe to awaken, and either the
// wakeup is free or there is either an expedited
// GP in flight or a potential need to deboost.
// Enabling BH or preempt does reschedule, so...
// Also if no expediting and no possible deboosting,
// slow is OK.  Plus nohz_full CPUs eventually get
// tick enabled.
// Get scheduler to re-evaluate and call hooks.
// If !IRQ_WORK, FQS scan will eventually IPI.
//
// Check that the list of blocked tasks for the newly completed grace
// period is in fact empty.  It is a serious bug to complete a grace
// period that still has RCU readers blocked!  This function must be
// invoked -before- updating this rnp's ->gp_seq.
//
// Also, if there are blocked tasks on the list, they automatically
// block the newly created grace period, so set up ->gp_tasks accordingly.
//
// Check for a quiescent state from the current CPU, including voluntary
// context switches for Tasks RCU.  When a task blocks, the task is
// recorded in the corresponding CPU's rcu_node structure, which is checked
// elsewhere, hence this function need only check for quiescent states
// related to the current CPU, not to those related to tasks.
//
// No QS, force context switch if deferred.
// If GP is oldish, ask for help from rcu_read_unlock_special().
//
// Check for a task exiting while in a preemptible-RCU read-side
// critical section, clean up if so.  No need to issue warnings, as
// debug_check_no_locks_held() already does this if lockdep is enabled.
// Besides, if this function does anything other than just immediately
// return, there was a bug of some sort.  Spewing warnings from this
// function is like as not to simply obscure important prior warnings.
//
// Dump the blocked-tasks state, but limit the list dump to the
// specified number of elements.
//

//
// If strict grace periods are enabled, and if the calling
// __rcu_read_unlock() marks the beginning of a quiescent state, immediately
// report that quiescent state and, if requested, spin for a bit.
//
// rcu_report_qs_rdp() can only be invoked with a stable rdp and
// from the local CPU.
//
// The in_atomic_preempt_off() check ensures that we come here holding
// the last preempt_count (which will get dropped once we return to
// __rcu_read_unlock()).
//
// Tell them what RCU they are running.
//
// Note a quiescent state for PREEMPTION=n.  Because we do not need to know
// how many quiescent states passed, just if there was at least one since
// the start of the grace period, this just sets a flag.  The caller must
// have disabled preemption.
//
// Register an urgently needed quiescent state.  If there is an
// emergency, invoke rcu_momentary_eqs() to do a heavy-weight
// dyntick-idle quiescent state visible to other CPUs, which will in
// some cases serve for expedited as well as normal grace periods.
// Either way, register a lightweight quiescent state.
//
// Load rcu_urgent_qs before other flags.
//
// Note a PREEMPTION=n context switch. The caller must have disabled interrupts.
//
// Load rcu_urgent_qs before other flags.
//
// Because preemptible RCU does not exist, there are never any preempted
// RCU readers.
//
// Because there is no preemptible RCU, there can be no readers blocked.
//
// Because there is no preemptible RCU, there can be no deferred quiescent
// states.
//
// Except that we do need to respond to a request by an expedited
// grace period for a quiescent state from this CPU.  Note that in
// non-preemptible kernels, there can be no context switches within RCU
// read-side critical sections, which in turn means that the leaf rcu_node
// structure's blocked-tasks list is always empty.  is therefore no need to
// actually check it.  Instead, a quiescent state from this CPU suffices,
// and this function is only called from such a quiescent state.
//
// Because there is no preemptible RCU, there can be no readers blocked,
// so there is no need to check for blocked tasks.  So check only for
// bogus qsmask values.
//
// Check to see if this CPU is in a non-context-switch quiescent state,
// namely user mode and idle loop.
//
// Get here if this CPU took its interrupt from user
// mode, from the idle loop without this being a nested
// interrupt, or while not holding the task preempt count
// (with PREEMPT_COUNT=y). In this case, the CPU is in a
// quiescent state, so note it.
//
// No memory barrier is required here because rcu_qs()
// references only CPU-local variables that other CPUs
// neither access nor modify, at least not while the
// corresponding CPU is online.
//
// Because preemptible RCU does not exist, tasks cannot possibly exit
// while in preemptible RCU read-side critical sections.
//
// Dump the guaranteed-empty blocked-tasks state.  Trust but verify.
//

//
// If boosting, set rcuc kthreads to realtime priority.
//

//
// Is the current CPU running the RCU-callbacks kthread?
// Caller must have preemption disabled.
//

//
// Carry out RCU priority boosting on the task indicated by ->exp_tasks
// or ->boost_tasks, advancing the pointer to the next task in the
// ->blkd_tasks list.
//
// Note that irqs must be enabled: boosting the task can block.
// Returns 1 if there are more tasks needing to be boosted.
//
// Recheck under the lock: all tasks in need of boosting
// might exit their RCU read-side critical sections on their own.
//
// Preferentially boost tasks blocking expedited grace periods.
// This cannot starve the normal grace periods because a second
// expedited grace period must boost all blocked tasks, including
// those blocking the pre-existing normal grace period.
//
// We boost task t by manufacturing an rt_mutex that appears to
// be held by task t.  We leave a pointer to that rt_mutex where
// task t can find it, and task t will release the mutex when it
// exits its outermost RCU read-side critical section.  Then
// simply acquiring this artificial rt_mutex will boost task
// t's priority.  (Thanks to tglx for suggesting this approach!)
//
// Note that task t must acquire rnp->lock to remove itself from
// the ->blkd_tasks list, which it will do from exit() if from
// nowhere else.  We therefore are guaranteed that task t will
// stay around at least until we drop rnp->lock.  Note that
// rnp->lock also resolves races between our priority boosting
// and task t's exiting its outermost RCU read-side critical
// section.
//
// Lock only for side effect: boosts task t's priority.
//
// Priority-boosting kthread, one per leaf rcu_node.
//
// NOTREACHED
//
// Check to see if it is time to start boosting RCU readers that are
// blocking the current grace period, and, if so, tell the per-rcu_node
// kthread to start boosting them.  If there is an expedited grace
// period in progress, it is always time to boost.
//
// The caller must hold rnp->lock, which this function releases.
// The ->boost_kthread_task is immortal, so we don't need to worry
// about it going away.
//

//
// Do priority-boost accounting for the start of a new grace period.
//
// Create an RCU-boost kthread for the specified node if one does not
// already exist.  We only create this kthread for preemptible RCU.
//

//
// Is the current task RCU priority boosted?  This is used by
// rcutorture to check that tasks are always deboosted once then exit
// an RCU read-side critical section, no matter how many overlapping
// segments of rcu_read_lock(), preempt_disable(), local_bh_disable(),
// or local_irq_disable() made up that reader.
//
// The lockless accesses in rt_mutex_owner(&rnp->boost_mtx.rtmutex)
// are safe because tasks release ->boost_mtx when they own it, they
// cannot be boosted unless current->rcu_blocked_node is non-NULL,
// current->rcu_blocked_node is modified only by the current task,
// rt_mutex_owner() uses READ_ONCE() on the ->owner field, and the owner
// switching among other tasks cannot force an equality comparison.
//

//
// Is this CPU a NO_HZ_FULL CPU that should ignore RCU so that the
// grace-period kthread will do force_quiescent_state() processing?
// The idea is to avoid waking up RCU core processing on such a
// CPU unless the grace period has extended for too long.
//
// This code relies on the fact that all NO_HZ_FULL CPUs are also
// RCU_NOCB_CPU CPUs.
//

//
// Bind the RCU grace-period kthreads to the housekeeping CPU.
//
